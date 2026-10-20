//! Roles for a project or a change: reads the settings and the schema, and
//! lets `codev-core::roles` decide (decision 0001).

use std::collections::BTreeMap;

use codev_core::Layout;
use codev_core::domains::{self, Domains};
use codev_core::roles::{self as core_roles, ResolvedRole, RoleFinding, RoleSetting, Settings};

use crate::change::ChangeContext;
use crate::config::ResolvedConfig;
use crate::error::{EngineError, Result};
use crate::ports::FileSystem;
use crate::schemas;

/// How `_codev/config.yaml` is named in role messages.
pub const PROJECT_FILE: &str = "_codev/config.yaml";

/// The roles of a project or of a change, settings applied, in workflow
/// order.
#[derive(Debug, Clone)]
pub struct ResolvedRoles {
    /// The schema the artifact roles come from.
    pub schema_name: String,
    pub roles: Vec<ResolvedRole>,
}

impl ResolvedRoles {
    /// The roles active at `step`; `unknown_step` when `step` is not one of
    /// the workflow steps.
    pub fn active_at(&self, step: &str) -> Result<Vec<&ResolvedRole>> {
        check_step(step)?;
        Ok(core_roles::active_at(&self.roles, step))
    }

    /// The roles serving `step` that the configuration turned off;
    /// `unknown_step` when `step` is not one of the workflow steps.
    pub fn disabled_at(&self, step: &str) -> Result<Vec<&ResolvedRole>> {
        check_step(step)?;
        Ok(core_roles::disabled_at(&self.roles, step))
    }

    /// The role that owns `artifact`, if its schema declares one.
    pub fn for_artifact(&self, artifact: &str) -> Option<&ResolvedRole> {
        self.roles.iter().find(|resolved| {
            matches!(&resolved.role.kind, core_roles::RoleKind::Artifact(id) if id == artifact)
        })
    }
}

/// `unknown_step` unless `step` is one of the workflow steps.
fn check_step(step: &str) -> Result<()> {
    if core_roles::STEPS.contains(&step) {
        Ok(())
    } else {
        Err(EngineError::UnknownStep {
            step: step.to_string(),
        })
    }
}

/// Resolves the roles of a change — its schema, the project's settings,
/// then its own — or, without a change, of the project and its configured
/// schema.
///
/// Fails with the code of the first invalid setting: a command must not
/// serve roles it cannot trust.
pub fn resolve(
    fs: &dyn FileSystem,
    layout: &Layout,
    config: &ResolvedConfig,
    change: Option<&ChangeContext>,
) -> Result<ResolvedRoles> {
    // Declared here so that the configured schema, when there is no change,
    // lives as long as the borrow below.
    let configured;
    let schema = match change {
        Some(ctx) => &ctx.schema,
        None => {
            configured = schemas::resolve(fs, layout, &config.schema)?;
            &configured
        }
    };
    let catalog = core_roles::catalog(schema.graph.schema(), schema.name());
    let change_settings = change.map(change_settings).unwrap_or_default();
    let change_file = change.map(change_file).unwrap_or_default();
    let domains = match change {
        Some(ctx) => change_domains(fs, layout, ctx)?,
        None => None,
    };
    let roles = core_roles::resolve(
        catalog,
        Settings {
            project: &config.roles,
            project_file: PROJECT_FILE,
            change: &change_settings,
            change_file: &change_file,
            domains: domains.as_deref(),
        },
    )
    .map_err(|finding| EngineError::RoleSetting {
        code: finding.code,
        message: finding.message,
    })?;
    Ok(ResolvedRoles {
        schema_name: schema.name().to_string(),
        roles,
    })
}

/// The invalid entries of the project's `roles:`, checked against its
/// configured schema. Empty when that schema cannot be resolved: the
/// commands that need it report that themselves.
pub fn project_findings(
    fs: &dyn FileSystem,
    layout: &Layout,
    config: &ResolvedConfig,
) -> Vec<RoleFinding> {
    if config.roles.is_empty() {
        return Vec::new();
    }
    let Ok(schema) = schemas::resolve(fs, layout, &config.schema) else {
        return Vec::new();
    };
    let catalog = core_roles::catalog(schema.graph.schema(), schema.name());
    core_roles::check_settings(&catalog, &config.roles, PROJECT_FILE)
}

/// The warnings on the project's `roles:` — a model on a role that never
/// runs as its own sub-agent — checked against its configured schema.
/// Empty when that schema cannot be resolved.
pub fn project_warnings(
    fs: &dyn FileSystem,
    layout: &Layout,
    config: &ResolvedConfig,
) -> Vec<RoleFinding> {
    if config.roles.is_empty() {
        return Vec::new();
    }
    let Ok(schema) = schemas::resolve(fs, layout, &config.schema) else {
        return Vec::new();
    };
    let catalog = core_roles::catalog(schema.graph.schema(), schema.name());
    core_roles::unused_models(&catalog, &config.roles, PROJECT_FILE)
}

/// The invalid entries of a change's `roles:`, checked against the
/// change's schema.
pub fn change_findings(ctx: &ChangeContext) -> Vec<RoleFinding> {
    let catalog = core_roles::catalog(ctx.schema.graph.schema(), ctx.schema.name());
    core_roles::check_settings(&catalog, &change_settings(ctx), &change_file(ctx))
}

/// The domains the change's proposal ticks; `None` while the proposal does
/// not exist or has no `## Domains` section. Its defects are left to
/// `codev validate`: a misspelled domain activates nothing.
fn change_domains(
    fs: &dyn FileSystem,
    layout: &Layout,
    ctx: &ChangeContext,
) -> Result<Option<Vec<String>>> {
    let path = layout.change_dir(&ctx.change).join("proposal.md");
    if !fs.exists(&path) {
        return Ok(None);
    }
    let source = fs
        .read_to_string(&path)
        .map_err(|e| EngineError::Unreadable {
            path: path.clone(),
            reason: e.to_string(),
        })?;
    Ok(match domains::parse(&source).value {
        Domains::Absent => None,
        Domains::Listed(ticked) => Some(ticked),
    })
}

fn change_settings(ctx: &ChangeContext) -> BTreeMap<String, RoleSetting> {
    ctx.metadata
        .roles
        .iter()
        .map(|(id, activation)| {
            (
                id.clone(),
                RoleSetting {
                    when: Some(activation.0.clone()),
                    // A change sets no model: `change.yaml` only overrides
                    // an activation.
                    ..RoleSetting::default()
                },
            )
        })
        .collect()
}

fn change_file(ctx: &ChangeContext) -> String {
    format!("_codev/changes/{}/change.yaml", ctx.change)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ports::{FixedEnv, MemoryFileSystem};
    use codev_core::ChangeId;
    use codev_core::roles::WhenOrigin;

    fn fs(config: &str, change_yaml: &str) -> MemoryFileSystem {
        MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", config)
            .with_file("/p/_codev/changes/add-auth/change.yaml", change_yaml)
    }

    fn roles(fs: &MemoryFileSystem, with_change: bool) -> Result<ResolvedRoles> {
        let layout = Layout::new("/p");
        let config = crate::config::resolve(fs, &FixedEnv::at("/p"), &layout)?;
        let ctx = crate::change::load(fs, &layout, &config, ChangeId::parse("add-auth").unwrap())?;
        resolve(fs, &layout, &config, with_change.then_some(&ctx))
    }

    #[test]
    fn a_role_disabled_by_the_project_is_not_served_at_its_step() {
        let fs = fs("roles:\n  newcomer: never\n", "schema: spec-driven\n");
        let resolved = roles(&fs, false).unwrap();
        assert!(resolved.active_at("code.review").unwrap().is_empty());
        let newcomer = resolved
            .roles
            .iter()
            .find(|r| r.role.id == "newcomer")
            .unwrap();
        assert_eq!(newcomer.when_origin, WhenOrigin::Project);
    }

    #[test]
    fn a_change_override_wins_only_for_that_change() {
        let fs = fs(
            "roles:\n  newcomer: never\n",
            "schema: spec-driven\nroles:\n  newcomer: always\n",
        );
        let with_change = roles(&fs, true).unwrap();
        let active = with_change.active_at("code.review").unwrap();
        assert_eq!(active.len(), 1);
        assert_eq!(active[0].when_origin, WhenOrigin::Change);
        assert!(
            roles(&fs, false)
                .unwrap()
                .active_at("code.review")
                .unwrap()
                .is_empty()
        );
    }

    #[test]
    fn a_project_finding_fails_with_its_code() {
        let fs = fs("roles:\n  reviewer: never\n", "schema: spec-driven\n");
        let err = roles(&fs, true).unwrap_err();
        assert_eq!(err.code(), "unknown_role");
        assert!(err.to_string().contains("reviewer"), "{err}");
    }

    #[test]
    fn a_change_finding_names_its_change_yaml() {
        let fs = fs("", "schema: spec-driven\nroles:\n  developer: never\n");
        let err = roles(&fs, true).unwrap_err();
        assert_eq!(err.code(), "role_not_disableable");
        assert!(
            err.to_string()
                .contains("_codev/changes/add-auth/change.yaml"),
            "{err}"
        );
    }

    #[test]
    fn an_unknown_step_is_refused() {
        let fs = fs("", "schema: spec-driven\n");
        let err = roles(&fs, false)
            .unwrap()
            .active_at("plan.review")
            .unwrap_err();
        assert_eq!(err.code(), "unknown_step");
        assert!(err.to_string().contains("plan.challenge"), "{err}");
    }

    #[test]
    fn a_project_schema_role_is_served_at_plan_write() {
        let fs = fs("schema: quick\n", "schema: quick\n").with_file(
            "/p/_codev/schemas/quick/schema.yaml",
            "name: quick\nartifacts:\n  - id: brief\n    generates: brief.md\n    role:\n      id: editor\n      name: { en: Editor }\n      question: Is the brief clear?\n      done_when: [One page at most]\napply:\n  requires: [brief]\n  tracks: brief.md\n",
        );
        let resolved = roles(&fs, true).unwrap();
        let writers = resolved.active_at("plan.write").unwrap();
        assert_eq!(writers.len(), 1);
        assert_eq!(writers[0].role.id, "editor");
        assert_eq!(writers[0].role.criteria[0].origin, "schema:quick");
        assert!(resolved.for_artifact("brief").is_some());
    }

    fn active_ids(resolved: &ResolvedRoles, step: &str) -> Vec<String> {
        resolved
            .active_at(step)
            .unwrap()
            .iter()
            .map(|r| r.role.id.clone())
            .collect()
    }

    #[test]
    fn a_ticked_domain_activates_its_expert_for_the_change() {
        let fs = fs("", "schema: spec-driven\n").with_file(
            "/p/_codev/changes/add-auth/proposal.md",
            "# Proposal: Auth\n\n## Domains\n\n- [x] security\n- [ ] performance\n",
        );
        let resolved = roles(&fs, true).unwrap();
        assert_eq!(
            active_ids(&resolved, "plan.challenge"),
            ["skeptic", "security"]
        );
        let security = resolved
            .roles
            .iter()
            .find(|r| r.role.id == "security")
            .unwrap();
        assert_eq!(
            security.activated_by,
            Some(codev_core::roles::ActivatedBy::Domain("security".into()))
        );
        // Without the change, the proposal is not read.
        assert_eq!(
            active_ids(&roles(&fs, false).unwrap(), "plan.challenge"),
            ["skeptic"]
        );
    }

    #[test]
    fn experts_stay_inactive_without_a_change_or_a_proposal() {
        let fs = fs("", "schema: spec-driven\n");
        for with_change in [true, false] {
            let resolved = roles(&fs, with_change).unwrap();
            assert_eq!(active_ids(&resolved, "code.review"), ["newcomer"]);
        }
        let without_section = fs.with_file(
            "/p/_codev/changes/add-auth/proposal.md",
            "# Proposal: Auth\n\n## Why\n\nBecause.\n",
        );
        assert_eq!(
            active_ids(&roles(&without_section, true).unwrap(), "code.review"),
            ["newcomer"]
        );
    }
}
