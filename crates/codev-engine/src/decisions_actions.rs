//! ADR creation and supersession plans — functional core: these
//! functions compute a `Plan`, the imperative CLI shell executes it.

use std::path::PathBuf;

use codev_core::decisions::DecisionStatus;
use codev_core::decisions::seal::{self, SealError, SealFile};
use codev_core::{Layout, Plan, WriteMode};

use crate::decisions::{DecisionIndex, Origin};
use crate::ports::FileSystem;

/// Reads and parses `seal.yaml` if it exists; otherwise returns an empty seal.
///
/// This is the single access point to `seal.yaml` on the shell side — used
/// before calling `plan_new`, `plan_accept` and `plan_seal`, and by
/// `validate` to check the hashes.
pub fn read_seal_file(fs: &dyn FileSystem, layout: &Layout) -> Result<SealFile, ActionError> {
    let path = layout.decisions_seal_file();
    if !fs.exists(&path) {
        return Ok(SealFile::empty());
    }
    let source = fs
        .read_to_string(&path)
        .map_err(|e| ActionError::Seal(SealError::Invalid(e.to_string())))?;
    seal::parse_seal_file(&source).map_err(ActionError::from)
}

const DECISION_TEMPLATE: &str = include_str!("../../../assets/templates/decision.md");

/// What an action can refuse.
///
/// The codes follow the same rule as the rest of the tool: stable and
/// testable on the agent side, while messages are free to be reworded.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionError {
    EmptyTitle,
    UnknownDecisionId {
        id: String,
    },
    CannotSupersedeInherited {
        qualified_id: String,
    },
    CannotSealInherited {
        qualified_id: String,
    },
    /// `decision accept` targets an inherited decision — read-only on the
    /// consumer side.
    CannotAcceptInherited {
        qualified_id: String,
    },
    /// `decision accept` targets a decision whose status is not
    /// `proposed` — only that transition carries a seal.
    DecisionNotProposed {
        id: String,
        status: String,
    },
    /// The decision to supersede — the target of `decision supersede`, or a
    /// predecessor listed in the `supersedes` of a decision being accepted —
    /// is not `accepted`, hence not in effect: there is nothing to replace.
    PredecessorNotAccepted {
        id: String,
        status: String,
        /// The accepted decision that superseded it in the meantime, if any.
        superseded_by: Option<String>,
    },
    /// The target passed to `decision deviate` is a local decision — the
    /// proper action for that is `decision supersede`.
    CannotDeviateFromLocal {
        qualified_id: String,
    },
    /// The target of a `decision promote` points to an archived change —
    /// an archived design is history, it is not modified.
    CannotPromoteFromArchived {
        change: String,
    },
    /// The change has no `design.md` — nothing can be promoted.
    DesignMissing {
        change: String,
    },
    /// No `### Decision: <title>` block matches.
    DecisionHeadingNotFound {
        title: String,
    },
    /// Several blocks share the same title — the user must disambiguate.
    AmbiguousDecisionHeading {
        title: String,
        lines: Vec<u32>,
    },
    AmbiguousDecisionId {
        id: String,
        candidates: Vec<String>,
    },
    /// A seal is already present, the body differs from the recorded hash,
    /// and `--force` was not requested. The stable code is `seal_conflict`.
    SealConflict {
        id: String,
    },
    /// An error from the `seal` module — stable code derived from it.
    Seal(SealError),
}

impl ActionError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::EmptyTitle => "empty_title",
            Self::UnknownDecisionId { .. } => "unknown_decision_id",
            Self::CannotSupersedeInherited { .. } => "cannot_supersede_inherited",
            Self::CannotSealInherited { .. } => "cannot_seal_inherited",
            Self::CannotAcceptInherited { .. } => "cannot_accept_inherited",
            Self::DecisionNotProposed { .. } => "decision_not_proposed",
            Self::PredecessorNotAccepted { .. } => "predecessor_not_accepted",
            Self::CannotDeviateFromLocal { .. } => "cannot_deviate_from_local",
            Self::CannotPromoteFromArchived { .. } => "cannot_promote_from_archived",
            Self::DesignMissing { .. } => "design_missing",
            Self::DecisionHeadingNotFound { .. } => "decision_heading_not_found",
            Self::AmbiguousDecisionHeading { .. } => "ambiguous_decision_heading",
            Self::AmbiguousDecisionId { .. } => "ambiguous_decision_id",
            Self::SealConflict { .. } => "seal_conflict",
            Self::Seal(err) => match err {
                SealError::MissingFrontmatterCloser => "seal_body_unreadable",
                SealError::UnsupportedVersion { .. } => "seal_version_unsupported",
                SealError::Invalid(_) => "seal_invalid",
                SealError::AlreadySealed { .. } => "seal_conflict",
                SealError::Unknown { .. } => "unknown_decision_id",
            },
        }
    }

    pub fn message(&self) -> String {
        match self {
            Self::EmptyTitle => "a non-empty title is required to create a decision".into(),
            Self::UnknownDecisionId { id } => format!(
                "no decision with identifier `{id}` in the project; \
                 `codev decision list` shows the existing ones"
            ),
            Self::CannotSupersedeInherited { qualified_id } => format!(
                "decision `{qualified_id}` is inherited, hence read-only; \
                 use `codev decision deviate` to depart from it locally"
            ),
            Self::CannotSealInherited { qualified_id } => format!(
                "decision `{qualified_id}` is inherited: sealing it is up to \
                 the source project, not the consumer"
            ),
            Self::CannotAcceptInherited { qualified_id } => format!(
                "decision `{qualified_id}` is inherited, hence read-only: \
                 accepting it is up to the source project"
            ),
            Self::DecisionNotProposed { id, status } if status == "accepted" => format!(
                "decision `{id}` is already accepted: nothing to do; \
                 to replace it, use `codev decision supersede {id}`"
            ),
            Self::DecisionNotProposed { id, status } => format!(
                "decision `{id}` has status `{status}`: only a `proposed` \
                 decision can be accepted"
            ),
            Self::PredecessorNotAccepted {
                id,
                status,
                superseded_by,
            } => {
                let by = superseded_by
                    .as_ref()
                    .map(|q| format!(", superseded by `{q}`"))
                    .unwrap_or_default();
                let hint = superseded_by
                    .as_ref()
                    .map(|q| format!("; supersede `{q}` instead"))
                    .unwrap_or_default();
                format!(
                    "decision `{id}` has status `{status}`{by}: only an \
                     `accepted` decision, in effect, can be superseded{hint}; \
                     `codev decision list` shows the chain"
                )
            }
            Self::CannotDeviateFromLocal { qualified_id } => format!(
                "decision `{qualified_id}` is local: the proper way to \
                 replace it is `codev decision supersede`, not \
                 `deviate` — which is reserved for inherited decisions"
            ),
            Self::CannotPromoteFromArchived { change } => format!(
                "change `{change}` is archived: an archived design is \
                 history; promote decisions before archiving"
            ),
            Self::DesignMissing { change } => format!(
                "change `{change}` has no `design.md` — nothing to promote; \
                 create it first with `codev-propose` or by hand"
            ),
            Self::DecisionHeadingNotFound { title } => format!(
                "no `### Decision: {title}` block found under `## Decisions` \
                 in the design; check the exact title (matching is \
                 case-sensitive)"
            ),
            Self::AmbiguousDecisionHeading { title, lines } => {
                let line_list = lines
                    .iter()
                    .map(u32::to_string)
                    .collect::<Vec<_>>()
                    .join(", ");
                format!(
                    "title `{title}` appears on multiple blocks \
                     (lines {line_list}) — edit one of the titles to make the \
                     choice unambiguous before running the promotion again"
                )
            }
            Self::AmbiguousDecisionId { id, candidates } => format!(
                "identifier `{id}` matches several decisions: {} — \
                 use a qualified identifier instead",
                candidates.join(", ")
            ),
            Self::SealConflict { id } => format!(
                "the body of `{id}` has changed since it was sealed; \
                 use `codev decision seal {id} --force` to deliberately \
                 rewrite the seal"
            ),
            Self::Seal(err) => err.to_string(),
        }
    }
}

impl From<SealError> for ActionError {
    fn from(err: SealError) -> Self {
        Self::Seal(err)
    }
}

impl std::fmt::Display for ActionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message())
    }
}

impl std::error::Error for ActionError {}

/// The plan computed for `codev decision new`.
#[derive(Debug)]
pub struct CreatePlan {
    pub plan: Plan,
    pub new_id: String,
    pub new_path: PathBuf,
    /// Hash of the new ADR's body — exposed so the CLI can report it in
    /// its success JSON without re-hashing.
    pub body_sha256: String,
}

/// The plan computed for `codev decision supersede`.
#[derive(Debug)]
pub struct SupersedePlan {
    pub plan: Plan,
    pub new_id: String,
    pub new_path: PathBuf,
    /// The decision the new one will supersede once accepted — it is not
    /// modified by the plan.
    pub old_qualified_id: String,
    pub old_path: PathBuf,
}

/// The plan computed for `codev decision seal`.
///
/// Two possible outcomes:
/// - non-empty `plan`, `body_sha256` carries the new hash → the shell has
///   a write to perform;
/// - empty `plan.writes` → silent no-op (seal already up to date).
#[derive(Debug)]
pub struct SealActionPlan {
    pub plan: Plan,
    pub id: String,
    pub body_sha256: String,
    pub sealed_at: String,
    pub was_noop: bool,
}

/// The plan computed for `codev decision accept`.
#[derive(Debug)]
pub struct AcceptPlan {
    pub plan: Plan,
    pub id: String,
    pub path: PathBuf,
    /// Hash of the accepted ADR's body — the one recorded in the seal.
    pub body_sha256: String,
    /// The predecessors listed in `supersedes`, rewritten to `superseded`
    /// by the same plan. Empty when the decision supersedes nothing.
    pub superseded: Vec<SupersededPredecessor>,
}

/// A predecessor that `decision accept` marks `superseded`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SupersededPredecessor {
    pub id: String,
    pub qualified_id: String,
    pub path: PathBuf,
}

/// The plan computed for `codev decision promote`.
#[derive(Debug)]
pub struct PromotePlan {
    pub plan: Plan,
    pub new_id: String,
    pub new_path: PathBuf,
    /// The name of the change the promotion comes from — useful for
    /// rendering and for the JSON contract.
    pub source_change: String,
    /// Path of the modified `design.md` — useful for the same reasons.
    pub design_path: PathBuf,
}

/// The plan computed for `codev decision deviate`.
#[derive(Debug)]
pub struct DeviatePlan {
    pub plan: Plan,
    pub new_id: String,
    pub new_path: PathBuf,
    /// The qualified identifier being deviated from — normalized (accepts
    /// a `path:...` or `git:...` with or without the explicit `origin/` as
    /// long as resolution is unambiguous).
    pub target_qualified_id: String,
}

/// Prepares the creation of a new local ADR.
///
/// `existing_seal` is the current content of `seal.yaml` — the shell has
/// read it beforehand. The resulting plan writes the ADR **and** the seal
/// update atomically: either both succeed, or nothing is written.
pub fn plan_new(
    index: &DecisionIndex,
    existing_seal: &SealFile,
    title: &str,
    status: DecisionStatus,
    today: &str,
    layout: &Layout,
) -> Result<CreatePlan, ActionError> {
    if title.trim().is_empty() {
        return Err(ActionError::EmptyTitle);
    }

    let next = next_local_id(index);
    let slug = slug_from_title(title);
    let filename = format!("{next}-{slug}.md");
    let new_path = layout.decisions_dir().join(&filename);

    let contents = render_new_adr(&next, title, &status, today, &[]);
    let body_sha256 = seal::body_hash(&contents)?;

    // The seal is only written if the ADR is `accepted` or `superseded` —
    // the other statuses (proposed, deprecated, rejected) carry no
    // immutability commitment.
    let mut plan = Plan::new();
    plan.dir(layout.decisions_dir());
    plan.write(new_path.clone(), contents, WriteMode::CreateOnly);
    if status_needs_seal(&status) {
        let new_seal = seal::plan_seal_new(
            existing_seal,
            next.clone(),
            body_sha256.clone(),
            today.to_string(),
        )?;
        plan.write(
            layout.decisions_seal_file(),
            seal::render_seal_file(&new_seal),
            WriteMode::Overwrite,
        );
    }

    Ok(CreatePlan {
        plan,
        new_id: next,
        new_path,
        body_sha256,
    })
}

/// Prepares a supersession: creates a `proposed` ADR that references the
/// old decision in its `supersedes`.
///
/// The old decision is not part of the plan: it stays `accepted`, and in
/// effect, until the new one is accepted — `plan_accept` rewrites it to
/// `superseded` then, in the same plan as the acceptance. Nothing is sealed
/// here: the new ADR's body is meant to be written first.
///
/// Explicit refusals: `EmptyTitle`, `UnknownDecisionId`,
/// `AmbiguousDecisionId`, `CannotSupersedeInherited`, and
/// `PredecessorNotAccepted` for a local decision that is not in effect —
/// its successor could never be accepted.
pub fn plan_supersede(
    index: &DecisionIndex,
    old_id: &str,
    new_title: &str,
    today: &str,
    layout: &Layout,
) -> Result<SupersedePlan, ActionError> {
    if new_title.trim().is_empty() {
        return Err(ActionError::EmptyTitle);
    }

    let (old_index, _) = resolve_old_entry(index, old_id)?;
    let old_entry = &index.entries[old_index];

    // An inherited decision stays read-only.
    if old_entry.qualified_id.origin != Origin::Project {
        return Err(ActionError::CannotSupersedeInherited {
            qualified_id: old_entry.qualified_id.as_str(),
        });
    }
    if old_entry.decision.status != DecisionStatus::Accepted {
        return Err(predecessor_not_accepted(index, old_entry, None));
    }

    let next = next_local_id(index);
    let new_slug = slug_from_title(new_title);
    let new_filename = format!("{next}-{new_slug}.md");
    let new_path = layout.decisions_dir().join(&new_filename);

    let new_adr = render_new_adr(
        &next,
        new_title,
        &DecisionStatus::Proposed,
        today,
        std::slice::from_ref(&old_entry.decision.id),
    );

    let mut plan = Plan::new();
    plan.dir(layout.decisions_dir());
    plan.write(new_path.clone(), new_adr, WriteMode::CreateOnly);

    Ok(SupersedePlan {
        plan,
        new_id: next,
        new_path,
        old_qualified_id: old_entry.qualified_id.as_str(),
        old_path: old_entry.path.clone(),
    })
}

/// Builds the `PredecessorNotAccepted` refusal for `entry`, naming the
/// accepted decision that superseded it, if any — other than `accepting`,
/// the decision whose acceptance is being planned.
fn predecessor_not_accepted(
    index: &DecisionIndex,
    entry: &crate::decisions::IndexEntry,
    accepting: Option<&str>,
) -> ActionError {
    let superseded_by = index
        .entries
        .iter()
        .filter(|e| e.qualified_id.origin == Origin::Project)
        .filter(|e| e.decision.status == DecisionStatus::Accepted)
        .filter(|e| Some(e.decision.id.as_str()) != accepting)
        .find(|e| e.decision.supersedes.contains(&entry.decision.id))
        .map(|e| e.qualified_id.as_str());
    ActionError::PredecessorNotAccepted {
        id: entry.decision.id.clone(),
        status: entry.decision.status.as_str().to_string(),
        superseded_by,
    }
}

/// Prepares a seal — the `codev decision seal <id>` command.
///
/// Three outcomes depending on the state:
/// - ADR not sealed → adds an entry (`was_noop = false`).
/// - ADR sealed, body unchanged → silent no-op (`was_noop = true`,
///   empty `plan.writes`).
/// - ADR sealed, body changed, `force = false` → refuses with
///   `SealConflict`.
/// - ADR sealed, body changed, `force = true` → rewrites the entry.
///
/// `adr_source` supplies the function with the current content of the ADR
/// file; same pattern as `plan_supersede` — the shell reads, the pure
/// function computes.
pub fn plan_seal(
    index: &DecisionIndex,
    existing_seal: &SealFile,
    id: &str,
    force: bool,
    today: &str,
    layout: &Layout,
    adr_source: impl FnOnce(&std::path::Path) -> std::io::Result<String>,
) -> Result<SealActionPlan, ActionError> {
    // Resolution: same mechanics as supersede — but an inherited decision
    // is refused with a dedicated code.
    let (idx, _) = resolve_old_entry(index, id)?;
    let entry = &index.entries[idx];
    if entry.qualified_id.origin != Origin::Project {
        return Err(ActionError::CannotSealInherited {
            qualified_id: entry.qualified_id.as_str(),
        });
    }

    let local_id = entry.decision.id.clone();
    let source = adr_source(&entry.path).map_err(|_| ActionError::UnknownDecisionId {
        id: local_id.clone(),
    })?;
    let body_sha256 = seal::body_hash(&source)?;

    let already = existing_seal.find(&local_id);
    let mut plan = Plan::new();

    match already {
        None => {
            // Case 1: not sealed yet — fresh insertion.
            let new_seal = seal::plan_seal_new(
                existing_seal,
                local_id.clone(),
                body_sha256.clone(),
                today.to_string(),
            )?;
            plan.dir(layout.decisions_dir());
            plan.write(
                layout.decisions_seal_file(),
                seal::render_seal_file(&new_seal),
                WriteMode::Overwrite,
            );
            Ok(SealActionPlan {
                plan,
                id: local_id,
                body_sha256,
                sealed_at: today.to_string(),
                was_noop: false,
            })
        }
        Some(existing_entry) if existing_entry.body_sha256 == body_sha256 => {
            // Case 2: no-op — the seal is already up to date.
            Ok(SealActionPlan {
                plan,
                id: local_id,
                body_sha256,
                sealed_at: existing_entry.sealed_at.clone(),
                was_noop: true,
            })
        }
        Some(_) if !force => {
            // Case 3: conflict without force.
            Err(ActionError::SealConflict { id: local_id })
        }
        Some(_) => {
            // Case 4: force → rewrite.
            let new_seal = seal::plan_seal_force(
                existing_seal,
                local_id.clone(),
                body_sha256.clone(),
                today.to_string(),
            )?;
            plan.dir(layout.decisions_dir());
            plan.write(
                layout.decisions_seal_file(),
                seal::render_seal_file(&new_seal),
                WriteMode::Overwrite,
            );
            Ok(SealActionPlan {
                plan,
                id: local_id,
                body_sha256,
                sealed_at: today.to_string(),
                was_noop: false,
            })
        }
    }
}

/// Prepares an acceptance — the `codev decision accept <id>` command.
///
/// Turns a local `proposed` decision into an `accepted` one: the plan
/// rewrites the frontmatter status **and** writes the seal entry, so the
/// shell writes both or, if the plan cannot be computed, neither. Only
/// the frontmatter changes, so the recorded hash is the hash of the body
/// the user wrote.
///
/// This is the single place where a decision takes effect: when the
/// decision carries a `supersedes`, the same plan rewrites each listed
/// predecessor to `superseded` — body byte for byte, seal untouched, so its
/// seal stays valid.
///
/// Explicit refusals: `UnknownDecisionId`, `AmbiguousDecisionId`,
/// `CannotAcceptInherited`, `DecisionNotProposed`, `SealConflict` if a
/// seal entry with another hash already exists for this id, and for a
/// predecessor: `UnknownDecisionId`, `CannotSupersedeInherited` or
/// `PredecessorNotAccepted`.
pub fn plan_accept(
    index: &DecisionIndex,
    existing_seal: &SealFile,
    id: &str,
    today: &str,
    layout: &Layout,
    mut adr_source: impl FnMut(&std::path::Path) -> std::io::Result<String>,
) -> Result<AcceptPlan, ActionError> {
    let (idx, _) = resolve_old_entry(index, id)?;
    let entry = &index.entries[idx];
    if entry.qualified_id.origin != Origin::Project {
        return Err(ActionError::CannotAcceptInherited {
            qualified_id: entry.qualified_id.as_str(),
        });
    }
    let local_id = entry.decision.id.clone();
    if entry.decision.status != DecisionStatus::Proposed {
        return Err(ActionError::DecisionNotProposed {
            id: local_id,
            status: entry.decision.status.as_str().to_string(),
        });
    }

    // Every predecessor is checked before anything is planned: one that
    // cannot be superseded refuses the whole acceptance.
    let mut predecessors = Vec::new();
    for target in &entry.decision.supersedes {
        let local = index
            .entries
            .iter()
            .find(|e| e.qualified_id.origin == Origin::Project && e.decision.id == *target);
        let Some(predecessor) = local else {
            // Not local: either inherited — read-only — or nowhere.
            return Err(
                match index.entries.iter().find(|e| e.decision.id == *target) {
                    Some(inherited) => ActionError::CannotSupersedeInherited {
                        qualified_id: inherited.qualified_id.as_str(),
                    },
                    None => ActionError::UnknownDecisionId { id: target.clone() },
                },
            );
        };
        if predecessor.decision.status != DecisionStatus::Accepted {
            return Err(predecessor_not_accepted(
                index,
                predecessor,
                Some(&local_id),
            ));
        }
        predecessors.push(predecessor);
    }

    let source = adr_source(&entry.path).map_err(|_| ActionError::UnknownDecisionId {
        id: local_id.clone(),
    })?;
    let accepted = rewrite_frontmatter_status(&source, &entry.decision, DecisionStatus::Accepted);
    let body_sha256 = seal::body_hash(&accepted)?;

    let mut plan = Plan::new();
    plan.dir(layout.decisions_dir());
    plan.write(entry.path.clone(), accepted, WriteMode::Overwrite);
    match existing_seal.find(&local_id) {
        // A leftover entry that already matches the body: nothing to add.
        Some(sealed) if sealed.body_sha256 == body_sha256 => {}
        // A leftover entry for another body: refuse rather than silently
        // rewrite a seal — `decision seal --force` is the deliberate path.
        Some(_) => return Err(ActionError::SealConflict { id: local_id }),
        None => {
            let new_seal = seal::plan_seal_new(
                existing_seal,
                local_id.clone(),
                body_sha256.clone(),
                today.to_string(),
            )?;
            plan.write(
                layout.decisions_seal_file(),
                seal::render_seal_file(&new_seal),
                WriteMode::Overwrite,
            );
        }
    }

    // The predecessors change status only; their body — hence their seal —
    // stays byte for byte identical, so `seal.yaml` needs no other change.
    let mut superseded = Vec::with_capacity(predecessors.len());
    for predecessor in predecessors {
        let old_source =
            adr_source(&predecessor.path).map_err(|_| ActionError::UnknownDecisionId {
                id: predecessor.decision.id.clone(),
            })?;
        plan.write(
            predecessor.path.clone(),
            rewrite_frontmatter_status(
                &old_source,
                &predecessor.decision,
                DecisionStatus::Superseded,
            ),
            WriteMode::Overwrite,
        );
        superseded.push(SupersededPredecessor {
            id: predecessor.decision.id.clone(),
            qualified_id: predecessor.qualified_id.as_str(),
            path: predecessor.path.clone(),
        });
    }

    Ok(AcceptPlan {
        plan,
        id: local_id,
        path: entry.path.clone(),
        body_sha256,
        superseded,
    })
}

/// Does a status carry an immutability commitment?
fn status_needs_seal(status: &DecisionStatus) -> bool {
    matches!(
        status,
        DecisionStatus::Accepted | DecisionStatus::Superseded
    )
}

/// The quote line that replaces the body of a `### Decision:` block after
/// promotion. Emitted here so it stays testable without I/O.
fn build_promote_reference(new_id: &str, new_slug: &str) -> String {
    format!("\n> Promoted to ADR **{new_id}** — see `_codev/decisions/{new_id}-{new_slug}.md`.\n\n")
}

/// Prepares the promotion of a `### Decision: <title>` block from a
/// `design.md` into a local `proposed` ADR. The promoted body is meant to
/// be reworked, so nothing is sealed: `plan_accept` seals the ADR once
/// its text is final.
///
/// `design_source`: current content of the design (the shell has read it).
/// `design_path`: on-disk path of the design, for the plan's write.
///
/// Explicit refusals — see `ActionError`: `EmptyTitle`, `DesignMissing`,
/// `DecisionHeadingNotFound`, `AmbiguousDecisionHeading`.
#[allow(clippy::too_many_arguments)]
pub fn plan_promote(
    index: &DecisionIndex,
    change_name: &str,
    heading: &str,
    design_source: &str,
    design_path: PathBuf,
    today: &str,
    layout: &Layout,
) -> Result<PromotePlan, ActionError> {
    if heading.trim().is_empty() {
        return Err(ActionError::EmptyTitle);
    }

    let blocks = crate::design::extract_decision_blocks(design_source);
    let idx = crate::design::find_decision_block(&blocks, heading).map_err(|err| match err {
        crate::design::LookupError::NotFound => ActionError::DecisionHeadingNotFound {
            title: heading.to_string(),
        },
        crate::design::LookupError::Ambiguous { lines } => ActionError::AmbiguousDecisionHeading {
            title: heading.to_string(),
            lines,
        },
    })?;
    let block = &blocks[idx];

    // ─── Building the ADR ───
    let next = next_local_id(index);
    let slug = slug_from_title(&block.title);
    let filename = format!("{next}-{slug}.md");
    let new_path = layout.decisions_dir().join(&filename);

    // ADR body rendering: template with placeholders for Context,
    // Consequences, Alternatives; the verbatim block goes under ## Decision.
    let contents = render_promoted_adr(&next, &block.title, today, &block.body, change_name);

    // ─── Substituting the block in the design ───
    // We keep the `### Decision: <title>` heading line as it was and only
    // replace the body with the quote. The heading ends at the first `\n`
    // after byte_range.start.
    let title_line_end = design_source[block.byte_range.start..]
        .find('\n')
        .map(|off| block.byte_range.start + off + 1)
        .unwrap_or(design_source.len());
    let reference = build_promote_reference(&next, &slug);

    let mut new_design = String::with_capacity(design_source.len());
    new_design.push_str(&design_source[..title_line_end]);
    new_design.push_str(&reference);
    new_design.push_str(&design_source[block.byte_range.end..]);

    // ─── Atomic plan ───
    let mut plan = Plan::new();
    plan.dir(layout.decisions_dir());
    plan.write(new_path.clone(), contents, WriteMode::CreateOnly);
    plan.write(design_path.clone(), new_design, WriteMode::Overwrite);

    Ok(PromotePlan {
        plan,
        new_id: next,
        new_path,
        source_change: change_name.to_string(),
        design_path,
    })
}

/// Renders the full content of an ADR promoted from a `design.md` — the
/// block's body goes under `## Decision`, the other sections keep their
/// placeholder for manual redistribution.
fn render_promoted_adr(
    id: &str,
    title: &str,
    today: &str,
    verbatim_body: &str,
    source_change: &str,
) -> String {
    let frontmatter =
        render_frontmatter(id, title, &DecisionStatus::Proposed, today, &[], &[], &[]);
    // Clean verbatim body: strip leading newlines to avoid a useless
    // blank line at the start of the section, and guarantee a trailing
    // `\n`.
    let trimmed_body = verbatim_body.trim_start_matches('\n');
    let body_with_newline = if trimmed_body.ends_with('\n') {
        trimmed_body.to_string()
    } else {
        format!("{trimmed_body}\n")
    };

    format!(
        "{frontmatter}\n\n\
         <!-- ADR promoted from _codev/changes/{source_change}/design.md.\n     \
         Split the body below across Context, Decision, Consequences\n     \
         and Alternatives considered, then run `codev decision accept {id}`. -->\n\n\
         ## Context\n\n\
         <!-- The problem or situation that calls for a decision. Keep it short: two\n     \
         or three sentences are enough. -->\n\n\
         ## Decision\n\n\
         {body_with_newline}\n\
         ## Consequences\n\n\
         <!-- What this decision imposes or enables — good and bad. -->\n\n\
         ## Alternatives considered\n\n\
         <!-- What could have been done instead, and why something else was chosen. -->\n"
    )
}

/// Prepares a local deviation from an inherited decision.
///
/// Creates a local `proposed` ADR carrying `deviates_from: ["<qualified>"]`
/// — like `plan_new`, but with the reference to the target. The deviation
/// takes effect once `plan_accept` accepts and seals it: the index ignores
/// the `deviates_from` of a decision that is not `accepted`.
///
/// Explicit refusals:
/// - `EmptyTitle` if `new_title` is empty;
/// - `CannotDeviateFromLocal` if `target` is a `project/…` decision;
/// - `UnknownDecisionId` if `target` is not indexed;
/// - `AmbiguousDecisionId` if two sources expose the same qualified id.
pub fn plan_deviate(
    index: &DecisionIndex,
    target: &str,
    new_title: &str,
    today: &str,
    layout: &Layout,
) -> Result<DeviatePlan, ActionError> {
    if new_title.trim().is_empty() {
        return Err(ActionError::EmptyTitle);
    }

    // Resolve the target — same mechanics as supersede and seal.
    let (idx, _) = resolve_old_entry(index, target)?;
    let entry = &index.entries[idx];
    if entry.qualified_id.origin == Origin::Project {
        return Err(ActionError::CannotDeviateFromLocal {
            qualified_id: entry.qualified_id.as_str(),
        });
    }

    let qualified = entry.qualified_id.as_str();

    // New local number and ADR rendering — no seal for a proposed ADR.
    let next = next_local_id(index);
    let slug = slug_from_title(new_title);
    let filename = format!("{next}-{slug}.md");
    let new_path = layout.decisions_dir().join(&filename);

    let contents =
        render_new_deviate_adr(&next, new_title, today, std::slice::from_ref(&qualified));

    let mut plan = Plan::new();
    plan.dir(layout.decisions_dir());
    plan.write(new_path.clone(), contents, WriteMode::CreateOnly);

    Ok(DeviatePlan {
        plan,
        new_id: next,
        new_path,
        target_qualified_id: qualified,
    })
}

// ─────────────────────────── identifier resolution ───────────────────────────

/// Resolves an `old_id` identifier to an index entry.
///
/// Two accepted forms:
/// - short `<id>` (for example `0007`) — when the origin is unambiguous
/// - qualified `<origin>/<id>` (for example `path:~/shared/0100`)
pub fn resolve_old_entry(
    index: &DecisionIndex,
    old_id: &str,
) -> Result<(usize, bool), ActionError> {
    // Qualified form?
    if let Some((origin_raw, id)) = split_qualified(old_id) {
        let matching: Vec<usize> = index
            .entries
            .iter()
            .enumerate()
            .filter(|(_, e)| e.qualified_id.origin.as_str() == origin_raw && e.decision.id == id)
            .map(|(i, _)| i)
            .collect();
        return match matching.len() {
            1 => Ok((matching[0], true)),
            _ => Err(ActionError::UnknownDecisionId {
                id: old_id.to_string(),
            }),
        };
    }

    // Short form: look up by `decision.id`.
    let matching: Vec<usize> = index
        .entries
        .iter()
        .enumerate()
        .filter(|(_, e)| e.decision.id == old_id)
        .map(|(i, _)| i)
        .collect();
    match matching.len() {
        0 => Err(ActionError::UnknownDecisionId {
            id: old_id.to_string(),
        }),
        1 => Ok((matching[0], false)),
        _ => Err(ActionError::AmbiguousDecisionId {
            id: old_id.to_string(),
            candidates: matching
                .iter()
                .map(|&i| index.entries[i].qualified_id.as_str())
                .collect(),
        }),
    }
}

fn split_qualified(raw: &str) -> Option<(String, String)> {
    // A qualified identifier ends with `/<id>` — the origin may contain
    // `/` characters (`path:~/shared/xxx` does). So we take the last `/`
    // as the separator.
    let idx = raw.rfind('/')?;
    let origin_raw = &raw[..idx];
    let id = &raw[idx + 1..];
    if origin_raw.is_empty() || id.is_empty() {
        return None;
    }
    // A qualified origin always starts with `project`, `path:` or `git:` —
    // otherwise it is a short id containing a `/`, which is very unlikely,
    // but we would rather not confuse the two.
    if origin_raw != "project"
        && !origin_raw.starts_with("path:")
        && !origin_raw.starts_with("git:")
    {
        return None;
    }
    Some((origin_raw.to_string(), id.to_string()))
}

// ─────────────────────────── numbering & slug ───────────────────────────

/// `NNNN` — one more than the largest known numeric `id` **on the project
/// side only**. Inherited decisions live in their own space; we do not
/// interleave with them.
fn next_local_id(index: &DecisionIndex) -> String {
    let max_local = index
        .entries
        .iter()
        .filter(|e| e.qualified_id.origin == Origin::Project)
        .filter_map(|e| e.decision.id.parse::<u32>().ok())
        .max()
        .unwrap_or(0);
    format!("{:04}", max_local + 1)
}

/// ASCII kebab-case from a free-form title. A slug that is empty after
/// normalization becomes `decision` — the fallback documented in the design.
pub fn slug_from_title(title: &str) -> String {
    let mut out = String::with_capacity(title.len());
    let mut previous_dash = true;
    for c in title.chars() {
        let mapped = match c {
            'A'..='Z' => Some(c.to_ascii_lowercase()),
            'a'..='z' | '0'..='9' => Some(c),
            _ if c.is_ascii_whitespace() || matches!(c, '-' | '_') => Some('-'),
            _ => None,
        };
        if let Some(ch) = mapped {
            if ch == '-' {
                if !previous_dash {
                    out.push('-');
                    previous_dash = true;
                }
            } else {
                out.push(ch);
                previous_dash = false;
            }
        }
    }
    let trimmed = out.trim_matches('-').to_string();
    if trimmed.is_empty() {
        "decision".to_string()
    } else {
        trimmed
    }
}

// ─────────────────────────── ADR rendering ───────────────────────────

fn render_new_adr(
    id: &str,
    title: &str,
    status: &DecisionStatus,
    today: &str,
    supersedes: &[String],
) -> String {
    let frontmatter = render_frontmatter(id, title, status, today, supersedes, &[], &[]);
    DECISION_TEMPLATE.replace("{{FRONTMATTER}}", &frontmatter)
}

/// Renders a deviation ADR — same template body, `proposed` frontmatter
/// with `deviates_from`.
fn render_new_deviate_adr(id: &str, title: &str, today: &str, deviates_from: &[String]) -> String {
    let frontmatter = render_frontmatter(
        id,
        title,
        &DecisionStatus::Proposed,
        today,
        &[],
        &[],
        deviates_from,
    );
    DECISION_TEMPLATE.replace("{{FRONTMATTER}}", &frontmatter)
}

fn render_frontmatter(
    id: &str,
    title: &str,
    status: &DecisionStatus,
    date: &str,
    supersedes: &[String],
    tags: &[String],
    deviates_from: &[String],
) -> String {
    let mut out = String::from("---\n");
    out.push_str(&format!("id: \"{id}\"\n"));
    out.push_str(&format!("title: {}\n", yaml_scalar(title)));
    out.push_str(&format!("status: {}\n", status.as_str()));
    out.push_str(&format!("date: {date}\n"));
    if !tags.is_empty() {
        out.push_str(&format!(
            "tags: [{}]\n",
            tags.iter()
                .map(|t| yaml_scalar(t))
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    if !supersedes.is_empty() {
        out.push_str(&format!(
            "supersedes: [{}]\n",
            supersedes
                .iter()
                .map(|s| format!("\"{s}\""))
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    if !deviates_from.is_empty() {
        out.push_str(&format!(
            "deviates_from: [{}]\n",
            deviates_from
                .iter()
                .map(|s| format!("\"{s}\""))
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    out.push_str("---");
    out
}

/// Quotes a YAML scalar in double quotes; escapes the minimal set of
/// problematic characters.
fn yaml_scalar(raw: &str) -> String {
    let escaped = raw.replace('\\', "\\\\").replace('"', "\\\"");
    format!("\"{escaped}\"")
}

/// Rewrites the content of an existing ADR, changing only the frontmatter
/// `status`. The body — everything after the second `---` — is preserved
/// character for character.
pub fn rewrite_frontmatter_status(
    source: &str,
    decision: &codev_core::decisions::Decision,
    new_status: DecisionStatus,
) -> String {
    // The body is cut exactly where the seal cuts it, so that rewriting the
    // status can never change the sealed bytes — whatever the line endings.
    let body = codev_core::decisions::seal::body_slice(source)
        .unwrap_or(&source[decision.frontmatter_span.byte_range.end..]);
    // Keep the file's line-ending style: a CRLF file (a Windows checkout)
    // gets a CRLF frontmatter back.
    let newline = if source.starts_with("---\r\n") {
        "\r\n"
    } else {
        "\n"
    };
    let frontmatter = render_frontmatter(
        &decision.id,
        &decision.title,
        &new_status,
        &decision.date,
        &decision.supersedes,
        &decision.tags,
        &decision.deviates_from,
    );
    // `render_frontmatter` ends on the closing `---`, without its line
    // ending.
    let frontmatter = frontmatter.replace('\n', newline);
    format!("{frontmatter}{newline}{body}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config;
    use crate::decisions::index as build_index;
    use crate::ports::{FixedEnv, MemoryFileSystem};

    fn env() -> FixedEnv {
        let mut env = FixedEnv::at("/p");
        env.vars.insert("HOME".into(), "/home".into());
        env
    }

    #[test]
    fn rewriting_the_status_keeps_a_crlf_body_byte_for_byte() {
        let source = "---\r\nid: \"0001\"\r\ntitle: T\r\nstatus: proposed\r\ndate: 2026-09-08\r\n---\r\n\r\n## Context\r\n\r\nx\r\n";
        let decision = codev_core::decisions::parse_decision(source)
            .value
            .expect("a well-formed ADR");
        let rewritten = rewrite_frontmatter_status(source, &decision, DecisionStatus::Accepted);
        assert!(rewritten.contains("status: accepted\r\n"), "{rewritten:?}");
        assert_eq!(
            codev_core::decisions::seal::body_slice(&rewritten).unwrap(),
            codev_core::decisions::seal::body_slice(source).unwrap(),
        );
        assert!(
            !rewritten.contains("\n\n\r"),
            "no mixed blank line: {rewritten:?}"
        );
    }

    fn adr(id: &str, status: &str) -> String {
        format!(
            "---\nid: \"{id}\"\ntitle: ADR {id}\nstatus: {status}\ndate: 2026-09-08\n---\n\n## Context\n\nx\n"
        )
    }

    fn empty_index_with_config(fs: &MemoryFileSystem) -> DecisionIndex {
        let layout = Layout::new("/p");
        let cfg = config::resolve(fs, &env(), &layout).unwrap();
        build_index(fs, &env(), &layout, &cfg).unwrap()
    }

    #[test]
    fn skeleton_is_embedded() {
        assert!(DECISION_TEMPLATE.contains("{{FRONTMATTER}}"));
        assert!(DECISION_TEMPLATE.contains("## Context"));
        assert!(DECISION_TEMPLATE.contains("## Decision"));
        assert!(DECISION_TEMPLATE.contains("## Consequences"));
        assert!(DECISION_TEMPLATE.contains("## Alternatives considered"));
    }

    #[test]
    fn plan_new_in_empty_project_yields_0001() {
        let fs = MemoryFileSystem::new().with_file("/p/_codev/config.yaml", "");
        let index = empty_index_with_config(&fs);
        let plan = plan_new(
            &index,
            &SealFile::empty(),
            "A first choice",
            DecisionStatus::Accepted,
            "2026-09-08",
            &Layout::new("/p"),
        )
        .unwrap();
        assert_eq!(plan.new_id, "0001");
        assert_eq!(
            plan.new_path,
            PathBuf::from("/p/_codev/decisions/0001-a-first-choice.md")
        );
        // 2 writes: ADR (CreateOnly) + seal.yaml (Overwrite).
        assert_eq!(plan.plan.writes.len(), 2);
        let adr_write = plan
            .plan
            .writes
            .iter()
            .find(|w| w.mode == WriteMode::CreateOnly)
            .unwrap();
        assert_eq!(adr_write.mode, WriteMode::CreateOnly);
    }

    #[test]
    fn numbering_ignores_inherited_sources() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "inherits:\n  - path: ~/shared\n")
            .with_file("/p/_codev/decisions/0001-local.md", adr("0001", "accepted"))
            // Inherited source with a "larger" id — must have no influence.
            .with_file(
                "/home/shared/_codev/decisions/9999-from-shared.md",
                adr("9999", "accepted"),
            );
        let index = empty_index_with_config(&fs);
        let plan = plan_new(
            &index,
            &SealFile::empty(),
            "Next",
            DecisionStatus::Accepted,
            "2026-09-08",
            &Layout::new("/p"),
        )
        .unwrap();
        assert_eq!(plan.new_id, "0002");
    }

    #[test]
    fn slug_from_title_cases() {
        assert_eq!(slug_from_title("Normal title"), "normal-title");
        assert_eq!(slug_from_title("Ωmega : 🎉"), "mega");
        assert_eq!(slug_from_title("???"), "decision");
        assert_eq!(slug_from_title(""), "decision");
        assert_eq!(slug_from_title("  multiple spaces  "), "multiple-spaces");
        assert_eq!(slug_from_title("under_score"), "under-score");
        assert_eq!(slug_from_title("ID-Kebab-Case"), "id-kebab-case");
    }

    #[test]
    fn empty_title_is_refused() {
        let fs = MemoryFileSystem::new().with_file("/p/_codev/config.yaml", "");
        let index = empty_index_with_config(&fs);
        let err = plan_new(
            &index,
            &SealFile::empty(),
            "   ",
            DecisionStatus::Accepted,
            "2026-09-08",
            &Layout::new("/p"),
        )
        .unwrap_err();
        assert_eq!(err.code(), "empty_title");
    }

    #[test]
    fn plan_new_writes_adr_as_create_only_and_seal_as_overwrite() {
        // The ADR must never overwrite an existing file (CreateOnly),
        // but the seal is rewritten on every new decision (Overwrite).
        let fs = MemoryFileSystem::new().with_file("/p/_codev/config.yaml", "");
        let index = empty_index_with_config(&fs);
        let plan = plan_new(
            &index,
            &SealFile::empty(),
            "X",
            DecisionStatus::Accepted,
            "2026-09-08",
            &Layout::new("/p"),
        )
        .unwrap();
        // 2 writes: the ADR + seal.yaml.
        assert_eq!(plan.plan.writes.len(), 2);
        let create_only = plan
            .plan
            .writes
            .iter()
            .filter(|w| w.mode == WriteMode::CreateOnly)
            .count();
        assert_eq!(create_only, 1, "exactly one CreateOnly expected (the ADR)");
    }

    #[test]
    fn plan_new_with_proposed_status_does_not_seal() {
        // A `proposed` ADR carries no immutability commitment —
        // no seal.
        let fs = MemoryFileSystem::new().with_file("/p/_codev/config.yaml", "");
        let index = empty_index_with_config(&fs);
        let plan = plan_new(
            &index,
            &SealFile::empty(),
            "Tentative idea",
            DecisionStatus::Proposed,
            "2026-09-08",
            &Layout::new("/p"),
        )
        .unwrap();
        assert_eq!(plan.plan.writes.len(), 1, "no seal for proposed");
    }

    // ─────────────── supersede ───────────────

    fn supersede(fs: &MemoryFileSystem, old_id: &str) -> Result<SupersedePlan, ActionError> {
        let index = empty_index_with_config(fs);
        plan_supersede(
            &index,
            old_id,
            "New choice",
            "2026-09-08",
            &Layout::new("/p"),
        )
    }

    #[test]
    fn plan_supersede_creates_a_proposed_adr_and_leaves_the_predecessor_untouched() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file("/p/_codev/decisions/0003-old.md", adr("0003", "accepted"));
        let plan = supersede(&fs, "0003").unwrap();

        assert_eq!(plan.new_id, "0004");
        assert_eq!(plan.old_qualified_id, "project/0003");
        assert_eq!(
            plan.old_path,
            PathBuf::from("/p/_codev/decisions/0003-old.md")
        );
        // A single write: the new ADR. The predecessor is only rewritten
        // when the new one is accepted.
        assert_eq!(plan.plan.writes.len(), 1);
        let new_write = &plan.plan.writes[0];
        assert_eq!(new_write.mode, WriteMode::CreateOnly);
        assert_eq!(new_write.path, plan.new_path);
        assert!(new_write.contents.contains("supersedes: [\"0003\"]"));
        assert!(new_write.contents.contains("id: \"0004\""));
        assert!(new_write.contents.contains("status: proposed"));
    }

    #[test]
    fn plan_supersede_writes_no_seal() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file("/p/_codev/decisions/0003-old.md", adr("0003", "accepted"));
        let plan = supersede(&fs, "0003").unwrap();
        assert!(
            !plan
                .plan
                .writes
                .iter()
                .any(|w| w.path == Layout::new("/p").decisions_seal_file()),
            "a proposed ADR is not sealed"
        );
    }

    #[test]
    fn plan_supersede_refuses_a_predecessor_that_is_not_accepted() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file("/p/_codev/decisions/0004-x.md", adr("0004", "proposed"));
        let err = supersede(&fs, "0004").unwrap_err();
        assert_eq!(err.code(), "predecessor_not_accepted");
        assert!(err.to_string().contains("`proposed`"), "{err}");
    }

    #[test]
    fn supersede_unknown_id_is_refused() {
        let fs = MemoryFileSystem::new().with_file("/p/_codev/config.yaml", "");
        let err = supersede(&fs, "9999").unwrap_err();
        assert_eq!(err.code(), "unknown_decision_id");
    }

    #[test]
    fn supersede_inherited_source_is_refused() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "inherits:\n  - path: ~/shared\n")
            .with_file(
                "/home/shared/_codev/decisions/0100-shared.md",
                adr("0100", "accepted"),
            );
        let err = supersede(&fs, "path:~/shared/0100").unwrap_err();
        assert_eq!(err.code(), "cannot_supersede_inherited");
        assert!(err.to_string().contains("deviate"));
    }

    #[test]
    fn supersede_frontmatter_keeps_original_fields() {
        // Build an explicit source with existing tags and supersedes, and
        // check that the rewrite keeps all of them.
        let source = "---\nid: \"0003\"\ntitle: \"Old title\"\nstatus: accepted\ndate: 2025-01-01\ntags: [\"a\", \"b\"]\nsupersedes: [\"0001\"]\n---\n\n## Body\n\nunchanged\n";
        let parsed = codev_core::decisions::parse_decision(source);
        let decision = parsed.value.expect("valid decision");
        let out = rewrite_frontmatter_status(source, &decision, DecisionStatus::Superseded);

        assert!(out.contains("status: superseded"));
        assert!(out.contains("id: \"0003\""));
        assert!(out.contains("title: \"Old title\""));
        assert!(out.contains("date: 2025-01-01"));
        assert!(out.contains("tags: [\"a\", \"b\"]"));
        assert!(out.contains("supersedes: [\"0001\"]"));
        // Body preserved character for character.
        assert!(out.ends_with("## Body\n\nunchanged\n"));
    }

    #[test]
    fn plan_supersede_includes_the_new_decision() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file("/p/_codev/decisions/0003-old.md", adr("0003", "accepted"));
        let plan = supersede(&fs, "0003").unwrap();
        assert_eq!(
            plan.new_path,
            PathBuf::from("/p/_codev/decisions/0004-new-choice.md")
        );
    }

    // ─────────────── plan_seal ───────────────

    #[test]
    fn plan_seal_fresh_adds_the_entry() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file("/p/_codev/decisions/0001-x.md", adr("0001", "accepted"));
        let index = empty_index_with_config(&fs);
        let plan = plan_seal(
            &index,
            &SealFile::empty(),
            "0001",
            false,
            "2026-09-09",
            &Layout::new("/p"),
            |path| Ok(fs.read(path).unwrap()),
        )
        .unwrap();
        assert!(!plan.was_noop);
        assert_eq!(plan.id, "0001");
        assert!(plan.body_sha256.starts_with("sha256:"));
        assert_eq!(plan.plan.writes.len(), 1);
    }

    #[test]
    fn plan_seal_is_noop_when_hash_already_matches() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file("/p/_codev/decisions/0001-x.md", adr("0001", "accepted"));
        let index = empty_index_with_config(&fs);
        let source = fs
            .read(std::path::Path::new("/p/_codev/decisions/0001-x.md"))
            .unwrap();
        let hash = seal::body_hash(&source).unwrap();
        let mut existing = SealFile::empty();
        existing.seals.push(codev_core::decisions::seal::Seal {
            id: "0001".into(),
            body_sha256: hash,
            sealed_at: "2026-09-08".into(),
        });
        let plan = plan_seal(
            &index,
            &existing,
            "0001",
            false,
            "2026-09-09",
            &Layout::new("/p"),
            |path| Ok(fs.read(path).unwrap()),
        )
        .unwrap();
        assert!(plan.was_noop);
        assert!(plan.plan.writes.is_empty(), "no-op → no writes");
    }

    #[test]
    fn plan_seal_refuses_without_force_when_body_changed() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file("/p/_codev/decisions/0001-x.md", adr("0001", "accepted"));
        let index = empty_index_with_config(&fs);
        let mut existing = SealFile::empty();
        existing.seals.push(codev_core::decisions::seal::Seal {
            id: "0001".into(),
            body_sha256: "sha256:an-old-hash".into(),
            sealed_at: "2026-09-08".into(),
        });
        let err = plan_seal(
            &index,
            &existing,
            "0001",
            false,
            "2026-09-09",
            &Layout::new("/p"),
            |path| Ok(fs.read(path).unwrap()),
        )
        .unwrap_err();
        assert_eq!(err.code(), "seal_conflict");
    }

    #[test]
    fn plan_seal_with_force_rewrites_the_entry() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file("/p/_codev/decisions/0001-x.md", adr("0001", "accepted"));
        let index = empty_index_with_config(&fs);
        let mut existing = SealFile::empty();
        existing.seals.push(codev_core::decisions::seal::Seal {
            id: "0001".into(),
            body_sha256: "sha256:an-old-hash".into(),
            sealed_at: "2026-09-08".into(),
        });
        let plan = plan_seal(
            &index,
            &existing,
            "0001",
            true,
            "2026-09-09",
            &Layout::new("/p"),
            |path| Ok(fs.read(path).unwrap()),
        )
        .unwrap();
        assert!(!plan.was_noop);
        assert!(plan.body_sha256.starts_with("sha256:"));
        assert_ne!(plan.body_sha256, "sha256:an-old-hash");
        assert_eq!(plan.plan.writes.len(), 1);
    }

    #[test]
    fn plan_seal_refuses_an_inherited_decision() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "inherits:\n  - path: ~/shared\n")
            .with_file(
                "/home/shared/_codev/decisions/0100-shared.md",
                adr("0100", "accepted"),
            );
        let index = empty_index_with_config(&fs);
        let err = plan_seal(
            &index,
            &SealFile::empty(),
            "path:~/shared/0100",
            false,
            "2026-09-09",
            &Layout::new("/p"),
            |path| Ok(fs.read(path).unwrap()),
        )
        .unwrap_err();
        assert_eq!(err.code(), "cannot_seal_inherited");
    }

    // ─────────────── plan_accept ───────────────

    fn accept(fs: &MemoryFileSystem, seal: &SealFile, id: &str) -> Result<AcceptPlan, ActionError> {
        let index = empty_index_with_config(fs);
        plan_accept(&index, seal, id, "2026-09-30", &Layout::new("/p"), |path| {
            Ok(fs.read(path).unwrap())
        })
    }

    #[test]
    fn plan_accept_rewrites_the_status_and_seals_in_one_plan() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file("/p/_codev/decisions/0007-x.md", adr("0007", "proposed"));
        let plan = accept(&fs, &SealFile::empty(), "0007").unwrap();

        assert_eq!(plan.id, "0007");
        assert_eq!(plan.path, PathBuf::from("/p/_codev/decisions/0007-x.md"));
        // Both writes live in the same plan: the ADR and seal.yaml.
        assert_eq!(plan.plan.writes.len(), 2);
        let adr_write = plan
            .plan
            .writes
            .iter()
            .find(|w| w.path == plan.path)
            .expect("the ADR is rewritten");
        assert!(adr_write.contents.contains("status: accepted"));
        let seal_write = plan
            .plan
            .writes
            .iter()
            .find(|w| w.path == std::path::Path::new("/p/_codev/decisions/seal.yaml"))
            .expect("the seal is written");
        let seal_file = seal::parse_seal_file(&seal_write.contents).unwrap();
        let entry = seal_file.find("0007").expect("entry for 0007");
        assert_eq!(entry.body_sha256, plan.body_sha256);
        assert_eq!(entry.sealed_at, "2026-09-30");
    }

    #[test]
    fn plan_accept_keeps_the_body_byte_for_byte() {
        let source = "---\nid: \"0007\"\ntitle: T\nstatus: proposed\ndate: 2026-09-08\n---\n\n## Context\n\nWritten by hand — “quotes” ✓.\n";
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file("/p/_codev/decisions/0007-t.md", source);
        let plan = accept(&fs, &SealFile::empty(), "0007").unwrap();
        let adr_write = plan
            .plan
            .writes
            .iter()
            .find(|w| w.path == plan.path)
            .unwrap();
        assert_eq!(
            seal::body_slice(&adr_write.contents).unwrap(),
            seal::body_slice(source).unwrap(),
        );
        // The recorded hash is the hash of the body the user wrote.
        assert_eq!(plan.body_sha256, seal::body_hash(source).unwrap());
    }

    #[test]
    fn plan_accept_refuses_an_unknown_id() {
        let fs = MemoryFileSystem::new().with_file("/p/_codev/config.yaml", "");
        let err = accept(&fs, &SealFile::empty(), "0042").unwrap_err();
        assert_eq!(err.code(), "unknown_decision_id");
    }

    #[test]
    fn plan_accept_refuses_an_inherited_decision() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "inherits:\n  - path: ~/shared\n")
            .with_file(
                "/home/shared/_codev/decisions/0100-shared.md",
                adr("0100", "proposed"),
            );
        let err = accept(&fs, &SealFile::empty(), "path:~/shared/0100").unwrap_err();
        assert_eq!(err.code(), "cannot_accept_inherited");
    }

    #[test]
    fn plan_accept_refuses_an_accepted_decision_and_points_to_supersede() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file("/p/_codev/decisions/0003-x.md", adr("0003", "accepted"));
        let err = accept(&fs, &SealFile::empty(), "0003").unwrap_err();
        assert_eq!(err.code(), "decision_not_proposed");
        assert!(err.to_string().contains("already accepted"));
        assert!(err.to_string().contains("codev decision supersede 0003"));
    }

    #[test]
    fn plan_accept_refuses_a_rejected_decision() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file("/p/_codev/decisions/0004-x.md", adr("0004", "rejected"));
        let err = accept(&fs, &SealFile::empty(), "0004").unwrap_err();
        assert_eq!(err.code(), "decision_not_proposed");
        assert!(err.to_string().contains("`rejected`"));
    }

    #[test]
    fn plan_accept_refuses_a_leftover_seal_for_another_body() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file("/p/_codev/decisions/0007-x.md", adr("0007", "proposed"));
        let mut existing = SealFile::empty();
        existing.seals.push(codev_core::decisions::seal::Seal {
            id: "0007".into(),
            body_sha256: "sha256:another-body".into(),
            sealed_at: "2026-09-08".into(),
        });
        let err = accept(&fs, &existing, "0007").unwrap_err();
        assert_eq!(err.code(), "seal_conflict");
    }

    /// A `proposed` ADR `0007` superseding each of `supersedes`.
    fn proposed_successor(supersedes: &[&str]) -> String {
        let list = supersedes
            .iter()
            .map(|s| format!("\"{s}\""))
            .collect::<Vec<_>>()
            .join(", ");
        format!(
            "---\nid: \"0007\"\ntitle: New\nstatus: proposed\ndate: 2026-09-30\nsupersedes: [{list}]\n---\n\n## Context\n\nThe new one.\n"
        )
    }

    #[test]
    fn plan_accept_marks_the_predecessor_superseded_in_the_same_plan() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file("/p/_codev/decisions/0003-old.md", adr("0003", "accepted"))
            .with_file(
                "/p/_codev/decisions/0007-new.md",
                proposed_successor(&["0003"]),
            );
        let plan = accept(&fs, &SealFile::empty(), "0007").unwrap();

        // Three writes in one plan: the new ADR, seal.yaml, the predecessor.
        assert_eq!(plan.plan.writes.len(), 3);
        let old_path = PathBuf::from("/p/_codev/decisions/0003-old.md");
        let old_write = plan
            .plan
            .writes
            .iter()
            .find(|w| w.path == old_path)
            .expect("the predecessor is rewritten");
        assert_eq!(old_write.mode, WriteMode::Overwrite);
        assert!(old_write.contents.contains("status: superseded"));
        assert_eq!(
            plan.superseded,
            vec![SupersededPredecessor {
                id: "0003".into(),
                qualified_id: "project/0003".into(),
                path: old_path,
            }]
        );
        // Only the new decision gets a seal entry.
        let seal_write = plan
            .plan
            .writes
            .iter()
            .find(|w| w.path == Layout::new("/p").decisions_seal_file())
            .unwrap();
        let seal_file = seal::parse_seal_file(&seal_write.contents).unwrap();
        assert!(seal_file.find("0007").is_some());
        assert!(seal_file.find("0003").is_none());
    }

    #[test]
    fn plan_accept_keeps_the_predecessor_body_byte_for_byte() {
        // Golden: after acceptance, everything after the predecessor's
        // frontmatter stays byte-for-byte identical, so its seal holds.
        let source = "---\nid: \"0003\"\ntitle: T\nstatus: accepted\ndate: 2026-09-08\n---\n\n## Context\n\nContent with `special` characters — “curly quotes” ✓.\n\n## Decision\n\nOK.\n";
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file("/p/_codev/decisions/0003.md", source)
            .with_file(
                "/p/_codev/decisions/0007-new.md",
                proposed_successor(&["0003"]),
            );
        let mut existing = SealFile::empty();
        existing.seals.push(codev_core::decisions::seal::Seal {
            id: "0003".into(),
            body_sha256: seal::body_hash(source).unwrap(),
            sealed_at: "2026-09-08".into(),
        });
        let plan = accept(&fs, &existing, "0007").unwrap();

        let old_write = plan
            .plan
            .writes
            .iter()
            .find(|w| w.path == std::path::Path::new("/p/_codev/decisions/0003.md"))
            .unwrap();
        assert_eq!(
            seal::body_slice(&old_write.contents).unwrap(),
            seal::body_slice(source).unwrap(),
            "body identical character for character"
        );
        let seal_write = plan
            .plan
            .writes
            .iter()
            .find(|w| w.path == Layout::new("/p").decisions_seal_file())
            .unwrap();
        let seal_file = seal::parse_seal_file(&seal_write.contents).unwrap();
        assert_eq!(
            seal_file.find("0003"),
            existing.find("0003"),
            "the predecessor's seal entry is untouched"
        );
    }

    #[test]
    fn plan_accept_refuses_a_predecessor_that_is_no_longer_accepted() {
        // 0008 superseded 0003 in the meantime: accepting 0007, which also
        // supersedes 0003, would fork the chain.
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file("/p/_codev/decisions/0003-old.md", adr("0003", "superseded"))
            .with_file("/p/_codev/decisions/0007-new.md", proposed_successor(&["0003"]))
            .with_file(
                "/p/_codev/decisions/0008-other.md",
                "---\nid: \"0008\"\ntitle: Other\nstatus: accepted\ndate: 2026-09-30\nsupersedes: [\"0003\"]\n---\n\nx\n",
            );
        let err = accept(&fs, &SealFile::empty(), "0007").unwrap_err();
        assert_eq!(err.code(), "predecessor_not_accepted");
        let message = err.to_string();
        assert!(message.contains("`0003`"), "{message}");
        assert!(message.contains("`superseded`"), "{message}");
        assert!(message.contains("project/0008"), "{message}");
    }

    #[test]
    fn plan_accept_refuses_an_inherited_predecessor() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "inherits:\n  - path: ~/shared\n")
            .with_file(
                "/home/shared/_codev/decisions/0100-shared.md",
                adr("0100", "accepted"),
            )
            .with_file(
                "/p/_codev/decisions/0007-new.md",
                proposed_successor(&["0100"]),
            );
        let err = accept(&fs, &SealFile::empty(), "0007").unwrap_err();
        assert_eq!(err.code(), "cannot_supersede_inherited");
    }

    #[test]
    fn plan_accept_refuses_an_unknown_predecessor() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file(
                "/p/_codev/decisions/0007-new.md",
                proposed_successor(&["9999"]),
            );
        let err = accept(&fs, &SealFile::empty(), "0007").unwrap_err();
        assert_eq!(err.code(), "unknown_decision_id");
        assert!(err.to_string().contains("9999"));
    }

    // ─────────────── plan_deviate ───────────────

    #[test]
    fn plan_deviate_creates_a_proposed_adr_without_seal() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "inherits:\n  - path: ~/shared\n")
            .with_file(
                "/home/shared/_codev/decisions/0100-shared.md",
                adr("0100", "accepted"),
            );
        let index = empty_index_with_config(&fs);
        let plan = plan_deviate(
            &index,
            "path:~/shared/0100",
            "Our local alternative",
            "2026-09-09",
            &Layout::new("/p"),
        )
        .unwrap();

        assert_eq!(plan.new_id, "0001");
        assert_eq!(plan.target_qualified_id, "path:~/shared/0100");
        assert_eq!(plan.plan.writes.len(), 1, "the ADR only, no seal");

        let adr_write = plan
            .plan
            .writes
            .iter()
            .find(|w| w.mode == WriteMode::CreateOnly)
            .unwrap();
        assert!(
            adr_write
                .contents
                .contains("deviates_from: [\"path:~/shared/0100\"]")
        );
        assert!(adr_write.contents.contains("status: proposed"));
        assert!(adr_write.contents.contains("id: \"0001\""));
    }

    #[test]
    fn plan_deviate_refuses_a_local_target() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file("/p/_codev/decisions/0003-local.md", adr("0003", "accepted"));
        let index = empty_index_with_config(&fs);
        let err = plan_deviate(
            &index,
            "project/0003",
            "…",
            "2026-09-09",
            &Layout::new("/p"),
        )
        .unwrap_err();
        assert_eq!(err.code(), "cannot_deviate_from_local");
        assert!(err.to_string().contains("supersede"));
    }

    #[test]
    fn plan_deviate_refuses_an_unknown_target() {
        let fs = MemoryFileSystem::new().with_file("/p/_codev/config.yaml", "");
        let index = empty_index_with_config(&fs);
        let err = plan_deviate(
            &index,
            "path:~/unknown/0100",
            "…",
            "2026-09-09",
            &Layout::new("/p"),
        )
        .unwrap_err();
        assert_eq!(err.code(), "unknown_decision_id");
    }

    #[test]
    fn plan_deviate_refuses_empty_title() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "inherits:\n  - path: ~/shared\n")
            .with_file(
                "/home/shared/_codev/decisions/0100-p.md",
                adr("0100", "accepted"),
            );
        let index = empty_index_with_config(&fs);
        let err = plan_deviate(
            &index,
            "path:~/shared/0100",
            "   ",
            "2026-09-09",
            &Layout::new("/p"),
        )
        .unwrap_err();
        assert_eq!(err.code(), "empty_title");
    }

    #[test]
    fn plan_deviate_renders_frontmatter_with_deviates_from() {
        // Exact frontmatter contract — the tests above already check the
        // content; here we golden the precise YAML format so that a
        // refactor does not break the rendered line.
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "inherits:\n  - path: ~/shared\n")
            .with_file(
                "/home/shared/_codev/decisions/0100-p.md",
                adr("0100", "accepted"),
            );
        let index = empty_index_with_config(&fs);
        let plan = plan_deviate(
            &index,
            "path:~/shared/0100",
            "Local choice",
            "2026-09-09",
            &Layout::new("/p"),
        )
        .unwrap();
        let adr_content = &plan
            .plan
            .writes
            .iter()
            .find(|w| w.mode == WriteMode::CreateOnly)
            .unwrap()
            .contents;
        // Exact line: `deviates_from: ["<qualified>"]`.
        assert!(adr_content.contains("\ndeviates_from: [\"path:~/shared/0100\"]\n"));
    }

    // ─────────────── plan_promote ───────────────

    const DESIGN_WITH_ONE_BLOCK: &str = "\
# Design: x

## Context

See proposal.

## Decisions

### Decision: Use JWT

The rationale for the choice.

Second paragraph.

## Risks

y
";

    #[test]
    fn plan_promote_produces_adr_and_references_design() {
        let fs = MemoryFileSystem::new().with_file("/p/_codev/config.yaml", "");
        let index = empty_index_with_config(&fs);
        let plan = plan_promote(
            &index,
            "add-auth",
            "Use JWT",
            DESIGN_WITH_ONE_BLOCK,
            PathBuf::from("/p/_codev/changes/add-auth/design.md"),
            "2026-09-09",
            &Layout::new("/p"),
        )
        .unwrap();

        assert_eq!(plan.new_id, "0001");
        assert_eq!(
            plan.new_path,
            PathBuf::from("/p/_codev/decisions/0001-use-jwt.md")
        );
        // 2 writes: ADR + design.md — no seal for a proposed ADR.
        assert_eq!(plan.plan.writes.len(), 2);

        // The new ADR does contain the verbatim body.
        let adr = plan
            .plan
            .writes
            .iter()
            .find(|w| w.mode == WriteMode::CreateOnly)
            .unwrap();
        assert!(adr.contents.contains("The rationale for the choice."));
        assert!(adr.contents.contains("Second paragraph."));
        assert!(adr.contents.contains("<!-- ADR promoted from"));
        assert!(adr.contents.contains("_codev/changes/add-auth/design.md"));

        // The rewritten design.md contains the textual reference.
        let new_design = plan
            .plan
            .writes
            .iter()
            .find(|w| w.path == std::path::Path::new("/p/_codev/changes/add-auth/design.md"))
            .unwrap();
        assert!(
            new_design
                .contents
                .contains("### Decision: Use JWT\n\n> Promoted to ADR **0001**")
        );
        // The original body is gone.
        assert!(
            !new_design
                .contents
                .contains("The rationale for the choice.")
        );
        // The other sections (Context, Risks) are preserved.
        assert!(new_design.contents.contains("## Risks\n\ny\n"));
    }

    #[test]
    fn plan_promote_refuses_missing_title() {
        let fs = MemoryFileSystem::new().with_file("/p/_codev/config.yaml", "");
        let index = empty_index_with_config(&fs);
        let err = plan_promote(
            &index,
            "add-auth",
            "Ghost title",
            DESIGN_WITH_ONE_BLOCK,
            PathBuf::from("/p/_codev/changes/add-auth/design.md"),
            "2026-09-09",
            &Layout::new("/p"),
        )
        .unwrap_err();
        assert_eq!(err.code(), "decision_heading_not_found");
    }

    #[test]
    fn plan_promote_refuses_ambiguous_title() {
        let ambiguous_design = "\
## Decisions

### Decision: X

v1

### Decision: X

v2

## End
";
        let fs = MemoryFileSystem::new().with_file("/p/_codev/config.yaml", "");
        let index = empty_index_with_config(&fs);
        let err = plan_promote(
            &index,
            "add-auth",
            "X",
            ambiguous_design,
            PathBuf::from("/p/_codev/changes/add-auth/design.md"),
            "2026-09-09",
            &Layout::new("/p"),
        )
        .unwrap_err();
        assert_eq!(err.code(), "ambiguous_decision_heading");
        assert!(err.to_string().contains("multiple blocks"));
    }

    #[test]
    fn plan_promote_refuses_empty_title() {
        let fs = MemoryFileSystem::new().with_file("/p/_codev/config.yaml", "");
        let index = empty_index_with_config(&fs);
        let err = plan_promote(
            &index,
            "add-auth",
            "   ",
            DESIGN_WITH_ONE_BLOCK,
            PathBuf::from("/p/_codev/changes/add-auth/design.md"),
            "2026-09-09",
            &Layout::new("/p"),
        )
        .unwrap_err();
        assert_eq!(err.code(), "empty_title");
    }

    #[test]
    fn plan_promote_creates_a_proposed_unsealed_adr() {
        // The promoted body is meant to be reworked: the ADR is proposed,
        // nothing is sealed, and the ADR says how to accept it.
        let fs = MemoryFileSystem::new().with_file("/p/_codev/config.yaml", "");
        let index = empty_index_with_config(&fs);
        let plan = plan_promote(
            &index,
            "add-auth",
            "Use JWT",
            DESIGN_WITH_ONE_BLOCK,
            PathBuf::from("/p/_codev/changes/add-auth/design.md"),
            "2026-09-09",
            &Layout::new("/p"),
        )
        .unwrap();

        assert!(
            !plan
                .plan
                .writes
                .iter()
                .any(|w| w.path == Layout::new("/p").decisions_seal_file()),
            "no seal for a proposed ADR"
        );
        let adr = plan
            .plan
            .writes
            .iter()
            .find(|w| w.path == plan.new_path)
            .unwrap();
        assert!(adr.contents.contains("status: proposed"));
        assert!(adr.contents.contains("`codev decision accept 0001`"));
    }
}
