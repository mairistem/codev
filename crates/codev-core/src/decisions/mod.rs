//! Décisions d'architecture — parseur pur.
//!
//! Aucune I/O — l'index et la fusion des sources héritées vivent dans
//! `codev-engine::decisions`. Suit les décisions
//! [0001](../../../_codev/decisions/0001-coeur-fonctionnel-coquille-imperative.md)
//! et
//! [0002](../../../_codev/decisions/0002-graphe-de-crates-comme-regle-de-dependance.md).

pub mod ast;
pub mod parser;
pub mod seal;

pub use ast::{Decision, DecisionStatus, Section};
pub use parser::parse_decision;
pub use seal::{
    body_hash, body_slice, parse_seal_file, plan_seal_force, plan_seal_new, render_seal_file,
    verify, Seal, SealError, SealFile, VerificationCase, SEAL_FILE_VERSION,
};
