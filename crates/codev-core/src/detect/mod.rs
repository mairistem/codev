//! Détection de l'environnement projet — parties pures.
//!
//! Ce module reçoit des `&[u8]` (contenus de manifestes lus par la coquille)
//! et retourne un rapport typé. Il n'ouvre aucun fichier, ne lance aucun
//! processus. L'orchestration qui interroge le système de fichiers vit
//! côté `codev-engine::detect`.

pub mod license;
pub mod mcp;
pub mod stack;

/// Ce que la sonde a extrait d'un projet.
///
/// Chaque champ est optionnel : la détection est best-effort — un manifeste
/// absent ou illisible produit `None`, jamais une valeur inventée. Un projet
/// sans manifeste connu produit un `Detected` majoritairement vide, ce n'est
/// pas une erreur.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Detected {
    pub stack: Option<stack::Stack>,
    pub project_name: Option<String>,
    pub license: Option<String>,
    pub has_ci: bool,
    pub is_git_repo: bool,
    pub mcps: Vec<mcp::DetectedMcp>,
}

impl Detected {
    /// Un rapport totalement vide — pour les tests, et comme point de départ
    /// de l'orchestrateur qui l'enrichit champ par champ.
    pub fn empty() -> Self {
        Self::default()
    }
}
