use std::collections::BTreeSet;
use std::path::Path;

use codev_core::ArtifactGraph;
use serde::{Deserialize, Serialize};

use crate::error::{EngineError, Result};
use crate::ports::FileSystem;

/// A change's `change.yaml`.
///
/// It lives in the change directory and is versioned with it: it is a
/// decision made by the change's author, not machine state.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChangeMetadata {
    pub schema: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub goal: Option<String>,

    /// Declares that this change intentionally has no spec delta:
    /// pure refactor, tooling, documentation.
    ///
    /// Without this marker, validation rejects a change with zero deltas —
    /// which is intended: it is what keeps specs from being forgotten. With
    /// it, validation accepts it. One or the other, but never inventing a
    /// requirement just to satisfy the tool.
    #[serde(default, skip_serializing_if = "is_false")]
    pub skip_specs: bool,

    /// Allows `sync` and `archive` to delete the spec of a capability whose
    /// last requirement this change removes.
    ///
    /// Explicit because the deletion can only be recovered from git: it is
    /// the author's choice, not something inferred from the shape of a
    /// delta. Without this marker, a `## REMOVED Requirements` delta that
    /// would empty the spec is rejected with `would_leave_spec_without_requirement`.
    #[serde(default, skip_serializing_if = "is_false")]
    pub retire_capabilities: bool,
}

fn is_false(value: &bool) -> bool {
    !*value
}

impl ChangeMetadata {
    pub fn new(schema: impl Into<String>, created: impl Into<String>) -> Self {
        Self {
            schema: schema.into(),
            created: Some(created.into()),
            goal: None,
            skip_specs: false,
            retire_capabilities: false,
        }
    }

    pub fn with_goal(mut self, goal: Option<String>) -> Self {
        self.goal = goal.filter(|g| !g.trim().is_empty());
        self
    }

    /// The artifacts this change neutralizes.
    ///
    /// The rule is based on the **output path prefix** (`specs/`), not on
    /// the `specs` identifier: a custom schema that names its artifact
    /// differently but writes under `specs/` inherits the same behavior,
    /// without having to know about it.
    pub fn skipped_artifacts(&self, graph: &ArtifactGraph) -> BTreeSet<String> {
        if !self.skip_specs {
            return BTreeSet::new();
        }
        graph
            .artifacts()
            .iter()
            .filter(|a| a.generates == "specs" || a.generates.starts_with("specs/"))
            .map(|a| a.id.clone())
            .collect()
    }

    pub fn to_yaml(&self) -> String {
        // A serialization failure here would be a bug in our types, not a
        // user error: the structure only holds scalars.
        serde_norway::to_string(self).expect("change metadata is serializable")
    }
}

/// Reads the `change.yaml`. Its absence is not an error: a hand-made change
/// may have none, and the schema then comes from the project config.
pub fn load(fs: &dyn FileSystem, path: &Path) -> Result<Option<ChangeMetadata>> {
    if !fs.exists(path) {
        return Ok(None);
    }
    let raw = fs
        .read_to_string(path)
        .map_err(|e| EngineError::Unreadable {
            path: path.to_path_buf(),
            reason: e.to_string(),
        })?;
    serde_norway::from_str(&raw)
        .map(Some)
        .map_err(|e| EngineError::Invalid {
            path: path.to_path_buf(),
            reason: e.to_string(),
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ports::MemoryFileSystem;

    const SPEC_DRIVEN: &str = r#"
name: spec-driven
artifacts:
  - id: proposal
    generates: proposal.md
  - id: specs
    generates: "specs/**/*.md"
    requires: [proposal]
apply:
  requires: [specs]
  tracks: proposal.md
"#;

    fn graph() -> ArtifactGraph {
        ArtifactGraph::from_yaml(SPEC_DRIVEN).unwrap()
    }

    #[test]
    fn without_skip_specs_nothing_is_neutralized() {
        let metadata = ChangeMetadata::new("spec-driven", "2026-09-08");
        assert!(metadata.skipped_artifacts(&graph()).is_empty());
    }

    #[test]
    fn skip_specs_neutralizes_by_path_prefix() {
        let mut metadata = ChangeMetadata::new("spec-driven", "2026-09-08");
        metadata.skip_specs = true;
        let skipped = metadata.skipped_artifacts(&graph());
        assert_eq!(skipped.len(), 1);
        assert!(skipped.contains("specs"));
    }

    #[test]
    fn skip_specs_follows_the_path_not_the_identifier() {
        // A custom schema that calls its artifact "contracts" but writes
        // under `specs/` must be neutralized as well.
        let yaml = SPEC_DRIVEN
            .replace("id: specs", "id: contracts")
            .replace("requires: [specs]", "requires: [contracts]");
        let graph = ArtifactGraph::from_yaml(&yaml).unwrap();
        let mut metadata = ChangeMetadata::new("spec-driven", "2026-09-08");
        metadata.skip_specs = true;
        assert!(metadata.skipped_artifacts(&graph).contains("contracts"));
    }

    #[test]
    fn yaml_omits_empty_fields() {
        let yaml = ChangeMetadata::new("spec-driven", "2026-09-08").to_yaml();
        assert!(yaml.contains("schema: spec-driven"));
        // The date comes out unquoted. That is harmless here — we read it
        // back as a `String`, which `reads_back_what_it_writes` checks.
        assert!(yaml.contains("created: 2026-09-08"), "{yaml}");
        assert!(
            !yaml.contains("goal"),
            "a missing goal is not written: {yaml}"
        );
        assert!(
            !yaml.contains("skip_specs"),
            "a false is not written: {yaml}"
        );
    }

    #[test]
    fn reads_back_what_it_writes() {
        let original = ChangeMetadata::new("spec-driven", "2026-09-08")
            .with_goal(Some("Add authentication".into()));
        let fs = MemoryFileSystem::new().with_file("/c/change.yaml", original.to_yaml());
        let reread = load(&fs, Path::new("/c/change.yaml")).unwrap().unwrap();
        assert_eq!(reread.schema, "spec-driven");
        assert_eq!(reread.goal.as_deref(), Some("Add authentication"));
        assert!(!reread.skip_specs);
    }

    #[test]
    fn rejects_an_unknown_key() {
        let fs = MemoryFileSystem::new()
            .with_file("/c/change.yaml", "schema: spec-driven\nskipspecs: true\n");
        let err = load(&fs, Path::new("/c/change.yaml")).unwrap_err();
        assert_eq!(err.code(), "invalid");
        assert!(err.to_string().contains("skipspecs"), "{err}");
    }
}
