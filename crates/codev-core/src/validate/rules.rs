//! Les six règles pures jouées par le validateur.
//!
//! Une règle par struct, sans état — c'est ce qui autorise le registre à en
//! garder une référence `&'static`. Toute la logique se lit ici, pour qu'un
//! contributeur puisse ajouter la septième par symétrie.

use std::collections::{BTreeMap, BTreeSet};

use super::codes;
use super::Rule;
use crate::parser::ast::{Delta, DeltaSection, Finding, Requirement, Spec};

// ─────────────────────────── règles structurelles ────────────────────────────

pub struct RequirementNoShall;

impl Rule for RequirementNoShall {
    fn code(&self) -> &'static str {
        codes::REQUIREMENT_NO_SHALL
    }

    fn check_spec(&self, spec: &Spec) -> Vec<Finding> {
        spec.requirements
            .iter()
            .filter_map(|r| check_requirement_shall(r, "la spec principale"))
            .collect()
    }

    fn check_delta(&self, delta: &Delta) -> Vec<Finding> {
        added_and_modified(delta)
            .iter()
            .filter_map(|(section_label, r)| check_requirement_shall(r, section_label))
            .collect()
    }
}

/// Une exigence est normative si sa description contient `SHALL` ou `MUST`
/// **en majuscules exactes** — c'est la convention documentée dans le
/// schéma. Le nom de l'exigence est laissé libre : c'est une étiquette.
fn check_requirement_shall(req: &Requirement, context: &str) -> Option<Finding> {
    if req.description.contains("SHALL") || req.description.contains("MUST") {
        return None;
    }
    Some(Finding::error(
        codes::REQUIREMENT_NO_SHALL,
        req.span.start_line(),
        format!(
            "ligne {} : {context} — exigence « {} » sans `SHALL` ni `MUST` ; \
             emploie l'un des deux dans sa description",
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
            "ligne {} : exigence « {} » sans aucun scénario ; ajoute au moins un `#### Scenario:`",
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
            "la spec principale n'a aucune exigence extractible ; \
             ajoute au moins un `### Requirement:` sous `## Requirements`",
        )]
    }
}

// ─────────────────────────── règles de cohérence ────────────────────────────

pub struct CrossSectionConflict;

impl Rule for CrossSectionConflict {
    fn code(&self) -> &'static str {
        codes::CROSS_SECTION_CONFLICT
    }

    fn check_delta(&self, delta: &Delta) -> Vec<Finding> {
        // Indexer le nom → liste de (section, ligne). Deux entrées ou plus
        // pour un même nom signifie collision.
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
            // La ligne rapportée est celle de la première section touchée —
            // c'est là qu'un utilisateur commencera à corriger.
            let first_line = occurrences[0].1;
            let listing = occurrences
                .iter()
                .map(|(label, line)| format!("{label} (ligne {line})"))
                .collect::<Vec<_>>()
                .join(", ");
            findings.push(Finding::error(
                codes::CROSS_SECTION_CONFLICT,
                first_line,
                format!(
                    "exigence « {name} » présente dans plusieurs sections : {listing} ; \
                     une même exigence ne peut figurer qu'une fois"
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
        // Un `RENAMED.TO` qui coïncide avec un `ADDED` produit une exigence
        // ambiguë au moment de l'archive.
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
                                "ligne {} : `TO: {}` collide avec un `ADDED` de même nom ; \
                                 choisis un autre nom cible, ou retire l'ADDED redondant",
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
        // Un `MODIFIED` porte le NOUVEAU nom, jamais l'ancien : l'ancien
        // n'existera plus après la fusion, donc la modification pointerait
        // dans le vide.
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
                                "ligne {} : `MODIFIED` référence « {} » qui est renommée en « {} » ; \
                                 utilise le nouveau nom",
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

// ─────────────────────────── outils partagés ────────────────────────────

fn section_label(section: &DeltaSection) -> &'static str {
    match section {
        DeltaSection::Added { .. } => "ADDED",
        DeltaSection::Modified { .. } => "MODIFIED",
        DeltaSection::Removed { .. } => "REMOVED",
        DeltaSection::Renamed { .. } => "RENAMED",
    }
}

/// Les noms d'exigences touchées par une section, quelle que soit sa forme.
///
/// Pour `RENAMED`, la « touche » est le couple, donc on n'expose ni `from`
/// ni `to` — la règle `CrossSectionConflict` ne raisonne que sur ce qui a
/// une position d'exigence unique (ADDED/MODIFIED/REMOVED).
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
                .map(|r| ("la section ADDED", r))
                .collect::<Vec<_>>(),
            DeltaSection::Modified { requirements, .. } => requirements
                .iter()
                .map(|r| ("la section MODIFIED", r))
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
    fn requirement_sans_shall_est_signalee() {
        // La description ne contient ni SHALL ni MUST : la règle mord.
        let source = "## ADDED Requirements\n\n### Requirement: X\nThe system does something.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n";
        let findings = run_delta(source);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].code, codes::REQUIREMENT_NO_SHALL);
        assert!(findings[0].message.contains("« X »"), "{}", findings[0].message);
    }

    #[test]
    fn shall_ou_must_satisfait_la_regle() {
        let source_shall = "## ADDED Requirements\n\n### Requirement: X\nThe system SHALL do it.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n";
        let source_must = "## ADDED Requirements\n\n### Requirement: X\nThe system MUST do it.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n";
        for s in [source_shall, source_must] {
            let findings = run_delta(s);
            assert!(
                !findings.iter().any(|f| f.code == codes::REQUIREMENT_NO_SHALL),
                "SHALL/MUST doit satisfaire la règle : {findings:?}"
            );
        }
    }

    #[test]
    fn requirement_sans_scenario_est_signalee() {
        // Une exigence dépourvue de scénario est repérée.
        let source = "## ADDED Requirements\n\n### Requirement: Y\nThe system SHALL do it.\n";
        let findings = run_delta(source);
        assert!(findings
            .iter()
            .any(|f| f.code == codes::REQUIREMENT_NO_SCENARIO && f.message.contains("Y")));
    }

    #[test]
    fn spec_sans_requirement_est_signalee() {
        let source = "## Purpose\n\nUne belle spec sans contenu.\n\n## Requirements\n";
        let findings = run_spec(source);
        assert!(findings.iter().any(|f| f.code == codes::SPEC_NO_REQUIREMENT));
    }

    #[test]
    fn exigence_dans_added_et_modified_est_signalee() {
        let source = "## ADDED Requirements\n\n### Requirement: Z\nThe system SHALL z.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n\n## MODIFIED Requirements\n\n### Requirement: Z\nThe system SHALL z updated.\n\n#### Scenario: T\n- **WHEN** c\n- **THEN** d\n";
        let findings = run_delta(source);
        let cross: Vec<_> = findings
            .iter()
            .filter(|f| f.code == codes::CROSS_SECTION_CONFLICT)
            .collect();
        assert_eq!(cross.len(), 1, "un seul conflit sur « Z »");
        assert!(cross[0].message.contains("ADDED"), "{}", cross[0].message);
        assert!(cross[0].message.contains("MODIFIED"), "{}", cross[0].message);
    }

    #[test]
    fn rename_to_qui_collide_avec_added_est_signale() {
        let source = "## ADDED Requirements\n\n### Requirement: New Name\nThe system SHALL x.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n\n## RENAMED Requirements\n\n- FROM: Old Name\n- TO: New Name\n";
        let findings = run_delta(source);
        assert!(findings
            .iter()
            .any(|f| f.code == codes::RENAME_TARGET_COLLISION
                && f.message.contains("New Name")));
    }

    #[test]
    fn modified_reference_ancien_nom_renamed_est_signale() {
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
