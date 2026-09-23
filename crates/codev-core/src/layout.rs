use std::path::{Path, PathBuf};

use crate::id::ChangeId;

/// Nom du dossier de planification.
///
/// Constante unique du projet : en changer coûte une recompilation, pas un
/// refactor. Le préfixe `_` le garde **visible** — les outils de recherche
/// (`ripgrep`, `fd`, et donc ceux de Claude Code) ignorent les dossiers cachés
/// par défaut, et une source de vérité que l'agent ne trouve pas ne sert à
/// rien. Voir `_codev/decisions/0003-racine-de-planification-_codev.md`.
pub const PLANNING_DIR: &str = "_codev";

pub const CONFIG_FILE: &str = "config.yaml";
pub const LOCK_FILE: &str = "codev.lock";
pub const CHANGE_METADATA_FILE: &str = "change.yaml";

const SPECS_DIR: &str = "specs";
const DECISIONS_DIR: &str = "decisions";
const DECISIONS_SEAL_FILE: &str = "seal.yaml";
const CHANGES_DIR: &str = "changes";
const ARCHIVE_DIR: &str = "archive";
const SCHEMAS_DIR: &str = "schemas";

/// Toutes les questions « où vit tel fichier ? », en un seul endroit.
///
/// De l'algèbre de chemins, donc pur : aucune de ces méthodes ne touche au
/// disque, et aucune ne vérifie l'existence de ce qu'elle nomme.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Layout {
    project_root: PathBuf,
}

impl Layout {
    /// `project_root` est le dossier **contenant** `_codev/`, pas `_codev/`
    /// lui-même.
    pub fn new(project_root: impl Into<PathBuf>) -> Self {
        Self {
            project_root: project_root.into(),
        }
    }

    pub fn project_root(&self) -> &Path {
        &self.project_root
    }

    pub fn planning_dir(&self) -> PathBuf {
        self.project_root.join(PLANNING_DIR)
    }

    pub fn config_file(&self) -> PathBuf {
        self.planning_dir().join(CONFIG_FILE)
    }

    pub fn lock_file(&self) -> PathBuf {
        self.planning_dir().join(LOCK_FILE)
    }

    pub fn specs_dir(&self) -> PathBuf {
        self.planning_dir().join(SPECS_DIR)
    }

    pub fn decisions_dir(&self) -> PathBuf {
        self.planning_dir().join(DECISIONS_DIR)
    }

    /// Fichier de sceau qui atteste du corps des ADR locaux au moment de
    /// leur acceptation — voir `codev-core::decisions::seal`.
    pub fn decisions_seal_file(&self) -> PathBuf {
        self.decisions_dir().join(DECISIONS_SEAL_FILE)
    }

    pub fn changes_dir(&self) -> PathBuf {
        self.planning_dir().join(CHANGES_DIR)
    }

    pub fn archive_dir(&self) -> PathBuf {
        self.changes_dir().join(ARCHIVE_DIR)
    }

    pub fn schemas_dir(&self) -> PathBuf {
        self.planning_dir().join(SCHEMAS_DIR)
    }

    pub fn change_dir(&self, change: &ChangeId) -> PathBuf {
        self.changes_dir().join(change.as_str())
    }

    pub fn change_metadata(&self, change: &ChangeId) -> PathBuf {
        self.change_dir(change).join(CHANGE_METADATA_FILE)
    }

    /// Le dossier d'un change archivé, préfixé par sa date pour un classement
    /// chronologique.
    pub fn archived_change_dir(&self, change: &ChangeId, date: &str) -> PathBuf {
        self.archive_dir().join(format!("{date}-{change}"))
    }

    /// Le dossier d'un schéma propre au projet.
    pub fn project_schema_dir(&self, name: &str) -> PathBuf {
        self.schemas_dir().join(name)
    }

    /// La spec principale d'une capacité, `<chemin>` étant relatif à `specs/`.
    pub fn spec_file(&self, capability_path: &str) -> PathBuf {
        self.specs_dir().join(capability_path).join("spec.md")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compose_les_chemins_depuis_la_racine_du_projet() {
        let layout = Layout::new("/tmp/projet");
        let change = ChangeId::parse("add-auth").unwrap();

        assert_eq!(layout.planning_dir(), Path::new("/tmp/projet/_codev"));
        assert_eq!(
            layout.config_file(),
            Path::new("/tmp/projet/_codev/config.yaml")
        );
        assert_eq!(
            layout.change_dir(&change),
            Path::new("/tmp/projet/_codev/changes/add-auth")
        );
        assert_eq!(
            layout.change_metadata(&change),
            Path::new("/tmp/projet/_codev/changes/add-auth/change.yaml")
        );
        assert_eq!(
            layout.archived_change_dir(&change, "2026-09-08"),
            Path::new("/tmp/projet/_codev/changes/archive/2026-09-08-add-auth")
        );
        assert_eq!(
            layout.spec_file("identity/user-auth"),
            Path::new("/tmp/projet/_codev/specs/identity/user-auth/spec.md")
        );
    }
}
