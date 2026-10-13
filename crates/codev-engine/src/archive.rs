//! `archive`: a sync preflighted by the validator + a chronological move of
//! the change directory.
//!
//! `archive` delegates all rule logic to the validator
//! (`codev-engine::validate::validate_change`). No error code is
//! duplicated here; on an error finding, pointing to `codev validate`
//! is enough. Then the review gate (`codev-engine::review`): a settled
//! review is a condition of archiving, not of validity.

use std::path::PathBuf;

use codev_core::{ChangeId, Layout, Plan};

use crate::apply;
use crate::change;
use crate::config::ResolvedConfig;
use crate::error::{EngineError, Result};
use crate::ports::{Clock, FileSystem};
use crate::review;
use crate::sync::{self, SyncPlan};

/// Full plan of an `archive`: the sync and the final move.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchivePlan {
    pub sync: SyncPlan,
    pub archive_dir: PathBuf,
    /// The combined `Plan` — sync.plan + final move.
    pub plan: Plan,
}

/// What the archive actually did.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct ArchiveOutcome {
    pub change: String,
    pub root: PathBuf,
    pub created: Vec<PathBuf>,
    pub updated: Vec<PathBuf>,
    pub unchanged: Vec<PathBuf>,
    /// Main specs deleted by the change (retired capabilities). Empty in the common case.
    pub deleted: Vec<PathBuf>,
    pub moved_to: PathBuf,
}

/// Builds the plan without writing. Checks with `validate` first (through
/// [`sync::plan_sync`]) — if the change has any error at all, it refuses
/// without even preparing the plan.
pub fn plan_archive(
    fs: &dyn FileSystem,
    layout: &Layout,
    config: &ResolvedConfig,
    clock: &dyn Clock,
    change_id: &ChangeId,
) -> Result<ArchivePlan> {
    // Preflight: `plan_sync` runs the validator first — an error finding
    // is a refusal before the plan is even prepared.
    let sync_plan = sync::plan_sync(fs, layout, config, change_id)?;

    // The review gate comes after the validation pre-flight: a broken
    // record is a validation error first, pointing to `codev validate`.
    let ctx = change::load(fs, layout, config, change_id.clone())?;
    let blockers = review::check(fs, layout, config, &ctx)?.blockers;
    if !blockers.is_empty() {
        return Err(EngineError::ReviewBlocked {
            change: change_id.to_string(),
            reasons: blockers
                .into_iter()
                .map(|b| format!("{} ({})", b.message, b.code))
                .collect(),
        });
    }

    // Move target: `changes/archive/<date>-<name>/`. The date prefix
    // gives a chronological ordering on disk.
    let archive_dir = layout.archived_change_dir(change_id, &clock.today());

    let mut combined = sync_plan.plan.clone();
    if let Some(parent) = archive_dir.parent() {
        combined.dir(parent.to_path_buf());
    }
    combined.move_dir(layout.change_dir(change_id), archive_dir.clone());

    Ok(ArchivePlan {
        sync: sync_plan,
        archive_dir,
        plan: combined,
    })
}

/// Executes the archive plan.
pub fn execute_archive(
    fs: &dyn FileSystem,
    layout: &Layout,
    config: &ResolvedConfig,
    clock: &dyn Clock,
    change_id: &ChangeId,
) -> Result<ArchiveOutcome> {
    let archive_plan = plan_archive(fs, layout, config, clock, change_id)?;
    let applied = apply::execute(&archive_plan.plan, fs)?;

    // The sync told creates and updates apart at planning time; we take them
    // from the sync_plan lists (rather than guessing from `applied.created`,
    // which would also include the planned dirs that we do not want to
    // count as "created files").
    let mut outcome = ArchiveOutcome {
        change: change_id.to_string(),
        root: layout.project_root().to_path_buf(),
        created: archive_plan.sync.creates.clone(),
        updated: archive_plan.sync.updates.clone(),
        unchanged: archive_plan.sync.unchanged.clone(),
        deleted: archive_plan.sync.deleted.clone(),
        moved_to: archive_plan.archive_dir.clone(),
    };
    // Sanity check: the move did happen.
    let expected_source = layout.change_dir(change_id);
    if !applied
        .moved
        .iter()
        .any(|(from, _)| from == &expected_source)
    {
        return Err(EngineError::Invalid {
            path: layout.change_dir(change_id),
            reason: "the change was not moved".into(),
        });
    }
    outcome.unchanged.extend(applied.untouched);
    outcome.unchanged.sort();
    outcome.unchanged.dedup();
    Ok(outcome)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config;
    use crate::ports::{FixedClock, FixedEnv, MemoryFileSystem};
    use crate::review::tests::{NEWCOMER_NO_FINDING, finding, record};

    fn env() -> FixedEnv {
        let mut env = FixedEnv::at("/p");
        env.vars.insert("HOME".into(), "/home".into());
        env
    }

    fn resolved(fs: &dyn FileSystem) -> ResolvedConfig {
        config::resolve(fs, &env(), &Layout::new("/p")).unwrap()
    }

    fn well_formed_project() -> MemoryFileSystem {
        MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file(
                "/p/_codev/changes/add-auth/change.yaml",
                "schema: spec-driven",
            )
            .with_file(
                "/p/_codev/changes/add-auth/specs/user-auth/spec.md",
                "## Purpose\n\nHandles authentication.\n\n## ADDED Requirements\n\n### Requirement: Login\nThe system SHALL emit a token.\n\n#### Scenario: OK\n- **WHEN** login\n- **THEN** token\n",
            )
            .with_file(
                "/p/_codev/changes/add-auth/review.md",
                record(NEWCOMER_NO_FINDING),
            )
    }

    fn archive(fs: &MemoryFileSystem) -> Result<ArchiveOutcome> {
        let layout = Layout::new("/p");
        let cfg = resolved(fs);
        execute_archive(
            fs,
            &layout,
            &cfg,
            &FixedClock("2026-09-08".into()),
            &ChangeId::parse("add-auth").unwrap(),
        )
    }

    fn still_active(fs: &MemoryFileSystem) -> bool {
        fs.read("/p/_codev/changes/add-auth/change.yaml").is_some()
    }

    #[test]
    fn archive_refuses_an_open_high_finding() {
        let fs = well_formed_project()
            .with_file("/p/src/auth.rs", "let token = random();\n")
            .with_file(
                "/p/_codev/changes/add-auth/review.md",
                record(&finding(
                    "newcomer",
                    "high",
                    "src/auth.rs:1",
                    "random()",
                    "",
                )),
            );
        let err = archive(&fs).unwrap_err();
        assert_eq!(err.code(), "review_blocked");
        let message = err.to_string();
        assert!(message.contains("R-newcomer-1"), "{message}");
        assert!(
            message.contains("codev review --change add-auth"),
            "{message}"
        );
        assert!(still_active(&fs));
        assert!(fs.read("/p/_codev/specs/user-auth/spec.md").is_none());
    }

    #[test]
    fn archive_accepts_an_accepted_risk() {
        let fs = well_formed_project()
            .with_file("/p/src/auth.rs", "let token = random();\n")
            .with_file(
                "/p/_codev/changes/add-auth/review.md",
                record(&finding(
                    "newcomer",
                    "high",
                    "src/auth.rs:1",
                    "random()",
                    "  - status: accepted-risk\n    by: human:Ludovic\n    note: a test fixture, never deployed\n",
                )),
            );
        let outcome = archive(&fs).unwrap();
        assert!(
            fs.read(outcome.moved_to.join("review.md"))
                .is_some_and(|r| r.contains("accepted-risk"))
        );
    }

    #[test]
    fn archive_refuses_without_a_record() {
        let fs = well_formed_project();
        fs.remove_file(std::path::Path::new("/p/_codev/changes/add-auth/review.md"))
            .unwrap();
        let err = archive(&fs).unwrap_err();
        assert_eq!(err.code(), "review_blocked");
        assert!(err.to_string().contains("review_record_missing"), "{err}");
        assert!(still_active(&fs));
        // A record without the code review step.
        let plan_only = record("")
            .split("## code.review")
            .next()
            .unwrap()
            .to_string();
        let fs = well_formed_project().with_file("/p/_codev/changes/add-auth/review.md", plan_only);
        let err = archive(&fs).unwrap_err();
        assert!(err.to_string().contains("review_step_missing"), "{err}");
    }

    #[test]
    fn archive_without_active_review_roles_needs_no_record() {
        let fs = well_formed_project().with_file(
            "/p/_codev/changes/add-auth/change.yaml",
            "schema: spec-driven\nroles:\n  skeptic: never\n  newcomer: never\n",
        );
        fs.remove_file(std::path::Path::new("/p/_codev/changes/add-auth/review.md"))
            .unwrap();
        archive(&fs).unwrap();
        assert!(!still_active(&fs));
    }

    #[test]
    fn archive_refuses_a_newer_format() {
        let fs = well_formed_project().with_file(
            "/p/_codev/changes/add-auth/review.md",
            "<!-- codev-review: 2 -->\n",
        );
        let err = archive(&fs).unwrap_err();
        assert_eq!(err.code(), "review_blocked");
        assert!(err.to_string().contains("update codev"), "{err}");
    }

    #[test]
    fn plan_includes_sync_then_dated_move() {
        let fs = well_formed_project();
        let layout = Layout::new("/p");
        let cfg = resolved(&fs);
        let clock = FixedClock("2026-09-08".into());
        let plan = plan_archive(
            &fs,
            &layout,
            &cfg,
            &clock,
            &ChangeId::parse("add-auth").unwrap(),
        )
        .unwrap();

        // The sync prepares the creation of the main spec.
        assert_eq!(plan.sync.creates.len(), 1);
        // The move is planned to `archive/<date>-<name>/`.
        assert_eq!(
            plan.archive_dir,
            PathBuf::from("/p/_codev/changes/archive/2026-09-08-add-auth")
        );
        assert!(plan.plan.moves.iter().any(|m| m.from
            == std::path::Path::new("/p/_codev/changes/add-auth")
            && m.to == plan.archive_dir));
    }

    #[test]
    fn archive_moves_the_change_and_keeps_its_files() {
        let fs = well_formed_project();
        let layout = Layout::new("/p");
        let cfg = resolved(&fs);
        let clock = FixedClock("2026-09-08".into());
        let outcome = execute_archive(
            &fs,
            &layout,
            &cfg,
            &clock,
            &ChangeId::parse("add-auth").unwrap(),
        )
        .unwrap();

        assert_eq!(
            outcome.moved_to,
            PathBuf::from("/p/_codev/changes/archive/2026-09-08-add-auth")
        );
        // Main spec created from the delta.
        assert!(
            fs.read("/p/_codev/specs/user-auth/spec.md")
                .is_some_and(|s| s.contains("### Requirement: Login"))
        );
        // The change's original directory no longer exists.
        assert!(!fs.exists(std::path::Path::new("/p/_codev/changes/add-auth")));
        // Its files are under the archive.
        assert!(
            fs.read("/p/_codev/changes/archive/2026-09-08-add-auth/change.yaml")
                .is_some()
        );
        assert!(
            fs.read("/p/_codev/changes/archive/2026-09-08-add-auth/specs/user-auth/spec.md")
                .is_some()
        );
    }

    #[test]
    fn preflight_validates_before_planning() {
        // A change whose delta contains a duplicate (validation finding)
        // must neither write nor move.
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file(
                "/p/_codev/changes/buggy/change.yaml",
                "schema: spec-driven",
            )
            .with_file(
                "/p/_codev/changes/buggy/specs/x/spec.md",
                "## Purpose\n\nCap.\n\n## ADDED Requirements\n\n### Requirement: X\nThe system SHALL x.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n\n### Requirement: X\nThe system SHALL x.\n\n#### Scenario: T\n- **WHEN** c\n- **THEN** d\n",
            );
        let layout = Layout::new("/p");
        let cfg = resolved(&fs);
        let clock = FixedClock("2026-09-08".into());
        let err = plan_archive(
            &fs,
            &layout,
            &cfg,
            &clock,
            &ChangeId::parse("buggy").unwrap(),
        )
        .unwrap_err();
        assert_eq!(err.code(), "validation_failed");
        assert!(err.to_string().contains("codev validate buggy"), "{err}");
        // No main spec created.
        assert!(fs.read("/p/_codev/specs/x/spec.md").is_none());
        // Change left intact.
        assert!(fs.read("/p/_codev/changes/buggy/specs/x/spec.md").is_some());
    }
}
