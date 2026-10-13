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
}

impl Step {
    pub fn parse(text: &str) -> Option<Step> {
        match text {
            "plan.challenge" => Some(Step::PlanChallenge),
            "code.review" => Some(Step::CodeReview),
            "plan.trace" => Some(Step::PlanTrace),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Step::PlanChallenge => "plan.challenge",
            Step::CodeReview => "code.review",
            Step::PlanTrace => "plan.trace",
        }
    }

    /// Whether the step writes a review record (the tracer fixes instead).
    pub fn records_findings(self) -> bool {
        !matches!(self, Step::PlanTrace)
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
    pub locations: Vec<Location>,
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
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ExpectedFile {
    id: String,
    severity: String,
    category: String,
    locations: Vec<String>,
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

/// Loads and checks a case: `id` is its directory name, `yaml` its
/// `case.yaml`, `fixture` its files, `roles` the catalog's steps.
pub fn load(id: &str, yaml: &str, fixture: &Fixture, roles: &RoleSteps) -> Result<Case, CaseError> {
    let fail = |message: String| CaseError {
        case: id.to_string(),
        message,
    };
    let file: CaseFile =
        serde_norway::from_str(yaml).map_err(|e| fail(format!("case.yaml: {e}")))?;

    let step = Step::parse(&file.step).ok_or_else(|| {
        fail(format!(
            "unknown step `{}`; expected plan.challenge, code.review or plan.trace",
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
        let mut locations = Vec::new();
        for text in &e.locations {
            let location = Location::parse(text);
            check_location(&location, &files, &change_dir)
                .map_err(|m| fail(format!("expected `{}`: {m}", e.id)))?;
            locations.push(location);
        }
        expected.push(Expected {
            id: e.id.clone(),
            role: file.role.clone(),
            severity: e.severity.clone(),
            category: e.category.clone(),
            locations,
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
    ("security", 2, 1),
    ("privacy", 2, 1),
    ("accessibility", 2, 1),
    ("performance", 2, 1),
    ("ops", 2, 1),
    ("migration", 2, 1),
    ("newcomer", 2, 1),
    ("skeptic", 2, 1),
    ("tracer", 1, 1),
];

/// The stacks the suite's fixtures must include, together.
pub const STACKS: [&str; 3] = ["rust", "typescript", "python"];

/// What the suite lacks against [`COVERAGE`] and [`STACKS`], one message
/// per gap; empty when it is complete.
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
    for stack in STACKS {
        if !cases.iter().any(|c| c.stacks.iter().any(|s| s == stack)) {
            gaps.push(format!("no case uses the `{stack}` stack"));
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
        let case = load("security-sqli", SEEDED, &fixture(), &roles()).unwrap();
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
        let err = load("c", &yaml, &fixture(), &roles()).unwrap_err();
        assert!(err.message.contains("expect"), "{err}");
        assert_eq!(err.case, "c");
    }

    #[test]
    fn a_location_beyond_the_end_is_refused() {
        let yaml = SEEDED.replace("src/users.ts:9-11", "src/users.ts:120");
        let err = load("c", &yaml, &fixture(), &roles()).unwrap_err();
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
        let err = load("c", SEEDED, &f, &roles()).unwrap_err();
        assert!(err.message.contains("change/logo.png"), "{err}");
    }

    #[test]
    fn a_file_over_the_size_limit_is_refused() {
        let mut f = fixture();
        f.base
            .insert("big.txt".into(), vec![b'a'; MAX_FIXTURE_FILE_BYTES + 1]);
        let err = load("c", SEEDED, &f, &roles()).unwrap_err();
        assert!(err.message.contains("base/big.txt"), "{err}");
    }

    #[test]
    fn a_role_that_does_not_serve_the_step_is_refused() {
        let yaml = SEEDED.replace("role: security", "role: skeptic");
        let err = load("c", &yaml, &fixture(), &roles()).unwrap_err();
        assert!(
            err.message.contains("does not serve `code.review`"),
            "{err}"
        );
    }

    #[test]
    fn a_control_with_an_expectation_is_refused() {
        let yaml = SEEDED.replace("kind: seeded", "kind: control");
        let err = load("c", &yaml, &fixture(), &roles()).unwrap_err();
        assert!(err.message.contains("control"), "{err}");
        let seeded_empty = SEEDED.split("expected:").next().unwrap();
        let err = load("c", seeded_empty, &fixture(), &roles()).unwrap_err();
        assert!(err.message.contains("seeded"), "{err}");
    }

    #[test]
    fn a_missing_heading_is_refused() {
        let yaml = SEEDED
            .replace("step: code.review", "step: plan.challenge")
            .replace("src/users.ts:9-11", "design.md#Decision: Other");
        let err = load("c", &yaml, &fixture(), &roles()).unwrap_err();
        assert!(err.message.contains("Decision: Other"), "{err}");
        let yaml = yaml.replace("Decision: Other", "decision:  tokens are STORED as given");
        assert!(load("c", &yaml, &fixture(), &roles()).is_ok());
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
        }
    }

    #[test]
    fn coverage_names_a_role_without_control() {
        let mut cases = Vec::new();
        for (role, seeded, controls) in COVERAGE {
            for _ in 0..seeded {
                cases.push(case(role, Kind::Seeded, "rust"));
            }
            for _ in 0..controls {
                cases.push(case(role, Kind::Control, "rust"));
            }
        }
        cases.push(case("newcomer", Kind::Seeded, "typescript"));
        cases.push(case("newcomer", Kind::Seeded, "python"));
        assert_eq!(coverage_gaps(&cases), Vec::<String>::new());
        cases.retain(|c| !(c.role == "ops" && c.kind == Kind::Control));
        let gaps = coverage_gaps(&cases);
        assert_eq!(gaps, ["ops: 0 control(s), at least 1 needed"]);
    }
}
