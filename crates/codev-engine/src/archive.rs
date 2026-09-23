//! `archive` : sync préflighté par le validateur + déplacement chronologique
//! du dossier de change.
//!
//! `archive` délègue la totalité de la logique de règles au validateur
//! (`codev-engine::validate::validate_change`). Aucun code d'erreur n'est
//! dupliqué ici ; en cas de finding d'erreur, un renvoi vers `codev validate`
//! suffit.

use std::path::PathBuf;

use codev_core::{ChangeId, Layout, Plan};

use crate::apply;
use crate::config::ResolvedConfig;
use crate::error::{EngineError, Result};
use crate::ports::{Clock, FileSystem};
use crate::sync::{self, SyncPlan};
use crate::validate;

/// Plan complet d'un `archive` : le sync et le déplacement final.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchivePlan {
    pub sync: SyncPlan,
    pub archive_dir: PathBuf,
    /// Le `Plan` composé — sync.plan + move final.
    pub plan: Plan,
}

/// Ce que l'archive a effectivement fait.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct ArchiveOutcome {
    pub change: String,
    pub root: PathBuf,
    pub created: Vec<PathBuf>,
    pub updated: Vec<PathBuf>,
    pub unchanged: Vec<PathBuf>,
    /// Main specs supprimées par le change (F5). Vide dans le cas courant.
    pub deleted: Vec<PathBuf>,
    pub moved_to: PathBuf,
}

/// Construit le plan sans écrire. Vérifie d'abord via `validate` — si le
/// change a la moindre erreur, on refuse sans même préparer le plan.
pub fn plan_archive(
    fs: &dyn FileSystem,
    layout: &Layout,
    config: &ResolvedConfig,
    clock: &dyn Clock,
    change_id: &ChangeId,
) -> Result<ArchivePlan> {
    // Pré-flight : le validateur donne la vérité. Un finding d'erreur → refus.
    let report = validate::validate_change(fs, layout, config, change_id)?;
    if report.has_errors() {
        return Err(EngineError::Invalid {
            path: layout.change_dir(change_id),
            reason: format!(
                "validation_failed : le change « {change_id} » a des erreurs ; \
                 lance `codev validate {change_id}` pour le détail"
            ),
        });
    }

    let sync_plan = sync::plan_sync(fs, layout, config, change_id)?;

    // Cible du déplacement : `changes/archive/<date>-<name>/`. Le préfixe
    // date sert au classement chronologique sur disque.
    let archive_dir = layout.archived_change_dir(change_id, &clock.today());

    let mut combined = sync_plan.plan.clone();
    if let Some(parent) = archive_dir.parent() {
        combined.dir(parent.to_path_buf());
    }
    combined.move_dir(layout.change_dir(change_id), archive_dir.clone());

    Ok(ArchivePlan {
        sync: sync_plan,
        archive_dir,
        plan: combined,
    })
}

/// Exécute le plan d'archive.
pub fn execute_archive(
    fs: &dyn FileSystem,
    layout: &Layout,
    config: &ResolvedConfig,
    clock: &dyn Clock,
    change_id: &ChangeId,
) -> Result<ArchiveOutcome> {
    let archive_plan = plan_archive(fs, layout, config, clock, change_id)?;
    let applied = apply::execute(&archive_plan.plan, fs)?;

    // Le sync a distingué les creates/updates au moment du plan ; on les
    // reprend depuis les listes du sync_plan (plutôt que de deviner depuis
    // `applied.created`, qui inclurait aussi les dirs planifiés qu'on ne
    // veut pas compter comme des « fichiers créés »).
    let mut outcome = ArchiveOutcome {
        change: change_id.to_string(),
        root: layout.project_root().to_path_buf(),
        created: archive_plan.sync.creates.clone(),
        updated: archive_plan.sync.updates.clone(),
        unchanged: archive_plan.sync.unchanged.clone(),
        deleted: archive_plan.sync.deleted.clone(),
        moved_to: archive_plan.archive_dir.clone(),
    };
    // Sanity : le move a bien eu lieu.
    let expected_source = layout.change_dir(change_id);
    if !applied
        .moved
        .iter()
        .any(|(from, _)| from == &expected_source)
    {
        return Err(EngineError::Invalid {
            path: layout.change_dir(change_id),
            reason: "le déplacement du change n'a pas eu lieu".into(),
        });
    }
    outcome.unchanged.extend(applied.untouched);
    outcome.unchanged.sort();
    outcome.unchanged.dedup();
    Ok(outcome)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config;
    use crate::ports::{FixedClock, FixedEnv, MemoryFileSystem};

    fn env() -> FixedEnv {
        let mut env = FixedEnv::at("/p");
        env.vars.insert("HOME".into(), "/home".into());
        env
    }

    fn resolved(fs: &dyn FileSystem) -> ResolvedConfig {
        config::resolve(fs, &env(), &Layout::new("/p")).unwrap()
    }

    fn projet_bien_forme() -> MemoryFileSystem {
        MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file(
                "/p/_codev/changes/add-auth/change.yaml",
                "schema: spec-driven",
            )
            .with_file(
                "/p/_codev/changes/add-auth/specs/user-auth/spec.md",
                "## Purpose\n\nGère l'authentification.\n\n## ADDED Requirements\n\n### Requirement: Login\nThe system SHALL emit a token.\n\n#### Scenario: OK\n- **WHEN** login\n- **THEN** token\n",
            )
    }

    #[test]
    fn plan_inclut_sync_puis_move_date() {
        let fs = projet_bien_forme();
        let layout = Layout::new("/p");
        let cfg = resolved(&fs);
        let clock = FixedClock("2026-09-08".into());
        let plan = plan_archive(
            &fs,
            &layout,
            &cfg,
            &clock,
            &ChangeId::parse("add-auth").unwrap(),
        )
        .unwrap();

        // Le sync prépare la création de la main spec.
        assert_eq!(plan.sync.creates.len(), 1);
        // Le move est planifié vers `archive/<date>-<name>/`.
        assert_eq!(
            plan.archive_dir,
            PathBuf::from("/p/_codev/changes/archive/2026-09-08-add-auth")
        );
        assert!(plan
            .plan
            .moves
            .iter()
            .any(|m| m.from == std::path::Path::new("/p/_codev/changes/add-auth")
                && m.to == plan.archive_dir));
    }

    #[test]
    fn archive_deplace_le_change_et_conserve_ses_fichiers() {
        let fs = projet_bien_forme();
        let layout = Layout::new("/p");
        let cfg = resolved(&fs);
        let clock = FixedClock("2026-09-08".into());
        let outcome = execute_archive(
            &fs,
            &layout,
            &cfg,
            &clock,
            &ChangeId::parse("add-auth").unwrap(),
        )
        .unwrap();

        assert_eq!(
            outcome.moved_to,
            PathBuf::from("/p/_codev/changes/archive/2026-09-08-add-auth")
        );
        // Main spec créée à partir du delta.
        assert!(fs
            .read("/p/_codev/specs/user-auth/spec.md")
            .is_some_and(|s| s.contains("### Requirement: Login")));
        // Le dossier initial du change n'existe plus.
        assert!(!fs.exists(std::path::Path::new(
            "/p/_codev/changes/add-auth"
        )));
        // Ses fichiers sont sous l'archive.
        assert!(fs
            .read("/p/_codev/changes/archive/2026-09-08-add-auth/change.yaml")
            .is_some());
        assert!(fs
            .read("/p/_codev/changes/archive/2026-09-08-add-auth/specs/user-auth/spec.md")
            .is_some());
    }

    #[test]
    fn preflight_valide_avant_de_planifier() {
        // Un change avec un delta contenant un doublon (finding de validation)
        // ne doit ni écrire, ni déplacer.
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file(
                "/p/_codev/changes/buggy/change.yaml",
                "schema: spec-driven",
            )
            .with_file(
                "/p/_codev/changes/buggy/specs/x/spec.md",
                "## Purpose\n\nCap.\n\n## ADDED Requirements\n\n### Requirement: X\nThe system SHALL x.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n\n### Requirement: X\nThe system SHALL x.\n\n#### Scenario: T\n- **WHEN** c\n- **THEN** d\n",
            );
        let layout = Layout::new("/p");
        let cfg = resolved(&fs);
        let clock = FixedClock("2026-09-08".into());
        let err = plan_archive(
            &fs,
            &layout,
            &cfg,
            &clock,
            &ChangeId::parse("buggy").unwrap(),
        )
        .unwrap_err();
        assert_eq!(err.code(), "invalid");
        assert!(err.to_string().contains("validation_failed"), "{err}");
        // Nulle main spec créée.
        assert!(fs.read("/p/_codev/specs/x/spec.md").is_none());
        // Change intact.
        assert!(fs
            .read("/p/_codev/changes/buggy/specs/x/spec.md")
            .is_some());
    }
}
