use std::collections::BTreeSet;

use serde::Deserialize;

use crate::error::{CoreError, Result};
use crate::id::is_kebab_case;
use crate::roles::{self, ArtifactRole};

/// Definition of a workflow: which artifacts exist, and which depend
/// on which.
///
/// This is **data**, never code. A user must be able to edit
/// a `schema.yaml` or a template and see the effect immediately, without
/// waiting for a release. This is the main lesson learned from OpenSpec, whose
/// legacy workflow buried its instructions in the binary.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Schema {
    pub name: String,
    #[serde(default = "default_version")]
    pub version: u32,
    #[serde(default)]
    pub description: Option<String>,
    pub artifacts: Vec<Artifact>,
    pub apply: Apply,
}

fn default_version() -> u32 {
    1
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
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
#[serde(deny_unknown_fields)]
pub struct Apply {
    pub requires: Vec<String>,
    /// The file whose checkboxes track progress.
    pub tracks: String,
    #[serde(default)]
    pub instruction: Option<String>,
}

impl Schema {
    /// Parses a `schema.yaml`.
    ///
    /// `deny_unknown_fields` is deliberate: these files are hand-written,
    /// and a misspelled key that was silently ignored
    /// would produce a workflow that does not do what its author thinks.
    pub fn parse(yaml: &str) -> Result<Self> {
        let schema: Self =
            serde_norway::from_str(yaml).map_err(|e| CoreError::SchemaUnreadable(e.to_string()))?;
        schema.validate()?;
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
    fn rejects_an_unknown_key() {
        let yaml = MINIMAL.replace("name: minimal", "name: minimal\nartefacts: []");
        let err = Schema::parse(&yaml).unwrap_err();
        assert_eq!(err.code(), "schema_unreadable");
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
    fn rejects_an_unknown_role_key() {
        let yaml = WITH_ROLE.replace("question: Is", "owns: why\n      question: Is");
        let err = Schema::parse(&yaml).unwrap_err();
        assert_eq!(err.code(), "schema_unreadable");
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
