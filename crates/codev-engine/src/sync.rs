//! `sync` orchestration: reads a change's deltas, computes a merge plan for
//! each affected capability, and aggregates them into a single [`Plan`].
//!
//! No merge rules here — they live in `codev-core::merge`. This module
//! coordinates, decides "create or update", and tells "unchanged" apart
//! from "updated" to make `sync` idempotent in the strict sense.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use codev_core::merge::{MergeError, apply_edits, build_new_spec, merge_into_existing};
use codev_core::parser::{parse_delta, parse_spec};
use codev_core::{ChangeId, Layout, Plan, WriteMode};

use crate::apply;
use crate::config::ResolvedConfig;
use crate::error::{EngineError, Result};
use crate::ports::FileSystem;

/// The complete plan of a `sync`: which `Plan` to execute, and the future
/// "report" filled in at execution time (at this stage the created and
/// updated specs are known; "unchanged" is inferred after application).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyncPlan {
    pub plan: Plan,
    /// Paths of the main specs that will be created.
    pub creates: Vec<PathBuf>,
    /// Paths of the main specs that will be rewritten (content genuinely
    /// different after applying the edits).
    pub updates: Vec<PathBuf>,
    /// Paths of the main specs already up to date — the sync would have no effect.
    pub unchanged: Vec<PathBuf>,
    /// Paths of the main specs to delete — the capability is retired
    /// by a `## REMOVED Requirements` that would empty the spec, combined
    /// with `retire_capabilities: true` in `change.yaml`.
    pub deleted: Vec<PathBuf>,
}

/// What the sync actually did on disk.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct SyncOutcome {
    pub change: String,
    pub root: PathBuf,
    pub updated: Vec<PathBuf>,
    pub created: Vec<PathBuf>,
    pub unchanged: Vec<PathBuf>,
    /// Deleted main specs (retired capabilities). Empty in the common case.
    pub deleted: Vec<PathBuf>,
}

impl SyncOutcome {
    pub fn changed_anything(&self) -> bool {
        !self.updated.is_empty() || !self.created.is_empty() || !self.deleted.is_empty()
    }
}

/// Builds the plan without writing.
///
/// Public function: `archive` uses it to compose a larger plan
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

    // Load the change context to know `retire_capabilities` — without it,
    // a total REMOVED cannot be told apart from an error.
    let ctx = crate::change::load(fs, layout, config, change_id.clone())?;

    let mut plan = Plan::new();
    let mut creates = Vec::new();
    let mut updates = Vec::new();
    let mut unchanged = Vec::new();
    let mut deleted = Vec::new();
    let mut seen: BTreeSet<PathBuf> = BTreeSet::new();

    for delta_path in locate_delta_files(fs, &change_dir)? {
        let capability = capability_from_delta_path(&change_dir, &delta_path).ok_or_else(|| {
            EngineError::Invalid {
                path: delta_path.clone(),
                reason: "delta path outside `changes/<name>/specs/`".into(),
            }
        })?;

        let delta_source = fs
            .read_to_string(&delta_path)
            .map_err(|e| EngineError::Unreadable {
                path: delta_path.clone(),
                reason: e.to_string(),
            })?;
        let delta_parsed = parse_delta(&delta_source);
        if delta_parsed.has_errors() {
            // Refuse to sync if the delta has an error-level finding —
            // reporting it is the validator's job, but a broken delta would
            // corrupt the merge.
            return Err(EngineError::Invalid {
                path: delta_path.clone(),
                reason: format!(
                    "delta contains {} error finding(s); run `codev validate` first",
                    delta_parsed.findings.len()
                ),
            });
        }
        let delta = delta_parsed.value;

        let main_spec_path = layout.spec_file(&capability);
        if !seen.insert(main_spec_path.clone()) {
            // The same main spec touched by two deltas of the same change
            // (through two different files) — an unusual case, but we merge
            // only once to avoid contradictory edits.
            continue;
        }

        if fs.exists(&main_spec_path) {
            let existing_source =
                fs.read_to_string(&main_spec_path)
                    .map_err(|e| EngineError::Unreadable {
                        path: main_spec_path.clone(),
                        reason: e.to_string(),
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
                // The capability is retired: neither updated nor unchanged,
                // the whole spec goes away.
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
            let contents =
                build_new_spec(&capability, &delta).map_err(|e| EngineError::Invalid {
                    path: main_spec_path.clone(),
                    reason: format!("{} ({})", e.message(), e.code()),
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

/// Executes the sync plan.
pub fn execute_sync(
    fs: &dyn FileSystem,
    layout: &Layout,
    config: &ResolvedConfig,
    change_id: &ChangeId,
) -> Result<SyncOutcome> {
    let sync_plan = plan_sync(fs, layout, config, change_id)?;
    let applied = apply::execute(&sync_plan.plan, fs)?;
    // `applied.created` covers the creates; `applied.overwritten` covers
    // the updates; `applied.untouched` (rare here, but possible) joins
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

/// The `*.md` files under `changes/<name>/specs/`, sorted.
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

/// Extracts `<capability-path>` from `changes/<name>/specs/<capability-path>/spec.md`.
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

/// Reminder: `MergeError` — the core's pure error type — is translated here
/// into [`EngineError::Invalid`] so as not to leak internal naming into the
/// public contract. `has_error` stays stable; the error code is preserved in
/// the message so that the JSON consumer can recover it.
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

    /// A minimal project: a change with an ADDED delta on a new capability
    /// `user-auth`. No existing main spec.
    fn project_with_added_delta() -> MemoryFileSystem {
        MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file(
                "/p/_codev/changes/add-auth/change.yaml",
                "schema: spec-driven",
            )
            .with_file(
                "/p/_codev/changes/add-auth/specs/user-auth/spec.md",
                "## Purpose\n\nHandles authentication.\n\n## ADDED Requirements\n\n### Requirement: Login\nThe system SHALL emit a token.\n\n#### Scenario: OK\n- **WHEN** login\n- **THEN** token\n",
            )
    }

    #[test]
    fn plan_sync_produces_one_write_per_affected_capability() {
        let fs = project_with_added_delta();
        let layout = Layout::new("/p");
        let cfg = resolved(&fs);
        let plan = plan_sync(&fs, &layout, &cfg, &ChangeId::parse("add-auth").unwrap()).unwrap();

        assert_eq!(plan.creates.len(), 1);
        assert_eq!(plan.updates.len(), 0);
        assert_eq!(plan.plan.writes.len(), 1);
        assert_eq!(
            plan.creates[0],
            PathBuf::from("/p/_codev/specs/user-auth/spec.md")
        );
    }

    #[test]
    fn sync_of_a_multi_capability_delta_succeeds() {
        // Two affected capabilities: one created (Purpose + ADDED), the
        // other already existing and modified.
        let fs = project_with_added_delta()
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

        assert_eq!(outcome.created.len(), 1, "user-auth created");
        assert_eq!(outcome.updated.len(), 1, "session updated");
        // On disk, everything is written.
        assert!(
            fs.read("/p/_codev/specs/user-auth/spec.md")
                .is_some_and(|s| s.contains("### Requirement: Login"))
        );
        assert!(
            fs.read("/p/_codev/specs/session/spec.md")
                .is_some_and(|s| s.contains("15 minutes"))
        );
    }

    #[test]
    fn second_sync_changes_nothing() {
        // The idempotency invariant: syncing twice in a row leaves the
        // second one with `changed_anything() == false`.
        let fs = project_with_added_delta();
        let layout = Layout::new("/p");
        let cfg = resolved(&fs);
        let change = ChangeId::parse("add-auth").unwrap();

        let first = execute_sync(&fs, &layout, &cfg, &change).unwrap();
        assert!(first.changed_anything());

        let second = execute_sync(&fs, &layout, &cfg, &change).unwrap();
        assert!(
            !second.changed_anything(),
            "sync must be idempotent; observed: {:?}",
            second
        );
    }

    #[test]
    fn sync_of_a_modified_delta_without_target_fails_with_a_useful_code() {
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
                "## MODIFIED Requirements\n\n### Requirement: Ghost\nThe system SHALL y.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n",
            );
        let layout = Layout::new("/p");
        let cfg = resolved(&fs);
        let err =
            execute_sync(&fs, &layout, &cfg, &ChangeId::parse("broken").unwrap()).unwrap_err();
        assert_eq!(err.code(), "invalid");
        assert!(err.to_string().contains("modified_target_missing"), "{err}");
        // Nothing was written.
        assert!(
            fs.read("/p/_codev/specs/x/spec.md")
                .is_some_and(|s| s.contains("Login") && !s.contains("Ghost"))
        );
    }

    #[test]
    fn capability_is_extracted_from_the_path_even_when_nested() {
        let change_dir = Path::new("/p/_codev/changes/add-auth");
        let delta = Path::new("/p/_codev/changes/add-auth/specs/identity/user-auth/spec.md");
        assert_eq!(
            capability_from_delta_path(change_dir, delta),
            Some("identity/user-auth".into())
        );
    }

    // ─────────────── retire_capabilities ───────────────

    fn project_with_retire_capabilities(flag: bool) -> MemoryFileSystem {
        let yaml = if flag {
            "schema: spec-driven\nretire_capabilities: true\n"
        } else {
            "schema: spec-driven\n"
        };
        MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            // Existing main spec with a single Solo requirement.
            .with_file(
                "/p/_codev/specs/user-auth/spec.md",
                "## Purpose\n\nHandles auth.\n\n## Requirements\n\n### Requirement: Solo\nThe system SHALL x.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n",
            )
            // Change that retires it entirely.
            .with_file("/p/_codev/changes/retire-auth/change.yaml", yaml)
            .with_file(
                "/p/_codev/changes/retire-auth/specs/user-auth/spec.md",
                "## REMOVED Requirements\n\n### Requirement: Solo\n**Reason**: obsolete\n**Migration**: none\n",
            )
    }

    #[test]
    fn sync_retire_capability_with_flag_deletes_the_spec() {
        let fs = project_with_retire_capabilities(true);
        let layout = Layout::new("/p");
        let cfg = resolved(&fs);

        let outcome =
            execute_sync(&fs, &layout, &cfg, &ChangeId::parse("retire-auth").unwrap()).unwrap();

        assert_eq!(
            outcome.deleted,
            vec![PathBuf::from("/p/_codev/specs/user-auth/spec.md")]
        );
        assert!(outcome.updated.is_empty());
        assert!(outcome.unchanged.is_empty());
        assert!(!fs.exists(std::path::Path::new("/p/_codev/specs/user-auth/spec.md")));
    }

    #[test]
    fn sync_retire_capability_without_flag_is_rejected() {
        let fs = project_with_retire_capabilities(false);
        let layout = Layout::new("/p");
        let cfg = resolved(&fs);

        let err =
            execute_sync(&fs, &layout, &cfg, &ChangeId::parse("retire-auth").unwrap()).unwrap_err();
        assert_eq!(err.code(), "invalid");
        assert!(
            err.to_string()
                .contains("would_leave_spec_without_requirement")
        );
        // The file still exists.
        assert!(fs.exists(std::path::Path::new("/p/_codev/specs/user-auth/spec.md")));
    }
}
