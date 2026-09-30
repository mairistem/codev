use thiserror::Error;

pub type Result<T> = std::result::Result<T, CoreError>;

/// Core errors.
///
/// Each variant carries a stable code via [`CoreError::code`]. The code is what
/// the JSON contract exposes in its `status[]` array: the message can be
/// reworded without breaking a consumer, the code cannot.
#[derive(Debug, Error)]
pub enum CoreError {
    #[error("invalid change name `{raw}`: {reason}")]
    InvalidChangeId { raw: String, reason: String },

    #[error("unreadable schema: {0}")]
    SchemaUnreadable(String),

    #[error("invalid schema `{schema}`: {reason}")]
    SchemaInvalid { schema: String, reason: String },

    #[error("unknown artifact `{artifact}` in schema `{schema}`")]
    UnknownArtifact { schema: String, artifact: String },

    #[error("invalid output pattern `{pattern}`: {reason}")]
    InvalidOutputPattern { pattern: String, reason: String },
}

impl CoreError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::InvalidChangeId { .. } => "invalid_change_id",
            Self::SchemaUnreadable(_) => "schema_unreadable",
            Self::SchemaInvalid { .. } => "schema_invalid",
            Self::UnknownArtifact { .. } => "unknown_artifact",
            Self::InvalidOutputPattern { .. } => "invalid_output_pattern",
        }
    }
}
