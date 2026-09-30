use std::path::PathBuf;

use codev_core::schema::Artifact;
use codev_core::{ArtifactGraph, Layout};

use crate::error::{EngineError, Result};
use crate::ports::FileSystem;

/// The only embedded schema. Everything else comes from the project.
pub const BUILTIN_SCHEMA: &str = "spec-driven";

const BUILTIN_SCHEMA_YAML: &str = include_str!("../../../assets/schemas/spec-driven/schema.yaml");

/// The embedded schema's templates, included at compile time.
///
/// A self-contained binary: `codev init` works without downloading anything,
/// and an installation can never end up with a schema from one version and
/// templates from another.
const BUILTIN_TEMPLATES: &[(&str, &str)] = &[
    (
        "proposal.md",
        include_str!("../../../assets/schemas/spec-driven/templates/proposal.md"),
    ),
    (
        "spec.md",
        include_str!("../../../assets/schemas/spec-driven/templates/spec.md"),
    ),
    (
        "design.md",
        include_str!("../../../assets/schemas/spec-driven/templates/design.md"),
    ),
    (
        "tasks.md",
        include_str!("../../../assets/schemas/spec-driven/templates/tasks.md"),
    ),
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchemaOrigin {
    /// Defined by the project, in `_codev/schemas/<name>/`.
    Project(PathBuf),
    /// Embedded in the binary.
    Builtin,
}

impl SchemaOrigin {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Project(_) => "project",
            Self::Builtin => "built-in",
        }
    }
}

#[derive(Debug)]
pub struct ResolvedSchema {
    pub graph: ArtifactGraph,
    pub origin: SchemaOrigin,
}

/// Resolves a schema by name.
///
/// Precedence: the project first, the embedded one second. A project can
/// therefore replace `spec-driven` with its own version without renaming it,
/// and without anything else moving.
pub fn resolve(fs: &dyn FileSystem, layout: &Layout, name: &str) -> Result<ResolvedSchema> {
    let project_dir = layout.project_schema_dir(name);
    let project_file = project_dir.join("schema.yaml");
    if fs.exists(&project_file) {
        let raw = fs
            .read_to_string(&project_file)
            .map_err(|e| EngineError::Unreadable {
                path: project_file.clone(),
                reason: e.to_string(),
            })?;
        let graph = ArtifactGraph::from_yaml(&raw)?;
        return Ok(ResolvedSchema {
            graph,
            origin: SchemaOrigin::Project(project_dir),
        });
    }

    if name == BUILTIN_SCHEMA {
        return Ok(ResolvedSchema {
            // A failure here would be a build bug on our side, not a user
            // error: the file is embedded.
            graph: ArtifactGraph::from_yaml(BUILTIN_SCHEMA_YAML)?,
            origin: SchemaOrigin::Builtin,
        });
    }

    Err(EngineError::SchemaNotFound {
        name: name.to_string(),
    })
}

/// The available schemas, the project's first.
pub fn list(fs: &dyn FileSystem, layout: &Layout) -> Vec<(String, SchemaOrigin)> {
    let mut schemas = Vec::new();
    let schemas_dir = layout.schemas_dir();
    if let Ok(names) = fs.list_dir(&schemas_dir) {
        for name in names {
            let dir = schemas_dir.join(&name);
            if fs.exists(&dir.join("schema.yaml")) {
                schemas.push((name, SchemaOrigin::Project(dir)));
            }
        }
    }
    if !schemas.iter().any(|(name, _)| name == BUILTIN_SCHEMA) {
        schemas.push((BUILTIN_SCHEMA.to_string(), SchemaOrigin::Builtin));
    }
    schemas
}

impl ResolvedSchema {
    pub fn name(&self) -> &str {
        &self.graph.schema().name
    }

    /// The content of an artifact's template, if it declares one.
    pub fn template(&self, fs: &dyn FileSystem, artifact: &Artifact) -> Result<Option<String>> {
        let Some(file) = artifact.template.as_deref() else {
            return Ok(None);
        };
        match &self.origin {
            SchemaOrigin::Project(dir) => {
                let path = dir.join("templates").join(file);
                if !fs.exists(&path) {
                    return Err(EngineError::TemplateNotFound {
                        artifact: artifact.id.clone(),
                        template: path.display().to_string(),
                    });
                }
                fs.read_to_string(&path)
                    .map(Some)
                    .map_err(|e| EngineError::Unreadable {
                        path,
                        reason: e.to_string(),
                    })
            }
            SchemaOrigin::Builtin => BUILTIN_TEMPLATES
                .iter()
                .find(|(name, _)| *name == file)
                .map(|(_, contents)| Some(contents.to_string()))
                .ok_or_else(|| EngineError::TemplateNotFound {
                    artifact: artifact.id.clone(),
                    template: file.to_string(),
                }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ports::MemoryFileSystem;

    #[test]
    fn the_embedded_schema_is_valid() {
        // This test guards the asset file: a typo in
        // `assets/schemas/spec-driven/schema.yaml` breaks here, when the
        // tests are built, and not on a user's machine.
        let fs = MemoryFileSystem::new();
        let resolved = resolve(&fs, &Layout::new("/p"), BUILTIN_SCHEMA).unwrap();
        assert_eq!(resolved.name(), "spec-driven");
        assert_eq!(resolved.origin, SchemaOrigin::Builtin);

        let ids: Vec<&str> = resolved
            .graph
            .topological_order()
            .iter()
            .map(|a| a.id.as_str())
            .collect();
        assert_eq!(ids, ["proposal", "specs", "design", "tasks"]);
    }

    #[test]
    fn every_embedded_artifact_has_its_template() {
        let fs = MemoryFileSystem::new();
        let resolved = resolve(&fs, &Layout::new("/p"), BUILTIN_SCHEMA).unwrap();
        for artifact in resolved.graph.artifacts() {
            let template = resolved
                .template(&fs, artifact)
                .unwrap_or_else(|e| panic!("template for `{}`: {e}", artifact.id));
            assert!(
                template.is_some_and(|t| !t.trim().is_empty()),
                "artifact `{}` must have a non-empty template",
                artifact.id
            );
        }
    }

    #[test]
    fn the_project_takes_precedence_over_the_embedded_schema() {
        let fs = MemoryFileSystem::new().with_file(
            "/p/_codev/schemas/spec-driven/schema.yaml",
            "name: spec-driven\nartifacts:\n  - id: proposal\n    generates: proposal.md\napply:\n  requires: [proposal]\n  tracks: proposal.md\n",
        );
        let resolved = resolve(&fs, &Layout::new("/p"), "spec-driven").unwrap();
        assert!(matches!(resolved.origin, SchemaOrigin::Project(_)));
        assert_eq!(resolved.graph.artifacts().len(), 1);
    }

    #[test]
    fn an_unknown_schema_is_an_error_that_guides() {
        let fs = MemoryFileSystem::new();
        let err = resolve(&fs, &Layout::new("/p"), "custom").unwrap_err();
        assert_eq!(err.code(), "schema_not_found");
        assert!(err.to_string().contains("codev schemas"), "{err}");
    }

    #[test]
    fn lists_the_project_then_the_embedded_schema() {
        let fs = MemoryFileSystem::new().with_file(
            "/p/_codev/schemas/custom/schema.yaml",
            "name: custom\nartifacts:\n  - id: a\n    generates: a.md\napply:\n  requires: [a]\n  tracks: a.md\n",
        );
        let names: Vec<String> = list(&fs, &Layout::new("/p"))
            .into_iter()
            .map(|(name, _)| name)
            .collect();
        assert_eq!(names, ["custom", "spec-driven"]);
    }
}
