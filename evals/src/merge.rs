//! Scoring a sync or archive run, without a model: whether the agent left
//! `_codev/` as codev's own command leaves it — or as it found it, for a
//! refusal —, touched nothing outside the step, and said what the case
//! expects. The reference command ran on a copy before the agent, so a
//! merge written by hand, a partial one or a missing move is caught
//! without the case listing what the merge touches. Pure: it reads the
//! observation only.

use std::collections::BTreeMap;

use crate::case::{Case, MergeCase, MergeOutcome};
use crate::checked::{CheckScore, Checks, holds};
use crate::observed::{Digests, Observed};

const ARCHIVE_DIR: &str = "_codev/changes/archive/";

/// The length of a date as an archive folder starts with it.
const DATE_LEN: usize = "YYYY-MM-DD".len();

/// Whether `text` is a date written `YYYY-MM-DD`.
fn is_iso_date(text: &str) -> bool {
    text.chars().enumerate().all(|(i, c)| match i {
        4 | 7 => c == '-',
        _ => c.is_ascii_digit(),
    })
}

/// A path with the date that starts an archived change's folder left out:
/// an archive made on another day than the reference's is the same.
fn without_archive_date(path: &str) -> String {
    let Some(rest) = path.strip_prefix(ARCHIVE_DIR) else {
        return path.to_string();
    };
    let dated = rest.get(..DATE_LEN).is_some_and(is_iso_date) && rest[DATE_LEN..].starts_with('-');
    if dated {
        format!("{ARCHIVE_DIR}<date>{}", &rest[DATE_LEN..])
    } else {
        path.to_string()
    }
}

fn normalized(changed: &Digests) -> BTreeMap<String, Option<&str>> {
    changed
        .iter()
        .map(|(path, digest)| (without_archive_date(path), digest.as_deref()))
        .collect()
}

/// The command a case's step runs.
fn command(case: &Case) -> &'static str {
    if case.step == crate::case::Step::ChangeSync {
        "codev sync"
    } else {
        "codev archive"
    }
}

/// Why the case no longer describes what codev does, from its reference
/// run: a `done` case whose command failed, or a `refused` case with a
/// `code` whose command succeeded or reported another code. `None` when
/// the reference agrees with the case, or when the run has none.
pub fn stale(case: &Case, expect: &MergeCase, observed: &Observed) -> Option<String> {
    let reference = &observed.merge.as_ref()?.reference;
    let reported = reference.code().unwrap_or("none");
    let exit = reference
        .exit_code
        .map_or_else(|| "a signal".to_string(), |c| c.to_string());
    match (expect.outcome, &expect.code) {
        (MergeOutcome::Done, _) if reference.exit_code != Some(0) => Some(format!(
            "case `{}`: `{}` exits with {exit} and the code `{reported}` on the fixture, where \
             the case expects it done; the case no longer describes what codev does",
            case.id,
            command(case)
        )),
        (MergeOutcome::Refused, Some(code))
            if reference.exit_code == Some(0) || reported != code =>
        {
            Some(format!(
                "case `{}`: the case expects `{}` to refuse with `{code}`, and it reports \
                 `{reported}` (exit {exit}); the case no longer describes what codev does",
                case.id,
                command(case)
            ))
        }
        _ => None,
    }
}

/// Scores one sync or archive run. A run that stopped is scored as one
/// that left nothing to compare and wrote nothing: every check that needs
/// its outcome fails.
pub fn score_merge(case: &Case, expect: &MergeCase, observed: &Observed) -> CheckScore {
    let seen = observed.merge.as_ref().filter(|_| observed.error.is_none());
    let messages = if seen.is_some() {
        observed.messages.join("\n")
    } else {
        String::new()
    };
    let mut checks = Checks::default();
    match expect.outcome {
        MergeOutcome::Done => checks.check(
            format!("files under `_codev/` as `{}` leaves them", command(case)),
            seen.is_some_and(|m| normalized(&m.changed) == normalized(&m.reference.changed)),
        ),
        MergeOutcome::Refused => checks.check(
            "files under `_codev/` as the fixture's",
            seen.is_some_and(|m| m.changed.is_empty()),
        ),
    }
    checks.check(
        "no file modified outside the step",
        seen.is_some() && observed.strayed.is_empty(),
    );
    for text in &expect.contains {
        checks.check(
            format!("says \"{text}\""),
            seen.is_some() && holds(&messages, text),
        );
    }
    for text in &expect.absent {
        checks.check(
            format!("does not say \"{text}\""),
            seen.is_some() && !holds(&messages, text),
        );
    }
    checks.score()
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::case::Step;
    use crate::observed::{FORMAT, MergeObserved, Reference};
    use serde_json::json;

    fn case(step: Step, outcome: MergeOutcome, code: Option<&str>) -> (Case, MergeCase) {
        let mut c = crate::score::tests::a_case("planner", crate::case::Kind::Seeded, &[]);
        c.id = "archive-x".into();
        c.step = step;
        c.change = "add-auth".into();
        let expect = MergeCase {
            git_ref: "abc".into(),
            outcome,
            code: code.map(str::to_string),
            contains: vec![],
            absent: vec![],
            files: BTreeMap::new(),
            overlay: BTreeMap::new(),
        };
        c.merge = Some(expect.clone());
        (c, expect)
    }

    fn digests(entries: &[(&str, Option<&str>)]) -> Digests {
        entries
            .iter()
            .map(|(p, d)| (p.to_string(), d.map(str::to_string)))
            .collect()
    }

    fn merged() -> Digests {
        digests(&[
            ("_codev/specs/doctor/spec.md", Some("d1")),
            ("_codev/changes/add-auth/proposal.md", None),
            (
                "_codev/changes/archive/2026-10-27-add-auth/proposal.md",
                Some("p"),
            ),
        ])
    }

    fn run(changed: Digests, reference: Reference, messages: &[&str]) -> Observed {
        Observed {
            format: FORMAT,
            case: "archive-x".into(),
            error: None,
            cost_usd: Some(0.2),
            duration_ms: Some(1000),
            consumption: None,
            strayed: vec![],
            review: None,
            files: BTreeMap::new(),
            validate: BTreeMap::new(),
            validate_before: None,
            messages: messages.iter().map(|m| m.to_string()).collect(),
            implement: None,
            merge: Some(MergeObserved { changed, reference }),
            configure: None,
        }
    }

    fn done_reference() -> Reference {
        Reference {
            exit_code: Some(0),
            report: Some(json!({"status": []})),
            changed: merged(),
        }
    }

    fn refused_reference(code: &str) -> Reference {
        Reference {
            exit_code: Some(1),
            report: Some(json!({"status": [{"code": code, "message": "m"}]})),
            changed: Digests::new(),
        }
    }

    fn failed(score: &CheckScore) -> Vec<&str> {
        score
            .checks
            .iter()
            .filter(|c| !c.passed)
            .map(|c| c.check.as_str())
            .collect()
    }

    #[test]
    fn a_merge_written_by_hand_fails() {
        let (c, expect) = case(Step::ChangeSync, MergeOutcome::Done, None);
        let reference = Reference {
            changed: digests(&[("_codev/specs/doctor/spec.md", Some("by-codev"))]),
            ..done_reference()
        };
        let by_hand = run(
            digests(&[("_codev/specs/doctor/spec.md", Some("by-hand"))]),
            reference.clone(),
            &[],
        );
        let score = score_merge(&c, &expect, &by_hand);
        assert_eq!(
            failed(&score),
            ["files under `_codev/` as `codev sync` leaves them"]
        );
        assert!(!score.passed);
        let through_codev = run(reference.changed.clone(), reference, &[]);
        assert!(score_merge(&c, &expect, &through_codev).passed);
    }

    #[test]
    fn a_refusal_well_made_passes_the_archive_checks() {
        let (c, mut expect) = case(
            Step::ChangeArchive,
            MergeOutcome::Refused,
            Some("validation_failed"),
        );
        expect.contains = vec!["codev validate".into()];
        let o = run(
            Digests::new(),
            refused_reference("validation_failed"),
            &["The change has errors. Run `codev validate add-auth` to see the details."],
        );
        assert_eq!(stale(&c, &expect, &o), None);
        let score = score_merge(&c, &expect, &o);
        assert_eq!(failed(&score), Vec::<&str>::new());
        assert_eq!(score.checks.len(), 3);
        let fixed_by_hand = run(
            digests(&[("_codev/changes/add-auth/proposal.md", Some("x"))]),
            refused_reference("validation_failed"),
            &["codev validate"],
        );
        assert_eq!(
            failed(&score_merge(&c, &expect, &fixed_by_hand)),
            ["files under `_codev/` as the fixture's"]
        );
    }

    #[test]
    fn the_archive_date_is_left_out() {
        let (c, expect) = case(Step::ChangeArchive, MergeOutcome::Done, None);
        let next_day = digests(&[
            ("_codev/specs/doctor/spec.md", Some("d1")),
            ("_codev/changes/add-auth/proposal.md", None),
            (
                "_codev/changes/archive/2026-10-28-add-auth/proposal.md",
                Some("p"),
            ),
        ]);
        assert!(score_merge(&c, &expect, &run(next_day, done_reference(), &[])).passed);
        let elsewhere = digests(&[
            ("_codev/specs/doctor/spec.md", Some("d1")),
            ("_codev/changes/add-auth/proposal.md", None),
            ("_codev/changes/archive/add-auth/proposal.md", Some("p")),
        ]);
        assert!(!score_merge(&c, &expect, &run(elsewhere, done_reference(), &[])).passed);
    }

    #[test]
    fn an_absent_text_in_the_reply_fails() {
        let (c, mut expect) = case(Step::ChangeSync, MergeOutcome::Done, None);
        expect.absent = vec!["ready to be archived".into()];
        let o = run(
            merged(),
            done_reference(),
            &["The change is   Ready to be archived if you want."],
        );
        assert_eq!(
            failed(&score_merge(&c, &expect, &o)),
            ["does not say \"ready to be archived\""]
        );
    }

    #[test]
    fn a_done_case_whose_reference_fails_is_in_error() {
        let (c, expect) = case(Step::ChangeArchive, MergeOutcome::Done, None);
        let o = run(Digests::new(), refused_reference("validation_failed"), &[]);
        let reason = stale(&c, &expect, &o).expect("stale");
        assert!(
            reason.contains("archive-x") && reason.contains("validation_failed"),
            "{reason}"
        );
        let score = crate::score::merge_case(&c, &o);
        assert_eq!(score.outcome.label(), "error");
        assert!(score.checks.is_none(), "a stale case is not scored");
    }

    #[test]
    fn a_refusal_for_another_code_is_in_error() {
        let (c, expect) = case(
            Step::ChangeArchive,
            MergeOutcome::Refused,
            Some("review_blocked"),
        );
        let o = run(Digests::new(), refused_reference("validation_failed"), &[]);
        let reason = stale(&c, &expect, &o).expect("stale");
        for needle in ["archive-x", "review_blocked", "validation_failed"] {
            assert!(reason.contains(needle), "{reason}");
        }
        let succeeded = run(Digests::new(), done_reference(), &[]);
        assert!(stale(&c, &expect, &succeeded).is_some());
        let (c, expect) = case(Step::ChangeArchive, MergeOutcome::Refused, None);
        assert_eq!(stale(&c, &expect, &succeeded), None);
    }

    #[test]
    fn a_stopped_run_fails_every_check() {
        let (c, mut expect) = case(Step::ChangeSync, MergeOutcome::Refused, None);
        expect.absent = vec!["Synced".into()];
        let mut o = run(Digests::new(), done_reference(), &[]);
        o.error = Some("timeout after 600 s".into());
        let score = score_merge(&c, &expect, &o);
        assert_eq!(score.checks_passed(), 0);
    }

    /// A `done` case of `step`, expecting `contains` in the reply.
    pub(crate) fn done_case(id: &str, step: Step, contains: &[&str]) -> Case {
        let (mut c, mut expect) = case(step, MergeOutcome::Done, None);
        c.id = id.into();
        expect.contains = contains.iter().map(|t| t.to_string()).collect();
        c.merge = Some(expect);
        c
    }

    /// A run of a `done` case that archived through codev, replying
    /// `messages`.
    pub(crate) fn run_through_codev(id: &str, messages: &[&str]) -> Observed {
        let mut o = run(merged(), done_reference(), messages);
        o.case = id.into();
        o
    }

    /// A run of a `done` case that wrote the merge by hand, replying
    /// `messages`.
    pub(crate) fn run_merged_by_hand(id: &str, messages: &[&str]) -> Observed {
        let by_hand = digests(&[("_codev/specs/doctor/spec.md", Some("by-hand"))]);
        let mut o = run(by_hand, done_reference(), messages);
        o.case = id.into();
        o
    }
}
