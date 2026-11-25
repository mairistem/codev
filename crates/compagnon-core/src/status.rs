use std::collections::BTreeSet;

use crate::graph::ArtifactGraph;
use crate::id::ChangeId;

/// The state of an artifact.
///
/// There is **no state file**: the state is inferred from the existence of
/// files on disk. A user who deletes `design.md` by hand
/// resets that artifact to `Ready`, with no repair command to learn.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArtifactState {
    /// Its output exists.
    Done,
    /// Its dependencies are satisfied; it can be written.
    Ready,
    /// It is waiting on at least one dependency.
    Blocked,
    /// The change has disabled it (`skip_specs`). It counts as satisfied, and
    /// its files must **not** be created.
    Skipped,
}

impl ArtifactState {
    /// True if this state satisfies a dependency.
    pub fn satisfies_dependency(self) -> bool {
        matches!(self, Self::Done | Self::Skipped)
    }
}

#[derive(Debug, Clone)]
pub struct ArtifactStatus {
    pub id: String,
    /// The schema's `generates`, verbatim — literal path or pattern.
    pub output_path: String,
    pub state: ArtifactState,
    pub requires: Vec<String>,
    /// The missing dependencies, when the state is `Blocked`.
    pub missing_deps: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ChangeStatus {
    pub change: ChangeId,
    pub schema_name: String,
    pub apply_requires: Vec<String>,
    /// In topological order: the first `Ready` one is the artifact to write
    /// now.
    pub artifacts: Vec<ArtifactStatus>,
    /// True when every artifact in the required closure is satisfied.
    /// Says **nothing** about the progress of implementation tasks.
    pub planning_complete: bool,
}

/// Computes the state of a change. Pure function: the disk has already been queried
/// by the caller, which supplies `existing` and `skipped`.
pub fn compute(
    graph: &ArtifactGraph,
    change: &ChangeId,
    existing: &BTreeSet<String>,
    skipped: &BTreeSet<String>,
) -> ChangeStatus {
    let mut artifacts = Vec::new();
    let mut satisfied: BTreeSet<&str> = BTreeSet::new();

    for artifact in graph.topological_order() {
        let state = if skipped.contains(&artifact.id) {
            ArtifactState::Skipped
        } else if existing.contains(&artifact.id) {
            ArtifactState::Done
        } else if artifact
            .requires
            .iter()
            .all(|d| satisfied.contains(d.as_str()))
        {
            ArtifactState::Ready
        } else {
            ArtifactState::Blocked
        };

        if state.satisfies_dependency() {
            satisfied.insert(artifact.id.as_str());
        }

        let missing_deps = if state == ArtifactState::Blocked {
            artifact
                .requires
                .iter()
                .filter(|d| !satisfied.contains(d.as_str()))
                .cloned()
                .collect()
        } else {
            Vec::new()
        };

        artifacts.push(ArtifactStatus {
            id: artifact.id.clone(),
            output_path: artifact.generates.clone(),
            state,
            requires: artifact.requires.clone(),
            missing_deps,
        });
    }

    let closure = graph.required_closure();
    let planning_complete = artifacts
        .iter()
        .filter(|a| closure.contains(&a.id))
        .all(|a| a.state.satisfies_dependency());

    ChangeStatus {
        change: change.clone(),
        schema_name: graph.schema().name.clone(),
        apply_requires: graph.schema().apply.requires.clone(),
        artifacts,
        planning_complete,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SPEC_DRIVEN: &str = r#"
name: spec-driven
artifacts:
  - id: proposal
    generates: proposal.md
    requires: []
  - id: specs
    generates: "specs/**/*.md"
    requires: [proposal]
  - id: design
    generates: design.md
    requires: [proposal]
  - id: tasks
    generates: tasks.md
    requires: [specs, design]
apply:
  requires: [tasks]
  tracks: tasks.md
"#;

    fn set(ids: &[&str]) -> BTreeSet<String> {
        ids.iter().map(|s| s.to_string()).collect()
    }

    fn status(existing: &[&str], skipped: &[&str]) -> ChangeStatus {
        let graph = ArtifactGraph::from_yaml(SPEC_DRIVEN).unwrap();
        let change = ChangeId::parse("add-auth").unwrap();
        compute(&graph, &change, &set(existing), &set(skipped))
    }

    fn state_of(status: &ChangeStatus, id: &str) -> ArtifactState {
        status
            .artifacts
            .iter()
            .find(|a| a.id == id)
            .unwrap_or_else(|| panic!("artifact `{id}` missing from status"))
            .state
    }

    #[test]
    fn an_empty_change_has_only_its_root_ready() {
        let status = status(&[], &[]);
        assert_eq!(state_of(&status, "proposal"), ArtifactState::Ready);
        assert_eq!(state_of(&status, "specs"), ArtifactState::Blocked);
        assert_eq!(state_of(&status, "tasks"), ArtifactState::Blocked);
        assert!(!status.planning_complete);
    }

    #[test]
    fn the_first_ready_is_the_artifact_to_write() {
        let status = status(&["proposal"], &[]);
        let first_ready = status
            .artifacts
            .iter()
            .find(|a| a.state == ArtifactState::Ready)
            .expect("something must be left to write");
        assert_eq!(first_ready.id, "specs");
    }

    #[test]
    fn names_the_missing_dependencies() {
        let status = status(&["proposal", "specs"], &[]);
        let tasks = status.artifacts.iter().find(|a| a.id == "tasks").unwrap();
        assert_eq!(tasks.state, ArtifactState::Blocked);
        assert_eq!(tasks.missing_deps, ["design"]);
    }

    #[test]
    fn a_skipped_artifact_satisfies_its_dependents() {
        // This is the `skip_specs` case: `specs` will never exist, and yet `tasks`
        // must become writable.
        let status = status(&["proposal", "design"], &["specs"]);
        assert_eq!(state_of(&status, "specs"), ArtifactState::Skipped);
        assert_eq!(state_of(&status, "tasks"), ArtifactState::Ready);
    }

    #[test]
    fn planning_is_complete_when_the_closure_is_satisfied() {
        let status = status(&["proposal", "specs", "design", "tasks"], &[]);
        assert!(status.planning_complete);
    }

    #[test]
    fn writing_tasks_first_does_not_complete_planning() {
        // The trap the contract must make visible: `status` only looks at
        // file existence, so `tasks` is `Done` even though `specs` and
        // `design` were never written.
        let status = status(&["tasks"], &[]);
        assert_eq!(state_of(&status, "tasks"), ArtifactState::Done);
        assert!(
            !status.planning_complete,
            "the required closure is not satisfied"
        );
    }
}
