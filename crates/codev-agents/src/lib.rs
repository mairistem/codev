//! La génération des skills : le seul crate qui sait quelque chose d'un outil
//! d'agent.
//!
//! C'est la raison d'être du projet. Le reste de codev gère des fichiers
//! markdown ; ici on rend le workflow atteignable depuis le chat de Claude
//! Code, en écrivant les skills qu'il découvre au démarrage.
//!
//! Une seule cible existe, et c'est assumé — mais elle passe par
//! [`AgentTarget`], pour qu'en ajouter une reste un fichier de plus.

pub mod claude;
pub mod target;
pub mod workflows;

pub use claude::ClaudeCode;
pub use target::{AgentTarget, SkillsPlan};
pub use workflows::{Workflow, CATALOG, DEFAULT_WORKFLOWS};
