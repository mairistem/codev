//! Briques réutilisées par `spec.rs` et `delta.rs` : décalages de lignes,
//! reconnaissance d'en-têtes, extraction d'exigences et de scénarios.
//!
//! Non exposé publiquement (`pub(super)` uniquement) : c'est de la mécanique
//! d'implémentation, pas du contrat.

use super::ast::{Finding, Requirement, Scenario, Span};

/// Une ligne du source, augmentée de ce qu'il faut savoir sur elle sans avoir
/// à recalculer.
///
/// - `line_number` est 1-indexé (comme un éditeur l'affiche).
/// - `byte_offset` est le décalage du premier octet de la ligne dans le
///   source d'entrée — celui-là même que les spans utilisent.
/// - `literal` vient du masque de fences ; il est consulté à chaque
///   reconnaissance d'en-tête pour ne pas tomber dans le piège du faux
///   positif à l'intérieur d'un bloc de code.
#[derive(Debug, Clone, Copy)]
pub(super) struct ScannedLine<'a> {
    pub text: &'a str,
    pub line_number: u32,
    pub byte_offset: usize,
    pub literal: bool,
}

pub(super) fn line_starts(source: &str) -> Vec<usize> {
    // Précondition : la longueur du vecteur retourné DOIT être exactement
    // celle de `source.split('\n')`. Sans cela, les indices lus par les
    // parseurs pointent dans le vide, silencieusement.
    let mut starts = Vec::with_capacity(source.len() / 40 + 1);
    starts.push(0);
    for (i, byte) in source.bytes().enumerate() {
        if byte == b'\n' {
            starts.push(i + 1);
        }
    }
    starts
}

/// Reconnaît un en-tête `## <titre>` de premier niveau, hors zone littérale.
///
/// Retourne le titre nettoyé, ou `None` si la ligne n'est pas un en-tête `##`.
/// Un en-tête `##` suivi d'un `#` (donc `###`) n'est pas de premier niveau.
pub(super) fn section_header(line: &str) -> Option<String> {
    let trimmed = line.trim_start();
    let after = trimmed.strip_prefix("## ")?;
    if after.starts_with('#') {
        return None;
    }
    Some(after.trim().to_string())
}

/// Reconnaît `## ADDED Requirements`, `## MODIFIED Requirements`, etc.
pub(super) fn is_delta_header(line: &str) -> bool {
    delta_section_kind(line).is_some()
}

pub(super) fn delta_section_kind(line: &str) -> Option<DeltaKind> {
    let title = section_header(line)?;
    let upper = title.to_ascii_uppercase();
    let stripped = upper.strip_suffix("REQUIREMENTS")?.trim_end();
    match stripped {
        "ADDED" => Some(DeltaKind::Added),
        "MODIFIED" => Some(DeltaKind::Modified),
        "REMOVED" => Some(DeltaKind::Removed),
        "RENAMED" => Some(DeltaKind::Renamed),
        _ => None,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum DeltaKind {
    Added,
    Modified,
    Removed,
    Renamed,
}

pub(super) struct RequirementPosition {
    pub name: String,
    pub header_index: usize,
    pub line_number: u32,
}

pub(super) fn find_requirement_positions(scanned: &[ScannedLine]) -> Vec<RequirementPosition> {
    find_requirement_positions_in_range(scanned, 0, scanned.len())
}

pub(super) fn find_requirement_positions_in_range(
    scanned: &[ScannedLine],
    start: usize,
    end: usize,
) -> Vec<RequirementPosition> {
    let bounded_end = end.min(scanned.len());
    scanned[start..bounded_end]
        .iter()
        .enumerate()
        .filter(|(_, entry)| !entry.literal)
        .filter_map(|(offset, entry)| {
            requirement_header(entry.text).map(|name| RequirementPosition {
                name,
                header_index: start + offset,
                line_number: entry.line_number,
            })
        })
        .collect()
}

fn requirement_header(line: &str) -> Option<String> {
    let trimmed = line.trim_start();
    let after = trimmed.strip_prefix("### Requirement:")?;
    if after.starts_with('#') {
        return None; // c'est un `####` : pas notre en-tête
    }
    Some(after.trim().to_string())
}

/// Extrait un `#### Scenario:` — exactement quatre dièses. Trois dièses est le
/// piège que l'on doit détecter et signaler, pas silencieusement traiter.
fn scenario_header(line: &str) -> Option<String> {
    let trimmed = line.trim_start();
    let after = trimmed.strip_prefix("#### Scenario:")?;
    if after.starts_with('#') {
        return None;
    }
    Some(after.trim().to_string())
}

/// Détecte le piège : `### Scenario:` au lieu de `#### Scenario:`.
fn wrong_level_scenario(line: &str) -> Option<String> {
    let trimmed = line.trim_start();
    let after = trimmed.strip_prefix("### Scenario:")?;
    Some(after.trim().to_string())
}

/// Signature d'un extracteur de corps d'exigence. Nommer le type garde la
/// signature de [`requirement_from_lines`] lisible.
pub(super) type BodyCollector =
    dyn Fn(&[ScannedLine], usize, usize, &mut Vec<Finding>) -> (String, Vec<Scenario>);

/// Assemble un `Requirement` complet à partir d'une plage de lignes délimitée
/// par son en-tête et le début de l'exigence suivante (ou la fin de la
/// section).
pub(super) fn requirement_from_lines(
    scanned: &[ScannedLine],
    header_index: usize,
    end_index: usize,
    name: String,
    collect: &BodyCollector,
) -> (Requirement, Vec<Finding>) {
    let mut findings = Vec::new();
    let (description, scenarios) = collect(scanned, header_index, end_index, &mut findings);
    let span = block_span(scanned, header_index, end_index);
    (
        Requirement {
            name,
            description,
            scenarios,
            span,
        },
        findings,
    )
}

/// Collecte le paragraphe descriptif jusqu'au premier scénario, puis les
/// scénarios eux-mêmes.
pub(super) fn collect_scenarios(
    scanned: &[ScannedLine],
    header_index: usize,
    end_index: usize,
    findings: &mut Vec<Finding>,
) -> (String, Vec<Scenario>) {
    // Trouver la première ligne « scénario » (bien formée) après l'en-tête,
    // en signalant au passage les mal-formées (`### Scenario:` — le piège
    // silencieux, cf. la spec).
    let mut first_scenario: Option<usize> = None;
    for (offset, entry) in scanned[header_index + 1..end_index].iter().enumerate() {
        if entry.literal {
            continue;
        }
        if scenario_header(entry.text).is_some() {
            first_scenario = Some(header_index + 1 + offset);
            break;
        }
        if wrong_level_scenario(entry.text).is_some() {
            findings.push(Finding::error(
                super::codes::SCENARIO_WRONG_HEADING_LEVEL,
                entry.line_number,
                format!(
                    "ligne {} : un scénario doit porter exactement quatre dièses (`#### Scenario:`), pas trois",
                    entry.line_number
                ),
            ));
        }
    }

    let description_end = first_scenario.unwrap_or(end_index);
    let description = trim_body(&scanned[header_index + 1..description_end]);

    // Collecte des scénarios : chaque `#### Scenario:` démarre un bloc, qui
    // s'étend jusqu'au prochain scénario ou la fin de l'exigence.
    let mut scenarios = Vec::new();
    if let Some(first) = first_scenario {
        let positions: Vec<usize> = (first..end_index)
            .filter(|&i| {
                let entry = &scanned[i];
                !entry.literal && scenario_header(entry.text).is_some()
            })
            .collect();
        for (rank, &pos) in positions.iter().enumerate() {
            let stop = positions.get(rank + 1).copied().unwrap_or(end_index);
            let name = scenario_header(scanned[pos].text).expect("filtré au-dessus");
            let body = trim_body(&scanned[pos + 1..stop]);
            let span = block_span(scanned, pos, stop);
            scenarios.push(Scenario { name, body, span });
        }
    }
    (description, scenarios)
}

/// Rassemble le texte d'un bloc en supprimant les lignes vides de tête et de
/// queue, sans altérer les lignes internes.
pub(super) fn trim_body(lines: &[ScannedLine]) -> String {
    let mut start = 0;
    while start < lines.len() && lines[start].text.trim().is_empty() {
        start += 1;
    }
    let mut end = lines.len();
    while end > start && lines[end - 1].text.trim().is_empty() {
        end -= 1;
    }
    lines[start..end]
        .iter()
        .map(|l| l.text)
        .collect::<Vec<_>>()
        .join("\n")
}

/// Le span exact d'un bloc : de l'en-tête (inclus) à la ligne juste avant le
/// prochain bloc (exclue).
///
/// Deux effets utiles :
/// - `byte_range` couvre les octets de l'en-tête à la fin du dernier caractère
///   du bloc — remplacer cette plage réécrit le bloc en entier.
/// - `line_range` couvre les lignes 1-indexées de la même zone. `start` est
///   la ligne de l'en-tête, `end` est la première ligne du bloc suivant (ou
///   `lines.len() + 1` si c'est le dernier).
pub(super) fn block_span(scanned: &[ScannedLine], start: usize, end: usize) -> Span {
    let start_byte = scanned[start].byte_offset;
    let end_byte = if end >= scanned.len() {
        // Dernier bloc : jusqu'à la fin du source. Le décalage de la ligne
        // « virtuelle » ajoutée par `split('\n')` couvre ce cas.
        scanned
            .last()
            .map(|l| l.byte_offset + l.text.len())
            .unwrap_or(0)
    } else {
        scanned[end].byte_offset
    };
    let start_line = scanned[start].line_number;
    let end_line = if end >= scanned.len() {
        scanned
            .last()
            .map(|l| l.line_number + 1)
            .unwrap_or(start_line + 1)
    } else {
        scanned[end].line_number
    };
    Span::new(start_byte..end_byte, start_line..end_line)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_starts_indique_le_decalage_de_chaque_ligne() {
        let source = "a\nbb\n\nccc\n";
        // Une ligne finale vide (issue du `\n` de fin) est attendue —
        // `split('\n')` en produit une, `line_starts` doit s'y aligner.
        assert_eq!(line_starts(source), vec![0, 2, 5, 6, 10]);
        assert_eq!(source.split('\n').count(), line_starts(source).len());
    }

    #[test]
    fn section_header_reconnait_les_niveaux_2_seulement() {
        assert_eq!(section_header("## Purpose").as_deref(), Some("Purpose"));
        assert_eq!(section_header("##   Requirements  ").as_deref(), Some("Requirements"));
        assert_eq!(section_header("### Requirement: X"), None);
        assert_eq!(section_header("# Titre"), None);
        assert_eq!(section_header("hello"), None);
    }

    #[test]
    fn delta_headers_sont_reconnus_insensiblement_a_la_casse() {
        assert_eq!(delta_section_kind("## ADDED Requirements"), Some(DeltaKind::Added));
        assert_eq!(delta_section_kind("## modified requirements"), Some(DeltaKind::Modified));
        assert_eq!(delta_section_kind("## REMOVED Requirements"), Some(DeltaKind::Removed));
        assert_eq!(delta_section_kind("## RENAMED Requirements"), Some(DeltaKind::Renamed));
        assert_eq!(delta_section_kind("## Removed"), None);
    }
}
