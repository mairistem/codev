use std::path::PathBuf;

use codev_core::keywords::Lang;
use codev_core::schema::Artifact;
use codev_core::{ArtifactGraph, Layout};

use crate::error::{EngineError, Result};
use crate::ports::FileSystem;

/// The only embedded schema. Everything else comes from the project.
pub const BUILTIN_SCHEMA: &str = "spec-driven";

const BUILTIN_SCHEMA_YAML: &str = include_str!("../../../assets/schemas/spec-driven/schema.yaml");

/// The embedded schema's templates, included at compile time, one set per
/// language of the keyword catalog: `(language, file, contents)`.
///
/// A self-contained binary: `codev init` works without downloading anything,
/// and an installation can never end up with a schema from one version and
/// templates from another.
const BUILTIN_TEMPLATES: &[(&str, &str, &str)] = &[
    (
        "en",
        "proposal.md",
        include_str!("../../../assets/schemas/spec-driven/templates/en/proposal.md"),
    ),
    (
        "en",
        "spec.md",
        include_str!("../../../assets/schemas/spec-driven/templates/en/spec.md"),
    ),
    (
        "en",
        "design.md",
        include_str!("../../../assets/schemas/spec-driven/templates/en/design.md"),
    ),
    (
        "en",
        "tasks.md",
        include_str!("../../../assets/schemas/spec-driven/templates/en/tasks.md"),
    ),
    (
        "fr",
        "proposal.md",
        include_str!("../../../assets/schemas/spec-driven/templates/fr/proposal.md"),
    ),
    (
        "fr",
        "spec.md",
        include_str!("../../../assets/schemas/spec-driven/templates/fr/spec.md"),
    ),
    (
        "fr",
        "design.md",
        include_str!("../../../assets/schemas/spec-driven/templates/fr/design.md"),
    ),
    (
        "fr",
        "tasks.md",
        include_str!("../../../assets/schemas/spec-driven/templates/fr/tasks.md"),
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
    /// The name the schema was resolved by — the folder of a project
    /// schema. It is the schema's identity: recorded in `change.yaml` and
    /// shown as `schemaName`, so that what is written always resolves again.
    name: String,
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
            name: name.to_string(),
            graph,
            origin: SchemaOrigin::Project(project_dir),
        });
    }

    if name == BUILTIN_SCHEMA {
        return Ok(ResolvedSchema {
            name: name.to_string(),
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
    /// The schema's name: the one it was resolved by.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The `name:` written inside `schema.yaml`, which may differ from the
    /// folder; only `codev schemas` looks at it, to report the mismatch.
    pub fn declared_name(&self) -> &str {
        &self.graph.schema().name
    }

    /// The content of an artifact's template in the keyword language
    /// `lang`, if it declares one. A project schema serves
    /// `templates/<lang>/<file>`, else `templates/<file>` in every
    /// language; the built-in schema ships a file per catalog language.
    pub fn template(
        &self,
        fs: &dyn FileSystem,
        artifact: &Artifact,
        lang: Lang,
    ) -> Result<Option<String>> {
        let Some(file) = artifact.template.as_deref() else {
            return Ok(None);
        };
        match &self.origin {
            SchemaOrigin::Project(dir) => {
                let templates = dir.join("templates");
                let per_language = templates.join(lang.code()).join(file);
                let path = if fs.exists(&per_language) {
                    per_language
                } else {
                    templates.join(file)
                };
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
            SchemaOrigin::Builtin => {
                let find = |code: &str| {
                    BUILTIN_TEMPLATES
                        .iter()
                        .find(|(language, name, _)| *language == code && *name == file)
                };
                find(lang.code())
                    .or_else(|| find(Lang::EN.code()))
                    .map(|(_, _, contents)| Some(contents.to_string()))
                    .ok_or_else(|| EngineError::TemplateNotFound {
                        artifact: artifact.id.clone(),
                        template: file.to_string(),
                    })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ports::MemoryFileSystem;
    use codev_core::keywords::{self, Key};

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
                .template(&fs, artifact, Lang::EN)
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
    fn a_resolved_schema_is_named_by_its_folder() {
        // The folder is what resolution goes by; the `name:` inside the file
        // must not leak into `change.yaml` or `schemaName`.
        let fs = MemoryFileSystem::new().with_file(
            "/p/_codev/schemas/lite/schema.yaml",
            "name: quick-flow\nartifacts:\n  - id: brief\n    generates: brief.md\napply:\n  requires: [brief]\n  tracks: brief.md\n",
        );
        let resolved = resolve(&fs, &Layout::new("/p"), "lite").unwrap();
        assert_eq!(resolved.name(), "lite");
        assert_eq!(resolved.declared_name(), "quick-flow");
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

    #[test]
    fn a_project_spec_driven_is_listed_once_as_project() {
        let fs = MemoryFileSystem::new().with_file(
            "/p/_codev/schemas/spec-driven/schema.yaml",
            "name: spec-driven\nartifacts:\n  - id: proposal\n    generates: proposal.md\napply:\n  requires: [proposal]\n  tracks: proposal.md\n",
        );
        let listed = list(&fs, &Layout::new("/p"));
        assert_eq!(listed.len(), 1);
        assert_eq!(listed[0].0, "spec-driven");
        assert_eq!(listed[0].1.label(), "project");
    }

    fn fr() -> Lang {
        Lang::all().find(|l| l.code() == "fr").unwrap()
    }

    /// The structural keyword a template line holds, and the language it is
    /// written in.
    fn keyword_of(line: &str) -> Option<(Key, Lang)> {
        use codev_core::keywords::{label, named_heading, section_line};
        Key::ALL.into_iter().find_map(|key| {
            section_line(line, key)
                .or_else(|| named_heading(line, key).map(|(lang, _)| lang))
                .or_else(|| label(line, key).map(|(lang, _)| lang))
                .map(|lang| (key, lang))
        })
    }

    #[test]
    fn every_embedded_template_uses_its_languages_keywords() {
        for (code, file, contents) in BUILTIN_TEMPLATES {
            let lang = Lang::all().find(|l| l.code() == *code).unwrap();
            let mut found = 0;
            for line in contents.lines() {
                let item = line.trim_start_matches("- ");
                let Some((key, written)) = keyword_of(item) else {
                    continue;
                };
                found += 1;
                assert_eq!(written, lang, "{code}/{file}: `{line}` is not in `{code}`");
                let text = keywords::text(key, lang);
                assert!(
                    item.trim_start().starts_with(text),
                    "{code}/{file}: `{line}` does not write `{text}` as the catalog does"
                );
            }
            // The tasks template holds no structural keyword.
            if *file != "tasks.md" {
                assert!(found > 0, "{code}/{file} holds no keyword");
            }
        }
    }

    #[test]
    fn proposal_templates_list_the_domains_codev_knows() {
        for (code, file, contents) in BUILTIN_TEMPLATES {
            if *file != "proposal.md" {
                continue;
            }
            let entries = codev_core::domains::entries(contents);
            assert_eq!(entries.len(), 6, "{code}: {entries:?}");
            assert!(entries.iter().all(|e| !e.ticked), "{code}");
        }
    }

    #[test]
    fn english_templates_are_unchanged() {
        use sha2::{Digest, Sha256};
        // The files before they moved to `templates/en/`.
        let before = [
            (
                "proposal.md",
                "e6349f37b241e23846549239f1530aa13094461e7d14d17b03255d6594839a4f",
            ),
            (
                "spec.md",
                "5e7acfeef40e5f3bf97f9ec218a2970a73a025cc42cedb450ddd12c9936915e0",
            ),
            (
                "design.md",
                "e37c06b8a0188cd8d75d34442b93ab666d6331935d76d5d10388fbe7312fe853",
            ),
            (
                "tasks.md",
                "beec0b815ede873aa4accd3553013bd0cf7beff02a10766b5a89e7610d1071bd",
            ),
        ];
        for (file, hash) in before {
            let (_, _, contents) = BUILTIN_TEMPLATES
                .iter()
                .find(|(code, name, _)| *code == "en" && *name == file)
                .unwrap();
            let digest: String = Sha256::digest(contents.as_bytes())
                .iter()
                .map(|b| format!("{b:02x}"))
                .collect();
            assert_eq!(digest, hash, "{file}");
        }
    }

    fn proposal_artifact(resolved: &ResolvedSchema) -> Artifact {
        resolved
            .graph
            .artifacts()
            .iter()
            .find(|a| a.id == "proposal")
            .unwrap()
            .clone()
    }

    #[test]
    fn builtin_template_in_french() {
        let fs = MemoryFileSystem::new();
        let resolved = resolve(&fs, &Layout::new("/p"), BUILTIN_SCHEMA).unwrap();
        let proposal = proposal_artifact(&resolved);
        let fr_text = resolved.template(&fs, &proposal, fr()).unwrap().unwrap();
        assert!(fr_text.contains("## Changements"), "{fr_text}");
        let en_text = resolved
            .template(&fs, &proposal, Lang::EN)
            .unwrap()
            .unwrap();
        assert!(en_text.contains("## What Changes"), "{en_text}");
    }

    const PROJECT_SCHEMA: &str = "name: lite\nartifacts:\n  - id: proposal\n    generates: proposal.md\n    template: proposal.md\napply:\n  requires: [proposal]\n  tracks: proposal.md\n";

    #[test]
    fn project_schema_template_per_language() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/schemas/lite/schema.yaml", PROJECT_SCHEMA)
            .with_file(
                "/p/_codev/schemas/lite/templates/fr/proposal.md",
                "# Proposition\n",
            )
            .with_file(
                "/p/_codev/schemas/lite/templates/proposal.md",
                "# Proposal\n",
            );
        let resolved = resolve(&fs, &Layout::new("/p"), "lite").unwrap();
        let proposal = proposal_artifact(&resolved);
        assert_eq!(
            resolved.template(&fs, &proposal, fr()).unwrap().unwrap(),
            "# Proposition\n"
        );
        assert_eq!(
            resolved
                .template(&fs, &proposal, Lang::EN)
                .unwrap()
                .unwrap(),
            "# Proposal\n"
        );
    }

    #[test]
    fn project_schema_single_template_is_served_in_every_language() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/schemas/lite/schema.yaml", PROJECT_SCHEMA)
            .with_file(
                "/p/_codev/schemas/lite/templates/proposal.md",
                "# Proposal\n",
            );
        let resolved = resolve(&fs, &Layout::new("/p"), "lite").unwrap();
        let proposal = proposal_artifact(&resolved);
        for lang in Lang::all() {
            assert_eq!(
                resolved.template(&fs, &proposal, lang).unwrap().unwrap(),
                "# Proposal\n"
            );
        }
    }

    #[test]
    fn project_schema_without_any_template_fails() {
        let fs =
            MemoryFileSystem::new().with_file("/p/_codev/schemas/lite/schema.yaml", PROJECT_SCHEMA);
        let resolved = resolve(&fs, &Layout::new("/p"), "lite").unwrap();
        let proposal = proposal_artifact(&resolved);
        let err = resolved.template(&fs, &proposal, fr()).unwrap_err();
        assert_eq!(err.code(), "template_not_found");
    }
}
