//! Loading the suite from the repository and scoring an output directory:
//! the file reading around the pure modules, shared by the binary and the
//! tests.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use crate::case::{self, Case, Catalog, Fixture, RoleSteps, Vocabulary};
use crate::observed::{self, Observed, RunInfo};
use crate::runner::read_tree;
use crate::score::{self, SuiteScore, Thresholds};

/// The steps of every built-in role, from `assets/roles/*.md`.
pub fn role_steps(repo: &Path) -> Result<RoleSteps, String> {
    let dir = repo.join("assets").join("roles");
    let mut roles = RoleSteps::new();
    let entries = fs::read_dir(&dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    for entry in entries {
        let path = entry.map_err(|e| e.to_string())?.path();
        if path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }
        let text = fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        if let Some((id, at)) = case::role_steps_from_file(&text) {
            roles.insert(id, at);
        }
    }
    Ok(roles)
}

/// The category vocabulary, `evals/categories.yaml`.
pub fn vocabulary(repo: &Path) -> Result<Vocabulary, String> {
    let path = repo.join("evals").join("categories.yaml");
    let text = fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
    case::parse_vocabulary(&text)
}

/// Every case of `evals/cases/`, sorted by id; the first invalid one is an
/// error naming it.
pub fn load_cases(repo: &Path) -> Result<Vec<Case>, String> {
    let catalog = Catalog {
        roles: role_steps(repo)?,
        vocabulary: vocabulary(repo)?,
    };
    let dir = repo.join("evals").join("cases");
    let mut ids: Vec<String> = fs::read_dir(&dir)
        .map_err(|e| format!("{}: {e}", dir.display()))?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().join("case.yaml").is_file())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    ids.sort();
    let mut cases = Vec::new();
    for id in ids {
        let root = dir.join(&id);
        let yaml = fs::read_to_string(root.join("case.yaml"))
            .map_err(|e| format!("case `{id}`: case.yaml: {e}"))?;
        let fixture = Fixture {
            base: read_tree(&root.join("base")).map_err(|e| format!("case `{id}`: {e}"))?,
            change: read_tree(&root.join("change")).map_err(|e| format!("case `{id}`: {e}"))?,
        };
        cases.push(case::load(&id, &yaml, &fixture, &catalog).map_err(|e| e.to_string())?);
    }
    Ok(cases)
}

/// The observation files of an output directory, by case id and
/// repetition: `<case>/<n>/observed.json`, and `<case>/observed.json`,
/// the layout of runs before repetitions, as repetition 1.
fn observation_paths(dir: &Path) -> Result<BTreeMap<(String, u32), PathBuf>, String> {
    let mut paths = BTreeMap::new();
    let entries = fs::read_dir(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    for entry in entries.filter_map(|e| e.ok()) {
        let case_dir = entry.path();
        if !case_dir.is_dir() {
            continue;
        }
        let id = entry.file_name().to_string_lossy().into_owned();
        let flat = case_dir.join("observed.json");
        if flat.is_file() {
            paths.insert((id.clone(), 1), flat);
        }
        for sub in fs::read_dir(&case_dir)
            .map_err(|e| format!("{}: {e}", case_dir.display()))?
            .filter_map(|e| e.ok())
        {
            let Ok(n) = sub.file_name().to_string_lossy().parse::<u32>() else {
                continue;
            };
            let path = sub.path().join("observed.json");
            if n >= 1 && path.is_file() {
                paths.insert((id.clone(), n), path);
            }
        }
    }
    Ok(paths)
}

/// The observations of an output directory, by case id and repetition.
pub fn load_observations(dir: &Path) -> Result<BTreeMap<(String, u32), Observed>, String> {
    let mut observations = BTreeMap::new();
    for ((id, n), path) in observation_paths(dir)? {
        let text = fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        observations.insert((id.clone(), n), observed::parse_observed(&text, &id)?);
    }
    Ok(observations)
}

/// How a recorded run ended, as far as resuming is concerned.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunState {
    Done,
    /// Stopped, or unreadable: run again on resume.
    InError,
}

/// The runs recorded in an output directory, by case id and repetition.
pub type Recorded = BTreeMap<(String, u32), RunState>;

/// The runs recorded in `dir`. An unreadable observation counts as in
/// error, so it is run again.
pub fn recorded(dir: &Path) -> Recorded {
    observation_paths(dir)
        .unwrap_or_default()
        .into_iter()
        .map(|((id, n), path)| {
            let in_error = fs::read_to_string(&path)
                .ok()
                .and_then(|text| observed::parse_observed(&text, &id).ok())
                .is_none_or(|o| o.error.is_some());
            let state = if in_error {
                RunState::InError
            } else {
                RunState::Done
            };
            ((id, n), state)
        })
        .collect()
}

/// Scores every observation of `dir` against its case; with `only`, the
/// observations of those cases alone.
pub fn score_dir(cases: &[Case], dir: &Path, thresholds: Thresholds) -> Result<SuiteScore, String> {
    score_dir_only(cases, dir, thresholds, None)
}

/// [`score_dir`], restricted to the case ids of `only` when given.
pub fn score_dir_only(
    cases: &[Case],
    dir: &Path,
    thresholds: Thresholds,
    only: Option<&BTreeSet<String>>,
) -> Result<SuiteScore, String> {
    let run = read_run(dir)?;
    let observations = load_observations(dir)?;
    let by_id: BTreeMap<&str, &Case> = cases.iter().map(|c| (c.id.as_str(), c)).collect();
    let mut scores = Vec::new();
    for ((id, n), observed) in &observations {
        if only.is_some_and(|only| !only.contains(id)) {
            continue;
        }
        let case = by_id
            .get(id.as_str())
            .ok_or_else(|| format!("`{id}` in {} is not a case of the suite", dir.display()))?;
        scores.push(score::case(case, observed).at(*n));
    }
    Ok(score::suite(run, scores, thresholds))
}

/// `run.json` of an output directory.
pub fn read_run(dir: &Path) -> Result<RunInfo, String> {
    let run_path = dir.join("run.json");
    observed::parse_run_info(
        &fs::read_to_string(&run_path).map_err(|e| format!("{}: {e}", run_path.display()))?,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_earlier_layout_is_one_repetition() {
        let dir = std::env::temp_dir().join(format!("codev-evals-layout-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        let observation = |case: &str, error: Option<&str>| {
            serde_json::json!({"format": 1, "case": case, "error": error,
                "costUsd": 0.3, "durationMs": 1000, "review": null})
            .to_string()
        };
        fs::create_dir_all(dir.join("old-case")).unwrap();
        fs::write(
            dir.join("old-case/observed.json"),
            observation("old-case", None),
        )
        .unwrap();
        for (n, error) in [(1, None), (2, Some("timeout after 600 s"))] {
            let sub = dir.join("new-case").join(n.to_string());
            fs::create_dir_all(&sub).unwrap();
            fs::write(sub.join("observed.json"), observation("new-case", error)).unwrap();
        }
        let keys: Vec<(String, u32)> = load_observations(&dir).unwrap().into_keys().collect();
        assert_eq!(
            keys,
            [
                ("new-case".to_string(), 1),
                ("new-case".to_string(), 2),
                ("old-case".to_string(), 1)
            ]
        );
        let recorded = recorded(&dir);
        assert_eq!(recorded[&("new-case".to_string(), 2)], RunState::InError);
        assert_eq!(recorded[&("old-case".to_string(), 1)], RunState::Done);
        let _ = fs::remove_dir_all(&dir);
    }
}
