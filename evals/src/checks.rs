//! Checks of the repository's own files that the suite depends on: the
//! guides that tell contributors to run it, and the baseline.

use std::collections::{BTreeMap, BTreeSet};

use crate::suite::{Recorded, RunState};

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

/// The cases of the baseline recorded fewer than `min` times.
pub fn too_few_repetitions(recorded: &Recorded, min: u32) -> Vec<String> {
    let mut counts: BTreeMap<&str, u32> = BTreeMap::new();
    for (id, _) in recorded.keys() {
        *counts.entry(id.as_str()).or_default() += 1;
    }
    counts
        .into_iter()
        .filter(|(_, n)| *n < min)
        .map(|(id, n)| {
            format!("case `{id}` has {n} repetition(s) in evals/baseline/, fewer than {min}")
        })
        .collect()
}

/// The repetitions of the baseline recorded in error.
pub fn repetitions_in_error(recorded: &Recorded) -> Vec<String> {
    recorded
        .iter()
        .filter(|(_, state)| **state == RunState::InError)
        .map(|((id, n), _)| {
            format!("case `{id}`, repetition {n}, is in error in evals/baseline/; run it again")
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
    fn a_guide_without_the_skills_is_named() {
        let files = [
            (
                "CONTRIBUTING.md",
                "A change to assets/roles/ pastes `codev-evals -- compare`.",
            ),
            (
                "CONTRIBUTING.fr.md",
                "Un changement de assets/workflows/ colle la comparaison.",
            ),
        ];
        assert_eq!(
            missing_mention(&files, "assets/workflows/"),
            ["`CONTRIBUTING.md` does not mention `assets/workflows/`"]
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

    #[test]
    fn a_guide_without_compare_is_named() {
        let files = [
            (
                "CONTRIBUTING.md",
                "paste `cargo run -p codev-evals -- compare --quality-only`",
            ),
            ("CONTRIBUTING.fr.md", "collez les scores avant et après"),
        ];
        assert_eq!(
            missing_mention(&files, "codev-evals -- compare"),
            ["`CONTRIBUTING.fr.md` does not mention `codev-evals -- compare`"]
        );
    }

    fn recorded(entries: &[(&str, u32, bool)]) -> Recorded {
        entries
            .iter()
            .map(|(id, n, in_error)| {
                let state = if *in_error {
                    RunState::InError
                } else {
                    RunState::Done
                };
                ((id.to_string(), *n), state)
            })
            .collect()
    }

    #[test]
    fn a_case_with_three_repetitions_is_named() {
        let mut entries: Vec<(&str, u32, bool)> = (1..=5).map(|n| ("full", n, false)).collect();
        entries.extend((1..=3).map(|n| ("short", n, false)));
        assert_eq!(
            too_few_repetitions(&recorded(&entries), 5),
            ["case `short` has 3 repetition(s) in evals/baseline/, fewer than 5"]
        );
    }

    #[test]
    fn a_repetition_in_error_is_named() {
        let entries = [("ops", 1, false), ("ops", 4, true)];
        assert_eq!(
            repetitions_in_error(&recorded(&entries)),
            ["case `ops`, repetition 4, is in error in evals/baseline/; run it again"]
        );
    }
}
