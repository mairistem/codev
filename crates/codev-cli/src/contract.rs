//! Le contrat JSON, version 1.
//!
//! **C'est une API publique.** Elle est consommée par des skills déjà
//! installées chez les utilisateurs, qui ne se régénèrent pas quand on
//! recompile. D'où la règle : ces types ne dérivent pas du modèle de domaine,
//! ils le traduisent. Un refactor interne peut donc casser une conversion, ce
//! que le compilateur signale, plutôt que la forme du JSON, que personne ne
//! remarquerait avant l'utilisateur.
//!
//! Deux invariants tenus par tout le module :
//! - stdout ne porte **qu'un seul** document JSON, y compris en cas d'échec ;
//! - tout document porte un tableau `status`, éventuellement vide.

use codev_core::{ArtifactState, ChangeStatus};
use codev_engine::config::Block;
use codev_engine::instructions::Instructions;
use codev_engine::Warning;
use serde::Serialize;
use serde_json::json;

/// Une entrée du tableau `status` : un constat, pas de la prose.
///
/// `code` est stable et fait pour être testé par un consommateur ; `message`
/// est fait pour être lu, et peut être reformulé sans préavis.
#[derive(Debug, Serialize)]
pub struct StatusEntry {
    pub level: &'static str,
    pub code: String,
    pub message: String,
}

impl StatusEntry {
    pub fn error(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            level: "error",
            code: code.into(),
            message: message.into(),
        }
    }

    pub fn warning(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            level: "warning",
            code: code.into(),
            message: message.into(),
        }
    }
}

pub fn statuses(warnings: &[Warning]) -> Vec<StatusEntry> {
    warnings
        .iter()
        .map(|w| StatusEntry::warning(w.code, w.message.clone()))
        .collect()
}

fn state_label(state: ArtifactState) -> &'static str {
    match state {
        ArtifactState::Done => "done",
        ArtifactState::Ready => "ready",
        ArtifactState::Blocked => "blocked",
        ArtifactState::Skipped => "skipped",
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtifactV1 {
    pub id: String,
    pub output_path: String,
    pub status: &'static str,
    pub requires: Vec<String>,
    pub missing_deps: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusV1 {
    pub change_name: String,
    pub schema_name: String,
    /// La racine du projet, résolue. Les skills doivent s'en servir plutôt que
    /// de supposer un chemin relatif au dossier courant.
    pub planning_home: String,
    pub change_root: String,
    pub apply_requires: Vec<String>,
    pub is_planning_complete: bool,
    pub artifacts: Vec<ArtifactV1>,
    pub status: Vec<StatusEntry>,
}

impl StatusV1 {
    pub fn new(
        status: &ChangeStatus,
        planning_home: String,
        change_root: String,
        warnings: &[Warning],
    ) -> Self {
        Self {
            change_name: status.change.to_string(),
            schema_name: status.schema_name.clone(),
            planning_home,
            change_root,
            apply_requires: status.apply_requires.clone(),
            is_planning_complete: status.planning_complete,
            artifacts: status
                .artifacts
                .iter()
                .map(|a| ArtifactV1 {
                    id: a.id.clone(),
                    output_path: a.output_path.clone(),
                    status: state_label(a.state),
                    requires: a.requires.clone(),
                    missing_deps: a.missing_deps.clone(),
                })
                .collect(),
            status: statuses(warnings),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct BlockV1 {
    pub origin: String,
    pub text: String,
}

impl From<&Block> for BlockV1 {
    fn from(block: &Block) -> Self {
        Self {
            origin: block.origin.clone(),
            text: block.text.clone(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct DependencyV1 {
    pub id: String,
    pub path: String,
    pub done: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DecisionRefV1 {
    pub id: String,
    pub qualified_id: String,
    pub title: String,
    pub status: String,
    pub tags: Vec<String>,
    pub path: String,
    pub origin: String,
}

/// La forme complète d'une décision — utilisée par `list` (chaque entrée
/// du tableau) et par `show` (au niveau racine). Un consommateur qui
/// veut le contenu du fichier le lit lui-même via `path`.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DecisionV1 {
    pub id: String,
    pub qualified_id: String,
    pub title: String,
    pub status: String,
    pub date: String,
    pub tags: Vec<String>,
    pub supersedes: Vec<String>,
    /// Champ additif (K6) — identifiants qualifiés dont cet ADR local
    /// s'écarte volontairement. Vide pour les ADR antérieurs.
    pub deviates_from: Vec<String>,
    pub path: String,
    pub origin: String,
    /// Vrai si la décision est en vigueur.
    pub in_effect: bool,
    /// Identifiant qualifié de la décision qui la supersede, s'il y en a.
    pub superseded_by: Option<String>,
    /// Champ additif (K6) — pour une décision héritée qu'un ADR local
    /// écarte via `deviates_from`, l'identifiant qualifié de l'ADR
    /// local qui la remplace. Absent si aucune dérive locale.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deviated_by: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DecisionListReportV1 {
    pub root: String,
    pub decisions: Vec<DecisionV1>,
    pub status: Vec<StatusEntry>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DecisionShowReportV1 {
    pub root: String,
    pub decision: Option<DecisionV1>,
    /// Contenu markdown complet du fichier — inclus pour éviter à un
    /// consommateur de re-lire le disque après un `show`.
    pub content: Option<String>,
    pub status: Vec<StatusEntry>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DecisionCreatedV1 {
    pub root: String,
    pub decision: Option<DecisionV1>,
    pub path: Option<String>,
    /// Hash du corps de l'ADR créé, préfixé `sha256:`.
    ///
    /// Champ additif — présent dès qu'un ADR est effectivement scellé
    /// (statut `accepted`/`superseded`), absent sinon. Un consommateur du
    /// contrat antérieur au sceau ignorera ce champ sans souci.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_sha256: Option<String>,
    pub status: Vec<StatusEntry>,
}

// ─────────────────────────── sources ───────────────────────────

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceStateV1 {
    #[serde(rename = "type")]
    pub kind: &'static str,
    pub address: String,
    pub state: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subpath: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_path: Option<String>,
}

impl SourceStateV1 {
    pub fn from(status: &codev_engine::sources::SourceStatus) -> Self {
        Self {
            kind: status.kind.as_str(),
            address: status.address.clone(),
            state: status.state.as_str(),
            git_ref: status.git_ref.clone(),
            subpath: status.subpath.clone(),
            sha: status.sha.clone(),
            resolved_path: status.resolved_path.as_ref().map(|p| to_slash(p)),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SourcesListReportV1 {
    pub root: String,
    pub sources: Vec<SourceStateV1>,
    pub status: Vec<StatusEntry>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PinChangeV1 {
    pub kind: &'static str,
    pub url: String,
    pub git_ref: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    pub to: String,
}

impl PinChangeV1 {
    pub fn from(pc: &codev_engine::sources::PinChange) -> Self {
        use codev_engine::sources::PinChange::*;
        match pc {
            Added { url, git_ref, to } => Self {
                kind: "added",
                url: url.clone(),
                git_ref: git_ref.clone(),
                from: None,
                to: to.clone(),
            },
            Moved { url, git_ref, from, to } => Self {
                kind: "moved",
                url: url.clone(),
                git_ref: git_ref.clone(),
                from: Some(from.clone()),
                to: to.clone(),
            },
            Unchanged { url, git_ref, sha } => Self {
                kind: "unchanged",
                url: url.clone(),
                git_ref: git_ref.clone(),
                from: None,
                to: sha.clone(),
            },
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SourcesUpdateReportV1 {
    pub root: String,
    pub changes: Vec<PinChangeV1>,
    pub lock_written: bool,
    pub status: Vec<StatusEntry>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceDetailV1 {
    pub source: SourceStateV1,
    pub files_exposed: Vec<String>,
    pub root: String,
    pub status: Vec<StatusEntry>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DecisionSupersededV1 {
    pub root: String,
    pub new_decision: Option<DecisionV1>,
    pub new_path: Option<String>,
    pub old_id: Option<String>,
    pub old_qualified_id: Option<String>,
    pub old_path: Option<String>,
    pub status: Vec<StatusEntry>,
}

/// Une entrée de sceau exposée par le contrat JSON.
///
/// Les noms sérialisés sont camelCase — même règle que le reste du
/// contrat public.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SealEntryV1 {
    pub id: String,
    pub body_sha256: String,
    pub sealed_at: String,
}

/// Le retour de `codev decision seal --json`.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DecisionSealedV1 {
    pub root: String,
    pub seal: Option<SealEntryV1>,
    /// `true` si l'appel n'a rien écrit — le sceau était déjà à jour.
    pub was_noop: bool,
    pub status: Vec<StatusEntry>,
}

/// Le retour de `codev decision deviate --json`.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DecisionDeviatedV1 {
    pub root: String,
    pub decision: Option<DecisionV1>,
    pub path: Option<String>,
    /// L'identifiant qualifié de la décision héritée qu'on écarte.
    pub target_qualified_id: Option<String>,
    /// Hash du corps du nouvel ADR local — scellé par K3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_sha256: Option<String>,
    pub status: Vec<StatusEntry>,
}

/// Le retour de `codev decision promote --json`.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DecisionPromotedV1 {
    pub root: String,
    pub decision: Option<DecisionV1>,
    pub path: Option<String>,
    /// Hash du corps du nouvel ADR — scellé par K3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_sha256: Option<String>,
    /// Le change d'où la promotion vient.
    pub source_change: Option<String>,
    /// Le `design.md` qui a été mis à jour.
    pub design_path: Option<String>,
    pub status: Vec<StatusEntry>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InstructionsV1 {
    pub change_name: String,
    pub schema_name: String,
    pub artifact: String,
    pub description: Option<String>,
    pub resolved_output_path: String,
    pub instruction: Option<String>,
    pub template: Option<String>,
    /// Du plus général au plus spécifique. Contraintes pour l'agent, jamais du
    /// contenu à recopier dans le fichier produit.
    pub context: Vec<BlockV1>,
    pub rules: Vec<BlockV1>,
    pub dependencies: Vec<DependencyV1>,
    pub unlocks: Vec<String>,
    /// Décisions d'architecture **en vigueur** — rempli pour l'artefact
    /// `design`, présent mais vide pour les autres. Le consommateur teste
    /// `decisions.length` sans branche conditionnelle.
    pub decisions: Vec<DecisionRefV1>,
    pub skipped: bool,
    pub status: Vec<StatusEntry>,
}

impl From<&Instructions> for InstructionsV1 {
    fn from(instructions: &Instructions) -> Self {
        Self {
            change_name: instructions.change.to_string(),
            schema_name: instructions.schema_name.clone(),
            artifact: instructions.artifact_id.clone(),
            description: instructions.description.clone(),
            resolved_output_path: instructions.resolved_output_path.display().to_string(),
            instruction: instructions.instruction.clone(),
            template: instructions.template.clone(),
            context: instructions.context.iter().map(BlockV1::from).collect(),
            rules: instructions.rules.iter().map(BlockV1::from).collect(),
            dependencies: instructions
                .dependencies
                .iter()
                .map(|d| DependencyV1 {
                    id: d.id.clone(),
                    path: d.path.display().to_string(),
                    done: d.done,
                })
                .collect(),
            unlocks: instructions.unlocks.clone(),
            decisions: instructions
                .decisions
                .iter()
                .map(|d| DecisionRefV1 {
                    id: d.id.clone(),
                    qualified_id: d.qualified_id.clone(),
                    title: d.title.clone(),
                    status: d.status.clone(),
                    tags: d.tags.clone(),
                    path: to_slash(&d.path),
                    origin: d.origin.clone(),
                })
                .collect(),
            skipped: instructions.skipped,
            status: statuses(&instructions.warnings),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ChangesV1 {
    pub changes: Vec<String>,
    pub root: Option<String>,
    pub status: Vec<StatusEntry>,
}

#[derive(Debug, Serialize)]
pub struct SpecsV1 {
    pub specs: Vec<String>,
    pub root: Option<String>,
    pub status: Vec<StatusEntry>,
}

#[derive(Debug, Serialize)]
pub struct SchemaV1 {
    pub name: String,
    pub origin: &'static str,
    pub flow: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct SchemasV1 {
    pub schemas: Vec<SchemaV1>,
    pub root: Option<String>,
    pub status: Vec<StatusEntry>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetupV1 {
    pub root: String,
    pub created: Vec<String>,
    pub updated: Vec<String>,
    pub untouched: Vec<String>,
    /// Les skills laissées en place parce qu'éditées à la main.
    pub preserved: Vec<String>,
    pub skills: Vec<String>,
    pub status: Vec<StatusEntry>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FindingV1 {
    /// Chemin relatif au projet, en séparateurs `/` — jamais absolu.
    pub path: String,
    pub line: u32,
    pub severity: &'static str,
    pub code: &'static str,
    pub message: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemReportV1 {
    /// `"change"` ou `"spec"` — les seuls types actuels.
    pub kind: &'static str,
    pub name: String,
    pub path: String,
    pub findings: Vec<FindingV1>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidateReportV1 {
    pub root: String,
    pub items: Vec<ItemReportV1>,
    /// `true` dès qu'un finding `Warning` est présent dans le rapport.
    /// Champ informatif — le vrai signal de sortie reste l'exit code
    /// (voir `--strict` sur la CLI). Toujours présent.
    pub has_warnings: bool,
    pub status: Vec<StatusEntry>,
}

impl ValidateReportV1 {
    pub fn from(report: &codev_engine::validate::ValidateReport) -> Self {
        use codev_core::parser::ast::Severity;
        use codev_engine::validate::ItemKind;

        Self {
            root: to_slash(&report.root),
            has_warnings: report.has_warnings(),
            items: report
                .items
                .iter()
                .map(|item| ItemReportV1 {
                    kind: match item.kind {
                        ItemKind::Change => "change",
                        ItemKind::Spec => "spec",
                        ItemKind::Decisions => "decisions",
                    },
                    name: item.name.clone(),
                    path: to_slash(&item.path),
                    findings: item
                        .findings
                        .iter()
                        .map(|f| FindingV1 {
                            path: to_slash(&f.path),
                            line: f.finding.line,
                            severity: match f.finding.severity {
                                Severity::Error => "error",
                                Severity::Warning => "warning",
                                Severity::Info => "info",
                            },
                            code: f.finding.code,
                            message: f.finding.message.clone(),
                        })
                        .collect(),
                })
                .collect(),
            status: Vec::new(),
        }
    }
}

/// Écrit un `Path` avec des séparateurs `/`, quel que soit le système.
///
/// Le contrat JSON est consommé par des skills qui n'ont pas à connaître
/// l'OS de l'utilisateur ; imposer un séparateur unique évite les branches
/// conditionnelles côté agent.
///
/// Détail piégeux : sur Unix, `path.components()` d'un chemin absolu produit
/// un premier `Component::RootDir` dont `as_os_str()` vaut déjà `"/"`. Un
/// `join("/")` naïf ajoute alors un séparateur en tête et produit
/// `"//Users/…"` — bug observable dans les rapports JSON avant ce correctif.
/// On passe donc par `to_string_lossy` sur le chemin entier et on remplace
/// uniquement le séparateur Windows, qui reste le seul cas où le chemin
/// natif diffère.
fn to_slash(path: &std::path::Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncReportV1 {
    pub change_name: String,
    pub root: String,
    pub updated: Vec<String>,
    pub created: Vec<String>,
    pub unchanged: Vec<String>,
    /// Main specs supprimées par le change (F5). Champ additif, toujours
    /// présent, vide dans le cas courant.
    pub deleted: Vec<String>,
    pub status: Vec<StatusEntry>,
}

impl SyncReportV1 {
    pub fn from(outcome: &codev_engine::sync::SyncOutcome) -> Self {
        Self {
            change_name: outcome.change.clone(),
            root: to_slash(&outcome.root),
            updated: paths_slash(&outcome.updated),
            created: paths_slash(&outcome.created),
            unchanged: paths_slash(&outcome.unchanged),
            deleted: paths_slash(&outcome.deleted),
            status: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveReportV1 {
    pub change_name: String,
    pub root: String,
    pub updated: Vec<String>,
    pub created: Vec<String>,
    pub unchanged: Vec<String>,
    /// Main specs supprimées par le change (F5). Champ additif, toujours
    /// présent, vide dans le cas courant.
    pub deleted: Vec<String>,
    pub moved_to: String,
    pub status: Vec<StatusEntry>,
}

impl ArchiveReportV1 {
    pub fn from(outcome: &codev_engine::archive::ArchiveOutcome) -> Self {
        Self {
            change_name: outcome.change.clone(),
            root: to_slash(&outcome.root),
            updated: paths_slash(&outcome.updated),
            created: paths_slash(&outcome.created),
            unchanged: paths_slash(&outcome.unchanged),
            deleted: paths_slash(&outcome.deleted),
            moved_to: to_slash(&outcome.moved_to),
            status: Vec::new(),
        }
    }
}

fn paths_slash(paths: &[std::path::PathBuf]) -> Vec<String> {
    paths.iter().map(|p| to_slash(p)).collect()
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewChangeV1 {
    pub change_name: String,
    pub schema_name: String,
    pub change_root: String,
    pub created: Vec<String>,
    pub status: Vec<StatusEntry>,
}

/// La forme nulle d'une commande, complétée par l'erreur qui l'a provoquée.
///
/// Un consommateur doit pouvoir désérialiser la réponse d'un échec avec le même
/// code que celle d'un succès : c'est pourquoi l'échec garde la forme de la
/// commande, avec ses champs vides, plutôt qu'un objet d'erreur d'une autre
/// forme.
pub fn failure(mut shape: serde_json::Value, code: &str, message: &str) -> serde_json::Value {
    if let Some(object) = shape.as_object_mut() {
        let entry = StatusEntry::error(code, message);
        object.insert(
            "status".into(),
            json!([serde_json::to_value(entry).expect("une entrée de status est sérialisable")]),
        );
    }
    shape
}

#[cfg(test)]
mod tests {
    use super::*;
    use codev_core::parser::ast::{Finding, Severity};
    use codev_core::{status, ArtifactGraph, ChangeId};
    use codev_engine::validate::{ItemKind, ItemReport, LocatedFinding, ValidateReport};
    use std::collections::BTreeSet;
    use std::path::PathBuf;

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

    fn statut_json() -> serde_json::Value {
        let graph = ArtifactGraph::from_yaml(SPEC_DRIVEN).unwrap();
        let change = ChangeId::parse("add-auth").unwrap();
        let existing: BTreeSet<String> = ["proposal".to_string()].into_iter().collect();
        let status = status::compute(&graph, &change, &existing, &BTreeSet::new());
        let v1 = StatusV1::new(
            &status,
            "/p".into(),
            "/p/_codev/changes/add-auth".into(),
            &[Warning::new("inherit_unresolved", "source absente")],
        );
        serde_json::to_value(v1).unwrap()
    }

    #[test]
    fn le_statut_expose_les_champs_du_contrat_en_camel_case() {
        let json = statut_json();
        for champ in [
            "changeName",
            "schemaName",
            "planningHome",
            "changeRoot",
            "applyRequires",
            "isPlanningComplete",
            "artifacts",
            "status",
        ] {
            assert!(json.get(champ).is_some(), "champ « {champ} » manquant");
        }
    }

    #[test]
    fn les_etats_dartefact_sont_des_libelles_stables() {
        let json = statut_json();
        let artifacts = json["artifacts"].as_array().unwrap();
        assert_eq!(artifacts[0]["status"], "done");
        assert_eq!(artifacts[1]["status"], "ready");
        assert_eq!(artifacts[0]["outputPath"], "proposal.md");
        assert_eq!(artifacts[1]["outputPath"], "specs/**/*.md");
    }

    #[test]
    fn to_slash_ne_double_pas_le_separateur_sur_chemin_absolu() {
        // Régression : `path.components().join("/")` produisait `//Users/x`
        // pour un chemin absolu Unix, parce que `Component::RootDir` vaut
        // déjà `"/"` et se retrouvait recollé par le `join("/")`. Le contrat
        // JSON parlait alors à ses consommateurs de chemins qui n'existaient
        // pas.
        assert_eq!(to_slash(std::path::Path::new("/Users/x/y")), "/Users/x/y");
        assert_eq!(to_slash(std::path::Path::new("/")), "/");
        assert_eq!(to_slash(std::path::Path::new("relative/path")), "relative/path");
    }

    #[test]
    fn to_slash_traduit_le_separateur_windows() {
        // Simule le résultat naturel d'un `path.to_string_lossy()` sur
        // Windows sans dépendre du système sous-jacent.
        assert_eq!(
            to_slash(std::path::Path::new("relative\\sub\\file.md")),
            "relative/sub/file.md"
        );
    }

    #[test]
    fn les_avertissements_remontent_dans_le_tableau_status() {
        let json = statut_json();
        let status = json["status"].as_array().unwrap();
        assert_eq!(status.len(), 1);
        assert_eq!(status[0]["level"], "warning");
        assert_eq!(status[0]["code"], "inherit_unresolved");
    }

    #[test]
    fn validate_report_expose_les_champs_attendus_en_camel_case() {
        let finding = Finding {
            severity: Severity::Error,
            code: "requirement_no_shall",
            line: 12,
            message: "manque SHALL".into(),
        };
        let item = ItemReport {
            kind: ItemKind::Change,
            name: "add-auth".into(),
            path: PathBuf::from("_codev/changes/add-auth"),
            findings: vec![LocatedFinding::from_finding(
                finding,
                PathBuf::from("_codev/changes/add-auth/specs/user-auth/spec.md"),
            )],
        };
        let report = ValidateReport {
            root: PathBuf::from("/p"),
            items: vec![item],
        };
        let v1 = ValidateReportV1::from(&report);
        let json = serde_json::to_value(&v1).unwrap();

        // Racine du contrat.
        for champ in ["root", "items", "status"] {
            assert!(json.get(champ).is_some(), "champ « {champ} » manquant");
        }
        // Item.
        let item_json = &json["items"][0];
        for champ in ["kind", "name", "path", "findings"] {
            assert!(
                item_json.get(champ).is_some(),
                "item : champ « {champ} » manquant"
            );
        }
        assert_eq!(item_json["kind"], "change");
        assert_eq!(item_json["path"], "_codev/changes/add-auth");
        // Finding.
        let finding_json = &item_json["findings"][0];
        assert_eq!(finding_json["severity"], "error");
        assert_eq!(finding_json["code"], "requirement_no_shall");
        assert_eq!(finding_json["line"], 12);
        assert_eq!(
            finding_json["path"],
            "_codev/changes/add-auth/specs/user-auth/spec.md"
        );
        assert_eq!(finding_json["message"], "manque SHALL");
    }

    #[test]
    fn validate_report_echec_garde_la_forme() {
        let shape = json!({ "root": null, "items": [] });
        let json = failure(shape, "no_codev_root", "aucun projet codev trouvé");
        assert_eq!(json["items"], json!([]));
        assert_eq!(json["root"], serde_json::Value::Null);
        assert_eq!(json["status"][0]["code"], "no_codev_root");
    }

    #[test]
    fn sync_report_shape_stable() {
        use codev_engine::sync::SyncOutcome;
        let outcome = SyncOutcome {
            change: "add-auth".into(),
            root: PathBuf::from("/p"),
            updated: vec![PathBuf::from("_codev/specs/a/spec.md")],
            created: vec![PathBuf::from("_codev/specs/b/spec.md")],
            unchanged: vec![],
            deleted: vec![],
        };
        let json = serde_json::to_value(SyncReportV1::from(&outcome)).unwrap();
        for champ in ["changeName", "root", "updated", "created", "unchanged", "status"] {
            assert!(json.get(champ).is_some(), "sync : « {champ} » manquant");
        }
        assert_eq!(json["changeName"], "add-auth");
        assert_eq!(json["updated"][0], "_codev/specs/a/spec.md");
    }

    #[test]
    fn archive_report_shape_stable() {
        use codev_engine::archive::ArchiveOutcome;
        let outcome = ArchiveOutcome {
            change: "add-auth".into(),
            root: PathBuf::from("/p"),
            updated: vec![],
            created: vec![PathBuf::from("_codev/specs/user-auth/spec.md")],
            unchanged: vec![],
            deleted: vec![],
            moved_to: PathBuf::from("_codev/changes/archive/2026-09-08-add-auth"),
        };
        let json = serde_json::to_value(ArchiveReportV1::from(&outcome)).unwrap();
        for champ in [
            "changeName",
            "root",
            "updated",
            "created",
            "unchanged",
            "movedTo",
            "status",
        ] {
            assert!(json.get(champ).is_some(), "archive : « {champ} » manquant");
        }
        assert_eq!(
            json["movedTo"],
            "_codev/changes/archive/2026-09-08-add-auth"
        );
    }

    #[test]
    fn un_echec_garde_la_forme_de_la_commande() {
        let shape = json!({ "changes": [], "root": null });
        let json = failure(shape, "no_codev_root", "aucun projet codev trouvé");

        assert_eq!(json["changes"], json!([]));
        assert_eq!(json["root"], serde_json::Value::Null);
        assert_eq!(json["status"][0]["level"], "error");
        assert_eq!(json["status"][0]["code"], "no_codev_root");
    }
}
