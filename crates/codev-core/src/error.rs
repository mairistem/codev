use thiserror::Error;

pub type Result<T> = std::result::Result<T, CoreError>;

/// Erreurs du cœur.
///
/// Chaque variante porte un code stable via [`CoreError::code`]. Le code est ce
/// que le contrat JSON expose dans son tableau `status[]` : le message peut être
/// reformulé sans casser un consommateur, le code non.
#[derive(Debug, Error)]
pub enum CoreError {
    #[error("nom de change invalide « {raw} » : {reason}")]
    InvalidChangeId { raw: String, reason: String },

    #[error("schéma illisible : {0}")]
    SchemaUnreadable(String),

    #[error("schéma « {schema} » invalide : {reason}")]
    SchemaInvalid { schema: String, reason: String },

    #[error("artefact inconnu « {artifact} » dans le schéma « {schema} »")]
    UnknownArtifact { schema: String, artifact: String },

    #[error("motif de sortie invalide « {pattern} » : {reason}")]
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
