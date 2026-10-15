//! Evaluation cases: `case.yaml`, the fixture it describes, and the rules
//! a case must follow before it is worth running.
//!
//! Pure: the caller reads the files and passes their bytes, so every rule
//! is tested without a disk.

use std::collections::BTreeMap;
use std::fmt;

use serde::Deserialize;

/// A fixture file larger than this is refused: cases must stay small enough
/// for a reviewer to read whole, and cheap to run.
pub const MAX_FIXTURE_FILE_BYTES: usize = 64 * 1024;

/// Lines of tolerance on each side of an expected line range.
pub const LINE_TOLERANCE: u32 = 2;

/// The step a case runs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Step {
    PlanChallenge,
    CodeReview,
    PlanTrace,
    /// `/codev-propose` writing a plan for a request: a planning case.
    PlanWrite,
}

impl Step {
    pub fn parse(text: &str) -> Option<Step> {
        match text {
            "plan.challenge" => Some(Step::PlanChallenge),
            "code.review" => Some(Step::CodeReview),
            "plan.trace" => Some(Step::PlanTrace),
            "plan.write" => Some(Step::PlanWrite),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Step::PlanChallenge => "plan.challenge",
            Step::CodeReview => "code.review",
            Step::PlanTrace => "plan.trace",
            Step::PlanWrite => "plan.write",
        }
    }

    /// The review steps, those of a review case.
    pub fn is_review(self) -> bool {
        !matches!(self, Step::PlanWrite)
    }

    /// Whether the step writes a review record (the tracer fixes instead).
    pub fn records_findings(self) -> bool {
        matches!(self, Step::PlanChallenge | Step::CodeReview)
    }
}

/// Seeded cases hold defects; controls hold none.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Seeded,
    Control,
}

impl Kind {
    pub fn as_str(self) -> &'static str {
        match self {
            Kind::Seeded => "seeded",
            Kind::Control => "control",
        }
    }
}

/// Where a finding points, in the review record's syntax.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Location {
    /// `path:line` or `path:start-end`.
    Lines { path: String, start: u32, end: u32 },
    /// `file#heading`, or a bare `file` (any place in it).
    Section {
        file: String,
        heading: Option<String>,
    },
}

impl Location {
    /// Reads a location the way the review record writes it. Never fails:
    /// anything that is not `path:lines` or `file#heading` is a whole file.
    pub fn parse(text: &str) -> Location {
        let text = text.trim();
        if let Some((file, heading)) = text.split_once('#') {
            return Location::Section {
                file: normalize_path(file),
                heading: Some(heading.trim().to_string()),
            };
        }
        if let Some((path, lines)) = text.rsplit_once(':') {
            let range = match lines.split_once('-') {
                Some((a, b)) => a.trim().parse().ok().zip(b.trim().parse().ok()),
                None => lines.trim().parse().ok().map(|n| (n, n)),
            };
            if let Some((start, end)) = range {
                return Location::Lines {
                    path: normalize_path(path),
                    start,
                    end,
                };
            }
        }
        Location::Section {
            file: normalize_path(text),
            heading: None,
        }
    }

    pub fn file(&self) -> &str {
        match self {
            Location::Lines { path, .. } => path,
            Location::Section { file, .. } => file,
        }
    }

    /// Whether `found` (a reviewer's location) matches `self` (an expected
    /// one): same file, overlapping lines within the tolerance, or the same
    /// heading ignoring case and spacing.
    pub fn matched_by(&self, found: &Location) -> bool {
        if self.file() != found.file() {
            return false;
        }
        match (self, found) {
            (Location::Section { heading: None, .. }, _) => true,
            (
                Location::Lines { start, end, .. },
                Location::Lines {
                    start: fs, end: fe, ..
                },
            ) => {
                let low = start.saturating_sub(LINE_TOLERANCE);
                let high = end.saturating_add(LINE_TOLERANCE);
                *fs <= high && *fe >= low
            }
            (
                Location::Section {
                    heading: Some(expected),
                    ..
                },
                Location::Section {
                    heading: Some(found),
                    ..
                },
            ) => normalize_heading(expected) == normalize_heading(found),
            _ => false,
        }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Location::Lines { path, start, end } if start == end => write!(f, "{path}:{start}"),
            Location::Lines { path, start, end } => write!(f, "{path}:{start}-{end}"),
            Location::Section {
                file,
                heading: Some(h),
            } => write!(f, "{file}#{h}"),
            Location::Section {
                file,
                heading: None,
            } => write!(f, "{file}"),
        }
    }
}

/// `\` read as `/`, a leading `./` ignored: a reviewer on Windows cites the
/// same file.
pub fn normalize_path(path: &str) -> String {
    let path = path.trim().replace('\\', "/");
    path.strip_prefix("./").unwrap_or(&path).to_string()
}

/// Headings compared ignoring case and runs of whitespace.
pub fn normalize_heading(heading: &str) -> String {
    heading
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

/// A defect seeded in a case, and where a reviewer should point.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Expected {
    pub id: String,
    pub role: String,
    pub severity: String,
    pub category: String,
    /// The category's terms, lowercased, from the vocabulary.
    pub terms: Vec<String>,
    /// Where the defect is.
    pub locations: Vec<Location>,
    /// Other places a reviewer may rightly cite it.
    pub alternatives: Vec<Location>,
}

impl Expected {
    /// Whether `found` points at this defect: one of its locations or
    /// alternatives.
    pub fn located_at(&self, found: &Location) -> bool {
        self.locations
            .iter()
            .chain(&self.alternatives)
            .any(|l| l.matched_by(found))
    }

    /// Whether a finding's text names this defect's category.
    pub fn named_by(&self, text: &str) -> bool {
        names_any(text, &self.terms)
    }
}

/// Whether `text` holds one of `terms` (lowercased) at the start of a
/// word, ignoring case: `alt` is in "alt text" and "alternative", not in
/// "default".
pub fn names_any(text: &str, terms: &[String]) -> bool {
    let text = text.to_lowercase();
    terms.iter().any(|term| {
        text.match_indices(term.as_str()).any(|(at, _)| {
            text[..at]
                .chars()
                .next_back()
                .is_none_or(|c| !c.is_alphanumeric())
        })
    })
}

/// The category vocabulary: each category's terms, lowercased.
pub type Vocabulary = BTreeMap<String, Vec<String>>;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct VocabularyFile {
    categories: BTreeMap<String, CategoryFile>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CategoryFile {
    /// Required, so that the file explains every category to case
    /// authors; the scoring reads only the terms.
    #[allow(dead_code)]
    description: String,
    terms: Vec<String>,
}

/// Reads `evals/categories.yaml`; a category without term or an unknown
/// key is refused, naming it.
pub fn parse_vocabulary(text: &str) -> Result<Vocabulary, String> {
    let file: VocabularyFile =
        serde_norway::from_str(text).map_err(|e| format!("evals/categories.yaml: {e}"))?;
    let mut vocabulary = Vocabulary::new();
    for (name, category) in file.categories {
        let terms: Vec<String> = category
            .terms
            .iter()
            .map(|t| t.trim().to_lowercase())
            .filter(|t| !t.is_empty())
            .collect();
        if terms.is_empty() {
            return Err(format!(
                "evals/categories.yaml: category `{name}` has no term"
            ));
        }
        vocabulary.insert(name, terms);
    }
    Ok(vocabulary)
}

/// A `plan.trace` check: the file must contain the text after the run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixedCheck {
    pub path: String,
    pub contains: String,
}

/// A file the tracer must leave as it was, with its original content.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnchangedCheck {
    pub path: String,
    pub original: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Case {
    pub id: String,
    pub role: String,
    pub step: Step,
    pub change: String,
    pub stacks: Vec<String>,
    pub kind: Kind,
    pub summary: String,
    pub expected: Vec<Expected>,
    pub fixed: Vec<FixedCheck>,
    pub unchanged: Vec<UnchangedCheck>,
    /// Files and tests the plan introduces, absent from the fixture.
    pub planned: Vec<String>,
    /// What a planning case replays and expects; `None` for a review case.
    pub plan: Option<PlanCase>,
}

/// A requirement named as `<capability>::<requirement name>`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequirementId {
    pub capability: String,
    pub name: String,
}

impl RequirementId {
    pub fn parse(text: &str) -> Option<RequirementId> {
        let (capability, name) = text.split_once("::")?;
        let (capability, name) = (capability.trim(), name.trim());
        (!capability.is_empty() && !name.is_empty()).then(|| RequirementId {
            capability: capability.to_string(),
            name: name.to_string(),
        })
    }
}

impl fmt::Display for RequirementId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}::{}", self.capability, self.name)
    }
}

/// What a plan must do with an accepted decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecisionExpect {
    Follow,
    Supersede,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpectedDecision {
    pub id: String,
    pub expect: DecisionExpect,
}

/// A planning case: a request replayed on the repository at `git_ref`, and
/// what the real change did.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlanCase {
    pub git_ref: String,
    pub request: String,
    /// The requirements the request is about.
    pub modified: Vec<RequirementId>,
    /// The requirements the real change also modified in passing.
    pub collateral: Vec<RequirementId>,
    pub decisions: Vec<ExpectedDecision>,
}

/// What the repository's history says at a commit, read by the shell.
pub trait History {
    fn has_commit(&self, git_ref: &str) -> bool;
    /// The requirement names of `_codev/specs/<capability>/spec.md` at
    /// `git_ref`; `None` when the spec is absent.
    fn requirement_names(&self, git_ref: &str, capability: &str) -> Option<Vec<String>>;
    /// The ids (`0007`) of `_codev/decisions/` at `git_ref`.
    fn decision_ids(&self, git_ref: &str) -> Vec<String>;
    fn has_path(&self, git_ref: &str, path: &str) -> bool;
}

/// Checks a planning case against the history: its commit exists, its
/// expected requirements and decisions existed at that commit, and the
/// state does not already hold the case — its answer.
pub fn check_history(case: &Case, history: &dyn History) -> Result<(), CaseError> {
    let Some(plan) = &case.plan else {
        return Ok(());
    };
    let fail = |message: String| CaseError {
        case: case.id.clone(),
        message,
    };
    if !history.has_commit(&plan.git_ref) {
        return Err(fail(format!(
            "`ref` {} is not a commit of the repository's history; the planning cases need \
             the full history (a shallow clone lacks it: fetch with --unshallow or fetch-depth 0)",
            plan.git_ref
        )));
    }
    for req in plan.modified.iter().chain(&plan.collateral) {
        let names = history
            .requirement_names(&plan.git_ref, &req.capability)
            .unwrap_or_default();
        if !names
            .iter()
            .any(|n| normalize_heading(n) == normalize_heading(&req.name))
        {
            return Err(fail(format!(
                "requirement `{req}` is not in `_codev/specs/{}/spec.md` at {}",
                req.capability, plan.git_ref
            )));
        }
    }
    let ids = history.decision_ids(&plan.git_ref);
    for d in &plan.decisions {
        if !ids.contains(&d.id) {
            return Err(fail(format!(
                "decision `{}` is not in `_codev/decisions/` at {}",
                d.id, plan.git_ref
            )));
        }
    }
    if history.has_path(&plan.git_ref, &format!("evals/cases/{}/case.yaml", case.id)) {
        return Err(fail(format!(
            "the tree at {} already holds `evals/cases/{}/`, which would hand the agent the answer",
            plan.git_ref, case.id
        )));
    }
    Ok(())
}

impl Case {
    /// The change's directory, relative to the project root.
    pub fn change_dir(&self) -> String {
        format!("_codev/changes/{}", self.change)
    }

    /// The files whose content the runner records after the run.
    pub fn checked_paths(&self) -> Vec<String> {
        let mut paths: Vec<String> = self
            .fixed
            .iter()
            .map(|c| c.path.clone())
            .chain(self.unchanged.iter().map(|c| c.path.clone()))
            .collect();
        paths.sort();
        paths.dedup();
        paths
    }
}

/// A case refused, with the reason.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CaseError {
    pub case: String,
    pub message: String,
}

impl fmt::Display for CaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "case `{}`: {}", self.case, self.message)
    }
}

impl std::error::Error for CaseError {}

/// The files of a case, by path relative to `base/` or `change/`.
#[derive(Debug, Default, Clone)]
pub struct Fixture {
    pub base: BTreeMap<String, Vec<u8>>,
    pub change: BTreeMap<String, Vec<u8>>,
}

impl Fixture {
    /// The project as the change leaves it: `change/` over `base/`.
    fn merged(&self) -> BTreeMap<&str, &[u8]> {
        let mut files: BTreeMap<&str, &[u8]> = BTreeMap::new();
        for (path, bytes) in self.base.iter().chain(self.change.iter()) {
            files.insert(path.as_str(), bytes.as_slice());
        }
        files
    }
}

/// The steps each role serves, from the `at:` of its role file.
pub type RoleSteps = BTreeMap<String, Vec<String>>;

/// Reads the `id` and `at` of a role file's frontmatter.
pub fn role_steps_from_file(text: &str) -> Option<(String, Vec<String>)> {
    let text = text.replace("\r\n", "\n");
    let rest = text.strip_prefix("---\n")?;
    let end = rest.find("\n---")?;
    let value: serde_norway::Value = serde_norway::from_str(&rest[..end]).ok()?;
    let id = value.get("id")?.as_str()?.to_string();
    let at = match value.get("at")? {
        serde_norway::Value::String(s) => vec![s.clone()],
        serde_norway::Value::Sequence(items) => items
            .iter()
            .filter_map(|v| v.as_str().map(str::to_string))
            .collect(),
        _ => return None,
    };
    Some((id, at))
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CaseFile {
    role: String,
    step: String,
    change: String,
    stacks: Vec<String>,
    kind: String,
    summary: String,
    #[serde(default)]
    expected: Vec<ExpectedFile>,
    #[serde(default)]
    checks: Option<ChecksFile>,
    #[serde(default)]
    planned: Vec<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ExpectedFile {
    id: String,
    severity: String,
    category: String,
    locations: Vec<String>,
    #[serde(default)]
    alternatives: Vec<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ChecksFile {
    #[serde(default)]
    fixed: Vec<FixedFile>,
    #[serde(default)]
    unchanged: Vec<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct FixedFile {
    path: String,
    contains: String,
}

const SEVERITIES: [&str; 3] = ["high", "medium", "low"];

/// What a case is checked against: the roles' steps and the category
/// vocabulary.
#[derive(Debug, Clone, Default)]
pub struct Catalog {
    pub roles: RoleSteps,
    pub vocabulary: Vocabulary,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct PlanCaseFile {
    #[allow(dead_code)]
    step: String,
    kind: String,
    #[serde(rename = "ref")]
    git_ref: String,
    request: String,
    summary: String,
    expected: PlanExpectedFile,
}

#[derive(Deserialize, Default)]
#[serde(deny_unknown_fields)]
struct PlanExpectedFile {
    #[serde(default)]
    modified: Vec<String>,
    #[serde(default)]
    collateral: Vec<String>,
    #[serde(default)]
    decisions: Vec<DecisionFile>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct DecisionFile {
    id: String,
    expect: String,
}

/// The role a planning case is filed under: it evaluates the planning
/// step, not a reviewing role.
pub const PLANNER: &str = "planner";

/// Loads a planning case: a `case.yaml` alone, no fixture directory.
fn load_plan(id: &str, yaml: &str, fixture: &Fixture) -> Result<Case, CaseError> {
    let fail = |message: String| CaseError {
        case: id.to_string(),
        message,
    };
    let file: PlanCaseFile =
        serde_norway::from_str(yaml).map_err(|e| fail(format!("case.yaml: {e}")))?;
    if !fixture.base.is_empty() || !fixture.change.is_empty() {
        return Err(fail(
            "a planning case holds only its case.yaml: its fixture is the repository at `ref`"
                .into(),
        ));
    }
    if file.kind != "seeded" {
        return Err(fail(format!(
            "unknown kind `{}` for a planning case; expected seeded",
            file.kind
        )));
    }
    if file.request.trim().is_empty() {
        return Err(fail("the request is empty".into()));
    }
    if file.git_ref.trim().is_empty() {
        return Err(fail("`ref` is empty".into()));
    }
    let parse = |texts: &[String]| -> Result<Vec<RequirementId>, CaseError> {
        texts
            .iter()
            .map(|t| {
                RequirementId::parse(t)
                    .ok_or_else(|| fail(format!("`{t}` is not `<capability>::<requirement name>`")))
            })
            .collect()
    };
    let modified = parse(&file.expected.modified)?;
    let collateral = parse(&file.expected.collateral)?;
    let mut decisions = Vec::new();
    for d in &file.expected.decisions {
        let expect = match d.expect.as_str() {
            "follow" => DecisionExpect::Follow,
            "supersede" => DecisionExpect::Supersede,
            other => {
                return Err(fail(format!(
                    "decision `{}`: unknown expect `{other}`; expected follow or supersede",
                    d.id
                )));
            }
        };
        decisions.push(ExpectedDecision {
            id: d.id.clone(),
            expect,
        });
    }
    if modified.is_empty() && collateral.is_empty() && decisions.is_empty() {
        return Err(fail(
            "a planning case must expect at least one requirement or one decision".into(),
        ));
    }
    Ok(Case {
        id: id.to_string(),
        role: PLANNER.to_string(),
        step: Step::PlanWrite,
        change: String::new(),
        stacks: Vec::new(),
        kind: Kind::Seeded,
        summary: file.summary,
        expected: Vec::new(),
        fixed: Vec::new(),
        unchanged: Vec::new(),
        planned: Vec::new(),
        plan: Some(PlanCase {
            git_ref: file.git_ref.trim().to_string(),
            request: file.request.trim().to_string(),
            modified,
            collateral,
            decisions,
        }),
    })
}

/// Loads and checks a case: `id` is its directory name, `yaml` its
/// `case.yaml`, `fixture` its files, `catalog` the roles' steps and the
/// category vocabulary.
pub fn load(id: &str, yaml: &str, fixture: &Fixture, catalog: &Catalog) -> Result<Case, CaseError> {
    let roles = &catalog.roles;
    let fail = |message: String| CaseError {
        case: id.to_string(),
        message,
    };
    #[derive(Deserialize)]
    struct StepOnly {
        step: Option<String>,
    }
    let peek: Option<StepOnly> = serde_norway::from_str(yaml).ok();
    if peek.and_then(|p| p.step).as_deref() == Some("plan.write") {
        return load_plan(id, yaml, fixture);
    }
    let file: CaseFile =
        serde_norway::from_str(yaml).map_err(|e| fail(format!("case.yaml: {e}")))?;

    let step = Step::parse(&file.step).ok_or_else(|| {
        fail(format!(
            "unknown step `{}`; expected plan.challenge, code.review, plan.trace or plan.write",
            file.step
        ))
    })?;
    let served = roles
        .get(&file.role)
        .ok_or_else(|| fail(format!("unknown role `{}`", file.role)))?;
    if !served.iter().any(|s| s == step.as_str()) {
        return Err(fail(format!(
            "role `{}` does not serve `{}` (it serves {})",
            file.role,
            step.as_str(),
            served.join(", ")
        )));
    }
    let kind = match file.kind.as_str() {
        "seeded" => Kind::Seeded,
        "control" => Kind::Control,
        other => {
            return Err(fail(format!(
                "unknown kind `{other}`; expected seeded or control"
            )));
        }
    };

    check_fixture_files(fixture).map_err(fail)?;
    let files = fixture.merged();
    let change_dir = format!("_codev/changes/{}", file.change);
    if !files.contains_key(format!("{change_dir}/proposal.md").as_str()) {
        return Err(fail(format!(
            "the fixture has no `{change_dir}/proposal.md` in `change/`"
        )));
    }

    let mut expected = Vec::new();
    for e in &file.expected {
        if !SEVERITIES.contains(&e.severity.as_str()) {
            return Err(fail(format!(
                "expected `{}`: unknown severity `{}`",
                e.id, e.severity
            )));
        }
        if e.locations.is_empty() {
            return Err(fail(format!("expected `{}` has no location", e.id)));
        }
        let terms = catalog.vocabulary.get(&e.category).ok_or_else(|| {
            fail(format!(
                "expected `{}`: category `{}` is not in evals/categories.yaml",
                e.id, e.category
            ))
        })?;
        let parse = |texts: &[String]| -> Result<Vec<Location>, CaseError> {
            texts
                .iter()
                .map(|text| {
                    let location = Location::parse(text);
                    check_location(&location, &files, &change_dir)
                        .map_err(|m| fail(format!("expected `{}`: {m}", e.id)))?;
                    Ok(location)
                })
                .collect()
        };
        expected.push(Expected {
            id: e.id.clone(),
            role: file.role.clone(),
            severity: e.severity.clone(),
            category: e.category.clone(),
            terms: terms.clone(),
            locations: parse(&e.locations)?,
            alternatives: parse(&e.alternatives)?,
        });
    }

    let (mut fixed, mut unchanged) = (Vec::new(), Vec::new());
    if let Some(checks) = &file.checks {
        if step != Step::PlanTrace {
            return Err(fail(
                "`checks` is for plan.trace cases; use `expected`".into(),
            ));
        }
        for c in &checks.fixed {
            let path = normalize_path(&c.path);
            if !files.contains_key(path.as_str()) {
                return Err(fail(format!(
                    "check on `{path}`, which the fixture does not have"
                )));
            }
            fixed.push(FixedCheck {
                path,
                contains: c.contains.clone(),
            });
        }
        for p in &checks.unchanged {
            let path = normalize_path(p);
            let bytes = files.get(path.as_str()).ok_or_else(|| {
                fail(format!(
                    "check on `{path}`, which the fixture does not have"
                ))
            })?;
            unchanged.push(UnchangedCheck {
                path,
                original: String::from_utf8_lossy(bytes).into_owned(),
            });
        }
    }
    if step == Step::PlanTrace && !expected.is_empty() {
        return Err(fail(
            "plan.trace cases state `checks`, not `expected`".into(),
        ));
    }

    let expects_something = !expected.is_empty() || !fixed.is_empty();
    match kind {
        Kind::Seeded if !expects_something => {
            return Err(fail(
                "a seeded case must expect at least one finding".into(),
            ));
        }
        Kind::Control if expects_something => {
            return Err(fail("a control case must expect nothing".into()));
        }
        Kind::Control if step == Step::PlanTrace && unchanged.is_empty() => {
            return Err(fail(
                "a plan.trace control must name the files to leave unchanged".into(),
            ));
        }
        _ => {}
    }

    let planned: Vec<String> = file.planned.iter().map(|p| normalize_path(p)).collect();
    if step == Step::CodeReview && !planned.is_empty() {
        return Err(fail(
            "`planned` is for plan steps: a code.review case has its tasks done, so a code case plans nothing"
                .into(),
        ));
    }
    let problems = crate::lint::lint(&change_dir, &files, &planned);
    if !problems.is_empty() {
        return Err(fail(problems.join("; ")));
    }

    Ok(Case {
        id: id.to_string(),
        role: file.role,
        step,
        change: file.change,
        stacks: file.stacks,
        kind,
        summary: file.summary,
        expected,
        fixed,
        unchanged,
        planned,
        plan: None,
    })
}

fn check_fixture_files(fixture: &Fixture) -> Result<(), String> {
    for (tree, files) in [("base", &fixture.base), ("change", &fixture.change)] {
        for (path, bytes) in files {
            if bytes.len() > MAX_FIXTURE_FILE_BYTES {
                return Err(format!(
                    "`{tree}/{path}` is {} bytes, over the {MAX_FIXTURE_FILE_BYTES}-byte limit",
                    bytes.len()
                ));
            }
            if std::str::from_utf8(bytes).is_err() {
                return Err(format!("`{tree}/{path}` is not UTF-8 text"));
            }
        }
    }
    Ok(())
}

/// A plan location is relative to the change's directory, a code location
/// to the project; a bare file may be either.
fn check_location(
    location: &Location,
    files: &BTreeMap<&str, &[u8]>,
    change_dir: &str,
) -> Result<(), String> {
    let lookup = |path: &str| -> Option<String> {
        files
            .get(path)
            .map(|b| String::from_utf8_lossy(b).into_owned())
    };
    match location {
        Location::Lines { path, start, end } => {
            let text = lookup(path).ok_or_else(|| format!("`{path}` is not in the fixture"))?;
            let count = text.lines().count() as u32;
            if *start == 0 || start > end || *end > count {
                return Err(format!(
                    "`{location}` is beyond the end of `{path}` ({count} lines)"
                ));
            }
            Ok(())
        }
        Location::Section { file, heading } => {
            let text = lookup(&format!("{change_dir}/{file}"))
                .or_else(|| lookup(file))
                .ok_or_else(|| format!("`{file}` is not in the fixture"))?;
            if let Some(heading) = heading {
                let wanted = normalize_heading(heading);
                let found = text.lines().any(|line| {
                    let line = line.trim_start();
                    line.starts_with('#')
                        && normalize_heading(line.trim_start_matches('#')) == wanted
                });
                if !found {
                    return Err(format!("`{file}` has no heading `{heading}`"));
                }
            }
            Ok(())
        }
    }
}

/// The minimum the suite holds per role: (seeded, controls).
pub const COVERAGE: [(&str, usize, usize); 9] = [
    ("security", 3, 2),
    ("privacy", 3, 2),
    ("accessibility", 3, 2),
    ("performance", 3, 2),
    ("ops", 3, 2),
    ("migration", 3, 2),
    ("newcomer", 2, 1),
    ("skeptic", 2, 1),
    ("tracer", 1, 1),
];

/// The experts: each needs a seeded case expecting two findings of
/// different severity.
pub const EXPERTS: [&str; 6] = [
    "security",
    "privacy",
    "accessibility",
    "performance",
    "ops",
    "migration",
];

/// The stacks the suite's fixtures must include, together.
pub const STACKS: [&str; 3] = ["rust", "typescript", "python"];

/// The roles that need a case in a given stack: (role, stack).
pub const ROLE_STACKS: [(&str, &str); 2] = [("security", "python"), ("privacy", "python")];

/// What the suite lacks against [`COVERAGE`], [`EXPERTS`], [`STACKS`] and
/// [`ROLE_STACKS`], one message per gap; empty when it is complete.
pub fn coverage_gaps(cases: &[Case]) -> Vec<String> {
    let mut gaps = Vec::new();
    for (role, seeded, controls) in COVERAGE {
        let count = |kind| {
            cases
                .iter()
                .filter(|c| c.role == role && c.kind == kind)
                .count()
        };
        let (s, c) = (count(Kind::Seeded), count(Kind::Control));
        if s < seeded {
            gaps.push(format!(
                "{role}: {s} seeded case(s), at least {seeded} needed"
            ));
        }
        if c < controls {
            gaps.push(format!(
                "{role}: {c} control(s), at least {controls} needed"
            ));
        }
    }
    for role in EXPERTS {
        let two_severities = cases.iter().any(|c| {
            c.role == role && {
                let mut severities: Vec<&str> =
                    c.expected.iter().map(|e| e.severity.as_str()).collect();
                severities.sort();
                severities.dedup();
                severities.len() >= 2
            }
        });
        if !two_severities {
            gaps.push(format!(
                "{role}: no seeded case expects two findings of different severity"
            ));
        }
    }
    for stack in STACKS {
        if !cases.iter().any(|c| c.stacks.iter().any(|s| s == stack)) {
            gaps.push(format!("no case uses the `{stack}` stack"));
        }
    }
    for (role, stack) in ROLE_STACKS {
        if !cases
            .iter()
            .any(|c| c.role == role && c.stacks.iter().any(|s| s == stack))
        {
            gaps.push(format!("{role}: no `{stack}` case"));
        }
    }
    gaps
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    pub(crate) fn roles() -> RoleSteps {
        let mut roles = RoleSteps::new();
        for (id, steps) in [
            ("security", vec!["plan.challenge", "code.review"]),
            ("privacy", vec!["plan.challenge", "code.review"]),
            ("ops", vec!["plan.challenge", "code.review"]),
            ("newcomer", vec!["code.review"]),
            ("skeptic", vec!["plan.challenge"]),
            ("tracer", vec!["plan.trace"]),
        ] {
            roles.insert(id.into(), steps.into_iter().map(String::from).collect());
        }
        roles
    }

    pub(crate) fn catalog() -> Catalog {
        let mut vocabulary = Vocabulary::new();
        vocabulary.insert("injection".into(), vec!["injection".into(), "sql".into()]);
        Catalog {
            roles: roles(),
            vocabulary,
        }
    }

    pub(crate) fn fixture() -> Fixture {
        let mut f = Fixture::default();
        f.base.insert("README.md".into(), b"# Shop\n".to_vec());
        let users: String = (1..=40).map(|n| format!("line {n}\n")).collect();
        f.change.insert("src/users.ts".into(), users.into_bytes());
        f.change.insert(
            "_codev/changes/add-search/proposal.md".into(),
            b"# Proposal: Search\n\n## Why\n\nFind users.\n".to_vec(),
        );
        f.change.insert(
            "_codev/changes/add-search/design.md".into(),
            b"# Design\n\n### Decision: Tokens are stored as given\n\nText.\n".to_vec(),
        );
        f
    }

    const SEEDED: &str = "role: security\nstep: code.review\nchange: add-search\nstacks: [typescript]\nkind: seeded\nsummary: SQL injection.\nexpected:\n  - id: sqli\n    severity: high\n    category: injection\n    locations: [\"src/users.ts:9-11\"]\n";

    #[test]
    fn a_well_formed_case_loads() {
        let case = load("security-sqli", SEEDED, &fixture(), &catalog()).unwrap();
        assert_eq!(case.step, Step::CodeReview);
        assert_eq!(
            case.expected[0].locations,
            [Location::Lines {
                path: "src/users.ts".into(),
                start: 9,
                end: 11
            }]
        );
    }

    #[test]
    fn a_case_with_an_unknown_key_is_refused() {
        let yaml = SEEDED.replace("expected:", "expect:");
        let err = load("c", &yaml, &fixture(), &catalog()).unwrap_err();
        assert!(err.message.contains("expect"), "{err}");
        assert_eq!(err.case, "c");
    }

    #[test]
    fn a_location_beyond_the_end_is_refused() {
        let yaml = SEEDED.replace("src/users.ts:9-11", "src/users.ts:120");
        let err = load("c", &yaml, &fixture(), &catalog()).unwrap_err();
        assert!(
            err.message.contains("src/users.ts") && err.message.contains("120"),
            "{err}"
        );
    }

    #[test]
    fn a_binary_fixture_file_is_refused() {
        let mut f = fixture();
        f.change
            .insert("logo.png".into(), vec![0x89, 0x50, 0xff, 0xfe]);
        let err = load("c", SEEDED, &f, &catalog()).unwrap_err();
        assert!(err.message.contains("change/logo.png"), "{err}");
    }

    #[test]
    fn a_file_over_the_size_limit_is_refused() {
        let mut f = fixture();
        f.base
            .insert("big.txt".into(), vec![b'a'; MAX_FIXTURE_FILE_BYTES + 1]);
        let err = load("c", SEEDED, &f, &catalog()).unwrap_err();
        assert!(err.message.contains("base/big.txt"), "{err}");
    }

    #[test]
    fn a_role_that_does_not_serve_the_step_is_refused() {
        let yaml = SEEDED.replace("role: security", "role: skeptic");
        let err = load("c", &yaml, &fixture(), &catalog()).unwrap_err();
        assert!(
            err.message.contains("does not serve `code.review`"),
            "{err}"
        );
    }

    #[test]
    fn a_control_with_an_expectation_is_refused() {
        let yaml = SEEDED.replace("kind: seeded", "kind: control");
        let err = load("c", &yaml, &fixture(), &catalog()).unwrap_err();
        assert!(err.message.contains("control"), "{err}");
        let seeded_empty = SEEDED.split("expected:").next().unwrap();
        let err = load("c", seeded_empty, &fixture(), &catalog()).unwrap_err();
        assert!(err.message.contains("seeded"), "{err}");
    }

    #[test]
    fn a_missing_heading_is_refused() {
        let yaml = SEEDED
            .replace("step: code.review", "step: plan.challenge")
            .replace("src/users.ts:9-11", "design.md#Decision: Other");
        let err = load("c", &yaml, &fixture(), &catalog()).unwrap_err();
        assert!(err.message.contains("Decision: Other"), "{err}");
        let yaml = yaml.replace("Decision: Other", "decision:  tokens are STORED as given");
        assert!(load("c", &yaml, &fixture(), &catalog()).is_ok());
    }

    #[test]
    fn locations_parse_like_the_review_record() {
        assert_eq!(
            Location::parse(".\\src\\db.rs:22"),
            Location::Lines {
                path: "src/db.rs".into(),
                start: 22,
                end: 22
            }
        );
        assert_eq!(
            Location::parse("design.md#Decision: A: b"),
            Location::Section {
                file: "design.md".into(),
                heading: Some("Decision: A: b".into())
            }
        );
        assert_eq!(
            Location::parse("proposal.md"),
            Location::Section {
                file: "proposal.md".into(),
                heading: None
            }
        );
    }

    #[test]
    fn role_steps_are_read_from_the_frontmatter() {
        let file = "---\nid: security\nat: [plan.challenge, code.review]\n---\n# Security\n";
        assert_eq!(
            role_steps_from_file(file),
            Some((
                "security".into(),
                vec!["plan.challenge".into(), "code.review".into()]
            ))
        );
        assert_eq!(
            role_steps_from_file("---\r\nid: tracer\r\nat: plan.trace\r\n---\r\n"),
            Some(("tracer".into(), vec!["plan.trace".into()]))
        );
    }

    #[test]
    fn a_vocabulary_category_without_term_is_refused() {
        let ok = "categories:\n  injection:\n    description: Input reaches SQL.\n    terms: [SQL, ' Inject ']\n";
        assert_eq!(
            parse_vocabulary(ok).unwrap()["injection"],
            ["sql", "inject"],
            "terms are lowercased and trimmed"
        );
        let empty =
            "categories:\n  injection:\n    description: Input reaches SQL.\n    terms: []\n";
        let err = parse_vocabulary(empty).unwrap_err();
        assert!(err.contains("`injection` has no term"), "{err}");
        let unknown = ok.replace("terms:", "words:");
        let err = parse_vocabulary(&unknown).unwrap_err();
        assert!(err.contains("words"), "{err}");
    }

    #[test]
    fn a_category_outside_the_vocabulary_is_refused() {
        let yaml = SEEDED.replace("category: injection", "category: sqli");
        let err = load("c", &yaml, &fixture(), &catalog()).unwrap_err();
        assert!(err.message.contains("`sqli`"), "{err}");
        let case = load("c", SEEDED, &fixture(), &catalog()).unwrap();
        assert_eq!(case.expected[0].terms, ["injection", "sql"]);
    }

    #[test]
    fn an_alternative_beyond_the_end_is_refused() {
        let with = |alt: &str| {
            SEEDED.replace(
                "    locations: [\"src/users.ts:9-11\"]\n",
                &format!("    locations: [\"src/users.ts:9-11\"]\n    alternatives: [\"{alt}\"]\n"),
            )
        };
        let case = load("c", &with("src/users.ts:30"), &fixture(), &catalog()).unwrap();
        assert_eq!(case.expected[0].alternatives.len(), 1);
        let err = load("c", &with("src/users.ts:41"), &fixture(), &catalog()).unwrap_err();
        assert!(err.message.contains("src/users.ts:41"), "{err}");
    }

    fn plan_case(tasks: &str, planned: &str) -> Result<Case, CaseError> {
        let mut f = fixture();
        f.change.insert(
            "_codev/changes/add-search/tasks.md".into(),
            tasks.as_bytes().to_vec(),
        );
        let yaml = SEEDED
            .replace("step: code.review", "step: plan.challenge")
            .replace(
                "src/users.ts:9-11",
                "design.md#Decision: Tokens are stored as given",
            )
            + planned;
        load("c", &yaml, &f, &catalog())
    }

    #[test]
    fn a_plan_citing_a_missing_file_is_refused() {
        let mut f = fixture();
        f.change.insert(
            "_codev/changes/add-search/proposal.md".into(),
            b"# Proposal: Search\n\n## Impact\n\n- `src/cli/progress.rs`, `src/users.ts`.\n"
                .to_vec(),
        );
        let err = load("c", SEEDED, &f, &catalog()).unwrap_err();
        assert!(
            err.message
                .contains("`proposal.md` cites `src/cli/progress.rs`"),
            "{err}"
        );
    }

    #[test]
    fn a_file_the_plan_creates_is_accepted_when_planned() {
        let tasks = "- [ ] 1.1 Write `src/export/csv.rs`\n";
        assert!(
            plan_case(tasks, "")
                .unwrap_err()
                .message
                .contains("src/export/csv.rs")
        );
        let case = plan_case(tasks, "planned: [src/export/csv.rs]\n").unwrap();
        assert_eq!(case.planned, ["src/export/csv.rs"]);
    }

    #[test]
    fn a_task_verified_by_a_missing_test_is_refused() {
        let mut f = fixture();
        f.change.insert(
            "_codev/changes/add-search/tasks.md".into(),
            b"- [x] 1.2 Docs, verified by `cargo test docs_mention_every_flag`\n".to_vec(),
        );
        let err = load("c", SEEDED, &f, &catalog()).unwrap_err();
        assert!(err.message.contains("docs_mention_every_flag"), "{err}");
        f.base.insert(
            "tests/docs.rs".into(),
            b"#[test]\nfn docs_mention_every_flag() {}\n".to_vec(),
        );
        assert!(load("c", SEEDED, &f, &catalog()).is_ok());
    }

    #[test]
    fn a_code_case_that_plans_is_refused() {
        let yaml = format!("{SEEDED}planned: [src/new.ts]\n");
        let err = load("c", &yaml, &fixture(), &catalog()).unwrap_err();
        assert!(err.message.contains("a code case plans nothing"), "{err}");
    }

    #[test]
    fn a_stale_or_unused_planned_entry_is_refused() {
        let tasks = "- [ ] 1.1 Change `src/users.ts`\n";
        let err = plan_case(tasks, "planned: [src/users.ts]\n").unwrap_err();
        assert!(err.message.contains("already has"), "{err}");
        let err = plan_case(tasks, "planned: [src/other.ts]\n").unwrap_err();
        assert!(err.message.contains("no artifact cites"), "{err}");
    }

    fn case(role: &str, kind: Kind, stack: &str) -> Case {
        Case {
            id: format!("{role}-{}", kind.as_str()),
            role: role.into(),
            step: Step::CodeReview,
            change: "c".into(),
            stacks: vec![stack.into()],
            kind,
            summary: String::new(),
            expected: vec![],
            fixed: vec![],
            unchanged: vec![],
            planned: vec![],
            plan: None,
        }
    }

    fn expecting(mut c: Case, severities: &[&str]) -> Case {
        c.expected = severities
            .iter()
            .enumerate()
            .map(|(n, severity)| Expected {
                id: format!("e{n}"),
                role: c.role.clone(),
                severity: (*severity).into(),
                category: "c".into(),
                terms: vec!["c".into()],
                locations: vec![],
                alternatives: vec![],
            })
            .collect();
        c
    }

    /// A suite meeting every minimum.
    fn complete_suite() -> Vec<Case> {
        let mut cases = Vec::new();
        for (role, seeded, controls) in COVERAGE {
            for n in 0..seeded {
                let severities: &[&str] = if n == 0 {
                    &["high", "medium"]
                } else {
                    &["high"]
                };
                cases.push(expecting(case(role, Kind::Seeded, "rust"), severities));
            }
            for _ in 0..controls {
                cases.push(case(role, Kind::Control, "python"));
            }
        }
        cases.push(case("newcomer", Kind::Seeded, "typescript"));
        cases
    }

    #[test]
    fn coverage_names_a_role_without_control() {
        let mut cases = complete_suite();
        assert_eq!(coverage_gaps(&cases), Vec::<String>::new());
        let mut dropped = false;
        cases.retain(|c| {
            let drop = !dropped && c.role == "ops" && c.kind == Kind::Control;
            dropped |= drop;
            !drop
        });
        assert_eq!(
            coverage_gaps(&cases),
            ["ops: 1 control(s), at least 2 needed"]
        );
    }

    #[test]
    fn coverage_names_an_expert_without_two_severities() {
        let cases: Vec<Case> = complete_suite()
            .into_iter()
            .map(|c| {
                if c.role == "migration" && c.kind == Kind::Seeded {
                    expecting(c, &["high", "high"])
                } else {
                    c
                }
            })
            .collect();
        assert_eq!(
            coverage_gaps(&cases),
            ["migration: no seeded case expects two findings of different severity"]
        );
    }

    #[test]
    fn coverage_names_security_without_python() {
        let cases: Vec<Case> = complete_suite()
            .into_iter()
            .map(|mut c| {
                if c.role == "security" {
                    c.stacks = vec!["rust".into()];
                }
                c
            })
            .collect();
        assert_eq!(coverage_gaps(&cases), ["security: no `python` case"]);
    }

    const PLAN_YAML: &str = "step: plan.write\nkind: seeded\nref: abc1234\nrequest: \"Je voudrais qu'une décision naisse en brouillon.\"\nsummary: A draft decision.\nexpected:\n  modified: [\"decisions::Creating a decision through the CLI\"]\n  collateral: [\"decisions::Stable JSON contract for all commands\"]\n  decisions:\n    - { id: \"0007\", expect: follow }\n";

    fn planning_case(yaml: &str) -> Result<Case, CaseError> {
        load(
            "plan-decision-accept",
            yaml,
            &Fixture::default(),
            &Catalog::default(),
        )
    }

    #[test]
    fn a_planning_case_loads() {
        let case = planning_case(PLAN_YAML).unwrap();
        assert_eq!(case.step, Step::PlanWrite);
        assert_eq!(case.role, PLANNER);
        let plan = case.plan.unwrap();
        assert_eq!(plan.git_ref, "abc1234");
        assert_eq!(plan.modified[0].capability, "decisions");
        assert_eq!(plan.modified[0].name, "Creating a decision through the CLI");
        assert_eq!(plan.collateral.len(), 1);
        assert_eq!(plan.decisions[0].expect, DecisionExpect::Follow);
    }

    #[test]
    fn a_planning_case_refuses_an_unknown_key_or_an_empty_request() {
        let unknown = PLAN_YAML.replace("summary:", "role: security\nsummary:");
        assert!(
            planning_case(&unknown)
                .unwrap_err()
                .message
                .contains("role")
        );
        let empty = PLAN_YAML.replace("Je voudrais qu'une décision naisse en brouillon.", " ");
        assert!(
            planning_case(&empty)
                .unwrap_err()
                .message
                .contains("request is empty")
        );
        let fixture = Fixture {
            base: [("a.txt".to_string(), b"x".to_vec())].into(),
            change: BTreeMap::new(),
        };
        let err = load("p", PLAN_YAML, &fixture, &Catalog::default()).unwrap_err();
        assert!(err.message.contains("only its case.yaml"), "{err}");
    }

    #[test]
    fn a_planning_case_expects_something() {
        let nothing =
            "step: plan.write\nkind: seeded\nref: abc\nrequest: x\nsummary: y\nexpected: {}\n";
        assert!(
            planning_case(nothing)
                .unwrap_err()
                .message
                .contains("at least one requirement or one decision")
        );
        let bad_id = PLAN_YAML.replace(
            "decisions::Creating a decision through the CLI",
            "no separator",
        );
        assert!(
            planning_case(&bad_id)
                .unwrap_err()
                .message
                .contains("<capability>::")
        );
    }

    #[derive(Default)]
    struct FakeHistory {
        commits: Vec<&'static str>,
        specs: BTreeMap<&'static str, Vec<&'static str>>,
        decisions: Vec<&'static str>,
        paths: Vec<String>,
    }

    impl History for FakeHistory {
        fn has_commit(&self, r: &str) -> bool {
            self.commits.contains(&r)
        }
        fn requirement_names(&self, _r: &str, cap: &str) -> Option<Vec<String>> {
            self.specs
                .get(cap)
                .map(|v| v.iter().map(|s| s.to_string()).collect())
        }
        fn decision_ids(&self, _r: &str) -> Vec<String> {
            self.decisions.iter().map(|s| s.to_string()).collect()
        }
        fn has_path(&self, _r: &str, path: &str) -> bool {
            self.paths.iter().any(|p| p == path)
        }
    }

    fn good_history() -> FakeHistory {
        FakeHistory {
            commits: vec!["abc1234"],
            specs: [(
                "decisions",
                vec![
                    "Creating a decision through the CLI",
                    "Stable JSON contract for all commands",
                ],
            )]
            .into(),
            decisions: vec!["0001", "0007"],
            paths: Vec::new(),
        }
    }

    #[test]
    fn a_requirement_absent_at_the_ref_is_named() {
        let case = planning_case(PLAN_YAML).unwrap();
        assert!(check_history(&case, &good_history()).is_ok());
        let mut h = good_history();
        h.specs
            .insert("decisions", vec!["Stable JSON contract for all commands"]);
        let err = check_history(&case, &h).unwrap_err();
        assert!(
            err.message
                .contains("decisions::Creating a decision through the CLI"),
            "{err}"
        );
    }

    #[test]
    fn a_ref_outside_the_history_asks_for_the_full_history() {
        let case = planning_case(PLAN_YAML).unwrap();
        let mut h = good_history();
        h.commits.clear();
        let err = check_history(&case, &h).unwrap_err();
        assert!(
            err.message.contains("abc1234") && err.message.contains("full history"),
            "{err}"
        );
    }

    #[test]
    fn an_unknown_decision_is_named() {
        let case = planning_case(PLAN_YAML).unwrap();
        let mut h = good_history();
        h.decisions = vec!["0001"];
        assert!(
            check_history(&case, &h)
                .unwrap_err()
                .message
                .contains("`0007`")
        );
    }

    #[test]
    fn a_ref_holding_the_case_is_refused() {
        let case = planning_case(PLAN_YAML).unwrap();
        let mut h = good_history();
        h.paths
            .push("evals/cases/plan-decision-accept/case.yaml".into());
        assert!(
            check_history(&case, &h)
                .unwrap_err()
                .message
                .contains("answer")
        );
    }
}
