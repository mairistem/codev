use std::path::Path;

use compagnon_core::{Layout, legacy};

use crate::error::{EngineError, Result};
use crate::ports::{Env, FileSystem};

/// Finds the project root by walking up from `start`.
///
/// A single rule: the first ancestor that contains a `_compagnon/` directory
/// wins. No project registry, no environment variable, no `--store` flag:
/// this is how `git` behaves, and it needs no explanation.
///
/// An ancestor holding only the former planning root is a project set up
/// before the rename: it is refused with `legacy_root`, so that no command
/// reads or writes a layout the user has not converted with
/// `compagnon migrate`.
pub fn discover(fs: &dyn FileSystem, start: &Path) -> Result<Layout> {
    for ancestor in start.ancestors() {
        let layout = Layout::new(ancestor);
        if fs.exists(&layout.planning_dir()) {
            return Ok(layout);
        }
        if holds_former_root(fs, ancestor) {
            return Err(EngineError::LegacyRoot {
                dir: ancestor.to_path_buf(),
            });
        }
    }
    Err(EngineError::NoRoot {
        from: start.to_path_buf(),
    })
}

/// Whether `dir` holds the planning root of the product's former name.
pub fn holds_former_root(fs: &dyn FileSystem, dir: &Path) -> bool {
    fs.exists(&dir.join(legacy::PLANNING_DIR))
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
    fn walks_up_to_the_root() {
        let fs = MemoryFileSystem::new().with_file("/p/_compagnon/config.yaml", "");
        let layout = discover(fs_ref(&fs), Path::new("/p/crates/thing/src")).unwrap();
        assert_eq!(layout.project_root(), Path::new("/p"));
    }

    #[test]
    fn finds_the_root_in_place() {
        let fs = MemoryFileSystem::new().with_file("/p/_compagnon/config.yaml", "");
        let layout = discover(fs_ref(&fs), Path::new("/p")).unwrap();
        assert_eq!(layout.project_root(), Path::new("/p"));
    }

    #[test]
    fn fails_naming_the_starting_point() {
        let fs = MemoryFileSystem::new();
        let err = discover(fs_ref(&fs), Path::new("/elsewhere/here")).unwrap_err();
        assert_eq!(err.code(), "no_compagnon_root");
        assert!(err.to_string().contains("/elsewhere/here"), "{err}");
        assert!(err.to_string().contains("compagnon init"), "{err}");
    }

    #[test]
    fn picks_the_nearest_root() {
        // A repository nested inside another: the work belongs to the nearest
        // one, never to the parent.
        let fs = MemoryFileSystem::new()
            .with_file("/p/_compagnon/config.yaml", "")
            .with_file("/p/sub-project/_compagnon/config.yaml", "");
        let layout = discover(fs_ref(&fs), Path::new("/p/sub-project/src")).unwrap();
        assert_eq!(layout.project_root(), Path::new("/p/sub-project"));
    }

    fn former(path: &str) -> String {
        format!("/p/{}/{path}", legacy::PLANNING_DIR)
    }

    #[test]
    fn a_codev_root_is_refused_with_legacy_root() {
        let fs = MemoryFileSystem::new().with_file(former("config.yaml"), "");
        let err = discover(fs_ref(&fs), Path::new("/p/src")).unwrap_err();
        assert_eq!(err.code(), "legacy_root");
        assert!(err.to_string().contains("/p"), "{err}");
        assert!(err.to_string().contains("compagnon migrate"), "{err}");
    }

    #[test]
    fn compagnon_root_wins_over_a_leftover_codev_root() {
        let fs = MemoryFileSystem::new()
            .with_file(former("config.yaml"), "")
            .with_file("/p/_compagnon/config.yaml", "");
        let layout = discover(fs_ref(&fs), Path::new("/p/src")).unwrap();
        assert_eq!(layout.planning_dir(), Path::new("/p/_compagnon"));
    }

    #[test]
    fn no_root_at_all_fails_with_no_compagnon_root() {
        let fs = MemoryFileSystem::new().with_file("/p/src/main.rs", "");
        let err = discover(fs_ref(&fs), Path::new("/p/src")).unwrap_err();
        assert_eq!(err.code(), "no_compagnon_root");
    }

    fn fs_ref(fs: &MemoryFileSystem) -> &dyn FileSystem {
        fs
    }
}
