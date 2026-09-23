use std::collections::BTreeSet;

use serde::Deserialize;

use crate::error::{CoreError, Result};
use crate::id::is_kebab_case;

/// Définition d'un workflow : quels artefacts existent, et lesquels dépendent
/// de lesquels.
///
/// C'est de la **donnée**, jamais du code. Un utilisateur doit pouvoir modifier
/// un `schema.yaml` ou un template et en voir l'effet immédiatement, sans
/// attendre une release. C'est la principale leçon retenue d'OpenSpec, dont le
/// workflow historique enfouissait ses instructions dans le binaire.
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
    /// Chemin de sortie relatif au dossier du change. Accepte un motif glob
    /// (`specs/**/*.md`) pour les artefacts qui produisent plusieurs fichiers.
    pub generates: String,
    #[serde(default)]
    pub description: Option<String>,
    /// Nom du fichier de template, cherché dans le `templates/` du schéma.
    #[serde(default)]
    pub template: Option<String>,
    #[serde(default)]
    pub instruction: Option<String>,
    #[serde(default)]
    pub requires: Vec<String>,
}

impl Artifact {
    /// Vrai si `generates` est un motif et non un chemin littéral.
    pub fn is_pattern(&self) -> bool {
        self.generates.contains('*') || self.generates.contains('?')
    }
}

/// La phase d'implémentation. Elle n'est pas un artefact : elle ne produit pas
/// de fichier de planification, elle en consomme.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Apply {
    pub requires: Vec<String>,
    /// Le fichier dont les cases à cocher suivent l'avancement.
    pub tracks: String,
    #[serde(default)]
    pub instruction: Option<String>,
}

impl Schema {
    /// Analyse un `schema.yaml`.
    ///
    /// `deny_unknown_fields` est délibéré : ces fichiers sont écrits à la main,
    /// et une clé mal orthographiée qui serait silencieusement ignorée
    /// produirait un workflow qui ne fait pas ce que son auteur croit.
    pub fn parse(yaml: &str) -> Result<Self> {
        let schema: Self = serde_norway::from_str(yaml)
            .map_err(|e| CoreError::SchemaUnreadable(e.to_string()))?;
        schema.validate()?;
        Ok(schema)
    }

    fn invalid(&self, reason: impl Into<String>) -> CoreError {
        CoreError::SchemaInvalid {
            schema: self.name.clone(),
            reason: reason.into(),
        }
    }

    /// Vérifie tout ce qui rendrait le graphe inexploitable.
    pub fn validate(&self) -> Result<()> {
        if self.name.is_empty() {
            return Err(self.invalid("le champ `name` est vide"));
        }
        if !is_kebab_case(&self.name) {
            return Err(self.invalid(format!(
                "le nom « {} » n'est pas en kebab-case",
                self.name
            )));
        }
        if self.artifacts.is_empty() {
            return Err(self.invalid("aucun artefact déclaré"));
        }

        let mut seen = BTreeSet::new();
        for artifact in &self.artifacts {
            if !is_kebab_case(&artifact.id) {
                return Err(self.invalid(format!(
                    "l'identifiant d'artefact « {} » n'est pas en kebab-case",
                    artifact.id
                )));
            }
            if !seen.insert(artifact.id.as_str()) {
                return Err(self.invalid(format!(
                    "l'artefact « {} » est déclaré deux fois",
                    artifact.id
                )));
            }
            if artifact.generates.is_empty() {
                return Err(self.invalid(format!(
                    "l'artefact « {} » ne déclare pas de `generates`",
                    artifact.id
                )));
            }
        }

        for artifact in &self.artifacts {
            for dep in &artifact.requires {
                if !seen.contains(dep.as_str()) {
                    return Err(self.invalid(format!(
                        "l'artefact « {} » dépend de « {dep} », qui n'existe pas",
                        artifact.id
                    )));
                }
                if dep == &artifact.id {
                    return Err(self.invalid(format!(
                        "l'artefact « {} » dépend de lui-même",
                        artifact.id
                    )));
                }
            }
        }

        if self.apply.requires.is_empty() {
            return Err(self.invalid("`apply.requires` est vide : rien ne déclencherait l'implémentation"));
        }
        for dep in &self.apply.requires {
            if !seen.contains(dep.as_str()) {
                return Err(self.invalid(format!(
                    "`apply.requires` mentionne « {dep} », qui n'est pas un artefact du schéma"
                )));
            }
        }
        if self.apply.tracks.is_empty() {
            return Err(self.invalid("`apply.tracks` est vide"));
        }

        self.assert_acyclic()
    }

    /// Détecte un cycle de dépendances.
    ///
    /// Un cycle est fatal et non réparable automatiquement : le message nomme
    /// donc les artefacts impliqués, sans quoi l'auteur du schéma n'a aucune
    /// piste.
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
                // Plus rien ne progresse : ce qui reste est dans un cycle ou en
                // dépend.
                None => break,
            }
        }
        if settled.len() == self.artifacts.len() {
            return Ok(());
        }
        let bloques: Vec<&str> = self
            .artifacts
            .iter()
            .map(|a| a.id.as_str())
            .filter(|id| !settled.contains(id))
            .collect();
        Err(self.invalid(format!(
            "cycle de dépendances entre : {}",
            bloques.join(", ")
        )))
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
    fn analyse_un_schema_minimal() {
        let schema = Schema::parse(MINIMAL).expect("devrait être valide");
        assert_eq!(schema.name, "minimal");
        assert_eq!(schema.version, 1, "la version doit valoir 1 par défaut");
        assert_eq!(schema.artifacts.len(), 2);
        assert_eq!(schema.apply.tracks, "tasks.md");
    }

    #[test]
    fn refuse_une_cle_inconnue() {
        let yaml = MINIMAL.replace("name: minimal", "name: minimal\nartefacts: []");
        let err = Schema::parse(&yaml).unwrap_err();
        assert_eq!(err.code(), "schema_unreadable");
    }

    #[test]
    fn refuse_une_dependance_fantome() {
        let yaml = MINIMAL.replace("requires: [proposal]", "requires: [design]");
        let err = Schema::parse(&yaml).unwrap_err();
        assert_eq!(err.code(), "schema_invalid");
        assert!(err.to_string().contains("design"));
    }

    #[test]
    fn refuse_un_cycle_et_nomme_les_coupables() {
        let yaml = r#"
name: boucle
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
    fn refuse_un_artefact_duplique() {
        let yaml = r#"
name: doublon
artifacts:
  - id: a
    generates: a.md
  - id: a
    generates: autre.md
apply:
  requires: [a]
  tracks: a.md
"#;
        let err = Schema::parse(yaml).unwrap_err();
        assert!(err.to_string().contains("deux fois"), "{err}");
    }

    #[test]
    fn reconnait_un_motif_glob() {
        let schema = Schema::parse(MINIMAL).unwrap();
        assert!(!schema.artifact("proposal").unwrap().is_pattern());

        let yaml = MINIMAL.replace("generates: proposal.md", "generates: \"specs/**/*.md\"");
        let schema = Schema::parse(&yaml).unwrap();
        assert!(schema.artifact("proposal").unwrap().is_pattern());
    }
}
