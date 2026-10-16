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

    /// `codev_version:` is not a version requirement. A code of its own: a
    /// team relying on it must learn that the alignment is off.
    #[error("{path} is invalid: {reason}")]
    InvalidCodevVersion { path: PathBuf, reason: String },

    /// `review.mode` is not a review mode, in `_codev/config.yaml` or a
    /// change's `change.yaml`. A code of its own, like
    /// `InvalidCodevVersion`: a skill tests the code, not the message.
    #[error("{path} is invalid: {reason}")]
    InvalidReviewMode { path: PathBuf, reason: String },

    /// The validation preflight of `sync` or `archive` found errors. A
    /// variant of its own, not an `Invalid`, so that the stable code a
    /// consumer tests is the code itself, not a word inside the message.
    #[error("change `{change}` has errors; run `codev validate {change}` for details")]
    ValidationFailed { change: String },

    /// Nothing is left of a `codev search` query once its common words are
    /// removed.
    #[error("the query holds no word to search: give words that name the behavior")]
    SearchQueryEmpty,

    /// `codev search --limit` outside the range it accepts.
    #[error("--limit must be from 1 to {max}, got {limit}")]
    SearchLimitInvalid { limit: usize, max: usize },

    /// A capability `codev show` does not know: not one of the main specs.
    #[error("no capability `{capability}` in the specs — `codev list --specs` lists them")]
    UnknownCapability { capability: String },

    /// A requirement the capability does not hold; the message lists the
    /// ones it does.
    #[error(
        "capability `{capability}` has no requirement `{requirement}`; its requirements are: {}",
        known.join(", ")
    )]
    UnknownRequirement {
        capability: String,
        requirement: String,
        known: Vec<String>,
    },

    /// The review record does not let the change be archived: missing for
    /// a step where roles were active, a `high` finding unresolved, or a
    /// format this codev cannot read. Its own code, as `ValidationFailed`.
    #[error(
        "change `{change}` cannot be archived yet: {}; run `codev review --change {change}` for details",
        reasons.join("; ")
    )]
    ReviewBlocked {
        change: String,
        reasons: Vec<String>,
    },

    #[error("failed to write {path}: {source}")]
    Write {
        path: PathBuf,
        #[source]
        source: io::Error,
    },

    /// An invalid entry of `roles:`, in the configuration or a change's
    /// `change.yaml`. The code is the finding's: `unknown_role`,
    /// `invalid_role_activation` or `role_not_disableable`.
    #[error("{message}")]
    RoleSetting { code: &'static str, message: String },

    #[error("unknown step `{step}`; the steps are {}", codev_core::roles::STEPS.join(", "))]
    UnknownStep { step: String },

    #[error("git does not know `{git_ref}`: {reason}")]
    UnknownRef { git_ref: String, reason: String },

    #[error("{root} is not in a git repository, so `--since` and `--until` cannot be resolved")]
    NotAGitRepository { root: PathBuf },

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
            Self::InvalidCodevVersion { .. } => "invalid_codev_version",
            Self::InvalidReviewMode { .. } => "invalid_review_mode",
            Self::ValidationFailed { .. } => "validation_failed",
            Self::SearchQueryEmpty => "search_query_empty",
            Self::SearchLimitInvalid { .. } => "search_limit_invalid",
            Self::UnknownCapability { .. } => "unknown_capability",
            Self::UnknownRequirement { .. } => "unknown_requirement",
            Self::ReviewBlocked { .. } => "review_blocked",
            Self::Write { .. } => "write_failed",
            Self::RoleSetting { code, .. } => code,
            Self::UnknownStep { .. } => "unknown_step",
            Self::UnknownRef { .. } => "unknown_ref",
            Self::NotAGitRepository { .. } => "not_a_git_repository",
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
