use std::collections::BTreeSet;

use codev_core::outputs::pattern_matches_any;
use codev_core::{ChangeId, ChangeStatus, Layout, status};

use crate::config::ResolvedConfig;
use crate::error::{EngineError, Result};
use crate::metadata::{self, ChangeMetadata};
use crate::ports::FileSystem;
use crate::schemas::{self, ResolvedSchema};

/// Everything needed to act on a change: its identity, its metadata, and the
/// schema that governs it.
#[derive(Debug)]
pub struct ChangeContext {
    pub change: ChangeId,
    pub metadata: ChangeMetadata,
    pub schema: ResolvedSchema,
}

/// Loads an existing change.
///
/// A missing `change.yaml` is not fatal: a hand-made change then inherits
/// the schema from the project config. Refusing would have made the tool the
/// owner of directories that the user is entitled to create themselves.
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

    let metadata =
        metadata::load(fs, &layout.change_metadata(&change))?.unwrap_or(ChangeMetadata {
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

/// The change's status, derived from what exists on disk.
pub fn status(fs: &dyn FileSystem, layout: &Layout, ctx: &ChangeContext) -> Result<ChangeStatus> {
    let dir = layout.change_dir(&ctx.change);
    // A single walk of the directory for all artifacts: the patterns are then
    // tested in memory, in the pure core.
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

/// The active changes, in alphabetical order.
///
/// `archive/` is excluded — it holds the finished changes — and any directory
/// whose name is not a valid identifier is silently ignored: the user is
/// entitled to keep directories of their own in there.
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

    fn context(fs: &MemoryFileSystem, name: &str) -> ChangeContext {
        let layout = Layout::new("/p");
        let cfg = config(fs, &layout);
        load(fs, &layout, &cfg, ChangeId::parse(name).unwrap()).unwrap()
    }

    #[test]
    fn an_unknown_change_is_an_error_that_guides() {
        let fs = MemoryFileSystem::new().with_file("/p/_codev/config.yaml", "");
        let layout = Layout::new("/p");
        let cfg = config(&fs, &layout);
        let err = load(&fs, &layout, &cfg, ChangeId::parse("ghost").unwrap()).unwrap_err();
        assert_eq!(err.code(), "unknown_change");
        assert!(err.to_string().contains("codev list"), "{err}");
    }

    #[test]
    fn a_change_without_metadata_inherits_the_project_schema() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file("/p/_codev/changes/hand-made/proposal.md", "# Proposal");
        let ctx = context(&fs, "hand-made");
        assert_eq!(ctx.metadata.schema, DEFAULT_SCHEMA);
        assert_eq!(ctx.metadata.created, None);
    }

    #[test]
    fn derives_artifact_states_from_disk() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file(
                "/p/_codev/changes/add-auth/change.yaml",
                "schema: spec-driven",
            )
            .with_file("/p/_codev/changes/add-auth/proposal.md", "# Proposal");

        let ctx = context(&fs, "add-auth");
        let status = status(&fs, &Layout::new("/p"), &ctx).unwrap();

        let state = |id: &str| status.artifacts.iter().find(|a| a.id == id).unwrap().state;
        assert_eq!(state("proposal"), ArtifactState::Done);
        assert_eq!(state("specs"), ArtifactState::Ready);
        assert_eq!(state("tasks"), ArtifactState::Blocked);
    }

    #[test]
    fn a_nested_spec_satisfies_the_artifact_pattern() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file(
                "/p/_codev/changes/add-auth/change.yaml",
                "schema: spec-driven",
            )
            .with_file("/p/_codev/changes/add-auth/proposal.md", "x")
            .with_file(
                "/p/_codev/changes/add-auth/specs/identity/user-auth/spec.md",
                "y",
            );

        let ctx = context(&fs, "add-auth");
        let status = status(&fs, &Layout::new("/p"), &ctx).unwrap();
        let specs = status.artifacts.iter().find(|a| a.id == "specs").unwrap();
        assert_eq!(specs.state, ArtifactState::Done);
    }

    #[test]
    fn skip_specs_makes_tasks_writable_without_specs() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file(
                "/p/_codev/changes/refactor/change.yaml",
                "schema: spec-driven\nskip_specs: true\n",
            )
            .with_file("/p/_codev/changes/refactor/proposal.md", "x")
            .with_file("/p/_codev/changes/refactor/design.md", "y");

        let ctx = context(&fs, "refactor");
        let status = status(&fs, &Layout::new("/p"), &ctx).unwrap();
        let state = |id: &str| status.artifacts.iter().find(|a| a.id == id).unwrap().state;
        assert_eq!(state("specs"), ArtifactState::Skipped);
        assert_eq!(state("tasks"), ArtifactState::Ready);
    }

    #[test]
    fn lists_active_changes_without_the_archive() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file("/p/_codev/changes/add-auth/proposal.md", "x")
            .with_file("/p/_codev/changes/fix-bug/proposal.md", "x")
            .with_file("/p/_codev/changes/archive/2026-01-01-old/proposal.md", "x");

        let names: Vec<String> = list(&fs, &Layout::new("/p"))
            .iter()
            .map(ToString::to_string)
            .collect();
        assert_eq!(names, ["add-auth", "fix-bug"]);
    }
}
