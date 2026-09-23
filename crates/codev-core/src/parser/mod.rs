//! Parseur de specs principales et de deltas.
//!
//! Aucune I/O — c'est du cœur pur au sens de la décision
//! `_codev/decisions/0001-coeur-fonctionnel-coquille-imperative.md`. Prend une
//! `&str`, rend un [`Parsed`] avec la valeur — même partielle — et la liste
//! des défauts localisés.
//!
//! Deux entrées publiques distinctes plutôt qu'une seule qui devinerait, pour
//! que « mauvais mélange » ne compile pas : `parse_spec` pour une spec
//! principale, `parse_delta` pour un fichier de change.

pub mod ast;
pub mod codes;
pub mod delta;
pub mod fence;
pub mod spec;
mod shared;

pub use ast::{
    Delta, DeltaOp, DeltaSection, Finding, Parsed, PurposeBlock, Removal, Rename, Requirement,
    Scenario, Severity, Span, Spec,
};
pub use delta::parse_delta;
pub use spec::parse_spec;
