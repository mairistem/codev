use std::collections::BTreeSet;

use codev_core::outputs::pattern_matches_any;
use codev_core::{status, ChangeId, ChangeStatus, Layout};

use crate::config::ResolvedConfig;
use crate::error::{EngineError, Result};
use crate::metadata::{self, ChangeMetadata};
use crate::ports::FileSystem;
use crate::schemas::{self, ResolvedSchema};

/// Tout ce qu'il faut pour agir sur un change : son identité, ses métadonnées,
/// et le schéma qui le régit.
#[derive(Debug)]
pub struct ChangeContext {
    pub change: ChangeId,
    pub metadata: ChangeMetadata,
    pub schema: ResolvedSchema,
}

/// Charge un change existant.
///
/// L'absence de `change.yaml` n'est pas fatale : un change créé à la main
/// hérite alors du schéma de la config du projet. Refuser aurait fait de
/// l'outil le propriétaire de dossiers que l'utilisateur a le droit de créer
/// lui-même.
pub fn load(
    fs: &dyn FileSystem,
    layout: &Layout,
    config: &ResolvedConfig,
    change: ChangeId,
) -> Result<ChangeContext> {
    if !fs.exists(&layout.change_dir(&change)) {
        return Err(EngineError::UnknownChange {
            change: change.to_string(),
        });
    }

    let metadata = metadata::load(fs, &layout.change_metadata(&change))?.unwrap_or(ChangeMetadata {
        schema: config.schema.clone(),
        created: None,
        goal: None,
        skip_specs: false,
        retire_capabilities: false,
    });

    let schema = schemas::resolve(fs, layout, &metadata.schema)?;
    Ok(ChangeContext {
        change,
        metadata,
        schema,
    })
}

/// L'état du change, déduit de ce qui existe sur le disque.
pub fn status(fs: &dyn FileSystem, layout: &Layout, ctx: &ChangeContext) -> Result<ChangeStatus> {
    let dir = layout.change_dir(&ctx.change);
    // Un seul parcours du dossier pour tous les artefacts : les motifs se
    // testent ensuite en mémoire, dans le cœur pur.
    let files = fs.walk_files(&dir).map_err(|e| EngineError::Unreadable {
        path: dir.clone(),
        reason: e.to_string(),
    })?;

    let mut existing = BTreeSet::new();
    for artifact in ctx.schema.graph.artifacts() {
        let present = if artifact.is_pattern() {
            pattern_matches_any(&artifact.generates, &files)?
        } else {
            files.iter().any(|f| f == &artifact.generates)
        };
        if present {
            existing.insert(artifact.id.clone());
        }
    }

    let skipped = ctx.metadata.skipped_artifacts(&ctx.schema.graph);
    Ok(status::compute(
        &ctx.schema.graph,
        &ctx.change,
        &existing,
        &skipped,
    ))
}

/// Les changes actifs, par ordre alphabétique.
///
/// `archive/` est exclu — il contient les changes terminés — et tout dossier
/// dont le nom n'est pas un identifiant valide est ignoré silencieusement :
/// l'utilisateur a le droit d'avoir des dossiers à lui là-dedans.
pub fn list(fs: &dyn FileSystem, layout: &Layout) -> Vec<ChangeId> {
    let changes_dir = layout.changes_dir();
    let Ok(names) = fs.list_dir(&changes_dir) else {
        return Vec::new();
    };
    names
        .into_iter()
        .filter(|name| name != "archive")
        .filter_map(|name| ChangeId::parse(&name).ok())
        .filter(|change| fs.exists(&layout.change_dir(change)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::DEFAULT_SCHEMA;
    use crate::ports::{FixedEnv, MemoryFileSystem};
    use codev_core::ArtifactState;

    fn config(fs: &dyn FileSystem, layout: &Layout) -> ResolvedConfig {
        let mut env = FixedEnv::at("/p");
        env.vars.insert("HOME".into(), "/home".into());
        crate::config::resolve(fs, &env, layout).unwrap()
    }

    fn contexte(fs: &MemoryFileSystem, nom: &str) -> ChangeContext {
        let layout = Layout::new("/p");
        let cfg = config(fs, &layout);
        load(fs, &layout, &cfg, ChangeId::parse(nom).unwrap()).unwrap()
    }

    #[test]
    fn un_change_inconnu_est_une_erreur_qui_oriente() {
        let fs = MemoryFileSystem::new().with_file("/p/_codev/config.yaml", "");
        let layout = Layout::new("/p");
        let cfg = config(&fs, &layout);
        let err = load(&fs, &layout, &cfg, ChangeId::parse("fantome").unwrap()).unwrap_err();
        assert_eq!(err.code(), "unknown_change");
        assert!(err.to_string().contains("codev list"), "{err}");
    }

    #[test]
    fn un_change_sans_metadonnees_herite_du_schema_du_projet() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file("/p/_codev/changes/a-la-main/proposal.md", "# Proposal");
        let ctx = contexte(&fs, "a-la-main");
        assert_eq!(ctx.metadata.schema, DEFAULT_SCHEMA);
        assert_eq!(ctx.metadata.created, None);
    }

    #[test]
    fn deduit_letat_des_artefacts_du_disque() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file("/p/_codev/changes/add-auth/change.yaml", "schema: spec-driven")
            .with_file("/p/_codev/changes/add-auth/proposal.md", "# Proposal");

        let ctx = contexte(&fs, "add-auth");
        let status = status(&fs, &Layout::new("/p"), &ctx).unwrap();

        let etat = |id: &str| {
            status
                .artifacts
                .iter()
                .find(|a| a.id == id)
                .unwrap()
                .state
        };
        assert_eq!(etat("proposal"), ArtifactState::Done);
        assert_eq!(etat("specs"), ArtifactState::Ready);
        assert_eq!(etat("tasks"), ArtifactState::Blocked);
    }

    #[test]
    fn une_spec_imbriquee_satisfait_le_motif_de_lartefact() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file("/p/_codev/changes/add-auth/change.yaml", "schema: spec-driven")
            .with_file("/p/_codev/changes/add-auth/proposal.md", "x")
            .with_file(
                "/p/_codev/changes/add-auth/specs/identity/user-auth/spec.md",
                "y",
            );

        let ctx = contexte(&fs, "add-auth");
        let status = status(&fs, &Layout::new("/p"), &ctx).unwrap();
        let specs = status.artifacts.iter().find(|a| a.id == "specs").unwrap();
        assert_eq!(specs.state, ArtifactState::Done);
    }

    #[test]
    fn skip_specs_rend_tasks_ecrivable_sans_specs() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file(
                "/p/_codev/changes/refactor/change.yaml",
                "schema: spec-driven\nskip_specs: true\n",
            )
            .with_file("/p/_codev/changes/refactor/proposal.md", "x")
            .with_file("/p/_codev/changes/refactor/design.md", "y");

        let ctx = contexte(&fs, "refactor");
        let status = status(&fs, &Layout::new("/p"), &ctx).unwrap();
        let etat = |id: &str| {
            status
                .artifacts
                .iter()
                .find(|a| a.id == id)
                .unwrap()
                .state
        };
        assert_eq!(etat("specs"), ArtifactState::Skipped);
        assert_eq!(etat("tasks"), ArtifactState::Ready);
    }

    #[test]
    fn liste_les_changes_actifs_sans_larchive() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file("/p/_codev/changes/add-auth/proposal.md", "x")
            .with_file("/p/_codev/changes/fix-bug/proposal.md", "x")
            .with_file(
                "/p/_codev/changes/archive/2026-01-01-vieux/proposal.md",
                "x",
            );

        let noms: Vec<String> = list(&fs, &Layout::new("/p"))
            .iter()
            .map(ToString::to_string)
            .collect();
        assert_eq!(noms, ["add-auth", "fix-bug"]);
    }
}
