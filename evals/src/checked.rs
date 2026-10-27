//! What the check-scored steps share — `plan.update`, `change.sync`,
//! `change.archive` and `configure.enrich`: a run's named checks, the
//! loose text comparison, the measures per repetition (the share of runs
//! that pass, the share of checks that pass, the runs straying), and how
//! `compare` judges them. Each step brings its own checks; the measures
//! and the verdict are the same, so that a reader learns them once.

use std::collections::BTreeMap;
use std::fmt::Write as _;

use serde::{Deserialize, Serialize};

use crate::score::{CaseConsumption, CaseScore, Measures, Spread, number, ratio, spread};

/// One check of a run, and whether it passed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Check {
    pub check: String,
    pub passed: bool,
}

/// A run's checks; it passes when every check passes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckScore {
    pub passed: bool,
    pub checks: Vec<Check>,
}

impl CheckScore {
    pub fn checks_passed(&self) -> usize {
        self.checks.iter().filter(|c| c.passed).count()
    }

    /// The score of `checks`, passing when all of them pass.
    pub fn of(checks: Vec<Check>) -> CheckScore {
        CheckScore {
            passed: checks.iter().all(|c| c.passed),
            checks,
        }
    }
}

/// The checks of a run as they are written: `check(name, passed)`.
#[derive(Debug, Default)]
pub struct Checks(pub Vec<Check>);

impl Checks {
    pub fn check(&mut self, check: impl Into<String>, passed: bool) {
        self.0.push(Check {
            check: check.into(),
            passed,
        });
    }

    pub fn score(self) -> CheckScore {
        CheckScore::of(self.0)
    }
}

/// Text compared ignoring case and runs of whitespace.
pub fn loose(text: &str) -> String {
    text.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

/// Whether `text` holds `wanted`, ignoring case and runs of whitespace.
pub fn holds(text: &str, wanted: &str) -> bool {
    loose(text).contains(&loose(wanted))
}

/// One repetition of a step's runs, pooled.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckRepetition {
    pub repetition: u32,
    pub runs: u32,
    /// The runs whose every check passed.
    pub passed: u32,
    pub checks: u32,
    pub checks_passed: u32,
    /// The runs that modified a file outside the step.
    pub strayed: u32,
    pub pass_rate: Option<f64>,
    pub check_rate: Option<f64>,
}

/// A check-scored step: its measures per repetition, then their median
/// and range. No threshold: `compare` judges them against the baseline.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckStepScore {
    pub cases: u32,
    pub repetitions: Vec<CheckRepetition>,
    /// The share of runs that pass.
    pub pass_rate: Option<Spread>,
    /// The share of checks that pass.
    pub check_rate: Option<Spread>,
    /// The runs that modified a file outside the step.
    pub strayed: Option<Spread>,
    /// The sums of every case's median consumption.
    pub consumption: Measures,
}

/// The section of a step: each repetition's runs pooled, then medians and
/// ranges; `None` without a run. `checks` gives a run's checks when it is
/// one of the step's.
pub fn step_score(
    numbers: &[u32],
    cases: &[CaseScore],
    case_consumption: &[CaseConsumption],
    checks: impl Fn(&CaseScore) -> Option<&CheckScore>,
) -> Option<CheckStepScore> {
    let runs: Vec<(&CaseScore, &CheckScore)> = cases
        .iter()
        .filter_map(|c| checks(c).map(|s| (c, s)))
        .collect();
    if runs.is_empty() {
        return None;
    }
    let repetitions: Vec<CheckRepetition> = numbers
        .iter()
        .filter_map(|n| {
            let these: Vec<&(&CaseScore, &CheckScore)> =
                runs.iter().filter(|(c, _)| c.repetition == *n).collect();
            if these.is_empty() {
                return None;
            }
            let count = these.len() as u32;
            let passed = these.iter().filter(|(_, s)| s.passed).count() as u32;
            let checks = these.iter().map(|(_, s)| s.checks.len()).sum::<usize>() as u32;
            let checks_passed = these.iter().map(|(_, s)| s.checks_passed()).sum::<usize>() as u32;
            Some(CheckRepetition {
                repetition: *n,
                runs: count,
                passed,
                checks,
                checks_passed,
                strayed: these.iter().filter(|(c, _)| !c.strayed.is_empty()).count() as u32,
                pass_rate: ratio(passed, count),
                check_rate: ratio(checks_passed, checks),
            })
        })
        .collect();
    let values = |f: fn(&CheckRepetition) -> Option<f64>| -> Vec<f64> {
        repetitions.iter().filter_map(f).collect()
    };
    let mut distinct: Vec<&str> = runs.iter().map(|(c, _)| c.id.as_str()).collect();
    distinct.dedup();
    Some(CheckStepScore {
        cases: distinct.len() as u32,
        pass_rate: spread(&values(|r| r.pass_rate)),
        check_rate: spread(&values(|r| r.check_rate)),
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

/// The quality failures of a step, judged as the planning step is: a
/// share of runs passing under the baseline's minimum, or more runs
/// modifying a file outside the step than its maximum. The share of checks
/// passing is reported and never decides: the runs passing already judge
/// the checks, a run at a time. `pass_label` names the share in the
/// message (`update pass rate` for `plan.update`, as it always has).
pub fn quality(
    step: &str,
    pass_label: &str,
    base: &CheckStepScore,
    cand: &CheckStepScore,
) -> Vec<String> {
    let mut out = Vec::new();
    if let (Some(c), Some(range)) = (cand.pass_rate, base.pass_rate)
        && c.median < range.min
    {
        out.push(format!(
            "{step}: {pass_label} {:.2} is under the baseline's minimum {:.2}",
            c.median, range.min
        ));
    }
    if let (Some(c), Some(range)) = (cand.strayed, base.strayed)
        && c.median > range.max
    {
        out.push(format!(
            "{step}: {} run(s) modifying a file outside the step exceed the baseline's maximum {}",
            number(c.median),
            number(range.max)
        ));
    }
    out
}

/// The measures on which the candidate's median does better than the
/// baseline's best repetition.
pub fn improvements(
    step: &str,
    pass_label: &str,
    base: &CheckStepScore,
    cand: &CheckStepScore,
) -> Vec<String> {
    let mut out = Vec::new();
    if let (Some(v), Some(r)) = (cand.pass_rate, base.pass_rate)
        && v.median > r.max
    {
        out.push(format!(
            "{step}: {pass_label}, from {:.2} at best to {:.2}",
            r.max, v.median
        ));
    }
    if let (Some(v), Some(r)) = (cand.strayed, base.strayed)
        && v.median < r.min
    {
        out.push(format!(
            "{step}: runs modifying a file outside the step, from {} at best to {}",
            number(r.min),
            number(v.median)
        ));
    }
    out
}

/// Each case's share of runs that pass, by case id.
pub fn case_pass_shares(
    cases: &[CaseScore],
    checks: impl Fn(&CaseScore) -> Option<&CheckScore>,
) -> BTreeMap<String, (u32, u32)> {
    let mut shares: BTreeMap<String, (u32, u32)> = BTreeMap::new();
    for c in cases {
        if let Some(s) = checks(c) {
            let entry = shares.entry(c.id.clone()).or_default();
            entry.0 += u32::from(s.passed);
            entry.1 += 1;
        }
    }
    shares
}

fn share((passed, runs): (u32, u32)) -> f64 {
    if runs == 0 {
        0.0
    } else {
        f64::from(passed) / f64::from(runs)
    }
}

/// The per-case verdict of a step: a case whose share of candidate runs
/// passing falls under its share in the baseline, both scored with the
/// case as it now stands. A step holds a handful of cases, so one case
/// falling from every run to a third moves the step's share by less than
/// the spread of its repetitions; this catches it. A case whose
/// expectation changed fails in the rescored baseline, and so decides
/// nothing.
pub fn case_quality(
    step: &str,
    baseline: &BTreeMap<String, (u32, u32)>,
    candidate: &BTreeMap<String, (u32, u32)>,
) -> Vec<String> {
    let mut out = Vec::new();
    for (id, cand) in candidate {
        let Some(base) = baseline.get(id) else {
            continue;
        };
        let (b, c) = (share(*base), share(*cand));
        if c < b {
            out.push(format!(
                "{step}: case `{id}` passes in {c:.2} of its runs, under its baseline share {b:.2}"
            ));
        }
    }
    out
}

/// The cases whose candidate runs do not all pass.
pub fn not_all_passing(candidate: &BTreeMap<String, (u32, u32)>) -> Vec<String> {
    candidate
        .iter()
        .filter(|(_, (passed, runs))| passed < runs)
        .map(|(id, (passed, runs))| format!("`{id}` ({passed} of {runs})"))
        .collect()
}

fn ratio_range(s: Option<Spread>) -> String {
    match s {
        None => "—".into(),
        Some(s) if s.min == s.max => format!("{:.2}", s.median),
        Some(s) => format!("{:.2} ({:.2}–{:.2})", s.median, s.min, s.max),
    }
}

fn count_range(s: Option<Spread>) -> String {
    match s {
        None => "—".into(),
        Some(s) if s.min == s.max => number(s.median),
        Some(s) => format!("{} ({}–{})", number(s.median), number(s.min), number(s.max)),
    }
}

/// The table of a step in the comparison: each measure, the baseline's
/// median and range against the candidate's median.
pub fn comparison_table(out: &mut String, b: &CheckStepScore, c: &CheckStepScore) {
    let _ = writeln!(
        out,
        "| Measure | Baseline (min–max) | Candidate |\n|---|---|---|"
    );
    let show = |s: Option<Spread>, is_ratio: bool| {
        s.map_or_else(
            || "—".to_string(),
            |s| {
                if is_ratio {
                    format!("{:.2}", s.median)
                } else {
                    number(s.median)
                }
            },
        )
    };
    for (label, bs, cs, is_ratio) in [
        ("Runs passing", b.pass_rate, c.pass_rate, true),
        ("Checks passing", b.check_rate, c.check_rate, true),
        (
            "Runs modifying a file outside the step",
            b.strayed,
            c.strayed,
            false,
        ),
    ] {
        let base = if is_ratio {
            ratio_range(bs)
        } else {
            count_range(bs)
        };
        let _ = writeln!(out, "| {label} | {base} | {} |", show(cs, is_ratio));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loose_text_ignores_case_and_spaces() {
        assert!(holds(
            "The  sessions live\nin Redis.",
            "sessions LIVE in redis"
        ));
        assert!(!holds("Postgres", "Redis"));
    }

    #[test]
    fn a_case_regressing_inside_the_range_is_named() {
        let base = BTreeMap::from([("a".to_string(), (5, 5)), ("b".to_string(), (0, 5))]);
        let cand = BTreeMap::from([("a".to_string(), (1, 3)), ("b".to_string(), (3, 3))]);
        assert_eq!(
            case_quality("change.archive", &base, &cand),
            ["change.archive: case `a` passes in 0.33 of its runs, under its baseline share 1.00"]
        );
        assert_eq!(not_all_passing(&cand), ["`a` (1 of 3)"]);
    }
}
