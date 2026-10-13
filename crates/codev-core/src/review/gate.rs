//! Completeness of a record and the archive gate, given who was active.
//!
//! "Active" is decided by `codev-core::roles` for the change; the engine
//! passes the result in, so these rules never need the configuration.

use crate::parser::ast::Finding;

use super::codes::{
    REVIEW_FORMAT_UNSUPPORTED, REVIEW_RECORD_MISSING, REVIEW_ROLE_MISSING, REVIEW_STEP_MISSING,
    REVIEW_UNKNOWN_SECTION, REVIEW_UNRESOLVED,
};
use super::{FORMAT, Record, RoleState, STEPS};

/// The roles active at a reviewing step, for a change.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StepRoles {
    pub step: String,
    pub active: Vec<String>,
}

/// Why the record does not let the change be archived.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Blocker {
    pub code: &'static str,
    pub message: String,
}

/// Every role active at a step the record holds has a section that does
/// not say it is disabled; a role the catalog does not know is a warning.
pub fn check_roles(record: &Record, active: &[StepRoles], known: &[String]) -> Vec<Finding> {
    let mut out = Vec::new();
    for step in &record.steps {
        for section in &step.roles {
            if !known.contains(&section.role) {
                out.push(Finding::warning(
                    REVIEW_UNKNOWN_SECTION,
                    section.line,
                    format!(
                        "`### {}` is not a role of this change's catalog; `codev roles --change <name>` lists them",
                        section.role
                    ),
                ));
            }
        }
        let Some(roles) = active.iter().find(|a| a.step == step.step) else {
            continue;
        };
        for role in &roles.active {
            let message = match step.role(role).map(|s| s.state) {
                None => format!(
                    "`{role}` is active at `{}` but has no `### {role}` section; write its findings or `No finding.`",
                    step.step
                ),
                Some(RoleState::Disabled) => format!(
                    "`{role}` is active at `{}`, yet its section says `Disabled by configuration.`; run it and record what it found",
                    step.step
                ),
                Some(_) => continue,
            };
            out.push(Finding::error(REVIEW_ROLE_MISSING, step.line, message));
        }
    }
    out
}

/// What keeps the change from being archived; empty when nothing does.
/// `record` is `None` when `review.md` does not exist.
pub fn gate(record: Option<&Record>, active: &[StepRoles]) -> Vec<Blocker> {
    let reviewed: Vec<&str> = STEPS
        .iter()
        .copied()
        .filter(|step| {
            active
                .iter()
                .any(|a| a.step == *step && !a.active.is_empty())
        })
        .collect();
    let Some(record) = record else {
        if reviewed.is_empty() {
            return Vec::new();
        }
        return vec![Blocker {
            code: REVIEW_RECORD_MISSING,
            message: format!(
                "no review record, while roles were active at {}; the skills write it, or write it by hand",
                reviewed.join(" and ")
            ),
        }];
    };
    if record.format > FORMAT {
        return vec![Blocker {
            code: REVIEW_FORMAT_UNSUPPORTED,
            message: format!(
                "the review record is in format {}, newer than this codev reads ({FORMAT}); update codev",
                record.format
            ),
        }];
    }
    let mut blockers: Vec<Blocker> = reviewed
        .iter()
        .filter(|step| record.step(step).is_none())
        .map(|step| Blocker {
            code: REVIEW_STEP_MISSING,
            message: format!(
                "the review record has no `## {step}` section, while roles were active at it"
            ),
        })
        .collect();
    blockers.extend(record.findings().filter(|f| f.blocks_archive()).map(|f| {
        Blocker {
            code: REVIEW_UNRESOLVED,
            message: format!(
                "{} ({}) is high and still {}; fix it, or a human accepts the risk or defers it",
                f.id,
                f.title,
                f.status().map_or("open", |s| s.as_str())
            ),
        }
    }));
    blockers
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::review::parse;

    fn active(plan: &[&str], code: &[&str]) -> Vec<StepRoles> {
        vec![
            StepRoles {
                step: "plan.challenge".into(),
                active: plan.iter().map(|s| s.to_string()).collect(),
            },
            StepRoles {
                step: "code.review".into(),
                active: code.iter().map(|s| s.to_string()).collect(),
            },
        ]
    }

    fn known() -> Vec<String> {
        ["skeptic", "newcomer", "security"]
            .map(String::from)
            .to_vec()
    }

    const BOTH: &str = "## plan.challenge\n\n### skeptic\n\nNo finding.\n\n## code.review\n\n### newcomer\n\nNo finding.\n";

    fn finding(severity: &str, history: &str) -> String {
        format!(
            "## plan.challenge\n\n### skeptic\n\nNo finding.\n\n## code.review\n\n### newcomer\n\nNo finding.\n\n### security\n\n#### R-security-1 Tokens\n\n```yaml\nseverity: {severity}\nconfidence: high\nlocation: a.rs:1\nevidence: x\nrecommendation: y\nhistory:\n  - status: open\n    by: security\n{history}```\n"
        )
    }

    #[test]
    fn an_active_role_without_a_section_is_missing() {
        let record = parse(BOTH).value;
        let findings = check_roles(
            &record,
            &active(&["skeptic", "security"], &["newcomer"]),
            &known(),
        );
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].code, REVIEW_ROLE_MISSING);
        assert!(findings[0].message.contains("security"));
        assert!(findings[0].message.contains("plan.challenge"));
        assert_eq!(findings[0].line, 1);
    }

    #[test]
    fn a_disabled_section_for_an_active_role_is_missing() {
        let record = parse("## code.review\n\n### newcomer\n\nDisabled by configuration.\n").value;
        let findings = check_roles(&record, &active(&[], &["newcomer"]), &known());
        assert_eq!(findings[0].code, REVIEW_ROLE_MISSING);
        // Inactive, the same section is fine.
        assert!(check_roles(&record, &active(&[], &[]), &known()).is_empty());
    }

    #[test]
    fn a_role_unknown_to_the_catalog_is_a_warning() {
        let record = parse("## code.review\n\n### reviewer\n\nNo finding.\n").value;
        let findings = check_roles(&record, &active(&[], &[]), &known());
        assert_eq!(findings[0].code, REVIEW_UNKNOWN_SECTION);
    }

    #[test]
    fn gate_requires_a_record_and_a_step_only_where_roles_are_active() {
        let blockers = gate(None, &active(&["skeptic"], &["newcomer"]));
        assert_eq!(blockers.len(), 1);
        assert_eq!(blockers[0].code, REVIEW_RECORD_MISSING);
        assert!(gate(None, &active(&[], &[])).is_empty());

        let plan_only = parse("## plan.challenge\n\n### skeptic\n\nNo finding.\n").value;
        let blockers = gate(Some(&plan_only), &active(&["skeptic"], &["newcomer"]));
        assert_eq!(blockers.len(), 1);
        assert_eq!(blockers[0].code, REVIEW_STEP_MISSING);
        assert!(blockers[0].message.contains("code.review"));
        assert!(gate(Some(&plan_only), &active(&["skeptic"], &[])).is_empty());
    }

    #[test]
    fn gate_blocks_an_open_or_disputed_high_finding() {
        let open = parse(&finding("high", "")).value;
        let blockers = gate(Some(&open), &active(&["skeptic"], &["newcomer"]));
        assert_eq!(blockers.len(), 1);
        assert_eq!(blockers[0].code, REVIEW_UNRESOLVED);
        assert!(blockers[0].message.contains("R-security-1"));
        let disputed = parse(&finding(
            "high",
            "  - status: disputed\n    by: developer\n    note: n\n  - status: disputed\n    by: security\n    note: m\n",
        ))
        .value;
        assert_eq!(gate(Some(&disputed), &[]).len(), 1);
        // A medium finding does not block.
        assert!(gate(Some(&parse(&finding("medium", "")).value), &[]).is_empty());
    }

    #[test]
    fn gate_lets_an_accepted_risk_through() {
        let accepted = parse(&finding(
            "high",
            "  - status: accepted-risk\n    by: human\n    note: internal network only\n",
        ))
        .value;
        assert!(gate(Some(&accepted), &active(&["skeptic"], &["newcomer"])).is_empty());
    }

    #[test]
    fn gate_refuses_a_newer_format() {
        let newer = parse("<!-- codev-review: 2 -->\n").value;
        let blockers = gate(Some(&newer), &[]);
        assert_eq!(blockers[0].code, REVIEW_FORMAT_UNSUPPORTED);
        assert!(blockers[0].message.contains("update codev"));
    }
}
