//! Règles de validation, appliquées à un AST déjà parsé.
//!
//! Deux temps, comme partout : décider n'est pas exécuter. Ici on décide, à
//! partir de la seule structure du contenu — aucune I/O. La coordination
//! (marche du disque, groupement, rapport) vit dans `codev-engine::validate`.
//!
//! Une règle est une implémentation de [`Rule`] enregistrée dans [`RULES`].
//! Ajouter une règle E4 (warnings du lot 2) sera une struct de plus dans le
//! registre, sans toucher aux appelants.

use crate::parser::ast::{Delta, Finding, Spec};

pub mod codes;
pub mod rules;

/// Contrat d'une règle : elle sait dire son `code` stable et se déclencher au
/// choix sur une spec principale ou un delta.
///
/// Les deux méthodes de `check_*` sont optionnelles pour qu'une règle
/// spécialisée (par exemple `SpecNoRequirement`) ne se dérange pas sur le
/// type qu'elle n'intéresse pas.
pub trait Rule: Sync {
    fn code(&self) -> &'static str;

    fn check_spec(&self, _spec: &Spec) -> Vec<Finding> {
        Vec::new()
    }

    fn check_delta(&self, _delta: &Delta) -> Vec<Finding> {
        Vec::new()
    }
}

/// Le catalogue des règles jouées par `codev validate`.
///
/// L'ordre est signifiant : les findings d'un même fichier apparaissent
/// dans cet ordre, ce qui rend la sortie stable et donc testable par
/// snapshot. Insérer une règle au milieu n'est pas une décision anodine —
/// c'est une modification du contrat visible.
pub const RULES: &[&dyn Rule] = &[
    &rules::RequirementNoShall,
    &rules::RequirementNoScenario,
    &rules::SpecNoRequirement,
    &rules::CrossSectionConflict,
    &rules::RenameTargetCollision,
    &rules::ModifiedUsesOldName,
];

/// Applique toutes les règles à une spec principale et concatène leurs
/// `Finding`.
pub fn check_spec(spec: &Spec) -> Vec<Finding> {
    RULES.iter().flat_map(|r| r.check_spec(spec)).collect()
}

/// Applique toutes les règles à un delta et concatène leurs `Finding`.
pub fn check_delta(delta: &Delta) -> Vec<Finding> {
    RULES.iter().flat_map(|r| r.check_delta(delta)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn registry_liste_au_moins_une_regle() {
        // Le catalogue doit contenir les 6 règles annoncées ; si l'une
        // disparaît par erreur, la commande `validate` couvrirait moins que
        // ce que la spec promet.
        assert_eq!(RULES.len(), 6);
    }

    #[test]
    fn codes_de_findings_sont_uniques() {
        // Le contrat public : chaque `code` identifie UN défaut. Une
        // collision entre parseur et validate — ou entre deux règles —
        // ferait qu'un consommateur qui teste sur `code` traite deux cas
        // pour un.
        let mut vus: BTreeSet<&'static str> = BTreeSet::new();
        for code in crate::parser::codes::ALL {
            assert!(
                vus.insert(code),
                "code du parseur en doublon : {code}"
            );
        }
        for rule in RULES {
            assert!(
                vus.insert(rule.code()),
                "code de règle en doublon avec ce qui précède : {}",
                rule.code()
            );
        }
    }
}
