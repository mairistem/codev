//! Architecture decisions — pure parser.
//!
//! No I/O — the index and the merging of inherited sources live in
//! `codev-engine::decisions`. Follows decisions
//! [0001](../../../_codev/decisions/0001-coeur-fonctionnel-coquille-imperative.md)
//! and
//! [0002](../../../_codev/decisions/0002-graphe-de-crates-comme-regle-de-dependance.md).

pub mod ast;
pub mod parser;
pub mod seal;

pub use ast::{Decision, DecisionStatus, Section};
pub use parser::parse_decision;
pub use seal::{
    SEAL_FILE_VERSION, Seal, SealError, SealFile, VerificationCase, body_hash, body_slice,
    parse_seal_file, plan_seal_force, plan_seal_new, render_seal_file, verify,
};
