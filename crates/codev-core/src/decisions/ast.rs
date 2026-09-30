//! Types for the "architecture decision" domain, populated by the parser.
//!
//! A decision is seen as a typed frontmatter plus a markdown body; the
//! body is split into top-level `## …` sections — the same mechanism as
//! the main spec — so that a consumer can display whatever it wants
//! without re-parsing.

use crate::parser::ast::Span;

/// Status of a decision. Only `Accepted` and `Superseded` take part in
/// the "in effect" computation; the others are exposed as-is so the
/// index stays readable without ever misrepresenting the effect.
///
/// `Unknown(String)` carries the raw value found in the frontmatter, so
/// that a finding can name it and a renderer can display it.
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

    /// A decision is a *candidate for being in effect* if its status is
    /// `Accepted`. Supersession is resolved at the index level, with the
    /// full picture.
    pub fn is_candidate_for_effect(&self) -> bool {
        matches!(self, Self::Accepted)
    }
}

/// A `## <name>` section of the decision body.
///
/// The spec does not prescribe particular headings — each team may
/// choose its sections (Context / Decision / Consequences… or others).
/// This type exposes what is found, without enforcing it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Section {
    pub name: String,
    pub body: String,
    pub span: Span,
}

/// A parsed decision, from a markdown file with frontmatter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Decision {
    pub id: String,
    pub title: String,
    pub status: DecisionStatus,
    pub date: String,
    pub tags: Vec<String>,
    pub supersedes: Vec<String>,
    /// Local deviations: when this ADR is local and references an
    /// inherited decision (`path:` or `git:`) it chooses to depart from.
    /// Each entry is a `qualified-id` (`<origin>/<id>`), never a bare id.
    /// Additive field: ADRs written before deviations existed come out with an empty list.
    pub deviates_from: Vec<String>,
    pub sections: Vec<Section>,
    /// Position of the frontmatter in the source — useful for a future
    /// tool that would rewrite it without touching the body.
    pub frontmatter_span: Span,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_recognizes_the_five_documented_values() {
        assert_eq!(
            DecisionStatus::from_raw("accepted"),
            DecisionStatus::Accepted
        );
        assert_eq!(
            DecisionStatus::from_raw("superseded"),
            DecisionStatus::Superseded
        );
        assert_eq!(
            DecisionStatus::from_raw("proposed"),
            DecisionStatus::Proposed
        );
        assert_eq!(
            DecisionStatus::from_raw("deprecated"),
            DecisionStatus::Deprecated
        );
        assert_eq!(
            DecisionStatus::from_raw("rejected"),
            DecisionStatus::Rejected
        );
    }

    #[test]
    fn unknown_status_keeps_the_raw_value() {
        let s = DecisionStatus::from_raw("pending");
        assert_eq!(s.as_str(), "pending");
        assert!(matches!(s, DecisionStatus::Unknown(_)));
        assert!(!s.is_candidate_for_effect());
    }

    #[test]
    fn only_accepted_is_candidate_for_effect() {
        assert!(DecisionStatus::Accepted.is_candidate_for_effect());
        for other in [
            DecisionStatus::Superseded,
            DecisionStatus::Proposed,
            DecisionStatus::Deprecated,
            DecisionStatus::Rejected,
        ] {
            assert!(
                !other.is_candidate_for_effect(),
                "{other:?} must not be a candidate"
            );
        }
    }

    #[test]
    fn ast_types_can_be_built_by_hand() {
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
                name: "Context".into(),
                body: "x".into(),
                span: span.clone(),
            }],
            frontmatter_span: span,
        };
        assert_eq!(decision.status.as_str(), "accepted");
        assert_eq!(decision.sections.len(), 1);
    }
}
