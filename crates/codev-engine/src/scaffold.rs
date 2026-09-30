use codev_core::{ChangeId, Layout, Plan, WriteMode};

use crate::metadata::ChangeMetadata;

/// Fallback template for `_codev/config.yaml`.
///
/// **Residual role** — since the `init-interactive-with-detection` batch,
/// `codev init` itself generates a `config.yaml` prefilled from the probe
/// and the user's choices (`codev-core::config::render`). This template
/// only serves as a safety net, when the file has been deleted by hand:
/// `codev update` regenerates it exactly as this content, with the
/// workflows commented out (so, when read back, `DEFAULT_WORKFLOWS`
/// applies — the 7 skills).
const DEFAULT_CONFIG: &str = r#"# codev configuration for this project.
schema: spec-driven

# The workflows installed as Claude Code skills by `codev init` and
# `codev update`. When absent, the default catalog applies.
# workflows:
#   - propose
#   - explore
#   - onboard
#   - apply
#   - sync
#   - archive
#   - update

# Context injected into the instructions of ALL artifacts: what the agent
# must know about this project before writing anything.
# context: |
#   Tech stack: ...
#   API conventions: ...
#   Tests: ...

# Per-artifact rules, injected only for the artifact concerned.
# rules:
#   specs:
#     - Describe observable behavior, never an implementation.
#   design:
#     - Cite the decisions in _codev/decisions/ that constrain the approach.

# Inherited sources, read-only, from the most general to the most specific.
# inherits:
#   - path: ~/codev/shared

# Names of the MCP tools to use in skills. The name depends on the user's
# Claude Code config — uncomment and replace with the exact name of the MCP
# available in your session. Without this key, ticket detection in
# `/codev-propose` is inactive (silent fallback).
# mcp:
#   jira_tool: mcp__claude_ai_Atlassian_Rovo__getJiraIssue
"#;

/// Marker placed in directories that are still empty.
///
/// Git does not track empty directories: without this file, a `codev init`
/// followed by a commit would not pass the structure on to the team, and the
/// next colleague would wonder where the directories went.
const KEEP_FILE: &str = ".gitkeep";

const KEEP_CONTENT: &str =
    "# This file keeps the directory in git while it is empty. Delete it once it no longer is.\n";

/// The `codev init` plan: the `_codev/` structure and its configuration.
///
/// Skills are not part of it — `codev-agents` plans them, and the CLI
/// combines both plans. One crate, one responsibility.
///
/// Everything is [`WriteMode::CreateOnly`]: rerunning `init` on an already
/// initialized project must overwrite nothing, and therefore risk nothing.
pub fn plan_init(layout: &Layout) -> Plan {
    let mut plan = Plan::new();

    plan.dir(layout.planning_dir());
    for dir in [
        layout.specs_dir(),
        layout.decisions_dir(),
        layout.changes_dir(),
        layout.archive_dir(),
        layout.schemas_dir(),
    ] {
        plan.dir(&dir);
        plan.write(dir.join(KEEP_FILE), KEEP_CONTENT, WriteMode::CreateOnly);
    }

    plan.write(layout.config_file(), DEFAULT_CONFIG, WriteMode::CreateOnly);
    plan
}

/// The `codev new change` plan.
pub fn plan_new_change(layout: &Layout, change: &ChangeId, metadata: &ChangeMetadata) -> Plan {
    let mut plan = Plan::new();
    plan.dir(layout.change_dir(change));
    plan.write(
        layout.change_metadata(change),
        metadata.to_yaml(),
        WriteMode::CreateOnly,
    );
    plan
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config;
    use crate::ports::{FixedEnv, MemoryFileSystem};
    use std::path::{Path, PathBuf};

    #[test]
    fn init_prepares_the_full_structure() {
        let plan = plan_init(&Layout::new("/p"));

        for expected in [
            "/p/_codev",
            "/p/_codev/specs",
            "/p/_codev/decisions",
            "/p/_codev/changes",
            "/p/_codev/changes/archive",
            "/p/_codev/schemas",
        ] {
            assert!(
                plan.dirs.contains(&PathBuf::from(expected)),
                "{expected} should be planned"
            );
        }
        assert!(
            plan.writes
                .iter()
                .any(|w| w.path == Path::new("/p/_codev/config.yaml"))
        );
    }

    #[test]
    fn init_never_overwrites_anything() {
        let plan = plan_init(&Layout::new("/p"));
        assert!(
            plan.writes.iter().all(|w| w.mode == WriteMode::CreateOnly),
            "no `init` write may overwrite a file"
        );
    }

    #[test]
    fn default_config_is_valid_and_readable() {
        // The file we write must pass our own reader, otherwise the first
        // `codev status` after an `init` would fail.
        let fs = MemoryFileSystem::new().with_file("/p/_codev/config.yaml", DEFAULT_CONFIG);
        let mut env = FixedEnv::at("/p");
        env.vars.insert("HOME".into(), "/home".into());
        let resolved = config::resolve(&fs, &env, &Layout::new("/p")).unwrap();
        assert_eq!(resolved.schema, "spec-driven");
        assert!(resolved.warnings.is_empty());
    }

    #[test]
    fn new_change_writes_the_metadata() {
        let change = ChangeId::parse("add-auth").unwrap();
        let metadata = ChangeMetadata::new("spec-driven", "2026-09-08");
        let plan = plan_new_change(&Layout::new("/p"), &change, &metadata);

        assert_eq!(plan.dirs, [PathBuf::from("/p/_codev/changes/add-auth")]);
        assert_eq!(plan.writes.len(), 1);
        let write = &plan.writes[0];
        assert_eq!(
            write.path,
            PathBuf::from("/p/_codev/changes/add-auth/change.yaml")
        );
        assert!(write.contents.contains("schema: spec-driven"));
        assert_eq!(write.mode, WriteMode::CreateOnly);
    }
}
