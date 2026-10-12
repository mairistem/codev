use codev_core::Layout;

use crate::ports::FileSystem;

const SPEC_FILE: &str = "spec.md";

/// The specified capabilities, by their path relative to `specs/`.
///
/// A capability is a directory containing a file named exactly `spec.md`;
/// any other file is ignored, not reported. Nested paths
/// (`identity/user-auth`) count, because how specs are organized belongs to
/// the project, not to the tool.
pub fn list(fs: &dyn FileSystem, layout: &Layout) -> Vec<String> {
    let Ok(files) = fs.walk_files(&layout.specs_dir()) else {
        return Vec::new();
    };
    let mut capabilities: Vec<String> = files
        .iter()
        // `walk_files` returns `/`-separated paths. Only a file named
        // exactly `spec.md` is a spec: matching the text suffix alone turned
        // `notes/myspec.md` into a capability `notes/my`. A `spec.md` at the
        // root of `specs/` has no folder, hence no capability.
        .filter_map(|relative| {
            let (folder, file) = relative.rsplit_once('/')?;
            (file == SPEC_FILE).then(|| folder.to_string())
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
    fn a_file_whose_name_ends_with_spec_md_is_not_a_capability() {
        // `notes/myspec.md` used to become the capability `notes/my`, whose
        // `spec.md` does not exist — and `validate` then failed reading it.
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/specs/user-auth/spec.md", "x")
            .with_file("/p/_codev/specs/notes/myspec.md", "x")
            .with_file("/p/_codev/specs/draftspec.md", "x");
        assert_eq!(list(&fs, &Layout::new("/p")), ["user-auth"]);
    }

    #[test]
    fn a_project_without_specs_yields_nothing() {
        let fs = MemoryFileSystem::new();
        assert!(list(&fs, &Layout::new("/p")).is_empty());
    }
}
