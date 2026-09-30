use std::path::PathBuf;

use codev_core::{Plan, WriteMode};

use crate::error::{EngineError, Result};
use crate::ports::FileSystem;

/// What a plan execution actually did.
///
/// Telling the three cases apart is not cosmetic: it is what lets `init`
/// say "nothing to do" rather than "12 files written" on an already
/// initialized project.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Applied {
    pub created: Vec<PathBuf>,
    pub overwritten: Vec<PathBuf>,
    /// Already up to date, or protected by [`WriteMode::CreateOnly`].
    pub untouched: Vec<PathBuf>,
    /// Deletions actually applied. A file that is missing at deletion
    /// time is not listed (this is not an error — the change may already
    /// have been applied).
    pub deleted: Vec<PathBuf>,
    /// Moves performed — `(from, to)`.
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

/// Executes a plan.
///
/// The imperative shell: no decisions here, only writes. Every choice has
/// been settled upstream, in a pure function.
pub fn execute(plan: &Plan, fs: &dyn FileSystem) -> Result<Applied> {
    let mut applied = Applied::default();

    for dir in &plan.dirs {
        fs.create_dir_all(dir)
            .map_err(|source| EngineError::Write {
                path: dir.clone(),
                source,
            })?;
    }

    for write in &plan.writes {
        let exists = fs.exists(&write.path);
        match write.mode {
            WriteMode::CreateOnly if exists => {
                applied.untouched.push(write.path.clone());
                continue;
            }
            WriteMode::Overwrite if exists => {
                // Compare before writing: otherwise `codev update` would report
                // having modified identical files, and the user would no
                // longer know what actually changed.
                let identical = fs
                    .read_to_string(&write.path)
                    .map(|current| current == write.contents)
                    .unwrap_or(false);
                if identical {
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

    // Deletions come after writes — "write what is new, remove what no
    // longer belongs" — and before moves, to keep the destructive step
    // atomic. A file missing at deletion time is not an error — the
    // change may already have been applied, or the file may have been
    // deleted by hand between planning and execution.
    for path in &plan.deletions {
        match fs.remove_file(path) {
            Ok(()) => applied.deleted.push(path.clone()),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                // Idempotent — no trace in `applied.deleted`.
            }
            Err(source) => {
                return Err(EngineError::Write {
                    path: path.clone(),
                    source,
                });
            }
        }
    }

    // Moves last. A `Move` may need a destination directory to exist
    // (covered by the `dirs` loop) and a source file to have been written
    // at the location about to be moved (covered by the `writes` loop).
    for mv in &plan.moves {
        fs.rename(&mv.from, &mv.to)
            .map_err(|source| EngineError::Write {
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
    fn creates_what_is_missing() {
        let fs = MemoryFileSystem::new();
        let mut plan = Plan::new();
        plan.dir("/p/_codev").write(
            "/p/_codev/config.yaml",
            "schema: spec-driven",
            WriteMode::CreateOnly,
        );

        let applied = execute(&plan, &fs).unwrap();

        assert_eq!(applied.created, [PathBuf::from("/p/_codev/config.yaml")]);
        assert!(applied.untouched.is_empty());
        assert_eq!(
            fs.read("/p/_codev/config.yaml").as_deref(),
            Some("schema: spec-driven")
        );
    }

    #[test]
    fn create_only_protects_the_users_work() {
        let fs = MemoryFileSystem::new().with_file("/p/_codev/config.yaml", "my own config");
        let mut plan = Plan::new();
        plan.write(
            "/p/_codev/config.yaml",
            "generated config",
            WriteMode::CreateOnly,
        );

        let applied = execute(&plan, &fs).unwrap();

        assert_eq!(applied.untouched, [PathBuf::from("/p/_codev/config.yaml")]);
        assert!(!applied.changed_anything());
        assert_eq!(
            fs.read("/p/_codev/config.yaml").as_deref(),
            Some("my own config"),
            "existing content must be left intact"
        );
    }

    #[test]
    fn overwrite_reports_only_real_changes() {
        let fs = MemoryFileSystem::new().with_file("/s/SKILL.md", "identical");
        let mut plan = Plan::new();
        plan.write("/s/SKILL.md", "identical", WriteMode::Overwrite);

        let applied = execute(&plan, &fs).unwrap();

        assert!(applied.overwritten.is_empty(), "nothing changed");
        assert_eq!(applied.untouched, [PathBuf::from("/s/SKILL.md")]);
    }

    #[test]
    fn overwrite_replaces_different_content() {
        let fs = MemoryFileSystem::new().with_file("/s/SKILL.md", "old version");
        let mut plan = Plan::new();
        plan.write("/s/SKILL.md", "new version", WriteMode::Overwrite);

        let applied = execute(&plan, &fs).unwrap();

        assert_eq!(applied.overwritten, [PathBuf::from("/s/SKILL.md")]);
        assert_eq!(fs.read("/s/SKILL.md").as_deref(), Some("new version"));
    }

    #[test]
    fn execute_moves_after_writing() {
        // Order matters: the `Move` only works if the source file has just
        // been written — this is the contract `archive` relies on to move a
        // directory it may have just completed.
        let fs = MemoryFileSystem::new();
        let mut plan = Plan::new();
        plan.write("/p/a/file.md", "x", WriteMode::Overwrite);
        plan.move_dir("/p/a", "/p/b");

        let applied = execute(&plan, &fs).unwrap();

        assert_eq!(applied.created, [PathBuf::from("/p/a/file.md")]);
        assert_eq!(
            applied.moved,
            [(PathBuf::from("/p/a"), PathBuf::from("/p/b"))]
        );
        assert!(!fs.exists(Path::new("/p/a/file.md")));
        assert_eq!(fs.read("/p/b/file.md").as_deref(), Some("x"));
    }

    #[test]
    fn execute_move_alone_changes_state() {
        let fs = MemoryFileSystem::new().with_file("/p/x", "content");
        let mut plan = Plan::new();
        plan.move_dir("/p/x", "/q/x");

        let applied = execute(&plan, &fs).unwrap();

        assert!(
            applied.changed_anything(),
            "a move alone counts as a real change"
        );
        assert_eq!(applied.moved.len(), 1);
    }

    #[test]
    fn execute_move_with_missing_source_is_a_typed_error() {
        let fs = MemoryFileSystem::new();
        let mut plan = Plan::new();
        plan.move_dir("/not/there", "/elsewhere");

        let err = execute(&plan, &fs).unwrap_err();
        assert_eq!(err.code(), "write_failed");
    }

    #[test]
    fn deletion_removes_an_existing_file() {
        let fs = MemoryFileSystem::new().with_file("/p/a.md", "x");
        let mut plan = Plan::new();
        plan.delete("/p/a.md");

        let applied = execute(&plan, &fs).unwrap();
        assert_eq!(applied.deleted, [PathBuf::from("/p/a.md")]);
        assert!(!fs.exists(Path::new("/p/a.md")));
    }

    #[test]
    fn deletion_of_missing_file_is_silent() {
        // Idempotence — a plan rerun after deletion raises no error and
        // reports nothing.
        let fs = MemoryFileSystem::new();
        let mut plan = Plan::new();
        plan.delete("/p/absent.md");

        let applied = execute(&plan, &fs).unwrap();
        assert!(applied.deleted.is_empty());
        assert!(!applied.changed_anything());
    }

    #[test]
    fn deletion_after_writes_and_before_moves() {
        // Order required by the design: dirs → writes → deletions → moves.
        let fs = MemoryFileSystem::new()
            .with_file("/p/old.md", "to remove")
            .with_file("/p/folder/new.md", "will be written first");
        let mut plan = Plan::new();
        plan.write("/p/folder/new.md", "modified", WriteMode::Overwrite);
        plan.delete("/p/old.md");
        plan.move_dir("/p/folder", "/p/folder-renamed");

        let applied = execute(&plan, &fs).unwrap();

        // The write did happen (in the original directory, before the move).
        assert_eq!(applied.overwritten, [PathBuf::from("/p/folder/new.md")]);
        assert_eq!(applied.deleted, [PathBuf::from("/p/old.md")]);
        assert_eq!(applied.moved.len(), 1);
        assert!(!fs.exists(Path::new("/p/old.md")));
        assert!(fs.exists(Path::new("/p/folder-renamed/new.md")));
    }

    #[test]
    fn replaying_a_plan_has_no_effect() {
        // Idempotence is what makes `codev init` and `codev update`
        // safe to rerun without a second thought.
        let fs = MemoryFileSystem::new();
        let mut plan = Plan::new();
        plan.write("/p/a.md", "x", WriteMode::CreateOnly).write(
            "/p/b.md",
            "y",
            WriteMode::Overwrite,
        );

        assert!(execute(&plan, &fs).unwrap().changed_anything());
        assert!(!execute(&plan, &fs).unwrap().changed_anything());
    }
}
