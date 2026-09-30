//! Everything that needs the outside world: on-disk layout, configuration,
//! schema resolution, plan execution.
//!
//! Effects go through the ports in [`ports`] — `FileSystem`, `Clock`,
//! `Env` — and never directly. This is not about "being able to swap the
//! file system", but about making `init`, `new change` and `archive`
//! testable in memory.
//!
//! This crate does not plan skills: `codev-agents` is the one that knows what
//! Claude Code expects. The CLI combines both plans and executes them in a
//! single pass.

pub mod apply;
pub mod archive;
pub mod change;
pub mod config;
pub mod decisions;
pub mod decisions_actions;
pub mod design;
pub mod detect;
pub mod error;
pub mod instructions;
pub mod metadata;
pub mod ports;
pub mod root;
pub mod scaffold;
pub mod schemas;
pub mod sources;
pub mod specs;
pub mod sync;
pub mod validate;

pub use error::{EngineError, Result, Warning};
pub use ports::{
    Clock, Env, FileSystem, ProcessOutput, ProcessRunner, RealFileSystem, RealProcessRunner,
    SystemClock, SystemEnv,
};
