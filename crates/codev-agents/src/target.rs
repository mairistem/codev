use std::path::{Path, PathBuf};

use codev_core::Plan;
use codev_engine::FileSystem;

use crate::workflows::Workflow;

/// Un outil d'agent capable d'accueillir les workflows de codev.
///
/// Une seule implémentation existe — Claude Code — et c'est assumé : ce trait
/// n'est pas là pour une généralité hypothétique, mais pour que l'ajout de
/// Cursor ou d'une cible `.agents/` reste un fichier de plus, sans toucher au
/// reste. C'est le seul endroit du projet qui sait quelque chose d'un outil.
///
/// Compatible `dyn` : les ports arrivent en référence dynamique pour que le CLI
/// puisse tenir une liste de cibles hétérogènes.
pub trait AgentTarget {
    fn id(&self) -> &'static str;

    fn label(&self) -> &'static str;

    /// Vrai si cet outil est visiblement utilisé dans ce projet.
    ///
    /// Sert à préremplir la sélection de `codev init`, jamais à décider seul :
    /// un projet peut vouloir des skills pour un outil qu'il n'a pas encore
    /// configuré.
    fn detect(&self, fs: &dyn FileSystem, project_root: &Path) -> bool;

    /// Planifie l'écriture des skills, sans rien écrire.
    fn plan_skills(
        &self,
        fs: &dyn FileSystem,
        project_root: &Path,
        workflows: &[&Workflow],
        version: &str,
        force: bool,
    ) -> SkillsPlan;
}

/// Le résultat d'une planification de skills.
#[derive(Debug, Default)]
pub struct SkillsPlan {
    pub plan: Plan,
    /// Les fichiers laissés en place parce qu'ils ont été édités à la main.
    ///
    /// Distinguer ce cas d'un simple « rien à faire » est ce qui permet de dire
    /// à l'utilisateur pourquoi sa skill n'a pas bougé, et comment forcer.
    pub preserved: Vec<PathBuf>,
}
