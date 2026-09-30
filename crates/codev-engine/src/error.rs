use std::io;
use std::path::PathBuf;

use codev_core::CoreError;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, EngineError>;

#[derive(Debug, Error)]
pub enum EngineError {
    #[error("no codev project found from {from} — run `codev init` at the project root")]
    NoRoot { from: PathBuf },

    #[error("change `{change}` does not exist — `codev list` shows the existing ones")]
    UnknownChange { change: String },

    #[error("change `{change}` already exists")]
    ChangeExists { change: String },

    #[error("schema `{name}` not found — `codev schemas` lists the available ones")]
    SchemaNotFound { name: String },

    #[error(
        "no artifact ready for change `{change}` — `codev status --change {change}` explains why"
    )]
    NoArtifactReady { change: String },

    #[error("template `{template}` not found for artifact `{artifact}`")]
    TemplateNotFound { artifact: String, template: String },

    #[error("{path} is unreadable: {reason}")]
    Unreadable { path: PathBuf, reason: String },

    #[error("{path} is invalid: {reason}")]
    Invalid { path: PathBuf, reason: String },

    /// The validation preflight of `sync` or `archive` found errors. A
    /// variant of its own, not an `Invalid`, so that the stable code a
    /// consumer tests is the code itself, not a word inside the message.
    #[error("change `{change}` has errors; run `codev validate {change}` for details")]
    ValidationFailed { change: String },

    #[error("failed to write {path}: {source}")]
    Write {
        path: PathBuf,
        #[source]
        source: io::Error,
    },

    #[error(transparent)]
    Core(#[from] CoreError),
}

impl EngineError {
    /// Stable code, exposed in the `status[]` array of the JSON contract.
    ///
    /// The message is free to change its wording; the code is not — a
    /// consumer can rely on it.
    pub fn code(&self) -> &'static str {
        match self {
            Self::NoRoot { .. } => "no_codev_root",
            Self::UnknownChange { .. } => "unknown_change",
            Self::ChangeExists { .. } => "change_exists",
            Self::SchemaNotFound { .. } => "schema_not_found",
            Self::NoArtifactReady { .. } => "no_artifact_ready",
            Self::TemplateNotFound { .. } => "template_not_found",
            Self::Unreadable { .. } => "unreadable",
            Self::Invalid { .. } => "invalid",
            Self::ValidationFailed { .. } => "validation_failed",
            Self::Write { .. } => "write_failed",
            Self::Core(inner) => inner.code(),
        }
    }
}

/// A non-blocking finding, reported to the user without failing the
/// command.
///
/// A missing inherited source is the typical example: the command must
/// succeed with what it has, while clearly saying what is missing and how to
/// fix it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Warning {
    pub code: &'static str,
    pub message: String,
}

impl Warning {
    pub fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}
