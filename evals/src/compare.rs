//! Comparing a candidate run with a baseline: a verdict on quality, held
//! within the range the baseline's repetitions spanned, and on consumption,
//! which must fall below the baseline's floor. Pure: both runs come scored
//! on the same cases, and the report is built from them alone.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

use crate::observed::RunInfo;
use crate::score::{Measure, Measures, Outcome, RoleScore, Spread, SuiteScore, number};

/// The verdict and the report of a comparison.
#[derive(Debug, Clone, PartialEq)]
pub struct Comparison {
    pub pass: bool,
    /// Every reason the verdict fails.
    pub failures: Vec<String>,
    pub report: String,
}

/// The models a run used, from its observations; empty when none recorded
/// them (observations of format 1).
pub fn models_used(score: &SuiteScore) -> BTreeSet<String> {
    score
        .cases
        .iter()
        .filter_map(|c| c.consumption.as_ref())
        .flat_map(|c| c.models.iter().cloned())
        .collect()
}

/// Why two runs cannot be compared at a constant model, if they cannot:
/// the models asked differ, or the models their observations record as used
/// differ. Runs that recorded none compare by the model asked alone.
pub fn models_differ(
    baseline: &RunInfo,
    baseline_used: &BTreeSet<String>,
    candidate: &RunInfo,
    candidate_used: &BTreeSet<String>,
) -> Option<String> {
    let asked = |m: &Option<String>| m.clone().unwrap_or_else(|| "Claude Code's default".into());
    if baseline.model != candidate.model {
        return Some(format!(
            "the baseline was recorded with the model {} and the candidate with {}; \
             compare runs of the same model",
            asked(&baseline.model),
            asked(&candidate.model)
        ));
    }
    if !baseline_used.is_empty() && !candidate_used.is_empty() && baseline_used != candidate_used {
        let list = |s: &BTreeSet<String>| s.iter().cloned().collect::<Vec<_>>().join(", ");
        return Some(format!(
            "the baseline used the models {} and the candidate {}; compare runs of the same model",
            list(baseline_used),
            list(candidate_used)
        ));
    }
    None
}

/// The baseline's floor: the sum, over its cases, of each case's lowest
/// cost over its repetitions not in error. An unchanged run cannot fall
/// under it by chance: each of its case medians is at least that minimum.
fn floor(score: &SuiteScore) -> f64 {
    let mut lowest: BTreeMap<&str, f64> = BTreeMap::new();
    for c in &score.cases {
        if matches!(c.outcome, Outcome::Error(_)) {
            continue;
        }
        if let Some(cost) = c.cost_usd {
            lowest
                .entry(c.id.as_str())
                .and_modify(|v| *v = v.min(cost))
                .or_insert(cost);
        }
    }
    (lowest.values().sum::<f64>() * 10000.0).round() / 10000.0
}

fn ratio(value: Option<f64>) -> String {
    value.map_or_else(|| "—".into(), |v| format!("{v:.2}"))
}

/// `+12.3 %`, or `—` without both values or with a zero baseline.
fn change(baseline: Option<f64>, candidate: Option<f64>) -> String {
    match (baseline, candidate) {
        (Some(b), Some(c)) if b != 0.0 => format!("{:+.1} %", (c - b) / b * 100.0),
        _ => "—".into(),
    }
}

/// The quality failures of a role: the candidate's median outside the
/// range of the baseline's repetitions.
fn quality(base: &RoleScore, cand: &RoleScore) -> Vec<String> {
    let mut out = Vec::new();
    if let (Some(c), Some(range)) = (cand.recall, base.ranges.recall)
        && c < range.min
    {
        out.push(format!(
            "{}: recall {c:.2} is under the baseline's minimum {:.2}",
            base.role, range.min
        ));
    }
    if let Some(range) = base.ranges.false_positives
        && cand.false_positives > range.max
    {
        out.push(format!(
            "{}: {} false positive(s) exceed the baseline's maximum {}",
            base.role,
            number(cand.false_positives),
            number(range.max)
        ));
    }
    if let (Some(c), Some(range)) = (cand.evidence_validity, base.ranges.evidence_validity)
        && c < range.min
    {
        out.push(format!(
            "{}: evidence validity {c:.2} is under the baseline's minimum {:.2}",
            base.role, range.min
        ));
    }
    out
}

/// What decides the verdict.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// Quality and cost: a change meant to consume less.
    QualityAndCost,
    /// Quality alone, the cost still reported: a change to a role.
    QualityOnly,
}

/// What the comparison found, which the report shows.
struct Verdict<'a> {
    mode: Mode,
    /// The baseline's floor: each compared case at its cheapest run.
    floor: f64,
    /// Why the consumption does not pass, whether or not it decides.
    consumption_failures: &'a [String],
    /// Every reason the verdict fails.
    failures: &'a [String],
}

/// Compares two runs already scored on the same cases. `not_compared`
/// lists the cases only one of them holds.
pub fn compare(
    baseline: &SuiteScore,
    candidate: &SuiteScore,
    not_compared: &[String],
    mode: Mode,
) -> Comparison {
    let mut failures = errors(baseline, candidate);
    let candidate_roles = roles(candidate);
    for base in &baseline.roles {
        if let Some(cand) = candidate_roles.get(base.role.as_str()) {
            failures.extend(quality(base, cand));
        }
    }
    let base_floor = floor(baseline);
    let consumption_failures = consumption(baseline, candidate, base_floor);
    if mode == Mode::QualityAndCost {
        failures.extend(consumption_failures.iter().cloned());
    }
    let verdict = Verdict {
        mode,
        floor: base_floor,
        consumption_failures: &consumption_failures,
        failures: &failures,
    };
    let report = [
        header(baseline, candidate, &verdict),
        quality_section(baseline, candidate),
        consumption_section(baseline, candidate, &verdict),
        per_case_section(baseline, candidate),
        not_compared_section(not_compared),
        failures_section(&verdict),
    ]
    .concat();
    Comparison {
        pass: failures.is_empty(),
        failures,
        report,
    }
}

/// The runs in error, in either run: a comparison cannot conclude on them.
fn errors(baseline: &SuiteScore, candidate: &SuiteScore) -> Vec<String> {
    let mut out = Vec::new();
    for (name, score) in [("baseline", baseline), ("candidate", candidate)] {
        for c in &score.cases {
            if let Outcome::Error(reason) = &c.outcome {
                out.push(format!(
                    "{name}: case `{}` (repetition {}) stopped: {reason}",
                    c.id, c.repetition
                ));
            }
        }
    }
    out
}

fn roles(score: &SuiteScore) -> BTreeMap<&str, &RoleScore> {
    score.roles.iter().map(|r| (r.role.as_str(), r)).collect()
}

/// Why the candidate's consumption does not pass: a case without a cost,
/// or a total cost not under the baseline's floor.
fn consumption(baseline: &SuiteScore, candidate: &SuiteScore, base_floor: f64) -> Vec<String> {
    let without_cost: Vec<String> = [("baseline", baseline), ("candidate", candidate)]
        .iter()
        .flat_map(|(name, score)| {
            score
                .case_consumption
                .iter()
                .filter(|c| c.measures.cost_usd.is_none())
                .map(move |c| format!("case `{}` has no cost in the {name}", c.id))
        })
        .collect();
    if !without_cost.is_empty() {
        return vec![format!(
            "the consumption cannot be compared: {}",
            without_cost.join("; ")
        )];
    }
    if candidate.cost_usd >= base_floor {
        return vec![format!(
            "cost {:.4} USD is not lower than the baseline's floor, {:.4} USD (each case at its \
             cheapest run; the baseline's median is {:.4} USD)",
            candidate.cost_usd, base_floor, baseline.cost_usd
        )];
    }
    Vec::new()
}

/// A ratio's median with the range of the baseline's runs.
fn ratio_range(s: Option<Spread>) -> String {
    s.map_or_else(
        || "—".into(),
        |s| format!("{:.2} ({:.2}–{:.2})", s.median, s.min, s.max),
    )
}

/// A count's median with the range of the baseline's runs.
fn count_range(s: Option<Spread>) -> String {
    s.map_or_else(
        || "—".into(),
        |s| format!("{} ({}–{})", number(s.median), number(s.min), number(s.max)),
    )
}

fn header(baseline: &SuiteScore, candidate: &SuiteScore, verdict: &Verdict) -> String {
    let mut out = String::new();
    let version = |r: &RunInfo| r.claude_version.clone().unwrap_or_else(|| "unknown".into());
    let _ = writeln!(out, "# Comparison with the baseline\n");
    let _ = writeln!(
        out,
        "- Verdict: **{}**{}",
        if verdict.failures.is_empty() {
            "pass"
        } else {
            "fail"
        },
        match verdict.mode {
            Mode::QualityOnly => " (quality only: the consumption is reported, it does not decide)",
            Mode::QualityAndCost => "",
        }
    );
    let _ = writeln!(
        out,
        "- Model: {}",
        baseline
            .run
            .model
            .as_deref()
            .unwrap_or("Claude Code's default")
    );
    let (bv, cv) = (version(&baseline.run), version(&candidate.run));
    let _ = writeln!(
        out,
        "- Claude Code: baseline {bv}, candidate {cv}{}",
        if bv != cv { " — **they differ**" } else { "" }
    );
    let jobs = |r: &RunInfo| {
        r.jobs
            .map_or_else(|| "unknown".to_string(), |j| j.to_string())
    };
    let (bj, cj) = (jobs(&baseline.run), jobs(&candidate.run));
    let _ = writeln!(
        out,
        "- Runs at the same time: baseline {bj}, candidate {cj}{}",
        if bj != cj {
            " — **they differ**: the runs did not share the prompt cache alike"
        } else {
            ""
        }
    );
    let _ = writeln!(
        out,
        "- Cases compared: {}; repetitions: baseline {}, candidate {}\n",
        candidate.case_consumption.len(),
        baseline.repetitions,
        candidate.repetitions
    );
    out
}

fn quality_section(baseline: &SuiteScore, candidate: &SuiteScore) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "## Quality\n");
    let _ = writeln!(
        out,
        "The candidate's median must stay within the range of the baseline's repetitions.\n"
    );
    let _ = writeln!(
        out,
        "| Role | Recall: baseline (min–max) | candidate | False positives: baseline (min–max) | candidate | Evidence validity: baseline (min–max) | candidate |"
    );
    let _ = writeln!(out, "|---|---|---|---|---|---|---|");
    let candidate_roles = roles(candidate);
    for b in &baseline.roles {
        let Some(c) = candidate_roles.get(b.role.as_str()) else {
            continue;
        };
        let _ = writeln!(
            out,
            "| {} | {} | {} | {} | {} | {} | {} |",
            b.role,
            ratio_range(b.ranges.recall),
            ratio(c.recall),
            count_range(b.ranges.false_positives),
            number(c.false_positives),
            ratio_range(b.ranges.evidence_validity),
            ratio(c.evidence_validity),
        );
    }
    out
}

fn consumption_section(baseline: &SuiteScore, candidate: &SuiteScore, verdict: &Verdict) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "\n## Consumption\n");
    let _ = writeln!(
        out,
        "Sums, over the compared cases, of each case's median over its repetitions. The \
         candidate's cost must be lower than the baseline's floor, {:.4} USD: each case at its \
         cheapest run. The duration is reported and never decides.\n",
        verdict.floor
    );
    let _ = writeln!(out, "| Measure | Baseline | Candidate | Change |");
    let _ = writeln!(out, "|---|---:|---:|---:|");
    for m in Measure::ALL {
        let (b, c) = (baseline.consumption.get(m), candidate.consumption.get(m));
        let _ = writeln!(
            out,
            "| {} | {} | {} | {} |",
            m.label(),
            m.show(b, 4),
            m.show(c, 4),
            change(b, c)
        );
    }
    out
}

fn per_case_section(baseline: &SuiteScore, candidate: &SuiteScore) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "\n## Per case\n");
    let _ = writeln!(
        out,
        "| Case | Cost: baseline | candidate | Turns: baseline | candidate | Tokens read from the cache: baseline | candidate |"
    );
    let _ = writeln!(out, "|---|---:|---:|---:|---:|---:|---:|");
    let base_cases: BTreeMap<&str, &Measures> = baseline
        .case_consumption
        .iter()
        .map(|c| (c.id.as_str(), &c.measures))
        .collect();
    let empty = Measures::default();
    for c in &candidate.case_consumption {
        let b = base_cases.get(c.id.as_str()).copied().unwrap_or(&empty);
        let m = &c.measures;
        let cell = |measure: Measure, of: &Measures| measure.show(of.get(measure), 4);
        let _ = writeln!(
            out,
            "| {} | {} | {} | {} | {} | {} | {} |",
            c.id,
            cell(Measure::Cost, b),
            cell(Measure::Cost, m),
            cell(Measure::Turns, b),
            cell(Measure::Turns, m),
            cell(Measure::CacheReads, b),
            cell(Measure::CacheReads, m),
        );
    }
    out
}

fn not_compared_section(not_compared: &[String]) -> String {
    if not_compared.is_empty() {
        return String::new();
    }
    let cases: Vec<String> = not_compared.iter().map(|c| format!("`{c}`")).collect();
    format!(
        "\n## Not compared\n\nCases only one run holds: {}.\n",
        cases.join(", ")
    )
}

fn failures_section(verdict: &Verdict) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "\n## Failures\n");
    if verdict.failures.is_empty() {
        let _ = writeln!(out, "None.");
    }
    for f in verdict.failures {
        let _ = writeln!(out, "- {f}");
    }
    if verdict.mode == Mode::QualityOnly {
        for f in verdict.consumption_failures {
            let _ = writeln!(out, "- (not deciding, `--quality-only`) {f}");
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::case::Kind;
    use crate::observed::{Consumption, Tokens};
    use crate::score::tests::{a_case, observed, run};
    use crate::score::{CaseScore, Thresholds, case, suite};
    use serde_json::json;

    /// A run of `security` on two cases: per repetition, whether the seeded
    /// defect is found, and the cost of each case.
    fn runs(reps: &[(bool, f64, f64)]) -> SuiteScore {
        let seeded = a_case(
            "security",
            Kind::Seeded,
            &[("sqli", &["src/users.ts:12-14"])],
        );
        let mut control = a_case("security", Kind::Control, &[]);
        control.id = "security-control".into();
        let mut scores: Vec<CaseScore> = Vec::new();
        for (i, (found, seeded_cost, control_cost)) in reps.iter().enumerate() {
            let findings: &[(&str, &str, &str, &str)] = if *found {
                &[("R-security-1", "security", "src/users.ts:12", "high")]
            } else {
                &[]
            };
            let mut o = observed("findings", findings, json!([]));
            o.cost_usd = Some(*seeded_cost);
            o.consumption = Some(Consumption {
                turns: Some(9),
                models: vec!["claude-opus-5-5".into()],
                tokens: Some(Tokens {
                    cache_read: 300_000,
                    ..Tokens::default()
                }),
                ..Consumption::default()
            });
            scores.push(case(&seeded, &o).at(i as u32 + 1));
            let mut o = observed("no-finding", &[], json!([]));
            o.cost_usd = Some(*control_cost);
            scores.push(case(&control, &o).at(i as u32 + 1));
        }
        suite(run(), scores, Thresholds::default())
    }

    #[test]
    fn a_cheaper_run_of_the_same_quality_passes() {
        let base = runs(&[(true, 1.0, 1.0), (true, 1.1, 0.95), (true, 1.05, 1.0)]);
        let cand = runs(&[(true, 0.8, 0.8), (true, 0.85, 0.8), (true, 0.8, 0.82)]);
        let result = compare(&base, &cand, &[], Mode::QualityAndCost);
        assert!(result.pass, "{:?}", result.failures);
        assert!(
            result
                .report
                .contains("| Cost (USD) | 2.0500 | 1.6000 | -22.0 % |"),
            "{}",
            result.report
        );
    }

    #[test]
    fn a_cheaper_run_that_misses_more_fails() {
        let base = runs(&[(true, 1.0, 1.0), (true, 1.0, 1.0), (true, 1.0, 1.0)]);
        let cand = runs(&[(false, 0.5, 0.5), (false, 0.5, 0.5), (true, 0.5, 0.5)]);
        let result = compare(&base, &cand, &[], Mode::QualityAndCost);
        assert!(!result.pass);
        assert_eq!(
            result.failures,
            ["security: recall 0.00 is under the baseline's minimum 1.00"]
        );
    }

    #[test]
    fn a_gain_within_the_baseline_spread_fails() {
        let base = runs(&[(true, 1.0, 0.9), (true, 1.2, 1.0), (true, 1.1, 1.1)]);
        // Floor 1.0 + 0.9 = 1.9; median 1.1 + 1.0 = 2.1; candidate 1.95.
        let cand = runs(&[(true, 1.0, 0.95), (true, 1.0, 0.95), (true, 1.0, 0.95)]);
        let result = compare(&base, &cand, &[], Mode::QualityAndCost);
        assert!(!result.pass);
        assert!(
            result.failures[0].contains("1.9500 USD") && result.failures[0].contains("1.9000 USD"),
            "{:?}",
            result.failures
        );
    }

    #[test]
    fn the_same_cost_fails_unless_quality_only() {
        let base = runs(&[(true, 1.0, 1.0), (true, 1.0, 1.0)]);
        let result = compare(&base, &base, &[], Mode::QualityAndCost);
        assert!(!result.pass);
        assert!(
            result.failures[0].contains("not lower"),
            "{:?}",
            result.failures
        );
        let lenient = compare(&base, &base, &[], Mode::QualityOnly);
        assert!(lenient.pass, "{:?}", lenient.failures);
        assert!(lenient.report.contains("Cost (USD)"));
        assert!(lenient.report.contains("(not deciding, `--quality-only`)"));
    }

    #[test]
    fn the_same_code_never_passes_on_cost_however_its_runs_spread() {
        // Each case is expensive in a different repetition: every
        // repetition costs 9, yet the medians sum to 0.
        let base = runs(&[(true, 0.0, 9.0), (true, 9.0, 0.0), (true, 0.0, 0.0)]);
        let result = compare(&base, &base, &[], Mode::QualityAndCost);
        assert!(!result.pass, "{}", result.report);
    }

    #[test]
    fn only_the_duration_grew_passes() {
        let base = runs(&[(true, 1.0, 1.0), (true, 1.0, 1.0)]);
        let mut cand = runs(&[(true, 0.5, 0.5), (true, 0.5, 0.5)]);
        for c in &mut cand.cases {
            c.duration_ms = Some(2000);
        }
        let result = compare(&base, &cand, &[], Mode::QualityAndCost);
        assert!(result.pass, "{:?}", result.failures);
    }

    #[test]
    fn a_baseline_without_costs_cannot_compare_consumption() {
        let mut base = runs(&[(true, 1.0, 1.0)]);
        for c in &mut base.case_consumption {
            c.measures.cost_usd = None;
        }
        let cand = runs(&[(true, 0.5, 0.5)]);
        let result = compare(&base, &cand, &[], Mode::QualityAndCost);
        assert!(result.failures[0].contains("cannot be compared"));
        assert!(compare(&base, &cand, &[], Mode::QualityOnly).pass);
    }

    #[test]
    fn a_repetition_in_error_fails_the_verdict() {
        let base = runs(&[(true, 1.0, 1.0), (true, 1.0, 1.0)]);
        let mut cand = runs(&[(true, 0.5, 0.5), (true, 0.5, 0.5)]);
        cand.cases[1].outcome = Outcome::Error("timeout after 600 s".into());
        let result = compare(&base, &cand, &[], Mode::QualityOnly);
        assert!(!result.pass);
        assert!(
            result.failures[0].starts_with("candidate: case `security-case` (repetition 2)"),
            "{:?}",
            result.failures
        );
    }

    #[test]
    fn another_claude_code_version_is_reported() {
        let base = runs(&[(true, 1.0, 1.0)]);
        let mut cand = runs(&[(true, 0.5, 0.5)]);
        cand.run.claude_version = Some("2.1.301 (Claude Code)".into());
        let report = compare(&base, &cand, &[], Mode::QualityAndCost).report;
        assert!(report.contains("2.1.301"), "{report}");
        assert!(report.contains("they differ"), "{report}");
    }

    #[test]
    fn another_job_count_is_reported() {
        let mut base = runs(&[(true, 1.0, 1.0)]);
        base.run.jobs = Some(4);
        let mut cand = runs(&[(true, 0.5, 0.5)]);
        cand.run.jobs = Some(2);
        let report = compare(&base, &cand, &[], Mode::QualityAndCost).report;
        assert!(
            report.contains("baseline 4, candidate 2 — **they differ**"),
            "{report}"
        );
    }

    #[test]
    fn the_cases_not_compared_are_listed() {
        let base = runs(&[(true, 1.0, 1.0)]);
        let cand = runs(&[(true, 0.5, 0.5)]);
        let report = compare(
            &base,
            &cand,
            &["ops-code-no-timeout".into()],
            Mode::QualityAndCost,
        )
        .report;
        assert!(report.contains("`ops-code-no-timeout`"), "{report}");
    }

    #[test]
    fn models_must_be_the_same_asked_and_used() {
        let base = run();
        let mut other = run();
        other.model = Some("claude-sonnet-4-5".into());
        let none = BTreeSet::new();
        let err = models_differ(&base, &none, &other, &none).unwrap();
        assert!(err.contains("Claude Code's default") && err.contains("claude-sonnet-4-5"));
        let opus: BTreeSet<String> = ["claude-opus-5-5".to_string()].into();
        let sonnet: BTreeSet<String> = ["claude-sonnet-5-5".to_string()].into();
        let err = models_differ(&base, &opus, &base, &sonnet).unwrap();
        assert!(err.contains("claude-opus-5-5") && err.contains("claude-sonnet-5-5"));
        assert_eq!(models_differ(&base, &opus, &base, &none), None);
        assert_eq!(models_differ(&base, &opus, &base, &opus), None);
    }
}
