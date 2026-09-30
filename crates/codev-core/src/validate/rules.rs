//! The six pure rules run by the validator.
//!
//! One stateless struct per rule — which is what lets the registry hold
//! a `&'static` reference to each. All the logic reads here, so that a
//! contributor can add the seventh by symmetry.

use std::collections::{BTreeMap, BTreeSet};

use super::Rule;
use super::codes;
use crate::parser::ast::{Delta, DeltaSection, Finding, Requirement, Spec};

// ─────────────────────────── structural rules ────────────────────────────

pub struct RequirementNoShall;

impl Rule for RequirementNoShall {
    fn code(&self) -> &'static str {
        codes::REQUIREMENT_NO_SHALL
    }

    fn check_spec(&self, spec: &Spec) -> Vec<Finding> {
        spec.requirements
            .iter()
            .filter_map(|r| check_requirement_shall(r, "main spec"))
            .collect()
    }

    fn check_delta(&self, delta: &Delta) -> Vec<Finding> {
        added_and_modified(delta)
            .iter()
            .filter_map(|(section_label, r)| check_requirement_shall(r, section_label))
            .collect()
    }
}

/// A requirement is normative if its description contains `SHALL` or `MUST`
/// **in exact uppercase** — this is the convention documented in the
/// schema. The requirement name is left free-form: it is a label.
fn check_requirement_shall(req: &Requirement, context: &str) -> Option<Finding> {
    if req.description.contains("SHALL") || req.description.contains("MUST") {
        return None;
    }
    Some(Finding::error(
        codes::REQUIREMENT_NO_SHALL,
        req.span.start_line(),
        format!(
            "line {}: {context} — requirement \"{}\" has neither `SHALL` nor `MUST`; \
             use one of them in its description",
            req.span.start_line(),
            req.name
        ),
    ))
}

pub struct RequirementNoScenario;

impl Rule for RequirementNoScenario {
    fn code(&self) -> &'static str {
        codes::REQUIREMENT_NO_SCENARIO
    }

    fn check_spec(&self, spec: &Spec) -> Vec<Finding> {
        spec.requirements
            .iter()
            .filter_map(check_requirement_scenario)
            .collect()
    }

    fn check_delta(&self, delta: &Delta) -> Vec<Finding> {
        added_and_modified(delta)
            .iter()
            .filter_map(|(_, r)| check_requirement_scenario(r))
            .collect()
    }
}

fn check_requirement_scenario(req: &Requirement) -> Option<Finding> {
    if !req.scenarios.is_empty() {
        return None;
    }
    Some(Finding::error(
        codes::REQUIREMENT_NO_SCENARIO,
        req.span.start_line(),
        format!(
            "line {}: requirement \"{}\" has no scenario; add at least one `#### Scenario:`",
            req.span.start_line(),
            req.name
        ),
    ))
}

pub struct SpecNoRequirement;

impl Rule for SpecNoRequirement {
    fn code(&self) -> &'static str {
        codes::SPEC_NO_REQUIREMENT
    }

    fn check_spec(&self, spec: &Spec) -> Vec<Finding> {
        if !spec.requirements.is_empty() {
            return Vec::new();
        }
        vec![Finding::error(
            codes::SPEC_NO_REQUIREMENT,
            1,
            "main spec has no extractable requirement; \
             add at least one `### Requirement:` under `## Requirements`",
        )]
    }
}

// ─────────────────────────── consistency rules ────────────────────────────

pub struct CrossSectionConflict;

impl Rule for CrossSectionConflict {
    fn code(&self) -> &'static str {
        codes::CROSS_SECTION_CONFLICT
    }

    fn check_delta(&self, delta: &Delta) -> Vec<Finding> {
        // Index name → list of (section, line). Two or more entries
        // for the same name means a collision.
        let mut index: BTreeMap<String, Vec<(&'static str, u32)>> = BTreeMap::new();
        for section in &delta.sections {
            let label = section_label(section);
            for name in requirement_names(section) {
                index
                    .entry(name)
                    .or_default()
                    .push((label, section.span().start_line()));
            }
        }

        let mut findings = Vec::new();
        for (name, occurrences) in index {
            if occurrences.len() < 2 {
                continue;
            }
            // The reported line is that of the first affected section —
            // that is where a user will start fixing.
            let first_line = occurrences[0].1;
            let listing = occurrences
                .iter()
                .map(|(label, line)| format!("{label} (line {line})"))
                .collect::<Vec<_>>()
                .join(", ");
            findings.push(Finding::error(
                codes::CROSS_SECTION_CONFLICT,
                first_line,
                format!(
                    "requirement \"{name}\" appears in several sections: {listing}; \
                     a requirement may appear only once"
                ),
            ));
        }
        findings
    }
}

pub struct RenameTargetCollision;

impl Rule for RenameTargetCollision {
    fn code(&self) -> &'static str {
        codes::RENAME_TARGET_COLLISION
    }

    fn check_delta(&self, delta: &Delta) -> Vec<Finding> {
        // A `RENAMED.TO` that coincides with an `ADDED` yields an ambiguous
        // requirement at archive time.
        let added_names: BTreeSet<&str> = delta
            .sections
            .iter()
            .filter_map(|s| match s {
                DeltaSection::Added { requirements, .. } => Some(requirements),
                _ => None,
            })
            .flatten()
            .map(|r| r.name.as_str())
            .collect();

        let mut findings = Vec::new();
        for section in &delta.sections {
            if let DeltaSection::Renamed { renames, span } = section {
                for rename in renames {
                    if added_names.contains(rename.to.as_str()) {
                        findings.push(Finding::error(
                            codes::RENAME_TARGET_COLLISION,
                            rename.span.start_line().max(span.start_line()),
                            format!(
                                "line {}: `TO: {}` collides with an `ADDED` of the same name; \
                                 choose another target name, or remove the redundant ADDED",
                                rename.span.start_line(),
                                rename.to
                            ),
                        ));
                    }
                }
            }
        }
        findings
    }
}

pub struct ModifiedUsesOldName;

impl Rule for ModifiedUsesOldName {
    fn code(&self) -> &'static str {
        codes::MODIFIED_USES_OLD_NAME
    }

    fn check_delta(&self, delta: &Delta) -> Vec<Finding> {
        // A `MODIFIED` carries the NEW name, never the old one: the old one
        // will no longer exist after the merge, so the modification would point
        // at nothing.
        let renames: BTreeMap<&str, &str> = delta
            .sections
            .iter()
            .filter_map(|s| match s {
                DeltaSection::Renamed { renames, .. } => Some(renames),
                _ => None,
            })
            .flatten()
            .map(|r| (r.from.as_str(), r.to.as_str()))
            .collect();

        if renames.is_empty() {
            return Vec::new();
        }

        let mut findings = Vec::new();
        for section in &delta.sections {
            if let DeltaSection::Modified { requirements, .. } = section {
                for req in requirements {
                    if let Some(new_name) = renames.get(req.name.as_str()) {
                        findings.push(Finding::error(
                            codes::MODIFIED_USES_OLD_NAME,
                            req.span.start_line(),
                            format!(
                                "line {}: `MODIFIED` references \"{}\", which is renamed to \"{}\"; \
                                 use the new name",
                                req.span.start_line(),
                                req.name,
                                new_name
                            ),
                        ));
                    }
                }
            }
        }
        findings
    }
}

// ─────────────────────────── shared helpers ────────────────────────────

fn section_label(section: &DeltaSection) -> &'static str {
    match section {
        DeltaSection::Added { .. } => "ADDED",
        DeltaSection::Modified { .. } => "MODIFIED",
        DeltaSection::Removed { .. } => "REMOVED",
        DeltaSection::Renamed { .. } => "RENAMED",
    }
}

/// The names of the requirements touched by a section, whatever its shape.
///
/// For `RENAMED`, what is "touched" is the pair, so neither `from`
/// nor `to` is exposed — the `CrossSectionConflict` rule only reasons about
/// what has a single requirement position (ADDED/MODIFIED/REMOVED).
fn requirement_names(section: &DeltaSection) -> Vec<String> {
    match section {
        DeltaSection::Added { requirements, .. } | DeltaSection::Modified { requirements, .. } => {
            requirements.iter().map(|r| r.name.clone()).collect()
        }
        DeltaSection::Removed { removals, .. } => removals.iter().map(|r| r.name.clone()).collect(),
        DeltaSection::Renamed { .. } => Vec::new(),
    }
}

fn added_and_modified(delta: &Delta) -> Vec<(&'static str, &Requirement)> {
    delta
        .sections
        .iter()
        .flat_map(|s| match s {
            DeltaSection::Added { requirements, .. } => requirements
                .iter()
                .map(|r| ("ADDED section", r))
                .collect::<Vec<_>>(),
            DeltaSection::Modified { requirements, .. } => requirements
                .iter()
                .map(|r| ("MODIFIED section", r))
                .collect::<Vec<_>>(),
            _ => Vec::new(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{parse_delta, parse_spec};

    fn run_delta(source: &str) -> Vec<Finding> {
        let parsed = parse_delta(source);
        super::super::check_delta(&parsed.value)
    }

    fn run_spec(source: &str) -> Vec<Finding> {
        let parsed = parse_spec(source);
        super::super::check_spec(&parsed.value)
    }

    #[test]
    fn requirement_without_shall_is_reported() {
        // The description contains neither SHALL nor MUST: the rule fires.
        let source = "## ADDED Requirements\n\n### Requirement: X\nThe system does something.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n";
        let findings = run_delta(source);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].code, codes::REQUIREMENT_NO_SHALL);
        assert!(
            findings[0].message.contains("\"X\""),
            "{}",
            findings[0].message
        );
    }

    #[test]
    fn shall_or_must_satisfies_the_rule() {
        let source_shall = "## ADDED Requirements\n\n### Requirement: X\nThe system SHALL do it.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n";
        let source_must = "## ADDED Requirements\n\n### Requirement: X\nThe system MUST do it.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n";
        for s in [source_shall, source_must] {
            let findings = run_delta(s);
            assert!(
                !findings
                    .iter()
                    .any(|f| f.code == codes::REQUIREMENT_NO_SHALL),
                "SHALL/MUST must satisfy the rule: {findings:?}"
            );
        }
    }

    #[test]
    fn requirement_without_scenario_is_reported() {
        // A requirement lacking a scenario is caught.
        let source = "## ADDED Requirements\n\n### Requirement: Y\nThe system SHALL do it.\n";
        let findings = run_delta(source);
        assert!(
            findings
                .iter()
                .any(|f| f.code == codes::REQUIREMENT_NO_SCENARIO && f.message.contains("Y"))
        );
    }

    #[test]
    fn spec_without_requirement_is_reported() {
        let source = "## Purpose\n\nA nice spec with no content.\n\n## Requirements\n";
        let findings = run_spec(source);
        assert!(
            findings
                .iter()
                .any(|f| f.code == codes::SPEC_NO_REQUIREMENT)
        );
    }

    #[test]
    fn requirement_in_added_and_modified_is_reported() {
        let source = "## ADDED Requirements\n\n### Requirement: Z\nThe system SHALL z.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n\n## MODIFIED Requirements\n\n### Requirement: Z\nThe system SHALL z updated.\n\n#### Scenario: T\n- **WHEN** c\n- **THEN** d\n";
        let findings = run_delta(source);
        let cross: Vec<_> = findings
            .iter()
            .filter(|f| f.code == codes::CROSS_SECTION_CONFLICT)
            .collect();
        assert_eq!(cross.len(), 1, "a single conflict on \"Z\"");
        assert!(cross[0].message.contains("ADDED"), "{}", cross[0].message);
        assert!(
            cross[0].message.contains("MODIFIED"),
            "{}",
            cross[0].message
        );
    }

    #[test]
    fn rename_to_colliding_with_added_is_reported() {
        let source = "## ADDED Requirements\n\n### Requirement: New Name\nThe system SHALL x.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n\n## RENAMED Requirements\n\n- FROM: Old Name\n- TO: New Name\n";
        let findings = run_delta(source);
        assert!(findings
            .iter()
            .any(|f| f.code == codes::RENAME_TARGET_COLLISION
                && f.message.contains("New Name")));
    }

    #[test]
    fn modified_referencing_renamed_old_name_is_reported() {
        let source = "## MODIFIED Requirements\n\n### Requirement: Old Name\nThe system MUST evolve.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n\n## RENAMED Requirements\n\n- FROM: Old Name\n- TO: New Name\n";
        let findings = run_delta(source);
        let modified_uses: Vec<_> = findings
            .iter()
            .filter(|f| f.code == codes::MODIFIED_USES_OLD_NAME)
            .collect();
        assert_eq!(modified_uses.len(), 1);
        assert!(modified_uses[0].message.contains("Old Name"));
        assert!(modified_uses[0].message.contains("New Name"));
    }
}
