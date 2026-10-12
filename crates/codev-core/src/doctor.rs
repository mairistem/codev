//! `codev doctor`: what a project's codev setup still lacks.
//!
//! Pure evaluation: the shell gathers [`DoctorFacts`] (reading files, never
//! writing, never spawning a process), [`evaluate`] turns them into the
//! list of checks. Ids and codes are part of the JSON contract: they never
//! change meaning.

/// A check's verdict.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckStatus {
    Ok,
    Missing,
    Warning,
}

impl CheckStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ok => "ok",
            Self::Missing => "missing",
            Self::Warning => "warning",
        }
    }
}

/// The group a check is displayed under.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckGroup {
    Configuration,
    Planning,
    Repository,
}

impl CheckGroup {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Configuration => "configuration",
            Self::Planning => "planning",
            Self::Repository => "repository",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Configuration => "Configuration",
            Self::Planning => "Planning",
            Self::Repository => "Repository",
        }
    }
}

/// One evaluated check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Check {
    pub id: &'static str,
    pub group: CheckGroup,
    pub status: CheckStatus,
    /// `ok` when the check passes, otherwise a stable code.
    pub code: &'static str,
    pub message: String,
    /// The exact command or skill that fixes it; `None` when `ok`.
    pub fix: Option<String>,
}

/// The state of one installed skill.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SkillState {
    /// Present and stamped with the running version.
    Current,
    /// Present, stamped with another version (or none).
    Outdated(Option<String>),
    Missing,
}

/// An inherited git source, as the lock and the cache see it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitSourceFact {
    pub url: String,
    pub pinned: bool,
    pub cached: bool,
}

/// How `CHANGELOG.md` compares with what `codev changelog` generates.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ChangelogState {
    /// No `CHANGELOG.md`.
    #[default]
    Missing,
    /// No managed section, or one that differs from a fresh generation.
    Outdated,
    Current,
}

/// The running codev against the project's requirement and the cached
/// update check.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct VersionFacts {
    pub running: String,
    /// `codev_version`, and whether `running` satisfies it.
    pub requirement: Option<(String, bool)>,
    /// A release newer than `running`, as the update cache knows it.
    pub newer_release: Option<String>,
    /// The latest release the update cache knows, newer or not: the
    /// downgrade target when `running` is above the requirement.
    pub latest_known: Option<String>,
}

/// Everything [`evaluate`] needs, read by the shell.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DoctorFacts {
    /// At least one `rules:` entry, inherited ones included.
    pub rules_present: bool,
    /// `language:` is written in the project's `config.yaml`.
    pub language_set: bool,
    /// Keys of the project's `config.yaml` the running codev ignores
    /// (`mcp.<key>` inside `mcp`).
    pub unknown_config_keys: Vec<String>,
    /// `(skill name, state)` for every configured workflow.
    pub skills: Vec<(String, SkillState)>,
    pub main_specs: usize,
    /// Accepted decisions, local and inherited.
    pub accepted_decisions: usize,
    pub git_sources: Vec<GitSourceFact>,
    /// Lock entries whose `(git, ref)` no source declares.
    pub stale_pins: Vec<String>,
    pub claude_md_section: bool,
    /// CI files (relative paths) with a line running `codev validate`.
    pub ci_validating_files: Vec<String>,
    pub is_github: bool,
    pub is_gitlab: bool,
    /// The pull request template holds the codev section. Only meaningful
    /// when `is_github`.
    pub pr_template_section: bool,
    pub validate_errors: usize,
    pub validate_warnings: usize,
    pub changelog: ChangelogState,
    pub version: VersionFacts,
}

/// Stable check ids, in report order.
pub const CHECK_IDS: &[&str] = &[
    "config.rules",
    "config.language",
    "config.keys",
    "skills",
    "sources",
    "codev_version",
    "specs",
    "decisions",
    "validate",
    "claude_md",
    "ci",
    "pr_template",
    "changelog",
];

fn ok(id: &'static str, group: CheckGroup, message: impl Into<String>) -> Check {
    Check {
        id,
        group,
        status: CheckStatus::Ok,
        code: "ok",
        message: message.into(),
        fix: None,
    }
}

fn not_ok(
    id: &'static str,
    group: CheckGroup,
    status: CheckStatus,
    code: &'static str,
    message: impl Into<String>,
    fix: impl Into<String>,
) -> Check {
    Check {
        id,
        group,
        status,
        code,
        message: message.into(),
        fix: Some(fix.into()),
    }
}

/// The fix of an unsatisfied `codev_version`: a plain `codev self-update`
/// refuses to downgrade, so a codev above the requirement gets an explicit
/// `--version`.
fn version_fix(requirement: &str, version: &VersionFacts) -> String {
    use crate::update::{RequirementSide, downgrade_target, requirement_side};
    if requirement_side(requirement, &version.running) != Some(RequirementSide::Above) {
        return "codev self-update".to_string();
    }
    match downgrade_target(requirement, version.latest_known.as_deref()) {
        Some(target) => format!("codev self-update --version {target}"),
        None => format!(
            "codev self-update --version <X.Y.Z>, with a release that satisfies \"{requirement}\""
        ),
    }
}

/// Evaluates every check, in the order of [`CHECK_IDS`]. `pr_template` is
/// left out when the repository is not on GitHub.
pub fn evaluate(facts: &DoctorFacts) -> Vec<Check> {
    use CheckGroup::*;
    use CheckStatus::*;
    let mut checks = Vec::new();

    checks.push(if facts.rules_present {
        ok("config.rules", Configuration, "rules: is set")
    } else {
        not_ok(
            "config.rules",
            Configuration,
            Missing,
            "rules_missing",
            "no rules: in _codev/config.yaml",
            "/codev-configure",
        )
    });

    checks.push(if facts.language_set {
        ok("config.language", Configuration, "language: is set")
    } else {
        not_ok(
            "config.language",
            Configuration,
            Missing,
            "language_missing",
            "no language: in _codev/config.yaml",
            "add `language: <code>` (e.g. `language: en`) to _codev/config.yaml",
        )
    });

    let version = &facts.version;
    let requirement_unsatisfied = matches!(version.requirement, Some((_, false)));
    checks.push(if facts.unknown_config_keys.is_empty() {
        ok(
            "config.keys",
            Configuration,
            "no unknown key in _codev/config.yaml",
        )
    } else {
        let keys = facts
            .unknown_config_keys
            .iter()
            .map(|k| format!("`{k}`"))
            .collect::<Vec<_>>()
            .join(", ");
        not_ok(
            "config.keys",
            Configuration,
            Warning,
            "unknown_config_key",
            format!("_codev/config.yaml has key(s) this codev ignores: {keys}"),
            if requirement_unsatisfied {
                "codev self-update".to_string()
            } else {
                format!(
                    "correct or remove {keys} in _codev/config.yaml, \
                     or run `codev self-update` if a newer codev added it"
                )
            },
        )
    });

    let missing: Vec<&str> = facts
        .skills
        .iter()
        .filter(|(_, s)| *s == SkillState::Missing)
        .map(|(n, _)| n.as_str())
        .collect();
    let outdated: Vec<&str> = facts
        .skills
        .iter()
        .filter(|(_, s)| matches!(s, SkillState::Outdated(_)))
        .map(|(n, _)| n.as_str())
        .collect();
    checks.push(if !missing.is_empty() {
        not_ok(
            "skills",
            Configuration,
            Missing,
            "skills_missing",
            format!("skill(s) not installed: {}", missing.join(", ")),
            "codev update",
        )
    } else if !outdated.is_empty() {
        not_ok(
            "skills",
            Configuration,
            Missing,
            "skills_outdated",
            format!(
                "skill(s) generated by another codev version: {}",
                outdated.join(", ")
            ),
            "codev update",
        )
    } else {
        ok(
            "skills",
            Configuration,
            format!("{} skill(s) installed and current", facts.skills.len()),
        )
    });

    let pending: Vec<&str> = facts
        .git_sources
        .iter()
        .filter(|s| !s.pinned || !s.cached)
        .map(|s| s.url.as_str())
        .collect();
    checks.push(if !pending.is_empty() {
        not_ok(
            "sources",
            Configuration,
            Missing,
            "source_unpinned",
            format!(
                "git source(s) not pinned or not cached: {}",
                pending.join(", ")
            ),
            "codev sources update",
        )
    } else if !facts.stale_pins.is_empty() {
        not_ok(
            "sources",
            Configuration,
            Warning,
            "lock_stale",
            format!(
                "_codev/codev.lock pins undeclared source(s): {}",
                facts.stale_pins.join(", ")
            ),
            "codev sources update",
        )
    } else if facts.git_sources.is_empty() {
        ok("sources", Configuration, "no inherited git source")
    } else {
        ok(
            "sources",
            Configuration,
            format!("{} git source(s) pinned", facts.git_sources.len()),
        )
    });

    checks.push(match (&version.requirement, &version.newer_release) {
        (Some((requirement, false)), _) => not_ok(
            "codev_version",
            Configuration,
            Missing,
            "codev_version_unsatisfied",
            format!(
                "codev {} does not satisfy codev_version: \"{requirement}\"",
                version.running
            ),
            version_fix(requirement, version),
        ),
        (_, Some(latest)) => not_ok(
            "codev_version",
            Configuration,
            Warning,
            "update_available",
            format!("codev {latest} is available (you have {})", version.running),
            "codev self-update",
        ),
        (Some((requirement, true)), None) => ok(
            "codev_version",
            Configuration,
            format!(
                "codev {} satisfies codev_version: \"{requirement}\"",
                version.running
            ),
        ),
        (None, None) => ok(
            "codev_version",
            Configuration,
            "no newer codev release known",
        ),
    });

    checks.push(if facts.main_specs > 0 {
        ok(
            "specs",
            Planning,
            format!("{} main spec(s)", facts.main_specs),
        )
    } else {
        not_ok(
            "specs",
            Planning,
            Missing,
            "no_main_spec",
            "no main spec in _codev/specs/",
            "/codev-propose <a change> — documenting existing behavior is planned work too",
        )
    });

    checks.push(if facts.accepted_decisions > 0 {
        ok(
            "decisions",
            Planning,
            format!("{} accepted decision(s)", facts.accepted_decisions),
        )
    } else {
        not_ok(
            "decisions",
            Planning,
            Warning,
            "no_accepted_decision",
            "no accepted decision (not every project needs one)",
            "codev decision new \"<title>\"",
        )
    });

    checks.push(if facts.validate_errors > 0 {
        not_ok(
            "validate",
            Planning,
            Missing,
            "validate_errors",
            format!("codev validate reports {} error(s)", facts.validate_errors),
            "codev validate",
        )
    } else if facts.validate_warnings > 0 {
        not_ok(
            "validate",
            Planning,
            Warning,
            "validate_warnings",
            format!(
                "codev validate reports {} warning(s)",
                facts.validate_warnings
            ),
            "codev validate",
        )
    } else {
        ok("validate", Planning, "codev validate reports no issue")
    });

    checks.push(if facts.claude_md_section {
        ok("claude_md", Repository, "CLAUDE.md has the codev section")
    } else {
        not_ok(
            "claude_md",
            Repository,
            Missing,
            "claude_md_section_missing",
            "CLAUDE.md has no codev section",
            "codev init",
        )
    });

    checks.push(if let Some(file) = facts.ci_validating_files.first() {
        ok("ci", Repository, format!("{file} runs codev validate"))
    } else {
        let fix = if facts.is_github {
            "codev init"
        } else if facts.is_gitlab {
            "add the job printed by `codev init` to .gitlab-ci.yml"
        } else {
            "add `codev validate --strict` to your CI pipeline"
        };
        not_ok(
            "ci",
            Repository,
            Missing,
            "ci_validate_missing",
            "no CI file runs codev validate",
            fix,
        )
    });

    if facts.is_github {
        checks.push(if facts.pr_template_section {
            ok(
                "pr_template",
                Repository,
                "the pull request template has the codev section",
            )
        } else {
            not_ok(
                "pr_template",
                Repository,
                Missing,
                "pr_template_section_missing",
                "the pull request template has no codev section",
                "codev init",
            )
        });
    }

    // Warning only: a project may keep its changelog elsewhere, and the
    // fix is one command.
    checks.push(match facts.changelog {
        ChangelogState::Current => ok(
            "changelog",
            Repository,
            "CHANGELOG.md matches the archived changes",
        ),
        ChangelogState::Missing => not_ok(
            "changelog",
            Repository,
            Warning,
            "changelog_missing",
            "no CHANGELOG.md",
            "codev changelog --write",
        ),
        ChangelogState::Outdated => not_ok(
            "changelog",
            Repository,
            Warning,
            "changelog_outdated",
            "the [Unreleased] section of CHANGELOG.md differs from what codev changelog generates",
            "codev changelog --write",
        ),
    });

    // Report order follows CHECK_IDS, whatever the evaluation order above.
    checks.sort_by_key(|c| CHECK_IDS.iter().position(|id| *id == c.id));
    checks
}

/// `(total, missing, warnings)`.
pub fn summary(checks: &[Check]) -> (usize, usize, usize) {
    let missing = checks
        .iter()
        .filter(|c| c.status == CheckStatus::Missing)
        .count();
    let warnings = checks
        .iter()
        .filter(|c| c.status == CheckStatus::Warning)
        .count();
    (checks.len(), missing, warnings)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn complete() -> DoctorFacts {
        DoctorFacts {
            rules_present: true,
            language_set: true,
            skills: vec![("codev-propose".into(), SkillState::Current)],
            main_specs: 3,
            accepted_decisions: 2,
            git_sources: vec![GitSourceFact {
                url: "git@github.com:acme/c.git".into(),
                pinned: true,
                cached: true,
            }],
            stale_pins: Vec::new(),
            claude_md_section: true,
            ci_validating_files: vec![".github/workflows/ci.yml".into()],
            is_github: true,
            is_gitlab: false,
            pr_template_section: true,
            validate_errors: 0,
            validate_warnings: 0,
            changelog: ChangelogState::Current,
            version: VersionFacts {
                running: "0.6.0".into(),
                requirement: Some((">=0.5".into(), true)),
                newer_release: None,
                latest_known: Some("0.6.0".into()),
            },
            unknown_config_keys: Vec::new(),
        }
    }

    fn check<'a>(checks: &'a [Check], id: &str) -> &'a Check {
        checks
            .iter()
            .find(|c| c.id == id)
            .unwrap_or_else(|| panic!("no check `{id}`"))
    }

    #[test]
    fn a_complete_project_is_all_ok_in_the_stable_order() {
        let checks = evaluate(&complete());
        assert!(checks.iter().all(|c| c.status == CheckStatus::Ok));
        let ids: Vec<&str> = checks.iter().map(|c| c.id).collect();
        assert_eq!(ids, CHECK_IDS);
        assert!(checks.iter().all(|c| c.fix.is_none() && c.code == "ok"));
    }

    #[test]
    fn fresh_project_checks() {
        let facts = DoctorFacts {
            language_set: true,
            skills: vec![("codev-propose".into(), SkillState::Current)],
            claude_md_section: true,
            ..DoctorFacts::default()
        };
        let checks = evaluate(&facts);
        assert_eq!(check(&checks, "config.rules").code, "rules_missing");
        assert_eq!(
            check(&checks, "config.rules").fix.as_deref(),
            Some("/codev-configure")
        );
        assert_eq!(check(&checks, "specs").status, CheckStatus::Missing);
        assert!(
            check(&checks, "specs")
                .fix
                .as_deref()
                .unwrap()
                .contains("/codev-propose")
        );
        assert_eq!(check(&checks, "decisions").status, CheckStatus::Warning);
        assert_eq!(check(&checks, "ci").code, "ci_validate_missing");
        for id in [
            "config.keys",
            "config.language",
            "skills",
            "sources",
            "codev_version",
            "claude_md",
            "validate",
        ] {
            assert_eq!(check(&checks, id).status, CheckStatus::Ok, "{id}");
        }
        assert!(checks.iter().all(|c| c.id != "pr_template"));
        assert_eq!(check(&checks, "changelog").code, "changelog_missing");
    }

    #[test]
    fn outdated_skills_are_missing_with_skills_outdated() {
        let facts = DoctorFacts {
            skills: vec![(
                "codev-propose".into(),
                SkillState::Outdated(Some("0.3.0".into())),
            )],
            ..complete()
        };
        let skills = evaluate(&facts)
            .into_iter()
            .find(|c| c.id == "skills")
            .unwrap();
        assert_eq!(skills.status, CheckStatus::Missing);
        assert_eq!(skills.code, "skills_outdated");
        assert_eq!(skills.fix.as_deref(), Some("codev update"));

        let facts = DoctorFacts {
            skills: vec![("codev-apply".into(), SkillState::Missing)],
            ..complete()
        };
        assert_eq!(check(&evaluate(&facts), "skills").code, "skills_missing");
    }

    #[test]
    fn unpinned_source_is_missing() {
        let facts = DoctorFacts {
            git_sources: vec![GitSourceFact {
                url: "https://example.com/c.git".into(),
                pinned: false,
                cached: false,
            }],
            ..complete()
        };
        let checks = evaluate(&facts);
        let sources = check(&checks, "sources");
        assert_eq!(sources.status, CheckStatus::Missing);
        assert_eq!(sources.code, "source_unpinned");
        assert_eq!(sources.fix.as_deref(), Some("codev sources update"));
    }

    #[test]
    fn stale_pin_is_a_warning() {
        let facts = DoctorFacts {
            stale_pins: vec!["https://old.example/c.git@main".into()],
            ..complete()
        };
        let checks = evaluate(&facts);
        assert_eq!(check(&checks, "sources").status, CheckStatus::Warning);
        assert_eq!(check(&checks, "sources").code, "lock_stale");
    }

    #[test]
    fn pr_template_check_only_on_github() {
        let facts = DoctorFacts {
            pr_template_section: false,
            ..complete()
        };
        assert_eq!(
            check(&evaluate(&facts), "pr_template").code,
            "pr_template_section_missing"
        );
        let facts = DoctorFacts {
            is_github: false,
            ..complete()
        };
        assert!(evaluate(&facts).iter().all(|c| c.id != "pr_template"));
    }

    #[test]
    fn ci_fix_depends_on_the_forge() {
        let base = DoctorFacts {
            ci_validating_files: Vec::new(),
            ..complete()
        };
        assert_eq!(
            check(&evaluate(&base), "ci").fix.as_deref(),
            Some("codev init")
        );
        let gitlab = DoctorFacts {
            is_github: false,
            is_gitlab: true,
            ..base.clone()
        };
        assert!(
            check(&evaluate(&gitlab), "ci")
                .fix
                .as_deref()
                .unwrap()
                .contains(".gitlab-ci.yml")
        );
    }

    #[test]
    fn validate_warnings_are_a_warning_and_errors_missing() {
        let warnings = DoctorFacts {
            validate_warnings: 2,
            ..complete()
        };
        assert_eq!(
            check(&evaluate(&warnings), "validate").code,
            "validate_warnings"
        );
        let errors = DoctorFacts {
            validate_errors: 1,
            validate_warnings: 2,
            ..complete()
        };
        let checks = evaluate(&errors);
        assert_eq!(check(&checks, "validate").status, CheckStatus::Missing);
        assert_eq!(check(&checks, "validate").code, "validate_errors");
    }

    #[test]
    fn changelog_check_is_a_warning_when_missing_or_outdated() {
        for (state, code) in [
            (ChangelogState::Missing, "changelog_missing"),
            (ChangelogState::Outdated, "changelog_outdated"),
        ] {
            let facts = DoctorFacts {
                changelog: state,
                ..complete()
            };
            let checks = evaluate(&facts);
            let changelog = check(&checks, "changelog");
            assert_eq!(changelog.status, CheckStatus::Warning);
            assert_eq!(changelog.code, code);
            assert_eq!(changelog.fix.as_deref(), Some("codev changelog --write"));
            assert_eq!(summary(&checks).1, 0, "never counted as missing");
        }
    }

    #[test]
    fn summary_counts_missing_and_warnings() {
        let facts = DoctorFacts {
            rules_present: false,
            main_specs: 0,
            accepted_decisions: 0,
            ..complete()
        };
        assert_eq!(summary(&evaluate(&facts)), (13, 2, 1));
    }

    #[test]
    fn codev_version_unsatisfied_is_missing() {
        let facts = DoctorFacts {
            version: VersionFacts {
                running: "0.6.0".into(),
                requirement: Some((">=0.7".into(), false)),
                newer_release: Some("0.7.0".into()),
                latest_known: Some("0.7.0".into()),
            },
            ..complete()
        };
        let checks = evaluate(&facts);
        let version = check(&checks, "codev_version");
        assert_eq!(version.status, CheckStatus::Missing);
        assert_eq!(version.code, "codev_version_unsatisfied");
        assert_eq!(version.fix.as_deref(), Some("codev self-update"));
        assert!(version.message.contains(">=0.7"), "{}", version.message);
        assert_eq!(version.group, CheckGroup::Configuration);
    }

    #[test]
    fn a_known_newer_release_is_a_warning() {
        let facts = DoctorFacts {
            version: VersionFacts {
                running: "0.6.0".into(),
                requirement: None,
                newer_release: Some("0.7.0".into()),
                latest_known: Some("0.7.0".into()),
            },
            ..complete()
        };
        let checks = evaluate(&facts);
        let version = check(&checks, "codev_version");
        assert_eq!(version.status, CheckStatus::Warning);
        assert_eq!(version.code, "update_available");
        assert_eq!(version.fix.as_deref(), Some("codev self-update"));
        assert_eq!(summary(&checks).1, 0, "never counted as missing");
    }

    fn above(requirement: &str, running: &str, latest_known: Option<&str>) -> Check {
        let facts = DoctorFacts {
            version: VersionFacts {
                running: running.into(),
                requirement: Some((requirement.into(), false)),
                newer_release: None,
                latest_known: latest_known.map(Into::into),
            },
            ..complete()
        };
        check(&evaluate(&facts), "codev_version").clone()
    }

    #[test]
    fn codev_version_above_the_bound_fixes_with_an_explicit_version() {
        let version = above(">=0.5, <0.7", "0.7.1", Some("0.6.4"));
        assert_eq!(version.status, CheckStatus::Missing);
        assert_eq!(version.code, "codev_version_unsatisfied");
        assert_eq!(
            version.fix.as_deref(),
            Some("codev self-update --version 0.6.4")
        );
        assert_eq!(
            above("<=0.6.2", "0.7.0", None).fix.as_deref(),
            Some("codev self-update --version 0.6.2")
        );
        assert_eq!(
            above("^0.5", "0.6.0", Some("0.6.0")).fix.as_deref(),
            Some("codev self-update --version 0.5.0")
        );
    }

    #[test]
    fn codev_version_above_without_any_target_names_the_requirement() {
        let fix = above("<0.6", "0.6.0", None).fix.unwrap();
        assert!(
            fix.starts_with("codev self-update --version <X.Y.Z>"),
            "{fix}"
        );
        assert!(fix.contains("\"<0.6\""), "{fix}");
    }

    #[test]
    fn unknown_config_keys_are_a_warning() {
        let facts = DoctorFacts {
            unknown_config_keys: vec!["review_policy".into(), "mcp.design_tool".into()],
            ..complete()
        };
        let checks = evaluate(&facts);
        let keys = check(&checks, "config.keys");
        assert_eq!(keys.status, CheckStatus::Warning);
        assert_eq!(keys.code, "unknown_config_key");
        assert_eq!(keys.group, CheckGroup::Configuration);
        assert!(keys.message.contains("`review_policy`"), "{}", keys.message);
        assert!(
            keys.message.contains("`mcp.design_tool`"),
            "{}",
            keys.message
        );
        let fix = keys.fix.as_deref().unwrap();
        assert!(fix.starts_with("correct or remove"), "{fix}");
        assert_eq!(summary(&checks).1, 0, "never counted as missing");
    }

    #[test]
    fn unknown_config_keys_with_an_unsatisfied_requirement_fix_with_self_update() {
        let facts = DoctorFacts {
            unknown_config_keys: vec!["review_policy".into()],
            version: VersionFacts {
                running: "0.6.0".into(),
                requirement: Some((">=9".into(), false)),
                newer_release: None,
                latest_known: None,
            },
            ..complete()
        };
        let checks = evaluate(&facts);
        assert_eq!(
            check(&checks, "config.keys").fix.as_deref(),
            Some("codev self-update")
        );
    }
}
