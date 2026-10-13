//! The review record of a change: reads `review.md` and the files its
//! findings cite, resolves who was active, and lets `codev-core::review`
//! judge (decision 0001).
//!
//! Read-only, local, without git: the evidence is compared with the
//! working tree as it is now.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use codev_core::Layout;
use codev_core::parser::ast::Finding;
use codev_core::review::codes::{
    REVIEW_EVIDENCE_MOVED, REVIEW_EVIDENCE_NOT_FOUND, REVIEW_EVIDENCE_UNCHECKED,
    REVIEW_LOCATION_MISSING,
};
use codev_core::review::{
    self as core_review, Blocker, EvidenceMatch, Location, MAX_FILE_BYTES, Record, ReviewFinding,
    StepRoles,
};

use crate::change::ChangeContext;
use crate::config::ResolvedConfig;
use crate::error::{EngineError, Result};
use crate::ports::FileSystem;
use crate::roles;

/// Everything codev can say about a change's review record.
#[derive(Debug, Clone)]
pub struct ReviewCheck {
    /// `review.md`, relative to the project root.
    pub path: PathBuf,
    /// `None` when `review.md` does not exist.
    pub record: Option<Record>,
    /// The record's defects, at their line in `review.md`.
    pub findings: Vec<Finding>,
    /// The roles active at each reviewing step for the change; empty when
    /// the role settings are invalid (validation reports those).
    pub steps: Vec<StepRoles>,
    /// What keeps the change from being archived.
    pub blockers: Vec<Blocker>,
}

impl ReviewCheck {
    pub fn has_errors(&self) -> bool {
        self.findings
            .iter()
            .any(|f| f.severity == codev_core::parser::ast::Severity::Error)
    }
}

/// Reads and checks the review record of `ctx`'s change.
pub fn check(
    fs: &dyn FileSystem,
    layout: &Layout,
    config: &ResolvedConfig,
    ctx: &ChangeContext,
) -> Result<ReviewCheck> {
    let change_dir = layout.change_dir(&ctx.change);
    let path = change_dir.join(core_review::FILE);
    let relative = path
        .strip_prefix(layout.project_root())
        .unwrap_or(&path)
        .to_path_buf();

    let (steps, known) = match roles::resolve(fs, layout, config, Some(ctx)) {
        Ok(resolved) => {
            let steps = core_review::STEPS
                .iter()
                .map(|step| StepRoles {
                    step: (*step).to_string(),
                    active: resolved
                        .active_at(step)
                        .map(|active| active.iter().map(|r| r.role.id.clone()).collect())
                        .unwrap_or_default(),
                })
                .collect();
            let known: Vec<String> = resolved.roles.iter().map(|r| r.role.id.clone()).collect();
            (steps, Some(known))
        }
        Err(_) => (Vec::new(), None),
    };

    if !fs.exists(&path) {
        return Ok(ReviewCheck {
            blockers: core_review::gate(None, &steps),
            path: relative,
            record: None,
            findings: Vec::new(),
            steps,
        });
    }
    let source = fs
        .read_to_string(&path)
        .map_err(|e| EngineError::Unreadable {
            path: path.clone(),
            reason: e.to_string(),
        })?;
    let parsed = core_review::parse(&source);
    let record = parsed.value;
    let mut findings = parsed.findings;
    if record.is_supported() {
        if let Some(known) = &known {
            findings.extend(core_review::check_roles(&record, &steps, known));
        }
        let mut files = Files::new(fs);
        for finding in record.findings() {
            findings.extend(check_location(
                &mut files,
                layout.project_root(),
                &change_dir,
                finding,
            ));
        }
        findings.sort_by_key(|f| f.line);
    }
    Ok(ReviewCheck {
        blockers: core_review::gate(Some(&record), &steps),
        path: relative,
        record: Some(record),
        findings,
        steps,
    })
}

/// A cited file, read once.
#[derive(Debug, Clone)]
enum Cited {
    Text(String),
    Missing,
    /// Not compared, and why.
    Unchecked(&'static str),
}

struct Files<'a> {
    fs: &'a dyn FileSystem,
    cache: BTreeMap<PathBuf, Cited>,
}

impl<'a> Files<'a> {
    fn new(fs: &'a dyn FileSystem) -> Self {
        Self {
            fs,
            cache: BTreeMap::new(),
        }
    }

    fn get(&mut self, path: &Path) -> &Cited {
        let fs = self.fs;
        self.cache.entry(path.to_path_buf()).or_insert_with(|| {
            if !fs.exists(path) {
                return Cited::Missing;
            }
            // The size is asked before reading: a cited dump of several
            // gigabytes must not be loaded to be skipped.
            if fs.len(path).is_ok_and(|len| len > MAX_FILE_BYTES as u64) {
                return Cited::Unchecked("it is larger than 4 MiB");
            }
            match fs.read_to_string(path) {
                Err(_) => Cited::Unchecked("it is not UTF-8 text"),
                Ok(text) if text.contains('\0') => Cited::Unchecked("it holds NUL bytes"),
                Ok(text) => Cited::Text(text),
            }
        })
    }
}

/// `relative` is `/`-separated and already refused if it could leave
/// `base` (absolute, drive, `..`): joining segment by segment is safe.
fn join(base: &Path, relative: &str) -> PathBuf {
    relative
        .split('/')
        .fold(base.to_path_buf(), |path, segment| path.join(segment))
}

/// The location and evidence of a finding that still describes the tree.
fn check_location(
    files: &mut Files,
    root: &Path,
    change_dir: &Path,
    finding: &ReviewFinding,
) -> Option<Finding> {
    if !finding
        .status()
        .is_some_and(|status| status.describes_current_tree())
    {
        return None;
    }
    // A missing evidence is already a `review_invalid`: comparing nothing
    // would only add a second, misleading error.
    if finding.evidence.trim().is_empty() {
        return None;
    }
    let id = &finding.id;
    let line = finding.line;
    let (shown, outcome) = match finding.location.as_ref()? {
        Location::Code { path, start, end } => {
            let outcome = match files.get(&join(root, path)) {
                Cited::Missing => Err(format!("{id}: `{path}` does not exist")),
                Cited::Unchecked(why) => return Some(unchecked(id, path, why, line)),
                Cited::Text(text) if *end > core_review::line_count(text) => Err(format!(
                    "{id}: `{path}` has {} lines, the finding cites line {end}",
                    core_review::line_count(text)
                )),
                Cited::Text(text) => Ok(core_review::match_code(
                    text,
                    &finding.evidence,
                    *start,
                    *end,
                )),
            };
            (path.clone(), outcome)
        }
        Location::Plan { file, heading } => {
            let outcome = match files.get(&join(change_dir, file)) {
                Cited::Missing => Err(format!("{id}: `{file}` does not exist in the change")),
                Cited::Unchecked(why) => return Some(unchecked(id, file, why, line)),
                Cited::Text(text) => core_review::match_plan(text, heading, &finding.evidence)
                    .ok_or_else(|| format!("{id}: `{file}` has no heading `{heading}`")),
            };
            (file.clone(), outcome)
        }
    };
    match outcome {
        Err(message) => Some(Finding::error(REVIEW_LOCATION_MISSING, line, message)),
        Ok(EvidenceMatch::Found) => None,
        Ok(EvidenceMatch::Moved { line: now }) => Some(Finding::warning(
            REVIEW_EVIDENCE_MOVED,
            line,
            format!(
                "{id}: the evidence is now at `{shown}` line {now}, away from `{}`; update the location",
                finding.location_text
            ),
        )),
        Ok(EvidenceMatch::NotFound) => Some(Finding::error(
            REVIEW_EVIDENCE_NOT_FOUND,
            line,
            format!(
                "{id}: the evidence is not in `{}`; quote the cited text verbatim, or mark the finding fixed if the code changed",
                finding.location_text
            ),
        )),
    }
}

fn unchecked(id: &str, path: &str, why: &str, line: u32) -> Finding {
    Finding::warning(
        REVIEW_EVIDENCE_UNCHECKED,
        line,
        format!("{id}: the evidence was not compared with `{path}`: {why}"),
    )
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::ports::{FixedEnv, MemoryFileSystem};
    use codev_core::ChangeId;
    use codev_core::review::codes::*;

    /// The newcomer's section when it found nothing.
    pub(crate) const NEWCOMER_NO_FINDING: &str = "### newcomer\n\nNo finding.\n";

    /// A record whose `## plan.challenge` holds the skeptic's
    /// `No finding.` then `plan`, and whose `## code.review` holds
    /// `code_review`.
    pub(crate) fn record_with(plan: &str, code_review: &str) -> String {
        format!(
            "# Review record: add-auth\n\n<!-- codev-review: 1 -->\n\n## plan.challenge\n\n### skeptic\n\nNo finding.\n\n{plan}\n## code.review\n\n{code_review}"
        )
    }

    /// A record with only the default roles at `plan.challenge`, and
    /// `code_review` as its `## code.review` body.
    pub(crate) fn record(code_review: &str) -> String {
        record_with("", code_review)
    }

    /// One finding of `role`, its history after `open`.
    pub(crate) fn finding(
        role: &str,
        severity: &str,
        location: &str,
        evidence: &str,
        answers: &str,
    ) -> String {
        format!(
            "### {role}\n\n#### R-{role}-1 A finding\n\n```yaml\nseverity: {severity}\nconfidence: high\nlocation: {location}\nevidence: |\n  {evidence}\nrecommendation: Do better.\nhistory:\n  - status: open\n    by: {role}\n{answers}```\n"
        )
    }

    const SOURCE: &str = "fn main() {\n    let name = input();\n    let query = format!(\"SELECT {}\", name);\n    run(query);\n}\n";

    fn project(proposal: &str, review: &str) -> MemoryFileSystem {
        MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file(
                "/p/_codev/changes/add-auth/change.yaml",
                "schema: spec-driven\n",
            )
            .with_file("/p/_codev/changes/add-auth/proposal.md", proposal)
            .with_file("/p/_codev/changes/add-auth/review.md", review)
            .with_file("/p/src/db.rs", SOURCE)
    }

    const SECURITY: &str = "# Proposal: Auth\n\n## Domains\n\n- [x] security\n";
    const NO_DOMAIN: &str = "# Proposal: Auth\n\n## Domains\n\nNone\n";

    fn run(fs: &MemoryFileSystem) -> ReviewCheck {
        let layout = Layout::new("/p");
        let config = crate::config::resolve(fs, &FixedEnv::at("/p"), &layout).unwrap();
        let ctx = crate::change::load(fs, &layout, &config, ChangeId::parse("add-auth").unwrap())
            .unwrap();
        check(fs, &layout, &config, &ctx).unwrap()
    }

    fn codes(check: &ReviewCheck) -> Vec<&'static str> {
        check.findings.iter().map(|f| f.code).collect()
    }

    /// The security expert finds nothing in the plan and one finding in
    /// the code; the newcomer finds nothing.
    fn security_record(severity: &str, location: &str, evidence: &str, answers: &str) -> String {
        record_with(
            "### security\n\nNo finding.\n",
            &format!(
                "{NEWCOMER_NO_FINDING}\n{}",
                finding("security", severity, location, evidence, answers)
            ),
        )
    }

    #[test]
    fn a_well_formed_record_has_no_finding() {
        let review = security_record(
            "high",
            "src/db.rs:3",
            "let query = format!(\"SELECT {}\", name);",
            "",
        );
        let check = run(&project(SECURITY, &review));
        assert!(check.findings.is_empty(), "{:?}", check.findings);
        assert_eq!(
            check.path,
            PathBuf::from("_codev/changes/add-auth/review.md")
        );
        // The same record with Windows line endings.
        let check = run(&project(SECURITY, &review.replace('\n', "\r\n")));
        assert!(check.findings.is_empty(), "{:?}", check.findings);
        // The open high finding is not a defect of the record, only a blocker.
        assert_eq!(check.blockers.len(), 1);
        assert_eq!(check.blockers[0].code, REVIEW_UNRESOLVED);
    }

    #[test]
    fn invented_evidence_is_an_error() {
        let review = security_record(
            "medium",
            "src/db.rs:3",
            "db.execute(raw_sql)",
            "  - status: disputed\n    by: developer\n    note: n\n",
        );
        let check = run(&project(SECURITY, &review));
        assert_eq!(codes(&check), [REVIEW_EVIDENCE_NOT_FOUND]);
        assert!(check.has_errors());
    }

    #[test]
    fn a_line_beyond_the_end_is_missing() {
        let review = security_record("low", "src/db.rs:400", "run(query)", "");
        let check = run(&project(SECURITY, &review));
        assert_eq!(codes(&check), [REVIEW_LOCATION_MISSING]);
        assert!(check.findings[0].message.contains("400"));
        let review = security_record("low", "src/gone.rs:1", "run(query)", "");
        assert_eq!(
            codes(&run(&project(SECURITY, &review))),
            [REVIEW_LOCATION_MISSING]
        );
    }

    #[test]
    fn evidence_further_than_the_tolerance_is_a_warning() {
        let long: String = (1..=50).map(|n| format!("line {n}\n")).collect();
        let fs = project(
            SECURITY,
            &security_record("low", "src/db.rs:10", "line 45", ""),
        )
        .with_file("/p/src/db.rs", long);
        let check = run(&fs);
        assert_eq!(codes(&check), [REVIEW_EVIDENCE_MOVED]);
        assert!(check.findings[0].message.contains("line 45"));
    }

    #[test]
    fn a_plan_finding_is_found_in_its_section() {
        let review = security_record(
            "medium",
            "\"design.md#Decision: Tokens are stored\"",
            "stored in the sessions table",
            "",
        );
        let fs = project(SECURITY, &review).with_file(
            "/p/_codev/changes/add-auth/design.md",
            "# Design\n\n## Decisions\n\n### Decision: Tokens are stored\n\nThe token is stored in the sessions table.\n",
        );
        assert!(run(&fs).findings.is_empty(), "{:?}", run(&fs).findings);
        let missing_heading = security_record("medium", "'design.md#Decision: Other'", "x", "");
        let fs = project(SECURITY, &missing_heading)
            .with_file("/p/_codev/changes/add-auth/design.md", "# Design\n");
        assert_eq!(codes(&run(&fs)), [REVIEW_LOCATION_MISSING]);
    }

    #[test]
    fn a_binary_file_is_unchecked() {
        let review = security_record("low", "assets/logo.png:1", "PNG", "");
        let fs = project(SECURITY, &review).with_file("/p/assets/logo.png", "\u{0}PNG\u{0}");
        let check = run(&fs);
        assert_eq!(codes(&check), [REVIEW_EVIDENCE_UNCHECKED]);
        assert!(!check.has_errors());
    }

    /// Refuses to read `big`: proves a file over the cap is never loaded.
    struct NeverReads<'a> {
        inner: &'a MemoryFileSystem,
        big: &'a Path,
    }

    impl FileSystem for NeverReads<'_> {
        fn exists(&self, path: &Path) -> bool {
            self.inner.exists(path)
        }
        fn read_to_string(&self, path: &Path) -> std::io::Result<String> {
            assert_ne!(path, self.big, "a file over the cap must not be read");
            self.inner.read_to_string(path)
        }
        fn len(&self, path: &Path) -> std::io::Result<u64> {
            self.inner.len(path)
        }
        fn is_regular_file(&self, path: &Path) -> bool {
            self.inner.is_regular_file(path)
        }
        fn write(&self, path: &Path, contents: &str) -> std::io::Result<()> {
            self.inner.write(path, contents)
        }
        fn create_dir_all(&self, path: &Path) -> std::io::Result<()> {
            self.inner.create_dir_all(path)
        }
        fn walk_files(&self, dir: &Path) -> std::io::Result<Vec<String>> {
            self.inner.walk_files(dir)
        }
        fn list_dir(&self, dir: &Path) -> std::io::Result<Vec<String>> {
            self.inner.list_dir(dir)
        }
        fn rename(&self, from: &Path, to: &Path) -> std::io::Result<()> {
            self.inner.rename(from, to)
        }
        fn remove_file(&self, path: &Path) -> std::io::Result<()> {
            self.inner.remove_file(path)
        }
    }

    #[test]
    fn a_file_over_the_size_cap_is_not_compared() {
        let review = security_record("low", "data/dump.sql:1", "INSERT", "");
        let memory = project(SECURITY, &review)
            .with_file("/p/data/dump.sql", "x".repeat(MAX_FILE_BYTES + 1));
        let fs = NeverReads {
            inner: &memory,
            big: Path::new("/p/data/dump.sql"),
        };
        let layout = Layout::new("/p");
        let config = crate::config::resolve(&fs, &FixedEnv::at("/p"), &layout).unwrap();
        let ctx = crate::change::load(&fs, &layout, &config, ChangeId::parse("add-auth").unwrap())
            .unwrap();
        let check = check(&fs, &layout, &config, &ctx).unwrap();
        assert_eq!(codes(&check), [REVIEW_EVIDENCE_UNCHECKED]);
        assert!(check.findings[0].message.contains("4 MiB"));
    }

    #[test]
    fn a_fixed_finding_is_not_checked() {
        let review = security_record(
            "high",
            "src/removed.rs:3",
            "unsafe { x }",
            "  - status: fixed\n    by: developer\n    ref: src/removed.rs deleted\n",
        );
        let check = run(&project(SECURITY, &review));
        assert!(check.findings.is_empty(), "{:?}", check.findings);
        assert!(check.blockers.is_empty());
    }

    #[test]
    fn a_location_outside_the_project_is_not_read() {
        let review = security_record("low", "../secrets.txt:1", "password", "");
        // Were it read, the evidence would be found and nothing reported.
        let fs = project(SECURITY, &review).with_file("/secrets.txt", "password");
        let check = run(&fs);
        assert_eq!(codes(&check), [REVIEW_INVALID]);
    }

    #[test]
    fn an_active_expert_without_a_section_is_missing() {
        let check = run(&project(SECURITY, &record(NEWCOMER_NO_FINDING)));
        assert_eq!(codes(&check), [REVIEW_ROLE_MISSING, REVIEW_ROLE_MISSING]);
        assert!(check.findings[0].message.contains("security"));
        assert!(check.findings[0].message.contains("plan.challenge"));
        assert!(check.findings[1].message.contains("code.review"));
    }

    #[test]
    fn a_disabled_role_may_say_so() {
        let review = record("### newcomer\n\nDisabled by configuration.\n");
        let fs = project(NO_DOMAIN, &review)
            .with_file("/p/_codev/config.yaml", "roles:\n  newcomer: never\n");
        let check = run(&fs);
        assert!(check.findings.is_empty(), "{:?}", check.findings);
        assert!(check.blockers.is_empty());
    }

    #[test]
    fn without_a_record_only_the_gate_speaks() {
        let fs = project(NO_DOMAIN, "");
        fs.remove_file(Path::new("/p/_codev/changes/add-auth/review.md"))
            .unwrap();
        let check = run(&fs);
        assert!(check.record.is_none());
        assert!(check.findings.is_empty());
        assert_eq!(check.blockers[0].code, REVIEW_RECORD_MISSING);
    }
}
