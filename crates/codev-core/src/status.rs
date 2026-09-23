use std::collections::BTreeSet;

use crate::graph::ArtifactGraph;
use crate::id::ChangeId;

/// L'état d'un artefact.
///
/// Il n'existe **aucun fichier d'état** : l'état est déduit de l'existence des
/// fichiers sur le disque. Un utilisateur qui supprime `design.md` à la main
/// remet cet artefact à `Ready`, sans commande de réparation à connaître.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArtifactState {
    /// Sa sortie existe.
    Done,
    /// Ses dépendances sont satisfaites, il peut être écrit.
    Ready,
    /// Il attend au moins une dépendance.
    Blocked,
    /// Le change l'a neutralisé (`skip_specs`). Il compte comme satisfait, et
    /// ses fichiers ne doivent **pas** être créés.
    Skipped,
}

impl ArtifactState {
    /// Vrai si cet état satisfait une dépendance.
    pub fn satisfies_dependency(self) -> bool {
        matches!(self, Self::Done | Self::Skipped)
    }
}

#[derive(Debug, Clone)]
pub struct ArtifactStatus {
    pub id: String,
    /// Le `generates` du schéma, tel quel — chemin littéral ou motif.
    pub output_path: String,
    pub state: ArtifactState,
    pub requires: Vec<String>,
    /// Les dépendances qui manquent, quand l'état est `Blocked`.
    pub missing_deps: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ChangeStatus {
    pub change: ChangeId,
    pub schema_name: String,
    pub apply_requires: Vec<String>,
    /// En ordre topologique : le premier `Ready` est l'artefact à écrire
    /// maintenant.
    pub artifacts: Vec<ArtifactStatus>,
    /// Vrai quand tous les artefacts de la fermeture requise sont satisfaits.
    /// Ne dit **rien** de l'avancement des tâches d'implémentation.
    pub planning_complete: bool,
}

/// Calcule l'état d'un change. Fonction pure : le disque a déjà été interrogé
/// par l'appelant, qui fournit `existing` et `skipped`.
pub fn compute(
    graph: &ArtifactGraph,
    change: &ChangeId,
    existing: &BTreeSet<String>,
    skipped: &BTreeSet<String>,
) -> ChangeStatus {
    let mut artifacts = Vec::new();
    let mut satisfied: BTreeSet<&str> = BTreeSet::new();

    for artifact in graph.topological_order() {
        let state = if skipped.contains(&artifact.id) {
            ArtifactState::Skipped
        } else if existing.contains(&artifact.id) {
            ArtifactState::Done
        } else if artifact
            .requires
            .iter()
            .all(|d| satisfied.contains(d.as_str()))
        {
            ArtifactState::Ready
        } else {
            ArtifactState::Blocked
        };

        if state.satisfies_dependency() {
            satisfied.insert(artifact.id.as_str());
        }

        let missing_deps = if state == ArtifactState::Blocked {
            artifact
                .requires
                .iter()
                .filter(|d| !satisfied.contains(d.as_str()))
                .cloned()
                .collect()
        } else {
            Vec::new()
        };

        artifacts.push(ArtifactStatus {
            id: artifact.id.clone(),
            output_path: artifact.generates.clone(),
            state,
            requires: artifact.requires.clone(),
            missing_deps,
        });
    }

    let closure = graph.required_closure();
    let planning_complete = artifacts
        .iter()
        .filter(|a| closure.contains(&a.id))
        .all(|a| a.state.satisfies_dependency());

    ChangeStatus {
        change: change.clone(),
        schema_name: graph.schema().name.clone(),
        apply_requires: graph.schema().apply.requires.clone(),
        artifacts,
        planning_complete,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    fn set(ids: &[&str]) -> BTreeSet<String> {
        ids.iter().map(|s| s.to_string()).collect()
    }

    fn status(existing: &[&str], skipped: &[&str]) -> ChangeStatus {
        let graph = ArtifactGraph::from_yaml(SPEC_DRIVEN).unwrap();
        let change = ChangeId::parse("add-auth").unwrap();
        compute(&graph, &change, &set(existing), &set(skipped))
    }

    fn state_of(status: &ChangeStatus, id: &str) -> ArtifactState {
        status
            .artifacts
            .iter()
            .find(|a| a.id == id)
            .unwrap_or_else(|| panic!("artefact « {id} » absent du statut"))
            .state
    }

    #[test]
    fn un_change_vide_na_que_sa_racine_de_prete() {
        let status = status(&[], &[]);
        assert_eq!(state_of(&status, "proposal"), ArtifactState::Ready);
        assert_eq!(state_of(&status, "specs"), ArtifactState::Blocked);
        assert_eq!(state_of(&status, "tasks"), ArtifactState::Blocked);
        assert!(!status.planning_complete);
    }

    #[test]
    fn le_premier_pret_est_lartefact_a_ecrire() {
        let status = status(&["proposal"], &[]);
        let premier_pret = status
            .artifacts
            .iter()
            .find(|a| a.state == ArtifactState::Ready)
            .expect("il doit rester quelque chose à écrire");
        assert_eq!(premier_pret.id, "specs");
    }

    #[test]
    fn nomme_les_dependances_manquantes() {
        let status = status(&["proposal", "specs"], &[]);
        let tasks = status.artifacts.iter().find(|a| a.id == "tasks").unwrap();
        assert_eq!(tasks.state, ArtifactState::Blocked);
        assert_eq!(tasks.missing_deps, ["design"]);
    }

    #[test]
    fn un_artefact_saute_satisfait_ses_dependants() {
        // C'est le cas `skip_specs` : `specs` n'existera jamais, et `tasks` doit
        // pourtant devenir écrivable.
        let status = status(&["proposal", "design"], &["specs"]);
        assert_eq!(state_of(&status, "specs"), ArtifactState::Skipped);
        assert_eq!(state_of(&status, "tasks"), ArtifactState::Ready);
    }

    #[test]
    fn la_planification_est_complete_quand_la_fermeture_est_satisfaite() {
        let status = status(&["proposal", "specs", "design", "tasks"], &[]);
        assert!(status.planning_complete);
    }

    #[test]
    fn ecrire_tasks_en_premier_ne_rend_pas_la_planification_complete() {
        // Le piège que le contrat doit rendre visible : `status` ne regarde que
        // l'existence des fichiers, donc `tasks` est `Done` alors que `specs` et
        // `design` n'ont jamais été écrits.
        let status = status(&["tasks"], &[]);
        assert_eq!(state_of(&status, "tasks"), ArtifactState::Done);
        assert!(
            !status.planning_complete,
            "la fermeture requise n'est pas satisfaite"
        );
    }
}
