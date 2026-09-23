//! Reconnaissance des zones littérales : blocs de code ` ``` ` et `~~~`, plus
//! commentaires HTML `<!-- … -->`.
//!
//! La logique vit ici, et non dans chaque parseur, pour la même raison qu'en
//! amont chez OpenSpec : trois parseurs indépendants qui la ré-implémentent
//! dérivent et finissent par accepter des faux positifs — un
//! `### Requirement:` à l'intérieur d'un bloc de code pris pour de vrai.
//! Le drame silencieux qu'on refuse.

/// Un masque par ligne : `true` si la ligne appartient à une zone littérale
/// (fence d'ouverture, fermeture, contenu ; ligne d'un commentaire HTML
/// multi-lignes).
///
/// Les parseurs de spec et de delta consultent ce masque avant de reconnaître
/// un en-tête. C'est ce qui rend une exigence à l'intérieur d'un bloc de code
/// invisible.
pub fn build_fence_mask(source: &str) -> Vec<bool> {
    let lines: Vec<&str> = source.split('\n').collect();
    let mut mask = vec![false; lines.len()];

    let mut active_fence: Option<ActiveFence> = None;
    let mut html_comment_open = false;

    for (i, line) in lines.iter().enumerate() {
        // À l'intérieur d'une fence : tout est littéral, y compris un éventuel
        // début de commentaire HTML — d'où le traitement de la fence en
        // premier.
        if let Some(fence) = &active_fence {
            mask[i] = true;
            if is_closing_fence(line, fence) {
                active_fence = None;
            }
            continue;
        }

        if let Some(fence) = opening_fence(line) {
            mask[i] = true;
            active_fence = Some(fence);
            continue;
        }

        // Hors fence : traiter les commentaires HTML. Un commentaire peut
        // s'étendre sur plusieurs lignes ; tant qu'il n'est pas refermé, les
        // lignes suivantes sont masquées.
        if html_comment_open {
            mask[i] = true;
            if line.contains("-->") {
                html_comment_open = false;
            }
            continue;
        }

        // Détection sur cette ligne. Un `<!--` suivi d'un `-->` **sur la même
        // ligne** est un commentaire clos qui masque quand même cette ligne,
        // sans ouvrir de fenêtre pour les suivantes.
        if let Some(start) = line.find("<!--") {
            let after = &line[start + 4..];
            if after.contains("-->") {
                mask[i] = true;
            } else {
                mask[i] = true;
                html_comment_open = true;
            }
        }
    }

    mask
}

#[derive(Debug, Clone, Copy)]
struct ActiveFence {
    marker: u8, // b'`' ou b'~'
    length: usize,
}

/// Reconnaît l'ouverture d'une fence : au moins trois `\`` ou `~`, optionnels
/// blancs devant, un « info string » optionnel (`rust`, `text`, …) après.
fn opening_fence(line: &str) -> Option<ActiveFence> {
    let trimmed_left = line.trim_start();
    if trimmed_left.is_empty() {
        return None;
    }
    let first = trimmed_left.as_bytes()[0];
    if first != b'`' && first != b'~' {
        return None;
    }
    let length = trimmed_left
        .bytes()
        .take_while(|&b| b == first)
        .count();
    if length < 3 {
        return None;
    }
    Some(ActiveFence {
        marker: first,
        length,
    })
}

/// Reconnaît la fermeture d'une fence : le même marqueur, au moins aussi long,
/// éventuellement suivi de blancs — mais rien d'autre. C'est la règle
/// CommonMark, et l'écart chez OpenSpec.
fn is_closing_fence(line: &str, fence: &ActiveFence) -> bool {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return false;
    }
    let bytes = trimmed.as_bytes();
    let run = bytes.iter().take_while(|&&b| b == fence.marker).count();
    run >= fence.length && bytes[run..].iter().all(|b| b.is_ascii_whitespace())
}

#[cfg(test)]
mod tests {
    use super::*;

    // `source.split('\n')` produit une ligne vide finale quand le source se
    // termine par `\n`. Le masque doit donc porter une valeur pour cette
    // ligne aussi. Un helper centralise la vérification, sinon chaque test
    // recompte les `\n` à la main et se trompe.
    fn check(source: &str, expected: Vec<bool>) {
        let observed = build_fence_mask(source);
        assert_eq!(
            observed.len(),
            source.split('\n').count(),
            "le masque doit avoir la même longueur que split('\\n')",
        );
        assert_eq!(observed, expected);
    }

    #[test]
    fn mask_reconnait_backticks_et_tildes() {
        let source = "```\ninside\n```\n\ntop\n~~~\nalso inside\n~~~\n";
        check(
            source,
            vec![true, true, true, false, false, true, true, true, false],
        );
    }

    #[test]
    fn mask_refuse_une_fermeture_de_marqueur_different() {
        let source = "```\nfoo\n~~~\nbar\n```\nout\n";
        check(source, vec![true, true, true, true, true, false, false]);
    }

    #[test]
    fn mask_traite_deux_blocs_successifs() {
        let source = "```\na\n```\n```\nb\n```\n";
        check(source, vec![true, true, true, true, true, true, false]);
    }

    #[test]
    fn mask_ignore_une_fermeture_trop_courte() {
        // Ouverture ````, fermeture ``` : la fermeture doit être au moins
        // aussi longue que l'ouverture.
        let source = "````\nx\n```\ny\n````\n";
        check(source, vec![true, true, true, true, true, false]);
    }

    #[test]
    fn commentaire_multi_lignes_est_masque() {
        let source =
            "avant\n<!-- début du commentaire\n### Requirement: Faux\nfin du commentaire -->\napres\n";
        check(source, vec![false, true, true, true, false, false]);
    }

    #[test]
    fn commentaire_sur_une_seule_ligne_est_masque_sans_ouvrir_de_fenetre() {
        let source = "avant\n<!-- inline -->\napres\n### Requirement: Vrai\n";
        check(source, vec![false, true, false, false, false]);
    }

    #[test]
    fn commentaire_dans_une_fence_reste_litteral_via_la_fence() {
        let source = "```\n<!-- pas un vrai commentaire\n```\napres\n";
        check(source, vec![true, true, true, false, false]);
    }
}
