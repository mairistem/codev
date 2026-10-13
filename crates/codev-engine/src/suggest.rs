//! The domains a change's diff suggests: the triggers of the experts the
//! change can activate, and the project's own, matched against the diff,
//! each suggestion with the proposal's answer.
//!
//! The diff is read once by the caller ([`crate::diff::read`]) and shared
//! by every change it asks about; deciding is `codev_core::triggers`'.

use std::collections::BTreeMap;

use codev_core::Layout;
use codev_core::roles::Activation;
use codev_core::triggers::{self, Problem, ScanSummary, Suggestion, Triggers};

use crate::change::ChangeContext;
use crate::config::ResolvedConfig;
use crate::diff::Diff;
use crate::error::Warning;
use crate::ports::FileSystem;
use crate::roles::ResolvedRoles;

/// The diff's summary, as `codev roles` reports it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiffSummary {
    pub available: bool,
    /// Why there is no diff; `None` when there is one.
    pub reason: Option<String>,
    /// The default branch used; `None` without a diff.
    pub base: Option<String>,
    pub files: usize,
    pub scanned: usize,
    pub skipped: BTreeMap<&'static str, usize>,
    pub truncated: bool,
}

/// The suggestions for one change.
#[derive(Debug, Clone)]
pub struct Suggestions {
    pub diff: DiffSummary,
    pub suggestions: Vec<Suggestion>,
    /// `invalid_trigger` for each project group that cannot be used.
    pub warnings: Vec<Warning>,
}

/// The suggested domains of the change `ctx`, whose roles are `roles`.
///
/// The triggers are those of the roles the change has not set to `never`
/// and the project's usable groups. The proposal is read for each
/// suggestion's state; without it, every suggestion is `unticked`.
pub fn for_change(
    fs: &dyn FileSystem,
    layout: &Layout,
    config: &ResolvedConfig,
    ctx: &ChangeContext,
    roles: &ResolvedRoles,
    diff: &Diff,
) -> Suggestions {
    let project = triggers::read_groups(config.triggers.as_ref());
    let warnings = project
        .problems
        .iter()
        .filter(|p| p.is_error)
        .map(|p| Warning::new(triggers::codes::INVALID_TRIGGER, ignored(p)))
        .collect();

    let (base, files) = match diff {
        Diff::Unavailable { reason } => {
            return Suggestions {
                diff: DiffSummary {
                    available: false,
                    reason: Some(reason.clone()),
                    base: None,
                    files: 0,
                    scanned: 0,
                    skipped: BTreeMap::new(),
                    truncated: false,
                },
                suggestions: Vec::new(),
                warnings,
            };
        }
        Diff::Available { base, files } => (base, files),
    };

    let builtin = roles
        .roles
        .iter()
        .filter(|resolved| resolved.when != Activation::Never)
        .flat_map(|resolved| {
            resolved
                .role
                .triggers
                .iter()
                .map(|group| (resolved.role.id.as_str(), group))
        });
    let project_groups = project
        .groups
        .iter()
        .map(|group| (triggers::PROJECT_SOURCE, group));
    let compiled = Triggers::compile(builtin.chain(project_groups));
    let (matches, scan) = triggers::evaluate(files, &compiled);

    let proposal = fs
        .read_to_string(&layout.change_dir(&ctx.change).join("proposal.md"))
        .ok();
    Suggestions {
        diff: summary(base, scan),
        suggestions: triggers::suggest(&matches, proposal.as_deref()),
        warnings,
    }
}

fn summary(base: &str, scan: ScanSummary) -> DiffSummary {
    DiffSummary {
        available: true,
        reason: None,
        base: Some(base.to_string()),
        files: scan.files,
        scanned: scan.scanned,
        skipped: scan.skipped,
        truncated: scan.truncated,
    }
}

fn ignored(problem: &Problem) -> String {
    format!(
        "_codev/config.yaml: {}; the group is ignored — `codev validate` points at it",
        problem.message
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ports::{FixedEnv, MemoryFileSystem};
    use codev_core::ChangeId;
    use codev_core::triggers::{ChangedFile, State};

    fn suggestions(fs: &MemoryFileSystem, paths: &[&str]) -> Suggestions {
        let layout = Layout::new("/p");
        let config = crate::config::resolve(fs, &FixedEnv::at("/p"), &layout).unwrap();
        let ctx = crate::change::load(fs, &layout, &config, ChangeId::parse("add-login").unwrap())
            .unwrap();
        let roles = crate::roles::resolve(fs, &layout, &config, Some(&ctx)).unwrap();
        let diff = Diff::Available {
            base: "main".into(),
            files: paths
                .iter()
                .map(|path| ChangedFile {
                    path: path.to_string(),
                    added_lines: Some(0),
                    lines: Some(Vec::new()),
                    ..ChangedFile::default()
                })
                .collect(),
        };
        for_change(fs, &layout, &config, &ctx, &roles, &diff)
    }

    fn project(config: &str) -> MemoryFileSystem {
        MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", config)
            .with_file(
                "/p/_codev/changes/add-login/change.yaml",
                "schema: spec-driven\n",
            )
    }

    fn domains(found: &Suggestions) -> Vec<&str> {
        found
            .suggestions
            .iter()
            .map(|s| s.domain.as_str())
            .collect()
    }

    #[test]
    fn a_disabled_expert_suggests_nothing() {
        let fs = project("roles:\n  accessibility: never\n");
        let found = suggestions(&fs, &["src/components/Button.tsx", "src/auth/login.rs"]);
        assert_eq!(domains(&found), ["security"]);
        let on = project("");
        assert_eq!(
            domains(&suggestions(&on, &["src/components/Button.tsx"])),
            ["user-interface"]
        );
    }

    #[test]
    fn a_project_trigger_suggests_its_domain() {
        let fs = project("triggers:\n  - domain: personal-data\n    paths: [\"src/billing/**\"]\n");
        let found = suggestions(&fs, &["src/billing/invoice.rs"]);
        assert_eq!(domains(&found), ["personal-data"]);
        assert_eq!(found.suggestions[0].matches[0].source, "project");
        assert!(found.warnings.is_empty());
    }

    #[test]
    fn an_invalid_project_group_is_ignored_with_a_warning() {
        let fs = project(
            "triggers:\n  - domain: secrets\n    paths: [\"**\"]\n  - domain: security\n    patterns: ['(unclosed']\n",
        );
        let found = suggestions(&fs, &["README"]);
        assert!(found.suggestions.is_empty());
        let codes: Vec<&str> = found.warnings.iter().map(|w| w.code).collect();
        assert_eq!(codes, ["invalid_trigger", "invalid_trigger"]);
        assert!(found.warnings[0].message.contains("secrets"));
    }

    #[test]
    fn a_missing_proposal_leaves_every_suggestion_unticked() {
        let found = suggestions(&project(""), &["src/auth/login.rs"]);
        assert_eq!(found.suggestions[0].state, State::Unticked);
        let ticked = project("").with_file(
            "/p/_codev/changes/add-login/proposal.md",
            "# Proposal: Login\n\n## Domains\n\n- [x] security\n",
        );
        let found = suggestions(&ticked, &["src/auth/login.rs"]);
        assert_eq!(found.suggestions[0].state, State::Ticked);
        assert!(found.diff.available);
        assert_eq!(found.diff.base.as_deref(), Some("main"));
    }

    #[test]
    fn no_diff_is_reported_with_its_reason() {
        let fs = project("");
        let layout = Layout::new("/p");
        let config = crate::config::resolve(&fs, &FixedEnv::at("/p"), &layout).unwrap();
        let ctx = crate::change::load(&fs, &layout, &config, ChangeId::parse("add-login").unwrap())
            .unwrap();
        let roles = crate::roles::resolve(&fs, &layout, &config, Some(&ctx)).unwrap();
        let diff = Diff::Unavailable {
            reason: "the project is not in a git repository".into(),
        };
        let found = for_change(&fs, &layout, &config, &ctx, &roles, &diff);
        assert!(!found.diff.available);
        assert_eq!(
            found.diff.reason.as_deref(),
            Some("the project is not in a git repository")
        );
        assert!(found.suggestions.is_empty());
    }
}
