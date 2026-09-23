use std::path::PathBuf;

use codev_core::{Plan, WriteMode};

use crate::error::{EngineError, Result};
use crate::ports::FileSystem;

/// Ce qu'une exécution de plan a réellement fait.
///
/// Distinguer les trois cas n'est pas cosmétique : c'est ce qui permet à
/// `init` de dire « rien à faire » plutôt que « 12 fichiers écrits » sur un
/// projet déjà initialisé.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Applied {
    pub created: Vec<PathBuf>,
    pub overwritten: Vec<PathBuf>,
    /// Déjà conformes, ou protégés par [`WriteMode::CreateOnly`].
    pub untouched: Vec<PathBuf>,
    /// Suppressions effectivement appliquées. Un fichier absent au
    /// moment de la deletion n'y figure pas (ce n'est pas une erreur —
    /// le change peut avoir déjà été appliqué).
    pub deleted: Vec<PathBuf>,
    /// Déplacements effectués — `(from, to)`.
    pub moved: Vec<(PathBuf, PathBuf)>,
}

impl Applied {
    pub fn changed_anything(&self) -> bool {
        !self.created.is_empty()
            || !self.overwritten.is_empty()
            || !self.deleted.is_empty()
            || !self.moved.is_empty()
    }
}

/// Exécute un plan.
///
/// La coquille impérative : aucune décision ici, seulement des écritures. Tout
/// ce qui relève du choix a été tranché en amont, dans une fonction pure.
pub fn execute(plan: &Plan, fs: &dyn FileSystem) -> Result<Applied> {
    let mut applied = Applied::default();

    for dir in &plan.dirs {
        fs.create_dir_all(dir).map_err(|source| EngineError::Write {
            path: dir.clone(),
            source,
        })?;
    }

    for write in &plan.writes {
        let existe = fs.exists(&write.path);
        match write.mode {
            WriteMode::CreateOnly if existe => {
                applied.untouched.push(write.path.clone());
                continue;
            }
            WriteMode::Overwrite if existe => {
                // Comparer avant d'écrire : sans cela, `codev update` annoncerait
                // avoir modifié des fichiers identiques, et l'utilisateur ne
                // saurait plus ce qui a vraiment bougé.
                let identique = fs
                    .read_to_string(&write.path)
                    .map(|actuel| actuel == write.contents)
                    .unwrap_or(false);
                if identique {
                    applied.untouched.push(write.path.clone());
                    continue;
                }
                write_file(fs, write.path.clone(), &write.contents)?;
                applied.overwritten.push(write.path.clone());
                continue;
            }
            _ => {}
        }
        write_file(fs, write.path.clone(), &write.contents)?;
        applied.created.push(write.path.clone());
    }

    // Les deletions viennent après les writes : « on écrit ce qui est
    // neuf, on retire ce qui n'a plus lieu d'être », et avant les moves
    // pour préserver l'atomicité du geste destructeur. Un fichier
    // absent au moment de la deletion n'est pas une erreur — le change
    // peut avoir été appliqué déjà, ou le fichier peut avoir été
    // supprimé manuellement entre le plan et l'exécution.
    for path in &plan.deletions {
        match fs.remove_file(path) {
            Ok(()) => applied.deleted.push(path.clone()),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                // Idempotent — pas de trace dans `applied.deleted`.
            }
            Err(source) => {
                return Err(EngineError::Write {
                    path: path.clone(),
                    source,
                });
            }
        }
    }

    // Les déplacements en dernier. Un `Move` peut avoir besoin qu'un
    // dossier de destination existe (couvert par la boucle `dirs`) et qu'un
    // fichier de source ait été écrit à l'endroit qu'on va déplacer (couvert
    // par la boucle `writes`).
    for mv in &plan.moves {
        fs.rename(&mv.from, &mv.to).map_err(|source| EngineError::Write {
            path: mv.from.clone(),
            source,
        })?;
        applied.moved.push((mv.from.clone(), mv.to.clone()));
    }

    Ok(applied)
}

fn write_file(fs: &dyn FileSystem, path: PathBuf, contents: &str) -> Result<()> {
    fs.write(&path, contents)
        .map_err(|source| EngineError::Write { path, source })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ports::MemoryFileSystem;
    use std::path::Path;

    #[test]
    fn cree_ce_qui_manque() {
        let fs = MemoryFileSystem::new();
        let mut plan = Plan::new();
        plan.dir("/p/_codev")
            .write("/p/_codev/config.yaml", "schema: spec-driven", WriteMode::CreateOnly);

        let applied = execute(&plan, &fs).unwrap();

        assert_eq!(applied.created, [PathBuf::from("/p/_codev/config.yaml")]);
        assert!(applied.untouched.is_empty());
        assert_eq!(fs.read("/p/_codev/config.yaml").as_deref(), Some("schema: spec-driven"));
    }

    #[test]
    fn create_only_protege_le_travail_de_lutilisateur() {
        let fs = MemoryFileSystem::new().with_file("/p/_codev/config.yaml", "ma config à moi");
        let mut plan = Plan::new();
        plan.write("/p/_codev/config.yaml", "config générée", WriteMode::CreateOnly);

        let applied = execute(&plan, &fs).unwrap();

        assert_eq!(applied.untouched, [PathBuf::from("/p/_codev/config.yaml")]);
        assert!(!applied.changed_anything());
        assert_eq!(
            fs.read("/p/_codev/config.yaml").as_deref(),
            Some("ma config à moi"),
            "le contenu existant doit être intact"
        );
    }

    #[test]
    fn overwrite_ne_signale_que_les_vrais_changements() {
        let fs = MemoryFileSystem::new().with_file("/s/SKILL.md", "identique");
        let mut plan = Plan::new();
        plan.write("/s/SKILL.md", "identique", WriteMode::Overwrite);

        let applied = execute(&plan, &fs).unwrap();

        assert!(applied.overwritten.is_empty(), "rien n'a changé");
        assert_eq!(applied.untouched, [PathBuf::from("/s/SKILL.md")]);
    }

    #[test]
    fn overwrite_remplace_un_contenu_different() {
        let fs = MemoryFileSystem::new().with_file("/s/SKILL.md", "ancienne version");
        let mut plan = Plan::new();
        plan.write("/s/SKILL.md", "nouvelle version", WriteMode::Overwrite);

        let applied = execute(&plan, &fs).unwrap();

        assert_eq!(applied.overwritten, [PathBuf::from("/s/SKILL.md")]);
        assert_eq!(fs.read("/s/SKILL.md").as_deref(), Some("nouvelle version"));
    }

    #[test]
    fn execute_deplace_apres_avoir_ecrit() {
        // L'ordre importe : le `Move` ne fonctionne que si le fichier de
        // source vient d'être écrit — c'est le contrat qu'utilise `archive`
        // pour déplacer un dossier qu'on vient éventuellement de compléter.
        let fs = MemoryFileSystem::new();
        let mut plan = Plan::new();
        plan.write("/p/a/file.md", "x", WriteMode::Overwrite);
        plan.move_dir("/p/a", "/p/b");

        let applied = execute(&plan, &fs).unwrap();

        assert_eq!(applied.created, [PathBuf::from("/p/a/file.md")]);
        assert_eq!(applied.moved, [(PathBuf::from("/p/a"), PathBuf::from("/p/b"))]);
        assert!(!fs.exists(Path::new("/p/a/file.md")));
        assert_eq!(fs.read("/p/b/file.md").as_deref(), Some("x"));
    }

    #[test]
    fn execute_move_seul_change_letat() {
        let fs = MemoryFileSystem::new().with_file("/p/x", "content");
        let mut plan = Plan::new();
        plan.move_dir("/p/x", "/q/x");

        let applied = execute(&plan, &fs).unwrap();

        assert!(applied.changed_anything(), "un move seul compte comme un vrai changement");
        assert_eq!(applied.moved.len(), 1);
    }

    #[test]
    fn execute_move_source_absente_est_une_erreur_typee() {
        let fs = MemoryFileSystem::new();
        let mut plan = Plan::new();
        plan.move_dir("/pas/la", "/ailleurs");

        let err = execute(&plan, &fs).unwrap_err();
        assert_eq!(err.code(), "write_failed");
    }

    #[test]
    fn deletion_supprime_un_fichier_existant() {
        let fs = MemoryFileSystem::new().with_file("/p/a.md", "x");
        let mut plan = Plan::new();
        plan.delete("/p/a.md");

        let applied = execute(&plan, &fs).unwrap();
        assert_eq!(applied.deleted, [PathBuf::from("/p/a.md")]);
        assert!(!fs.exists(Path::new("/p/a.md")));
    }

    #[test]
    fn deletion_sur_absent_est_silencieuse() {
        // Idempotence — un plan relancé après suppression ne remonte pas
        // d'erreur, et ne signale rien.
        let fs = MemoryFileSystem::new();
        let mut plan = Plan::new();
        plan.delete("/p/absent.md");

        let applied = execute(&plan, &fs).unwrap();
        assert!(applied.deleted.is_empty());
        assert!(!applied.changed_anything());
    }

    #[test]
    fn deletion_apres_writes_et_avant_moves() {
        // Ordre exigé par le design : dirs → writes → deletions → moves.
        let fs = MemoryFileSystem::new()
            .with_file("/p/vieux.md", "à retirer")
            .with_file("/p/dossier/nouveau.md", "sera écrit avant");
        let mut plan = Plan::new();
        plan.write("/p/dossier/nouveau.md", "modifié", WriteMode::Overwrite);
        plan.delete("/p/vieux.md");
        plan.move_dir("/p/dossier", "/p/dossier-renomme");

        let applied = execute(&plan, &fs).unwrap();

        // Le write a bien eu lieu (dans le dossier d'origine, avant move).
        assert_eq!(
            applied.overwritten,
            [PathBuf::from("/p/dossier/nouveau.md")]
        );
        assert_eq!(applied.deleted, [PathBuf::from("/p/vieux.md")]);
        assert_eq!(applied.moved.len(), 1);
        assert!(!fs.exists(Path::new("/p/vieux.md")));
        assert!(fs.exists(Path::new("/p/dossier-renomme/nouveau.md")));
    }

    #[test]
    fn rejouer_un_plan_est_sans_effet() {
        // L'idempotence est ce qui rend `codev init` et `codev update`
        // relançables sans y penser.
        let fs = MemoryFileSystem::new();
        let mut plan = Plan::new();
        plan.write("/p/a.md", "x", WriteMode::CreateOnly)
            .write("/p/b.md", "y", WriteMode::Overwrite);

        assert!(execute(&plan, &fs).unwrap().changed_anything());
        assert!(!execute(&plan, &fs).unwrap().changed_anything());
    }
}
