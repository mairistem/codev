//! Loading the suite from the repository and scoring an output directory:
//! the file reading around the pure modules, shared by the binary and the
//! tests.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use crate::case::{self, Case, Fixture, RoleSteps};
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

/// Every case of `evals/cases/`, sorted by id; the first invalid one is an
/// error naming it.
pub fn load_cases(repo: &Path) -> Result<Vec<Case>, String> {
    let roles = role_steps(repo)?;
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
        cases.push(case::load(&id, &yaml, &fixture, &roles).map_err(|e| e.to_string())?);
    }
    Ok(cases)
}

/// The observations of an output directory, by case id.
pub fn load_observations(dir: &Path) -> Result<BTreeMap<String, Observed>, String> {
    let mut observations = BTreeMap::new();
    let entries = fs::read_dir(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    for entry in entries.filter_map(|e| e.ok()) {
        let path = entry.path().join("observed.json");
        if !path.is_file() {
            continue;
        }
        let id = entry.file_name().to_string_lossy().into_owned();
        let text = fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        observations.insert(id.clone(), observed::parse_observed(&text, &id)?);
    }
    Ok(observations)
}

/// The ids of the cases that have an observation in `dir`.
pub fn recorded(dir: &Path) -> BTreeSet<String> {
    fs::read_dir(dir)
        .into_iter()
        .flatten()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().join("observed.json").is_file())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect()
}

/// Scores every observation of `dir` against its case.
pub fn score_dir(cases: &[Case], dir: &Path, thresholds: Thresholds) -> Result<SuiteScore, String> {
    let run_path = dir.join("run.json");
    let run: RunInfo = observed::parse_run_info(
        &fs::read_to_string(&run_path).map_err(|e| format!("{}: {e}", run_path.display()))?,
    )?;
    let observations = load_observations(dir)?;
    let by_id: BTreeMap<&str, &Case> = cases.iter().map(|c| (c.id.as_str(), c)).collect();
    let mut scores = Vec::new();
    for (id, observed) in &observations {
        let case = by_id
            .get(id.as_str())
            .ok_or_else(|| format!("`{id}` in {} is not a case of the suite", dir.display()))?;
        scores.push(score::case(case, observed));
    }
    Ok(score::suite(run, scores, thresholds))
}
