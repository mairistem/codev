//! Types du domaine « décision d'architecture », alimentés par le parseur.
//!
//! Une décision est vue comme un frontmatter typé plus un corps markdown ;
//! le corps est découpé en sections `## …` de premier niveau — même
//! mécanique que la spec principale — pour qu'un consommateur puisse
//! afficher ce qu'il veut sans re-parser.

use crate::parser::ast::Span;

/// Statut d'une décision. Seuls `Accepted` et `Superseded` interviennent
/// dans le calcul « en vigueur » ; les autres sont exposés tels quels pour
/// que l'index reste lisible sans jamais mentir sur l'effet.
///
/// `Unknown(String)` porte la valeur brute rencontrée dans le frontmatter,
/// pour qu'un finding puisse la nommer et qu'un rendu puisse l'afficher.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecisionStatus {
    Accepted,
    Superseded,
    Proposed,
    Deprecated,
    Rejected,
    Unknown(String),
}

impl DecisionStatus {
    pub fn from_raw(raw: &str) -> Self {
        match raw {
            "accepted" => Self::Accepted,
            "superseded" => Self::Superseded,
            "proposed" => Self::Proposed,
            "deprecated" => Self::Deprecated,
            "rejected" => Self::Rejected,
            other => Self::Unknown(other.to_string()),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Accepted => "accepted",
            Self::Superseded => "superseded",
            Self::Proposed => "proposed",
            Self::Deprecated => "deprecated",
            Self::Rejected => "rejected",
            Self::Unknown(raw) => raw,
        }
    }

    /// Une décision est *candidate à être en vigueur* si son statut est
    /// `Accepted`. La supersession se joue au niveau de l'index, avec la
    /// vue d'ensemble.
    pub fn is_candidate_for_effect(&self) -> bool {
        matches!(self, Self::Accepted)
    }
}

/// Une section `## <name>` du corps de la décision.
///
/// La spec ne prescrit pas de titres particuliers — chaque équipe peut
/// choisir ses sections (Contexte / Décision / Conséquences… ou autres).
/// Ce type expose ce qui est trouvé, sans l'imposer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Section {
    pub name: String,
    pub body: String,
    pub span: Span,
}

/// Une décision parsée, à partir d'un fichier markdown à frontmatter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Decision {
    pub id: String,
    pub title: String,
    pub status: DecisionStatus,
    pub date: String,
    pub tags: Vec<String>,
    pub supersedes: Vec<String>,
    /// Dérives locales : quand cet ADR est local et référence une décision
    /// héritée (`path:` ou `git:`) dont il choisit de s'écarter. Chaque
    /// entrée est un `qualified-id` (`<origin>/<id>`), jamais un id nu.
    /// Champ additif : les ADR antérieurs à K6 sortent avec une liste vide.
    pub deviates_from: Vec<String>,
    pub sections: Vec<Section>,
    /// Position du frontmatter dans le source — utile pour un futur outil
    /// qui voudrait le réécrire (K5) sans toucher au corps.
    pub frontmatter_span: Span,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_reconnait_les_cinq_valeurs_documentees() {
        assert_eq!(DecisionStatus::from_raw("accepted"), DecisionStatus::Accepted);
        assert_eq!(DecisionStatus::from_raw("superseded"), DecisionStatus::Superseded);
        assert_eq!(DecisionStatus::from_raw("proposed"), DecisionStatus::Proposed);
        assert_eq!(DecisionStatus::from_raw("deprecated"), DecisionStatus::Deprecated);
        assert_eq!(DecisionStatus::from_raw("rejected"), DecisionStatus::Rejected);
    }

    #[test]
    fn status_inconnu_conserve_la_valeur_brute() {
        let s = DecisionStatus::from_raw("pending");
        assert_eq!(s.as_str(), "pending");
        assert!(matches!(s, DecisionStatus::Unknown(_)));
        assert!(!s.is_candidate_for_effect());
    }

    #[test]
    fn seul_accepted_est_candidat_a_effet() {
        assert!(DecisionStatus::Accepted.is_candidate_for_effect());
        for autre in [
            DecisionStatus::Superseded,
            DecisionStatus::Proposed,
            DecisionStatus::Deprecated,
            DecisionStatus::Rejected,
        ] {
            assert!(
                !autre.is_candidate_for_effect(),
                "{autre:?} ne doit pas être candidat"
            );
        }
    }

    #[test]
    fn les_types_dast_se_construisent_a_la_main() {
        let span = Span::new(0..1, 1..2);
        let decision = Decision {
            id: "0001".into(),
            title: "Test".into(),
            status: DecisionStatus::Accepted,
            date: "2026-09-08".into(),
            tags: vec!["architecture".into()],
            supersedes: vec![],
            deviates_from: vec![],
            sections: vec![Section {
                name: "Contexte".into(),
                body: "x".into(),
                span: span.clone(),
            }],
            frontmatter_span: span,
        };
        assert_eq!(decision.status.as_str(), "accepted");
        assert_eq!(decision.sections.len(), 1);
    }
}
