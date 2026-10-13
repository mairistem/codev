//! The scoring rules: what a run's observation says about a role. Pure and
//! deterministic — the same case and observation always give the same
//! score, so a recorded run can be scored again without a model.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::case::{Case, Expected, Kind, Location, Step};
use crate::observed::{Observed, Review, ReviewFinding, RunInfo};

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
}

impl Outcome {
    pub fn label(&self) -> &'static str {
        match self {
            Outcome::Scored => "scored",
            Outcome::Error(_) => "error",
            Outcome::NoRecord => "no-record",
            Outcome::RecordInvalid(_) => "record-invalid",
            Outcome::RoleNotRun(_) => "role-not-run",
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
}

/// Scores one case from its observation.
pub fn case(case: &Case, observed: &Observed) -> CaseScore {
    let mut score = CaseScore {
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
    };
    if case.step == Step::PlanTrace {
        trace(case, observed, &mut score);
    } else {
        review(case, observed, &mut score);
    }
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

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceCounts {
    pub found: u32,
    pub moved: u32,
    pub invalid: u32,
    pub unchecked: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleScore {
    pub role: String,
    pub cases: u32,
    pub seeded: u32,
    pub controls: u32,
    pub expected: u32,
    pub true_positives: u32,
    pub misses: u32,
    pub false_positives: u32,
    pub unreported: u32,
    pub duplicates: u32,
    pub cross_role: u32,
    pub severity_agreement: u32,
    pub evidence: EvidenceCounts,
    /// True positives over expected; `None` without expectation.
    pub recall: Option<f64>,
    /// (found + moved) over checked findings; `None` without one.
    pub evidence_validity: Option<f64>,
    pub cost_usd: f64,
    pub failures: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SuiteScore {
    pub format: u32,
    pub run: RunInfo,
    pub thresholds: Thresholds,
    pub pass: bool,
    pub cost_usd: f64,
    /// Every reason the suite fails: role measures under their threshold,
    /// cases that stopped.
    pub failures: Vec<String>,
    pub roles: Vec<RoleScore>,
    pub cases: Vec<CaseScore>,
}

/// Rounded to three decimals: the report shows ratios, not float noise.
fn ratio(numerator: u32, denominator: u32) -> Option<f64> {
    (denominator > 0)
        .then(|| (f64::from(numerator) / f64::from(denominator) * 1000.0).round() / 1000.0)
}

fn cents(value: f64) -> f64 {
    (value * 10000.0).round() / 10000.0
}

fn role_score<'a>(roles: &'a mut BTreeMap<String, RoleScore>, role: &str) -> &'a mut RoleScore {
    roles.entry(role.to_string()).or_insert_with(|| RoleScore {
        role: role.to_string(),
        cases: 0,
        seeded: 0,
        controls: 0,
        expected: 0,
        true_positives: 0,
        misses: 0,
        false_positives: 0,
        unreported: 0,
        duplicates: 0,
        cross_role: 0,
        severity_agreement: 0,
        evidence: EvidenceCounts::default(),
        recall: None,
        evidence_validity: None,
        cost_usd: 0.0,
        failures: Vec::new(),
    })
}

/// Aggregates the cases per role and applies the thresholds.
pub fn suite(run: RunInfo, mut cases: Vec<CaseScore>, thresholds: Thresholds) -> SuiteScore {
    cases.sort_by(|a, b| a.id.cmp(&b.id));
    let mut roles: BTreeMap<String, RoleScore> = BTreeMap::new();
    for c in &cases {
        let r = role_score(&mut roles, &c.role);
        r.cases += 1;
        if c.kind == Kind::Seeded.as_str() {
            r.seeded += 1;
        } else {
            r.controls += 1;
        }
        r.cost_usd += c.cost_usd.unwrap_or(0.0);
        r.false_positives += c.changed.len() as u32;
        for e in &c.expected {
            let r = role_score(&mut roles, &e.role);
            r.expected += 1;
            if e.matched_by.is_some() {
                r.true_positives += 1;
                r.severity_agreement += u32::from(e.severity_agrees);
            } else {
                r.misses += 1;
            }
        }
        for f in &c.findings {
            let r = role_score(&mut roles, &f.role);
            match f.class {
                Class::FalsePositive => r.false_positives += 1,
                Class::Unreported => r.unreported += 1,
                Class::Duplicate => r.duplicates += 1,
                Class::CrossRole => r.cross_role += 1,
                Class::TruePositive => {}
            }
            match f.evidence {
                Evidence::Found => r.evidence.found += 1,
                Evidence::Moved => r.evidence.moved += 1,
                Evidence::Invalid => r.evidence.invalid += 1,
                Evidence::Unchecked => r.evidence.unchecked += 1,
            }
        }
    }

    let mut failures = Vec::new();
    for r in roles.values_mut() {
        r.cost_usd = cents(r.cost_usd);
        r.recall = ratio(r.true_positives, r.expected);
        let checked = r.evidence.found + r.evidence.moved + r.evidence.invalid;
        r.evidence_validity = ratio(r.evidence.found + r.evidence.moved, checked);
        if let Some(recall) = r.recall.filter(|v| *v < thresholds.min_recall) {
            r.failures.push(format!(
                "{}: recall {recall:.2} is under {:.2}",
                r.role, thresholds.min_recall
            ));
        }
        if r.false_positives > thresholds.max_false_positives {
            r.failures.push(format!(
                "{}: {} false positive(s) exceed {}",
                r.role, r.false_positives, thresholds.max_false_positives
            ));
        }
        if let Some(validity) = r.evidence_validity.filter(|v| *v < thresholds.min_evidence) {
            r.failures.push(format!(
                "{}: evidence validity {validity:.2} is under {:.2}",
                r.role, thresholds.min_evidence
            ));
        }
        failures.extend(r.failures.iter().cloned());
    }
    for c in &cases {
        if let Outcome::Error(reason) = &c.outcome {
            failures.push(format!("case `{}` stopped: {reason}", c.id));
        }
    }
    let cost_usd = cents(cases.iter().filter_map(|c| c.cost_usd).sum());
    SuiteScore {
        format: crate::observed::FORMAT,
        run,
        thresholds,
        pass: failures.is_empty(),
        cost_usd,
        failures,
        roles: roles.into_values().collect(),
        cases,
    }
}

/// 0 when every role passes and no case stopped, 1 otherwise.
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
            strayed: vec![],
            review: Some(json!({"exists": true,
                "steps": [{"step": "code.review", "recorded": true, "roles": roles}],
                "findings": findings, "issues": issues})),
            files: BTreeMap::new(),
        }
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
        assert_eq!(suite.roles[0].false_positives, 1);
        assert_eq!(suite.roles[0].unreported, 1);
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
        assert_eq!(suite.roles[0].false_positives, 1);
    }

    pub(crate) fn run() -> RunInfo {
        RunInfo {
            format: FORMAT,
            model: None,
            date: "2026-10-13".into(),
            claude_version: None,
            codev_version: None,
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
            ["case `security-case` stopped: timeout after 600 s"]
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
}
