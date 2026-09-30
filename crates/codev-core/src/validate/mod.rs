//! Validation rules, applied to an already-parsed AST.
//!
//! Two phases, as everywhere: deciding is not executing. Here we decide, from
//! the structure of the content alone — no I/O. The coordination
//! (disk walk, grouping, reporting) lives in `codev-engine::validate`.
//!
//! A rule is an implementation of [`Rule`] registered in [`RULES`].
//! Adding a rule (for instance a new warning) is one more struct in the
//! registry, without touching the callers.

use crate::parser::ast::{Delta, Finding, Spec};

pub mod codes;
pub mod rules;

/// Contract of a rule: it can report its stable `code` and fire on
/// a main spec, a delta, or both.
///
/// Both `check_*` methods are optional so that a specialized rule
/// (for example `SpecNoRequirement`) need not bother with the
/// type it does not care about.
pub trait Rule: Sync {
    fn code(&self) -> &'static str;

    fn check_spec(&self, _spec: &Spec) -> Vec<Finding> {
        Vec::new()
    }

    fn check_delta(&self, _delta: &Delta) -> Vec<Finding> {
        Vec::new()
    }
}

/// The catalog of rules run by `codev validate`.
///
/// Order is significant: findings for the same file appear
/// in this order, which keeps the output stable and therefore testable by
/// snapshot. Inserting a rule in the middle is not a trivial decision —
/// it changes the visible contract.
pub const RULES: &[&dyn Rule] = &[
    &rules::RequirementNoShall,
    &rules::RequirementNoScenario,
    &rules::SpecNoRequirement,
    &rules::CrossSectionConflict,
    &rules::RenameTargetCollision,
    &rules::ModifiedUsesOldName,
];

/// Applies every rule to a main spec and concatenates their
/// `Finding`s.
pub fn check_spec(spec: &Spec) -> Vec<Finding> {
    RULES.iter().flat_map(|r| r.check_spec(spec)).collect()
}

/// Applies every rule to a delta and concatenates their `Finding`s.
pub fn check_delta(delta: &Delta) -> Vec<Finding> {
    RULES.iter().flat_map(|r| r.check_delta(delta)).collect()
}

/// Codes of the checks that read a delta together with its main spec.
///
/// Kept apart from [`RULES`]: a [`Rule`] sees one file, these need two.
pub const CROSS_FILE_CODES: &[&str] = &[codes::RENAME_SOURCE_MISSING];

/// Checks a delta against the main spec it targets — `None` when the
/// capability has no main spec yet.
///
/// A `RENAMED.FROM` must name an existing requirement. A rename already
/// applied by an earlier `sync` (`FROM` gone, `TO` present) is not an
/// error: re-syncing, then archiving, must stay possible.
pub fn check_delta_against_spec(delta: &Delta, spec: Option<&Spec>) -> Vec<Finding> {
    let exists = |name: &str| spec.is_some_and(|s| s.requirements.iter().any(|r| r.name == name));
    let mut findings = Vec::new();
    for section in &delta.sections {
        let crate::parser::ast::DeltaSection::Renamed { renames, .. } = section else {
            continue;
        };
        for rename in renames {
            if exists(&rename.from) || exists(&rename.to) {
                continue;
            }
            let line = rename.span.start_line();
            findings.push(Finding::error(
                codes::RENAME_SOURCE_MISSING,
                line,
                format!(
                    "line {line}: `FROM: {}` matches no requirement of the main spec; \
                     use the exact name of an existing `### Requirement:`",
                    rename.from
                ),
            ));
        }
    }
    findings
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn registry_lists_all_rules() {
        // The catalog must contain the 6 advertised rules; if one
        // disappeared by mistake, the `validate` command would cover less than
        // what the spec promises.
        assert_eq!(RULES.len(), 6);
    }

    #[test]
    fn finding_codes_are_unique() {
        // The public contract: each `code` identifies ONE defect. A
        // collision between the parser and validate — or between two rules —
        // would make a consumer that matches on `code` treat two cases
        // as one.
        let mut seen: BTreeSet<&'static str> = BTreeSet::new();
        for code in crate::parser::codes::ALL {
            assert!(seen.insert(code), "duplicate parser code: {code}");
        }
        for rule in RULES {
            assert!(
                seen.insert(rule.code()),
                "rule code duplicates an earlier one: {}",
                rule.code()
            );
        }
        for code in CROSS_FILE_CODES {
            assert!(seen.insert(code), "cross-file code duplicates: {code}");
        }
    }

    fn spec(source: &str) -> Spec {
        crate::parser::parse_spec(source).value
    }

    fn delta(source: &str) -> Delta {
        crate::parser::parse_delta(source).value
    }

    const MAIN: &str = "## Purpose\n\nx.\n\n## Requirements\n\n### Requirement: Session Expiration\nThe system MUST expire.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n";

    #[test]
    fn rename_from_an_unknown_requirement_is_an_error() {
        let d = delta("## RENAMED Requirements\n\n- FROM: Session Expiry\n- TO: Session Timeout\n");
        let findings = check_delta_against_spec(&d, Some(&spec(MAIN)));
        assert_eq!(findings.len(), 1, "{findings:?}");
        assert_eq!(findings[0].code, codes::RENAME_SOURCE_MISSING);
        assert!(findings[0].message.contains("Session Expiry"));
        // No main spec at all: nothing to rename either.
        assert_eq!(check_delta_against_spec(&d, None).len(), 1);
    }

    #[test]
    fn rename_from_an_existing_or_already_renamed_requirement_passes() {
        let backticked = delta(
            "## RENAMED Requirements\n\n- FROM: `### Requirement: Session Expiration`\n- TO: `### Requirement: Session Timeout`\n",
        );
        assert!(check_delta_against_spec(&backticked, Some(&spec(MAIN))).is_empty());
        let applied = spec(&MAIN.replace("Session Expiration", "Session Timeout"));
        assert!(check_delta_against_spec(&backticked, Some(&applied)).is_empty());
    }
}
