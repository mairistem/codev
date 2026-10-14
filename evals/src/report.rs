//! The suite's report, in JSON and markdown. Roles and cases come sorted
//! by id from [`crate::score::suite`], so the same observations give the
//! same bytes.

use std::fmt::Write as _;

use crate::score::{Class, Measure, Measures, Outcome, Spread, SuiteScore, number};

pub fn json(score: &SuiteScore) -> String {
    let mut text = serde_json::to_string_pretty(score).expect("the report serializes");
    text.push('\n');
    text
}

fn ratio(value: Option<f64>) -> String {
    value.map_or_else(|| "—".to_string(), |v| format!("{v:.2}"))
}

/// A ratio's median, with its range when the runs were repeated.
fn ratio_range(value: Option<f64>, range: Option<Spread>, repeated: bool) -> String {
    match (value, range) {
        (Some(v), Some(r)) if repeated && r.min != r.max => {
            format!("{v:.2} ({:.2}–{:.2})", r.min, r.max)
        }
        _ => ratio(value),
    }
}

/// A count's median, with its range when the runs were repeated.
fn count_range(value: f64, range: Option<Spread>, repeated: bool) -> String {
    match range {
        Some(r) if repeated && r.min != r.max => {
            format!("{} ({}–{})", number(value), number(r.min), number(r.max))
        }
        _ => number(value),
    }
}

/// The consumption columns, in [`Measure::ALL`]'s order.
fn consumption_cells(m: &Measures) -> String {
    Measure::ALL
        .iter()
        .map(|measure| measure.show(m.get(*measure), 2))
        .collect::<Vec<_>>()
        .join(" | ")
}

fn consumption_header() -> String {
    Measure::ALL
        .iter()
        .map(|m| m.column())
        .collect::<Vec<_>>()
        .join(" | ")
}

pub fn markdown(score: &SuiteScore) -> String {
    let mut out = String::new();
    let run = &score.run;
    let repeated = score.repetitions > 1;
    let distinct = score.case_consumption.len();
    let _ = writeln!(out, "# Role evaluation report\n");
    let _ = writeln!(
        out,
        "- Date: {}\n- Model: {}\n- Claude Code: {}\n- codev: {}\n- Cases: {}\n- Repetitions: {}\n- Cost: {:.2} USD for one run of the suite (sum of the cases' medians), {:.2} USD in all",
        run.date,
        run.model.as_deref().unwrap_or("Claude Code's default"),
        run.claude_version.as_deref().unwrap_or("unknown"),
        run.codev_version.as_deref().unwrap_or("unknown"),
        distinct,
        score.repetitions,
        score.cost_usd,
        score.total_cost_usd,
    );
    let t = &score.thresholds;
    let _ = writeln!(
        out,
        "- Thresholds: recall ≥ {:.2}, false positives ≤ {}, evidence validity ≥ {:.2}",
        t.min_recall, t.max_false_positives, t.min_evidence
    );
    let _ = writeln!(
        out,
        "- Verdict: **{}**\n",
        if score.pass { "pass" } else { "fail" }
    );
    if repeated {
        let _ = writeln!(
            out,
            "{} runs per case: each measure is the median over the runs, its range in parentheses; compared with a baseline on the same model, not a proof.\n",
            score.repetitions
        );
    } else {
        let _ = writeln!(
            out,
            "One run per case: a score is a noisy point, compared with a baseline on the same model, not a proof.\n"
        );
    }

    let f = &score.figures;
    let _ = writeln!(out, "## The suite\n");
    let spread_ratio = |s: Option<Spread>| ratio_range(s.map(|s| s.median), s, repeated);
    let spread_count = |s: Option<Spread>| {
        s.map_or_else(
            || "—".to_string(),
            |s| count_range(s.median, Some(s), repeated),
        )
    };
    let _ = writeln!(
        out,
        "- Recall: {} ({} of {} seeded defects found)\n- Evidence validity: {} ({} of {} checked quotes)\n- False positives: {}, {} per case\n",
        spread_ratio(f.recall),
        spread_count(f.true_positives),
        number(f.expected),
        spread_ratio(f.evidence_validity),
        number(f.valid),
        number(f.checked),
        spread_count(f.false_positives),
        f.false_positives_per_case.map_or_else(
            || "—".to_string(),
            |s| ratio_range(Some(s.median), Some(s), repeated)
        ),
    );

    let _ = writeln!(out, "## Per role\n");
    let _ = writeln!(
        out,
        "| Role | Cases (seeded/control) | Expected | True positives | Misses | False positives | Unreported | Duplicates | Cross-role | Recall | Evidence validity (found/moved/invalid/unchecked) | Severity agreement | Cost (USD) |"
    );
    let _ = writeln!(out, "|---|---|---|---|---|---|---|---|---|---|---|---|---|");
    for r in &score.roles {
        let e = &r.evidence;
        let _ = writeln!(
            out,
            "| {} | {} ({}/{}) | {} | {} | {} | {} | {} | {} | {} | {} | {} ({}/{}/{}/{}) | {}/{} | {:.2} |",
            r.role,
            r.cases,
            r.seeded,
            r.controls,
            number(r.expected),
            number(r.true_positives),
            number(r.misses),
            count_range(r.false_positives, r.ranges.false_positives, repeated),
            number(r.unreported),
            number(r.duplicates),
            number(r.cross_role),
            ratio_range(r.recall, r.ranges.recall, repeated),
            ratio_range(r.evidence_validity, r.ranges.evidence_validity, repeated),
            number(e.found),
            number(e.moved),
            number(e.invalid),
            number(e.unchecked),
            number(r.severity_agreement),
            number(r.true_positives),
            r.cost_usd,
        );
    }

    let _ = writeln!(out, "\n## Consumption\n");
    let _ = writeln!(
        out,
        "Per case, the median over its runs not in error; per role and for the suite, the sum of those medians. Tokens are summed over every model a run used.\n"
    );
    let _ = writeln!(out, "| Role | {} |", consumption_header());
    let _ = writeln!(out, "|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|");
    for r in &score.roles {
        let _ = writeln!(
            out,
            "| {} | {} |",
            r.role,
            consumption_cells(&r.consumption)
        );
    }
    let _ = writeln!(
        out,
        "| **Suite** | {} |",
        consumption_cells(&score.consumption)
    );
    let _ = writeln!(out, "\n## Failures\n");
    if score.failures.is_empty() {
        let _ = writeln!(out, "None.");
    }
    for f in &score.failures {
        let _ = writeln!(out, "- {f}");
    }

    let _ = writeln!(out, "\n## Per case\n");
    // The runs come sorted by case, then repetition: one group per case.
    for runs in score.cases.chunk_by(|a, b| a.id == b.id) {
        let first = &runs[0];
        let _ = writeln!(
            out,
            "### {}\n\n- Role: {} at `{}`, {}",
            first.id, first.role, first.step, first.kind
        );
        if let Some(m) = score.case_consumption.iter().find(|m| m.id == first.id) {
            let _ = writeln!(
                out,
                "- Consumption{}: {}\n  {}",
                if runs.len() > 1 { " (median)" } else { "" },
                consumption_header(),
                consumption_cells(&m.measures).replace(" | ", " · ")
            );
        }
        out.push('\n');
        for c in runs {
            if runs.len() > 1 {
                let _ = writeln!(out, "#### Run {}\n", c.repetition);
            }
            let _ = writeln!(
                out,
                "- Outcome: {}",
                match &c.outcome {
                    Outcome::Scored | Outcome::NoRecord => c.outcome.label().to_string(),
                    Outcome::Error(reason)
                    | Outcome::RecordInvalid(reason)
                    | Outcome::RoleNotRun(reason) => format!("{} ({reason})", c.outcome.label()),
                }
            );
            if let (Some(cost), Some(ms)) = (c.cost_usd, c.duration_ms) {
                let _ = writeln!(out, "- Cost: {cost:.2} USD, {} s", ms / 1000);
            }
            for e in &c.expected {
                let _ = match &e.matched_by {
                    Some(by) => writeln!(
                        out,
                        "- Expected `{}` ({}, {}): found by {by}{}",
                        e.id,
                        e.category,
                        if e.severity.is_empty() {
                            "check"
                        } else {
                            &e.severity
                        },
                        if e.severity_agrees || e.severity.is_empty() {
                            ""
                        } else {
                            ", another severity"
                        }
                    ),
                    None => writeln!(
                        out,
                        "- Expected `{}` ({}, {}): **missed**",
                        e.id,
                        e.category,
                        if e.severity.is_empty() {
                            "check"
                        } else {
                            &e.severity
                        },
                    ),
                };
            }
            for f in &c.findings {
                let class = match f.class {
                    Class::TruePositive => "true positive",
                    Class::Duplicate => "duplicate",
                    Class::CrossRole => "cross-role",
                    Class::FalsePositive => "**false positive**",
                    Class::Unreported => "unreported (low confidence)",
                };
                let evidence = match f.evidence {
                    crate::score::Evidence::Found => "",
                    crate::score::Evidence::Moved => ", evidence moved",
                    crate::score::Evidence::Invalid => ", **evidence invalid**",
                    crate::score::Evidence::Unchecked => ", evidence unchecked",
                };
                let near = f
                    .near
                    .as_ref()
                    .map(|id| format!(" (at the location of `{id}`, another category)"))
                    .unwrap_or_default();
                let _ = writeln!(
                    out,
                    "- Finding {} [{} / {}] `{}` {}: {class}{near}{evidence}",
                    f.id, f.severity, f.confidence, f.location, f.title
                );
            }
            for path in &c.changed {
                let _ = writeln!(
                    out,
                    "- Changed although it should not: `{path}` (**false positive**)"
                );
            }
            for path in &c.strayed {
                let _ = writeln!(out, "- Modified outside the step: `{path}`");
            }
            for id in &c.answered {
                let _ = writeln!(
                    out,
                    "- Answered although the evaluation leaves findings open: {id}"
                );
            }
            for e in &c.record_errors {
                let _ = writeln!(out, "- Record error: {e}");
            }
            let _ = writeln!(out);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::case::Kind;
    use crate::score::tests::{a_case, observed, run};
    use crate::score::{Thresholds, case, suite};
    use serde_json::json;

    fn scores() -> SuiteScore {
        let a = a_case(
            "security",
            Kind::Seeded,
            &[("sqli", &["src/users.ts:12-14"])],
        );
        let mut b = a_case("privacy", Kind::Control, &[]);
        b.id = "a-privacy-control".into();
        let oa = observed(
            "findings",
            &[("R-security-1", "security", "src/users.ts:12", "high")],
            json!([]),
        );
        let ob = observed(
            "findings",
            &[("R-privacy-1", "privacy", "src/a.ts:3", "high")],
            json!([]),
        );
        suite(
            run(),
            vec![case(&a, &oa), case(&b, &ob)],
            Thresholds::default(),
        )
    }

    #[test]
    fn the_report_is_sorted_and_identical_twice() {
        let first = scores();
        assert_eq!(first.cases[0].id, "a-privacy-control", "cases sorted by id");
        assert_eq!(first.roles[0].role, "privacy", "roles sorted by id");
        assert_eq!(json(&first), json(&scores()));
        assert_eq!(markdown(&first), markdown(&scores()));
        let md = markdown(&first);
        assert!(
            md.contains("| security | 1 (1/0) | 1 | 1 | 0 | 0 |"),
            "{md}"
        );
        assert!(md.contains("One run per case"), "{md}");
        assert!(md.contains("R-privacy-1"), "{md}");
    }

    #[test]
    fn the_report_shows_the_range_over_repetitions() {
        let a = a_case(
            "security",
            Kind::Seeded,
            &[("sqli", &["src/users.ts:12-14"])],
        );
        let found = observed(
            "findings",
            &[("R-security-1", "security", "src/users.ts:12", "high")],
            json!([]),
        );
        let missed = observed("findings", &[], json!([]));
        let s = suite(
            run(),
            vec![
                case(&a, &found).at(1),
                case(&a, &missed).at(2),
                case(&a, &found).at(3),
            ],
            Thresholds::default(),
        );
        let md = markdown(&s);
        assert!(md.contains("- Repetitions: 3"), "{md}");
        assert!(md.contains("3 runs per case"), "{md}");
        assert!(md.contains("| 1.00 (0.00–1.00) |"), "{md}");
        assert!(md.contains("#### Run 2"), "{md}");
    }

    #[test]
    fn an_absent_measure_everywhere_shows_a_dash() {
        let md = markdown(&scores());
        assert!(
            md.contains("| **Suite** | 1.00 | 2 | — | — | — | — | — | — | — |"),
            "{md}"
        );
    }
}
