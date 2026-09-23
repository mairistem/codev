use std::path::Path;

use codev_core::Layout;

use crate::error::{EngineError, Result};
use crate::ports::{Env, FileSystem};

/// Trouve la racine du projet en remontant depuis `start`.
///
/// Une seule règle : le premier ancêtre qui contient un dossier `_codev/`
/// gagne. Pas de registre de projets, pas de variable d'environnement, pas de
/// flag `--store` : c'est le comportement de `git`, et il n'a besoin d'aucune
/// explication.
pub fn discover(fs: &dyn FileSystem, start: &Path) -> Result<Layout> {
    for ancestor in start.ancestors() {
        let layout = Layout::new(ancestor);
        if fs.exists(&layout.planning_dir()) {
            return Ok(layout);
        }
    }
    Err(EngineError::NoRoot {
        from: start.to_path_buf(),
    })
}

pub fn discover_from_cwd(fs: &dyn FileSystem, env: &dyn Env) -> Result<Layout> {
    let cwd = env.current_dir().map_err(|e| EngineError::Unreadable {
        path: ".".into(),
        reason: e.to_string(),
    })?;
    discover(fs, &cwd)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ports::MemoryFileSystem;

    #[test]
    fn remonte_jusqua_la_racine() {
        let fs = MemoryFileSystem::new().with_file("/p/_codev/config.yaml", "");
        let layout = discover(fs_ref(&fs), Path::new("/p/crates/truc/src")).unwrap();
        assert_eq!(layout.project_root(), Path::new("/p"));
    }

    #[test]
    fn trouve_la_racine_sur_place() {
        let fs = MemoryFileSystem::new().with_file("/p/_codev/config.yaml", "");
        let layout = discover(fs_ref(&fs), Path::new("/p")).unwrap();
        assert_eq!(layout.project_root(), Path::new("/p"));
    }

    #[test]
    fn echoue_en_nommant_le_point_de_depart() {
        let fs = MemoryFileSystem::new();
        let err = discover(fs_ref(&fs), Path::new("/ailleurs/ici")).unwrap_err();
        assert_eq!(err.code(), "no_codev_root");
        assert!(err.to_string().contains("/ailleurs/ici"), "{err}");
        assert!(err.to_string().contains("codev init"), "{err}");
    }

    #[test]
    fn prend_la_racine_la_plus_proche() {
        // Un dépôt imbriqué dans un autre : le travail appartient au plus
        // proche, jamais au parent.
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file("/p/sous-projet/_codev/config.yaml", "");
        let layout = discover(fs_ref(&fs), Path::new("/p/sous-projet/src")).unwrap();
        assert_eq!(layout.project_root(), Path::new("/p/sous-projet"));
    }

    fn fs_ref(fs: &MemoryFileSystem) -> &dyn FileSystem {
        fs
    }
}
