//! The JSON contract, version 1.
//!
//! **This is a public API.** It is consumed by skills already installed on
//! users' machines, which are not regenerated when we recompile. Hence the
//! rule: these types do not derive from the domain model, they translate it.
//! An internal refactor can therefore break a conversion, which the compiler
//! reports, rather than the shape of the JSON, which nobody would notice
//! before the user.
//!
//! Two invariants held by the whole module:
//! - stdout carries **exactly one** JSON document, including on failure;
//! - every document carries a `status` array, possibly empty.

use codev_core::{ArtifactState, ChangeStatus};
use codev_engine::Warning;
use codev_engine::config::Block;
use codev_engine::instructions::Instructions;
use serde::Serialize;
use serde_json::json;

/// An entry of the `status` array: a finding, not prose.
///
/// `code` is stable and meant to be tested by a consumer; `message` is meant
/// to be read, and may be reworded without notice.
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
    /// The project root, resolved. Skills must use it rather than assume a
    /// path relative to the current folder.
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

/// The complete shape of a decision — used by `list` (each entry of the
/// array) and by `show` (at the root level). A consumer who wants the file's
/// content reads it themselves via `path`.
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
    /// Additive field (local deviation) — qualified identifiers this local
    /// ADR deliberately departs from. Empty for older ADRs.
    pub deviates_from: Vec<String>,
    pub path: String,
    pub origin: String,
    /// True if the decision is in effect.
    pub in_effect: bool,
    /// Qualified identifier of the decision that supersedes it, if any.
    pub superseded_by: Option<String>,
    /// Additive field (local deviation) — for an inherited decision that a
    /// local ADR sets aside via `deviates_from`, the qualified identifier of
    /// the local ADR that replaces it. Absent if there is no local
    /// deviation.
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
    /// Full markdown content of the file — included so that a consumer does
    /// not have to read the disk again after a `show`.
    pub content: Option<String>,
    pub status: Vec<StatusEntry>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DecisionCreatedV1 {
    pub root: String,
    pub decision: Option<DecisionV1>,
    pub path: Option<String>,
    /// Hash of the created ADR's body, prefixed with `sha256:`.
    ///
    /// Additive field — present as soon as an ADR is actually sealed
    /// (`accepted`/`superseded` status), absent otherwise. A consumer of the
    /// contract predating the seal will safely ignore this field.
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
            Moved {
                url,
                git_ref,
                from,
                to,
            } => Self {
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

/// A seal entry exposed by the JSON contract.
///
/// Serialized names are camelCase — the same rule as the rest of the public
/// contract.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SealEntryV1 {
    pub id: String,
    pub body_sha256: String,
    pub sealed_at: String,
}

/// The output of `codev decision seal --json`.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DecisionSealedV1 {
    pub root: String,
    pub seal: Option<SealEntryV1>,
    /// `true` if the call wrote nothing — the seal was already up to date.
    pub was_noop: bool,
    pub status: Vec<StatusEntry>,
}

/// The output of `codev decision deviate --json`.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DecisionDeviatedV1 {
    pub root: String,
    pub decision: Option<DecisionV1>,
    pub path: Option<String>,
    /// The qualified identifier of the inherited decision being set aside.
    pub target_qualified_id: Option<String>,
    /// Hash of the new local ADR's body — the ADR is sealed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_sha256: Option<String>,
    pub status: Vec<StatusEntry>,
}

/// The output of `codev decision promote --json`.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DecisionPromotedV1 {
    pub root: String,
    pub decision: Option<DecisionV1>,
    pub path: Option<String>,
    /// Hash of the new ADR's body — the ADR is sealed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_sha256: Option<String>,
    /// The change the promotion comes from.
    pub source_change: Option<String>,
    /// The `design.md` that was updated.
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
    /// From most general to most specific. Constraints for the agent, never
    /// content to copy into the produced file.
    pub context: Vec<BlockV1>,
    pub rules: Vec<BlockV1>,
    pub dependencies: Vec<DependencyV1>,
    pub unlocks: Vec<String>,
    /// Architecture decisions **in effect** — filled for the `design`
    /// artifact, present but empty for the others. The consumer tests
    /// `decisions.length` without a conditional branch.
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
    /// The skills left in place because they were edited by hand.
    pub preserved: Vec<String>,
    pub skills: Vec<String>,
    pub status: Vec<StatusEntry>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FindingV1 {
    /// Path relative to the project, with `/` separators — never absolute.
    pub path: String,
    pub line: u32,
    pub severity: &'static str,
    pub code: &'static str,
    pub message: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemReportV1 {
    /// `"change"`, `"spec"` or `"decisions"` — the only current kinds.
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
    /// `true` as soon as a `Warning` finding is present in the report.
    /// Informative field — the real output signal remains the exit code
    /// (see `--strict` on the CLI). Always present.
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

/// Writes a `Path` with `/` separators, whatever the system.
///
/// The JSON contract is consumed by skills that do not need to know the
/// user's OS; enforcing a single separator avoids conditional branches on
/// the agent's side.
///
/// Tricky detail: on Unix, `path.components()` of an absolute path yields a
/// first `Component::RootDir` whose `as_os_str()` is already `"/"`. A naive
/// `join("/")` then adds a leading separator and produces `"//Users/…"` — a
/// bug observable in JSON reports before this fix. So we go through
/// `to_string_lossy` on the whole path and only replace the Windows
/// separator, which remains the only case where the native path differs.
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
    /// Main specs deleted by the change (capability removal). Additive
    /// field, always present, empty in the common case.
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
    /// Main specs deleted by the change (capability removal). Additive
    /// field, always present, empty in the common case.
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

/// The null shape of a command, completed with the error that caused it.
///
/// A consumer must be able to deserialize a failure response with the same
/// code as a success response: that is why a failure keeps the command's
/// shape, with its fields empty, rather than an error object of a different
/// shape.
pub fn failure(mut shape: serde_json::Value, code: &str, message: &str) -> serde_json::Value {
    if let Some(object) = shape.as_object_mut() {
        let entry = StatusEntry::error(code, message);
        object.insert(
            "status".into(),
            json!([serde_json::to_value(entry).expect("a status entry is serializable")]),
        );
    }
    shape
}

#[cfg(test)]
mod tests {
    use super::*;
    use codev_core::parser::ast::{Finding, Severity};
    use codev_core::{ArtifactGraph, ChangeId, status};
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

    fn status_json() -> serde_json::Value {
        let graph = ArtifactGraph::from_yaml(SPEC_DRIVEN).unwrap();
        let change = ChangeId::parse("add-auth").unwrap();
        let existing: BTreeSet<String> = ["proposal".to_string()].into_iter().collect();
        let status = status::compute(&graph, &change, &existing, &BTreeSet::new());
        let v1 = StatusV1::new(
            &status,
            "/p".into(),
            "/p/_codev/changes/add-auth".into(),
            &[Warning::new("inherit_unresolved", "missing source")],
        );
        serde_json::to_value(v1).unwrap()
    }

    #[test]
    fn status_exposes_the_contract_fields_in_camel_case() {
        let json = status_json();
        for field in [
            "changeName",
            "schemaName",
            "planningHome",
            "changeRoot",
            "applyRequires",
            "isPlanningComplete",
            "artifacts",
            "status",
        ] {
            assert!(json.get(field).is_some(), "missing field `{field}`");
        }
    }

    #[test]
    fn artifact_states_are_stable_labels() {
        let json = status_json();
        let artifacts = json["artifacts"].as_array().unwrap();
        assert_eq!(artifacts[0]["status"], "done");
        assert_eq!(artifacts[1]["status"], "ready");
        assert_eq!(artifacts[0]["outputPath"], "proposal.md");
        assert_eq!(artifacts[1]["outputPath"], "specs/**/*.md");
    }

    #[test]
    fn to_slash_does_not_double_the_separator_on_an_absolute_path() {
        // Regression: `path.components().join("/")` produced `//Users/x` for
        // an absolute Unix path, because `Component::RootDir` is already
        // `"/"` and got glued back by the `join("/")`. The JSON contract
        // then told its consumers about paths that did not exist.
        assert_eq!(to_slash(std::path::Path::new("/Users/x/y")), "/Users/x/y");
        assert_eq!(to_slash(std::path::Path::new("/")), "/");
        assert_eq!(
            to_slash(std::path::Path::new("relative/path")),
            "relative/path"
        );
    }

    #[test]
    fn to_slash_translates_the_windows_separator() {
        // Simulates the natural result of a `path.to_string_lossy()` on
        // Windows without depending on the underlying system.
        assert_eq!(
            to_slash(std::path::Path::new("relative\\sub\\file.md")),
            "relative/sub/file.md"
        );
    }

    #[test]
    fn warnings_surface_in_the_status_array() {
        let json = status_json();
        let status = json["status"].as_array().unwrap();
        assert_eq!(status.len(), 1);
        assert_eq!(status[0]["level"], "warning");
        assert_eq!(status[0]["code"], "inherit_unresolved");
    }

    #[test]
    fn validate_report_exposes_the_expected_fields_in_camel_case() {
        let finding = Finding {
            severity: Severity::Error,
            code: "requirement_no_shall",
            line: 12,
            message: "missing SHALL".into(),
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

        // Contract root.
        for field in ["root", "items", "status"] {
            assert!(json.get(field).is_some(), "missing field `{field}`");
        }
        // Item.
        let item_json = &json["items"][0];
        for field in ["kind", "name", "path", "findings"] {
            assert!(
                item_json.get(field).is_some(),
                "item: missing field `{field}`"
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
        assert_eq!(finding_json["message"], "missing SHALL");
    }

    #[test]
    fn validate_report_failure_keeps_the_shape() {
        let shape = json!({ "root": null, "items": [] });
        let json = failure(shape, "no_codev_root", "no codev project found");
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
        for field in [
            "changeName",
            "root",
            "updated",
            "created",
            "unchanged",
            "status",
        ] {
            assert!(json.get(field).is_some(), "sync: missing `{field}`");
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
        for field in [
            "changeName",
            "root",
            "updated",
            "created",
            "unchanged",
            "movedTo",
            "status",
        ] {
            assert!(json.get(field).is_some(), "archive: missing `{field}`");
        }
        assert_eq!(
            json["movedTo"],
            "_codev/changes/archive/2026-09-08-add-auth"
        );
    }

    #[test]
    fn a_failure_keeps_the_shape_of_the_command() {
        let shape = json!({ "changes": [], "root": null });
        let json = failure(shape, "no_codev_root", "no codev project found");

        assert_eq!(json["changes"], json!([]));
        assert_eq!(json["root"], serde_json::Value::Null);
        assert_eq!(json["status"][0]["level"], "error");
        assert_eq!(json["status"][0]["code"], "no_codev_root");
    }
}
