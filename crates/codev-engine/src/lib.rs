//! Ce qui a besoin du monde extérieur : disposition disque, configuration,
//! résolution de schémas, exécution des plans.
//!
//! Les effets passent par les ports de [`ports`] — `FileSystem`, `Clock`,
//! `Env` — et jamais directement. Ce n'est pas pour « pouvoir changer de
//! système de fichiers », mais pour que `init`, `new change` et `archive`
//! soient testables en mémoire.
//!
//! Ce crate ne planifie pas les skills : c'est `codev-agents` qui sait ce
//! qu'attend Claude Code. Le CLI réunit les deux plans et les exécute en une
//! passe.

pub mod apply;
pub mod archive;
pub mod change;
pub mod config;
pub mod decisions;
pub mod decisions_actions;
pub mod design;
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
