//! The suite's report, in JSON and markdown. Roles and cases come sorted
//! by id from [`crate::score::suite`], so the same observations give the
//! same bytes.

use std::fmt::Write as _;

use crate::score::{Class, Outcome, SuiteScore};

pub fn json(score: &SuiteScore) -> String {
    let mut text = serde_json::to_string_pretty(score).expect("the report serializes");
    text.push('\n');
    text
}

fn ratio(value: Option<f64>) -> String {
    value.map_or_else(|| "—".to_string(), |v| format!("{v:.2}"))
}

pub fn markdown(score: &SuiteScore) -> String {
    let mut out = String::new();
    let run = &score.run;
    let _ = writeln!(out, "# Role evaluation report\n");
    let _ = writeln!(
        out,
        "- Date: {}\n- Model: {}\n- Claude Code: {}\n- codev: {}\n- Cases: {}\n- Cost: {:.2} USD",
        run.date,
        run.model.as_deref().unwrap_or("Claude Code's default"),
        run.claude_version.as_deref().unwrap_or("unknown"),
        run.codev_version.as_deref().unwrap_or("unknown"),
        score.cases.len(),
        score.cost_usd,
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
    let _ = writeln!(
        out,
        "One run per case: a score is a noisy point, compared with a baseline on the same model, not a proof.\n"
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
            r.expected,
            r.true_positives,
            r.misses,
            r.false_positives,
            r.unreported,
            r.duplicates,
            r.cross_role,
            ratio(r.recall),
            ratio(r.evidence_validity),
            e.found,
            e.moved,
            e.invalid,
            e.unchecked,
            r.severity_agreement,
            r.true_positives,
            r.cost_usd,
        );
    }

    let _ = writeln!(out, "\n## Failures\n");
    if score.failures.is_empty() {
        let _ = writeln!(out, "None.");
    }
    for f in &score.failures {
        let _ = writeln!(out, "- {f}");
    }

    let _ = writeln!(out, "\n## Per case\n");
    for c in &score.cases {
        let _ = writeln!(
            out,
            "### {}\n\n- Role: {} at `{}`, {}\n- Outcome: {}",
            c.id,
            c.role,
            c.step,
            c.kind,
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
            let _ = writeln!(
                out,
                "- Finding {} [{} / {}] `{}` {}: {class}{evidence}",
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
        assert!(md.contains("R-privacy-1"), "{md}");
    }
}
