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
    }
}
