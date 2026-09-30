//! codev's pure core: domain model, schemas, artifact graph, plans.
//!
//! **Invariant of this crate: no input/output.** No `std::fs`, no
//! `std::env`, no clock, no network. Anything that needs the outside world
//! lives in `codev-engine`, behind a port.
//!
//! This is not discipline for its own sake: it is what makes spec merging
//! and the JSON contract testable with golden tests, without a temporary
//! directory. See `_codev/decisions/0001-functional-core-imperative-shell.md`.
//!
//! Note also what this crate does **not** do: it does not derive `Serialize`
//! on its domain types. The JSON output is a public API consumed by
//! already-installed skills; it has its own types in
//! `codev-cli::contract`, so that an internal refactor cannot break it.

pub mod config;
pub mod decisions;
pub mod detect;
pub mod error;
pub mod graph;
pub mod id;
pub mod layout;
pub mod merge;
pub mod outputs;
pub mod parser;
pub mod plan;
pub mod schema;
pub mod status;
pub mod validate;

pub use error::{CoreError, Result};
pub use graph::ArtifactGraph;
pub use id::ChangeId;
pub use layout::Layout;
pub use plan::{FileWrite, Move, Plan, WriteMode};
pub use schema::{Apply, Artifact, Schema};
pub use status::{ArtifactState, ArtifactStatus, ChangeStatus};
