use std::collections::{BTreeSet, VecDeque};

use crate::error::Result;
use crate::schema::{Artifact, Schema};

/// The dependency graph between a schema's artifacts.
///
/// Dependencies are **enablers, not gates**: they say what
/// can be created, not what must be created next. A `design.md`
/// can be skipped; whatever depends on it remains writable.
#[derive(Debug)]
pub struct ArtifactGraph {
    schema: Schema,
}

impl ArtifactGraph {
    /// Builds the graph from an already-deserialized schema.
    pub fn new(schema: Schema) -> Result<Self> {
        schema.validate()?;
        Ok(Self { schema })
    }

    pub fn from_yaml(yaml: &str) -> Result<Self> {
        Ok(Self {
            schema: Schema::parse(yaml)?,
        })
    }

    pub fn schema(&self) -> &Schema {
        &self.schema
    }

    pub fn artifacts(&self) -> &[Artifact] {
        &self.schema.artifacts
    }

    pub fn artifact(&self, id: &str) -> Option<&Artifact> {
        self.schema.artifact(id)
    }

    /// Topological order: a dependency never appears after what
    /// depends on it.
    ///
    /// Ties are broken by the schema's **declaration order**, not
    /// alphabetically. The graph leaves `specs` and `design` tied — both
    /// depend only on `proposal` — and an alphabetical sort would put
    /// `design` first, contradicting the sequence the schema itself announces
    /// (`proposal → specs → design → tasks`). Following the author's writing
    /// order is just as deterministic and does not betray them.
    pub fn topological_order(&self) -> Vec<&Artifact> {
        let mut settled: BTreeSet<&str> = BTreeSet::new();
        let mut order = Vec::with_capacity(self.schema.artifacts.len());
        // Deliberately quadratic loop: a schema has a handful
        // of artifacts, and this shape makes the tie-breaking criterion obvious.
        loop {
            let next = self
                .schema
                .artifacts
                .iter()
                .filter(|a| !settled.contains(a.id.as_str()))
                .find(|a| a.requires.iter().all(|d| settled.contains(d.as_str())));
            match next {
                Some(artifact) => {
                    settled.insert(artifact.id.as_str());
                    order.push(artifact);
                }
                // `Schema::validate` has already ruled out cycles, so we only
                // exit here once everyone has been placed.
                None => break,
            }
        }
        order
    }

    /// The set of artifacts the implementation depends on, transitively.
    ///
    /// `apply.requires` is not enough: with `spec-driven` it only names
    /// `tasks`, whereas `tasks` depends on `specs` and `design`, which depend
    /// on `proposal`. An agent relying on the `apply.requires` list alone
    /// would write `tasks.md` and stop there.
    pub fn required_closure(&self) -> BTreeSet<String> {
        let mut closure = BTreeSet::new();
        let mut queue: VecDeque<&str> = self
            .schema
            .apply
            .requires
            .iter()
            .map(String::as_str)
            .collect();
        while let Some(id) = queue.pop_front() {
            if !closure.insert(id.to_string()) {
                continue;
            }
            if let Some(artifact) = self.artifact(id) {
                queue.extend(artifact.requires.iter().map(String::as_str));
            }
        }
        closure
    }

    /// The artifacts that creating `id` makes possible.
    pub fn unlocked_by(&self, id: &str) -> Vec<&str> {
        self.schema
            .artifacts
            .iter()
            .filter(|a| a.requires.iter().any(|d| d == id))
            .map(|a| a.id.as_str())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The shape of `spec-driven`: `specs` and `design` are tied, and
    /// `specs` is declared first.
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

    fn graph() -> ArtifactGraph {
        ArtifactGraph::from_yaml(SPEC_DRIVEN).expect("valid test schema")
    }

    fn ordered_ids(graph: &ArtifactGraph) -> Vec<&str> {
        graph
            .topological_order()
            .iter()
            .map(|a| a.id.as_str())
            .collect()
    }

    #[test]
    fn orders_by_dependencies() {
        let graph = graph();
        assert_eq!(
            ordered_ids(&graph),
            ["proposal", "specs", "design", "tasks"]
        );
    }

    #[test]
    fn breaks_ties_by_declaration_order_not_alphabetically() {
        // The trap: "design" precedes "specs" alphabetically. If alphabetical
        // sorting ever crept back in, this test would fail.
        let graph = graph();
        let ids = ordered_ids(&graph);
        let position = |id: &str| ids.iter().position(|x| *x == id).unwrap();
        assert!(
            position("specs") < position("design"),
            "actual order is {ids:?}"
        );
    }

    #[test]
    fn closes_transitively_over_apply_requires() {
        let closure = graph().required_closure();
        let expected: BTreeSet<String> = ["proposal", "specs", "design", "tasks"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(closure, expected);
    }

    #[test]
    fn tells_what_an_artifact_unlocks() {
        assert_eq!(graph().unlocked_by("proposal"), ["specs", "design"]);
        assert_eq!(graph().unlocked_by("specs"), ["tasks"]);
        assert!(graph().unlocked_by("tasks").is_empty());
    }
}
