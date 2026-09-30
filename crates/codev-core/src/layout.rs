use std::path::{Path, PathBuf};

use crate::id::ChangeId;

/// Name of the planning directory.
///
/// A single project-wide constant: changing it costs a recompile, not a
/// refactor. The `_` prefix keeps it **visible** — search tools
/// (`ripgrep`, `fd`, and therefore Claude Code's) ignore hidden directories
/// by default, and a source of truth the agent cannot find is
/// useless. See `_codev/decisions/0003-racine-de-planification-_codev.md`.
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

/// Every "where does this file live?" question, in one place.
///
/// Path algebra, hence pure: none of these methods touch the
/// disk, and none checks that what it names exists.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Layout {
    project_root: PathBuf,
}

impl Layout {
    /// `project_root` is the directory **containing** `_codev/`, not `_codev/`
    /// itself.
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

    /// Seal file attesting to the body of local ADRs at the time of
    /// their acceptance — see `codev-core::decisions::seal`.
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

    /// The directory of an archived change, prefixed with its date for
    /// chronological sorting.
    pub fn archived_change_dir(&self, change: &ChangeId, date: &str) -> PathBuf {
        self.archive_dir().join(format!("{date}-{change}"))
    }

    /// The directory of a project-specific schema.
    pub fn project_schema_dir(&self, name: &str) -> PathBuf {
        self.schemas_dir().join(name)
    }

    /// The main spec of a capability, `capability_path` being relative to `specs/`.
    pub fn spec_file(&self, capability_path: &str) -> PathBuf {
        self.specs_dir().join(capability_path).join("spec.md")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_paths_from_the_project_root() {
        let layout = Layout::new("/tmp/project");
        let change = ChangeId::parse("add-auth").unwrap();

        assert_eq!(layout.planning_dir(), Path::new("/tmp/project/_codev"));
        assert_eq!(
            layout.config_file(),
            Path::new("/tmp/project/_codev/config.yaml")
        );
        assert_eq!(
            layout.change_dir(&change),
            Path::new("/tmp/project/_codev/changes/add-auth")
        );
        assert_eq!(
            layout.change_metadata(&change),
            Path::new("/tmp/project/_codev/changes/add-auth/change.yaml")
        );
        assert_eq!(
            layout.archived_change_dir(&change, "2026-09-08"),
            Path::new("/tmp/project/_codev/changes/archive/2026-09-08-add-auth")
        );
        assert_eq!(
            layout.spec_file("identity/user-auth"),
            Path::new("/tmp/project/_codev/specs/identity/user-auth/spec.md")
        );
    }
}
