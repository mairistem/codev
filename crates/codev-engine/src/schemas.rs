use std::path::PathBuf;

use codev_core::schema::Artifact;
use codev_core::{ArtifactGraph, Layout};

use crate::error::{EngineError, Result};
use crate::ports::FileSystem;

/// Le seul schéma embarqué. Tout le reste vient du projet.
pub const BUILTIN_SCHEMA: &str = "spec-driven";

const BUILTIN_SCHEMA_YAML: &str = include_str!("../../../assets/schemas/spec-driven/schema.yaml");

/// Les templates du schéma embarqué, inclus à la compilation.
///
/// Un binaire autoportant : `codev init` fonctionne sans rien télécharger, et
/// une installation ne peut pas se retrouver avec un schéma d'une version et
/// des templates d'une autre.
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
    /// Défini par le projet, dans `_codev/schemas/<nom>/`.
    Project(PathBuf),
    /// Embarqué dans le binaire.
    Builtin,
}

impl SchemaOrigin {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Project(_) => "projet",
            Self::Builtin => "intégré",
        }
    }
}

#[derive(Debug)]
pub struct ResolvedSchema {
    pub graph: ArtifactGraph,
    pub origin: SchemaOrigin,
}

/// Résout un schéma par son nom.
///
/// Précédence : le projet d'abord, l'embarqué ensuite. Un projet peut donc
/// remplacer `spec-driven` par sa propre version sans changer de nom, et sans
/// que rien d'autre ne bouge.
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
            // Un échec ici serait un bug de compilation de notre côté, pas une
            // erreur de l'utilisateur : le fichier est embarqué.
            graph: ArtifactGraph::from_yaml(BUILTIN_SCHEMA_YAML)?,
            origin: SchemaOrigin::Builtin,
        });
    }

    Err(EngineError::SchemaNotFound {
        name: name.to_string(),
    })
}

/// Les schémas disponibles, ceux du projet en premier.
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

    /// Le contenu du template d'un artefact, s'il en déclare un.
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
    fn le_schema_embarque_est_valide() {
        // Ce test est le garde-fou du fichier d'assets : une faute de frappe
        // dans `assets/schemas/spec-driven/schema.yaml` casse ici, à la
        // compilation des tests, et non chez un utilisateur.
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
    fn chaque_artefact_embarque_a_son_template() {
        let fs = MemoryFileSystem::new();
        let resolved = resolve(&fs, &Layout::new("/p"), BUILTIN_SCHEMA).unwrap();
        for artifact in resolved.graph.artifacts() {
            let template = resolved
                .template(&fs, artifact)
                .unwrap_or_else(|e| panic!("template de « {} » : {e}", artifact.id));
            assert!(
                template.is_some_and(|t| !t.trim().is_empty()),
                "l'artefact « {} » doit avoir un template non vide",
                artifact.id
            );
        }
    }

    #[test]
    fn le_projet_prime_sur_lembarque() {
        let fs = MemoryFileSystem::new().with_file(
            "/p/_codev/schemas/spec-driven/schema.yaml",
            "name: spec-driven\nartifacts:\n  - id: proposal\n    generates: proposal.md\napply:\n  requires: [proposal]\n  tracks: proposal.md\n",
        );
        let resolved = resolve(&fs, &Layout::new("/p"), "spec-driven").unwrap();
        assert!(matches!(resolved.origin, SchemaOrigin::Project(_)));
        assert_eq!(resolved.graph.artifacts().len(), 1);
    }

    #[test]
    fn un_schema_inconnu_est_une_erreur_qui_oriente() {
        let fs = MemoryFileSystem::new();
        let err = resolve(&fs, &Layout::new("/p"), "maison").unwrap_err();
        assert_eq!(err.code(), "schema_not_found");
        assert!(err.to_string().contains("codev schemas"), "{err}");
    }

    #[test]
    fn liste_le_projet_puis_lembarque() {
        let fs = MemoryFileSystem::new().with_file(
            "/p/_codev/schemas/maison/schema.yaml",
            "name: maison\nartifacts:\n  - id: a\n    generates: a.md\napply:\n  requires: [a]\n  tracks: a.md\n",
        );
        let noms: Vec<String> = list(&fs, &Layout::new("/p"))
            .into_iter()
            .map(|(name, _)| name)
            .collect();
        assert_eq!(noms, ["maison", "spec-driven"]);
    }
}
