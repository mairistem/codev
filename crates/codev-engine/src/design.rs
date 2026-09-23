//! Parseur léger des blocs `### Décision : ...` d'un `design.md` de change.
//!
//! Format très cadré : la section `## Décisions` contient N blocs, chacun
//! commençant par `### Décision : <titre>`, et se terminant au prochain
//! `### ` ou `## ` (au niveau H3 ou H2). Un scan ligne à ligne suffit — pas
//! besoin d'un parseur markdown complet, comme la décision
//! `_codev/decisions/0001-coeur-fonctionnel-coquille-imperative.md` le
//! rappelle : ce module est pur, aucune I/O, il vit côté engine parce
//! qu'appelé par des fonctions qui ont besoin des ports.
//!
//! Le corps d'un bloc est extrait **byte pour byte** — cohérent avec K3
//! (sceau sur le corps d'ADR byte pour byte) : un consommateur qui
//! promeut un bloc en ADR retrouve son texte exactement.

use std::ops::Range;

const H2_DECISIONS: &str = "## Décisions";
const H3_DECISION_PREFIX: &str = "### Décision : ";

/// Un bloc `### Décision : <titre>` extrait d'un `design.md`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionBlock {
    /// Ce qui suit `### Décision : `, trimmé.
    pub title: String,
    /// Les octets entre la fin de la ligne de titre et le début du bloc
    /// suivant (ou la fin du fichier), **verbatim**.
    pub body: String,
    /// Position du bloc dans la source — utile pour la substitution du
    /// design lors de la promotion.
    pub byte_range: Range<usize>,
    /// Ligne du titre (1-indexée) — utile pour signaler une ambiguïté.
    pub line: u32,
}

/// Extrait tous les blocs `### Décision : <titre>` sous la section
/// `## Décisions` du source.
///
/// Retourne une liste vide si :
/// - le source ne contient pas `## Décisions` ;
/// - la section `## Décisions` existe mais ne porte aucun bloc `###
///   Décision : ...`.
///
/// Les blocs sont retournés dans leur ordre d'apparition.
pub fn extract_decision_blocks(source: &str) -> Vec<DecisionBlock> {
    let Some((section_start, section_end)) = find_decisions_section(source) else {
        return Vec::new();
    };

    let section = &source[section_start..section_end];
    let base_offset = section_start;

    // Étape 1 : collecter les positions (relatives à `section`) des lignes
    // `### Décision : <titre>`.
    let mut heads: Vec<(usize, String, u32)> = Vec::new();
    let mut rel = 0usize;
    for line in section.split_inclusive('\n') {
        let trimmed = line.trim_end_matches(['\n', '\r']);
        if let Some(after) = trimmed.strip_prefix(H3_DECISION_PREFIX) {
            let title = after.trim().to_string();
            let abs_line = line_at_byte(source, base_offset + rel);
            heads.push((rel, title, abs_line));
        }
        rel += line.len();
    }

    // Étape 2 : construire les blocs — un bloc s'étend de la ligne de
    // titre jusqu'au début de la ligne suivante marquée `### ` ou `## `
    // (ou la fin de la section).
    let mut blocks = Vec::with_capacity(heads.len());
    for (i, (head_rel, title, line)) in heads.iter().enumerate() {
        let block_start = *head_rel;
        let block_end = heads.get(i + 1).map(|(next, _, _)| *next).unwrap_or(section.len());

        // Fin de la ligne de titre = premier `\n` après `head_rel`.
        let title_line_end = section[*head_rel..]
            .find('\n')
            .map(|off| head_rel + off + 1)
            .unwrap_or(section.len());
        // Corps = octets entre fin de la ligne de titre et début du bloc suivant.
        let body = section[title_line_end..block_end].to_string();

        blocks.push(DecisionBlock {
            title: title.clone(),
            body,
            byte_range: (base_offset + block_start)..(base_offset + block_end),
            line: *line,
        });
    }

    blocks
}

/// Repère la portée de la section `## Décisions` : de la ligne du titre
/// (inclus) jusqu'à la ligne du prochain `## ` (exclu) ou la fin du
/// fichier.
///
/// Retourne `None` si la section est absente.
fn find_decisions_section(source: &str) -> Option<(usize, usize)> {
    let mut cursor = 0usize;
    let mut section_start: Option<usize> = None;
    let mut section_end: usize = source.len();
    for line in source.split_inclusive('\n') {
        let trimmed = line.trim_end_matches(['\n', '\r']);
        match section_start {
            None => {
                if trimmed == H2_DECISIONS {
                    section_start = Some(cursor);
                }
            }
            Some(_) => {
                // Une nouvelle section H2 clôt la section Décisions.
                if trimmed.starts_with("## ") && !trimmed.starts_with(H3_DECISION_PREFIX) {
                    section_end = cursor;
                    break;
                }
            }
        }
        cursor += line.len();
    }
    section_start.map(|start| (start, section_end))
}

fn line_at_byte(source: &str, byte: usize) -> u32 {
    let clamped = byte.min(source.len());
    1 + source[..clamped].bytes().filter(|&b| b == b'\n').count() as u32
}

/// Erreurs de recherche par titre — codes stables portés côté action.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LookupError {
    NotFound,
    Ambiguous { lines: Vec<u32> },
}

/// Cherche un bloc par son titre exact (après trim). Retourne l'index
/// dans la slice, ou une erreur.
pub fn find_decision_block(blocks: &[DecisionBlock], title: &str) -> Result<usize, LookupError> {
    let matches: Vec<usize> = blocks
        .iter()
        .enumerate()
        .filter(|(_, b)| b.title == title)
        .map(|(i, _)| i)
        .collect();
    match matches.as_slice() {
        [] => Err(LookupError::NotFound),
        [only] => Ok(*only),
        many => Err(LookupError::Ambiguous {
            lines: many.iter().map(|i| blocks[*i].line).collect(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ─────────────── extract_decision_blocks ───────────────

    #[test]
    fn source_sans_section_decisions_donne_liste_vide() {
        let source = "# Design\n\n## Contexte\n\nx\n\n## Autre\n\ny\n";
        assert!(extract_decision_blocks(source).is_empty());
    }

    #[test]
    fn section_vide_donne_liste_vide() {
        let source = "# Design\n\n## Décisions\n\n## Après\n";
        assert!(extract_decision_blocks(source).is_empty());
    }

    #[test]
    fn un_bloc_extrait_titre_et_corps() {
        let source =
            "# Design\n\n## Décisions\n\n### Décision : Utiliser JWT\n\nLe rationale.\n\nDeuxième paragraphe.\n\n## Après\n\nz\n";
        let blocks = extract_decision_blocks(source);
        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].title, "Utiliser JWT");
        assert_eq!(blocks[0].body, "\nLe rationale.\n\nDeuxième paragraphe.\n\n");
    }

    #[test]
    fn deux_blocs_sont_extraits_dans_lordre() {
        let source =
            "## Décisions\n\n### Décision : Alpha\n\nA1\n\n### Décision : Beta\n\nB1\n\n## Fin\n";
        let blocks = extract_decision_blocks(source);
        assert_eq!(blocks.len(), 2);
        assert_eq!(blocks[0].title, "Alpha");
        assert_eq!(blocks[1].title, "Beta");
        assert_eq!(blocks[0].body, "\nA1\n\n");
        assert_eq!(blocks[1].body, "\nB1\n\n");
    }

    #[test]
    fn corps_est_verbatim_meme_avec_markdown_riche() {
        // Tableau, code fence, italique — tout reste dans le corps.
        let source =
            "## Décisions\n\n### Décision : Table\n\n| A | B |\n|---|---|\n| 1 | 2 |\n\n```\ncode\n```\n\n*emphase*\n\n## Après\n";
        let blocks = extract_decision_blocks(source);
        assert_eq!(blocks.len(), 1);
        assert!(blocks[0].body.contains("| A | B |"));
        assert!(blocks[0].body.contains("```\ncode\n```"));
        assert!(blocks[0].body.contains("*emphase*"));
    }

    #[test]
    fn byte_range_permet_la_substitution() {
        let source =
            "AVANT\n## Décisions\n\n### Décision : X\n\nCorps X.\n\n### Décision : Y\n\nCorps Y.\n";
        let blocks = extract_decision_blocks(source);
        assert_eq!(blocks.len(), 2);
        // Extraire par byte_range doit reproduire chaque bloc entier
        // (ligne de titre + corps).
        let bloc0 = &source[blocks[0].byte_range.clone()];
        assert!(bloc0.starts_with("### Décision : X\n"));
        assert!(bloc0.ends_with("Corps X.\n\n"));
        let bloc1 = &source[blocks[1].byte_range.clone()];
        assert!(bloc1.starts_with("### Décision : Y\n"));
        assert!(bloc1.ends_with("Corps Y.\n"));
    }

    #[test]
    fn ligne_du_titre_est_reportee() {
        let source = "L1\nL2\n## Décisions\n\n### Décision : X\n\ncorps\n";
        let blocks = extract_decision_blocks(source);
        // Titre est ligne 5 (L1, L2, blank-after-L2? no just 2 lignes puis ##
        // Décisions puis blank puis ###).
        assert_eq!(blocks[0].line, 5);
    }

    #[test]
    fn titre_est_trime() {
        let source = "## Décisions\n\n### Décision :    Avec espaces    \n\ncorps\n";
        let blocks = extract_decision_blocks(source);
        assert_eq!(blocks[0].title, "Avec espaces");
    }

    // ─────────────── find_decision_block ───────────────

    #[test]
    fn lookup_trouve_par_titre_exact() {
        let source =
            "## Décisions\n\n### Décision : Alpha\n\nA\n\n### Décision : Beta\n\nB\n";
        let blocks = extract_decision_blocks(source);
        assert_eq!(find_decision_block(&blocks, "Beta").unwrap(), 1);
    }

    #[test]
    fn lookup_absent_retourne_not_found() {
        let source = "## Décisions\n\n### Décision : Alpha\n\nA\n";
        let blocks = extract_decision_blocks(source);
        assert_eq!(
            find_decision_block(&blocks, "Fantome").unwrap_err(),
            LookupError::NotFound
        );
    }

    #[test]
    fn lookup_ambigu_retourne_positions() {
        let source =
            "## Décisions\n\n### Décision : X\n\nv1\n\n### Décision : X\n\nv2\n";
        let blocks = extract_decision_blocks(source);
        let err = find_decision_block(&blocks, "X").unwrap_err();
        match err {
            LookupError::Ambiguous { lines } => {
                assert_eq!(lines.len(), 2);
            }
            other => panic!("attendu Ambiguous, reçu {:?}", other),
        }
    }
}
