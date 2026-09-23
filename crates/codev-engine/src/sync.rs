//! Orchestration du `sync` : lit les deltas d'un change, calcule un plan de
//! fusion pour chaque capacité touchée, agrège en un unique [`Plan`].
//!
//! Aucune règle de fusion ici — elles vivent dans `codev-core::merge`. On
//! coordonne, on décide « créer ou mettre à jour », on distingue
//! « inchangé » de « mis à jour » pour rendre `sync` idempotent au sens
//! strict.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use codev_core::merge::{apply_edits, build_new_spec, merge_into_existing, MergeError};
use codev_core::parser::{parse_delta, parse_spec};
use codev_core::{ChangeId, Layout, Plan, WriteMode};

use crate::apply;
use crate::config::ResolvedConfig;
use crate::error::{EngineError, Result};
use crate::ports::FileSystem;

/// Le plan complet d'un `sync` : quel `Plan` à exécuter, et le futur
/// « report » qu'on remplira à l'exécution (à ce stade on connaît créés et
/// mis à jour ; « inchangé » se déduit après application).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyncPlan {
    pub plan: Plan,
    /// Chemins de main specs qui vont être créés.
    pub creates: Vec<PathBuf>,
    /// Chemins de main specs qui vont être réécrites (contenu vraiment
    /// différent après application des édits).
    pub updates: Vec<PathBuf>,
    /// Chemins de main specs déjà à jour — le sync serait sans effet.
    pub unchanged: Vec<PathBuf>,
    /// Chemins de main specs à supprimer (F5) — la capacité est retirée
    /// par un `## REMOVED Requirements` qui viderait la spec, combiné à
    /// `retire_capabilities: true` dans le `change.yaml`.
    pub deleted: Vec<PathBuf>,
}

/// Ce que le sync a effectivement fait sur le disque.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct SyncOutcome {
    pub change: String,
    pub root: PathBuf,
    pub updated: Vec<PathBuf>,
    pub created: Vec<PathBuf>,
    pub unchanged: Vec<PathBuf>,
    /// Main specs supprimées (F5). Vide dans le cas courant.
    pub deleted: Vec<PathBuf>,
}

impl SyncOutcome {
    pub fn changed_anything(&self) -> bool {
        !self.updated.is_empty() || !self.created.is_empty() || !self.deleted.is_empty()
    }
}

/// Construit le plan sans écrire.
///
/// Fonction publique : `archive` s'en sert pour composer un plan plus large
/// (sync + move).
pub fn plan_sync(
    fs: &dyn FileSystem,
    layout: &Layout,
    config: &ResolvedConfig,
    change_id: &ChangeId,
) -> Result<SyncPlan> {
    let change_dir = layout.change_dir(change_id);
    if !fs.exists(&change_dir) {
        return Err(EngineError::UnknownChange {
            change: change_id.to_string(),
        });
    }

    // Charger le contexte du change pour connaître `retire_capabilities`
    // — sans quoi un REMOVED total ne peut pas se distinguer d'une
    // erreur.
    let ctx = crate::change::load(fs, layout, config, change_id.clone())?;

    let mut plan = Plan::new();
    let mut creates = Vec::new();
    let mut updates = Vec::new();
    let mut unchanged = Vec::new();
    let mut deleted = Vec::new();
    let mut vus: BTreeSet<PathBuf> = BTreeSet::new();

    for delta_path in locate_delta_files(fs, &change_dir)? {
        let capability = capability_from_delta_path(&change_dir, &delta_path)
            .ok_or_else(|| EngineError::Invalid {
                path: delta_path.clone(),
                reason: "chemin de delta hors de `changes/<name>/specs/`".into(),
            })?;

        let delta_source = fs
            .read_to_string(&delta_path)
            .map_err(|e| EngineError::Unreadable {
                path: delta_path.clone(),
                reason: e.to_string(),
            })?;
        let delta_parsed = parse_delta(&delta_source);
        if delta_parsed.has_errors() {
            // On refuse silencieusement de sync si le delta a un finding
            // d'erreur — c'est le rôle du validateur, mais un delta cassé
            // parasiterait la fusion.
            return Err(EngineError::Invalid {
                path: delta_path.clone(),
                reason: format!(
                    "delta contient {} finding(s) d'erreur ; lance `codev validate` d'abord",
                    delta_parsed.findings.len()
                ),
            });
        }
        let delta = delta_parsed.value;

        let main_spec_path = layout.spec_file(&capability);
        if !vus.insert(main_spec_path.clone()) {
            // Un même main spec touché par deux deltas du même change (via
            // deux fichiers différents) — cas inhabituel, mais on ne
            // fusionne qu'une fois pour éviter des edits contradictoires.
            continue;
        }

        if fs.exists(&main_spec_path) {
            let existing_source = fs.read_to_string(&main_spec_path).map_err(|e| {
                EngineError::Unreadable {
                    path: main_spec_path.clone(),
                    reason: e.to_string(),
                }
            })?;
            let existing_parsed = parse_spec(&existing_source);
            let merge_plan = merge_into_existing(
                &existing_source,
                &existing_parsed.value,
                &delta,
                ctx.metadata.retire_capabilities,
            )
            .map_err(|e| EngineError::Invalid {
                path: main_spec_path.clone(),
                reason: format!("{} ({})", e.message(), e.code()),
            })?;
            if merge_plan.should_delete_spec {
                // La capacité est retirée : ni update ni unchanged, la
                // spec entière s'en va.
                deleted.push(main_spec_path.clone());
                plan.delete(main_spec_path);
            } else {
                let new_contents = apply_edits(&existing_source, &merge_plan.edits);
                if new_contents == existing_source {
                    unchanged.push(main_spec_path.clone());
                } else {
                    updates.push(main_spec_path.clone());
                    plan.write(main_spec_path, new_contents, WriteMode::Overwrite);
                }
            }
        } else {
            let contents = build_new_spec(&capability, &delta).map_err(|e| {
                EngineError::Invalid {
                    path: main_spec_path.clone(),
                    reason: format!("{} ({})", e.message(), e.code()),
                }
            })?;
            if let Some(parent) = main_spec_path.parent() {
                plan.dir(parent.to_path_buf());
            }
            creates.push(main_spec_path.clone());
            plan.write(main_spec_path, contents, WriteMode::CreateOnly);
        }
    }

    Ok(SyncPlan {
        plan,
        creates,
        updates,
        unchanged,
        deleted,
    })
}

/// Exécute le plan de sync.
pub fn execute_sync(
    fs: &dyn FileSystem,
    layout: &Layout,
    config: &ResolvedConfig,
    change_id: &ChangeId,
) -> Result<SyncOutcome> {
    let sync_plan = plan_sync(fs, layout, config, change_id)?;
    let applied = apply::execute(&sync_plan.plan, fs)?;
    // `applied.created` couvre les creates ; `applied.overwritten` couvre
    // les updates ; `applied.untouched` (rare ici, mais possible) rejoint
    // `unchanged`.
    let mut outcome = SyncOutcome {
        change: change_id.to_string(),
        root: layout.project_root().to_path_buf(),
        created: applied.created,
        updated: applied.overwritten,
        unchanged: sync_plan.unchanged,
        deleted: applied.deleted,
    };
    outcome.unchanged.extend(applied.untouched);
    outcome.unchanged.sort();
    outcome.unchanged.dedup();
    Ok(outcome)
}

/// Les fichiers `*.md` sous `changes/<name>/specs/`, triés.
fn locate_delta_files(fs: &dyn FileSystem, change_dir: &Path) -> Result<Vec<PathBuf>> {
    let specs_dir = change_dir.join("specs");
    let files = fs
        .walk_files(&specs_dir)
        .map_err(|e| EngineError::Unreadable {
            path: specs_dir.clone(),
            reason: e.to_string(),
        })?;
    let mut out: Vec<PathBuf> = files
        .into_iter()
        .filter(|p| p.ends_with(".md"))
        .map(|p| specs_dir.join(p))
        .collect();
    out.sort();
    Ok(out)
}

/// Extrait `<capability-path>` depuis `changes/<name>/specs/<capability-path>/spec.md`.
fn capability_from_delta_path(change_dir: &Path, delta_path: &Path) -> Option<String> {
    let specs_dir = change_dir.join("specs");
    let relative = delta_path.strip_prefix(&specs_dir).ok()?;
    let parent = relative.parent()?;
    if parent.as_os_str().is_empty() {
        return None;
    }
    Some(
        parent
            .components()
            .map(|c| c.as_os_str().to_string_lossy())
            .collect::<Vec<_>>()
            .join("/"),
    )
}

/// Rappel : `MergeError` — le type d'erreur pur du cœur — est traduit ici en
/// [`EngineError::Invalid`] pour ne pas fuiter la nomenclature interne dans
/// le contrat public. `has_error` reste stable ; le code d'erreur est
/// préservé dans le message pour que le consommateur JSON puisse le retrouver.
#[allow(dead_code)]
fn _merge_error_marker(_: &MergeError) {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config;
    use crate::ports::{FixedEnv, MemoryFileSystem};

    fn env() -> FixedEnv {
        let mut env = FixedEnv::at("/p");
        env.vars.insert("HOME".into(), "/home".into());
        env
    }

    fn resolved(fs: &dyn FileSystem) -> ResolvedConfig {
        config::resolve(fs, &env(), &Layout::new("/p")).unwrap()
    }

    /// Un projet minimal : un change avec un delta ADDED sur une capacité
    /// nouvelle `user-auth`. Aucune main spec existante.
    fn projet_avec_delta_added() -> MemoryFileSystem {
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
    fn plan_sync_produit_un_write_par_capacite_touchee() {
        let fs = projet_avec_delta_added();
        let layout = Layout::new("/p");
        let cfg = resolved(&fs);
        let plan =
            plan_sync(&fs, &layout, &cfg, &ChangeId::parse("add-auth").unwrap()).unwrap();

        assert_eq!(plan.creates.len(), 1);
        assert_eq!(plan.updates.len(), 0);
        assert_eq!(plan.plan.writes.len(), 1);
        assert_eq!(
            plan.creates[0],
            PathBuf::from("/p/_codev/specs/user-auth/spec.md")
        );
    }

    #[test]
    fn sync_dun_delta_multi_capacites_reussit() {
        // Deux capacités touchées : l'une créée (Purpose + ADDED), l'autre
        // déjà existante et modifiée.
        let fs = projet_avec_delta_added()
            .with_file(
                "/p/_codev/specs/session/spec.md",
                "## Purpose\n\nSession.\n\n## Requirements\n\n### Requirement: Timeout\nThe system SHALL expire.\n\n#### Scenario: T\n- **WHEN** idle\n- **THEN** expire\n",
            )
            .with_file(
                "/p/_codev/changes/add-auth/specs/session/spec.md",
                "## MODIFIED Requirements\n\n### Requirement: Timeout\nThe system MUST expire after 15 minutes.\n\n#### Scenario: T\n- **WHEN** idle\n- **THEN** expire\n",
            );
        let layout = Layout::new("/p");
        let cfg = resolved(&fs);
        let outcome =
            execute_sync(&fs, &layout, &cfg, &ChangeId::parse("add-auth").unwrap()).unwrap();

        assert_eq!(outcome.created.len(), 1, "user-auth créé");
        assert_eq!(outcome.updated.len(), 1, "session mis à jour");
        // Sur disque, tout est écrit.
        assert!(fs
            .read("/p/_codev/specs/user-auth/spec.md")
            .is_some_and(|s| s.contains("### Requirement: Login")));
        assert!(fs
            .read("/p/_codev/specs/session/spec.md")
            .is_some_and(|s| s.contains("15 minutes")));
    }

    #[test]
    fn deuxieme_sync_ne_change_rien() {
        // L'invariant d'idempotence : sync deux fois de suite laisse le
        // deuxième `changed_anything() == false`.
        let fs = projet_avec_delta_added();
        let layout = Layout::new("/p");
        let cfg = resolved(&fs);
        let change = ChangeId::parse("add-auth").unwrap();

        let premier = execute_sync(&fs, &layout, &cfg, &change).unwrap();
        assert!(premier.changed_anything());

        let second = execute_sync(&fs, &layout, &cfg, &change).unwrap();
        assert!(
            !second.changed_anything(),
            "sync doit être idempotent ; observé : {:?}",
            second
        );
    }

    #[test]
    fn sync_dun_delta_modified_sans_cible_echoue_avec_code_utile() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file(
                "/p/_codev/changes/broken/change.yaml",
                "schema: spec-driven",
            )
            .with_file(
                "/p/_codev/specs/x/spec.md",
                "## Purpose\n\nx.\n\n## Requirements\n\n### Requirement: Login\nThe system SHALL x.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n",
            )
            .with_file(
                "/p/_codev/changes/broken/specs/x/spec.md",
                "## MODIFIED Requirements\n\n### Requirement: Fantome\nThe system SHALL y.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n",
            );
        let layout = Layout::new("/p");
        let cfg = resolved(&fs);
        let err = execute_sync(&fs, &layout, &cfg, &ChangeId::parse("broken").unwrap()).unwrap_err();
        assert_eq!(err.code(), "invalid");
        assert!(err.to_string().contains("modified_target_missing"), "{err}");
        // Rien n'a été écrit.
        assert!(fs
            .read("/p/_codev/specs/x/spec.md")
            .is_some_and(|s| s.contains("Login") && !s.contains("Fantome")));
    }

    #[test]
    fn capability_est_extraite_du_chemin_meme_imbriquee() {
        let change_dir = Path::new("/p/_codev/changes/add-auth");
        let delta = Path::new("/p/_codev/changes/add-auth/specs/identity/user-auth/spec.md");
        assert_eq!(
            capability_from_delta_path(change_dir, delta),
            Some("identity/user-auth".into())
        );
    }

    // ─────────────── F5 : retire_capabilities ───────────────

    fn projet_avec_retire_capabilities(flag: bool) -> MemoryFileSystem {
        let yaml = if flag {
            "schema: spec-driven\nretire_capabilities: true\n"
        } else {
            "schema: spec-driven\n"
        };
        MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            // Spec principale existante avec une unique exigence Solo.
            .with_file(
                "/p/_codev/specs/user-auth/spec.md",
                "## Purpose\n\nGère l'auth.\n\n## Requirements\n\n### Requirement: Solo\nThe system SHALL x.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n",
            )
            // Change qui la retire entièrement.
            .with_file("/p/_codev/changes/retire-auth/change.yaml", yaml)
            .with_file(
                "/p/_codev/changes/retire-auth/specs/user-auth/spec.md",
                "## REMOVED Requirements\n\n### Requirement: Solo\n**Reason**: obsolete\n**Migration**: none\n",
            )
    }

    #[test]
    fn sync_retire_capability_avec_flag_supprime_la_spec() {
        let fs = projet_avec_retire_capabilities(true);
        let layout = Layout::new("/p");
        let cfg = resolved(&fs);

        let outcome = execute_sync(&fs, &layout, &cfg, &ChangeId::parse("retire-auth").unwrap())
            .unwrap();

        assert_eq!(
            outcome.deleted,
            vec![PathBuf::from("/p/_codev/specs/user-auth/spec.md")]
        );
        assert!(outcome.updated.is_empty());
        assert!(outcome.unchanged.is_empty());
        assert!(!fs.exists(std::path::Path::new(
            "/p/_codev/specs/user-auth/spec.md"
        )));
    }

    #[test]
    fn sync_retire_capability_sans_flag_refuse() {
        let fs = projet_avec_retire_capabilities(false);
        let layout = Layout::new("/p");
        let cfg = resolved(&fs);

        let err = execute_sync(&fs, &layout, &cfg, &ChangeId::parse("retire-auth").unwrap())
            .unwrap_err();
        assert_eq!(err.code(), "invalid");
        assert!(err.to_string().contains("would_leave_spec_without_requirement"));
        // Le fichier existe toujours.
        assert!(fs.exists(std::path::Path::new(
            "/p/_codev/specs/user-auth/spec.md"
        )));
    }
}
