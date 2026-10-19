//! The suite's report, in JSON and markdown. Roles and cases come sorted
//! by id from [`crate::score::suite`], so the same observations give the
//! same bytes.

use std::fmt::Write as _;

use crate::score::{Class, Measure, Measures, Outcome, SourceShare, Spread, SuiteScore, number};

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

fn sources_table(out: &mut String, ranked: &[SourceShare]) {
    let _ = writeln!(out, "| Source | Tokens | Share |\n|---|---:|---:|");
    for r in ranked {
        let _ = writeln!(
            out,
            "| {} | {} | {:.0} % |",
            r.source,
            number(r.tokens),
            r.share * 100.0
        );
    }
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

    if let Some(p) = &score.planning {
        let spread = |s: Option<Spread>, ratio: bool| match s {
            None => "—".to_string(),
            Some(s) if ratio => ratio_range(Some(s.median), Some(s), repeated),
            Some(s) => count_range(s.median, Some(s), repeated),
        };
        let _ = writeln!(
            out,
            "\n## `plan.write`\n\nPer repetition, every planning run pooled; then the median over the repetitions, \
             with its range. No threshold: `codev-evals compare` judges these against the baseline.\n"
        );
        let _ = writeln!(
            out,
            "| Cases | Primary recall | Collateral recall | Probable duplicates | Decisions ignored | Plans that do not validate | Cost (USD) |\n|---|---|---|---|---|---|---|"
        );
        let _ = writeln!(
            out,
            "| {} | {} | {} | {} | {} | {} | {} |",
            p.cases,
            spread(p.primary_recall, true),
            spread(p.collateral_recall, true),
            spread(p.duplicates, false),
            spread(p.decisions_ignored, false),
            spread(p.invalid, false),
            Measure::Cost.show(p.consumption.cost_usd, 2),
        );
    }

    if let Some(u) = &score.update {
        let spread = |s: Option<Spread>, ratio: bool| match s {
            None => "—".to_string(),
            Some(s) if ratio => ratio_range(Some(s.median), Some(s), repeated),
            Some(s) => count_range(s.median, Some(s), repeated),
        };
        let _ = writeln!(
            out,
            "\n## `plan.update`\n\nPer repetition, every update run pooled: the share of runs whose every check \
             passes, the share of checks that pass, the runs that modified a file outside the change's folder; \
             then the median over the repetitions, with its range. No threshold: `codev-evals compare` judges \
             these against the baseline.\n"
        );
        let _ = writeln!(
            out,
            "| Cases | Runs passing | Checks passing | Runs modifying a file outside the step | Cost (USD) |\n|---|---|---|---|---|"
        );
        let _ = writeln!(
            out,
            "| {} | {} | {} | {} | {} |",
            u.cases,
            spread(u.pass_rate, true),
            spread(u.check_rate, true),
            spread(u.strayed, false),
            Measure::Cost.show(u.consumption.cost_usd, 2),
        );
    }

    let _ = writeln!(out, "\n## Consumption\n");
    let _ = writeln!(
        out,
        "Per case, the median over its runs not in error; per role and for the suite, the sum of those medians. Tokens are summed over every model a run used.\n"
    );
    let _ = writeln!(out, "| Role | {} |", consumption_header());
    let _ = writeln!(out, "|---|{}", "---:|".repeat(Measure::ALL.len()));
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
    if !score.sources.is_empty() {
        let _ = writeln!(
            out,
            "\n## Where the tokens go\n\nThe tokens written to the cache, by source: per case the median over its runs, \
             summed over the cases; their share is their share of what those writes cost.\n"
        );
        sources_table(&mut out, &score.sources);
        if score.sources_by_step.len() > 1 {
            for (step, ranked) in &score.sources_by_step {
                let _ = writeln!(out, "\n### `{step}`\n");
                sources_table(&mut out, ranked);
            }
        }
    }
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
                    Outcome::Scored | Outcome::NoRecord | Outcome::NoPlan => {
                        c.outcome.label().to_string()
                    }
                    Outcome::Error(reason)
                    | Outcome::RecordInvalid(reason)
                    | Outcome::RoleNotRun(reason) => format!("{} ({reason})", c.outcome.label()),
                }
            );
            if let (Some(cost), Some(ms)) = (c.cost_usd, c.duration_ms) {
                let _ = writeln!(out, "- Cost: {cost:.2} USD, {} s", ms / 1000);
            }
            if let Some(tools) = c.consumption.as_ref().and_then(|m| m.tool_calls) {
                let orientation = c
                    .consumption
                    .as_ref()
                    .and_then(|m| m.orientation_calls)
                    .map_or_else(
                        || ", no artifact written".to_string(),
                        |n| format!(", {n} before the first artifact"),
                    );
                let _ = writeln!(out, "- Tool calls: {tools}{orientation}");
            }
            if let Some(p) = &c.plan {
                for id in &p.primary_found {
                    let _ = writeln!(out, "- Requirement `{id}`: modified");
                }
                for id in &p.primary_missed {
                    let _ = writeln!(out, "- Requirement `{id}`: **missed**");
                }
                for id in &p.collateral_found {
                    let _ = writeln!(out, "- Collateral `{id}`: modified");
                }
                for id in &p.collateral_missed {
                    let _ = writeln!(out, "- Collateral `{id}`: missed");
                }
                for id in &p.duplicates {
                    let _ = writeln!(out, "- Added `{id}`: **probable duplicate**");
                }
                for id in &p.decisions_ignored {
                    let _ = writeln!(out, "- Decision {id}: **ignored**");
                }
                if p.outcome == crate::plan::PlanOutcome::Planned && !p.valid {
                    let _ = writeln!(out, "- The plan does not validate");
                }
            }
            if let Some(u) = &c.update {
                let _ = writeln!(
                    out,
                    "- Checks: {} of {} passed, the run {}",
                    u.checks_passed(),
                    u.checks.len(),
                    if u.passed { "passes" } else { "**fails**" }
                );
                for check in &u.checks {
                    let _ = writeln!(
                        out,
                        "- Check {}: {}",
                        check.check,
                        if check.passed { "passed" } else { "**failed**" }
                    );
                }
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

    #[test]
    fn the_report_gives_the_planning_section() {
        use crate::case::{Kind, PlanCase, RequirementId, Step};
        use crate::plan::AtRef;
        use crate::score::tests::{a_case, observed, run};
        use crate::score::{Thresholds, case, plan_case, suite};
        let mut plan = a_case("planner", Kind::Seeded, &[]);
        plan.id = "plan-a".into();
        plan.step = Step::PlanWrite;
        plan.plan = Some(PlanCase {
            git_ref: "abc".into(),
            request: "r".into(),
            modified: vec![RequirementId::parse("d::One").unwrap()],
            collateral: vec![],
            decisions: vec![],
        });
        let mut o = observed("findings", &[], serde_json::json!([]));
        o.review = None;
        o.files.insert(
            "_codev/changes/c/proposal.md".into(),
            Some("# Proposal\n".into()),
        );
        o.consumption = Some(crate::observed::Consumption {
            tool_calls: Some(27),
            orientation_calls: Some(11),
            ..Default::default()
        });
        let review = a_case(
            "security",
            Kind::Seeded,
            &[("sqli", &["src/users.ts:12-14"])],
        );
        let s = suite(
            run(),
            vec![
                case(&review, &observed("no-finding", &[], serde_json::json!([]))),
                plan_case(&plan, &o, &AtRef::default()),
            ],
            Thresholds::default(),
        );
        let md = markdown(&s);
        assert!(md.contains("## `plan.write`"), "{md}");
        assert!(md.contains("| 1 | 0.00 | — | 0 | 0 | 1 |"), "{md}");
        assert!(md.contains("- Requirement `d::One`: **missed**"), "{md}");
        assert!(
            md.contains("- Tool calls: 27, 11 before the first artifact"),
            "{md}"
        );
        assert!(md.contains("- The plan does not validate"), "{md}");
        let json: serde_json::Value = serde_json::from_str(&json(&s)).unwrap();
        assert_eq!(json["planning"]["cases"], 1);
        assert!(!markdown(&scores()).contains("plan.write"));
    }

    fn update_suite() -> SuiteScore {
        use crate::case::{Artifact, Step, UpdateCase};
        use crate::score::update_case;
        use std::collections::BTreeMap;
        let mut c = a_case("planner", Kind::Seeded, &[]);
        c.id = "update-proposal-ripple".into();
        c.step = Step::PlanUpdate;
        c.change = "c".into();
        c.update = Some(UpdateCase {
            git_ref: "abc".into(),
            artifact: Artifact::Proposal,
            request: "r".into(),
            revised: vec!["proposal.md".into()],
            kept: vec![],
            ripple: vec!["specs/doctor/spec.md".into()],
            contains: vec![],
            refuse: false,
            files: BTreeMap::from([
                ("proposal.md".to_string(), "# P\n".to_string()),
                ("specs/doctor/spec.md".to_string(), "# D\n".to_string()),
            ]),
        });
        let mut o = observed("findings", &[], json!([]));
        o.review = None;
        o.files = BTreeMap::from([
            (
                "_codev/changes/c/proposal.md".to_string(),
                Some("# P, revised\n".to_string()),
            ),
            (
                "_codev/changes/c/specs/doctor/spec.md".to_string(),
                Some("# D\n".to_string()),
            ),
        ]);
        o.validate
            .insert("c".into(), json!({"items": [{"findings": []}]}));
        o.validate_before = Some(json!({"items": [{"findings": []}]}));
        o.strayed = vec!["src/lib.rs".into()];
        let review = a_case(
            "security",
            Kind::Seeded,
            &[("sqli", &["src/users.ts:12-14"])],
        );
        suite(
            run(),
            vec![
                case(&review, &observed("no-finding", &[], json!([]))),
                update_case(&c, &o),
            ],
            Thresholds::default(),
        )
    }

    #[test]
    fn the_report_has_an_update_section() {
        let s = update_suite();
        let md = markdown(&s);
        assert!(md.contains("## `plan.update`"), "{md}");
        assert!(md.contains("| 1 | 0.00 | 0.75 | 1 | 0.50 |"), "{md}");
        assert!(
            md.contains("- Checks: 3 of 4 passed, the run **fails**"),
            "{md}"
        );
        assert!(md.contains("- Check `proposal.md` revised: passed"), "{md}");
        assert!(
            md.contains("- Check ripple `specs/doctor/spec.md` named: **failed**"),
            "{md}"
        );
        assert!(
            md.contains("- Modified outside the step: `src/lib.rs`"),
            "{md}"
        );
        let json: serde_json::Value = serde_json::from_str(&json(&s)).unwrap();
        assert_eq!(json["update"]["cases"], 1);
        assert_eq!(json["update"]["passRate"]["median"], 0.0);
        let run = json["cases"]
            .as_array()
            .unwrap()
            .iter()
            .find(|c| c["id"] == "update-proposal-ripple")
            .unwrap();
        assert_eq!(run["update"]["passed"], false);
        assert_eq!(run["update"]["checks"][0]["check"], "`proposal.md` revised");
        assert_eq!(run["update"]["checks"][0]["passed"], true);
    }

    #[test]
    fn a_report_without_update_cases_has_no_section() {
        let s = scores();
        assert!(!markdown(&s).contains("plan.update"));
        let json: serde_json::Value = serde_json::from_str(&json(&s)).unwrap();
        assert!(json.get("update").is_none());
        assert!(json["cases"][0].get("update").is_none());
    }
}
