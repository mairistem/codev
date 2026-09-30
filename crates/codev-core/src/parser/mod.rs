//! Parser for main specs and deltas.
//!
//! No I/O — this is pure core in the sense of decision
//! `_codev/decisions/0001-functional-core-imperative-shell.md`. Takes a
//! `&str`, returns a [`Parsed`] with the value — even partial — and the list
//! of located defects.
//!
//! Two distinct public entry points rather than a single one that would
//! guess, so that "wrong mix" does not compile: `parse_spec` for a main
//! spec, `parse_delta` for a change file.
pub mod ast;
pub mod codes;
pub mod delta;
pub mod fence;
mod shared;
pub mod spec;

pub use ast::{
    Delta, DeltaOp, DeltaSection, Finding, Parsed, PurposeBlock, Removal, Rename, Requirement,
    Scenario, Severity, Span, Spec,
};
pub use delta::parse_delta;
pub use spec::parse_spec;
