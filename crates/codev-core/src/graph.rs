use std::collections::{BTreeSet, VecDeque};

use crate::error::Result;
use crate::schema::{Artifact, Schema};

/// Le graphe de dépendances entre artefacts d'un schéma.
///
/// Les dépendances sont des **activateurs, pas des barrières** : elles disent ce
/// qu'il est possible de créer, pas ce qu'il faut créer ensuite. Un `design.md`
/// peut être sauté ; ce qui en dépend reste écrivable.
#[derive(Debug)]
pub struct ArtifactGraph {
    schema: Schema,
}

impl ArtifactGraph {
    /// Construit le graphe à partir d'un schéma déjà désérialisé.
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

    /// Ordre topologique : une dépendance n'apparaît jamais après ce qui en
    /// dépend.
    ///
    /// Les égalités sont tranchées par l'**ordre de déclaration** du schéma, pas
    /// alphabétiquement. Le graphe laisse `specs` et `design` à égalité — tous
    /// deux ne dépendent que de `proposal` — et un tri alphabétique placerait
    /// `design` devant, contredisant la séquence que le schéma annonce lui-même
    /// (`proposal → specs → design → tasks`). Suivre l'ordre d'écriture de
    /// l'auteur est tout aussi déterministe et ne le trahit pas.
    pub fn topological_order(&self) -> Vec<&Artifact> {
        let mut settled: BTreeSet<&str> = BTreeSet::new();
        let mut order = Vec::with_capacity(self.schema.artifacts.len());
        // Boucle quadratique assumée : un schéma compte une poignée
        // d'artefacts, et cette forme rend le critère de départage évident.
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
                // `Schema::validate` a déjà écarté les cycles, donc on ne sort
                // ici qu'une fois tout le monde placé.
                None => break,
            }
        }
        order
    }

    /// L'ensemble des artefacts dont l'implémentation dépend, transitivement.
    ///
    /// `apply.requires` ne suffit pas : avec `spec-driven` il ne nomme que
    /// `tasks`, alors que `tasks` dépend de `specs` et `design`, qui dépendent
    /// de `proposal`. Un agent qui se fierait à la seule liste `apply.requires`
    /// écrirait `tasks.md` et s'arrêterait là.
    pub fn required_closure(&self) -> BTreeSet<String> {
        let mut closure = BTreeSet::new();
        let mut queue: VecDeque<&str> =
            self.schema.apply.requires.iter().map(String::as_str).collect();
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

    /// Les artefacts que la création de `id` rend possibles.
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

    /// La forme de `spec-driven` : `specs` et `design` sont à égalité, et
    /// `specs` est déclaré en premier.
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
        ArtifactGraph::from_yaml(SPEC_DRIVEN).expect("schéma de test valide")
    }

    fn ordered_ids(graph: &ArtifactGraph) -> Vec<&str> {
        graph
            .topological_order()
            .iter()
            .map(|a| a.id.as_str())
            .collect()
    }

    #[test]
    fn ordonne_selon_les_dependances() {
        let graph = graph();
        assert_eq!(
            ordered_ids(&graph),
            ["proposal", "specs", "design", "tasks"]
        );
    }

    #[test]
    fn tranche_les_egalites_par_ordre_de_declaration_et_non_alphabetiquement() {
        // Le piège : « design » précède « specs » dans l'alphabet. Si le tri
        // alphabétique se réinstallait un jour, ce test tomberait.
        let graph = graph();
        let ids = ordered_ids(&graph);
        let position = |id: &str| ids.iter().position(|x| *x == id).unwrap();
        assert!(
            position("specs") < position("design"),
            "l'ordre obtenu est {ids:?}"
        );
    }

    #[test]
    fn ferme_transitivement_sur_apply_requires() {
        let closure = graph().required_closure();
        let attendu: BTreeSet<String> = ["proposal", "specs", "design", "tasks"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(closure, attendu);
    }

    #[test]
    fn dit_ce_que_debloque_un_artefact() {
        assert_eq!(graph().unlocked_by("proposal"), ["specs", "design"]);
        assert_eq!(graph().unlocked_by("specs"), ["tasks"]);
        assert!(graph().unlocked_by("tasks").is_empty());
    }
}
