//! Parseur d'une spec principale : `## Purpose`, `## Requirements`, avec ses
//! `### Requirement:` et leurs `#### Scenario:`.
//!
//! Ligne à ligne, un seul passage sur la source, avec le masque de fences
//! précalculé. Voir `design.md` de `parse-specs-and-deltas` pour le pourquoi.

use super::ast::{Finding, Parsed, PurposeBlock, Requirement, Spec};
use super::codes;
use super::fence;
use super::shared::{
    block_span, collect_scenarios, is_delta_header, line_starts, requirement_from_lines,
    section_header, trim_body, ScannedLine,
};

pub fn parse_spec(source: &str) -> Parsed<Spec> {
    let lines: Vec<&str> = source.split('\n').collect();
    let mask = fence::build_fence_mask(source);
    let starts = line_starts(source);

    let mut findings = Vec::new();
    let mut purpose: Option<PurposeBlock> = None;
    let mut requirements: Vec<Requirement> = Vec::new();

    // Repérer les sections `##` de premier niveau, en dehors des zones
    // littérales. On travaille sur `ScannedLine` pour porter la ligne
    // 1-indexée et le décalage en octets.
    let scanned: Vec<ScannedLine> = (0..lines.len())
        .map(|i| ScannedLine {
            text: lines[i],
            line_number: (i as u32) + 1,
            byte_offset: starts[i],
            literal: mask[i],
        })
        .collect();

    // Repérage des sections de premier niveau.
    #[derive(Debug)]
    struct H2 {
        title: String,
        header_index: usize,
    }
    let mut h2: Vec<H2> = Vec::new();
    for (i, entry) in scanned.iter().enumerate() {
        if entry.literal {
            continue;
        }
        if let Some(title) = section_header(entry.text) {
            h2.push(H2 {
                title,
                header_index: i,
            });
        }
    }

    // Détection des en-têtes de delta perdus dans une spec principale.
    for entry in &scanned {
        if entry.literal {
            continue;
        }
        if is_delta_header(entry.text) {
            findings.push(Finding::error(
                codes::DELTA_HEADER_IN_MAIN_SPEC,
                entry.line_number,
                format!(
                    "ligne {} : « {} » n'appartient qu'aux fichiers de change ; \
                     retire-le d'une spec principale",
                    entry.line_number,
                    entry.text.trim()
                ),
            ));
        }
    }

    // Purpose : premier `## Purpose` de premier niveau.
    let purpose_idx = h2
        .iter()
        .position(|h| h.title.eq_ignore_ascii_case("Purpose"));
    if let Some(idx) = purpose_idx {
        let start_line = h2[idx].header_index;
        let end_line = h2
            .get(idx + 1)
            .map(|next| next.header_index)
            .unwrap_or(lines.len());
        let body = trim_body(&scanned[start_line + 1..end_line]);
        let span = block_span(&scanned, start_line, end_line);
        purpose = Some(PurposeBlock { text: body, span });
    } else {
        findings.push(Finding::error(
            codes::SPEC_PURPOSE_MISSING,
            1,
            "la spec principale n'a pas de section `## Purpose`",
        ));
    }

    // Requirements : premier `## Requirements` — les exigences hors de cette
    // section sont signalées.
    let requirements_idx = h2
        .iter()
        .position(|h| h.title.eq_ignore_ascii_case("Requirements"));

    // `requirements_section_end` : point d'insertion des ADDED, en octets.
    // C'est le début de la prochaine section `##` de premier niveau après
    // `## Requirements`, ou la longueur du source si aucune ne suit.
    let requirements_section_end = requirements_idx.map(|idx| {
        h2.get(idx + 1)
            .map(|next| scanned[next.header_index].byte_offset)
            .unwrap_or(source.len())
    });

    let (req_start, req_end) = match requirements_idx {
        Some(idx) => {
            let start = h2[idx].header_index;
            let end = h2
                .get(idx + 1)
                .map(|next| next.header_index)
                .unwrap_or(lines.len());
            (Some(start), end)
        }
        None => (None, lines.len()),
    };

    // Repérer toutes les exigences de la source (même hors section) pour
    // pouvoir en signaler l'errance.
    let requirement_positions = super::shared::find_requirement_positions(&scanned);

    for pos in &requirement_positions {
        let inside_requirements = match req_start {
            Some(start) => pos.header_index > start && pos.header_index < req_end,
            None => false,
        };
        if !inside_requirements {
            findings.push(Finding::error(
                codes::REQUIREMENT_OUTSIDE_SECTION,
                pos.line_number,
                format!(
                    "ligne {} : `### Requirement:` hors de `## Requirements`",
                    pos.line_number
                ),
            ));
        }
    }

    // Extraction des exigences valides.
    if let Some(start) = req_start {
        let inside: Vec<_> = requirement_positions
            .iter()
            .filter(|p| p.header_index > start && p.header_index < req_end)
            .collect();
        for (rank, pos) in inside.iter().enumerate() {
            let next_start = inside
                .get(rank + 1)
                .map(|next| next.header_index)
                .unwrap_or(req_end);
            let (requirement, sub_findings) = requirement_from_lines(
                &scanned,
                pos.header_index,
                next_start,
                pos.name.clone(),
                &collect_scenarios,
            );
            findings.extend(sub_findings);
            requirements.push(requirement);
        }
    }

    Parsed {
        value: Spec {
            purpose,
            requirements,
            requirements_section_end,
        },
        findings,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast::Severity;

    fn purpose_only() -> &'static str {
        "# Auth Specification\n\n## Purpose\n\nAuthentification pour l'appli.\n\n## Requirements\n\n### Requirement: Session Expiration\nThe system MUST expire sessions after inactivity.\n\n#### Scenario: Idle\n\n- **WHEN** the user is idle\n- **THEN** the session expires\n"
    }

    #[test]
    fn extrait_purpose_et_une_exigence_avec_scenario() {
        let parsed = parse_spec(purpose_only());
        assert!(!parsed.has_errors(), "findings : {:?}", parsed.findings);

        let purpose = parsed.value.purpose.expect("purpose attendu");
        assert!(purpose.text.contains("Authentification"));

        assert_eq!(parsed.value.requirements.len(), 1);
        let req = &parsed.value.requirements[0];
        assert_eq!(req.name, "Session Expiration");
        assert_eq!(req.scenarios.len(), 1);
        assert_eq!(req.scenarios[0].name, "Idle");
    }

    #[test]
    fn purpose_manquant_est_un_finding_localise() {
        let source = "## Requirements\n\n### Requirement: X\nThe system SHALL x.\n\n#### Scenario: Y\n- **WHEN** a\n- **THEN** b\n";
        let parsed = parse_spec(source);
        assert!(parsed
            .findings
            .iter()
            .any(|f| f.code == "spec_purpose_missing" && f.severity == Severity::Error));
        // L'exigence reste extractible.
        assert_eq!(parsed.value.requirements.len(), 1);
    }

    #[test]
    fn exigence_hors_section_est_signalee() {
        // Requirement placé AVANT `## Requirements`.
        let source = "## Purpose\n\nx\n\n### Requirement: Errante\nThe system MUST y.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n\n## Requirements\n";
        let parsed = parse_spec(source);
        let f = parsed
            .findings
            .iter()
            .find(|f| f.code == "requirement_outside_section")
            .expect("finding attendu");
        assert!(f.message.contains("hors de"), "{}", f.message);
    }

    #[test]
    fn header_de_delta_dans_main_spec_est_signale() {
        let source = "## Purpose\n\nx\n\n## ADDED Requirements\n\n### Requirement: X\nThe system SHALL x.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n";
        let parsed = parse_spec(source);
        let f = parsed
            .findings
            .iter()
            .find(|f| f.code == "delta_header_in_main_spec")
            .expect("finding attendu");
        assert!(f.message.contains("ADDED"), "{}", f.message);
    }

    #[test]
    fn exigence_dans_fence_est_ignoree() {
        // Le piège : un `### Requirement: Exemple` dans un bloc de code n'est
        // pas une vraie exigence, et n'apparaît donc pas dans le résultat.
        let source = "## Purpose\n\nExemple :\n\n```\n### Requirement: Faux\n```\n\n## Requirements\n";
        let parsed = parse_spec(source);
        assert!(parsed.value.requirements.is_empty());
        // Et surtout pas de finding « requirement_outside_section », le
        // masquage doit intervenir avant.
        assert!(!parsed
            .findings
            .iter()
            .any(|f| f.code == "requirement_outside_section"));
    }

    #[test]
    fn requirements_section_end_est_avant_section_libre_qui_suit() {
        // La section Requirements est suivie d'une `## Notes` : le point
        // d'insertion des ADDED doit être juste avant cette section, pas à
        // la fin du fichier.
        let source = "## Purpose\n\nx.\n\n## Requirements\n\n### Requirement: R\nThe system SHALL r.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n\n## Notes\n\nBonus.\n";
        let parsed = parse_spec(source);
        let end = parsed.value.requirements_section_end.expect("Requirements présent");
        assert_eq!(
            &source[end..end + 8],
            "## Notes",
            "l'insertion doit tomber juste au début de la section suivante"
        );
    }

    #[test]
    fn requirements_section_end_va_a_la_fin_sans_section_suivante() {
        let source = "## Purpose\n\nx.\n\n## Requirements\n\n### Requirement: R\nThe system SHALL r.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n";
        let parsed = parse_spec(source);
        assert_eq!(
            parsed.value.requirements_section_end,
            Some(source.len()),
            "sans section suivante, on va au bout du fichier"
        );
    }

    #[test]
    fn scenario_trois_dieses_est_signale() {
        // `### Scenario:` au lieu de `#### Scenario:` : c'est le drame
        // silencieux qu'on refuse. L'exigence continue d'apparaître, sans ce
        // scénario.
        let source = "## Purpose\n\nx\n\n## Requirements\n\n### Requirement: R\nThe system SHALL r.\n\n### Scenario: Faux\n- **WHEN** a\n- **THEN** b\n";
        let parsed = parse_spec(source);
        assert!(parsed
            .findings
            .iter()
            .any(|f| f.code == "scenario_wrong_heading_level"));
        assert_eq!(parsed.value.requirements.len(), 1);
        assert!(parsed.value.requirements[0].scenarios.is_empty());
    }
}
