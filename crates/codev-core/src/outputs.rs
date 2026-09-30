use globset::{Glob, GlobSetBuilder};

use crate::error::{CoreError, Result};

/// True if any of the given paths satisfies an artifact's output pattern.
///
/// `relative_paths` is relative to the change directory, with `/` separators.
/// Walking the disk is an effect: it belongs to the caller, in
/// `codev-engine`. Here, only the matching — hence testable without
/// any file.
pub fn pattern_matches_any(pattern: &str, relative_paths: &[String]) -> Result<bool> {
    let glob = Glob::new(pattern).map_err(|e| CoreError::InvalidOutputPattern {
        pattern: pattern.to_string(),
        reason: e.to_string(),
    })?;
    let set =
        GlobSetBuilder::new()
            .add(glob)
            .build()
            .map_err(|e| CoreError::InvalidOutputPattern {
                pattern: pattern.to_string(),
                reason: e.to_string(),
            })?;
    Ok(relative_paths.iter().any(|p| set.is_match(p)))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn paths(items: &[&str]) -> Vec<String> {
        items.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn recognizes_a_nested_spec() {
        let present = paths(&["proposal.md", "specs/identity/user-auth/spec.md"]);
        assert!(pattern_matches_any("specs/**/*.md", &present).unwrap());
    }

    #[test]
    fn does_not_mistake_an_empty_directory_for_an_output() {
        let present = paths(&["proposal.md"]);
        assert!(!pattern_matches_any("specs/**/*.md", &present).unwrap());
    }

    #[test]
    fn ignores_a_file_outside_the_pattern() {
        // A `README.md` dropped at the change root must not suggest
        // that the specs have been written.
        let present = paths(&["README.md", "specs/notes.txt"]);
        assert!(!pattern_matches_any("specs/**/*.md", &present).unwrap());
    }

    #[test]
    fn rejects_an_unreadable_pattern() {
        let err = pattern_matches_any("specs/[", &paths(&["a.md"])).unwrap_err();
        assert_eq!(err.code(), "invalid_output_pattern");
    }
}
