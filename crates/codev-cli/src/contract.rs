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

use codev_core::roles::{ResolvedRole, RoleKind};
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
    /// The update check; absent when the latest release is unknown, in CI,
    /// or with `CODEV_NO_UPDATE_CHECK=1`. Additive: skills generated before
    /// it ignore it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update: Option<UpdateV1>,
    /// The project's `codev_version`; absent when it sets none.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_requirement: Option<VersionRequirementV1>,
    pub status: Vec<StatusEntry>,
}

/// `update` in `codev status --json`.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateV1 {
    pub current: String,
    pub latest: String,
    pub url: String,
    pub available: bool,
    pub snoozed: bool,
}

impl From<&codev_core::update::UpdateNotice> for UpdateV1 {
    fn from(notice: &codev_core::update::UpdateNotice) -> Self {
        Self {
            current: notice.current.clone(),
            latest: notice.latest.clone(),
            url: notice.url.clone(),
            available: notice.available,
            snoozed: notice.snoozed,
        }
    }
}

/// `versionRequirement` in `codev status --json`.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionRequirementV1 {
    pub requirement: String,
    pub satisfied: bool,
}

/// `codev self-update --json`.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SelfUpdateV1 {
    pub current: String,
    pub latest: Option<String>,
    pub url: Option<String>,
    /// `updated`, `up_to_date`, `available`, `snoozed`,
    /// `nothing_to_snooze`, `managed_elsewhere` (`cancelled` never appears:
    /// `--json` implies `--yes`).
    pub action: &'static str,
    /// `standalone`, `cargo`, `build` or `package_manager`.
    pub install_method: &'static str,
    pub path: String,
    /// The upgrade command for another installation method.
    pub hint: Option<String>,
    pub project_updated: bool,
    pub status: Vec<StatusEntry>,
}

impl From<&codev_engine::update::Outcome> for SelfUpdateV1 {
    fn from(outcome: &codev_engine::update::Outcome) -> Self {
        Self {
            current: outcome.current.clone(),
            latest: outcome.latest.clone(),
            url: outcome.url.clone(),
            action: outcome.action.as_str(),
            install_method: outcome.method.as_str(),
            path: json_path(&outcome.path),
            hint: outcome.method.upgrade_hint(),
            project_updated: outcome.project_updated.is_some(),
            status: statuses(&outcome.warnings),
        }
    }
}

/// The shape of `codev self-update --json` on failure.
pub fn self_update_shape(current: &str) -> serde_json::Value {
    json!({
        "current": current,
        "latest": null,
        "url": null,
        "action": null,
        "installMethod": null,
        "path": null,
        "hint": null,
        "projectUpdated": false,
    })
}

impl StatusV1 {
    pub fn new(
        status: &ChangeStatus,
        planning_home: &std::path::Path,
        change_root: &std::path::Path,
        warnings: &[Warning],
    ) -> Self {
        Self {
            change_name: status.change.to_string(),
            schema_name: status.schema_name.clone(),
            planning_home: json_path(planning_home),
            change_root: json_path(change_root),
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
            update: None,
            version_requirement: None,
            status: statuses(warnings),
        }
    }

    pub fn with_update(
        mut self,
        update: Option<&codev_core::update::UpdateNotice>,
        requirement: Option<&(String, bool)>,
    ) -> Self {
        self.update = update.map(UpdateV1::from);
        self.version_requirement =
            requirement.map(|(requirement, satisfied)| VersionRequirementV1 {
                requirement: requirement.clone(),
                satisfied: *satisfied,
            });
        self
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
            resolved_path: status.resolved_path.as_ref().map(|p| json_path(p)),
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
    // No `bodySha256`: the deviation is created `proposed`, hence unsealed;
    // `decision accept --json` carries the hash once it is sealed.
    pub status: Vec<StatusEntry>,
}

/// The output of `codev decision accept --json`.
///
/// Same fields as `DecisionCreatedV1`: the accepted decision, its file and
/// the hash now recorded in `seal.yaml` — plus the predecessors the
/// acceptance marked `superseded`.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DecisionAcceptedV1 {
    pub root: String,
    pub decision: Option<DecisionV1>,
    pub path: Option<String>,
    /// Hash of the accepted ADR's body, prefixed with `sha256:`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_sha256: Option<String>,
    /// Additive field — the decisions listed in `supersedes` that this
    /// acceptance rewrote to `status: superseded`. Always present, empty
    /// when the decision supersedes nothing.
    pub superseded: Vec<SupersededDecisionV1>,
    pub status: Vec<StatusEntry>,
}

/// A predecessor marked `superseded` by `decision accept`.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SupersededDecisionV1 {
    pub id: String,
    pub qualified_id: String,
    pub path: String,
}

/// The output of `codev decision promote --json`.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DecisionPromotedV1 {
    pub root: String,
    pub decision: Option<DecisionV1>,
    pub path: Option<String>,
    // No `bodySha256`: the promoted ADR is created `proposed`, hence
    // unsealed; `decision accept --json` carries the hash once it is sealed.
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
    /// Language of the prose to write (ISO 639 code, from `language:` in
    /// the config, `en` by default). Structural keywords of the template
    /// stay in English whatever the language.
    pub language: String,
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
    /// The role that owns the artifact; `null` when the schema declares
    /// none. Additive in v1.
    pub role: Option<RoleV1>,
    pub status: Vec<StatusEntry>,
}

/// A role, as `codev roles` and the `role` field of `codev instructions`
/// serve it.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleV1 {
    pub id: String,
    /// The display name in the configured language, English otherwise.
    pub name: String,
    /// `step` or `artifact`.
    pub kind: &'static str,
    /// The step asked with `--at`, otherwise the role's first step, such as
    /// `plan.challenge`.
    pub at: String,
    /// Every step the role serves, in workflow order. Additive in v1.
    pub steps: Vec<String>,
    /// The artifact an artifact role owns; `null` for a step role.
    pub artifact: Option<String>,
    /// The domains that activate an `on_domain` role; empty for a role
    /// bound to none. Additive in v1.
    pub domains: Vec<String>,
    pub question: String,
    /// `always`, `never` or `on_domain`.
    pub when: &'static str,
    /// `default`, `project` or `change`.
    pub when_origin: &'static str,
    /// Why the role is active: `always`, `project`, `change` or
    /// `domain:<name>`; `null` when it is not. Additive in v1.
    pub activated_by: Option<String>,
    pub disableable: bool,
    /// The role's knowledge, as markdown, for an active role that has one;
    /// `null` otherwise. Additive in v1.
    pub knowledge: Option<String>,
    /// The role's criteria, then the project's, each with its origin:
    /// `built-in`, `schema:<name>` or `project`.
    pub done_when: Vec<BlockV1>,
}

impl RoleV1 {
    /// `at` is the step the role is served at, when one was asked.
    pub fn from(resolved: &ResolvedRole, language: &str, at: Option<&str>) -> Self {
        let role = &resolved.role;
        let (kind, artifact) = match &role.kind {
            RoleKind::Step => ("step", None),
            RoleKind::Artifact(id) => ("artifact", Some(id.clone())),
        };
        Self {
            id: role.id.clone(),
            name: role.name(language).to_string(),
            kind,
            at: at.unwrap_or(role.first_step()).to_string(),
            steps: role.steps.clone(),
            artifact,
            domains: role.domains.clone(),
            question: role.question.clone(),
            when: resolved.when.as_str(),
            when_origin: resolved.when_origin.as_str(),
            activated_by: resolved.activated_by.as_ref().map(|by| by.label()),
            disableable: role.disableable,
            knowledge: if resolved.is_active() {
                role.knowledge.clone()
            } else {
                None
            },
            done_when: role
                .criteria
                .iter()
                .map(|c| BlockV1 {
                    origin: c.origin.clone(),
                    text: c.text.clone(),
                })
                .collect(),
        }
    }
}

/// `codev roles --json`.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RolesV1 {
    /// `null` without `--change`.
    pub change_name: Option<String>,
    pub schema_name: Option<String>,
    /// The step asked with `--at`; `null` lists every role.
    pub at: Option<String>,
    pub language: String,
    /// The languages the knowledge is filtered for. Additive in v1;
    /// `null` only in a failure's document.
    pub stacks: Option<StacksV1>,
    /// With `--change`, the change's diff; `null` otherwise. Additive in
    /// v1.
    pub diff: Option<DiffV1>,
    /// With `--change`, the domains the diff suggests, in the order of the
    /// domains list; empty otherwise. Additive in v1.
    pub suggestions: Vec<SuggestionV1>,
    pub roles: Vec<RoleV1>,
    /// With `--at`, the roles serving the step that the configuration turned
    /// off (activation `never`), so that a skill writes `Disabled by
    /// configuration.` for them without a second call; empty otherwise.
    /// Additive in v1.
    pub disabled: Vec<DisabledRoleV1>,
    /// The review mode in effect and the material of a reviewing step.
    /// Additive in v1; `null` only in a failure's document.
    pub review: Option<ReviewModeV1>,
    pub status: Vec<StatusEntry>,
}

/// A role the configuration turned off at a step.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DisabledRoleV1 {
    pub id: String,
    /// The display name in the configured language, English otherwise.
    pub name: String,
    /// `project` or `change`: the file whose setting won.
    pub when_origin: &'static str,
}

/// How the reviewing roles run, and what they read.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewModeV1 {
    /// `inline` or `isolated`.
    pub mode: &'static str,
    /// `default`, `project` or `change`.
    pub origin: &'static str,
    /// With `--change` at `plan.trace`, `plan.challenge` or `code.review`;
    /// `null` otherwise, and at `code.review` without a diff.
    pub material: Option<MaterialV1>,
}

/// The material of a reviewing step, measured, never read.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MaterialV1 {
    pub limit_bytes: u64,
    pub total_bytes: u64,
    /// At least one file is to be summarized.
    pub summarized: bool,
    /// In path order.
    pub files: Vec<MaterialFileV1>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MaterialFileV1 {
    /// Relative to the project root, `/`-separated.
    pub path: String,
    pub bytes: u64,
    /// `full` or `summarize`.
    pub treatment: &'static str,
}

impl From<&crate::commands::ReviewInfo> for ReviewModeV1 {
    fn from(info: &crate::commands::ReviewInfo) -> Self {
        Self {
            mode: info.mode.as_str(),
            origin: info.origin.as_str(),
            material: info.material.as_ref().map(|material| MaterialV1 {
                limit_bytes: material.limit_bytes,
                total_bytes: material.total_bytes,
                summarized: material.summarized(),
                files: material
                    .files
                    .iter()
                    .map(|file| MaterialFileV1 {
                        path: file.path.clone(),
                        bytes: file.bytes,
                        treatment: file.treatment.as_str(),
                    })
                    .collect(),
            }),
        }
    }
}

/// The project's languages, as `codev roles` filters the knowledge for
/// them.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StacksV1 {
    /// `Rust`, `TypeScript`, `JavaScript`, `Python`, `Go`, `Java`, in that
    /// order.
    pub detected: Vec<&'static str>,
    /// `detected`, `none-detected` or `all-stacks`.
    pub filter: &'static str,
}

/// The change's diff, as the triggers read it.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DiffV1 {
    pub available: bool,
    /// Why there is no diff; `null` when there is one.
    pub reason: Option<String>,
    /// The default branch the diff starts from; `null` without a diff.
    pub base: Option<String>,
    /// Every changed file in the project directory.
    pub files: usize,
    /// The files whose added lines the patterns read.
    pub scanned: usize,
    /// The files skipped per category (`planning`, `vendored`, `lockfile`,
    /// `generated`, `binary`, `prose`, `large`), only those met.
    pub skipped: std::collections::BTreeMap<&'static str, usize>,
    /// A cap on files or bytes was reached.
    pub truncated: bool,
}

/// A domain the change's diff suggests.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SuggestionV1 {
    pub domain: String,
    /// `ticked`, `justified` or `unticked`.
    pub state: &'static str,
    /// The reason on the unticked line; `null` unless `justified`.
    pub justification: Option<String>,
    pub match_count: usize,
    /// At most five, in path then line order.
    pub matches: Vec<TriggerMatchV1>,
}

/// A trigger that matched the diff.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TriggerMatchV1 {
    /// The role whose trigger it is, or `project`.
    pub source: String,
    /// `paths`, `manifests` or `patterns`.
    pub kind: &'static str,
    /// The glob, file name or expression, as written.
    pub trigger: String,
    pub path: String,
    /// The added line, for a pattern; `null` otherwise.
    pub line: Option<u32>,
}

/// The most matches a suggestion lists.
const SUGGESTION_MATCHES: usize = 5;

impl From<&codev_engine::suggest::DiffSummary> for DiffV1 {
    fn from(diff: &codev_engine::suggest::DiffSummary) -> Self {
        Self {
            available: diff.available,
            reason: diff.reason.clone(),
            base: diff.base.clone(),
            files: diff.files,
            scanned: diff.scanned,
            skipped: diff.skipped.clone(),
            truncated: diff.truncated,
        }
    }
}

impl From<&codev_core::triggers::Suggestion> for SuggestionV1 {
    fn from(suggestion: &codev_core::triggers::Suggestion) -> Self {
        Self {
            domain: suggestion.domain.clone(),
            state: suggestion.state.as_str(),
            justification: suggestion.justification.clone(),
            match_count: suggestion.matches.len(),
            matches: suggestion
                .matches
                .iter()
                .take(SUGGESTION_MATCHES)
                .map(|m| TriggerMatchV1 {
                    source: m.source.clone(),
                    kind: m.kind.as_str(),
                    trigger: m.trigger.clone(),
                    path: m.path.clone(),
                    line: m.line,
                })
                .collect(),
        }
    }
}

impl From<&crate::commands::RolesOutcome> for RolesV1 {
    fn from(outcome: &crate::commands::RolesOutcome) -> Self {
        Self {
            change_name: outcome.change.as_ref().map(ToString::to_string),
            schema_name: Some(outcome.schema_name.clone()),
            at: outcome.at.clone(),
            language: outcome.language.clone(),
            stacks: Some(StacksV1 {
                detected: outcome.stacks.detected.clone(),
                filter: outcome.stacks.filter.as_str(),
            }),
            diff: outcome
                .suggestions
                .as_ref()
                .map(|found| DiffV1::from(&found.diff)),
            suggestions: outcome
                .suggestions
                .iter()
                .flat_map(|found| found.suggestions.iter().map(SuggestionV1::from))
                .collect(),
            roles: outcome
                .roles
                .iter()
                .map(|role| RoleV1::from(role, &outcome.language, outcome.at.as_deref()))
                .collect(),
            disabled: outcome
                .disabled
                .iter()
                .map(|role| DisabledRoleV1 {
                    id: role.role.id.clone(),
                    name: role.role.name(&outcome.language).to_string(),
                    when_origin: role.when_origin.as_str(),
                })
                .collect(),
            review: Some(ReviewModeV1::from(&outcome.review)),
            status: statuses(&outcome.warnings),
        }
    }
}

impl From<&Instructions> for InstructionsV1 {
    fn from(instructions: &Instructions) -> Self {
        Self {
            change_name: instructions.change.to_string(),
            schema_name: instructions.schema_name.clone(),
            artifact: instructions.artifact_id.clone(),
            description: instructions.description.clone(),
            resolved_output_path: json_path(&instructions.resolved_output_path),
            instruction: instructions.instruction.clone(),
            template: instructions.template.clone(),
            language: instructions.language.clone(),
            context: instructions.context.iter().map(BlockV1::from).collect(),
            rules: instructions.rules.iter().map(BlockV1::from).collect(),
            dependencies: instructions
                .dependencies
                .iter()
                .map(|d| DependencyV1 {
                    id: d.id.clone(),
                    path: json_path(&d.path),
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
                    path: json_path(&d.path),
                    origin: d.origin.clone(),
                })
                .collect(),
            skipped: instructions.skipped,
            role: instructions
                .role
                .as_ref()
                .map(|role| RoleV1::from(role, &instructions.language, None)),
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
    /// One entry per repository integration step (`CLAUDE.md`, CI, pull
    /// request template). Additive in v1; empty when no step ran.
    pub integration: Vec<IntegrationStepV1>,
    pub status: Vec<StatusEntry>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IntegrationStepV1 {
    /// `claude_md`, `ci` or `pr_template`.
    pub step: &'static str,
    /// `created`, `appended`, `refreshed`, `unchanged` or `skipped`.
    pub action: &'static str,
    /// Relative to the project, with `/` separators; `null` when no file is
    /// concerned.
    pub path: Option<String>,
    pub reason: Option<String>,
}

impl From<&codev_engine::integration::IntegrationStep> for IntegrationStepV1 {
    fn from(step: &codev_engine::integration::IntegrationStep) -> Self {
        Self {
            step: step.step.as_str(),
            action: step.action.as_str(),
            path: step.path.as_deref().map(json_path),
            reason: step.reason.clone(),
        }
    }
}

/// `codev doctor --json`.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DoctorReportV1 {
    pub root: String,
    pub checks: Vec<CheckV1>,
    pub summary: DoctorSummaryV1,
    pub status: Vec<StatusEntry>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckV1 {
    /// Stable: `config.rules`, `config.language`, `skills`, `sources`,
    /// `codev_version`, `specs`, `decisions`, `validate`, `claude_md`, `ci`, `pr_template`,
    /// `changelog`.
    pub id: &'static str,
    /// `configuration`, `planning` or `repository`.
    pub group: &'static str,
    /// `ok`, `missing` or `warning`.
    pub status: &'static str,
    /// `ok`, or a stable code such as `rules_missing`.
    pub code: &'static str,
    pub message: String,
    /// The command or skill that fixes it; `null` when `ok`.
    pub fix: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DoctorSummaryV1 {
    pub total: usize,
    pub missing: usize,
    pub warnings: usize,
}

impl From<&crate::commands::DoctorOutcome> for DoctorReportV1 {
    fn from(outcome: &crate::commands::DoctorOutcome) -> Self {
        let (total, missing, warnings) = codev_core::doctor::summary(&outcome.checks);
        Self {
            root: json_path(&outcome.root),
            checks: outcome
                .checks
                .iter()
                .map(|c| CheckV1 {
                    id: c.id,
                    group: c.group.as_str(),
                    status: c.status.as_str(),
                    code: c.code,
                    message: c.message.clone(),
                    fix: c.fix.clone(),
                })
                .collect(),
            summary: DoctorSummaryV1 {
                total,
                missing,
                warnings,
            },
            status: Vec::new(),
        }
    }
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
            root: json_path(&report.root),
            has_warnings: report.has_warnings(),
            items: report
                .items
                .iter()
                .map(|item| ItemReportV1 {
                    kind: match item.kind {
                        ItemKind::Change => "change",
                        ItemKind::Spec => "spec",
                        ItemKind::Decisions => "decisions",
                        ItemKind::Config => "config",
                    },
                    name: item.name.clone(),
                    path: json_path(&item.path),
                    findings: item
                        .findings
                        .iter()
                        .map(|f| FindingV1 {
                            path: json_path(&f.path),
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

/// Every path of a JSON document goes through this: `/` separators on
/// every OS (see `codev_core::layout::slashed`).
pub fn json_path(path: &std::path::Path) -> String {
    codev_core::layout::slashed(path)
}

/// A list of paths for a JSON document, each through [`json_path`].
pub fn json_paths(paths: &[std::path::PathBuf]) -> Vec<String> {
    paths.iter().map(|p| json_path(p)).collect()
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
            root: json_path(&outcome.root),
            updated: json_paths(&outcome.updated),
            created: json_paths(&outcome.created),
            unchanged: json_paths(&outcome.unchanged),
            deleted: json_paths(&outcome.deleted),
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
            root: json_path(&outcome.root),
            updated: json_paths(&outcome.updated),
            created: json_paths(&outcome.created),
            unchanged: json_paths(&outcome.unchanged),
            deleted: json_paths(&outcome.deleted),
            moved_to: json_path(&outcome.moved_to),
            status: Vec::new(),
        }
    }
}

/// `codev review --json`: a change's review record, its defects and the
/// archive gate.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewV1 {
    pub change_name: String,
    /// `review.md`, relative to the project root.
    pub path: String,
    pub exists: bool,
    /// The record's format; `null` without a record.
    pub format: Option<u32>,
    pub steps: Vec<ReviewStepV1>,
    pub findings: Vec<ReviewFindingV1>,
    /// The record's defects, as `codev validate` reports them.
    pub issues: Vec<FindingV1>,
    pub archive: ReviewGateV1,
    pub status: Vec<StatusEntry>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewStepV1 {
    pub step: String,
    /// The record has a section for this step.
    pub recorded: bool,
    /// The roles active at this step for the change.
    pub active_roles: Vec<String>,
    pub roles: Vec<ReviewRoleV1>,
    /// Its `Mode:` line, `inline` or `isolated`; `null` when not written.
    /// Additive in v1.
    pub mode: Option<&'static str>,
    /// Its `Summarized:` lines, in file order. Additive in v1.
    pub summarized: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewRoleV1 {
    pub role: String,
    /// `findings`, `no-finding`, `disabled`, `empty`, or `missing` for an
    /// active role without a section.
    pub state: &'static str,
    /// Its `Fallback:` line: why it ran inline. Additive in v1.
    pub fallback: Option<String>,
    /// The findings of other roles of the step that credit it in their
    /// `raised_by`. Additive in v1.
    pub credited: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewFindingV1 {
    pub id: String,
    pub title: String,
    pub role: String,
    pub step: String,
    /// The line of its heading in `review.md`.
    pub line: u32,
    pub severity: Option<&'static str>,
    pub confidence: Option<&'static str>,
    /// Presented to the human: confidence `high`.
    pub reported: bool,
    pub location: String,
    pub evidence: String,
    pub recommendation: String,
    pub status: Option<&'static str>,
    /// `author`, `reviewer`, `human`, or `null` when settled.
    pub awaiting: Option<&'static str>,
    pub history: Vec<ReviewEntryV1>,
    /// Every role that raised it, its own first. Additive in v1.
    pub raised_by: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewEntryV1 {
    pub status: &'static str,
    pub by: String,
    pub note: Option<String>,
    #[serde(rename = "ref")]
    pub reference: Option<String>,
    pub to: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewGateV1 {
    pub ready: bool,
    pub blockers: Vec<ReviewBlockerV1>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewBlockerV1 {
    pub code: &'static str,
    pub message: String,
}

impl From<&crate::commands::ReviewOutcome> for ReviewV1 {
    fn from(outcome: &crate::commands::ReviewOutcome) -> Self {
        let check = &outcome.check;
        let record = check.record.as_ref();
        Self {
            change_name: outcome.change.to_string(),
            path: json_path(&check.path),
            exists: record.is_some(),
            format: record.map(|r| r.format),
            steps: codev_core::review::STEPS
                .iter()
                .map(|step| review_step(check, step))
                .collect(),
            findings: record
                .map(|r| r.findings().map(review_finding).collect())
                .unwrap_or_default(),
            issues: check
                .findings
                .iter()
                .map(|f| FindingV1 {
                    path: json_path(&check.path),
                    line: f.line,
                    severity: severity_label(f.severity),
                    code: f.code,
                    message: f.message.clone(),
                })
                .collect(),
            archive: ReviewGateV1 {
                ready: check.blockers.is_empty(),
                blockers: check
                    .blockers
                    .iter()
                    .map(|b| ReviewBlockerV1 {
                        code: b.code,
                        message: b.message.clone(),
                    })
                    .collect(),
            },
            status: statuses(&outcome.warnings),
        }
    }
}

/// A step of the record: its role sections, plus the active roles that
/// have none when the step is recorded.
fn review_step(check: &codev_engine::review::ReviewCheck, step: &str) -> ReviewStepV1 {
    let active: Vec<String> = check
        .steps
        .iter()
        .find(|s| s.step == step)
        .map(|s| s.active.clone())
        .unwrap_or_default();
    let section = check.record.as_ref().and_then(|r| r.step(step));
    let mut roles: Vec<ReviewRoleV1> = section
        .map(|s| {
            s.roles
                .iter()
                .map(|r| ReviewRoleV1 {
                    role: r.role.clone(),
                    state: r.state.as_str(),
                    fallback: r.fallback.clone(),
                    credited: s.credited(&r.role).iter().map(|f| f.id.clone()).collect(),
                })
                .collect()
        })
        .unwrap_or_default();
    if let Some(section) = section {
        for role in active.iter().filter(|role| section.role(role).is_none()) {
            roles.push(ReviewRoleV1 {
                role: role.clone(),
                state: "missing",
                fallback: None,
                credited: section
                    .credited(role)
                    .iter()
                    .map(|f| f.id.clone())
                    .collect(),
            });
        }
    }
    ReviewStepV1 {
        step: step.to_string(),
        recorded: section.is_some(),
        active_roles: active,
        roles,
        mode: section.and_then(|s| s.mode).map(|m| m.as_str()),
        summarized: section.map(|s| s.summarized.clone()).unwrap_or_default(),
    }
}

fn review_finding(f: &codev_core::review::ReviewFinding) -> ReviewFindingV1 {
    ReviewFindingV1 {
        id: f.id.clone(),
        title: f.title.clone(),
        role: f.role.clone(),
        step: f.step.clone(),
        line: f.line,
        severity: f.severity.map(|s| s.as_str()),
        confidence: f.confidence.map(|c| c.as_str()),
        reported: f.reported(),
        location: f.location_text.clone(),
        evidence: f.evidence.clone(),
        recommendation: f.recommendation.clone(),
        status: f.status().map(|s| s.as_str()),
        awaiting: f.awaiting().map(|a| a.as_str()),
        history: f
            .history
            .iter()
            .map(|e| ReviewEntryV1 {
                status: e.status.as_str(),
                by: e.by.clone(),
                note: e.note.clone(),
                reference: e.reference.clone(),
                to: e.to.clone(),
            })
            .collect(),
        raised_by: f.raisers(),
    }
}

fn severity_label(severity: codev_core::parser::ast::Severity) -> &'static str {
    use codev_core::parser::ast::Severity;
    match severity {
        Severity::Error => "error",
        Severity::Warning => "warning",
        Severity::Info => "info",
    }
}

/// The shape of a failed `codev review --json`.
pub fn review_shape() -> serde_json::Value {
    serde_json::json!({
        "changeName": null,
        "path": null,
        "exists": false,
        "format": null,
        "steps": [],
        "findings": [],
        "issues": [],
        "archive": { "ready": false, "blockers": [] },
    })
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

impl From<&crate::commands::NewChangeOutcome> for NewChangeV1 {
    fn from(outcome: &crate::commands::NewChangeOutcome) -> Self {
        Self {
            change_name: outcome.change.to_string(),
            schema_name: outcome.schema_name.clone(),
            change_root: json_path(&outcome.change_root),
            created: json_paths(&outcome.created),
            status: statuses(&outcome.warnings),
        }
    }
}

/// `codev changelog --format json`.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangelogV1 {
    pub root: Option<String>,
    /// The `--since` used, explicit or the default tag; `null` when every
    /// archived change is in range.
    pub since: Option<String>,
    /// The `--until` used; `null` for the working tree.
    pub until: Option<String>,
    pub release: Option<ChangelogReleaseV1>,
    pub entries: Vec<ChangelogEntryV1>,
    /// The entries rendered as Keep a Changelog markdown.
    pub markdown: String,
    /// What `--write` did; `null` without it.
    pub written: Option<ChangelogWrittenV1>,
    pub status: Vec<StatusEntry>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangelogReleaseV1 {
    pub version: String,
    pub date: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangelogEntryV1 {
    /// `Added`, `Changed`, `Deprecated`, `Removed`, `Fixed` or `Security`.
    pub category: &'static str,
    /// As written, `**BREAKING**` marker included.
    pub text: String,
    pub breaking: bool,
    /// The archive folder name, `<date>-<name>`.
    pub change: String,
    /// Relative to the project root, `/`-separated.
    pub archive_path: String,
    /// `release_note`, or `what_changes` for a change archived without one.
    pub source: &'static str,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangelogWrittenV1 {
    pub path: String,
    /// `created`, `updated` or `unchanged`.
    pub action: &'static str,
}

impl From<&crate::commands::ChangelogOutcome> for ChangelogV1 {
    fn from(outcome: &crate::commands::ChangelogOutcome) -> Self {
        use codev_core::changelog::EntrySource;
        Self {
            root: Some(json_path(&outcome.root)),
            since: outcome.since.clone(),
            until: outcome.until.clone(),
            release: outcome
                .release
                .as_ref()
                .map(|(version, date)| ChangelogReleaseV1 {
                    version: version.clone(),
                    date: date.clone(),
                }),
            entries: outcome
                .entries
                .iter()
                .map(|e| ChangelogEntryV1 {
                    category: e.category.as_str(),
                    text: e.text.clone(),
                    breaking: e.breaking,
                    change: e.change.clone(),
                    archive_path: e.archive_path.clone(),
                    source: match e.source {
                        EntrySource::ReleaseNote => "release_note",
                        EntrySource::WhatChanges => "what_changes",
                    },
                })
                .collect(),
            markdown: outcome.rendered.clone(),
            written: outcome
                .written
                .as_ref()
                .map(|(path, action)| ChangelogWrittenV1 {
                    path: json_path(path),
                    action: action.as_str(),
                }),
            status: statuses(&outcome.warnings),
        }
    }
}

/// The empty shape of `ChangelogV1`, for failures.
pub fn changelog_shape() -> serde_json::Value {
    json!({
        "root": null,
        "since": null,
        "until": null,
        "release": null,
        "entries": [],
        "markdown": "",
        "written": null,
    })
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
            std::path::Path::new("/p"),
            std::path::Path::new("/p/_codev/changes/add-auth"),
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

    /// A Windows project root, built with `\` explicitly: on Unix it is a
    /// single odd component, which is enough to prove the conversion on
    /// every platform (the Windows CI job runs it on real paths).
    const WINDOWS_ROOT: &str = "C:\\work\\acme";

    fn windows(path: &str) -> PathBuf {
        PathBuf::from(format!("{WINDOWS_ROOT}\\{}", path.replace('/', "\\")))
    }

    #[test]
    fn status_paths_use_slashes_on_windows() {
        let graph = ArtifactGraph::from_yaml(SPEC_DRIVEN).unwrap();
        let change = ChangeId::parse("add-auth").unwrap();
        let status = status::compute(&graph, &change, &BTreeSet::new(), &BTreeSet::new());
        let json = serde_json::to_value(StatusV1::new(
            &status,
            std::path::Path::new(WINDOWS_ROOT),
            &windows("_codev/changes/add-auth"),
            &[],
        ))
        .unwrap();
        assert_eq!(json["planningHome"], "C:/work/acme");
        assert_eq!(json["changeRoot"], "C:/work/acme/_codev/changes/add-auth");
    }

    #[test]
    fn instructions_paths_use_slashes_on_windows() {
        use codev_engine::instructions::{DecisionRef, Dependency};
        let instructions = Instructions {
            change: ChangeId::parse("add-auth").unwrap(),
            schema_name: "spec-driven".into(),
            artifact_id: "specs".into(),
            description: None,
            resolved_output_path: windows("_codev/changes/add-auth/specs/**/*.md"),
            instruction: None,
            template: None,
            language: "en".into(),
            context: Vec::new(),
            rules: Vec::new(),
            dependencies: vec![Dependency {
                id: "proposal".into(),
                path: windows("_codev/changes/add-auth/proposal.md"),
                done: true,
            }],
            unlocks: Vec::new(),
            decisions: vec![DecisionRef {
                id: "0001".into(),
                qualified_id: "project/0001".into(),
                title: "Foundation".into(),
                status: "accepted".into(),
                tags: vec![],
                path: PathBuf::from("_codev\\decisions\\0001-foundation.md"),
                origin: "project".into(),
            }],
            skipped: false,
            role: None,
            warnings: Vec::new(),
        };
        let json = serde_json::to_value(InstructionsV1::from(&instructions)).unwrap();
        assert_eq!(
            json["resolvedOutputPath"],
            "C:/work/acme/_codev/changes/add-auth/specs/**/*.md"
        );
        assert_eq!(
            json["dependencies"][0]["path"],
            "C:/work/acme/_codev/changes/add-auth/proposal.md"
        );
        assert_eq!(
            json["decisions"][0]["path"],
            "_codev/decisions/0001-foundation.md"
        );
    }

    fn skeptic_with_a_project_criterion() -> ResolvedRole {
        use codev_core::roles::{self, ActivatedBy, Activation, Criterion, WhenOrigin};
        let mut role = roles::builtin_step_roles()
            .into_iter()
            .find(|r| r.id == "skeptic")
            .unwrap();
        role.criteria.push(Criterion {
            origin: "project".into(),
            text: "The rollback plan is stated".into(),
        });
        ResolvedRole {
            role,
            when: Activation::Always,
            when_origin: WhenOrigin::Project,
            activated_by: Some(ActivatedBy::Project),
        }
    }

    #[test]
    fn review_document_exposes_the_contract_fields() {
        let record = codev_core::review::parse(
            "## code.review\n\n### newcomer\n\n#### R-newcomer-1 T\n\n```yaml\nseverity: low\nconfidence: high\nlocation: a.rs:1\nevidence: x\nrecommendation: y\nhistory:\n  - status: open\n    by: newcomer\n  - status: fixed\n    by: developer\n    ref: abc\n```\n",
        )
        .value;
        let outcome = crate::commands::ReviewOutcome {
            change: ChangeId::parse("add-auth").unwrap(),
            check: codev_engine::review::ReviewCheck {
                path: "_codev/changes/add-auth/review.md".into(),
                record: Some(record),
                findings: Vec::new(),
                steps: vec![codev_core::review::StepRoles {
                    step: "code.review".into(),
                    active: vec!["newcomer".into(), "security".into()],
                }],
                blockers: Vec::new(),
            },
            warnings: Vec::new(),
        };
        let json = serde_json::to_value(ReviewV1::from(&outcome)).unwrap();
        for field in [
            "changeName",
            "path",
            "exists",
            "format",
            "steps",
            "findings",
            "issues",
            "archive",
            "status",
        ] {
            assert!(json.get(field).is_some(), "missing `{field}`");
        }
        let finding = &json["findings"][0];
        for field in [
            "id",
            "title",
            "role",
            "step",
            "line",
            "severity",
            "confidence",
            "reported",
            "location",
            "evidence",
            "recommendation",
            "status",
            "awaiting",
            "history",
            "raisedBy",
        ] {
            assert!(finding.get(field).is_some(), "missing `findings[].{field}`");
        }
        assert_eq!(finding["raisedBy"], serde_json::json!(["newcomer"]));
        for field in ["mode", "summarized"] {
            assert!(
                json["steps"][1].get(field).is_some(),
                "missing `steps[].{field}`"
            );
        }
        for field in ["fallback", "credited"] {
            assert!(
                json["steps"][1]["roles"][0].get(field).is_some(),
                "missing `steps[].roles[].{field}`"
            );
        }
        assert_eq!(finding["history"][1]["ref"], "abc");
        assert_eq!(finding["awaiting"], "reviewer");
        let code_review = &json["steps"][1];
        assert_eq!(code_review["activeRoles"][1], "security");
        // An active role without a section is reported as missing.
        assert_eq!(code_review["roles"][1]["role"], "security");
        assert_eq!(code_review["roles"][1]["state"], "missing");
        assert_eq!(json["archive"]["ready"], true);
    }

    /// A suggestion for `security` with `count` path matches.
    fn security_suggestions(count: usize) -> codev_engine::suggest::Suggestions {
        use codev_core::triggers::{Kind, Match, State, Suggestion};
        codev_engine::suggest::Suggestions {
            diff: codev_engine::suggest::DiffSummary {
                available: true,
                reason: None,
                base: Some("origin/main".into()),
                files: 9,
                scanned: 7,
                skipped: [("lockfile", 1), ("binary", 1)].into_iter().collect(),
                truncated: false,
            },
            suggestions: vec![Suggestion {
                domain: "security".into(),
                state: State::Justified,
                justification: Some("only renames a variable".into()),
                line: Some(12),
                matches: (0..count)
                    .map(|n| Match {
                        path: format!("src/auth/f{n}.rs"),
                        line: None,
                        domain: "security".into(),
                        source: "security".into(),
                        kind: Kind::Paths,
                        trigger: "**/auth/**".into(),
                    })
                    .collect(),
            }],
            warnings: Vec::new(),
        }
    }

    #[test]
    fn roles_document_exposes_stacks_diff_and_suggestions() {
        let outcome = crate::commands::RolesOutcome {
            change: Some(ChangeId::parse("add-auth").unwrap()),
            schema_name: "spec-driven".into(),
            at: None,
            language: "en".into(),
            roles: Vec::new(),
            disabled: Vec::new(),
            knowledge: false,
            stacks: crate::commands::Stacks {
                detected: vec!["Rust", "Go"],
                filter: crate::commands::StackFilter::Detected,
            },
            suggestions: Some(security_suggestions(7)),
            review: Default::default(),
            warnings: Vec::new(),
        };
        let json = serde_json::to_value(RolesV1::from(&outcome)).unwrap();
        assert_eq!(
            json["stacks"],
            serde_json::json!({ "detected": ["Rust", "Go"], "filter": "detected" })
        );
        assert_eq!(
            json["diff"],
            serde_json::json!({
                "available": true, "reason": null, "base": "origin/main",
                "files": 9, "scanned": 7,
                "skipped": { "binary": 1, "lockfile": 1 }, "truncated": false
            })
        );
        let suggestion = &json["suggestions"][0];
        assert_eq!(suggestion["domain"], "security");
        assert_eq!(suggestion["state"], "justified");
        assert_eq!(suggestion["justification"], "only renames a variable");
        assert_eq!(suggestion["matchCount"], 7);
        assert_eq!(
            suggestion["matches"].as_array().unwrap().len(),
            5,
            "at most five"
        );
        assert_eq!(
            suggestion["matches"][0],
            serde_json::json!({
                "source": "security", "kind": "paths", "trigger": "**/auth/**",
                "path": "src/auth/f0.rs", "line": null
            })
        );
    }

    #[test]
    fn roles_document_exposes_the_review_mode() {
        let outcome = crate::commands::RolesOutcome {
            change: Some(ChangeId::parse("add-auth").unwrap()),
            schema_name: "spec-driven".into(),
            at: Some("code.review".into()),
            language: "en".into(),
            roles: Vec::new(),
            disabled: Vec::new(),
            knowledge: false,
            stacks: crate::commands::Stacks::default(),
            suggestions: None,
            review: crate::commands::ReviewInfo {
                mode: codev_core::review::ReviewMode::Isolated,
                origin: codev_core::review::ModeOrigin::Change,
                material: Some(codev_core::review::material::budget(
                    &[("src/a.rs".into(), 10), ("src/big.rs".into(), 200_000)],
                    codev_core::review::material::LIMIT_BYTES,
                )),
            },
            warnings: Vec::new(),
        };
        let json = serde_json::to_value(RolesV1::from(&outcome)).unwrap();
        assert_eq!(
            json["review"],
            serde_json::json!({
                "mode": "isolated", "origin": "change",
                "material": {
                    "limitBytes": 163840, "totalBytes": 200010, "summarized": true,
                    "files": [
                        { "path": "src/a.rs", "bytes": 10, "treatment": "full" },
                        { "path": "src/big.rs", "bytes": 200000, "treatment": "summarize" }
                    ]
                }
            })
        );
    }

    #[test]
    fn roles_document_exposes_the_contract_fields() {
        let outcome = crate::commands::RolesOutcome {
            change: Some(ChangeId::parse("add-auth").unwrap()),
            schema_name: "spec-driven".into(),
            at: Some("plan.challenge".into()),
            language: "fr".into(),
            roles: vec![skeptic_with_a_project_criterion()],
            disabled: Vec::new(),
            knowledge: false,
            stacks: crate::commands::Stacks::default(),
            suggestions: None,
            review: Default::default(),
            warnings: Vec::new(),
        };
        let json = serde_json::to_value(RolesV1::from(&outcome)).unwrap();
        for field in [
            "changeName",
            "schemaName",
            "at",
            "language",
            "stacks",
            "diff",
            "suggestions",
            "roles",
            "disabled",
            "review",
            "status",
        ] {
            assert!(json.get(field).is_some(), "missing `{field}`");
        }
        assert!(json["diff"].is_null(), "no suggestions computed, no diff");
        assert_eq!(
            json["review"],
            serde_json::json!({ "mode": "inline", "origin": "default", "material": null })
        );
        let role = &json["roles"][0];
        for field in [
            "id",
            "name",
            "kind",
            "at",
            "steps",
            "artifact",
            "domains",
            "question",
            "when",
            "whenOrigin",
            "activatedBy",
            "disableable",
            "knowledge",
            "doneWhen",
        ] {
            assert!(role.get(field).is_some(), "missing `roles[].{field}`");
        }
        assert_eq!(role["name"], "Contradicteur");
        assert_eq!(role["whenOrigin"], "project");
        assert_eq!(role["activatedBy"], "project");
        assert_eq!(role["steps"], serde_json::json!(["plan.challenge"]));
        assert!(
            role["knowledge"].as_str().unwrap().contains("## Signals"),
            "an active skeptic is served its knowledge"
        );
        let last = role["doneWhen"].as_array().unwrap().last().unwrap().clone();
        assert_eq!(
            last,
            serde_json::json!({"origin": "project", "text": "The rollback plan is stated"})
        );
    }

    #[test]
    fn instructions_expose_the_role_field() {
        use codev_core::roles::{self, ActivatedBy, Activation, WhenOrigin};
        let yaml = include_str!("../../../assets/schemas/spec-driven/schema.yaml");
        let schema = codev_core::Schema::parse(yaml).unwrap();
        let qa = roles::catalog(&schema, "spec-driven")
            .into_iter()
            .find(|r| r.id == "qa")
            .unwrap();
        let mut instructions = bare_instructions();
        let json = serde_json::to_value(InstructionsV1::from(&instructions)).unwrap();
        assert!(json["role"].is_null(), "no role: `role` is null");
        instructions.role = Some(ResolvedRole {
            role: qa,
            when: Activation::Always,
            when_origin: WhenOrigin::Default,
            activated_by: Some(ActivatedBy::Always),
        });
        let json = serde_json::to_value(InstructionsV1::from(&instructions)).unwrap();
        assert_eq!(json["role"]["id"], "qa");
        assert_eq!(json["role"]["kind"], "artifact");
        assert_eq!(json["role"]["artifact"], "specs");
        assert_eq!(json["role"]["at"], "plan.write");
        assert_eq!(json["role"]["disableable"], false);
        assert_eq!(json["role"]["doneWhen"][0]["origin"], "schema:spec-driven");
    }

    fn bare_instructions() -> Instructions {
        Instructions {
            change: ChangeId::parse("add-auth").unwrap(),
            schema_name: "spec-driven".into(),
            artifact_id: "specs".into(),
            description: None,
            resolved_output_path: PathBuf::from("/p/_codev/changes/add-auth/specs/**/*.md"),
            instruction: None,
            template: None,
            language: "en".into(),
            context: Vec::new(),
            rules: Vec::new(),
            dependencies: Vec::new(),
            unlocks: Vec::new(),
            decisions: Vec::new(),
            skipped: false,
            role: None,
            warnings: Vec::new(),
        }
    }

    #[test]
    fn new_change_paths_use_slashes_on_windows() {
        let outcome = crate::commands::NewChangeOutcome {
            change: ChangeId::parse("fix-bug").unwrap(),
            schema_name: "spec-driven".into(),
            change_root: windows("_codev/changes/fix-bug"),
            created: vec![windows("_codev/changes/fix-bug/change.yaml")],
            warnings: Vec::new(),
        };
        let json = serde_json::to_value(NewChangeV1::from(&outcome)).unwrap();
        assert_eq!(json["changeRoot"], "C:/work/acme/_codev/changes/fix-bug");
        assert_eq!(
            json["created"],
            serde_json::json!(["C:/work/acme/_codev/changes/fix-bug/change.yaml"])
        );
    }

    #[test]
    fn unix_paths_are_written_unchanged() {
        let graph = ArtifactGraph::from_yaml(SPEC_DRIVEN).unwrap();
        let change = ChangeId::parse("add-auth").unwrap();
        let status = status::compute(&graph, &change, &BTreeSet::new(), &BTreeSet::new());
        let json = serde_json::to_value(StatusV1::new(
            &status,
            std::path::Path::new("/home/you/acme"),
            std::path::Path::new("/home/you/acme/_codev/changes/add-auth"),
            &[],
        ))
        .unwrap();
        assert_eq!(json["planningHome"], "/home/you/acme");
        assert_eq!(json["changeRoot"], "/home/you/acme/_codev/changes/add-auth");
    }

    #[test]
    fn json_documents_never_render_a_native_path() {
        // Every JSON field is filled in this file or in `main.rs`; a path
        // rendered with `display()` keeps `\` on Windows, which the contract
        // forbids. Human output lives in `render.rs` and may stay native, so
        // the two files that build JSON documents must never use it: go
        // through `json_path` / `json_paths` instead.
        for (file, source) in [
            ("contract.rs", include_str!("contract.rs")),
            ("main.rs", include_str!("main.rs")),
        ] {
            let pattern = ["display()", ".to_string()"].concat();
            let offending: Vec<(usize, &str)> = source
                .lines()
                .enumerate()
                .filter(|(_, line)| line.contains(&pattern))
                .map(|(n, line)| (n + 1, line.trim()))
                .collect();
            assert!(
                offending.is_empty(),
                "{file} renders a native path into JSON: {offending:?}"
            );
        }
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
    fn validate_report_v1_carries_the_config_kind() {
        let item = ItemReport {
            kind: ItemKind::Config,
            name: "config".into(),
            path: PathBuf::from("_codev").join("config.yaml"),
            findings: vec![LocatedFinding {
                path: PathBuf::from("_codev").join("config.yaml"),
                finding: Finding {
                    severity: Severity::Warning,
                    code: "unknown_config_key",
                    line: 3,
                    message: "unknown".into(),
                },
            }],
        };
        let report = ValidateReport {
            root: PathBuf::from("/p"),
            items: vec![item],
        };
        let json = serde_json::to_value(ValidateReportV1::from(&report)).unwrap();
        assert_eq!(json["items"][0]["kind"], "config");
        assert_eq!(json["items"][0]["path"], "_codev/config.yaml");
        assert_eq!(json["items"][0]["findings"][0]["severity"], "warning");
        assert_eq!(json["hasWarnings"], true);
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
    fn accepted_report_exposes_the_expected_fields_in_camel_case() {
        let report = DecisionAcceptedV1 {
            root: "/p".into(),
            decision: None,
            path: Some("/p/_codev/decisions/0007-x.md".into()),
            body_sha256: Some("sha256:abc".into()),
            superseded: Vec::new(),
            status: Vec::new(),
        };
        let json = serde_json::to_value(&report).unwrap();
        for field in [
            "root",
            "decision",
            "path",
            "bodySha256",
            "superseded",
            "status",
        ] {
            assert!(json.get(field).is_some(), "accept: missing `{field}`");
        }
        assert_eq!(json["bodySha256"], "sha256:abc");
        assert_eq!(json["superseded"], serde_json::json!([]));
    }

    #[test]
    fn accepted_report_lists_the_superseded_predecessors() {
        let report = DecisionAcceptedV1 {
            root: "/p".into(),
            decision: None,
            path: Some("/p/_codev/decisions/0007-x.md".into()),
            body_sha256: Some("sha256:abc".into()),
            superseded: vec![SupersededDecisionV1 {
                id: "0003".into(),
                qualified_id: "project/0003".into(),
                path: "/p/_codev/decisions/0003-old.md".into(),
            }],
            status: Vec::new(),
        };
        let json = serde_json::to_value(&report).unwrap();
        assert_eq!(
            json["superseded"],
            serde_json::json!([{
                "id": "0003",
                "qualifiedId": "project/0003",
                "path": "/p/_codev/decisions/0003-old.md",
            }])
        );
    }

    #[test]
    fn deviated_and_promoted_reports_carry_no_hash() {
        // Both commands create a proposed, unsealed ADR: no `bodySha256`.
        let deviated = serde_json::to_value(DecisionDeviatedV1 {
            root: "/p".into(),
            decision: None,
            path: Some("/p/_codev/decisions/0001-x.md".into()),
            target_qualified_id: Some("path:~/shared/0100".into()),
            status: Vec::new(),
        })
        .unwrap();
        let promoted = serde_json::to_value(DecisionPromotedV1 {
            root: "/p".into(),
            decision: None,
            path: Some("/p/_codev/decisions/0001-x.md".into()),
            source_change: Some("add-auth".into()),
            design_path: Some("/p/_codev/changes/add-auth/design.md".into()),
            status: Vec::new(),
        })
        .unwrap();
        for json in [&deviated, &promoted] {
            assert!(json.get("bodySha256").is_none(), "{json}");
            assert!(json.get("path").is_some(), "{json}");
        }
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

    #[test]
    fn setup_report_exposes_integration_in_camel_case() {
        use codev_engine::integration::{IntegrationStep, Step, StepAction};
        let report = SetupV1 {
            root: "/p".into(),
            created: Vec::new(),
            updated: Vec::new(),
            untouched: Vec::new(),
            preserved: Vec::new(),
            skills: Vec::new(),
            integration: vec![
                IntegrationStepV1::from(&IntegrationStep {
                    step: Step::Ci,
                    action: StepAction::Created,
                    path: Some(PathBuf::from(".github/workflows/codev.yml")),
                    reason: None,
                    snippet: Some("not in the JSON".into()),
                }),
                IntegrationStepV1::from(&IntegrationStep::skipped(
                    Step::PrTemplate,
                    None,
                    "not a GitHub repository",
                )),
            ],
            status: Vec::new(),
        };
        let json = serde_json::to_value(report).unwrap();
        let steps = json["integration"].as_array().unwrap();
        assert_eq!(steps[0]["step"], "ci");
        assert_eq!(steps[0]["action"], "created");
        assert_eq!(steps[0]["path"], ".github/workflows/codev.yml");
        assert!(steps[0]["reason"].is_null());
        assert!(steps[0].get("snippet").is_none());
        assert_eq!(steps[1]["action"], "skipped");
        assert!(steps[1]["path"].is_null());
        assert_eq!(steps[1]["reason"], "not a GitHub repository");
    }

    #[test]
    fn doctor_report_exposes_the_expected_fields() {
        use codev_core::doctor::{DoctorFacts, evaluate};
        let outcome = crate::commands::DoctorOutcome {
            root: "/p".into(),
            checks: evaluate(&DoctorFacts {
                language_set: true,
                ..DoctorFacts::default()
            }),
        };
        let json = serde_json::to_value(DoctorReportV1::from(&outcome)).unwrap();
        for field in ["root", "checks", "summary", "status"] {
            assert!(json.get(field).is_some(), "missing `{field}`");
        }
        let checks = json["checks"].as_array().unwrap();
        for check in checks {
            for field in ["id", "group", "status", "code", "message", "fix"] {
                assert!(check.get(field).is_some(), "missing `{field}` in {check}");
            }
        }
        let rules = checks.iter().find(|c| c["id"] == "config.rules").unwrap();
        assert_eq!(rules["group"], "configuration");
        assert_eq!(rules["status"], "missing");
        assert_eq!(rules["code"], "rules_missing");
        assert_eq!(rules["fix"], "/codev-configure");
        let language = checks
            .iter()
            .find(|c| c["id"] == "config.language")
            .unwrap();
        assert_eq!(language["code"], "ok");
        assert!(language["fix"].is_null());
        let missing = checks.iter().filter(|c| c["status"] == "missing").count();
        assert_eq!(json["summary"]["missing"], missing);
        assert_eq!(json["summary"]["total"], checks.len());
    }

    #[test]
    fn new_change_json_carries_the_change_and_its_files() {
        let json = serde_json::to_value(NewChangeV1 {
            change_name: "add-auth".into(),
            schema_name: "spec-driven".into(),
            change_root: "/p/_codev/changes/add-auth".into(),
            created: vec!["/p/_codev/changes/add-auth/change.yaml".into()],
            status: Vec::new(),
        })
        .unwrap();
        assert_eq!(json["changeName"], "add-auth");
        assert_eq!(json["schemaName"], "spec-driven");
        assert_eq!(json["changeRoot"], "/p/_codev/changes/add-auth");
        assert_eq!(
            json["created"],
            json!(["/p/_codev/changes/add-auth/change.yaml"])
        );
        assert_eq!(json["status"], json!([]));
    }

    #[test]
    fn list_documents_carry_their_items_and_root() {
        let changes = serde_json::to_value(ChangesV1 {
            changes: vec!["add-auth".into(), "fix-bug".into()],
            root: Some("/p".into()),
            status: Vec::new(),
        })
        .unwrap();
        assert_eq!(
            changes,
            json!({ "changes": ["add-auth", "fix-bug"], "root": "/p", "status": [] })
        );
        let specs = serde_json::to_value(SpecsV1 {
            specs: vec!["identity/sso".into(), "user-auth".into()],
            root: Some("/p".into()),
            status: Vec::new(),
        })
        .unwrap();
        assert_eq!(
            specs,
            json!({ "specs": ["identity/sso", "user-auth"], "root": "/p", "status": [] })
        );
    }
}
