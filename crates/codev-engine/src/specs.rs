use codev_core::Layout;

use crate::ports::FileSystem;

const SPEC_FILE: &str = "spec.md";

/// The specified capabilities, by their path relative to `specs/`.
///
/// A capability is a directory containing a `spec.md` — nested paths
/// (`identity/user-auth`) count, because how specs are organized belongs to
/// the project, not to the tool.
pub fn list(fs: &dyn FileSystem, layout: &Layout) -> Vec<String> {
    let Ok(files) = fs.walk_files(&layout.specs_dir()) else {
        return Vec::new();
    };
    let mut capabilities: Vec<String> = files
        .iter()
        .filter_map(|relative| match relative.strip_suffix(SPEC_FILE) {
            // `specs/spec.md` — a spec at the root, with no named capability.
            Some("") => None,
            Some(prefix) => Some(prefix.trim_end_matches('/').to_string()),
            None => None,
        })
        .collect();
    capabilities.sort();
    capabilities.dedup();
    capabilities
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ports::MemoryFileSystem;

    #[test]
    fn lists_capabilities_including_nested_ones() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/specs/user-auth/spec.md", "x")
            .with_file("/p/_codev/specs/identity/sso/spec.md", "x")
            .with_file("/p/_codev/specs/.gitkeep", "");

        assert_eq!(list(&fs, &Layout::new("/p")), ["identity/sso", "user-auth"]);
    }

    #[test]
    fn ignores_what_is_not_a_spec() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/specs/notes.md", "x")
            .with_file("/p/_codev/specs/user-auth/README.md", "x");
        assert!(list(&fs, &Layout::new("/p")).is_empty());
    }

    #[test]
    fn a_project_without_specs_yields_nothing() {
        let fs = MemoryFileSystem::new();
        assert!(list(&fs, &Layout::new("/p")).is_empty());
    }
}
