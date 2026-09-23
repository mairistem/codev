//! Cœur pur de codev : modèle de domaine, schémas, graphe d'artefacts, plans.
//!
//! **Invariant de ce crate : aucune entrée-sortie.** Pas de `std::fs`, pas de
//! `std::env`, pas d'horloge, pas de réseau. Ce qui a besoin du monde extérieur
//! vit dans `codev-engine`, derrière un port.
//!
//! Ce n'est pas de la discipline gratuite : c'est ce qui rend la fusion de
//! specs et le contrat JSON testables par golden tests, sans répertoire
//! temporaire. Voir `_codev/decisions/0001-coeur-fonctionnel-coquille-imperative.md`.
//!
//! Noter aussi ce que ce crate ne fait **pas** : il ne dérive pas `Serialize`
//! sur ses types de domaine. La sortie JSON est une API publique consommée par
//! des skills déjà installées ; elle a ses propres types dans
//! `codev-cli::contract`, pour qu'un refactor interne ne la casse pas.

pub mod decisions;
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
