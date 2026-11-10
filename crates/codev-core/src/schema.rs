use std::collections::BTreeSet;

use serde::Deserialize;

use crate::error::{CoreError, Result};
use crate::id::is_kebab_case;
use crate::roles::{self, ARTIFACT_ROLE_KEYS, ArtifactRole};
use crate::yaml_lines::{Step, UnknownKey, unknown_keys_of};

/// The keys of `yaml` this codev does not know, at every level it reads.
fn unknown_keys(yaml: &str) -> Vec<UnknownKey> {
    let Ok(serde_norway::Value::Mapping(map)) = serde_norway::from_str(yaml) else {
        return Vec::new();
    };
    let mut unknown = unknown_keys_of(yaml, &map, SCHEMA_KEYS, &[], "");
    if let Some(artifacts) = map.get("artifacts").and_then(|a| a.as_sequence()) {
        for (i, artifact) in artifacts.iter().enumerate() {
            let Some(artifact) = artifact.as_mapping() else {
                continue;
            };
            let id = artifact
                .get("id")
                .and_then(|id| id.as_str())
                .map_or_else(|| i.to_string(), str::to_string);
            let at = [Step::Key("artifacts"), Step::Item(i)];
            unknown.extend(unknown_keys_of(
                yaml,
                artifact,
                ARTIFACT_KEYS,
                &at,
                &format!("artifacts.{id}."),
            ));
            if let Some(role) = artifact.get("role").and_then(|r| r.as_mapping()) {
                unknown.extend(unknown_keys_of(
                    yaml,
                    role,
                    ARTIFACT_ROLE_KEYS,
                    &[at[0], at[1], Step::Key("role")],
                    &format!("artifacts.{id}.role."),
                ));
            }
        }
    }
    if let Some(apply) = map.get("apply").and_then(|a| a.as_mapping()) {
        unknown.extend(unknown_keys_of(
            yaml,
            apply,
            APPLY_KEYS,
            &[Step::Key("apply")],
            "apply.",
        ));
    }
    unknown.sort_by_key(|u| u.line);
    unknown
}

/// Definition of a workflow: which artifacts exist, and which depend
/// on which.
///
/// This is **data**, never code. A user must be able to edit
/// a `schema.yaml` or a template and see the effect immediately, without
/// waiting for a release. This is the main lesson learned from OpenSpec, whose
/// legacy workflow buried its instructions in the binary.
///
/// A key this codev does not know is ignored and kept in `unknown_keys`,
/// for a `schema_unknown_key` warning: a project schema may be written for
/// a newer codev (decision 0013). The built-in schema holds none, which a
/// test of `codev-engine` asserts.
#[derive(Debug, Clone, Deserialize)]
pub struct Schema {
    pub name: String,
    #[serde(default = "default_version")]
    pub version: u32,
    #[serde(default)]
    pub description: Option<String>,
    pub artifacts: Vec<Artifact>,
    pub apply: Apply,
    /// The keys this codev ignored, in file order, named
    /// `<key>`, `apply.<key>`, `artifacts.<id>.<key>` or
    /// `artifacts.<id>.role.<key>`.
    #[serde(skip)]
    pub unknown_keys: Vec<UnknownKey>,
}

/// The keys this codev knows at each level of a `schema.yaml`. Keep them in
/// sync with the structs by hand: `every_known_key_is_recognized` only fails
/// when a key is removed or the counts change, not when a field is added to
/// a struct alone.
pub const SCHEMA_KEYS: &[&str] = &["name", "version", "description", "artifacts", "apply"];
pub const ARTIFACT_KEYS: &[&str] = &[
    "id",
    "generates",
    "description",
    "template",
    "instruction",
    "requires",
    "role",
];
pub const APPLY_KEYS: &[&str] = &["requires", "tracks", "instruction"];

fn default_version() -> u32 {
    1
}

#[derive(Debug, Clone, Deserialize)]
pub struct Artifact {
    pub id: String,
    /// Output path relative to the change directory. Accepts a glob pattern
    /// (`specs/**/*.md`) for artifacts that produce several files.
    pub generates: String,
    #[serde(default)]
    pub description: Option<String>,
    /// Template file name, looked up in the schema's `templates/`.
    #[serde(default)]
    pub template: Option<String>,
    #[serde(default)]
    pub instruction: Option<String>,
    #[serde(default)]
    pub requires: Vec<String>,
    /// The role that owns this artifact, as data: `codev instructions`
    /// serves it in its `role` field and `codev roles` lists it.
    #[serde(default)]
    pub role: Option<ArtifactRole>,
}

impl Artifact {
    /// True if `generates` is a pattern rather than a literal path.
    pub fn is_pattern(&self) -> bool {
        self.generates.contains('*') || self.generates.contains('?')
    }
}

/// The implementation phase. It is not an artifact: it produces no
/// planning file, it consumes them.
#[derive(Debug, Clone, Deserialize)]
pub struct Apply {
    pub requires: Vec<String>,
    /// The file whose checkboxes track progress.
    pub tracks: String,
    #[serde(default)]
    pub instruction: Option<String>,
}

impl Schema {
    /// Parses a `schema.yaml`, with the keys it does not know in
    /// `unknown_keys`.
    ///
    /// They are not silent: every command reports them as warnings, and
    /// `codev validate --strict` fails on them, so a misspelled key is
    /// still caught. Refusing them instead would leave a teammate one
    /// release behind unable to use a schema written for a newer codev.
    pub fn parse(yaml: &str) -> Result<Self> {
        let mut schema: Self =
            serde_norway::from_str(yaml).map_err(|e| CoreError::SchemaUnreadable(e.to_string()))?;
        schema.validate()?;
        schema.unknown_keys = unknown_keys(yaml);
        Ok(schema)
    }

    fn invalid(&self, reason: impl Into<String>) -> CoreError {
        CoreError::SchemaInvalid {
            schema: self.name.clone(),
            reason: reason.into(),
        }
    }

    /// Checks everything that would make the graph unusable.
    pub fn validate(&self) -> Result<()> {
        if self.name.is_empty() {
            return Err(self.invalid("the `name` field is empty"));
        }
        if !is_kebab_case(&self.name) {
            return Err(self.invalid(format!("name `{}` is not kebab-case", self.name)));
        }
        if self.artifacts.is_empty() {
            return Err(self.invalid("no artifact declared"));
        }

        let mut seen = BTreeSet::new();
        for artifact in &self.artifacts {
            if !is_kebab_case(&artifact.id) {
                return Err(
                    self.invalid(format!("artifact id `{}` is not kebab-case", artifact.id))
                );
            }
            if !seen.insert(artifact.id.as_str()) {
                return Err(self.invalid(format!("artifact `{}` is declared twice", artifact.id)));
            }
            if artifact.generates.is_empty() {
                return Err(self.invalid(format!(
                    "artifact `{}` does not declare `generates`",
                    artifact.id
                )));
            }
        }

        for artifact in &self.artifacts {
            for dep in &artifact.requires {
                if !seen.contains(dep.as_str()) {
                    return Err(self.invalid(format!(
                        "artifact `{}` depends on `{dep}`, which does not exist",
                        artifact.id
                    )));
                }
                if dep == &artifact.id {
                    return Err(
                        self.invalid(format!("artifact `{}` depends on itself", artifact.id))
                    );
                }
            }
        }

        if self.apply.requires.is_empty() {
            return Err(
                self.invalid("`apply.requires` is empty: nothing would trigger implementation")
            );
        }
        for dep in &self.apply.requires {
            if !seen.contains(dep.as_str()) {
                return Err(self.invalid(format!(
                    "`apply.requires` mentions `{dep}`, which is not an artifact of the schema"
                )));
            }
        }
        if self.apply.tracks.is_empty() {
            return Err(self.invalid("`apply.tracks` is empty"));
        }

        self.validate_roles()?;
        self.assert_acyclic()
    }

    /// Checks the artifact roles: each one must be addressable by its id in
    /// `roles:` — unique, kebab-case, distinct from the built-in step roles —
    /// and have an English name, the fallback of every language.
    fn validate_roles(&self) -> Result<()> {
        let mut seen = BTreeSet::new();
        for role in self.artifacts.iter().filter_map(|a| a.role.as_ref()) {
            if !is_kebab_case(&role.id) {
                return Err(self.invalid(format!("role id `{}` is not kebab-case", role.id)));
            }
            if !seen.insert(role.id.as_str()) {
                return Err(self.invalid(format!("role `{}` is declared twice", role.id)));
            }
            if roles::is_builtin_step_role(&role.id) {
                return Err(self.invalid(format!(
                    "role `{}` is a built-in step role; choose another id",
                    role.id
                )));
            }
            if !role.name.contains_key("en") {
                return Err(self.invalid(format!(
                    "role `{}` has no English name; add `en` to its `name`",
                    role.id
                )));
            }
        }
        Ok(())
    }

    /// Detects a dependency cycle.
    ///
    /// A cycle is fatal and cannot be repaired automatically: the message therefore
    /// names the artifacts involved, otherwise the schema author would have no
    /// lead.
    fn assert_acyclic(&self) -> Result<()> {
        let mut settled: BTreeSet<&str> = BTreeSet::new();
        loop {
            let next = self
                .artifacts
                .iter()
                .filter(|a| !settled.contains(a.id.as_str()))
                .find(|a| a.requires.iter().all(|d| settled.contains(d.as_str())));
            match next {
                Some(artifact) => {
                    settled.insert(artifact.id.as_str());
                }
                // Nothing progresses any more: what remains is in a cycle or
                // depends on one.
                None => break,
            }
        }
        if settled.len() == self.artifacts.len() {
            return Ok(());
        }
        let blocked: Vec<&str> = self
            .artifacts
            .iter()
            .map(|a| a.id.as_str())
            .filter(|id| !settled.contains(id))
            .collect();
        Err(self.invalid(format!("dependency cycle between: {}", blocked.join(", "))))
    }

    pub fn artifact(&self, id: &str) -> Option<&Artifact> {
        self.artifacts.iter().find(|a| a.id == id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MINIMAL: &str = r#"
name: minimal
artifacts:
  - id: proposal
    generates: proposal.md
    requires: []
  - id: tasks
    generates: tasks.md
    requires: [proposal]
apply:
  requires: [tasks]
  tracks: tasks.md
"#;

    #[test]
    fn parses_a_minimal_schema() {
        let schema = Schema::parse(MINIMAL).expect("should be valid");
        assert_eq!(schema.name, "minimal");
        assert_eq!(schema.version, 1, "version must default to 1");
        assert_eq!(schema.artifacts.len(), 2);
        assert_eq!(schema.apply.tracks, "tasks.md");
    }

    #[test]
    fn unknown_schema_key_is_ignored_with_its_line() {
        let yaml = MINIMAL
            .replace("name: minimal", "name: minimal\ngates: strict")
            .replace("apply:\n", "apply:\n  gate: hard\n");
        let schema = Schema::parse(&yaml).unwrap();
        assert_eq!(schema.artifacts.len(), 2);
        let line_of = |needle: &str| yaml.lines().position(|l| l.contains(needle)).unwrap() + 1;
        assert_eq!(
            schema.unknown_keys,
            [
                UnknownKey {
                    key: "gates".into(),
                    line: line_of("gates:"),
                },
                UnknownKey {
                    key: "apply.gate".into(),
                    line: line_of("gate: hard"),
                },
            ]
        );
    }

    #[test]
    fn misspelled_artifact_key_is_ignored_and_named() {
        let yaml = MINIMAL.replace("requires: [proposal]", "require: [proposal]");
        let schema = Schema::parse(&yaml).unwrap();
        assert!(schema.artifacts[1].requires.is_empty());
        assert_eq!(schema.unknown_keys.len(), 1);
        assert_eq!(schema.unknown_keys[0].key, "artifacts.tasks.require");
        let line = yaml.lines().position(|l| l.contains("require:")).unwrap() + 1;
        assert_eq!(schema.unknown_keys[0].line, line);
    }

    #[test]
    fn every_known_key_is_recognized() {
        let yaml = r#"
name: full
version: 1
description: d
artifacts:
  - id: a
    generates: a.md
    description: d
    template: a.md
    instruction: i
    requires: []
    role:
      id: owner
      name: { en: Owner }
      question: "?"
      done_when: [x]
apply:
  requires: [a]
  tracks: a.md
  instruction: i
"#;
        assert!(Schema::parse(yaml).unwrap().unknown_keys.is_empty());
        assert_eq!(
            (SCHEMA_KEYS.len(), ARTIFACT_KEYS.len(), APPLY_KEYS.len()),
            (5, 7, 3),
            "a new key belongs in this test too"
        );
        assert_eq!(ARTIFACT_ROLE_KEYS.len(), 4);
    }

    #[test]
    fn schema_that_is_not_yaml_is_unreadable() {
        let err = Schema::parse("name: [unclosed\n").unwrap_err();
        assert_eq!(err.code(), "schema_unreadable");
        let err = Schema::parse("name: x\nartifacts: []\n").unwrap_err();
        assert_eq!(err.code(), "schema_unreadable", "a missing `apply`");
    }

    #[test]
    fn rejects_a_phantom_dependency() {
        let yaml = MINIMAL.replace("requires: [proposal]", "requires: [design]");
        let err = Schema::parse(&yaml).unwrap_err();
        assert_eq!(err.code(), "schema_invalid");
        assert!(err.to_string().contains("design"));
    }

    #[test]
    fn rejects_a_cycle_and_names_the_culprits() {
        let yaml = r#"
name: loop
artifacts:
  - id: a
    generates: a.md
    requires: [b]
  - id: b
    generates: b.md
    requires: [a]
apply:
  requires: [a]
  tracks: a.md
"#;
        let err = Schema::parse(yaml).unwrap_err();
        let message = err.to_string();
        assert!(message.contains("cycle"), "{message}");
        assert!(message.contains('a') && message.contains('b'), "{message}");
    }

    #[test]
    fn rejects_a_duplicate_artifact() {
        let yaml = r#"
name: duplicate
artifacts:
  - id: a
    generates: a.md
  - id: a
    generates: other.md
apply:
  requires: [a]
  tracks: a.md
"#;
        let err = Schema::parse(yaml).unwrap_err();
        assert!(err.to_string().contains("twice"), "{err}");
    }

    const WITH_ROLE: &str = r#"
name: roles
artifacts:
  - id: brief
    generates: brief.md
    role:
      id: editor
      name: { en: Editor, fr: Rédacteur }
      question: Is the brief clear?
      done_when: [One page at most]
apply:
  requires: [brief]
  tracks: brief.md
"#;

    #[test]
    fn parses_an_artifact_role() {
        let schema = Schema::parse(WITH_ROLE).unwrap();
        let role = schema.artifact("brief").unwrap().role.as_ref().unwrap();
        assert_eq!(role.id, "editor");
        assert_eq!(role.name["fr"], "Rédacteur");
        assert_eq!(role.done_when, ["One page at most"]);
        assert!(Schema::parse(MINIMAL).unwrap().artifacts[0].role.is_none());
    }

    #[test]
    fn rejects_a_role_id_taken_by_a_step_role() {
        let err = Schema::parse(&WITH_ROLE.replace("id: editor", "id: skeptic")).unwrap_err();
        assert_eq!(err.code(), "schema_invalid");
        assert!(err.to_string().contains("skeptic"), "{err}");
    }

    #[test]
    fn rejects_a_role_without_an_english_name() {
        let yaml = WITH_ROLE.replace("{ en: Editor, fr: Rédacteur }", "{ fr: Rédacteur }");
        let err = Schema::parse(&yaml).unwrap_err();
        assert_eq!(err.code(), "schema_invalid");
        assert!(err.to_string().contains("editor"), "{err}");
    }

    #[test]
    fn rejects_a_duplicate_role_id() {
        let yaml = r#"
name: twice
artifacts:
  - id: a
    generates: a.md
    role: { id: owner, name: { en: Owner }, question: "?" }
  - id: b
    generates: b.md
    role: { id: owner, name: { en: Owner }, question: "?" }
apply:
  requires: [a]
  tracks: a.md
"#;
        let err = Schema::parse(yaml).unwrap_err();
        assert!(err.to_string().contains("twice"), "{err}");
        let err = Schema::parse(&WITH_ROLE.replace("id: editor", "id: Editor")).unwrap_err();
        assert!(err.to_string().contains("kebab-case"), "{err}");
    }

    #[test]
    fn unknown_artifact_role_key_is_ignored() {
        let yaml = WITH_ROLE.replace("question: Is", "model: sonnet\n      question: Is");
        let schema = Schema::parse(&yaml).unwrap();
        let role = schema.artifact("brief").unwrap().role.as_ref().unwrap();
        assert_eq!(role.id, "editor");
        assert_eq!(schema.unknown_keys.len(), 1);
        assert_eq!(schema.unknown_keys[0].key, "artifacts.brief.role.model");
        let line = yaml.lines().position(|l| l.contains("model:")).unwrap() + 1;
        assert_eq!(schema.unknown_keys[0].line, line);
    }

    #[test]
    fn recognizes_a_glob_pattern() {
        let schema = Schema::parse(MINIMAL).unwrap();
        assert!(!schema.artifact("proposal").unwrap().is_pattern());

        let yaml = MINIMAL.replace("generates: proposal.md", "generates: \"specs/**/*.md\"");
        let schema = Schema::parse(&yaml).unwrap();
        assert!(schema.artifact("proposal").unwrap().is_pattern());
    }

    #[test]
    fn rejects_an_empty_artifact_list_and_an_empty_apply_requires() {
        let no_artifact = "name: empty\nartifacts: []\napply:\n  requires: [a]\n  tracks: a.md\n";
        let err = Schema::parse(no_artifact).unwrap_err();
        assert_eq!(err.code(), "schema_invalid");
        assert!(err.to_string().contains("no artifact"), "{err}");

        let no_apply = MINIMAL.replace("requires: [tasks]", "requires: []");
        let err = Schema::parse(&no_apply).unwrap_err();
        assert_eq!(err.code(), "schema_invalid");
        assert!(err.to_string().contains("apply.requires"), "{err}");
    }
}
