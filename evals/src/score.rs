//! The scoring rules: what a run's observation says about a role. Pure and
//! deterministic — the same case and observation always give the same
//! score, so a recorded run can be scored again without a model.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::case::{Case, Expected, Kind, Location, Step};
use crate::observed::{Consumption, Observed, Review, ReviewFinding, RunInfo};
use crate::plan::{AtRef, PlanOutcome, PlanScore, score_plan};
use crate::update::{UpdateScore, UpdateStepScore, score_update};

/// Evidence and location codes of `codev review`, by their effect here.
const EVIDENCE_INVALID: [&str; 2] = ["review_evidence_not_found", "review_location_missing"];
const EVIDENCE_MOVED: &str = "review_evidence_moved";
const EVIDENCE_UNCHECKED: &str = "review_evidence_unchecked";

/// How a case's run ended, as far as the scoring is concerned.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case", tag = "outcome", content = "reason")]
pub enum Outcome {
    Scored,
    /// The run stopped: timeout, Claude Code failure, setup failure.
    Error(String),
    /// No review record was written.
    NoRecord,
    /// The record has an error that kept codev from reading the role.
    RecordInvalid(String),
    /// The evaluated role has no section, or reads disabled.
    RoleNotRun(String),
    /// A planning run that created no change.
    NoPlan,
}

impl Outcome {
    pub fn label(&self) -> &'static str {
        match self {
            Outcome::Scored => "scored",
            Outcome::Error(_) => "error",
            Outcome::NoRecord => "no-record",
            Outcome::RecordInvalid(_) => "record-invalid",
            Outcome::RoleNotRun(_) => "role-not-run",
            Outcome::NoPlan => "no-plan",
        }
    }
}

/// What a finding counts as.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Class {
    TruePositive,
    Duplicate,
    CrossRole,
    FalsePositive,
    Unreported,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Evidence {
    Found,
    Moved,
    Invalid,
    Unchecked,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FindingScore {
    pub id: String,
    pub role: String,
    pub title: String,
    pub severity: String,
    pub confidence: String,
    pub location: String,
    pub status: String,
    pub class: Class,
    /// The expected finding it matched, for true positives, duplicates and
    /// cross-role findings.
    pub expected: Option<String>,
    /// For a false positive or an unreported finding: the expected finding
    /// of its role at the same place, whose category it does not name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub near: Option<String>,
    pub evidence: Evidence,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExpectedScore {
    pub id: String,
    pub role: String,
    pub severity: String,
    pub category: String,
    /// The finding that matched it; `None` is a miss.
    pub matched_by: Option<String>,
    pub severity_agrees: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CaseScore {
    pub id: String,
    pub role: String,
    pub step: String,
    pub kind: String,
    #[serde(flatten)]
    pub outcome: Outcome,
    pub expected: Vec<ExpectedScore>,
    pub findings: Vec<FindingScore>,
    /// `plan.trace`: files that were to stay unchanged and did not.
    pub changed: Vec<String>,
    /// Files the run modified outside what its step allows.
    pub strayed: Vec<String>,
    /// Findings whose status is no longer `open`: the agent answered them
    /// although the evaluation asks it not to.
    pub answered: Vec<String>,
    /// Errors of the record other than evidence and location.
    pub record_errors: Vec<String>,
    pub cost_usd: Option<f64>,
    pub duration_ms: Option<u64>,
    /// The repetition this run is, from 1.
    pub repetition: u32,
    pub consumption: Option<Consumption>,
    /// A planning run's score; `None` for a review run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<PlanScore>,
    /// An update run's checks; `None` for any other run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update: Option<UpdateScore>,
}

impl CaseScore {
    fn is_planning(&self) -> bool {
        self.step == Step::PlanWrite.as_str()
    }

    /// A run of a reviewing role, which the role measures count: neither a
    /// planning run nor an update run.
    fn is_review(&self) -> bool {
        !self.is_planning() && self.step != Step::PlanUpdate.as_str()
    }

    /// The same score, as repetition `n`.
    pub fn at(mut self, n: u32) -> CaseScore {
        self.repetition = n;
        self
    }
}

/// Scores one run of a case from its observation, as repetition 1.
pub fn case(case: &Case, observed: &Observed) -> CaseScore {
    let mut score = unscored(case, observed);
    if case.step == Step::PlanTrace {
        trace(case, observed, &mut score);
    } else {
        review(case, observed, &mut score);
    }
    score
}

/// A run's score before its step's rules apply: what every run records,
/// as repetition 1.
fn unscored(case: &Case, observed: &Observed) -> CaseScore {
    CaseScore {
        id: case.id.clone(),
        role: case.role.clone(),
        step: case.step.as_str().into(),
        kind: case.kind.as_str().into(),
        outcome: Outcome::Scored,
        expected: Vec::new(),
        findings: Vec::new(),
        changed: Vec::new(),
        strayed: observed.strayed.clone(),
        answered: Vec::new(),
        record_errors: Vec::new(),
        cost_usd: observed.cost_usd,
        duration_ms: observed.duration_ms,
        repetition: 1,
        consumption: observed.consumption.clone(),
        plan: None,
        update: None,
    }
}

/// Scores one run of an update case by its checks, as repetition 1.
pub fn update_case(case: &Case, observed: &Observed) -> CaseScore {
    let mut score = unscored(case, observed);
    let Some(expect) = &case.update else {
        return score;
    };
    score.outcome = match &observed.error {
        Some(error) => Outcome::Error(error.clone()),
        None => Outcome::Scored,
    };
    score.update = Some(score_update(expect, &case.change_dir(), observed));
    score
}

/// Scores one run of a planning case, against the repository at its
/// `ref`, as repetition 1. A run that stopped is scored as a plan that
/// modifies nothing, as a review that stopped misses what it expected.
pub fn plan_case(case: &Case, observed: &Observed, at_ref: &AtRef) -> CaseScore {
    let mut score = unscored(case, observed);
    let Some(expect) = &case.plan else {
        return score;
    };
    let (no_files, no_validate) = (BTreeMap::new(), BTreeMap::new());
    let (files, validate) = match &observed.error {
        Some(_) => (&no_files, &no_validate),
        None => (&observed.files, &observed.validate),
    };
    let plan = score_plan(expect, files, validate, at_ref);
    score.outcome = match &observed.error {
        Some(error) => Outcome::Error(error.clone()),
        None if plan.outcome == PlanOutcome::NoPlan => Outcome::NoPlan,
        None => Outcome::Scored,
    };
    score.plan = Some(plan);
    score
}

fn unmatched_expectations(case: &Case) -> Vec<ExpectedScore> {
    case.expected
        .iter()
        .map(|e| ExpectedScore {
            id: e.id.clone(),
            role: e.role.clone(),
            severity: e.severity.clone(),
            category: e.category.clone(),
            matched_by: None,
            severity_agrees: false,
        })
        .chain(case.fixed.iter().map(|f| ExpectedScore {
            id: format!("{} contains \"{}\"", f.path, f.contains),
            role: case.role.clone(),
            severity: String::new(),
            category: "traceability".into(),
            matched_by: None,
            severity_agrees: false,
        }))
        .collect()
}

fn trace(case: &Case, observed: &Observed, score: &mut CaseScore) {
    score.expected = unmatched_expectations(case);
    if let Some(error) = &observed.error {
        score.outcome = Outcome::Error(error.clone());
        return;
    }
    for (check, expected) in case.fixed.iter().zip(score.expected.iter_mut()) {
        let after = observed.files.get(&check.path).cloned().flatten();
        if after.is_some_and(|text| contains_ignoring_case(&text, &check.contains)) {
            expected.matched_by = Some(check.path.clone());
            expected.severity_agrees = true;
        }
    }
    for check in &case.unchanged {
        let after = observed.files.get(&check.path).cloned().flatten();
        let same = after.is_some_and(|text| without_cr(&text) == without_cr(&check.original));
        if !same {
            score.changed.push(check.path.clone());
        }
    }
}

fn contains_ignoring_case(text: &str, needle: &str) -> bool {
    text.to_lowercase().contains(&needle.to_lowercase())
}

fn without_cr(text: &str) -> String {
    text.replace('\r', "")
}

fn review(case: &Case, observed: &Observed, score: &mut CaseScore) {
    score.expected = unmatched_expectations(case);
    if let Some(error) = &observed.error {
        score.outcome = Outcome::Error(error.clone());
        return;
    }
    let Some(review) = observed.review().filter(|r| r.exists) else {
        score.outcome = Outcome::NoRecord;
        return;
    };
    score.record_errors = review
        .issues
        .iter()
        .filter(|i| i.severity == "error" && !is_evidence_code(&i.code))
        .map(|i| format!("{}: {}", i.code, i.message))
        .collect();

    let state = review
        .steps
        .iter()
        .find(|s| s.step == case.step.as_str())
        .and_then(|s| s.roles.iter().find(|r| r.role == case.role))
        .map(|r| r.state.clone())
        .unwrap_or_else(|| "missing".into());
    if state != "findings" && state != "no-finding" {
        score.outcome = match score.record_errors.first() {
            Some(error) => Outcome::RecordInvalid(error.clone()),
            None => Outcome::RoleNotRun(state),
        };
        return;
    }

    let findings: Vec<&ReviewFinding> = review
        .findings
        .iter()
        .filter(|f| f.step == case.step.as_str())
        .collect();
    let locations: Vec<Location> = findings
        .iter()
        .map(|f| Location::parse(&f.location))
        .collect();
    let texts: Vec<String> = findings
        .iter()
        .map(|f| format!("{}\n{}", f.title, f.recommendation))
        .collect();
    let matches = |e: &Expected, i: usize| e.located_at(&locations[i]) && e.named_by(&texts[i]);
    let edges: Vec<Vec<bool>> = case
        .expected
        .iter()
        .map(|e| {
            (0..findings.len())
                .map(|i| findings[i].role == e.role && matches(e, i))
                .collect()
        })
        .collect();
    let taken = maximum_matching(&edges, findings.len());
    for (i, pair) in taken.iter().enumerate() {
        if let Some(e_index) = *pair {
            let slot = &mut score.expected[e_index];
            slot.matched_by = Some(findings[i].id.clone());
            slot.severity_agrees = findings[i].severity == case.expected[e_index].severity;
        }
    }

    for (i, finding) in findings.iter().enumerate() {
        let Unpaired {
            class,
            expected: matched,
            near,
        } = match taken[i] {
            Some(e) => Unpaired {
                class: Class::TruePositive,
                expected: Some(case.expected[e].id.clone()),
                near: None,
            },
            None => classify_unmatched(
                case,
                finding,
                |e| matches(e, i),
                |e| e.located_at(&locations[i]),
            ),
        };
        if finding.status != "open" {
            score
                .answered
                .push(format!("{} ({})", finding.id, finding.status));
        }
        score.findings.push(FindingScore {
            id: finding.id.clone(),
            role: finding.role.clone(),
            title: finding.title.clone(),
            severity: finding.severity.clone(),
            confidence: finding.confidence.clone(),
            location: finding.location.clone(),
            status: finding.status.clone(),
            class,
            expected: matched,
            near,
            evidence: evidence(&review, &finding.id),
        });
    }
}

/// A maximum one-to-one pairing of expected findings (rows of `edges`)
/// with findings (columns), by augmenting paths: expected findings in case
/// order, each trying findings in record order; an earlier pair gives way
/// only when that lets one more expected finding be paired. Returns, per
/// finding, the expected finding it is paired with.
fn maximum_matching(edges: &[Vec<bool>], findings: usize) -> Vec<Option<usize>> {
    fn augment(
        e: usize,
        edges: &[Vec<bool>],
        owner: &mut [Option<usize>],
        seen: &mut [bool],
    ) -> bool {
        for i in 0..owner.len() {
            if !edges[e][i] || seen[i] {
                continue;
            }
            seen[i] = true;
            let free = match owner[i] {
                None => true,
                Some(other) => augment(other, edges, owner, seen),
            };
            if free {
                owner[i] = Some(e);
                return true;
            }
        }
        false
    }
    let mut owner = vec![None; findings];
    for e in 0..edges.len() {
        let mut seen = vec![false; findings];
        augment(e, edges, &mut owner, &mut seen);
    }
    owner
}

/// How a finding is classified, and the expected findings it relates to.
struct Unpaired {
    class: Class,
    /// The expected finding it matches (duplicate, cross-role).
    expected: Option<String>,
    /// The expected finding of its role at the same place, whose category
    /// it does not name (false positive, unreported).
    near: Option<String>,
}

/// An unpaired finding: a duplicate or a cross-role finding when it matches
/// an expected finding (location and category), otherwise unreported or a
/// false positive.
fn classify_unmatched(
    case: &Case,
    finding: &ReviewFinding,
    matches: impl Fn(&Expected) -> bool,
    located: impl Fn(&Expected) -> bool,
) -> Unpaired {
    let find = |same_role: bool, test: &dyn Fn(&Expected) -> bool| {
        case.expected
            .iter()
            .find(|e| (e.role == finding.role) == same_role && test(e))
    };
    if let Some(e) = find(true, &matches) {
        return Unpaired {
            class: Class::Duplicate,
            expected: Some(e.id.clone()),
            near: None,
        };
    }
    if let Some(e) = find(false, &matches) {
        return Unpaired {
            class: Class::CrossRole,
            expected: Some(e.id.clone()),
            near: None,
        };
    }
    let near = find(true, &located).map(|e| e.id.clone());
    let class = if finding.confidence == "low" {
        Class::Unreported
    } else {
        Class::FalsePositive
    };
    Unpaired {
        class,
        expected: None,
        near,
    }
}

fn is_evidence_code(code: &str) -> bool {
    EVIDENCE_INVALID.contains(&code) || code == EVIDENCE_MOVED || code == EVIDENCE_UNCHECKED
}

/// The evidence state of a finding, from the issues codev reported for it.
/// codev starts each evidence message with the finding's id.
fn evidence(review: &Review, id: &str) -> Evidence {
    let prefix = format!("{id}:");
    let codes: Vec<&str> = review
        .issues
        .iter()
        .filter(|i| i.message.starts_with(&prefix))
        .map(|i| i.code.as_str())
        .collect();
    if codes.iter().any(|c| EVIDENCE_INVALID.contains(c)) {
        Evidence::Invalid
    } else if codes.contains(&EVIDENCE_MOVED) {
        Evidence::Moved
    } else if codes.contains(&EVIDENCE_UNCHECKED) {
        Evidence::Unchecked
    } else {
        Evidence::Found
    }
}

/// The thresholds a role must meet.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thresholds {
    pub min_recall: f64,
    pub max_false_positives: u32,
    pub min_evidence: f64,
}

impl Default for Thresholds {
    fn default() -> Self {
        Thresholds {
            min_recall: 0.5,
            max_false_positives: 2,
            min_evidence: 0.8,
        }
    }
}

/// Thresholds given on the command line; `None` falls back to the
/// environment, then to the default.
#[derive(Debug, Clone, Default)]
pub struct ThresholdFlags {
    pub min_recall: Option<String>,
    pub max_false_positives: Option<String>,
    pub min_evidence: Option<String>,
}

impl Thresholds {
    /// Flags win over `CODEV_EVAL_*` variables, which win over defaults. A
    /// value that does not parse names its flag or variable.
    pub fn resolve(
        flags: &ThresholdFlags,
        env: &dyn Fn(&str) -> Option<String>,
    ) -> Result<Thresholds, String> {
        let default = Thresholds::default();
        let pick = |flag: &Option<String>, name: &str, var: &str| -> Option<(String, String)> {
            flag.clone()
                .map(|v| (v, format!("--{name}")))
                .or_else(|| env(var).map(|v| (v, var.to_string())))
        };
        let ratio = |value: Option<(String, String)>, fallback: f64| -> Result<f64, String> {
            match value {
                None => Ok(fallback),
                Some((text, source)) => text
                    .trim()
                    .parse::<f64>()
                    .ok()
                    .filter(|v| (0.0..=1.0).contains(v))
                    .ok_or_else(|| format!("{source}: `{text}` is not a number between 0 and 1")),
            }
        };
        let min_recall = ratio(
            pick(&flags.min_recall, "min-recall", "CODEV_EVAL_MIN_RECALL"),
            default.min_recall,
        )?;
        let min_evidence = ratio(
            pick(
                &flags.min_evidence,
                "min-evidence",
                "CODEV_EVAL_MIN_EVIDENCE",
            ),
            default.min_evidence,
        )?;
        let max_false_positives = match pick(
            &flags.max_false_positives,
            "max-false-positives",
            "CODEV_EVAL_MAX_FALSE_POSITIVES",
        ) {
            None => default.max_false_positives,
            Some((text, source)) => text
                .trim()
                .parse()
                .map_err(|_| format!("{source}: `{text}` is not a whole number"))?,
        };
        Ok(Thresholds {
            min_recall,
            max_false_positives,
            min_evidence,
        })
    }
}

/// Evidence states counted for a role: in one repetition, whole numbers;
/// in a report, their medians over the repetitions.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceCounts {
    pub found: f64,
    pub moved: f64,
    pub invalid: f64,
    pub unchecked: f64,
}

/// A measure over the repetitions: its median, minimum and maximum.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Spread {
    pub median: f64,
    pub min: f64,
    pub max: f64,
}

/// The median of `values` — of an even count, the mean of the two middle
/// ones — rounded to four decimals; `None` without a value.
pub fn median(values: &[f64]) -> Option<f64> {
    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.total_cmp(b));
    let n = sorted.len();
    if n == 0 {
        return None;
    }
    let middle = if n % 2 == 1 {
        sorted[n / 2]
    } else {
        (sorted[n / 2 - 1] + sorted[n / 2]) / 2.0
    };
    Some(round4(middle))
}

/// The spread of `values`; `None` without a value.
pub fn spread(values: &[f64]) -> Option<Spread> {
    Some(Spread {
        median: median(values)?,
        min: round4(values.iter().copied().fold(f64::INFINITY, f64::min)),
        max: round4(values.iter().copied().fold(f64::NEG_INFINITY, f64::max)),
    })
}

/// The ranges over the repetitions of the measures a threshold applies to.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleRanges {
    pub recall: Option<Spread>,
    pub false_positives: Option<Spread>,
    pub evidence_validity: Option<Spread>,
}

/// A role's measures: each the median over the repetitions of what the
/// role scored in that repetition.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleScore {
    pub role: String,
    /// The distinct cases of the role, and how many are seeded or controls.
    pub cases: u32,
    pub seeded: u32,
    pub controls: u32,
    /// The repetitions in which the role has a case.
    pub repetitions: u32,
    pub expected: f64,
    pub true_positives: f64,
    pub misses: f64,
    pub false_positives: f64,
    pub unreported: f64,
    pub duplicates: f64,
    pub cross_role: f64,
    pub severity_agreement: f64,
    pub evidence: EvidenceCounts,
    /// True positives over expected; `None` without expectation.
    pub recall: Option<f64>,
    /// (found + moved) over checked findings; `None` without one.
    pub evidence_validity: Option<f64>,
    pub ranges: RoleRanges,
    /// The sum of its cases' median costs.
    pub cost_usd: f64,
    /// The sums of its cases' median consumption.
    pub consumption: Measures,
    pub failures: Vec<String>,
}

/// The suite-level figures, each computed for a repetition over all its
/// cases, then given as a median with its range.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SuiteFigures {
    pub expected: f64,
    pub true_positives: Option<Spread>,
    pub recall: Option<Spread>,
    pub checked: f64,
    pub valid: f64,
    pub evidence_validity: Option<Spread>,
    pub false_positives: Option<Spread>,
    pub false_positives_per_case: Option<Spread>,
}

/// What runs consumed: per case, the medians over its repetitions; per
/// role and for the suite, the sums of those medians. `None` when no run
/// reported the measure.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Measures {
    pub cost_usd: Option<f64>,
    pub duration_ms: Option<f64>,
    pub duration_api_ms: Option<f64>,
    pub turns: Option<f64>,
    pub permission_denials: Option<f64>,
    pub input_tokens: Option<f64>,
    pub cache_creation_tokens: Option<f64>,
    pub cache_read_tokens: Option<f64>,
    pub output_tokens: Option<f64>,
    /// The calls to the model; `None` without the event stream.
    #[serde(default)]
    pub calls: Option<f64>,
    /// The tokens a call reads — written to the cache, read from it, read
    /// without it — divided by the calls.
    #[serde(default)]
    pub input_per_call: Option<f64>,
    #[serde(default)]
    pub output_per_call: Option<f64>,
}

/// One consumption measure: what it is called in the reports and how its
/// value reads. The order of [`Measure::ALL`] is the reports' order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Measure {
    Cost,
    Duration,
    ApiDuration,
    Turns,
    PermissionDenials,
    InputTokens,
    CacheWrites,
    CacheReads,
    OutputTokens,
    Calls,
    InputPerCall,
    OutputPerCall,
}

impl Measure {
    pub const ALL: [Measure; 12] = [
        Measure::Cost,
        Measure::Duration,
        Measure::ApiDuration,
        Measure::Turns,
        Measure::PermissionDenials,
        Measure::InputTokens,
        Measure::CacheWrites,
        Measure::CacheReads,
        Measure::OutputTokens,
        Measure::Calls,
        Measure::InputPerCall,
        Measure::OutputPerCall,
    ];

    /// Its name in a row of the comparison.
    pub fn label(self) -> &'static str {
        match self {
            Measure::Cost => "Cost (USD)",
            Measure::Duration => "Duration (s)",
            Measure::ApiDuration => "API duration (s)",
            Measure::Turns => "Turns",
            Measure::PermissionDenials => "Permission denials",
            Measure::InputTokens => "Input tokens, uncached",
            Measure::CacheWrites => "Tokens written to the cache",
            Measure::CacheReads => "Tokens read from the cache",
            Measure::OutputTokens => "Output tokens",
            Measure::Calls => "Calls to the model",
            Measure::InputPerCall => "Input per call",
            Measure::OutputPerCall => "Output per call",
        }
    }

    /// Its name in a column of the report.
    pub fn column(self) -> &'static str {
        match self {
            Measure::Cost => "Cost (USD)",
            Measure::Duration => "Duration (s)",
            Measure::ApiDuration => "API duration (s)",
            Measure::Turns => "Turns",
            Measure::PermissionDenials => "Denials",
            Measure::InputTokens => "Input, uncached",
            Measure::CacheWrites => "Cache writes",
            Measure::CacheReads => "Cache reads",
            Measure::OutputTokens => "Output",
            Measure::Calls => "Calls",
            Measure::InputPerCall => "Input/call",
            Measure::OutputPerCall => "Output/call",
        }
    }

    /// A value of the measure: `—` when absent, seconds for a duration,
    /// dollars with `cost_decimals` decimals, whole counts otherwise.
    pub fn show(self, value: Option<f64>, cost_decimals: usize) -> String {
        match (self, value) {
            (_, None) => "—".into(),
            (Measure::Cost, Some(v)) => format!("{v:.cost_decimals$}"),
            (Measure::Duration | Measure::ApiDuration, Some(v)) => format!("{:.0}", v / 1000.0),
            (_, Some(v)) => number(v.round()),
        }
    }
}

impl Measures {
    pub fn get(&self, m: Measure) -> Option<f64> {
        match m {
            Measure::Cost => self.cost_usd,
            Measure::Duration => self.duration_ms,
            Measure::ApiDuration => self.duration_api_ms,
            Measure::Turns => self.turns,
            Measure::PermissionDenials => self.permission_denials,
            Measure::InputTokens => self.input_tokens,
            Measure::CacheWrites => self.cache_creation_tokens,
            Measure::CacheReads => self.cache_read_tokens,
            Measure::OutputTokens => self.output_tokens,
            Measure::Calls => self.calls,
            Measure::InputPerCall => self.input_per_call,
            Measure::OutputPerCall => self.output_per_call,
        }
    }

    fn slot(&mut self, m: Measure) -> &mut Option<f64> {
        match m {
            Measure::Cost => &mut self.cost_usd,
            Measure::Duration => &mut self.duration_ms,
            Measure::ApiDuration => &mut self.duration_api_ms,
            Measure::Turns => &mut self.turns,
            Measure::PermissionDenials => &mut self.permission_denials,
            Measure::InputTokens => &mut self.input_tokens,
            Measure::CacheWrites => &mut self.cache_creation_tokens,
            Measure::CacheReads => &mut self.cache_read_tokens,
            Measure::OutputTokens => &mut self.output_tokens,
            Measure::Calls => &mut self.calls,
            Measure::InputPerCall => &mut self.input_per_call,
            Measure::OutputPerCall => &mut self.output_per_call,
        }
    }

    /// One run's measures.
    fn of(c: &CaseScore) -> Measures {
        let consumption = c.consumption.as_ref();
        let tokens = consumption.and_then(|c| c.tokens);
        let f = |v: Option<u64>| v.map(|v| v as f64);
        let calls = consumption
            .map(|c| c.calls.len() as f64)
            .filter(|n| *n > 0.0);
        let per_call = |tokens: Option<f64>| match (tokens, calls) {
            (Some(t), Some(n)) => Some(t / n),
            _ => None,
        };
        // The per-call measures count the session's own calls: the
        // sub-agents' tokens, in the totals, are not theirs.
        let own = tokens.map(|t| {
            let sub = consumption.and_then(|c| c.sub_agents).unwrap_or_default();
            crate::observed::Tokens {
                input: t.input.saturating_sub(sub.input),
                cache_creation: t.cache_creation.saturating_sub(sub.cache_creation),
                cache_read: t.cache_read.saturating_sub(sub.cache_read),
                output: t.output.saturating_sub(sub.output),
            }
        });
        Measures {
            cost_usd: c.cost_usd,
            duration_ms: f(c.duration_ms),
            duration_api_ms: f(consumption.and_then(|c| c.duration_api_ms)),
            turns: f(consumption.and_then(|c| c.turns)),
            permission_denials: f(consumption.and_then(|c| c.permission_denials)),
            input_tokens: f(tokens.map(|t| t.input)),
            cache_creation_tokens: f(tokens.map(|t| t.cache_creation)),
            cache_read_tokens: f(tokens.map(|t| t.cache_read)),
            output_tokens: f(tokens.map(|t| t.output)),
            calls,
            input_per_call: per_call(f(own.map(|t| t.input + t.cache_creation + t.cache_read))),
            output_per_call: per_call(f(own.map(|t| t.output))),
        }
    }

    /// The median of each measure over the runs that report it.
    pub fn median_of(runs: &[Measures]) -> Measures {
        let mut out = Measures::default();
        for m in Measure::ALL {
            let present: Vec<f64> = runs.iter().filter_map(|r| r.get(m)).collect();
            *out.slot(m) = median(&present);
        }
        out
    }

    /// The sum of each measure over the items that report it; a per-call
    /// measure, which does not add up, is the items' mean weighted by
    /// their calls: their tokens divided by their calls.
    pub fn sum_of<'a>(items: impl IntoIterator<Item = &'a Measures>) -> Measures {
        let mut out = Measures::default();
        // Per-call measures: their tokens and their calls, summed apart.
        let (mut input, mut output) = ((0.0, 0.0), (0.0, 0.0));
        for item in items {
            for m in Measure::ALL {
                if let Some(v) = item.get(m) {
                    match (m, item.calls) {
                        (Measure::InputPerCall | Measure::OutputPerCall, Some(calls)) => {
                            let (tokens, all_calls) = match m {
                                Measure::InputPerCall => &mut input,
                                _ => &mut output,
                            };
                            *tokens += v * calls;
                            *all_calls += calls;
                        }
                        (Measure::InputPerCall | Measure::OutputPerCall, None) => {}
                        _ => {
                            let slot = out.slot(m);
                            *slot = Some(round4(slot.unwrap_or(0.0) + v));
                        }
                    }
                }
            }
        }
        let mean = |(tokens, calls): (f64, f64)| (calls > 0.0).then(|| (tokens / calls).round());
        out.input_per_call = mean(input);
        out.output_per_call = mean(output);
        out
    }
}

/// A case's consumption: the medians over its repetitions not in error.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CaseConsumption {
    pub id: String,
    pub role: String,
    #[serde(default)]
    pub step: String,
    pub repetitions: u32,
    pub measures: Measures,
    /// The tokens written to the cache by source: per source, the median
    /// over the runs that record their calls.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub sources: BTreeMap<String, f64>,
}

/// The source of the tokens an isolated run's sub-agents wrote to the
/// cache: what the session's totals hold beyond its own calls.
pub const SUB_AGENTS_SOURCE: &str = "sub-agents";

/// Where the tokens go: one source, the sum of its per-case median
/// tokens written to the cache, and its share of what those writes cost.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceShare {
    pub source: String,
    pub tokens: f64,
    pub share: f64,
}

/// The sources of `cases`, ranked from the most to the least costly. A
/// token written to the cache costs the same whatever wrote it, so the
/// share of the tokens is the share of the cost.
pub fn rank_sources<'a>(cases: impl IntoIterator<Item = &'a CaseConsumption>) -> Vec<SourceShare> {
    let mut sums: BTreeMap<&str, f64> = BTreeMap::new();
    for c in cases {
        for (source, tokens) in &c.sources {
            *sums.entry(source.as_str()).or_default() += tokens;
        }
    }
    let total: f64 = sums.values().sum();
    let mut ranked: Vec<SourceShare> = sums
        .into_iter()
        .filter(|(_, tokens)| *tokens > 0.0)
        .map(|(source, tokens)| SourceShare {
            source: source.to_string(),
            tokens: tokens.round(),
            share: round3(tokens / total),
        })
        .collect();
    ranked.sort_by(|a, b| b.tokens.total_cmp(&a.tokens).then(a.source.cmp(&b.source)));
    ranked
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SuiteScore {
    pub format: u32,
    pub run: RunInfo,
    pub thresholds: Thresholds,
    pub pass: bool,
    /// The highest repetition number scored.
    pub repetitions: u32,
    /// The sum of every case's median cost: what one run of the suite
    /// costs.
    pub cost_usd: f64,
    /// What every run cost, every repetition included.
    pub total_cost_usd: f64,
    pub figures: SuiteFigures,
    /// The sums of every case's median consumption.
    pub consumption: Measures,
    /// Every reason the suite fails: role measures under their threshold,
    /// runs that stopped.
    pub failures: Vec<String>,
    pub roles: Vec<RoleScore>,
    /// Every run, by case then repetition.
    pub cases: Vec<CaseScore>,
    /// Every case's consumption, by case.
    pub case_consumption: Vec<CaseConsumption>,
    /// Where the tokens go, for the whole suite; empty when no run recorded
    /// its calls.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sources: Vec<SourceShare>,
    /// Where the tokens go, per step.
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub sources_by_step: BTreeMap<String, Vec<SourceShare>>,
    /// The step `plan.write`, when the suite has planning runs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub planning: Option<PlanningScore>,
    /// The step `plan.update`, when the suite has update runs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update: Option<UpdateStepScore>,
}

/// One repetition of the planning runs, pooled as a role's tally is.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlanRepetition {
    pub repetition: u32,
    pub runs: u32,
    pub primary_recall: Option<f64>,
    pub collateral_recall: Option<f64>,
    pub duplicates: u32,
    pub decisions_ignored: u32,
    pub invalid: u32,
}

/// The step `plan.write`: its measures per repetition, then their median
/// and range. No threshold: `compare` judges them against the baseline.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlanningScore {
    pub cases: u32,
    pub repetitions: Vec<PlanRepetition>,
    pub primary_recall: Option<Spread>,
    pub collateral_recall: Option<Spread>,
    pub duplicates: Option<Spread>,
    pub decisions_ignored: Option<Spread>,
    pub invalid: Option<Spread>,
    /// The sums of every planning case's median consumption.
    pub consumption: Measures,
}

/// Rounded to three decimals: the report shows ratios, not float noise.
pub(crate) fn ratio(numerator: u32, denominator: u32) -> Option<f64> {
    (denominator > 0)
        .then(|| (f64::from(numerator) / f64::from(denominator) * 1000.0).round() / 1000.0)
}

fn round4(value: f64) -> f64 {
    (value * 10000.0).round() / 10000.0
}

/// What a role scored in one repetition.
#[derive(Debug, Clone, Default)]
struct Tally {
    expected: u32,
    true_positives: u32,
    misses: u32,
    false_positives: u32,
    unreported: u32,
    duplicates: u32,
    cross_role: u32,
    severity_agreement: u32,
    found: u32,
    moved: u32,
    invalid: u32,
    unchecked: u32,
}

impl Tally {
    fn recall(&self) -> Option<f64> {
        ratio(self.true_positives, self.expected)
    }

    fn checked(&self) -> u32 {
        self.found + self.moved + self.invalid
    }

    fn evidence_validity(&self) -> Option<f64> {
        ratio(self.found + self.moved, self.checked())
    }
}

/// Each role's tally over the runs of one repetition.
fn tally(runs: &[&CaseScore]) -> BTreeMap<String, Tally> {
    let mut roles: BTreeMap<String, Tally> = BTreeMap::new();
    for c in runs {
        roles.entry(c.role.clone()).or_default().false_positives += c.changed.len() as u32;
        for e in &c.expected {
            let r = roles.entry(e.role.clone()).or_default();
            r.expected += 1;
            if e.matched_by.is_some() {
                r.true_positives += 1;
                r.severity_agreement += u32::from(e.severity_agrees);
            } else {
                r.misses += 1;
            }
        }
        for f in &c.findings {
            let r = roles.entry(f.role.clone()).or_default();
            match f.class {
                Class::FalsePositive => r.false_positives += 1,
                Class::Unreported => r.unreported += 1,
                Class::Duplicate => r.duplicates += 1,
                Class::CrossRole => r.cross_role += 1,
                Class::TruePositive => {}
            }
            match f.evidence {
                Evidence::Found => r.found += 1,
                Evidence::Moved => r.moved += 1,
                Evidence::Invalid => r.invalid += 1,
                Evidence::Unchecked => r.unchecked += 1,
            }
        }
    }
    roles
}

/// One repetition: its runs and each role's tally over them.
struct Repetition<'a> {
    runs: Vec<&'a CaseScore>,
    tallies: BTreeMap<String, Tally>,
}

/// Scores every run: each role per repetition, then the medians over the
/// repetitions, the thresholds applied to the medians.
pub fn suite(run: RunInfo, mut cases: Vec<CaseScore>, thresholds: Thresholds) -> SuiteScore {
    cases.sort_by(|a, b| a.id.cmp(&b.id).then(a.repetition.cmp(&b.repetition)));
    let mut numbers: Vec<u32> = cases.iter().map(|c| c.repetition).collect();
    numbers.sort_unstable();
    numbers.dedup();
    let repetitions: Vec<Repetition> = numbers
        .iter()
        .map(|n| {
            let runs: Vec<&CaseScore> = cases
                .iter()
                .filter(|c| c.repetition == *n && c.is_review())
                .collect();
            let tallies = tally(&runs);
            Repetition { runs, tallies }
        })
        .collect();
    let case_consumption = case_consumption(&cases);

    let mut role_ids: Vec<String> = repetitions
        .iter()
        .flat_map(|r| r.tallies.keys().cloned())
        .chain(
            cases
                .iter()
                .filter(|c| c.is_review())
                .map(|c| c.role.clone()),
        )
        .collect();
    role_ids.sort();
    role_ids.dedup();
    let roles: Vec<RoleScore> = role_ids
        .iter()
        .map(|id| role_score(id, &repetitions, &cases, &case_consumption, &thresholds))
        .collect();
    let mut failures: Vec<String> = roles.iter().flat_map(|r| r.failures.clone()).collect();
    for c in &cases {
        match &c.outcome {
            Outcome::Error(reason) => failures.push(format!(
                "case `{}` (repetition {}) stopped: {reason}",
                c.id, c.repetition
            )),
            Outcome::NoPlan => failures.push(format!(
                "case `{}` (repetition {}) planned nothing: no change was created",
                c.id, c.repetition
            )),
            _ => {}
        }
    }
    let planning = planning_score(&numbers, &cases, &case_consumption);
    let update = crate::update::step_score(&numbers, &cases, &case_consumption);
    let sources = rank_sources(&case_consumption);
    let mut sources_by_step = BTreeMap::new();
    for step in case_consumption.iter().map(|c| c.step.as_str()) {
        if !sources_by_step.contains_key(step) {
            let ranked = rank_sources(case_consumption.iter().filter(|c| c.step == step));
            if !ranked.is_empty() {
                sources_by_step.insert(step.to_string(), ranked);
            }
        }
    }

    let figures = suite_figures(&repetitions);
    let consumption = Measures::sum_of(case_consumption.iter().map(|c| &c.measures));
    SuiteScore {
        format: crate::observed::FORMAT,
        run,
        thresholds,
        pass: failures.is_empty(),
        repetitions: numbers.last().copied().unwrap_or(0),
        cost_usd: consumption.cost_usd.unwrap_or(0.0),
        total_cost_usd: round4(cases.iter().filter_map(|c| c.cost_usd).sum()),
        figures,
        consumption,
        failures,
        roles,
        cases,
        case_consumption,
        sources,
        sources_by_step,
        planning,
        update,
    }
}

/// The planning section: each repetition's runs pooled, then medians and
/// ranges; `None` without a planning run.
fn planning_score(
    numbers: &[u32],
    cases: &[CaseScore],
    case_consumption: &[CaseConsumption],
) -> Option<PlanningScore> {
    let planning: Vec<&CaseScore> = cases.iter().filter(|c| c.is_planning()).collect();
    if planning.is_empty() {
        return None;
    }
    let repetitions: Vec<PlanRepetition> = numbers
        .iter()
        .filter_map(|n| {
            let plans: Vec<&PlanScore> = planning
                .iter()
                .filter(|c| c.repetition == *n)
                .filter_map(|c| c.plan.as_ref())
                .collect();
            if plans.is_empty() {
                return None;
            }
            let count =
                |f: fn(&PlanScore) -> usize| plans.iter().map(|p| f(p)).sum::<usize>() as u32;
            let pooled = |found: u32, missed: u32| ratio(found, found + missed);
            Some(PlanRepetition {
                repetition: *n,
                runs: plans.len() as u32,
                primary_recall: pooled(
                    count(|p| p.primary_found.len()),
                    count(|p| p.primary_missed.len()),
                ),
                collateral_recall: pooled(
                    count(|p| p.collateral_found.len()),
                    count(|p| p.collateral_missed.len()),
                ),
                duplicates: count(|p| p.duplicates.len()),
                decisions_ignored: count(|p| p.decisions_ignored.len()),
                invalid: count(|p| usize::from(!p.valid)),
            })
        })
        .collect();
    let values = |f: fn(&PlanRepetition) -> Option<f64>| -> Vec<f64> {
        repetitions.iter().filter_map(f).collect()
    };
    let mut distinct: Vec<&str> = planning.iter().map(|c| c.id.as_str()).collect();
    distinct.dedup();
    Some(PlanningScore {
        cases: distinct.len() as u32,
        primary_recall: spread(&values(|r| r.primary_recall)),
        collateral_recall: spread(&values(|r| r.collateral_recall)),
        duplicates: spread(&values(|r| Some(f64::from(r.duplicates)))),
        decisions_ignored: spread(&values(|r| Some(f64::from(r.decisions_ignored)))),
        invalid: spread(&values(|r| Some(f64::from(r.invalid)))),
        consumption: Measures::sum_of(
            case_consumption
                .iter()
                .filter(|c| distinct.contains(&c.id.as_str()))
                .map(|c| &c.measures),
        ),
        repetitions,
    })
}

/// Each case's consumption: the medians over its runs not in error.
/// `cases` come sorted by case id.
fn case_consumption(cases: &[CaseScore]) -> Vec<CaseConsumption> {
    cases
        .chunk_by(|a, b| a.id == b.id)
        .map(|runs| {
            let measured: Vec<Measures> = runs
                .iter()
                .filter(|r| !matches!(r.outcome, Outcome::Error(_)))
                .map(Measures::of)
                .collect();
            let traced: Vec<BTreeMap<String, u64>> = runs
                .iter()
                .filter(|r| !matches!(r.outcome, Outcome::Error(_)))
                .filter_map(|r| r.consumption.as_ref())
                .filter(|c| !c.calls.is_empty())
                .map(|c| {
                    let mut by_source: BTreeMap<String, u64> = BTreeMap::new();
                    for call in &c.calls {
                        for (source, tokens) in &call.sources {
                            *by_source.entry(source.clone()).or_default() += tokens;
                        }
                    }
                    if let Some(sub) = c.sub_agents {
                        *by_source.entry(SUB_AGENTS_SOURCE.into()).or_default() +=
                            sub.cache_creation;
                    }
                    by_source
                })
                .collect();
            let mut names: Vec<&String> = traced.iter().flat_map(|s| s.keys()).collect();
            names.sort();
            names.dedup();
            let sources = names
                .into_iter()
                .filter_map(|name| {
                    let values: Vec<f64> = traced
                        .iter()
                        .map(|s| s.get(name).copied().unwrap_or(0) as f64)
                        .collect();
                    median(&values).map(|m| (name.clone(), m))
                })
                .collect();
            CaseConsumption {
                id: runs[0].id.clone(),
                role: runs[0].role.clone(),
                step: runs[0].step.clone(),
                repetitions: runs.len() as u32,
                measures: Measures::median_of(&measured),
                sources,
            }
        })
        .collect()
}

/// A role's medians and ranges over the repetitions in which it has a
/// case, its consumption, and the thresholds it fails.
fn role_score(
    id: &str,
    repetitions: &[Repetition],
    cases: &[CaseScore],
    case_consumption: &[CaseConsumption],
    thresholds: &Thresholds,
) -> RoleScore {
    let tallies: Vec<&Tally> = repetitions
        .iter()
        .filter_map(|r| r.tallies.get(id))
        .collect();
    // The median over the repetitions of one count of the tally.
    let median_of = |count: fn(&Tally) -> u32| -> f64 {
        let values: Vec<f64> = tallies.iter().map(|t| f64::from(count(t))).collect();
        median(&values).unwrap_or(0.0)
    };
    let recalls: Vec<f64> = tallies.iter().filter_map(|t| t.recall()).collect();
    let validities: Vec<f64> = tallies
        .iter()
        .filter_map(|t| t.evidence_validity())
        .collect();
    let fps: Vec<f64> = tallies
        .iter()
        .map(|t| f64::from(t.false_positives))
        .collect();
    let mut distinct: Vec<&CaseScore> = cases.iter().filter(|c| c.role == id).collect();
    distinct.dedup_by(|a, b| a.id == b.id);
    let seeded = distinct
        .iter()
        .filter(|c| c.kind == Kind::Seeded.as_str())
        .count() as u32;
    let consumption = Measures::sum_of(
        case_consumption
            .iter()
            .filter(|c| c.role == id)
            .map(|c| &c.measures),
    );
    let mut r = RoleScore {
        role: id.to_string(),
        cases: distinct.len() as u32,
        seeded,
        controls: distinct.len() as u32 - seeded,
        repetitions: tallies.len() as u32,
        expected: median_of(|t| t.expected),
        true_positives: median_of(|t| t.true_positives),
        misses: median_of(|t| t.misses),
        false_positives: median_of(|t| t.false_positives),
        unreported: median_of(|t| t.unreported),
        duplicates: median_of(|t| t.duplicates),
        cross_role: median_of(|t| t.cross_role),
        severity_agreement: median_of(|t| t.severity_agreement),
        evidence: EvidenceCounts {
            found: median_of(|t| t.found),
            moved: median_of(|t| t.moved),
            invalid: median_of(|t| t.invalid),
            unchecked: median_of(|t| t.unchecked),
        },
        recall: median(&recalls).map(round3),
        evidence_validity: median(&validities).map(round3),
        ranges: RoleRanges {
            recall: spread(&recalls),
            false_positives: spread(&fps),
            evidence_validity: spread(&validities),
        },
        cost_usd: consumption.cost_usd.unwrap_or(0.0),
        consumption,
        failures: Vec::new(),
    };
    r.failures = threshold_failures(&r, thresholds);
    r
}

/// The thresholds a role's medians fail, in the report's order.
fn threshold_failures(r: &RoleScore, thresholds: &Thresholds) -> Vec<String> {
    let mut out = Vec::new();
    if let Some(recall) = r.recall.filter(|v| *v < thresholds.min_recall) {
        out.push(format!(
            "{}: recall {recall:.2} is under {:.2}",
            r.role, thresholds.min_recall
        ));
    }
    if r.false_positives > f64::from(thresholds.max_false_positives) {
        out.push(format!(
            "{}: {} false positive(s) exceed {}",
            r.role,
            number(r.false_positives),
            thresholds.max_false_positives
        ));
    }
    if let Some(validity) = r.evidence_validity.filter(|v| *v < thresholds.min_evidence) {
        out.push(format!(
            "{}: evidence validity {validity:.2} is under {:.2}",
            r.role, thresholds.min_evidence
        ));
    }
    out
}

/// The suite-level figures: for each repetition, the tally of all its
/// roles together, then the median and range over the repetitions.
fn suite_figures(repetitions: &[Repetition]) -> SuiteFigures {
    let totals: Vec<(Tally, usize)> = repetitions
        .iter()
        .map(|r| {
            let mut all = Tally::default();
            for t in r.tallies.values() {
                all.expected += t.expected;
                all.true_positives += t.true_positives;
                all.false_positives += t.false_positives;
                all.found += t.found;
                all.moved += t.moved;
                all.invalid += t.invalid;
            }
            (all, r.runs.len())
        })
        .collect();
    let values = |f: fn(&Tally) -> Option<f64>| -> Vec<f64> {
        totals.iter().filter_map(|(t, _)| f(t)).collect()
    };
    let per_case: Vec<f64> = totals
        .iter()
        .filter(|(_, runs)| *runs > 0)
        .map(|(t, runs)| f64::from(t.false_positives) / *runs as f64)
        .collect();
    SuiteFigures {
        expected: median(&values(|t| Some(f64::from(t.expected)))).unwrap_or(0.0),
        true_positives: spread(&values(|t| Some(f64::from(t.true_positives)))),
        recall: spread(&values(Tally::recall)),
        checked: median(&values(|t| Some(f64::from(t.checked())))).unwrap_or(0.0),
        valid: median(&values(|t| Some(f64::from(t.found + t.moved)))).unwrap_or(0.0),
        evidence_validity: spread(&values(Tally::evidence_validity)),
        false_positives: spread(&values(|t| Some(f64::from(t.false_positives)))),
        false_positives_per_case: spread(&per_case),
    }
}

fn round3(value: f64) -> f64 {
    (value * 1000.0).round() / 1000.0
}

/// A median count: whole numbers without decimals, halves with one.
pub fn number(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{value:.0}")
    } else {
        format!("{value}")
    }
}

/// 0 when every role passes and no run stopped, 1 otherwise.
pub fn exit_code(score: &SuiteScore) -> i32 {
    if score.pass { 0 } else { 1 }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::case::{Expected, FixedCheck, UnchangedCheck};
    use crate::observed::FORMAT;
    use serde_json::json;

    pub(crate) fn a_case(role: &str, kind: Kind, expected: &[(&str, &[&str])]) -> Case {
        Case {
            id: format!("{role}-case"),
            role: role.into(),
            step: Step::CodeReview,
            change: "add-search".into(),
            stacks: vec!["typescript".into()],
            kind,
            summary: String::new(),
            expected: expected
                .iter()
                .map(|(id, locations)| Expected {
                    id: (*id).into(),
                    role: role.into(),
                    severity: "high".into(),
                    category: "c".into(),
                    terms: vec!["defect".into()],
                    locations: locations.iter().map(|l| Location::parse(l)).collect(),
                    alternatives: vec![],
                })
                .collect(),
            fixed: vec![],
            unchanged: vec![],
            planned: vec![],
            plan: None,
            update: None,
        }
    }

    /// A finding: (id, role, location, confidence), titled after the
    /// test cases' category term, `defect`.
    pub(crate) fn observed(
        state: &str,
        findings: &[(&str, &str, &str, &str)],
        issues: serde_json::Value,
    ) -> Observed {
        let titled: Vec<_> = findings
            .iter()
            .map(|(id, role, location, confidence)| {
                (*id, *role, *location, *confidence, "A defect")
            })
            .collect();
        observed_titled(state, &titled, issues)
    }

    /// A finding: (id, role, location, confidence, title).
    pub(crate) fn observed_titled(
        state: &str,
        findings: &[(&str, &str, &str, &str, &str)],
        issues: serde_json::Value,
    ) -> Observed {
        let findings: Vec<_> = findings
            .iter()
            .map(|(id, role, location, confidence, title)| {
                json!({"id": id, "title": title, "role": role, "step": "code.review",
                       "severity": "high", "confidence": confidence,
                       "location": location, "status": "open",
                       "recommendation": "Fix it."})
            })
            .collect();
        let roles: Vec<_> = ["security", "privacy"]
            .iter()
            .map(|r| json!({"role": r, "state": state}))
            .collect();
        Observed {
            format: FORMAT,
            case: "c".into(),
            error: None,
            cost_usd: Some(0.5),
            duration_ms: Some(1000),
            consumption: None,
            strayed: vec![],
            review: Some(json!({"exists": true,
                "steps": [{"step": "code.review", "recorded": true, "roles": roles}],
                "findings": findings, "issues": issues})),
            files: BTreeMap::new(),
            validate: BTreeMap::new(),
            validate_before: None,
            messages: Vec::new(),
        }
    }

    fn a_planning_case(id: &str, modified: &[&str]) -> Case {
        let mut c = a_case("planner", Kind::Seeded, &[]);
        c.id = id.into();
        c.step = Step::PlanWrite;
        c.plan = Some(crate::case::PlanCase {
            git_ref: "abc".into(),
            request: "r".into(),
            modified: modified
                .iter()
                .map(|m| crate::case::RequirementId::parse(m).unwrap())
                .collect(),
            collateral: vec![],
            decisions: vec![],
        });
        c
    }

    /// A planning run whose delta of `cap` modifies `modified`; no change
    /// at all when `modified` is `None`.
    fn a_planning_run(id: &str, cap: &str, modified: Option<&str>) -> Observed {
        let mut o = observed("findings", &[], json!([]));
        o.case = id.into();
        o.review = None;
        if let Some(name) = modified {
            o.files.insert(
                format!("_codev/changes/c/specs/{cap}/spec.md"),
                Some(format!(
                    "## MODIFIED Requirements\n\n### Requirement: {name}\n\nIt SHALL.\n\n#### Scenario: S\n\n- **WHEN** w\n- **THEN** t\n"
                )),
            );
            o.validate
                .insert("c".into(), json!({"items": [{"findings": []}]}));
        }
        o
    }

    /// A call: cache writes, cache reads, uncached input, output, and its
    /// writes by source.
    type TestCall<'a> = (u64, u64, u64, u64, &'a [(&'a str, u64)]);

    fn with_calls(mut o: Observed, calls: &[TestCall]) -> Observed {
        use crate::observed::{Call, Tokens};
        let calls: Vec<Call> = calls
            .iter()
            .map(
                |(cache_creation, cache_read, input, output, sources)| Call {
                    cache_creation: *cache_creation,
                    cache_read: *cache_read,
                    input: *input,
                    output: *output,
                    tool_calls: 1,
                    sources: sources.iter().map(|(s, n)| (s.to_string(), *n)).collect(),
                },
            )
            .collect();
        let tokens = calls.iter().fold(Tokens::default(), |t, c| Tokens {
            input: t.input + c.input,
            cache_creation: t.cache_creation + c.cache_creation,
            cache_read: t.cache_read + c.cache_read,
            output: t.output + c.output,
        });
        o.consumption = Some(Consumption {
            tokens: Some(tokens),
            calls,
            ..Consumption::default()
        });
        o
    }

    #[test]
    fn input_and_output_per_call_are_medians_per_case() {
        let c = a_case(
            "security",
            Kind::Seeded,
            &[("sqli", &["src/users.ts:12-14"])],
        );
        let run = |input_per_call: u64, output: u64| {
            with_calls(
                observed("no-finding", &[], json!([])),
                &[
                    (input_per_call, 0, 0, output, &[]),
                    (0, input_per_call, 0, output, &[]),
                ],
            )
        };
        let scores = vec![
            case(&c, &run(30_000, 100)).at(1),
            case(&c, &run(38_000, 300)).at(2),
            case(&c, &run(90_000, 200)).at(3),
        ];
        let s = suite(super::tests::run(), scores, Thresholds::default());
        let m = &s.case_consumption[0].measures;
        assert_eq!(m.calls, Some(2.0));
        assert_eq!(m.input_per_call, Some(38_000.0));
        assert_eq!(m.output_per_call, Some(200.0));
        // The suite's figure is per call too, never a sum of per-call figures.
        assert_eq!(s.consumption.input_per_call, Some(38_000.0));
        assert_eq!(s.roles[0].consumption.output_per_call, Some(200.0));
    }

    #[test]
    fn the_sources_are_ranked_by_cost() {
        let review = a_case(
            "security",
            Kind::Seeded,
            &[("sqli", &["src/users.ts:12-14"])],
        );
        let traced = with_calls(
            observed("no-finding", &[], json!([])),
            &[
                (100_000, 0, 0, 10, &[("prompt", 0), ("file", 15_000)]),
                (0, 0, 0, 10, &[("skill", 60_000), ("codev roles", 25_000)]),
            ],
        );
        let s = suite(run(), vec![case(&review, &traced)], Thresholds::default());
        let ranked: Vec<(&str, f64, f64)> = s
            .sources
            .iter()
            .map(|r| (r.source.as_str(), r.tokens, r.share))
            .collect();
        assert_eq!(
            ranked,
            [
                ("skill", 60_000.0, 0.6),
                ("codev roles", 25_000.0, 0.25),
                ("file", 15_000.0, 0.15)
            ]
        );
        assert_eq!(s.sources_by_step["code.review"], s.sources);
        let md = crate::report::markdown(&s);
        let at = |needle: &str| {
            md.find(needle)
                .unwrap_or_else(|| panic!("{needle} in {md}"))
        };
        assert!(at("## Where the tokens go") < at("| skill | 60000 | 60 % |"));
        assert!(at("| skill |") < at("| codev roles |") && at("| codev roles |") < at("| file |"));
    }

    #[test]
    fn review_and_planning_runs_are_scored_apart() {
        let review = a_case(
            "security",
            Kind::Seeded,
            &[("sqli", &["src/users.ts:12-14"])],
        );
        let found = observed(
            "findings",
            &[("R-security-1", "security", "src/users.ts:12", "high")],
            json!([]),
        );
        let plan_a = a_planning_case("plan-a", &["d::One", "d::Two"]);
        let plan_b = a_planning_case("plan-b", &["d::Three"]);
        let scores = vec![
            case(&review, &found),
            plan_case(
                &plan_a,
                &a_planning_run("plan-a", "d", Some("One")),
                &AtRef::default(),
            ),
            plan_case(
                &plan_b,
                &a_planning_run("plan-b", "d", Some("Three")),
                &AtRef::default(),
            ),
        ];
        let s = suite(run(), scores, Thresholds::default());
        assert_eq!(
            s.roles.iter().map(|r| r.role.as_str()).collect::<Vec<_>>(),
            ["security"]
        );
        assert_eq!(s.roles[0].recall, Some(1.0));
        assert_eq!(s.figures.expected, 1.0);
        let planning = s.planning.expect("a planning section");
        assert_eq!(planning.cases, 2);
        // Two of the three expected requirements, pooled over the runs.
        assert_eq!(planning.primary_recall.unwrap().median, 0.667);
        assert_eq!(planning.invalid.unwrap().median, 0.0);
        assert!(s.pass, "{:?}", s.failures);
    }

    #[test]
    fn a_suite_of_review_cases_has_no_planning_section() {
        let review = a_case(
            "security",
            Kind::Seeded,
            &[("sqli", &["src/users.ts:12-14"])],
        );
        let s = suite(
            run(),
            vec![case(&review, &observed("no-finding", &[], json!([])))],
            Thresholds::default(),
        );
        assert!(s.planning.is_none());
        assert!(serde_json::to_value(&s).unwrap().get("planning").is_none());
    }

    #[test]
    fn a_no_plan_run_fails_the_suite() {
        let plan = a_planning_case("plan-a", &["d::One"]);
        let score = plan_case(
            &plan,
            &a_planning_run("plan-a", "d", None),
            &AtRef::default(),
        )
        .at(2);
        assert_eq!(score.outcome, Outcome::NoPlan);
        let s = suite(run(), vec![score], Thresholds::default());
        assert!(!s.pass);
        assert_eq!(exit_code(&s), 1);
        assert!(
            s.failures
                .iter()
                .any(|f| f.contains("plan-a") && f.contains("repetition 2")),
            "{:?}",
            s.failures
        );
        let planning = s.planning.unwrap();
        assert_eq!(planning.primary_recall.unwrap().median, 0.0);
        assert_eq!(planning.invalid.unwrap().median, 1.0);
    }

    fn classes(score: &CaseScore) -> Vec<(String, Class)> {
        score
            .findings
            .iter()
            .map(|f| (f.id.clone(), f.class.clone()))
            .collect()
    }

    #[test]
    fn overlapping_lines_match() {
        let c = a_case(
            "security",
            Kind::Seeded,
            &[("sqli", &["src/users.ts:12-14"])],
        );
        let o = observed(
            "findings",
            &[("R-security-1", "security", "src/users.ts:15", "high")],
            json!([]),
        );
        let s = case(&c, &o);
        assert_eq!(s.outcome, Outcome::Scored);
        assert_eq!(s.expected[0].matched_by.as_deref(), Some("R-security-1"));
        assert!(s.expected[0].severity_agrees);
        assert_eq!(classes(&s), [("R-security-1".into(), Class::TruePositive)]);
    }

    #[test]
    fn lines_too_far_away_are_a_miss_and_a_false_positive() {
        let c = a_case(
            "security",
            Kind::Seeded,
            &[("sqli", &["src/users.ts:12-14"])],
        );
        let o = observed(
            "findings",
            &[("R-security-1", "security", "src/users.ts:30", "high")],
            json!([]),
        );
        let s = case(&c, &o);
        assert_eq!(s.expected[0].matched_by, None);
        assert_eq!(classes(&s), [("R-security-1".into(), Class::FalsePositive)]);
    }

    #[test]
    fn plan_headings_match_ignoring_case_and_spacing() {
        let c = a_case(
            "security",
            Kind::Seeded,
            &[("clear", &["design.md#Decision: Tokens are stored as given"])],
        );
        let o = observed(
            "findings",
            &[(
                "R-security-1",
                "security",
                "design.md#decision:  tokens are stored as given",
                "high",
            )],
            json!([]),
        );
        assert!(case(&c, &o).expected[0].matched_by.is_some());
    }

    #[test]
    fn a_windows_path_matches() {
        let c = a_case(
            "security",
            Kind::Seeded,
            &[("n1", &["src/db/orders.rs:20-24"])],
        );
        let o = observed(
            "findings",
            &[("R-security-1", "security", "src\\db\\orders.rs:22", "high")],
            json!([]),
        );
        assert!(case(&c, &o).expected[0].matched_by.is_some());
    }

    #[test]
    fn a_location_without_heading_matches_the_whole_file() {
        let c = a_case("security", Kind::Seeded, &[("x", &["proposal.md"])]);
        let o = observed(
            "findings",
            &[("R-security-1", "security", "proposal.md#Impact", "high")],
            json!([]),
        );
        assert!(case(&c, &o).expected[0].matched_by.is_some());
    }

    #[test]
    fn a_control_counts_high_confidence_findings_as_false_positives() {
        let c = a_case("privacy", Kind::Control, &[]);
        let o = observed(
            "findings",
            &[
                ("R-privacy-1", "privacy", "src/a.ts:3", "high"),
                ("R-privacy-2", "privacy", "src/a.ts:9", "low"),
            ],
            json!([]),
        );
        let s = case(&c, &o);
        assert_eq!(
            classes(&s),
            [
                ("R-privacy-1".into(), Class::FalsePositive),
                ("R-privacy-2".into(), Class::Unreported)
            ]
        );
        let suite = suite(run(), vec![s], Thresholds::default());
        assert_eq!(suite.roles[0].false_positives, 1.0);
        assert_eq!(suite.roles[0].unreported, 1.0);
    }

    #[test]
    fn the_same_defect_twice_is_one_true_positive_and_one_duplicate() {
        let c = a_case(
            "security",
            Kind::Seeded,
            &[("sqli", &["src/users.ts:12-14"])],
        );
        let o = observed(
            "findings",
            &[
                ("R-security-1", "security", "src/users.ts:12", "high"),
                ("R-security-2", "security", "src/users.ts:13-14", "high"),
            ],
            json!([]),
        );
        let s = case(&c, &o);
        assert_eq!(
            classes(&s),
            [
                ("R-security-1".into(), Class::TruePositive),
                ("R-security-2".into(), Class::Duplicate)
            ]
        );
    }

    #[test]
    fn a_finding_matching_another_roles_defect_is_cross_role() {
        let c = a_case(
            "security",
            Kind::Seeded,
            &[("sqli", &["src/users.ts:12-14"])],
        );
        let o = observed(
            "findings",
            &[("R-privacy-1", "privacy", "src/users.ts:13", "high")],
            json!([]),
        );
        let s = case(&c, &o);
        assert_eq!(classes(&s), [("R-privacy-1".into(), Class::CrossRole)]);
        assert_eq!(s.expected[0].matched_by, None, "a miss for security");
    }

    #[test]
    fn evidence_not_found_is_invalid() {
        let c = a_case(
            "security",
            Kind::Seeded,
            &[("sqli", &["src/users.ts:12-14"])],
        );
        let o = observed(
            "findings",
            &[("R-security-1", "security", "src/users.ts:12", "high")],
            json!([{"severity": "error", "code": "review_evidence_not_found",
                    "message": "R-security-1: the evidence is not in `src/users.ts:12`"}]),
        );
        let s = case(&c, &o);
        assert_eq!(s.findings[0].evidence, Evidence::Invalid);
        assert_eq!(s.outcome, Outcome::Scored);
        assert!(s.record_errors.is_empty(), "evidence is not a record error");
        let suite = suite(run(), vec![s], Thresholds::default());
        assert_eq!(suite.roles[0].evidence_validity, Some(0.0));
    }

    #[test]
    fn evidence_moved_counts_as_valid() {
        let c = a_case(
            "security",
            Kind::Seeded,
            &[("sqli", &["src/users.ts:12-14"])],
        );
        let o = observed(
            "findings",
            &[
                ("R-security-1", "security", "src/users.ts:12", "high"),
                ("R-security-10", "security", "src/users.ts:40", "high"),
            ],
            json!([{"severity": "warning", "code": "review_evidence_moved",
                    "message": "R-security-1: the evidence is now at line 20"}]),
        );
        let s = case(&c, &o);
        assert_eq!(s.findings[0].evidence, Evidence::Moved);
        assert_eq!(
            s.findings[1].evidence,
            Evidence::Found,
            "R-security-10 is not R-security-1"
        );
        let suite = suite(run(), vec![s], Thresholds::default());
        assert_eq!(suite.roles[0].evidence_validity, Some(1.0));
    }

    #[test]
    fn a_disabled_evaluated_role_is_role_not_run() {
        let c = a_case(
            "security",
            Kind::Seeded,
            &[("sqli", &["src/users.ts:12-14"])],
        );
        let s = case(&c, &observed("disabled", &[], json!([])));
        assert_eq!(s.outcome, Outcome::RoleNotRun("disabled".into()));
        assert_eq!(s.expected[0].matched_by, None);
        let control = a_case("security", Kind::Control, &[]);
        assert_eq!(
            case(&control, &observed("missing", &[], json!([])))
                .outcome
                .label(),
            "role-not-run",
            "never a clean control"
        );
    }

    #[test]
    fn a_missing_record_misses_every_expectation() {
        let c = a_case(
            "security",
            Kind::Seeded,
            &[("sqli", &["src/users.ts:12-14"])],
        );
        let mut o = observed("findings", &[], json!([]));
        o.review = Some(json!({"exists": false}));
        let s = case(&c, &o);
        assert_eq!(s.outcome, Outcome::NoRecord);
        assert_eq!(s.expected.len(), 1);
        let mut o = observed(
            "missing",
            &[],
            json!([{"severity": "error", "code": "review_invalid",
                    "message": "R-security-1: the block is not valid YAML"}]),
        );
        o.review.as_mut().unwrap()["findings"] = json!([]);
        assert_eq!(case(&c, &o).outcome.label(), "record-invalid");
    }

    /// A case with expected findings (id, category terms, locations,
    /// alternatives).
    /// (id, category terms, locations, alternatives).
    type Seeded<'a> = (&'a str, &'a [&'a str], &'a [&'a str], &'a [&'a str]);

    fn categorized(role: &str, expected: &[Seeded]) -> Case {
        let mut c = a_case(role, Kind::Seeded, &[]);
        c.expected = expected
            .iter()
            .map(|(id, terms, locations, alternatives)| Expected {
                id: (*id).into(),
                role: role.into(),
                severity: "high".into(),
                category: (*id).into(),
                terms: terms.iter().map(|t| (*t).into()).collect(),
                locations: locations.iter().map(|l| Location::parse(l)).collect(),
                alternatives: alternatives.iter().map(|l| Location::parse(l)).collect(),
            })
            .collect();
        c
    }

    fn with_role(mut o: Observed, role: &str) -> Observed {
        o.review.as_mut().unwrap()["steps"][0]["roles"] =
            json!([{"role": role, "state": "findings"}]);
        o
    }

    #[test]
    fn the_right_place_with_another_issue_is_a_miss() {
        let c = categorized(
            "accessibility",
            &[(
                "image-without-alt",
                &["alt", "image"],
                &["src/Toolbar.tsx:12"],
                &[],
            )],
        );
        let o = with_role(
            observed_titled(
                "findings",
                &[(
                    "R-accessibility-1",
                    "accessibility",
                    "src/Toolbar.tsx:13",
                    "high",
                    "Clickable div unreachable by keyboard",
                )],
                json!([]),
            ),
            "accessibility",
        );
        let s = case(&c, &o);
        assert_eq!(s.expected[0].matched_by, None);
        assert_eq!(
            classes(&s),
            [("R-accessibility-1".into(), Class::FalsePositive)]
        );
    }

    #[test]
    fn two_close_defects_cited_in_reverse_order_both_pair() {
        let c = categorized(
            "accessibility",
            &[
                (
                    "div-button",
                    &["keyboard", "button"],
                    &["src/Toolbar.tsx:13-15"],
                    &[],
                ),
                (
                    "image-without-alt",
                    &["alt", "image"],
                    &["src/Toolbar.tsx:12"],
                    &[],
                ),
            ],
        );
        let o = with_role(
            observed_titled(
                "findings",
                &[
                    (
                        "R-accessibility-1",
                        "accessibility",
                        "src/Toolbar.tsx:13",
                        "high",
                        "Avatar image has no alt text; the keyboard order is fine",
                    ),
                    (
                        "R-accessibility-2",
                        "accessibility",
                        "src/Toolbar.tsx:14",
                        "high",
                        "Delete is a div, unreachable by keyboard",
                    ),
                ],
                json!([]),
            ),
            "accessibility",
        );
        let s = case(&c, &o);
        assert_eq!(
            s.expected[0].matched_by.as_deref(),
            Some("R-accessibility-2")
        );
        assert_eq!(
            s.expected[1].matched_by.as_deref(),
            Some("R-accessibility-1")
        );
        assert_eq!(
            classes(&s),
            [
                ("R-accessibility-1".into(), Class::TruePositive),
                ("R-accessibility-2".into(), Class::TruePositive)
            ],
            "the first finding names both categories, yet gives way so both pair"
        );
        assert_eq!(case(&c, &o), s, "the same inputs give the same pairs");
    }

    #[test]
    fn an_alternative_location_matches() {
        let c = categorized(
            "privacy",
            &[(
                "email-in-logs",
                &["log"],
                &["src/customers.ts:21"],
                &["src/errors.ts:8"],
            )],
        );
        let o = observed_titled(
            "findings",
            &[(
                "R-privacy-1",
                "privacy",
                "src/errors.ts:8",
                "high",
                "The e-mail address reaches the logs",
            )],
            json!([]),
        );
        assert_eq!(
            case(&c, &o).expected[0].matched_by.as_deref(),
            Some("R-privacy-1")
        );
    }

    #[test]
    fn another_issue_at_the_defects_place_names_it_in_the_report() {
        let c = categorized(
            "ops",
            &[(
                "unbounded-retry",
                &["retry", "retries"],
                &["app/payments.py:12-18"],
                &[],
            )],
        );
        let o = with_role(
            observed_titled(
                "findings",
                &[(
                    "R-ops-1",
                    "ops",
                    "app/payments.py:17",
                    "high",
                    "Provider URL is hard-coded",
                )],
                json!([]),
            ),
            "ops",
        );
        let s = case(&c, &o);
        assert_eq!(classes(&s), [("R-ops-1".into(), Class::FalsePositive)]);
        assert_eq!(s.findings[0].near.as_deref(), Some("unbounded-retry"));
        let report = suite(run(), vec![s], Thresholds::default());
        let md = crate::report::markdown(&report);
        assert!(
            md.contains(
                "**false positive** (at the location of `unbounded-retry`, another category)"
            ),
            "{md}"
        );
        assert!(crate::report::json(&report).contains("\"near\": \"unbounded-retry\""));
    }

    #[test]
    fn a_term_counts_only_at_the_start_of_a_word() {
        let terms = ["alt".to_string()];
        assert!(crate::case::names_any("No ALT text", &terms));
        assert!(crate::case::names_any("text alternative", &terms));
        assert!(crate::case::names_any("missing `alt`", &terms));
        assert!(!crate::case::names_any("the default health check", &terms));
    }

    fn trace_case(kind: Kind) -> Case {
        let mut c = a_case("tracer", kind, &[]);
        c.step = Step::PlanTrace;
        if kind == Kind::Seeded {
            c.fixed = vec![FixedCheck {
                path: "_codev/changes/c/tasks.md".into(),
                contains: "expired link".into(),
            }];
        } else {
            c.unchanged = vec![UnchangedCheck {
                path: "_codev/changes/c/tasks.md".into(),
                original: "- [ ] 1.1 Task\n".into(),
            }];
        }
        c
    }

    fn files(text: &str) -> Observed {
        let mut o = observed("findings", &[], json!([]));
        o.review = None;
        o.files
            .insert("_codev/changes/c/tasks.md".into(), Some(text.into()));
        o
    }

    #[test]
    fn a_fixed_check_is_a_true_positive_when_the_text_is_there() {
        let c = trace_case(Kind::Seeded);
        let s = case(&c, &files("- [ ] 2.1 Cover the Expired Link scenario\n"));
        assert!(s.expected[0].matched_by.is_some());
        let s = case(&c, &files("- [ ] 1.1 Task\n"));
        assert_eq!(s.expected[0].matched_by, None);
    }

    #[test]
    fn an_unchanged_file_that_changed_is_a_false_positive() {
        let c = trace_case(Kind::Control);
        assert!(case(&c, &files("- [ ] 1.1 Task\r\n")).changed.is_empty());
        let s = case(&c, &files("- [ ] 1.1 Task, verified by a test\n"));
        assert_eq!(s.changed, ["_codev/changes/c/tasks.md"]);
        let suite = suite(run(), vec![s], Thresholds::default());
        assert_eq!(suite.roles[0].false_positives, 1.0);
    }

    pub(crate) fn run() -> RunInfo {
        RunInfo {
            format: FORMAT,
            model: None,
            date: "2026-10-13".into(),
            claude_version: None,
            codev_version: None,
            repeat: None,
            jobs: None,
            review_mode: None,
            fixture_config: None,
        }
    }

    fn ops_scores(found: usize) -> Vec<CaseScore> {
        let locations: [&[&str]; 4] = [&["a.py:1"], &["a.py:10"], &["a.py:20"], &["a.py:30"]];
        let expected: Vec<(&str, &[&str])> = ["e1", "e2", "e3", "e4"]
            .into_iter()
            .zip(locations)
            .collect();
        let c = a_case("ops", Kind::Seeded, &expected);
        let all = [
            ("R-ops-1", "ops", "a.py:1", "high"),
            ("R-ops-2", "ops", "a.py:10", "high"),
            ("R-ops-3", "ops", "a.py:20", "high"),
            ("R-ops-4", "ops", "a.py:30", "high"),
        ];
        let mut o = observed("findings", &all[..found], json!([]));
        o.review.as_mut().unwrap()["steps"][0]["roles"] =
            json!([{"role": "ops", "state": "findings"}]);
        vec![case(&c, &o)]
    }

    #[test]
    fn every_role_passing_exits_zero() {
        let s = suite(run(), ops_scores(4), Thresholds::default());
        assert!(s.pass, "{:?}", s.failures);
        assert_eq!(exit_code(&s), 0);
        assert_eq!(s.roles[0].recall, Some(1.0));
    }

    #[test]
    fn a_role_under_the_recall_threshold_fails_with_its_name() {
        let s = suite(run(), ops_scores(1), Thresholds::default());
        assert_eq!(exit_code(&s), 1);
        assert_eq!(s.failures, ["ops: recall 0.25 is under 0.50"]);
    }

    #[test]
    fn a_timed_out_run_is_an_error_and_fails_the_suite() {
        let c = a_case(
            "security",
            Kind::Seeded,
            &[("sqli", &["src/users.ts:12-14"])],
        );
        let mut o = observed("findings", &[], json!([]));
        o.error = Some("timeout after 600 s".into());
        let s = case(&c, &o);
        assert_eq!(s.outcome, Outcome::Error("timeout after 600 s".into()));
        let lenient = Thresholds {
            min_recall: 0.0,
            ..Thresholds::default()
        };
        let suite = suite(run(), vec![s], lenient);
        assert_eq!(exit_code(&suite), 1);
        assert_eq!(
            suite.failures,
            ["case `security-case` (repetition 1) stopped: timeout after 600 s"]
        );
    }

    #[test]
    fn an_invalid_threshold_in_the_environment_is_a_usage_error() {
        let env = |name: &str| (name == "CODEV_EVAL_MIN_RECALL").then(|| "lots".to_string());
        let err = Thresholds::resolve(&ThresholdFlags::default(), &env).unwrap_err();
        assert!(
            err.contains("CODEV_EVAL_MIN_RECALL") && err.contains("lots"),
            "{err}"
        );
        let flags = ThresholdFlags {
            min_recall: Some("0.7".into()),
            ..ThresholdFlags::default()
        };
        let resolved = Thresholds::resolve(&flags, &env).unwrap();
        assert_eq!(resolved.min_recall, 0.7, "the flag wins over the variable");
        let env = |name: &str| (name == "CODEV_EVAL_MAX_FALSE_POSITIVES").then(|| "3".to_string());
        let resolved = Thresholds::resolve(&ThresholdFlags::default(), &env).unwrap();
        assert_eq!(resolved.max_false_positives, 3);
    }

    #[test]
    fn a_role_without_checked_evidence_fails_no_evidence_threshold() {
        let c = a_case("security", Kind::Control, &[]);
        let s = case(&c, &observed("no-finding", &[], json!([])));
        let suite = suite(run(), vec![s], Thresholds::default());
        assert_eq!(suite.roles[0].evidence_validity, None);
        assert_eq!(suite.roles[0].recall, None);
        assert!(suite.pass);
    }

    /// `ops_scores` runs, one per repetition, finding `found[i]` of four.
    fn repeated(found: &[usize]) -> Vec<CaseScore> {
        found
            .iter()
            .enumerate()
            .flat_map(|(i, f)| ops_scores(*f).into_iter().map(move |s| s.at(i as u32 + 1)))
            .collect()
    }

    #[test]
    fn role_measures_are_medians_over_repetitions() {
        let s = suite(run(), repeated(&[4, 2, 4]), Thresholds::default());
        let ops = &s.roles[0];
        assert_eq!(ops.recall, Some(1.0));
        let range = ops.ranges.recall.unwrap();
        assert_eq!((range.min, range.max), (0.5, 1.0));
        assert_eq!(ops.true_positives, 4.0);
        assert_eq!(ops.repetitions, 3);
        assert_eq!(ops.cases, 1, "one distinct case, run three times");
        assert!(
            s.pass,
            "the median passes a recall of 0.5: {:?}",
            s.failures
        );
        assert_eq!(s.repetitions, 3);
    }

    #[test]
    fn an_even_count_takes_the_mean_of_the_middle_values() {
        assert_eq!(median(&[1.0, 4.0, 2.0, 3.0]), Some(2.5));
        assert_eq!(median(&[]), None);
        let s = spread(&[0.5, 1.0]).unwrap();
        assert_eq!((s.median, s.min, s.max), (0.75, 0.5, 1.0));
    }

    #[test]
    fn one_repetition_in_error_fails_and_is_named() {
        let mut runs = repeated(&[4, 4, 4]);
        runs[1].outcome = Outcome::Error("timeout after 600 s".into());
        runs[1].cost_usd = Some(9.0);
        let s = suite(run(), runs, Thresholds::default());
        assert!(!s.pass);
        assert!(
            s.failures.contains(
                &"case `ops-case` (repetition 2) stopped: timeout after 600 s".to_string()
            ),
            "{:?}",
            s.failures
        );
        assert_eq!(
            s.case_consumption[0].measures.cost_usd,
            Some(0.5),
            "the run in error is left out of the median"
        );
    }

    #[test]
    fn a_single_repetition_scores_as_before() {
        let s = suite(run(), ops_scores(1), Thresholds::default());
        assert_eq!(s.repetitions, 1);
        assert_eq!(s.roles[0].recall, Some(0.25));
        let range = s.roles[0].ranges.recall.unwrap();
        assert_eq!((range.min, range.median, range.max), (0.25, 0.25, 0.25));
        assert_eq!(s.cost_usd, 0.5);
        assert_eq!(s.total_cost_usd, 0.5);
    }

    #[test]
    fn the_suite_recall_is_a_median_over_repetitions() {
        // 39 seeded defects; repetitions find 36, 38 and 39 of them.
        let expected: Vec<String> = (0..39).map(|i| format!("e{i}")).collect();
        let locations: Vec<String> = (0..39).map(|i| format!("a.py:{}", i * 10 + 1)).collect();
        let pairs: Vec<(&str, Vec<&str>)> = expected
            .iter()
            .zip(&locations)
            .map(|(e, l)| (e.as_str(), vec![l.as_str()]))
            .collect();
        let slices: Vec<(&str, &[&str])> = pairs.iter().map(|(e, l)| (*e, l.as_slice())).collect();
        let c = a_case("ops", Kind::Seeded, &slices);
        let ids: Vec<String> = (0..39).map(|i| format!("R-ops-{i}")).collect();
        let runs: Vec<CaseScore> = [36usize, 38, 39]
            .iter()
            .enumerate()
            .map(|(n, found)| {
                let findings: Vec<(&str, &str, &str, &str)> = (0..*found)
                    .map(|i| (ids[i].as_str(), "ops", locations[i].as_str(), "high"))
                    .collect();
                let mut o = observed("findings", &findings, json!([]));
                o.review.as_mut().unwrap()["steps"][0]["roles"] =
                    json!([{"role": "ops", "state": "findings"}]);
                case(&c, &o).at(n as u32 + 1)
            })
            .collect();
        let s = suite(run(), runs, Thresholds::default());
        let recall = s.figures.recall.unwrap();
        assert_eq!((recall.median, recall.min, recall.max), (0.974, 0.923, 1.0));
        assert_eq!(s.figures.expected, 39.0);
    }

    fn with_consumption(cache_read: Option<u64>) -> Consumption {
        Consumption {
            turns: Some(9),
            tokens: cache_read.map(|r| crate::observed::Tokens {
                cache_read: r,
                ..Default::default()
            }),
            ..Consumption::default()
        }
    }

    #[test]
    fn consumption_is_the_median_per_case() {
        let mut runs = repeated(&[4, 4, 4]);
        for (r, read) in runs.iter_mut().zip([300_000, 320_000, 900_000]) {
            r.consumption = Some(with_consumption(Some(read)));
        }
        let s = suite(run(), runs, Thresholds::default());
        assert_eq!(
            s.case_consumption[0].measures.cache_read_tokens,
            Some(320_000.0)
        );
        assert_eq!(s.consumption.cache_read_tokens, Some(320_000.0));
        assert_eq!(s.roles[0].consumption.turns, Some(9.0));
    }

    #[test]
    fn an_absent_measure_is_left_out_of_the_median() {
        let mut runs = repeated(&[4, 4, 4]);
        runs[0].consumption = Some(with_consumption(Some(100)));
        runs[1].consumption = Some(with_consumption(None));
        runs[2].consumption = Some(with_consumption(Some(300)));
        let s = suite(run(), runs, Thresholds::default());
        assert_eq!(
            s.case_consumption[0].measures.cache_read_tokens,
            Some(200.0)
        );
        let none = suite(run(), repeated(&[4]), Thresholds::default());
        assert_eq!(none.consumption.cache_read_tokens, None, "absent, not zero");
    }
}
