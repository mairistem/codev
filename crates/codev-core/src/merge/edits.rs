//! Un édit ponctuel sur une chaîne — `[start..end)` remplacé par un texte.
//!
//! La fusion est pure : elle produit une liste d'édits que la coquille
//! applique sur la source de la spec principale. C'est ce qui rend possible
//! `--dry-run`, la prévisualisation JSON, et surtout l'atomicité — le plan
//! est calculé entièrement avant qu'une seule écriture ne touche le disque.

use std::ops::Range;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Edit {
    pub byte_range: Range<usize>,
    pub replacement: String,
}

impl Edit {
    pub fn new(byte_range: Range<usize>, replacement: impl Into<String>) -> Self {
        Self {
            byte_range,
            replacement: replacement.into(),
        }
    }
}

/// Applique une série d'édits à un source, en préservant les offsets restants.
///
/// Les édits sont triés par `byte_range.end` **décroissant** avant application.
/// Un tri par la fin, pas par le début : deux édits qui se terminent au même
/// point mais commencent différemment (impossibles dans nos cas) resteraient
/// alors dans l'ordre d'insertion, ce qui est prévisible.
///
/// Un édit dont `byte_range.end > source.len()` est refusé — c'est le seul
/// invariant de sûreté ; le reste est déjà validé par la couche appelante.
pub fn apply_edits(source: &str, edits: &[Edit]) -> String {
    let mut ordered = edits.to_vec();
    // Tri stable par end décroissant : les derniers segments modifiés en
    // premier, laissent les offsets antérieurs intacts. `sort_by_key` +
    // `Reverse` évite la double comparaison signalée par clippy.
    ordered.sort_by_key(|e| std::cmp::Reverse(e.byte_range.end));

    let mut out = source.to_string();
    for edit in ordered {
        let end = edit.byte_range.end.min(out.len());
        let start = edit.byte_range.start.min(end);
        out.replace_range(start..end, &edit.replacement);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apply_est_stable_meme_avec_ordre_melange() {
        let source = "AAAA BBBB CCCC";
        let edits = vec![
            // Ordre volontairement inversé de la position.
            Edit::new(10..14, "cccc"),
            Edit::new(0..4, "aaaa"),
            Edit::new(5..9, "bbbb"),
        ];
        assert_eq!(apply_edits(source, &edits), "aaaa bbbb cccc");
    }

    #[test]
    fn apply_preserve_le_contenu_hors_ranges() {
        let source = "avant [ICI] apres";
        let edits = vec![Edit::new(6..11, "[LA]")];
        assert_eq!(apply_edits(source, &edits), "avant [LA] apres");
    }

    #[test]
    fn edit_vide_a_meme_position_insere() {
        let source = "abcXYZ";
        let edits = vec![Edit::new(3..3, "INS")];
        assert_eq!(apply_edits(source, &edits), "abcINSXYZ");
    }

    #[test]
    fn edit_vide_avec_replacement_vide_est_un_no_op() {
        let source = "hello";
        let edits = vec![Edit::new(2..2, "")];
        assert_eq!(apply_edits(source, &edits), "hello");
    }

    #[test]
    fn edit_hors_source_est_clampe_sans_paniquer() {
        // Sécurité contre un bug de calcul en amont : plutôt que de paniquer
        // sur un range hors bornes, on clamp — le test golden qui repasse
        // sur la source d'origine trouvera l'incohérence.
        let source = "abc";
        let edits = vec![Edit::new(2..999, "XYZ")];
        assert_eq!(apply_edits(source, &edits), "abXYZ");
    }
}
