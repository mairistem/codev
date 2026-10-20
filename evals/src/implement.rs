//! Scoring an implementation run, without a model: whether `/codev-apply`
//! checked every task, left the project's checks passing, delivered the
//! behavior the hidden acceptance script checks, reviewed its code, and
//! touched nothing it must leave. Pure: the runner records the checks it
//! ran itself, the script's lines, the change's files, the changed paths,
//! `codev validate` and `codev review` before and after the run.

use serde::{Deserialize, Serialize};

use crate::case::{ImplementCase, tasks};
use crate::implement_run::{CHECKS, acceptance_results};
use crate::observed::Observed;
use crate::score::{CaseConsumption, CaseScore, Measures, Spread, ratio, spread};
use crate::update::{PROPOSE_SKILL, added_errors};

/// One check of an implementation run, and whether it passed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImplementCheck {
    pub check: String,
    pub passed: bool,
}

/// An implementation run's checks; it passes when every check passes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImplementScore {
    pub passed: bool,
    pub refuse: bool,
    pub checks: Vec<ImplementCheck>,
    /// The acceptance assertions, and how many passed.
    pub assertions: u32,
    pub assertions_passed: u32,
}

impl ImplementScore {
    pub fn checks_passed(&self) -> usize {
        self.checks.iter().filter(|c| c.passed).count()
    }
}

/// Whether `codev review --json` holds a recorded `code.review` step.
fn reviewed(review: Option<&serde_json::Value>) -> bool {
    review
        .and_then(|r| r["steps"].as_array())
        .into_iter()
        .flatten()
        .any(|s| s["step"] == "code.review" && s["recorded"] != false)
}

/// Scores one implementation run of `case`, whose change lives in
/// `change_dir`. A run that stopped is scored as one that did nothing:
/// every check that needs its outcome fails.
pub fn score_implement(
    case: &ImplementCase,
    change_dir: &str,
    observed: &Observed,
) -> ImplementScore {
    let mut checks = Vec::new();
    let mut check = |check: String, passed: bool| checks.push(ImplementCheck { check, passed });
    let (assertions, assertions_passed) = if case.refuse {
        score_refusal(observed, &mut check);
        (0, 0)
    } else {
        score_implementation(case, change_dir, observed, &mut check)
    };
    ImplementScore {
        passed: checks.iter().all(|c| c.passed),
        refuse: case.refuse,
        checks,
        assertions,
        assertions_passed,
    }
}

/// What the runner recorded after the run, `None` when the run stopped.
fn seen(observed: &Observed) -> Option<&crate::observed::ImplementObserved> {
    observed
        .implement
        .as_ref()
        .filter(|_| observed.error.is_none())
}

/// A refusal changes no file, points to `/codev-propose`, and reads
/// nothing of the cases' repository.
fn score_refusal(observed: &Observed, check: &mut impl FnMut(String, bool)) {
    let seen = seen(observed);
    check(
        "no file changed".to_string(),
        seen.is_some_and(|s| s.changed.is_empty()),
    );
    let messages = if seen.is_some() {
        observed.messages.join("\n")
    } else {
        String::new()
    };
    check(
        format!("points to `{PROPOSE_SKILL}`"),
        messages.contains(PROPOSE_SKILL),
    );
    check(
        "no read of the cases' repository".to_string(),
        seen.is_some_and(|s| s.read_repository.is_empty()),
    );
}

/// An implementation's checks, in order; returns how many acceptance
/// assertions there are and how many passed.
fn score_implementation(
    case: &ImplementCase,
    change_dir: &str,
    observed: &Observed,
    check: &mut impl FnMut(String, bool),
) -> (u32, u32) {
    let stopped = observed.error.is_some();
    let seen = seen(observed);
    let file = |name: &str| -> Option<&str> {
        if stopped {
            return None;
        }
        observed
            .files
            .get(&format!("{change_dir}/{name}"))
            .and_then(|t| t.as_deref())
    };
    let planned = case
        .files
        .get("tasks.md")
        .map(|t| tasks(t))
        .unwrap_or_default();
    let done = file("tasks.md").map(tasks).unwrap_or_default();
    let left: Vec<&str> = planned
        .iter()
        .filter(|(id, _)| !done.iter().any(|(d, checked)| d == id && *checked))
        .map(|(id, _)| id.as_str())
        .collect();
    check(
        if left.is_empty() {
            "every task checked".to_string()
        } else {
            format!("every task checked ({} left)", left.join(", "))
        },
        file("tasks.md").is_some() && left.is_empty(),
    );
    for command in &CHECKS {
        let passed =
            seen.is_some_and(|s| s.checks.iter().any(|c| c.name == command.name && c.passed));
        check(format!("`{}` passes", command.name), passed);
    }
    let printed = seen.map(|s| s.printed.clone()).unwrap_or_default();
    let (mut assertions, mut assertions_passed) = (0, 0);
    for (name, ok) in acceptance_results(&printed, &case.acceptance) {
        assertions += 1;
        if ok {
            assertions_passed += 1;
        }
        check(format!("acceptance `{name}`"), ok);
    }
    check(
        "the review record holds `code.review`".to_string(),
        !stopped && reviewed(observed.review.as_ref()),
    );
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
    check(
        "nothing modified outside the step".to_string(),
        !stopped && observed.strayed.is_empty(),
    );
    // Archiving moves the whole folder: a change whose proposal is still in
    // its folder was not archived.
    check(
        "the change is still in `_codev/changes/`".to_string(),
        file("proposal.md").is_some(),
    );
    check(
        "no read of the cases' repository".to_string(),
        seen.is_some_and(|s| s.read_repository.is_empty()),
    );
    if case.docs {
        for language in ["en", "fr"] {
            let prefix = format!("docs/{language}/");
            check(
                format!("`{prefix}` touched"),
                seen.is_some_and(|s| s.changed.iter().any(|p| p.starts_with(&prefix))),
            );
        }
    }
    (assertions, assertions_passed)
}

/// One repetition of the implementation runs, pooled. The refusing runs
/// are counted apart, so that a refusal, easy to pass, does not hide a
/// failed implementation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImplementRepetition {
    pub repetition: u32,
    /// The runs of the cases that do not refuse.
    pub runs: u32,
    pub passed: u32,
    pub checks: u32,
    pub checks_passed: u32,
    pub assertions: u32,
    pub assertions_passed: u32,
    pub refusals: u32,
    pub refusals_passed: u32,
    /// The runs, refusing or not, that modified a file outside the step.
    pub strayed: u32,
    pub pass_rate: Option<f64>,
    pub check_rate: Option<f64>,
    pub acceptance_rate: Option<f64>,
    pub refusal_rate: Option<f64>,
}

/// The step `code.implement`: its measures per repetition, then their
/// median and range. No threshold: `compare` judges them against the
/// baseline.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImplementStepScore {
    pub cases: u32,
    pub repetitions: Vec<ImplementRepetition>,
    /// The share of runs that pass, refusals left out.
    pub pass_rate: Option<Spread>,
    /// The share of checks that pass, refusals left out.
    pub check_rate: Option<Spread>,
    /// The share of acceptance assertions that pass.
    pub acceptance_rate: Option<Spread>,
    /// The share of refusing runs that pass.
    pub refusal_rate: Option<Spread>,
    /// The runs that modified a file outside the step.
    pub strayed: Option<Spread>,
    /// The sums of every implementation case's median consumption.
    pub consumption: Measures,
}

/// The implementation section: each repetition's runs pooled, then
/// medians and ranges; `None` without an implementation run.
pub fn step_score(
    numbers: &[u32],
    cases: &[CaseScore],
    case_consumption: &[CaseConsumption],
) -> Option<ImplementStepScore> {
    let runs: Vec<(&CaseScore, &ImplementScore)> = cases
        .iter()
        .filter_map(|c| c.implement.as_ref().map(|s| (c, s)))
        .collect();
    if runs.is_empty() {
        return None;
    }
    let repetitions: Vec<ImplementRepetition> = numbers
        .iter()
        .filter_map(|n| {
            let these: Vec<&(&CaseScore, &ImplementScore)> =
                runs.iter().filter(|(c, _)| c.repetition == *n).collect();
            if these.is_empty() {
                return None;
            }
            let (refusing, building): (Vec<&(&CaseScore, &ImplementScore)>, Vec<_>) =
                these.iter().copied().partition(|(_, s)| s.refuse);
            let count = building.len() as u32;
            let passed = building.iter().filter(|(_, s)| s.passed).count() as u32;
            let checks = building.iter().map(|(_, s)| s.checks.len()).sum::<usize>() as u32;
            let checks_passed = building
                .iter()
                .map(|(_, s)| s.checks_passed())
                .sum::<usize>() as u32;
            let assertions = building.iter().map(|(_, s)| s.assertions).sum::<u32>();
            let assertions_passed = building
                .iter()
                .map(|(_, s)| s.assertions_passed)
                .sum::<u32>();
            let refusals = refusing.len() as u32;
            let refusals_passed = refusing.iter().filter(|(_, s)| s.passed).count() as u32;
            Some(ImplementRepetition {
                repetition: *n,
                runs: count,
                passed,
                checks,
                checks_passed,
                assertions,
                assertions_passed,
                refusals,
                refusals_passed,
                strayed: these.iter().filter(|(c, _)| !c.strayed.is_empty()).count() as u32,
                pass_rate: ratio(passed, count),
                check_rate: ratio(checks_passed, checks),
                acceptance_rate: ratio(assertions_passed, assertions),
                refusal_rate: ratio(refusals_passed, refusals),
            })
        })
        .collect();
    let values = |f: fn(&ImplementRepetition) -> Option<f64>| -> Vec<f64> {
        repetitions.iter().filter_map(f).collect()
    };
    let mut distinct: Vec<&str> = runs.iter().map(|(c, _)| c.id.as_str()).collect();
    distinct.sort();
    distinct.dedup();
    Some(ImplementStepScore {
        cases: distinct.len() as u32,
        pass_rate: spread(&values(|r| r.pass_rate)),
        check_rate: spread(&values(|r| r.check_rate)),
        acceptance_rate: spread(&values(|r| r.acceptance_rate)),
        refusal_rate: spread(&values(|r| r.refusal_rate)),
        strayed: spread(&values(|r| Some(f64::from(r.strayed)))),
        consumption: Measures::sum_of(
            case_consumption
                .iter()
                .filter(|c| distinct.contains(&c.id.as_str()))
                .map(|c| &c.measures),
        ),
        repetitions,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::implement_run::CommandCheck;
    use crate::observed::{FORMAT, ImplementObserved};
    use serde_json::json;
    use std::collections::BTreeMap;

    const CHANGE_DIR: &str = "_codev/changes/decision-accept-command";
    const TASKS: &str = "## 1. Code\n\n- [ ] 1.1 A\n- [ ] 2.3 B\n";

    fn case(refuse: bool, docs: bool) -> ImplementCase {
        let mut files: BTreeMap<String, String> =
            BTreeMap::from([("proposal.md".to_string(), "# Proposal\n".to_string())]);
        if !refuse {
            files.insert("tasks.md".into(), TASKS.into());
        }
        ImplementCase {
            git_ref: "abc".into(),
            merged: "def".into(),
            refuse,
            docs,
            acceptance: if refuse {
                vec![]
            } else {
                vec!["works".into(), "refuses-twice".into()]
            },
            script: (!refuse).then(|| "echo ok works\n".to_string()),
            files,
        }
    }

    fn validate(errors: &[(&str, &str)]) -> serde_json::Value {
        let findings: Vec<_> = errors
            .iter()
            .map(|(path, code)| {
                json!({"path": path, "severity": "error", "code": code, "message": "m"})
            })
            .collect();
        json!({"items": [{"kind": "change", "findings": findings}]})
    }

    /// A run that did everything right.
    fn good_run() -> Observed {
        let files = BTreeMap::from([
            (
                format!("{CHANGE_DIR}/proposal.md"),
                Some("# Proposal\n".to_string()),
            ),
            (
                format!("{CHANGE_DIR}/tasks.md"),
                Some("## 1. Code\n\n- [x] 1.1 A\n- [x] 2.3 B\n".to_string()),
            ),
        ]);
        let checks = CHECKS
            .iter()
            .map(|c| CommandCheck {
                name: c.name.to_string(),
                passed: true,
                timed_out: false,
                tail: String::new(),
            })
            .collect();
        Observed {
            format: FORMAT,
            case: "implement-x".into(),
            error: None,
            cost_usd: Some(5.0),
            duration_ms: Some(1000),
            consumption: None,
            strayed: vec![],
            review: Some(json!({"steps": [{"step": "code.review", "recorded": true}]})),
            files,
            validate: BTreeMap::from([("decision-accept-command".to_string(), validate(&[]))]),
            validate_before: Some(validate(&[])),
            messages: vec!["Done.".into()],
            implement: Some(ImplementObserved {
                checks,
                script: None,
                printed: vec![("works".into(), true), ("refuses-twice".into(), true)],
                changed: vec![
                    "crates/codev-cli/src/main.rs".into(),
                    "docs/en/src/cli.md".into(),
                    "docs/fr/src/cli.md".into(),
                ],
                read_repository: vec![],
            }),
        }
    }

    fn failed(score: &ImplementScore) -> Vec<&str> {
        score
            .checks
            .iter()
            .filter(|c| !c.passed)
            .map(|c| c.check.as_str())
            .collect()
    }

    #[test]
    fn a_run_that_does_everything_passes() {
        let score = score_implement(&case(false, true), CHANGE_DIR, &good_run());
        assert!(score.passed, "{:?}", failed(&score));
        assert_eq!((score.assertions, score.assertions_passed), (2, 2));
    }

    #[test]
    fn a_run_that_leaves_a_task_unchecked() {
        let mut run = good_run();
        run.files.insert(
            format!("{CHANGE_DIR}/tasks.md"),
            Some("## 1. Code\n\n- [x] 1.1 A\n- [ ] 2.3 B\n".to_string()),
        );
        let score = score_implement(&case(false, true), CHANGE_DIR, &run);
        assert!(!score.passed);
        assert_eq!(failed(&score), ["every task checked (2.3 left)"]);
    }

    #[test]
    fn a_run_whose_tests_fail() {
        let mut run = good_run();
        let seen = run.implement.as_mut().unwrap();
        seen.checks
            .iter_mut()
            .find(|c| c.name == "tests")
            .unwrap()
            .passed = false;
        seen.printed = vec![("works".into(), true), ("refuses-twice".into(), false)];
        let score = score_implement(&case(false, true), CHANGE_DIR, &run);
        assert!(!score.passed);
        assert_eq!(
            failed(&score),
            ["`tests` passes", "acceptance `refuses-twice`"]
        );
        assert_eq!((score.assertions, score.assertions_passed), (2, 1));
    }

    #[test]
    fn a_refusal_well_made() {
        let mut run = good_run();
        run.messages = vec!["Planning is not ready: run `/codev-propose` first.".into()];
        run.implement.as_mut().unwrap().changed.clear();
        let score = score_implement(&case(true, false), CHANGE_DIR, &run);
        assert!(score.passed, "{:?}", failed(&score));
        assert!(score.refuse);
        run.implement
            .as_mut()
            .unwrap()
            .changed
            .push("src/lib.rs".into());
        let score = score_implement(&case(true, false), CHANGE_DIR, &run);
        assert_eq!(failed(&score), ["no file changed"]);
    }

    #[test]
    fn a_run_that_reads_the_merged_solution() {
        let mut run = good_run();
        run.implement.as_mut().unwrap().read_repository = vec!["git -C /src/codev show def".into()];
        let score = score_implement(&case(false, true), CHANGE_DIR, &run);
        assert_eq!(failed(&score), ["no read of the cases' repository"]);
    }

    #[test]
    fn a_run_that_adds_a_validation_error() {
        let mut run = good_run();
        // The fixture already had one error; the run kept it, then added one.
        run.validate_before = Some(validate(&[("tasks.md", "task_format")]));
        run.validate.insert(
            "decision-accept-command".into(),
            validate(&[("tasks.md", "task_format")]),
        );
        let score = score_implement(&case(false, true), CHANGE_DIR, &run);
        assert!(score.passed, "an error already there is not the run's");
        run.validate.insert(
            "decision-accept-command".into(),
            validate(&[
                ("tasks.md", "task_format"),
                ("review.md", "review_evidence"),
            ]),
        );
        let score = score_implement(&case(false, true), CHANGE_DIR, &run);
        assert_eq!(
            failed(&score),
            ["no validation error added (review_evidence in review.md)"]
        );
    }

    #[test]
    fn a_run_without_documentation_when_expected() {
        let mut run = good_run();
        run.implement
            .as_mut()
            .unwrap()
            .changed
            .retain(|p| !p.starts_with("docs/fr/"));
        let score = score_implement(&case(false, true), CHANGE_DIR, &run);
        assert_eq!(failed(&score), ["`docs/fr/` touched"]);
        let score = score_implement(&case(false, false), CHANGE_DIR, &run);
        assert!(score.passed, "a case that expects no documentation");
    }

    #[test]
    fn a_run_without_review_or_that_strays_fails() {
        let mut run = good_run();
        run.review = Some(json!({"steps": [{"step": "plan.challenge", "recorded": true}]}));
        run.strayed = vec!["_codev/specs/decisions/spec.md".into()];
        let score = score_implement(&case(false, true), CHANGE_DIR, &run);
        assert_eq!(
            failed(&score),
            [
                "the review record holds `code.review`",
                "nothing modified outside the step"
            ]
        );
    }

    #[test]
    fn a_stopped_run_fails_every_check_that_needs_its_outcome() {
        let mut run = good_run();
        run.error = Some("timeout after 5400 s".into());
        let score = score_implement(&case(false, true), CHANGE_DIR, &run);
        assert!(!score.passed);
        assert_eq!(score.checks_passed(), 0);
    }

    fn scored(id: &str, n: u32, refuse: bool, passed: bool, assertions: (u32, u32)) -> CaseScore {
        let mut score = crate::score::tests_support::bare_score(id, "code.implement", n);
        score.implement = Some(ImplementScore {
            passed,
            refuse,
            checks: vec![ImplementCheck {
                check: "c".into(),
                passed,
            }],
            assertions: assertions.0,
            assertions_passed: assertions.1,
        });
        score
    }

    #[test]
    fn a_refusal_is_kept_out_of_the_pass_share() {
        let cases = vec![
            scored("implement-a", 1, false, false, (4, 2)),
            scored("implement-b", 1, false, true, (6, 6)),
            scored("implement-refuse", 1, true, true, (0, 0)),
        ];
        let step = step_score(&[1], &cases, &[]).unwrap();
        assert_eq!(step.cases, 3);
        let r = &step.repetitions[0];
        assert_eq!(
            (r.runs, r.passed, r.refusals, r.refusals_passed),
            (2, 1, 1, 1)
        );
        assert_eq!(step.pass_rate.unwrap().median, 0.5);
        assert_eq!(step.acceptance_rate.unwrap().median, 0.8);
        assert_eq!(step.refusal_rate.unwrap().median, 1.0);
    }
}
