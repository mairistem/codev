use globset::{Glob, GlobSetBuilder};

use crate::error::{CoreError, Result};

/// Vrai si l'un des chemins fournis satisfait le motif de sortie d'un artefact.
///
/// `relative_paths` est relatif au dossier du change, avec des séparateurs `/`.
/// Le parcours du disque est un effet : il appartient à l'appelant, dans
/// `codev-engine`. Ici, seule la mise en correspondance — donc testable sans
/// aucun fichier.
pub fn pattern_matches_any(pattern: &str, relative_paths: &[String]) -> Result<bool> {
    let glob = Glob::new(pattern).map_err(|e| CoreError::InvalidOutputPattern {
        pattern: pattern.to_string(),
        reason: e.to_string(),
    })?;
    let set = GlobSetBuilder::new()
        .add(glob)
        .build()
        .map_err(|e| CoreError::InvalidOutputPattern {
            pattern: pattern.to_string(),
            reason: e.to_string(),
        })?;
    Ok(relative_paths.iter().any(|p| set.is_match(p)))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn paths(items: &[&str]) -> Vec<String> {
        items.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn reconnait_une_spec_imbriquee() {
        let presents = paths(&["proposal.md", "specs/identity/user-auth/spec.md"]);
        assert!(pattern_matches_any("specs/**/*.md", &presents).unwrap());
    }

    #[test]
    fn ne_confond_pas_un_dossier_vide_avec_une_sortie() {
        let presents = paths(&["proposal.md"]);
        assert!(!pattern_matches_any("specs/**/*.md", &presents).unwrap());
    }

    #[test]
    fn ignore_un_fichier_hors_motif() {
        // Un `README.md` déposé à la racine du change ne doit pas faire croire
        // que les specs ont été écrites.
        let presents = paths(&["README.md", "specs/notes.txt"]);
        assert!(!pattern_matches_any("specs/**/*.md", &presents).unwrap());
    }

    #[test]
    fn refuse_un_motif_illisible() {
        let err = pattern_matches_any("specs/[", &paths(&["a.md"])).unwrap_err();
        assert_eq!(err.code(), "invalid_output_pattern");
    }
}
