//! Checks of the repository's own files that the suite depends on: the
//! guides that tell contributors to run it, and the baseline.

use std::collections::BTreeSet;

/// The files among `files` (name, text) that do not mention `needle`.
pub fn missing_mention(files: &[(&str, &str)], needle: &str) -> Vec<String> {
    files
        .iter()
        .filter(|(_, text)| !text.contains(needle))
        .map(|(name, _)| format!("`{name}` does not mention `{needle}`"))
        .collect()
}

/// The cases of the suite that have no observation in the baseline.
pub fn missing_from_baseline(cases: &[String], observed: &BTreeSet<String>) -> Vec<String> {
    cases
        .iter()
        .filter(|c| !observed.contains(*c))
        .map(|c| {
            format!("case `{c}` has no observation in evals/baseline/; record the baseline again")
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_documentation_check_names_the_file_that_forgets() {
        let files = [
            ("CONTRIBUTING.md", "Changing assets/roles/ needs the evals."),
            (
                "CONTRIBUTING.fr.md",
                "Modifier un rôle demande les évaluations.",
            ),
        ];
        assert_eq!(
            missing_mention(&files, "assets/roles/"),
            ["`CONTRIBUTING.fr.md` does not mention `assets/roles/`"]
        );
    }

    #[test]
    fn missing_from_baseline_names_the_case() {
        let cases = vec!["a".to_string(), "new-case".to_string()];
        let observed: BTreeSet<String> = ["a".to_string()].into();
        let gaps = missing_from_baseline(&cases, &observed);
        assert_eq!(gaps.len(), 1);
        assert!(gaps[0].contains("new-case"));
    }
}
