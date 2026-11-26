//! Scoring an explore or onboard run, without a model: both skills are
//! read-only, so the run passes when it touched no file, created no change,
//! and its final message holds the mark of the case's outcome or state and
//! none of the marks of the others. Only the final message is read: the
//! narration between tool calls names files, commands and the doctor's fix
//! lines, which would let a wrong conclusion pass. Pure: it reads the
//! observation only.

use crate::case::ReadOnlyCase;
use crate::checked::{CheckScore, Checks, holds};
use crate::observed::Observed;

/// Scores one explore or onboard run. A run that stopped is scored as one
/// that left nothing to read: every check fails.
pub fn score_read_only(expect: &ReadOnlyCase, observed: &Observed) -> CheckScore {
    let seen = observed
        .read_only
        .as_ref()
        .filter(|_| observed.error.is_none());
    let mut checks = Checks::default();
    checks.check(
        "no file modified, created or removed",
        seen.is_some() && observed.strayed.is_empty(),
    );
    checks.check(
        "no change created under `_compagnon/changes/`",
        seen.is_some_and(|s| s.changes_created.is_empty()),
    );
    let last = seen.and_then(|s| s.final_message.as_deref());
    for text in &expect.contains {
        checks.check(
            format!("the final message says \"{text}\""),
            last.is_some_and(|m| holds(m, text)),
        );
    }
    for text in &expect.absent {
        checks.check(
            format!("the final message does not say \"{text}\""),
            last.is_some_and(|m| !holds(m, text)),
        );
    }
    checks.score()
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::case::{ExploreOutcome, OnboardState, Situation};
    use crate::observed::{FORMAT, ReadOnlyObserved};
    use std::collections::BTreeMap;

    fn expect(situation: Situation, contains: &[&str], absent: &[&str]) -> ReadOnlyCase {
        ReadOnlyCase {
            situation,
            contains: contains.iter().map(|t| t.to_string()).collect(),
            absent: absent.iter().map(|t| t.to_string()).collect(),
        }
    }

    fn explore(outcome: ExploreOutcome, contains: &[&str], absent: &[&str]) -> ReadOnlyCase {
        expect(
            Situation::Explore {
                request: "rate limiting for the API".into(),
                outcome,
            },
            contains,
            absent,
        )
    }

    fn onboard(state: OnboardState, contains: &[&str], absent: &[&str]) -> ReadOnlyCase {
        expect(Situation::Onboard(state), contains, absent)
    }

    /// A run that wrote `messages`, the last one its conclusion, and
    /// touched `strayed`.
    pub(crate) fn run(messages: &[&str], strayed: &[&str]) -> Observed {
        Observed {
            format: FORMAT,
            case: "explore-x".into(),
            error: None,
            cost_usd: Some(0.3),
            duration_ms: Some(1000),
            consumption: None,
            strayed: strayed.iter().map(|s| s.to_string()).collect(),
            review: None,
            files: BTreeMap::new(),
            validate: BTreeMap::new(),
            validate_before: None,
            messages: messages.iter().map(|m| m.to_string()).collect(),
            implement: None,
            merge: None,
            configure: None,
            read_only: Some(ReadOnlyObserved {
                final_message: messages.last().map(|m| m.to_string()),
                changes_created: vec![],
            }),
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
    fn a_reply_concluding_on_its_own_outcome_passes() {
        let e = explore(
            ExploreOutcome::Crystallizes,
            &["/compagnon-propose"],
            &["is missing"],
        );
        let score = score_read_only(
            &e,
            &run(&["It crystallizes. Ask me for `/compagnon-propose`."], &[]),
        );
        assert_eq!(failed(&score), Vec::<&str>::new());
        assert!(score.passed);
    }

    #[test]
    fn an_explore_run_that_writes_a_file() {
        let e = explore(
            ExploreOutcome::Crystallizes,
            &["/compagnon-propose"],
            &["is missing"],
        );
        let score = score_read_only(&e, &run(&["Ask me for /compagnon-propose."], &["notes.md"]));
        assert_eq!(failed(&score), ["no file modified, created or removed"]);
        assert!(!score.passed);
        let mut created = run(&["Ask me for /compagnon-propose."], &[]);
        created.read_only.as_mut().unwrap().changes_created =
            vec!["_compagnon/changes/rate-limit".into()];
        assert_eq!(
            failed(&score_read_only(&e, &created)),
            ["no change created under `_compagnon/changes/`"]
        );
    }

    #[test]
    fn an_onboard_run_that_recommends_the_wrong_action() {
        let e = onboard(
            OnboardState::ThinConfig,
            &["/compagnon-configure"],
            &["/compagnon-apply"],
        );
        let score = score_read_only(
            &e,
            &run(
                &["**What's next**: type `/compagnon-propose <an-idea>`."],
                &[],
            ),
        );
        assert_eq!(
            failed(&score),
            ["the final message says \"/compagnon-configure\""]
        );
    }

    #[test]
    fn a_reply_concluding_on_another_outcome() {
        let e = explore(
            ExploreOutcome::NotWorth,
            &["src/limiter.rs"],
            &["/compagnon-propose"],
        );
        let score = score_read_only(
            &e,
            &run(
                &[
                    "Reading `src/limiter.rs`.",
                    "The limiter in src/limiter.rs is a start; ask me for /compagnon-propose.",
                ],
                &[],
            ),
        );
        assert_eq!(
            failed(&score),
            ["the final message does not say \"/compagnon-propose\""]
        );
        assert!(!score.passed);
    }

    #[test]
    fn narration_is_not_the_conclusion() {
        let e = onboard(
            OnboardState::PlanningIncomplete,
            &["/compagnon-propose add-export"],
            &["/compagnon-apply"],
        );
        let score = score_read_only(
            &e,
            &run(
                &[
                    "`.claude/skills/` holds /compagnon-apply.",
                    "**What's next**: type  `/compagnon-propose  add-export` to continue.",
                ],
                &[],
            ),
        );
        assert_eq!(failed(&score), Vec::<&str>::new());
        // The marks are read in the final message only, and loosely.
        let narrated = run(&["Type /compagnon-propose add-export.", "Done."], &[]);
        assert_eq!(
            failed(&score_read_only(&e, &narrated)),
            ["the final message says \"/compagnon-propose add-export\""]
        );
    }

    #[test]
    fn a_run_in_error_fails_every_check() {
        let e = onboard(
            OnboardState::NoCompagnon,
            &["compagnon init"],
            &["/compagnon-apply"],
        );
        let mut o = run(&["Run `compagnon init`."], &[]);
        o.error = Some("timeout".into());
        let score = score_read_only(&e, &o);
        assert_eq!(score.checks_passed(), 0);
        let mut silent = run(&[], &[]);
        silent.read_only.as_mut().unwrap().final_message = None;
        assert_eq!(
            failed(&score_read_only(&e, &silent)),
            [
                "the final message says \"compagnon init\"",
                "the final message does not say \"/compagnon-apply\""
            ]
        );
    }

    /// An onboard case named `id`, expecting `/compagnon-apply`.
    pub(crate) fn onboard_case(id: &str) -> crate::case::Case {
        let mut c = crate::score::tests::a_case("planner", crate::case::Kind::Seeded, &[]);
        c.id = id.into();
        c.step = crate::case::Step::ProjectOnboard;
        c.change = String::new();
        c.read_only = Some(onboard(
            OnboardState::PlanningComplete,
            &["/compagnon-apply"],
            &["/compagnon-configure"],
        ));
        c
    }

    /// An explore case named `id`, expecting `/compagnon-propose`.
    pub(crate) fn explore_case(id: &str) -> crate::case::Case {
        let mut c = onboard_case(id);
        c.step = crate::case::Step::IdeaExplore;
        c.read_only = Some(explore(
            ExploreOutcome::Crystallizes,
            &["/compagnon-propose"],
            &["is missing"],
        ));
        c
    }

    /// A run concluding with `final`.
    pub(crate) fn run_ending(last: &str) -> Observed {
        run(&[last], &[])
    }
}
