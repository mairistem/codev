use std::path::Path;

use codev_core::Layout;

use crate::error::{EngineError, Result};
use crate::ports::{Env, FileSystem};

/// Finds the project root by walking up from `start`.
///
/// A single rule: the first ancestor that contains a `_codev/` directory
/// wins. No project registry, no environment variable, no `--store` flag:
/// this is how `git` behaves, and it needs no explanation.
pub fn discover(fs: &dyn FileSystem, start: &Path) -> Result<Layout> {
    for ancestor in start.ancestors() {
        let layout = Layout::new(ancestor);
        if fs.exists(&layout.planning_dir()) {
            return Ok(layout);
        }
    }
    Err(EngineError::NoRoot {
        from: start.to_path_buf(),
    })
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
        let fs = MemoryFileSystem::new().with_file("/p/_codev/config.yaml", "");
        let layout = discover(fs_ref(&fs), Path::new("/p/crates/thing/src")).unwrap();
        assert_eq!(layout.project_root(), Path::new("/p"));
    }

    #[test]
    fn finds_the_root_in_place() {
        let fs = MemoryFileSystem::new().with_file("/p/_codev/config.yaml", "");
        let layout = discover(fs_ref(&fs), Path::new("/p")).unwrap();
        assert_eq!(layout.project_root(), Path::new("/p"));
    }

    #[test]
    fn fails_naming_the_starting_point() {
        let fs = MemoryFileSystem::new();
        let err = discover(fs_ref(&fs), Path::new("/elsewhere/here")).unwrap_err();
        assert_eq!(err.code(), "no_codev_root");
        assert!(err.to_string().contains("/elsewhere/here"), "{err}");
        assert!(err.to_string().contains("codev init"), "{err}");
    }

    #[test]
    fn picks_the_nearest_root() {
        // A repository nested inside another: the work belongs to the nearest
        // one, never to the parent.
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file("/p/sub-project/_codev/config.yaml", "");
        let layout = discover(fs_ref(&fs), Path::new("/p/sub-project/src")).unwrap();
        assert_eq!(layout.project_root(), Path::new("/p/sub-project"));
    }

    fn fs_ref(fs: &MemoryFileSystem) -> &dyn FileSystem {
        fs
    }
}
