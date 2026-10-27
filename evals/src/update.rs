//! Scoring an update run, without a model: whether `/codev-update` revised
//! what it should, left the rest byte-identical, named the ripple instead
//! of editing it, kept the change valid, and refused what it must. Every
//! check is one a reader can redo by hand on the observation. Pure: the
//! runner records the change's files, the agent's messages and `codev
//! validate` before and after the run.

use std::collections::BTreeMap;

use crate::case::UpdateCase;
use crate::checked::{self, Check, CheckRepetition, CheckScore, CheckStepScore, Checks, loose};
use crate::observed::Observed;
use crate::score::{CaseConsumption, CaseScore};

/// The skill a refusal must point to for a missing artifact.
pub const PROPOSE_SKILL: &str = "/codev-propose";

/// One check of an update run, and whether it passed.
pub type UpdateCheck = Check;

/// An update run's checks; it passes when every check passes.
pub type UpdateScore = CheckScore;

/// Text compared as a file is: line endings aside, so a checkout that
/// converts them does not read as an edit.
pub(crate) fn same(a: &str, b: &str) -> bool {
    a.replace('\r', "") == b.replace('\r', "")
}

/// The errors of a `codev validate --json` report, counted by file and
/// code. The message is left out: it may cite a line the revision moved.
fn errors(report: Option<&serde_json::Value>) -> BTreeMap<(String, String), usize> {
    let mut counts = BTreeMap::new();
    let findings = report
        .and_then(|r| r["items"].as_array())
        .into_iter()
        .flatten()
        .flat_map(|item| item["findings"].as_array().into_iter().flatten());
    for f in findings.filter(|f| f["severity"] == "error") {
        let key = (
            f["path"].as_str().unwrap_or("").to_string(),
            f["code"].as_str().unwrap_or("").to_string(),
        );
        *counts.entry(key).or_insert(0) += 1;
    }
    counts
}

/// The errors `after` reports beyond those `before` already had.
pub(crate) fn added_errors(
    before: Option<&serde_json::Value>,
    after: Option<&serde_json::Value>,
) -> Vec<String> {
    let before = errors(before);
    errors(after)
        .into_iter()
        .filter(|(key, n)| *n > before.get(key).copied().unwrap_or(0))
        .map(|((path, code), _)| format!("{code} in {path}"))
        .collect()
}

/// Scores one update run of `case`, whose change lives in `change_dir`. A
/// run that stopped is scored as one that left no file and wrote nothing:
/// every check that needs the outcome fails.
pub fn score_update(case: &UpdateCase, change_dir: &str, observed: &Observed) -> UpdateScore {
    let stopped = observed.error.is_some();
    let prefix = format!("{change_dir}/");
    let after: BTreeMap<&str, &str> = if stopped {
        BTreeMap::new()
    } else {
        observed
            .files
            .iter()
            .filter_map(|(path, text)| Some((path.strip_prefix(&prefix)?, text.as_deref()?)))
            .collect()
    };
    let messages = if stopped {
        String::new()
    } else {
        observed.messages.join("\n")
    };
    // The loader checked that every expected path is a file of `change/`,
    // so `case.files[path]` exists for each of them.
    let unchanged = |path: &str| {
        after
            .get(path)
            .is_some_and(|text| same(text, &case.files[path]))
    };
    let mut checks = Checks::default();
    let mut check = |check: String, passed: bool| checks.check(check, passed);

    for path in &case.revised {
        let changed = after
            .get(path.as_str())
            .is_some_and(|text| !same(text, &case.files[path]));
        check(format!("`{path}` revised"), changed);
    }
    for path in &case.kept {
        check(format!("`{path}` kept"), unchanged(path));
    }
    for path in &case.ripple {
        check(format!("ripple `{path}` left unedited"), unchanged(path));
        check(
            format!("ripple `{path}` named"),
            messages.contains(path.as_str()),
        );
    }
    for text in &case.contains {
        let wanted = loose(text);
        let held = case
            .revised
            .iter()
            .filter_map(|path| after.get(path.as_str()))
            .any(|revised| loose(revised).contains(&wanted));
        check(format!("holds \"{text}\""), held);
    }
    // An update run records one report after the run, under its change's
    // name, the last segment of `change_dir`.
    let change = change_dir.rsplit('/').next().unwrap_or(change_dir);
    let validate_after = if stopped {
        None
    } else {
        observed.validate.get(change)
    };
    let added = added_errors(observed.validate_before.as_ref(), validate_after);
    check(
        if added.is_empty() {
            "no validation error added".to_string()
        } else {
            format!("no validation error added ({})", added.join(", "))
        },
        validate_after.is_some() && added.is_empty(),
    );
    if case.refuse {
        let untouched =
            after.len() == case.files.len() && case.files.keys().all(|path| unchanged(path));
        check("change folder untouched".to_string(), untouched);
        check(
            format!("points to `{PROPOSE_SKILL}`"),
            messages.contains(PROPOSE_SKILL),
        );
    }
    checks.score()
}

/// One repetition of the update runs, pooled.
pub type UpdateRepetition = CheckRepetition;

/// The step `plan.update`: its measures per repetition, then their median
/// and range. No threshold: `compare` judges them against the baseline.
pub type UpdateStepScore = CheckStepScore;

/// The update section: each repetition's runs pooled, then medians and
/// ranges; `None` without an update run.
pub fn step_score(
    numbers: &[u32],
    cases: &[CaseScore],
    case_consumption: &[CaseConsumption],
) -> Option<UpdateStepScore> {
    checked::step_score(numbers, cases, case_consumption, |c| c.update.as_ref())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::case::Artifact;
    use crate::observed::FORMAT;
    use serde_json::json;

    const CHANGE_DIR: &str = "_codev/changes/forward-compatible-config";

    fn case(revised: &[&str], kept: &[&str], ripple: &[&str], refuse: bool) -> UpdateCase {
        let files = [
            "proposal.md",
            "design.md",
            "tasks.md",
            "specs/doctor/spec.md",
        ]
        .iter()
        .map(|p| (p.to_string(), format!("# {p}\n\nOriginal text.\n")))
        .collect();
        let list = |v: &[&str]| v.iter().map(|s| s.to_string()).collect();
        UpdateCase {
            git_ref: "abc".into(),
            artifact: Artifact::Proposal,
            request: "r".into(),
            revised: list(revised),
            kept: list(kept),
            ripple: list(ripple),
            contains: vec![],
            refuse,
            files,
        }
    }

    fn validate(errors: &[(&str, &str)]) -> serde_json::Value {
        let findings: Vec<_> = errors
            .iter()
            .map(|(path, code)| {
                json!({"path": path, "line": 3, "severity": "error", "code": code, "message": "m"})
            })
            .collect();
        json!({"items": [{"kind": "change", "findings": findings}], "status": []})
    }

    /// A run that leaves the case's files as they were, except `edits`
    /// (path, new text), and wrote `messages`.
    fn run(case: &UpdateCase, edits: &[(&str, &str)], messages: &[&str]) -> Observed {
        let mut files: BTreeMap<String, Option<String>> = case
            .files
            .iter()
            .map(|(p, t)| (format!("{CHANGE_DIR}/{p}"), Some(t.clone())))
            .collect();
        for (path, text) in edits {
            files.insert(format!("{CHANGE_DIR}/{path}"), Some(text.to_string()));
        }
        Observed {
            format: FORMAT,
            case: "update-x".into(),
            error: None,
            cost_usd: Some(0.5),
            duration_ms: Some(1000),
            consumption: None,
            strayed: vec![],
            review: None,
            files,
            validate: BTreeMap::from([("forward-compatible-config".to_string(), validate(&[]))]),
            validate_before: Some(validate(&[])),
            messages: messages.iter().map(|m| m.to_string()).collect(),
            implement: None,
            merge: None,
            configure: None,
        }
    }

    fn failed(score: &UpdateScore) -> Vec<&str> {
        score
            .checks
            .iter()
            .filter(|c| !c.passed)
            .map(|c| c.check.as_str())
            .collect()
    }

    #[test]
    fn a_revision_that_reports_its_ripple_passes() {
        let c = case(
            &["proposal.md"],
            &["design.md", "tasks.md"],
            &["specs/doctor/spec.md"],
            false,
        );
        let o = run(
            &c,
            &[("proposal.md", "# Proposal\n\nRevised.\n")],
            &["The file `specs/doctor/spec.md` is now orphaned; delete it?"],
        );
        let score = score_update(&c, CHANGE_DIR, &o);
        assert_eq!(failed(&score), Vec::<&str>::new());
        assert!(score.passed);
        assert_eq!(score.checks.len(), 6);
    }

    #[test]
    fn editing_the_ripple_fails_the_run() {
        let c = case(&["proposal.md"], &[], &["specs/doctor/spec.md"], false);
        let o = run(
            &c,
            &[
                ("proposal.md", "# Proposal\n\nRevised.\n"),
                ("specs/doctor/spec.md", "# Emptied\n"),
            ],
            &["I removed `specs/doctor/spec.md` too."],
        );
        let score = score_update(&c, CHANGE_DIR, &o);
        assert_eq!(
            failed(&score),
            ["ripple `specs/doctor/spec.md` left unedited"]
        );
        assert!(!score.passed);
    }

    #[test]
    fn a_ripple_the_agent_never_mentions_is_not_named() {
        let c = case(&["proposal.md"], &[], &["specs/doctor/spec.md"], false);
        let o = run(
            &c,
            &[("proposal.md", "Revised.\n")],
            &["Done: proposal revised."],
        );
        let score = score_update(&c, CHANGE_DIR, &o);
        assert_eq!(failed(&score), ["ripple `specs/doctor/spec.md` named"]);
    }

    #[test]
    fn a_refusal_well_made_passes() {
        let mut c = case(&[], &["proposal.md", "tasks.md"], &[], true);
        c.artifact = Artifact::Design;
        c.files.remove("design.md");
        let o = run(
            &c,
            &[],
            &["The change has no design.md: run `/codev-propose` to create it."],
        );
        let score = score_update(&c, CHANGE_DIR, &o);
        assert_eq!(failed(&score), Vec::<&str>::new());
        assert!(score.passed);
        let created = run(&c, &[("design.md", "# Design\n")], &["/codev-propose"]);
        assert_eq!(
            failed(&score_update(&c, CHANGE_DIR, &created)),
            ["change folder untouched"]
        );
        let silent = run(&c, &[], &["I cannot do that."]);
        assert_eq!(
            failed(&score_update(&c, CHANGE_DIR, &silent)),
            ["points to `/codev-propose`"]
        );
    }

    #[test]
    fn an_error_already_in_the_fixture_passes_validity() {
        let c = case(&["design.md"], &[], &[], false);
        let mut o = run(&c, &[("design.md", "# Design\n\nRevised.\n")], &[]);
        let existing = [("_codev/changes/c/review.md", "review_role_missing")];
        o.validate_before = Some(validate(&existing));
        o.validate
            .insert("forward-compatible-config".into(), validate(&existing));
        let score = score_update(&c, CHANGE_DIR, &o);
        assert!(score.passed, "{:?}", failed(&score));
        o.validate.insert(
            "forward-compatible-config".into(),
            validate(&[
                existing[0],
                ("_codev/changes/c/design.md", "decision_missing"),
            ]),
        );
        let score = score_update(&c, CHANGE_DIR, &o);
        assert_eq!(
            failed(&score),
            ["no validation error added (decision_missing in _codev/changes/c/design.md)"]
        );
    }

    #[test]
    fn expected_text_ignores_case_and_spaces() {
        let mut c = case(&["design.md"], &[], &[], false);
        c.contains = vec!["Sessions live in Redis".into()];
        let o = run(
            &c,
            &[("design.md", "# Design\n\nThe sessions live in   Redis.\n")],
            &[],
        );
        assert!(score_update(&c, CHANGE_DIR, &o).passed);
        let o = run(
            &c,
            &[("design.md", "# Design\n\nThe sessions live in Postgres.\n")],
            &[],
        );
        assert_eq!(
            failed(&score_update(&c, CHANGE_DIR, &o)),
            ["holds \"Sessions live in Redis\""]
        );
    }

    #[test]
    fn a_stopped_run_fails_every_check_that_needs_its_outcome() {
        let c = case(&["proposal.md"], &["design.md"], &[], false);
        let mut o = run(&c, &[("proposal.md", "Revised.\n")], &[]);
        o.error = Some("timeout after 1800 s".into());
        let score = score_update(&c, CHANGE_DIR, &o);
        assert!(!score.passed);
        assert_eq!(score.checks_passed(), 0);
    }

    #[test]
    fn the_update_measures_are_medians_over_repetitions() {
        use crate::score::{Thresholds, suite, update_case};
        let expect = case(&["proposal.md"], &["design.md"], &[], false);
        let mut a = crate::score::tests::a_case("planner", crate::case::Kind::Seeded, &[]);
        a.id = "update-a".into();
        a.step = crate::case::Step::PlanUpdate;
        a.change = "forward-compatible-config".into();
        a.update = Some(expect.clone());
        let mut b = a.clone();
        b.id = "update-b".into();
        let good = run(&expect, &[("proposal.md", "Revised.\n")], &[]);
        let bad = run(&expect, &[], &[]);
        let mut strayed = good.clone();
        strayed.strayed = vec!["src/lib.rs".into()];
        let scores = vec![
            update_case(&a, &good).at(1),
            update_case(&b, &bad).at(1),
            update_case(&a, &good).at(2),
            update_case(&b, &good).at(2),
            update_case(&a, &strayed).at(3),
            update_case(&b, &bad).at(3),
        ];
        let s = suite(crate::score::tests::run(), scores, Thresholds::default());
        assert!(s.roles.is_empty(), "update runs count for no role");
        let u = s.update.expect("an update section");
        assert_eq!(u.cases, 2);
        assert_eq!(u.repetitions.len(), 3);
        let rate = u.pass_rate.unwrap();
        assert_eq!((rate.median, rate.min, rate.max), (0.5, 0.5, 1.0));
        assert_eq!(u.check_rate.unwrap().median, 0.833);
        let strayed = u.strayed.unwrap();
        assert_eq!((strayed.median, strayed.max), (0.0, 1.0));
        assert!(
            s.pass,
            "a failed check does not fail the suite: {:?}",
            s.failures
        );
    }

    #[test]
    fn a_line_ending_conversion_is_not_an_edit() {
        let c = case(&["proposal.md"], &["design.md"], &[], false);
        let crlf = c.files["design.md"].replace('\n', "\r\n");
        let o = run(
            &c,
            &[("proposal.md", "Revised.\n"), ("design.md", crlf.as_str())],
            &[],
        );
        assert!(score_update(&c, CHANGE_DIR, &o).passed);
    }
}
