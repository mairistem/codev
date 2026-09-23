use codev_core::Layout;

use crate::ports::FileSystem;

const SPEC_FILE: &str = "spec.md";

/// Les capacités spécifiées, par leur chemin relatif à `specs/`.
///
/// Une capacité est un dossier contenant un `spec.md` — les chemins imbriqués
/// (`identity/user-auth`) comptent, parce que l'organisation des specs
/// appartient au projet et non à l'outil.
pub fn list(fs: &dyn FileSystem, layout: &Layout) -> Vec<String> {
    let Ok(files) = fs.walk_files(&layout.specs_dir()) else {
        return Vec::new();
    };
    let mut capabilities: Vec<String> = files
        .iter()
        .filter_map(|relative| match relative.strip_suffix(SPEC_FILE) {
            // `specs/spec.md` — une spec à la racine, sans capacité nommée.
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
    fn liste_les_capacites_y_compris_imbriquees() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/specs/user-auth/spec.md", "x")
            .with_file("/p/_codev/specs/identity/sso/spec.md", "x")
            .with_file("/p/_codev/specs/.gitkeep", "");

        assert_eq!(
            list(&fs, &Layout::new("/p")),
            ["identity/sso", "user-auth"]
        );
    }

    #[test]
    fn ignore_ce_qui_nest_pas_une_spec() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/specs/notes.md", "x")
            .with_file("/p/_codev/specs/user-auth/README.md", "x");
        assert!(list(&fs, &Layout::new("/p")).is_empty());
    }

    #[test]
    fn un_projet_sans_specs_ne_donne_rien() {
        let fs = MemoryFileSystem::new();
        assert!(list(&fs, &Layout::new("/p")).is_empty());
    }
}
