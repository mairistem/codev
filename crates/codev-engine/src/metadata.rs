use std::collections::BTreeSet;
use std::path::Path;

use codev_core::ArtifactGraph;
use serde::{Deserialize, Serialize};

use crate::error::{EngineError, Result};
use crate::ports::FileSystem;

/// Le `change.yaml` d'un change.
///
/// Il vit dans le dossier du change et se versionne avec lui : c'est une
/// décision de l'auteur du change, pas un état de machine.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChangeMetadata {
    pub schema: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub goal: Option<String>,

    /// Déclare que ce change n'a volontairement aucun delta de spec :
    /// refactor pur, outillage, documentation.
    ///
    /// Sans ce marqueur, la validation refuse un change à zéro delta — ce qui
    /// est voulu : c'est ce qui empêche d'oublier les specs. Avec, elle
    /// l'accepte. L'un ou l'autre, mais jamais l'invention d'une exigence pour
    /// satisfaire l'outil.
    #[serde(default, skip_serializing_if = "is_false")]
    pub skip_specs: bool,

    /// Autorise `sync` et `archive` à supprimer la spec d'une capacité dont
    /// ce change retire la dernière exigence.
    ///
    /// Explicite parce que la suppression n'est récupérable que depuis git :
    /// c'est un choix de l'auteur, pas une déduction à partir de la forme d'un
    /// delta. Sans ce marqueur, un delta `## REMOVED Requirements` qui
    /// viderait la spec est refusé avec `would_leave_spec_without_requirement`.
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

    /// Les artefacts que ce change neutralise.
    ///
    /// La règle porte sur le **préfixe du chemin de sortie** (`specs/`), pas sur
    /// l'identifiant `specs` : un schéma maison qui nomme son artefact
    /// autrement mais écrit sous `specs/` hérite du même comportement, sans
    /// avoir à le savoir.
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
        // Une sérialisation qui échoue ici serait un bug de nos types, pas une
        // erreur d'utilisateur : la structure n'a que des scalaires.
        serde_norway::to_string(self).expect("les métadonnées de change sont sérialisables")
    }
}

/// Lit le `change.yaml`. Absent, il n'est pas une erreur : un change créé à la
/// main peut n'en avoir aucun, et le schéma vient alors de la config du projet.
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
    fn sans_skip_specs_rien_nest_neutralise() {
        let metadata = ChangeMetadata::new("spec-driven", "2026-09-08");
        assert!(metadata.skipped_artifacts(&graph()).is_empty());
    }

    #[test]
    fn skip_specs_neutralise_par_prefixe_de_chemin() {
        let mut metadata = ChangeMetadata::new("spec-driven", "2026-09-08");
        metadata.skip_specs = true;
        let skipped = metadata.skipped_artifacts(&graph());
        assert_eq!(skipped.len(), 1);
        assert!(skipped.contains("specs"));
    }

    #[test]
    fn skip_specs_suit_le_chemin_et_non_lidentifiant() {
        // Un schéma maison qui appelle son artefact « contrats » mais écrit
        // sous `specs/` doit être neutralisé lui aussi.
        let yaml = SPEC_DRIVEN.replace("id: specs", "id: contrats").replace(
            "requires: [specs]",
            "requires: [contrats]",
        );
        let graph = ArtifactGraph::from_yaml(&yaml).unwrap();
        let mut metadata = ChangeMetadata::new("spec-driven", "2026-09-08");
        metadata.skip_specs = true;
        assert!(metadata.skipped_artifacts(&graph).contains("contrats"));
    }

    #[test]
    fn le_yaml_omet_les_champs_vides() {
        let yaml = ChangeMetadata::new("spec-driven", "2026-09-08").to_yaml();
        assert!(yaml.contains("schema: spec-driven"));
        // La date sort sans guillemets. C'est sans conséquence ici — nous la
        // relisons en `String`, ce que vérifie `relit_ce_quil_ecrit`.
        assert!(yaml.contains("created: 2026-09-08"), "{yaml}");
        assert!(!yaml.contains("goal"), "un goal absent ne s'écrit pas : {yaml}");
        assert!(!yaml.contains("skip_specs"), "un faux ne s'écrit pas : {yaml}");
    }

    #[test]
    fn relit_ce_quil_ecrit() {
        let origine = ChangeMetadata::new("spec-driven", "2026-09-08")
            .with_goal(Some("Ajouter l'authentification".into()));
        let fs = MemoryFileSystem::new().with_file("/c/change.yaml", origine.to_yaml());
        let relu = load(&fs, Path::new("/c/change.yaml")).unwrap().unwrap();
        assert_eq!(relu.schema, "spec-driven");
        assert_eq!(relu.goal.as_deref(), Some("Ajouter l'authentification"));
        assert!(!relu.skip_specs);
    }

    #[test]
    fn refuse_une_cle_inconnue() {
        let fs = MemoryFileSystem::new()
            .with_file("/c/change.yaml", "schema: spec-driven\nskipspecs: true\n");
        let err = load(&fs, Path::new("/c/change.yaml")).unwrap_err();
        assert_eq!(err.code(), "invalid");
        assert!(err.to_string().contains("skipspecs"), "{err}");
    }
}
