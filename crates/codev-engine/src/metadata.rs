use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use codev_core::ArtifactGraph;
use codev_core::review::ReviewMode;
use codev_core::roles::RoleActivation;
use serde::{Deserialize, Serialize};

use crate::error::{EngineError, Result};
use crate::ports::FileSystem;
use codev_core::yaml_lines::{Step, UnknownKey, unknown_keys_of};

/// The top-level keys of `change.yaml` this codev knows. Any other key is
/// ignored with a `change_unknown_key` warning (decision 0013): the change
/// may have been written by a newer codev. Keep it in sync with
/// [`ChangeMetadata`] by hand: `every_known_key_is_recognized` only fails
/// when a key is removed from this list or the count changes, not when a
/// field is added to the struct alone.
pub const CHANGE_KEYS: &[&str] = &[
    "schema",
    "created",
    "goal",
    "skip_specs",
    "retire_capabilities",
    "roles",
    "review",
];

/// The code of the warning and of the validate finding for a key of
/// `change.yaml` this codev ignores.
pub const CHANGE_UNKNOWN_KEY: &str = "change_unknown_key";

/// A change's `change.yaml`.
///
/// It lives in the change directory and is versioned with it: it is a
/// decision made by the change's author, not machine state. No
/// `deny_unknown_fields`: [`load`] returns the keys this codev does not
/// know beside the metadata. The markers that guard a check fail closed
/// without it — an older codev that ignores `retire_capabilities` refuses
/// to delete a spec — so ignoring a newer key never weakens one.
#[derive(Debug, Clone, Serialize, Deserialize)]
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

    /// Role activations for this change only (`newcomer: always`), over
    /// the project's `roles:`. Activations only: criteria belong to the
    /// project.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub roles: BTreeMap<String, RoleActivation>,

    /// `review: { mode: inline | isolated }` for this change only, over
    /// the project's. Kept as written and checked by [`load`], so that an
    /// invalid mode fails with `invalid_review_mode` like the project's.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review: Option<serde_norway::Value>,
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
            roles: BTreeMap::new(),
            review: None,
        }
    }

    /// The change's review mode, when it sets one. `load` has already
    /// refused an invalid one.
    pub fn review_mode(&self) -> Option<ReviewMode> {
        crate::config::read_review_mode(self.review.as_ref())
            .ok()
            .flatten()
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

/// Reads the `change.yaml`, with the keys this codev does not know — at
/// the top level, and `review.<key>` under `review:` — in file order. Its
/// absence is not an error: a hand-made change may have none, and the
/// schema then comes from the project config.
///
/// The values stay strict: an invalid `review.mode` or role activation
/// fails, since a value cannot be ignored without guessing what the change
/// meant.
pub fn load(fs: &dyn FileSystem, path: &Path) -> Result<Option<(ChangeMetadata, Vec<UnknownKey>)>> {
    if !fs.exists(path) {
        return Ok(None);
    }
    let raw = fs
        .read_to_string(path)
        .map_err(|e| EngineError::Unreadable {
            path: path.to_path_buf(),
            reason: e.to_string(),
        })?;
    let metadata: ChangeMetadata =
        serde_norway::from_str(&raw).map_err(|e| EngineError::Invalid {
            path: path.to_path_buf(),
            reason: e.to_string(),
        })?;
    crate::config::read_review_mode(metadata.review.as_ref()).map_err(|reason| {
        EngineError::InvalidReviewMode {
            path: path.to_path_buf(),
            reason,
        }
    })?;
    Ok(Some((metadata, unknown_keys(&raw))))
}

/// The keys of `raw` this codev does not know, top level and `review`.
fn unknown_keys(raw: &str) -> Vec<UnknownKey> {
    let Ok(serde_norway::Value::Mapping(map)) = serde_norway::from_str(raw) else {
        return Vec::new();
    };
    let mut unknown = unknown_keys_of(raw, &map, CHANGE_KEYS, &[], "");
    if let Some(review) = map.get("review").and_then(|r| r.as_mapping()) {
        unknown.extend(unknown_keys_of(
            raw,
            review,
            crate::config::REVIEW_KEYS,
            &[Step::Key("review")],
            "review.",
        ));
    }
    unknown.sort_by_key(|u| u.line);
    unknown
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
        let reread = load(&fs, Path::new("/c/change.yaml")).unwrap().unwrap().0;
        assert_eq!(reread.schema, "spec-driven");
        assert_eq!(reread.goal.as_deref(), Some("Add authentication"));
        assert!(!reread.skip_specs);
    }

    #[test]
    fn change_roles_are_read() {
        let fs = MemoryFileSystem::new().with_file(
            "/c/change.yaml",
            "schema: spec-driven\nroles:\n  newcomer: always\n",
        );
        let metadata = load(&fs, Path::new("/c/change.yaml")).unwrap().unwrap().0;
        assert_eq!(metadata.roles["newcomer"], RoleActivation("always".into()));
        assert!(!metadata.to_yaml().is_empty());
        let fresh = ChangeMetadata::new("spec-driven", "2026-09-08").to_yaml();
        assert!(!fresh.contains("roles"), "{fresh}");
    }

    #[test]
    fn change_review_mode_is_read() {
        let fs = MemoryFileSystem::new().with_file(
            "/c/change.yaml",
            "schema: spec-driven\nreview:\n  mode: isolated\n",
        );
        let metadata = load(&fs, Path::new("/c/change.yaml")).unwrap().unwrap().0;
        assert_eq!(metadata.review_mode(), Some(ReviewMode::Isolated));
        assert!(metadata.to_yaml().contains("mode: isolated"));
        let fresh = ChangeMetadata::new("spec-driven", "2026-09-08");
        assert_eq!(fresh.review_mode(), None);
        assert!(!fresh.to_yaml().contains("review"));
    }

    #[test]
    fn an_unknown_change_review_mode_fails() {
        let fs = MemoryFileSystem::new().with_file(
            "/c/change.yaml",
            "schema: spec-driven\nreview:\n  mode: alone\n",
        );
        let err = load(&fs, Path::new("/c/change.yaml")).unwrap_err();
        assert_eq!(err.code(), "invalid_review_mode");
        let message = err.to_string();
        assert!(message.contains("change.yaml"), "{message}");
        assert!(message.contains("`alone`"), "{message}");
        assert!(message.contains("inline or isolated"), "{message}");
    }

    #[test]
    fn unknown_change_key_is_ignored_and_reported() {
        let fs = MemoryFileSystem::new().with_file(
            "/c/change.yaml",
            "schema: spec-driven\nticket: JVS-12\nskip_spec: true\n",
        );
        let (metadata, unknown) = load(&fs, Path::new("/c/change.yaml")).unwrap().unwrap();
        assert_eq!(metadata.schema, "spec-driven");
        assert!(!metadata.skip_specs, "a misspelled marker is not read");
        assert_eq!(
            unknown,
            [
                UnknownKey {
                    key: "ticket".into(),
                    line: 2
                },
                UnknownKey {
                    key: "skip_spec".into(),
                    line: 3
                },
            ]
        );
    }

    #[test]
    fn unknown_review_key_in_change_yaml_keeps_the_mode() {
        let fs = MemoryFileSystem::new().with_file(
            "/c/change.yaml",
            "schema: spec-driven\nreview:\n  mode: isolated\n  budget: 3\n",
        );
        let (metadata, unknown) = load(&fs, Path::new("/c/change.yaml")).unwrap().unwrap();
        assert_eq!(metadata.review_mode(), Some(ReviewMode::Isolated));
        assert_eq!(
            unknown,
            [UnknownKey {
                key: "review.budget".into(),
                line: 4
            }]
        );
    }

    #[test]
    fn every_known_key_is_recognized() {
        let fs = MemoryFileSystem::new().with_file(
            "/c/change.yaml",
            "schema: spec-driven\ncreated: 2026-10-26\ngoal: g\nskip_specs: true\n\
             retire_capabilities: true\nroles:\n  newcomer: always\nreview:\n  mode: inline\n",
        );
        let (_, unknown) = load(&fs, Path::new("/c/change.yaml")).unwrap().unwrap();
        assert!(unknown.is_empty(), "{unknown:?}");
        assert_eq!(CHANGE_KEYS.len(), 7, "a new key belongs in this test too");
    }
}
