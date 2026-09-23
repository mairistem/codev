use std::io;
use std::path::PathBuf;

use codev_core::CoreError;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, EngineError>;

#[derive(Debug, Error)]
pub enum EngineError {
    #[error("aucun projet codev trouvé depuis {from} — lance `codev init` à la racine du projet")]
    NoRoot { from: PathBuf },

    #[error("le change « {change} » n'existe pas — `codev list` montre ceux qui existent")]
    UnknownChange { change: String },

    #[error("le change « {change} » existe déjà")]
    ChangeExists { change: String },

    #[error("schéma « {name} » introuvable — `codev schemas` liste ceux qui sont disponibles")]
    SchemaNotFound { name: String },

    #[error(
        "aucun artefact prêt pour le change « {change} » — `codev status --change {change}` dit pourquoi"
    )]
    NoArtifactReady { change: String },

    #[error("template « {template} » introuvable pour l'artefact « {artifact} »")]
    TemplateNotFound { artifact: String, template: String },

    #[error("{path} est illisible : {reason}")]
    Unreadable { path: PathBuf, reason: String },

    #[error("{path} est invalide : {reason}")]
    Invalid { path: PathBuf, reason: String },

    #[error("erreur d'écriture sur {path} : {source}")]
    Write {
        path: PathBuf,
        #[source]
        source: io::Error,
    },

    #[error(transparent)]
    Core(#[from] CoreError),
}

impl EngineError {
    /// Code stable, exposé dans le tableau `status[]` du contrat JSON.
    ///
    /// Le message est libre de changer de formulation ; le code, non — un
    /// consommateur peut s'y fier.
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
            Self::Write { .. } => "write_failed",
            Self::Core(inner) => inner.code(),
        }
    }
}

/// Un constat non bloquant, remonté à l'utilisateur sans faire échouer la
/// commande.
///
/// Une source héritée introuvable en est l'exemple type : la commande doit
/// aboutir avec ce qu'elle a, en disant clairement ce qui manque et comment le
/// réparer.
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
