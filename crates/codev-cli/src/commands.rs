use std::fmt;
use std::path::{Path, PathBuf};

use codev_agents::claude::ClaudeCode;
use codev_agents::target::AgentTarget;
use codev_agents::workflows;
use codev_core::changelog;
use codev_core::config::InheritArg;
use codev_core::doctor::{
    self as core_doctor, ChangelogState, Check, CheckStatus, DoctorFacts, GitSourceFact,
    RolesFacts, VersionFacts,
};
use codev_core::layout::slashed;
use codev_core::managed::{self, SpliceMode};
use codev_core::roles::{Activation, ResolvedRole};
use codev_core::update::UpdateNotice;
use codev_core::{ChangeId, ChangeStatus, CoreError, Layout, Plan, WriteMode};
use codev_engine::apply::{self, Applied};
use codev_engine::archive as engine_archive;
use codev_engine::archive::ArchiveOutcome;
use codev_engine::changelog as engine_changelog;
use codev_engine::config;
use codev_engine::decisions as engine_decisions;
use codev_engine::decisions_actions as engine_actions;
use codev_engine::detect as engine_detect;
use codev_engine::diff as engine_diff;
use codev_engine::instructions::Instructions;
use codev_engine::integration::{self, IntegrationPlan, IntegrationStep};
use codev_engine::metadata::ChangeMetadata;
use codev_engine::review as engine_review;
use codev_engine::roles as engine_roles;
use codev_engine::sources as engine_sources;
use codev_engine::suggest as engine_suggest;
use codev_engine::sync as engine_sync;
use codev_engine::sync::SyncOutcome;
use codev_engine::update as engine_update;
use codev_engine::validate as engine_validate;
use codev_engine::validate::ValidateReport;
use codev_engine::{Clock, EngineError, Env, Executable, FileSystem, Http, ProcessRunner, Warning};
use codev_engine::{change, instructions, root, scaffold, schemas, specs};

/// A command failure, reduced to what both outputs need: a stable code for
/// the JSON, a readable message for the terminal.
#[derive(Debug)]
pub struct Failure {
    pub code: String,
    pub message: String,
    /// The fix to suggest, when it is known.
    pub fix: Option<String>,
}

impl fmt::Display for Failure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl Failure {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            fix: None,
        }
    }

    pub fn with_fix(mut self, fix: impl Into<String>) -> Self {
        self.fix = Some(fix.into());
        self
    }
}

impl From<EngineError> for Failure {
    fn from(error: EngineError) -> Self {
        Self::new(error.code(), error.to_string())
    }
}

impl From<engine_update::UpdateError> for Failure {
    fn from(error: engine_update::UpdateError) -> Self {
        Self::new(error.code(), error.to_string())
    }
}

impl From<CoreError> for Failure {
    fn from(error: CoreError) -> Self {
        Self::new(error.code(), error.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Failure>;

/// The ports, bundled together. The only object the commands receive from
/// the outside world.
pub struct Ctx<'a> {
    pub fs: &'a dyn FileSystem,
    pub env: &'a dyn Env,
    pub clock: &'a dyn Clock,
    /// `codev sources update` and `codev init --inherit <git>` use it, and
    /// `codev changelog` and the doctor `changelog` check for local,
    /// read-only git queries.
    pub runner: &'a dyn ProcessRunner,
    /// The release API, for the update check of `codev status` and
    /// `codev self-update` — the only commands that use it.
    pub http: &'a dyn Http,
    /// The running binary, for `codev self-update`.
    pub exe: &'a dyn Executable,
    pub version: &'a str,
    /// The build target triple, to pick the release asset.
    pub target: &'a str,
}

// ─────────────────────────────── init / update ───────────────────────────────

#[derive(Debug)]
pub struct SetupOutcome {
    pub root: PathBuf,
    pub created: Vec<PathBuf>,
    pub updated: Vec<PathBuf>,
    pub untouched: Vec<PathBuf>,
    pub preserved: Vec<PathBuf>,
    pub skills: Vec<String>,
    pub warnings: Vec<Warning>,
    /// True if the resolved `_codev/config.yaml` is thin (no rules). Used to
    /// decide on the hint in human output; does not appear in the JSON
    /// contract.
    pub config_thin: bool,
    /// What each repository integration step did.
    pub integration: Vec<IntegrationStep>,
    /// The pins `codev init --inherit` added to `_codev/codev.lock`.
    pub pins: Vec<engine_sources::PinChange>,
    /// Number of `codev doctor` checks still missing, for the human hint.
    /// `None` when doctor could not run — the hint is then left out.
    pub doctor_missing: Option<usize>,
}

/// Initializes codev in a project.
///
/// Three stages, and the order matters:
///
/// 1. **Probe** (unless `--no-detect`): reads manifests, MCPs, license, CI —
///    no writing.
/// 2. **Prompts** (unless `--yes` or non-TTY stdin): two questions, plus an
///    optional MCP confirmation.
/// 3. **Scaffolding + generation**: if `_codev/config.yaml` is missing, it is
///    written prefilled with provenance comments; otherwise it is left
///    untouched (idempotent behavior). Then the resolved configuration is
///    **read back** to install the skills.
///
/// A project that had already chosen its workflows therefore keeps its
/// choice.
pub fn init(
    ctx: &Ctx,
    path: &str,
    force: bool,
    opts: &crate::init_prompts::InitOptions,
) -> Result<SetupOutcome> {
    let root_path = absolute(ctx, path)?;
    let layout = Layout::new(&root_path);
    let config_path = layout.project_root().join("_codev").join("config.yaml");

    // Inherited sources are checked before the first write: a typo in a
    // path must not leave a half-initialized project behind.
    let inherits = resolve_inherits(ctx, &opts.inherits)?;
    let mut warnings = Vec::new();

    // Writes the generated config.yaml ONLY if missing — an existing file is
    // never touched; the user's configuration stays in charge. It goes
    // through a plan like every other write, so that it is counted among
    // the created files.
    let mut config_written = Applied::default();
    if ctx.fs.exists(&config_path) {
        if !inherits.is_empty() {
            let listed: Vec<String> = inherits.iter().map(describe_inherit).collect();
            warnings.push(Warning::new(
                "inherit_not_written",
                format!(
                    "{} already exists and is never rewritten: add {} under `inherits:` by hand",
                    config_path.display(),
                    listed.join(", ")
                ),
            ));
        }
    } else {
        let detected = if opts.no_detect {
            codev_core::detect::Detected::empty()
        } else {
            codev_engine::detect::run(ctx.fs, ctx.env, layout.project_root())
        };
        let choices = crate::init_prompts::run(&detected, opts)
            .map_err(|e| Failure::new("prompt_failed", e.to_string()))?;
        let mut generated = codev_core::config::from_detected(&detected, &choices);
        generated.inherits = inherits.clone();
        let mut plan = Plan::new();
        plan.dir(layout.planning_dir()).write(
            &config_path,
            codev_core::config::render(&generated),
            WriteMode::CreateOnly,
        );
        config_written = apply::execute(&plan, ctx.fs)?;
    }

    // Scaffold to create the missing folders (specs/, changes/, etc.) — it
    // does not rewrite the config.yaml written above thanks to
    // WriteMode::CreateOnly.
    let scaffolded = apply::execute(&scaffold::plan_init(&layout), ctx.fs)?;
    let config_created = !config_written.created.is_empty();

    // Pin the git sources this init has just declared, before the skills
    // resolve the configuration — so that resolution sees the pins. Only
    // into a project without a lock: init may add the first pins, never
    // move one — that is `codev sources update`'s job (decision 0005).
    let git_sources: Vec<engine_sources::GitSourceInput> = if !config_created {
        Vec::new()
    } else {
        inherits
            .iter()
            .filter_map(|source| match source {
                InheritArg::Git { url, git_ref } => Some(engine_sources::GitSourceInput {
                    git: url.clone(),
                    git_ref: git_ref.clone(),
                    subpath: None,
                }),
                InheritArg::Path(_) => None,
            })
            .collect()
    };
    let mut pins = Vec::new();
    let mut lock_created = None;
    if !git_sources.is_empty() {
        let lock_path = layout.planning_dir().join("codev.lock");
        if ctx.fs.exists(&lock_path) {
            warnings.push(Warning::new(
                "inherit_pin_failed",
                format!(
                    "{} already exists; init never moves a pin",
                    lock_path.display()
                ),
            ));
        } else {
            match engine_sources::run_sources_update(
                ctx.fs,
                ctx.env,
                ctx.runner,
                ctx.clock,
                &layout,
                &git_sources,
            ) {
                Ok(pinned) => {
                    if pinned.lock_written {
                        lock_created = Some(lock_path);
                    }
                    pins = pinned.diff;
                }
                Err(err) => warnings.push(Warning::new(
                    "inherit_pin_failed",
                    format!("the inherited git source(s) could not be pinned: {err}"),
                )),
            }
        }
    }

    let mut outcome = install_skills(ctx, &layout, force)?;
    outcome.absorb(config_written);
    outcome.absorb(scaffolded);
    outcome.created.extend(lock_created);
    outcome.pins = pins;
    outcome.warnings.extend(warnings);

    let integration = plan_integration(
        ctx,
        &layout,
        &outcome.skills,
        SpliceMode::Init,
        IntegrationFlags {
            claude_md: !opts.no_claude_md,
            ci: !opts.no_ci,
            pr_template: !opts.no_pr_template,
        },
    );
    outcome.apply_integration(ctx, integration)?;

    // The scaffold reports the config written above as untouched: it is
    // listed once, as created.
    let created = outcome.created.clone();
    outcome.untouched.retain(|p| !created.contains(p));

    outcome.doctor_missing = doctor_at(ctx, &layout).ok().map(|d| {
        d.checks
            .iter()
            .filter(|c| c.status == CheckStatus::Missing)
            .count()
    });
    Ok(outcome)
}

/// Parses `--inherit` values, checks them against the file system and makes
/// relative paths absolute, so the written config resolves from any folder.
fn resolve_inherits(ctx: &Ctx, raw: &[String]) -> Result<Vec<InheritArg>> {
    let mut out = Vec::new();
    for value in raw {
        let source = codev_core::config::inherit::parse(value).map_err(|reason| {
            Failure::new(
                "invalid_inherit",
                format!("invalid --inherit value: {reason}"),
            )
        })?;
        match &source {
            InheritArg::Path(path) => {
                let written = if path.starts_with('~') || Path::new(path).is_absolute() {
                    path.clone()
                } else {
                    slashed(&absolute(ctx, path)?)
                };
                let on_disk = config::expand_tilde(&written, ctx.env);
                if !ctx.fs.exists(&on_disk) || ctx.fs.list_dir(&on_disk).is_err() {
                    return Err(Failure::new(
                        "invalid_inherit",
                        format!("inherited source `{path}` is not an existing folder"),
                    )
                    .with_fix("check the path, or give a git URL (`…/conventions.git@main`)"));
                }
                out.push(InheritArg::Path(written));
            }
            InheritArg::Git { url, git_ref } => {
                // A local repository given by a relative path: git would
                // resolve it against whatever folder it runs in.
                let is_local_relative = !url.contains("://")
                    && !url.contains(':')
                    && !url.starts_with('~')
                    && !Path::new(url).is_absolute();
                let url = if is_local_relative {
                    slashed(&absolute(ctx, url)?)
                } else {
                    url.clone()
                };
                out.push(InheritArg::Git {
                    url,
                    git_ref: git_ref.clone(),
                });
            }
        }
    }
    Ok(out)
}

fn describe_inherit(source: &InheritArg) -> String {
    match source {
        InheritArg::Git { url, git_ref } => format!("`git: {url}` (ref `{git_ref}`)"),
        InheritArg::Path(path) => format!("`path: {path}`"),
    }
}

/// Which integration steps run.
struct IntegrationFlags {
    claude_md: bool,
    ci: bool,
    pr_template: bool,
}

/// Plans the repository integration: `CLAUDE.md` (Claude Code's file), the
/// CI workflow and the pull request template (the forge's files).
fn plan_integration(
    ctx: &Ctx,
    layout: &Layout,
    skills: &[String],
    mode: SpliceMode,
    flags: IntegrationFlags,
) -> IntegrationPlan {
    let root = layout.project_root();
    let facts = integration::RepoFacts::probe(ctx.fs, root);
    let mut planned = IntegrationPlan::default();
    if flags.claude_md {
        planned.merge(ClaudeCode::plan_claude_md(ctx.fs, root, skills, mode));
    }
    if flags.ci {
        planned.merge(integration::plan_ci(&facts, ctx.version, mode));
    }
    if flags.pr_template {
        planned.merge(integration::plan_pr_template(&facts, mode));
    }
    planned
}

/// Regenerates the skills of an already initialized project.
///
/// Also runs the scaffolding plan: in "create only what is missing" mode, it
/// restores a deleted folder without touching anything else. Integration
/// files are only refreshed where codev's markers already are: `update`
/// never adds a section a project does not have.
pub fn update(ctx: &Ctx, force: bool) -> Result<SetupOutcome> {
    let layout = root::discover_from_cwd(ctx.fs, ctx.env)?;
    let scaffolded = apply::execute(&scaffold::plan_init(&layout), ctx.fs)?;
    let mut outcome = install_skills(ctx, &layout, force)?;
    outcome.absorb(scaffolded);
    let integration = plan_integration(
        ctx,
        &layout,
        &outcome.skills,
        SpliceMode::Refresh,
        IntegrationFlags {
            claude_md: true,
            ci: true,
            pr_template: true,
        },
    );
    outcome.apply_integration(ctx, integration)?;
    Ok(outcome)
}

fn install_skills(ctx: &Ctx, layout: &Layout, force: bool) -> Result<SetupOutcome> {
    let config = config::resolve(ctx.fs, ctx.env, layout)?;
    let (selected, mut warnings) = workflows::select(config.workflows.as_deref());
    warnings.extend(config.warnings.clone());

    // The render context carries the project's MCP config: the name of the
    // Jira tool to inject into the frontmatter, other MCPs to come. Without
    // config, RenderCtx is empty and the placeholder is removed cleanly.
    let target = ClaudeCode::with_ctx(codev_agents::claude::RenderCtx {
        jira_mcp_tool: config.mcp.jira_tool.clone(),
    });
    let planned = target.plan_skills(ctx.fs, layout.project_root(), &selected, ctx.version, force);
    let applied = apply::execute(&planned.plan, ctx.fs)?;

    // Hint indicator — evaluated on the resolved config. Used to decide
    // whether the human output of `codev init` should point to
    // `/codev-configure`. The decision rests solely on the absence of
    // `rules:`: `context:` is often auto-filled by the probe and is not a
    // reliable signal of user intent.
    let config_thin = codev_core::config::is_config_thin(config.rules.is_empty());

    let mut outcome = SetupOutcome {
        root: layout.project_root().to_path_buf(),
        created: Vec::new(),
        updated: Vec::new(),
        untouched: Vec::new(),
        preserved: planned.preserved,
        skills: selected
            .iter()
            .map(|w| ClaudeCode::skill_name(w.id))
            .collect(),
        warnings,
        config_thin,
        integration: Vec::new(),
        pins: Vec::new(),
        doctor_missing: None,
    };
    outcome.absorb(applied);
    Ok(outcome)
}

impl SetupOutcome {
    fn absorb(&mut self, applied: Applied) {
        self.created.extend(applied.created);
        self.updated.extend(applied.overwritten);
        self.untouched.extend(applied.untouched);
    }

    pub fn changed_anything(&self) -> bool {
        !self.created.is_empty() || !self.updated.is_empty()
    }

    fn apply_integration(&mut self, ctx: &Ctx, planned: IntegrationPlan) -> Result<()> {
        let applied = apply::execute(&planned.plan, ctx.fs)?;
        self.absorb(applied);
        self.integration.extend(planned.steps);
        self.warnings.extend(planned.warnings);
        Ok(())
    }
}

// ─────────────────────────────── new change ───────────────────────────────

#[derive(Debug)]
pub struct NewChangeOutcome {
    pub change: ChangeId,
    pub schema_name: String,
    pub change_root: PathBuf,
    pub created: Vec<PathBuf>,
    pub warnings: Vec<Warning>,
}

pub fn new_change(
    ctx: &Ctx,
    name: &str,
    schema: Option<&str>,
    goal: Option<String>,
) -> Result<NewChangeOutcome> {
    let layout = root::discover_from_cwd(ctx.fs, ctx.env)?;
    let config = config::resolve(ctx.fs, ctx.env, &layout)?;
    let change = ChangeId::parse(name)?;

    if ctx.fs.exists(&layout.change_dir(&change)) {
        return Err(EngineError::ChangeExists {
            change: change.to_string(),
        }
        .into());
    }

    // The schema is resolved now, not at the first command that needs it: a
    // wrong name must fail here, before a change folder carries invalid
    // metadata.
    let schema_name = schema.unwrap_or(&config.schema).to_string();
    let resolved = schemas::resolve(ctx.fs, &layout, &schema_name)?;

    // Recorded under the name it was resolved by, never the `name:` inside
    // `schema.yaml`: the latter may differ from the folder, and the change
    // could then no longer be loaded.
    let metadata = ChangeMetadata::new(resolved.name(), ctx.clock.today()).with_goal(goal);
    let applied = apply::execute(
        &scaffold::plan_new_change(&layout, &change, &metadata),
        ctx.fs,
    )?;

    Ok(NewChangeOutcome {
        change_root: layout.change_dir(&change),
        change,
        schema_name,
        created: applied.created,
        warnings: config.warnings,
    })
}

// ─────────────────────────────── status ───────────────────────────────

#[derive(Debug)]
pub struct StatusOutcome {
    pub status: ChangeStatus,
    pub planning_home: PathBuf,
    pub change_root: PathBuf,
    pub warnings: Vec<Warning>,
    /// The update check; `None` when the latest release is unknown or the
    /// check is disabled.
    pub update: Option<UpdateNotice>,
    /// `codev_version`, and whether the running version satisfies it.
    pub requirement: Option<(String, bool)>,
}

pub fn status(ctx: &Ctx, requested: Option<&str>) -> Result<StatusOutcome> {
    let layout = root::discover_from_cwd(ctx.fs, ctx.env)?;
    let config = config::resolve(ctx.fs, ctx.env, &layout)?;
    let change = resolve_change(ctx.fs, &layout, requested)?;
    let ctxt = change::load(ctx.fs, &layout, &config, change.clone())?;
    // An invalid role setting fails the status too: the skills read it
    // first, and must not go on with roles they cannot trust.
    engine_roles::resolve(ctx.fs, &layout, &config, Some(&ctxt))?;

    Ok(StatusOutcome {
        status: change::status(ctx.fs, &layout, &ctxt)?,
        planning_home: layout.project_root().to_path_buf(),
        change_root: layout.change_dir(&change),
        warnings: config.warnings,
        update: update_notice(ctx),
        requirement: version_requirement(ctx, config.codev_version.as_deref()),
    })
}

/// The update check of `codev status`: at most one request a day, silent
/// on failure, skipped in CI and on opt-out.
pub fn update_notice(ctx: &Ctx) -> Option<UpdateNotice> {
    engine_update::status_notice(ctx.fs, ctx.env, ctx.clock, ctx.http, ctx.version)
}

fn version_requirement(ctx: &Ctx, requirement: Option<&str>) -> Option<(String, bool)> {
    let requirement = requirement?;
    let satisfied = codev_core::update::satisfies(requirement, ctx.version).unwrap_or(false);
    Some((requirement.to_string(), satisfied))
}

// ─────────────────────────────── self-update ───────────────────────────────

#[derive(Debug, Clone, Default)]
pub struct SelfUpdateArgs {
    pub version: Option<String>,
    pub check: bool,
    pub snooze: bool,
}

/// `codev self-update`. `confirm` is asked once, before the download.
pub fn self_update(
    ctx: &Ctx,
    args: &SelfUpdateArgs,
    confirm: &mut dyn FnMut(&str, &str) -> bool,
) -> Result<engine_update::Outcome> {
    let ports = engine_update::Ports {
        fs: ctx.fs,
        env: ctx.env,
        clock: ctx.clock,
        http: ctx.http,
        runner: ctx.runner,
        exe: ctx.exe,
    };
    let request = engine_update::Request {
        current: ctx.version.to_string(),
        target: ctx.target.to_string(),
        version: args.version.clone(),
        check_only: args.check,
        snooze: args.snooze,
    };
    Ok(engine_update::self_update(&ports, &request, confirm)?)
}

// ─────────────────────────────── instructions ───────────────────────────────

/// The language of the current project, for a failure document that must
/// keep the `language` field: the configured one when the project and its
/// configuration can be read, the default otherwise. Never fails — it only
/// decorates an error already being reported.
pub fn configured_language(ctx: &Ctx) -> String {
    root::discover_from_cwd(ctx.fs, ctx.env)
        .ok()
        .and_then(|layout| config::resolve(ctx.fs, ctx.env, &layout).ok())
        .map_or_else(
            || codev_core::config::DEFAULT_LANGUAGE.to_string(),
            |config| config.language,
        )
}

pub fn artifact_instructions(
    ctx: &Ctx,
    artifact: Option<&str>,
    requested_change: Option<&str>,
) -> Result<Instructions> {
    let layout = root::discover_from_cwd(ctx.fs, ctx.env)?;
    let config = config::resolve(ctx.fs, ctx.env, &layout)?;
    let change = resolve_change(ctx.fs, &layout, requested_change)?;
    let ctxt = change::load(ctx.fs, &layout, &config, change)?;
    Ok(instructions::for_artifact(
        ctx.fs, ctx.env, &layout, &ctxt, &config, artifact,
    )?)
}

// ─────────────────────────────── roles ───────────────────────────────

/// How the `By stack` part of the served knowledge was filtered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StackFilter {
    /// Kept for the detected languages only.
    Detected,
    /// No language detected: kept whole.
    NoneDetected,
    /// `--all-stacks`: kept whole.
    AllStacks,
}

impl StackFilter {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Detected => "detected",
            Self::NoneDetected => "none-detected",
            Self::AllStacks => "all-stacks",
        }
    }
}

/// The project's languages and what was done with them.
#[derive(Debug, Clone)]
pub struct Stacks {
    pub detected: Vec<&'static str>,
    pub filter: StackFilter,
}

impl Default for Stacks {
    /// Nothing detected: the knowledge is served whole.
    fn default() -> Self {
        Self {
            detected: Vec::new(),
            filter: StackFilter::NoneDetected,
        }
    }
}

#[derive(Debug)]
pub struct RolesOutcome {
    /// `None` without `--change`: the project's roles.
    pub change: Option<ChangeId>,
    pub schema_name: String,
    /// The step asked with `--at`; `None` lists every role.
    pub at: Option<String>,
    /// For the display names.
    pub language: String,
    /// The roles active at `at`, or every role without it.
    pub roles: Vec<ResolvedRole>,
    /// `--knowledge`: the human output prints each role's knowledge.
    pub knowledge: bool,
    /// The languages the knowledge is filtered for.
    pub stacks: Stacks,
    /// With `--change`: the diff and the domains it suggests.
    pub suggestions: Option<engine_suggest::Suggestions>,
    pub warnings: Vec<Warning>,
}

/// `codev roles`: the roles of a step, or every role with its activation.
/// With `--change`, the change's schema and `change.yaml` apply, and the
/// change's diff is read for the domains it suggests. The `By stack` part
/// of the knowledge keeps the project's languages, unless `all_stacks`.
pub fn roles(
    ctx: &Ctx,
    at: Option<&str>,
    requested_change: Option<&str>,
    knowledge: bool,
    all_stacks: bool,
) -> Result<RolesOutcome> {
    let layout = root::discover_from_cwd(ctx.fs, ctx.env)?;
    let config = config::resolve(ctx.fs, ctx.env, &layout)?;
    let change_ctx = match requested_change {
        Some(name) => Some(change::load(
            ctx.fs,
            &layout,
            &config,
            ChangeId::parse(name)?,
        )?),
        None => None,
    };
    let resolved = engine_roles::resolve(ctx.fs, &layout, &config, change_ctx.as_ref())?;
    let mut roles: Vec<ResolvedRole> = match at {
        Some(step) => resolved.active_at(step)?.into_iter().cloned().collect(),
        None => resolved.roles.clone(),
    };

    let detected = engine_detect::stacks(ctx.fs, layout.project_root());
    let filter = if all_stacks {
        StackFilter::AllStacks
    } else if detected.is_empty() {
        StackFilter::NoneDetected
    } else {
        StackFilter::Detected
    };
    if filter == StackFilter::Detected {
        for role in &mut roles {
            if let Some(text) = &role.role.knowledge {
                role.role.knowledge = Some(codev_core::roles::filter_by_stack(text, &detected));
            }
        }
    }

    let mut warnings = config.warnings.clone();
    let suggestions = change_ctx.as_ref().map(|change| {
        let diff = engine_diff::read(ctx.fs, ctx.runner, layout.project_root());
        engine_suggest::for_change(ctx.fs, &layout, &config, change, &resolved, &diff)
    });
    if let Some(found) = &suggestions {
        warnings.extend(found.warnings.iter().cloned());
    }
    Ok(RolesOutcome {
        change: change_ctx.map(|c| c.change),
        schema_name: resolved.schema_name,
        at: at.map(str::to_string),
        language: config.language,
        roles,
        knowledge,
        stacks: Stacks { detected, filter },
        suggestions,
        warnings,
    })
}

// ─────────────────────────────── list / schemas ───────────────────────────────

#[derive(Debug)]
pub struct ChangesOutcome {
    pub root: PathBuf,
    pub changes: Vec<ChangeId>,
}

pub fn list_changes(ctx: &Ctx) -> Result<ChangesOutcome> {
    let layout = root::discover_from_cwd(ctx.fs, ctx.env)?;
    Ok(ChangesOutcome {
        changes: change::list(ctx.fs, &layout),
        root: layout.project_root().to_path_buf(),
    })
}

#[derive(Debug)]
pub struct SpecsOutcome {
    pub root: PathBuf,
    pub specs: Vec<String>,
}

pub fn list_specs(ctx: &Ctx) -> Result<SpecsOutcome> {
    let layout = root::discover_from_cwd(ctx.fs, ctx.env)?;
    Ok(SpecsOutcome {
        specs: specs::list(ctx.fs, &layout),
        root: layout.project_root().to_path_buf(),
    })
}

#[derive(Debug)]
pub struct SchemaSummary {
    pub name: String,
    pub origin: &'static str,
    pub flow: Vec<String>,
}

#[derive(Debug)]
pub struct SchemasOutcome {
    pub root: PathBuf,
    pub schemas: Vec<SchemaSummary>,
    pub warnings: Vec<Warning>,
}

pub fn list_schemas(ctx: &Ctx) -> Result<SchemasOutcome> {
    let layout = root::discover_from_cwd(ctx.fs, ctx.env)?;
    let mut summaries = Vec::new();
    let mut warnings = Vec::new();

    for (name, origin) in schemas::list(ctx.fs, &layout) {
        match schemas::resolve(ctx.fs, &layout, &name) {
            Ok(resolved) => {
                // The folder is the schema's name; a different `name:` is
                // harmless but misleading, so it is pointed out, not refused.
                if resolved.declared_name() != name {
                    warnings.push(Warning::new(
                        "schema_name_mismatch",
                        format!(
                            "schema `{name}` declares the name `{declared}`; codev names it after its folder, `{name}` — set `name: {name}` in its schema.yaml",
                            declared = resolved.declared_name()
                        ),
                    ));
                }
                summaries.push(SchemaSummary {
                    name,
                    origin: origin.label(),
                    flow: resolved
                        .graph
                        .topological_order()
                        .iter()
                        .map(|a| a.id.clone())
                        .collect(),
                });
            }
            // A broken custom schema must not prevent listing the others: it is
            // precisely the command one runs to understand what is going on.
            Err(err) => warnings.push(Warning::new(
                "schema_unusable",
                format!("schema `{name}` ({}) is unusable: {err}", origin.label()),
            )),
        }
    }

    Ok(SchemasOutcome {
        root: layout.project_root().to_path_buf(),
        schemas: summaries,
        warnings,
    })
}

// ─────────────────────────────── validate ───────────────────────────────

pub fn validate(ctx: &Ctx, scope: ValidateArgs) -> Result<ValidateReport> {
    let layout = root::discover_from_cwd(ctx.fs, ctx.env)?;
    let config = config::resolve(ctx.fs, ctx.env, &layout)?;
    let mut report = validate_items(ctx, &layout, &config, scope)?;
    add_suggested_domains(ctx, &layout, &config, &mut report);
    Ok(report)
}

/// Adds `domain_suggested` to each active change of `report`, for the
/// suggested domains its proposal neither ticks nor justifies. The diff is
/// read once, and only when the report holds a change: `codev validate
/// --specs` never runs git. Kept out of `engine::validate::validate_change`
/// so that the pre-flights of `sync` and `archive`, and `codev doctor`,
/// never read a diff.
fn add_suggested_domains(
    ctx: &Ctx,
    layout: &Layout,
    config: &config::ResolvedConfig,
    report: &mut ValidateReport,
) {
    let mut diff = None;
    for item in &mut report.items {
        if item.kind != engine_validate::ItemKind::Change {
            continue;
        }
        let Ok(change_id) = ChangeId::parse(&item.name) else {
            continue;
        };
        let proposal_path = layout.change_dir(&change_id).join("proposal.md");
        let Ok(proposal) = ctx.fs.read_to_string(&proposal_path) else {
            continue;
        };
        // A change whose roles cannot be resolved already carries the
        // finding that says why.
        let Ok(change_ctx) = change::load(ctx.fs, layout, config, change_id) else {
            continue;
        };
        let Ok(roles) = engine_roles::resolve(ctx.fs, layout, config, Some(&change_ctx)) else {
            continue;
        };
        let diff = diff
            .get_or_insert_with(|| engine_diff::read(ctx.fs, ctx.runner, layout.project_root()));
        let found = engine_suggest::for_change(ctx.fs, layout, config, &change_ctx, &roles, diff);
        let heading = codev_core::domains::heading(&proposal);
        let relative = proposal_path
            .strip_prefix(layout.project_root())
            .unwrap_or(&proposal_path)
            .to_path_buf();
        for suggestion in &found.suggestions {
            if suggestion.state == codev_core::triggers::State::Unticked {
                item.findings
                    .push(engine_validate::LocatedFinding::from_finding(
                        codev_core::triggers::finding(suggestion, heading),
                        relative.clone(),
                    ));
            }
        }
    }
}

fn validate_items(
    ctx: &Ctx,
    layout: &Layout,
    config: &config::ResolvedConfig,
    scope: ValidateArgs,
) -> Result<ValidateReport> {
    match scope {
        ValidateArgs::All => Ok(engine_validate::validate_all(
            ctx.fs, ctx.env, layout, config,
        )?),
        ValidateArgs::Changes => {
            let mut items = Vec::new();
            for change_id in change::list(ctx.fs, layout) {
                items.push(engine_validate::validate_change(
                    ctx.fs, layout, config, &change_id,
                )?);
            }
            Ok(ValidateReport {
                root: layout.project_root().to_path_buf(),
                items,
            })
        }
        ValidateArgs::Specs => {
            let mut items = Vec::new();
            for capability in specs::list(ctx.fs, layout) {
                items.push(engine_validate::validate_spec(ctx.fs, layout, &capability)?);
            }
            Ok(ValidateReport {
                root: layout.project_root().to_path_buf(),
                items,
            })
        }
        ValidateArgs::Item(name) => {
            let item = resolve_validate_item(ctx.fs, layout, config, &name)?;
            Ok(ValidateReport {
                root: layout.project_root().to_path_buf(),
                items: vec![item],
            })
        }
    }
}

/// How `validate` was called. `Item` carries a `String` rather than a `&str`
/// because the name may come from a CLI argument owned by the CLI — a `&str`
/// would force its lifetime to be propagated.
#[derive(Debug, Clone)]
pub enum ValidateArgs {
    All,
    Changes,
    Specs,
    Item(String),
}

fn resolve_validate_item(
    fs: &dyn FileSystem,
    layout: &Layout,
    config: &config::ResolvedConfig,
    name: &str,
) -> Result<engine_validate::ItemReport> {
    // The same name may designate a change AND a spec capability — in that
    // case refuse rather than guess, naming the candidates.
    let changes: Vec<ChangeId> = change::list(fs, layout)
        .into_iter()
        .filter(|c| c.as_str() == name)
        .collect();
    let matching_specs: Vec<String> = specs::list(fs, layout)
        .into_iter()
        .filter(|s| s == name)
        .collect();

    match (changes.len(), matching_specs.len()) {
        (1, 0) => Ok(engine_validate::validate_change(
            fs,
            layout,
            config,
            &changes[0],
        )?),
        (0, 1) => Ok(engine_validate::validate_spec(
            fs,
            layout,
            &matching_specs[0],
        )?),
        (0, 0) => Err(Failure::new(
            "unknown_item",
            format!("no change or spec is named `{name}`"),
        )
        .with_fix("`codev list` and `codev list --specs` show what exists")),
        _ => Err(Failure::new(
            "ambiguous_item",
            format!(
                "`{name}` refers to both a change and a spec; narrow it down with `--changes` or `--specs`"
            ),
        )),
    }
}

// ─────────────────────────────── sync / archive ───────────────────────────────

pub fn sync(ctx: &Ctx, requested: Option<&str>) -> Result<SyncOutcome> {
    let layout = root::discover_from_cwd(ctx.fs, ctx.env)?;
    let config = config::resolve(ctx.fs, ctx.env, &layout)?;
    let change = resolve_change(ctx.fs, &layout, requested)?;
    Ok(engine_sync::execute_sync(
        ctx.fs, &layout, &config, &change,
    )?)
}

pub fn archive(ctx: &Ctx, requested: Option<&str>) -> Result<ArchiveOutcome> {
    let layout = root::discover_from_cwd(ctx.fs, ctx.env)?;
    let config = config::resolve(ctx.fs, ctx.env, &layout)?;
    let change = resolve_change(ctx.fs, &layout, requested)?;
    Ok(engine_archive::execute_archive(
        ctx.fs, &layout, &config, ctx.clock, &change,
    )?)
}

// ─────────────────────────────── review ───────────────────────────────

#[derive(Debug)]
pub struct ReviewOutcome {
    pub change: ChangeId,
    pub check: engine_review::ReviewCheck,
    pub warnings: Vec<Warning>,
}

impl ReviewOutcome {
    /// 1 when the record has an error, 0 otherwise: the archive gate is a
    /// state of the change, read from the output and enforced by
    /// `codev archive`, not a failure of this command.
    pub fn exit_code(&self) -> i32 {
        i32::from(self.check.has_errors())
    }
}

/// `codev review`: the change's review record, its defects, and whether it
/// lets the change be archived.
pub fn review(ctx: &Ctx, requested: Option<&str>) -> Result<ReviewOutcome> {
    let layout = root::discover_from_cwd(ctx.fs, ctx.env)?;
    let config = config::resolve(ctx.fs, ctx.env, &layout)?;
    let change = resolve_change(ctx.fs, &layout, requested)?;
    let change_ctx = change::load(ctx.fs, &layout, &config, change.clone())?;
    Ok(ReviewOutcome {
        check: engine_review::check(ctx.fs, &layout, &config, &change_ctx)?,
        change,
        warnings: config.warnings,
    })
}

// ─────────────────────────────── decision ───────────────────────────────

/// Enriched description of an index entry — surfaces whether it is in
/// effect and its supersession, for rendering.
#[derive(Debug)]
pub struct DecisionSummary {
    pub id: String,
    pub qualified_id: String,
    pub title: String,
    pub status: String,
    pub date: String,
    pub tags: Vec<String>,
    pub supersedes: Vec<String>,
    /// Local deviations — additive. Empty on older ADRs.
    pub deviates_from: Vec<String>,
    pub path: PathBuf,
    pub origin: String,
    pub in_effect: bool,
    pub superseded_by: Option<String>,
    /// Set for inherited entries that a local ADR sets aside via
    /// `deviates_from`. Additive — `None` otherwise.
    pub deviated_by: Option<String>,
}

pub struct DecisionListOutcome {
    pub root: PathBuf,
    pub decisions: Vec<DecisionSummary>,
}

pub fn decision_list(ctx: &Ctx) -> Result<DecisionListOutcome> {
    let layout = root::discover_from_cwd(ctx.fs, ctx.env)?;
    let cfg = config::resolve(ctx.fs, ctx.env, &layout)?;
    let index = engine_decisions::index(ctx.fs, ctx.env, &layout, &cfg)?;
    Ok(DecisionListOutcome {
        decisions: build_summaries(&index, &layout),
        root: layout.project_root().to_path_buf(),
    })
}

pub struct DecisionShowOutcome {
    pub root: PathBuf,
    pub decision: DecisionSummary,
    pub content: String,
}

pub fn decision_show(ctx: &Ctx, id: &str) -> Result<DecisionShowOutcome> {
    let layout = root::discover_from_cwd(ctx.fs, ctx.env)?;
    let cfg = config::resolve(ctx.fs, ctx.env, &layout)?;
    let index = engine_decisions::index(ctx.fs, ctx.env, &layout, &cfg)?;
    let (idx, _) = engine_actions::resolve_old_entry(&index, id).map_err(|err| Failure {
        code: err.code().to_string(),
        message: err.to_string(),
        fix: None,
    })?;
    let summaries = build_summaries(&index, &layout);
    let entry = &index.entries[idx];
    let content = ctx
        .fs
        .read_to_string(&entry.path)
        .map_err(|e| Failure::new("unreadable", format!("{}: {e}", entry.path.display())))?;
    // Find the matching summary.
    let summary = summaries
        .into_iter()
        .find(|s| s.qualified_id == entry.qualified_id.as_str())
        .expect("the resolved entry comes from the index");
    Ok(DecisionShowOutcome {
        root: layout.project_root().to_path_buf(),
        decision: summary,
        content,
    })
}

pub struct DecisionCreatedOutcome {
    pub root: PathBuf,
    pub decision: DecisionSummary,
    pub path: PathBuf,
    /// Hash of the new ADR's body — `None` if the status is not sealed
    /// (proposed, deprecated, rejected).
    pub body_sha256: Option<String>,
}

pub fn decision_new(ctx: &Ctx, title: &str, status_raw: &str) -> Result<DecisionCreatedOutcome> {
    let layout = root::discover_from_cwd(ctx.fs, ctx.env)?;
    let cfg = config::resolve(ctx.fs, ctx.env, &layout)?;
    let index = engine_decisions::index(ctx.fs, ctx.env, &layout, &cfg)?;
    let status = codev_core::decisions::DecisionStatus::from_raw(status_raw);
    let today = ctx.clock.today();
    let seal_file = engine_actions::read_seal_file(ctx.fs, &layout).map_err(action_to_failure)?;
    let create_plan =
        engine_actions::plan_new(&index, &seal_file, title, status.clone(), &today, &layout)
            .map_err(action_to_failure)?;
    apply::execute(&create_plan.plan, ctx.fs)?;

    // Only surface the hash if the ADR is actually sealed (`accepted` /
    // `superseded` statuses). For the others, `body_sha256` stays `None` —
    // an additive field of the JSON contract.
    let body_sha256 = if matches!(
        status,
        codev_core::decisions::DecisionStatus::Accepted
            | codev_core::decisions::DecisionStatus::Superseded
    ) {
        Some(create_plan.body_sha256.clone())
    } else {
        None
    };

    // Read back to build an up-to-date summary. The comparison is made on the
    // qualified identifier — the summary's `path` is relative to the
    // project, the plan's is absolute; they would never match as is.
    let new_qualified = format!("project/{}", create_plan.new_id);
    let index_after = engine_decisions::index(ctx.fs, ctx.env, &layout, &cfg)?;
    let summary = build_summaries(&index_after, &layout)
        .into_iter()
        .find(|s| s.qualified_id == new_qualified)
        .ok_or_else(|| {
            Failure::new(
                "write_failed",
                "the created decision could not be read back",
            )
        })?;
    Ok(DecisionCreatedOutcome {
        root: layout.project_root().to_path_buf(),
        decision: summary,
        path: create_plan.new_path,
        body_sha256,
    })
}

#[derive(Debug)]
pub struct DecisionSupersededOutcome {
    pub root: PathBuf,
    pub new_decision: DecisionSummary,
    pub new_path: PathBuf,
    pub old_id: String,
    pub old_qualified_id: String,
    pub old_path: PathBuf,
}

pub fn decision_supersede(
    ctx: &Ctx,
    old_id: &str,
    new_title: &str,
) -> Result<DecisionSupersededOutcome> {
    let layout = root::discover_from_cwd(ctx.fs, ctx.env)?;
    let cfg = config::resolve(ctx.fs, ctx.env, &layout)?;
    let index = engine_decisions::index(ctx.fs, ctx.env, &layout, &cfg)?;
    let today = ctx.clock.today();
    // No seal is read nor written: the new decision is `proposed`, and the
    // old one is only marked superseded by `decision accept`.
    let plan = engine_actions::plan_supersede(&index, old_id, new_title, &today, &layout)
        .map_err(action_to_failure)?;

    let old_id_str = old_id.split('/').next_back().unwrap_or(old_id).to_string();
    let old_qualified_id = plan.old_qualified_id.clone();
    let old_path = plan.old_path.clone();
    let new_path = plan.new_path.clone();

    let new_id = plan.new_id.clone();
    apply::execute(&plan.plan, ctx.fs)?;

    let new_qualified = format!("project/{new_id}");
    let index_after = engine_decisions::index(ctx.fs, ctx.env, &layout, &cfg)?;
    let new_summary = build_summaries(&index_after, &layout)
        .into_iter()
        .find(|s| s.qualified_id == new_qualified)
        .ok_or_else(|| {
            Failure::new(
                "write_failed",
                "the created decision could not be read back",
            )
        })?;
    Ok(DecisionSupersededOutcome {
        root: layout.project_root().to_path_buf(),
        new_decision: new_summary,
        new_path,
        old_id: old_id_str,
        old_qualified_id,
        old_path,
    })
}

fn action_to_failure(err: engine_actions::ActionError) -> Failure {
    Failure::new(err.code().to_string(), err.to_string())
}

#[derive(Debug)]
pub struct DecisionDeviatedOutcome {
    pub root: PathBuf,
    pub decision: DecisionSummary,
    pub path: PathBuf,
    /// The qualified identifier of the decision being set aside — the same
    /// form that `codev decision show` accepts.
    pub target_qualified_id: String,
}

pub fn decision_deviate(
    ctx: &Ctx,
    target: &str,
    new_title: &str,
) -> Result<DecisionDeviatedOutcome> {
    let layout = root::discover_from_cwd(ctx.fs, ctx.env)?;
    let cfg = config::resolve(ctx.fs, ctx.env, &layout)?;
    let index = engine_decisions::index(ctx.fs, ctx.env, &layout, &cfg)?;
    let today = ctx.clock.today();

    let plan = engine_actions::plan_deviate(&index, target, new_title, &today, &layout)
        .map_err(action_to_failure)?;

    let target_qualified_id = plan.target_qualified_id.clone();
    let new_path = plan.new_path.clone();
    let new_id = plan.new_id.clone();

    apply::execute(&plan.plan, ctx.fs)?;

    // Read back to build an up-to-date summary, exactly like
    // decision_new/supersede.
    let new_qualified = format!("project/{new_id}");
    let index_after = engine_decisions::index(ctx.fs, ctx.env, &layout, &cfg)?;
    let summary = build_summaries(&index_after, &layout)
        .into_iter()
        .find(|s| s.qualified_id == new_qualified)
        .ok_or_else(|| {
            Failure::new(
                "write_failed",
                "the created deviation could not be read back",
            )
        })?;

    Ok(DecisionDeviatedOutcome {
        root: layout.project_root().to_path_buf(),
        decision: summary,
        path: new_path,
        target_qualified_id,
    })
}

#[derive(Debug)]
pub struct DecisionPromotedOutcome {
    pub root: PathBuf,
    pub decision: DecisionSummary,
    pub path: PathBuf,
    /// The change the promotion comes from.
    pub source_change: String,
    /// The `design.md` that was updated (absolute path).
    pub design_path: PathBuf,
}

pub fn decision_promote(
    ctx: &Ctx,
    change_name: &str,
    heading: &str,
) -> Result<DecisionPromotedOutcome> {
    let layout = root::discover_from_cwd(ctx.fs, ctx.env)?;
    let cfg = config::resolve(ctx.fs, ctx.env, &layout)?;

    // Change resolution — refuses a missing or archived folder upfront, with
    // a dedicated stable code for the archived case.
    let change_id = ChangeId::parse(change_name).map_err(Failure::from)?;
    let change_dir = layout.change_dir(&change_id);
    if !ctx.fs.exists(&change_dir) {
        if is_change_in_archive(ctx.fs, &layout, change_name) {
            return Err(Failure::new(
                "cannot_promote_from_archived",
                format!(
                    "change `{change_name}` is archived: an archived design \
                     is history; promotion happens before archiving"
                ),
            ));
        }
        return Err(Failure::new(
            "unknown_change",
            format!("change `{change_name}` does not exist"),
        ));
    }

    // The change's design.md — required to promote.
    let design_path = change_dir.join("design.md");
    if !ctx.fs.exists(&design_path) {
        return Err(Failure::new(
            "design_missing",
            format!(
                "change `{change_name}` has no `design.md` — nothing to \
                 promote; create it first with `codev-propose` or by hand"
            ),
        ));
    }
    let design_source = ctx.fs.read_to_string(&design_path).map_err(|e| {
        Failure::new(
            "read_failed",
            format!("cannot read {}: {e}", design_path.display()),
        )
    })?;

    let index = engine_decisions::index(ctx.fs, ctx.env, &layout, &cfg)?;
    let today = ctx.clock.today();

    let plan = engine_actions::plan_promote(
        &index,
        change_name,
        heading,
        &design_source,
        design_path.clone(),
        &today,
        &layout,
    )
    .map_err(action_to_failure)?;

    let new_path = plan.new_path.clone();
    let source_change = plan.source_change.clone();
    let new_id = plan.new_id.clone();

    apply::execute(&plan.plan, ctx.fs)?;

    let new_qualified = format!("project/{new_id}");
    let index_after = engine_decisions::index(ctx.fs, ctx.env, &layout, &cfg)?;
    let summary = build_summaries(&index_after, &layout)
        .into_iter()
        .find(|s| s.qualified_id == new_qualified)
        .ok_or_else(|| {
            Failure::new(
                "write_failed",
                "the promoted decision could not be read back",
            )
        })?;

    Ok(DecisionPromotedOutcome {
        root: layout.project_root().to_path_buf(),
        decision: summary,
        path: new_path,
        source_change,
        design_path,
    })
}

/// Looks for a `<date>-<change_name>` folder under
/// `_codev/changes/archive/`. Used to tell "missing change" apart from
/// "archived change".
fn is_change_in_archive(
    fs: &dyn codev_engine::FileSystem,
    layout: &Layout,
    change_name: &str,
) -> bool {
    let archive = layout.archive_dir();
    let Ok(entries) = fs.list_dir(&archive) else {
        return false;
    };
    let suffix = format!("-{change_name}");
    entries.iter().any(|name| name.ends_with(&suffix))
}

#[derive(Debug)]
pub struct DecisionSealedOutcome {
    pub root: PathBuf,
    pub id: String,
    pub body_sha256: String,
    pub sealed_at: String,
    pub was_noop: bool,
    /// `true` if the seal was rewritten via `--force` (useful for human
    /// rendering: "sealed" vs "resealed" vs "already up to date").
    pub was_forced: bool,
}

pub fn decision_seal(ctx: &Ctx, id: &str, force: bool) -> Result<DecisionSealedOutcome> {
    let layout = root::discover_from_cwd(ctx.fs, ctx.env)?;
    let cfg = config::resolve(ctx.fs, ctx.env, &layout)?;
    let index = engine_decisions::index(ctx.fs, ctx.env, &layout, &cfg)?;
    let seal_file = engine_actions::read_seal_file(ctx.fs, &layout).map_err(action_to_failure)?;
    let today = ctx.clock.today();

    // Note whether there already was an entry, to distinguish, when
    // rendering, "freshly sealed" from "resealed with force".
    let had_previous_entry = seal_file.find(id).is_some();

    let plan = engine_actions::plan_seal(&index, &seal_file, id, force, &today, &layout, |path| {
        ctx.fs.read_to_string(path)
    })
    .map_err(action_to_failure)?;

    apply::execute(&plan.plan, ctx.fs)?;

    Ok(DecisionSealedOutcome {
        root: layout.project_root().to_path_buf(),
        id: plan.id,
        body_sha256: plan.body_sha256,
        sealed_at: plan.sealed_at,
        was_noop: plan.was_noop,
        was_forced: had_previous_entry && !plan.was_noop,
    })
}

#[derive(Debug)]
pub struct DecisionAcceptedOutcome {
    pub root: PathBuf,
    pub decision: DecisionSummary,
    pub path: PathBuf,
    pub body_sha256: String,
    /// The predecessors marked `superseded` by the same plan — empty when
    /// the decision supersedes nothing.
    pub superseded: Vec<engine_actions::SupersededPredecessor>,
}

pub fn decision_accept(ctx: &Ctx, id: &str) -> Result<DecisionAcceptedOutcome> {
    let layout = root::discover_from_cwd(ctx.fs, ctx.env)?;
    let cfg = config::resolve(ctx.fs, ctx.env, &layout)?;
    let index = engine_decisions::index(ctx.fs, ctx.env, &layout, &cfg)?;
    let seal_file = engine_actions::read_seal_file(ctx.fs, &layout).map_err(action_to_failure)?;
    let today = ctx.clock.today();
    let plan = engine_actions::plan_accept(&index, &seal_file, id, &today, &layout, |path| {
        ctx.fs.read_to_string(path)
    })
    .map_err(action_to_failure)?;
    apply::execute(&plan.plan, ctx.fs)?;

    // Read back, as `decision new` does, so the summary reflects the
    // rewritten status and the effect computed with it.
    let qualified = format!("project/{}", plan.id);
    let index_after = engine_decisions::index(ctx.fs, ctx.env, &layout, &cfg)?;
    let summary = build_summaries(&index_after, &layout)
        .into_iter()
        .find(|s| s.qualified_id == qualified)
        .ok_or_else(|| {
            Failure::new(
                "write_failed",
                "the accepted decision could not be read back",
            )
        })?;
    Ok(DecisionAcceptedOutcome {
        root: layout.project_root().to_path_buf(),
        decision: summary,
        path: plan.path,
        body_sha256: plan.body_sha256,
        superseded: plan.superseded,
    })
}

/// Converts the index into `DecisionSummary` values, with effect and
/// supersession.
fn build_summaries(
    index: &engine_decisions::DecisionIndex,
    layout: &Layout,
) -> Vec<DecisionSummary> {
    // Precompute who supersedes whom: `superseded_by[id] = qualified_id` of
    // the superseding decision.
    let mut superseded_by = std::collections::BTreeMap::<String, String>::new();
    for entry in &index.entries {
        if matches!(
            entry.decision.status,
            codev_core::decisions::DecisionStatus::Accepted
        ) {
            for target in &entry.decision.supersedes {
                superseded_by.insert(target.clone(), entry.qualified_id.as_str());
            }
        }
    }
    let in_effect_set: std::collections::BTreeSet<String> =
        index.in_effect.iter().map(|q| q.as_str()).collect();

    index
        .entries
        .iter()
        .map(|entry| {
            let qualified = entry.qualified_id.as_str();
            let status = match &entry.decision.status {
                codev_core::decisions::DecisionStatus::Unknown(raw) => raw.clone(),
                other => other.as_str().to_string(),
            };
            let relative = entry
                .path
                .strip_prefix(layout.project_root())
                .map(|p| p.to_path_buf())
                .unwrap_or_else(|_| entry.path.clone());
            DecisionSummary {
                id: entry.decision.id.clone(),
                qualified_id: qualified.clone(),
                title: entry.decision.title.clone(),
                status,
                date: entry.decision.date.clone(),
                tags: entry.decision.tags.clone(),
                supersedes: entry.decision.supersedes.clone(),
                deviates_from: entry.decision.deviates_from.clone(),
                path: relative,
                origin: match &entry.qualified_id.origin {
                    engine_decisions::Origin::Project => "project".into(),
                    engine_decisions::Origin::Path(raw) => format!("path:{raw}"),
                    engine_decisions::Origin::Git(url) => format!("git:{url}"),
                },
                in_effect: in_effect_set.contains(&qualified),
                superseded_by: superseded_by.get(&entry.decision.id).cloned(),
                deviated_by: entry.deviated_by.as_ref().map(|q| q.as_str()),
            }
        })
        .collect()
}

// ─────────────────────────────── sources ───────────────────────────────

pub struct SourcesListOutcome {
    pub root: PathBuf,
    pub sources: Vec<engine_sources::SourceStatus>,
}

pub fn sources_list(ctx: &Ctx) -> Result<SourcesListOutcome> {
    let layout = root::discover_from_cwd(ctx.fs, ctx.env)?;
    Ok(SourcesListOutcome {
        sources: engine_sources::list_source_states(ctx.fs, ctx.env, &layout)?,
        root: layout.project_root().to_path_buf(),
    })
}

pub struct SourcesUpdateOutcome {
    pub root: PathBuf,
    pub diff: Vec<engine_sources::PinChange>,
    pub lock_written: bool,
}

pub fn sources_update(ctx: &Ctx) -> Result<SourcesUpdateOutcome> {
    let layout = root::discover_from_cwd(ctx.fs, ctx.env)?;
    let cfg_path = layout.config_file();
    let project = config::load(ctx.fs, &cfg_path)?.unwrap_or_default();
    let sources = engine_sources::GitSourceInput::from_inherits(&project.inherits);
    // With `init --inherit`, the only command that drives `git`.
    let outcome = engine_sources::run_sources_update(
        ctx.fs, ctx.env, ctx.runner, ctx.clock, &layout, &sources,
    )?;
    Ok(SourcesUpdateOutcome {
        root: outcome.root,
        diff: outcome.diff,
        lock_written: outcome.lock_written,
    })
}

pub struct SourcesShowOutcome {
    pub root: PathBuf,
    pub source: engine_sources::SourceStatus,
    pub files_exposed: Vec<String>,
}

pub fn sources_show(ctx: &Ctx, target: &str) -> Result<SourcesShowOutcome> {
    let layout = root::discover_from_cwd(ctx.fs, ctx.env)?;
    let states = engine_sources::list_source_states(ctx.fs, ctx.env, &layout)?;
    let source = states
        .into_iter()
        .find(|s| s.address == target)
        .ok_or_else(|| {
            Failure::new(
                "unknown_source",
                format!("no declared source has the address `{target}`"),
            )
        })?;
    let files_exposed = if let Some(path) = &source.resolved_path {
        engine_sources::list_files_exposed(ctx.fs, path)
    } else {
        Vec::new()
    };
    Ok(SourcesShowOutcome {
        root: layout.project_root().to_path_buf(),
        source,
        files_exposed,
    })
}

// ─────────────────────────────── changelog ───────────────────────────────

/// The text format of `codev changelog`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChangelogFormat {
    KeepAChangelog,
    Markdown,
    Json,
}

#[derive(Debug, Clone, Default)]
pub struct ChangelogArgs {
    pub since: Option<String>,
    pub until: Option<String>,
    pub release: Option<String>,
    pub date: Option<String>,
    pub write: bool,
}

/// What `--write` did to `CHANGELOG.md`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChangelogWrite {
    Created,
    Updated,
    Unchanged,
}

impl ChangelogWrite {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Created => "created",
            Self::Updated => "updated",
            Self::Unchanged => "unchanged",
        }
    }
}

pub struct ChangelogOutcome {
    pub root: PathBuf,
    pub since: Option<String>,
    pub until: Option<String>,
    /// `(version, date)` with `--release`.
    pub release: Option<(String, String)>,
    pub entries: Vec<changelog::Entry>,
    /// The entries rendered in the requested text format (Keep a Changelog
    /// for `json`).
    pub rendered: String,
    pub written: Option<(PathBuf, ChangelogWrite)>,
    pub warnings: Vec<Warning>,
}

/// `codev changelog`: the release notes of the changes archived in range.
pub fn changelog(
    ctx: &Ctx,
    format: ChangelogFormat,
    args: &ChangelogArgs,
) -> Result<ChangelogOutcome> {
    if args.write && format == ChangelogFormat::Markdown {
        return Err(Failure::new(
            "invalid_argument",
            "--write writes the Keep a Changelog format; drop `--format markdown`",
        ));
    }
    let release = match &args.release {
        Some(raw) => {
            let version = changelog::version::normalize_release(raw)
                .map_err(|m| Failure::new("invalid_release_version", m))?;
            let date = match &args.date {
                Some(date) if !changelog::version::is_valid_date(date) => {
                    return Err(Failure::new(
                        "invalid_date",
                        format!("`{date}` is not a date; use YYYY-MM-DD"),
                    ));
                }
                Some(date) => date.clone(),
                None => ctx.clock.today(),
            };
            Some((version, date))
        }
        None => None,
    };

    let layout = root::discover_from_cwd(ctx.fs, ctx.env)?;
    let generated = generate_changelog(ctx, &layout, args.since.as_deref(), args.until.as_deref())?;
    let heading = match (&release, format) {
        (_, ChangelogFormat::Markdown) => changelog::Heading::None,
        (Some((version, date)), _) => changelog::Heading::Release {
            version: version.clone(),
            date: date.clone(),
        },
        (None, _) => changelog::Heading::Unreleased,
    };
    let rendered = changelog::render(&generated.entries, &heading);

    let written = if args.write {
        let path = layout.project_root().join(changelog::file::FILE_NAME);
        let existing = if ctx.fs.exists(&path) {
            Some(ctx.fs.read_to_string(&path).map_err(|e| {
                Failure::new(
                    "unreadable",
                    format!("{} is unreadable: {e}", path.display()),
                )
            })?)
        } else {
            None
        };
        let unreleased = changelog::render(&generated.entries, &changelog::Heading::Unreleased);
        let release_block = release.is_some().then(|| rendered.clone());
        let contents = changelog::file::write(
            existing.as_deref(),
            if release.is_some() { "" } else { &unreleased },
            release_block.as_deref(),
        )
        .map_err(|reason| {
            Failure::new(
                "changelog_malformed",
                format!(
                    "{}: {reason}; fix the codev markers, nothing was written",
                    path.display()
                ),
            )
        })?;
        let action = match &existing {
            None => ChangelogWrite::Created,
            Some(text) if *text == contents => ChangelogWrite::Unchanged,
            Some(_) => ChangelogWrite::Updated,
        };
        if action != ChangelogWrite::Unchanged {
            let mut plan = Plan::new();
            plan.write(&path, contents, WriteMode::Overwrite);
            apply::execute(&plan, ctx.fs)?;
        }
        Some((path, action))
    } else {
        None
    };

    Ok(ChangelogOutcome {
        root: layout.project_root().to_path_buf(),
        since: generated.since,
        until: generated.until,
        release,
        entries: generated.entries,
        rendered,
        written,
        warnings: generated.warnings,
    })
}

/// The entries in range, with every warning — the shared core of
/// `codev changelog` and the doctor `changelog` check.
struct GeneratedChangelog {
    since: Option<String>,
    until: Option<String>,
    entries: Vec<changelog::Entry>,
    warnings: Vec<Warning>,
}

fn generate_changelog(
    ctx: &Ctx,
    layout: &Layout,
    since: Option<&str>,
    until: Option<&str>,
) -> Result<GeneratedChangelog> {
    let project = config::load(ctx.fs, &layout.config_file())?.unwrap_or_default();
    let gathered = engine_changelog::gather(ctx.fs, ctx.runner, layout, since, until)?;
    let collected = changelog::collect(&gathered.changes, &project.changelog.exclude);
    let mut warnings = gathered.warnings;
    warnings.extend(
        collected
            .notices
            .into_iter()
            .map(|n| Warning::new(n.code, n.message)),
    );
    Ok(GeneratedChangelog {
        since: gathered.since,
        until: gathered.until,
        entries: collected.entries,
        warnings,
    })
}

/// How `CHANGELOG.md` compares with a fresh `codev changelog --write`.
/// Anything that prevents the comparison counts as outdated: doctor
/// reports, it never fails on a fact it could not gather.
fn changelog_state(ctx: &Ctx, layout: &Layout) -> ChangelogState {
    let path = layout.project_root().join(changelog::file::FILE_NAME);
    let Ok(text) = ctx.fs.read_to_string(&path) else {
        return if ctx.fs.exists(&path) {
            ChangelogState::Outdated
        } else {
            ChangelogState::Missing
        };
    };
    let Some(current) = changelog::file::managed_block(&text) else {
        return ChangelogState::Outdated;
    };
    match generate_changelog(ctx, layout, None, None) {
        Ok(generated) => {
            let expected = changelog::render(&generated.entries, &changelog::Heading::Unreleased);
            if expected.trim() == current {
                ChangelogState::Current
            } else {
                ChangelogState::Outdated
            }
        }
        Err(_) => ChangelogState::Outdated,
    }
}

// ─────────────────────────────── doctor ───────────────────────────────

pub struct DoctorOutcome {
    pub root: PathBuf,
    pub checks: Vec<Check>,
}

impl DoctorOutcome {
    pub fn missing(&self) -> usize {
        core_doctor::summary(&self.checks).1
    }
}

/// `codev doctor`: read-only, offline. Gathers the facts through the
/// engine and the Claude Code target, and lets the core judge them.
pub fn doctor(ctx: &Ctx) -> Result<DoctorOutcome> {
    let layout = root::discover_from_cwd(ctx.fs, ctx.env)?;
    doctor_at(ctx, &layout)
}

fn doctor_at(ctx: &Ctx, layout: &Layout) -> Result<DoctorOutcome> {
    let root = layout.project_root();
    let project = config::load(ctx.fs, &layout.config_file())?.unwrap_or_default();
    let resolved = config::resolve(ctx.fs, ctx.env, layout)?;
    let (selected, _) = workflows::select(resolved.workflows.as_deref());

    let index = engine_decisions::index(ctx.fs, ctx.env, layout, &resolved)?;
    let accepted_decisions = index
        .entries
        .iter()
        .filter(|e| {
            matches!(
                e.decision.status,
                codev_core::decisions::DecisionStatus::Accepted
            )
        })
        .count();

    let states = engine_sources::list_source_states(ctx.fs, ctx.env, layout)?;
    let git_sources: Vec<GitSourceFact> = states
        .iter()
        .filter(|s| s.kind == engine_sources::SourceKind::Git)
        .map(|s| GitSourceFact {
            url: s.address.clone(),
            pinned: s.state != engine_sources::SourceState::Unlocked,
            cached: s.state == engine_sources::SourceState::Locked,
        })
        .collect();
    let declared = engine_sources::GitSourceInput::from_inherits(&project.inherits);
    let lock = engine_sources::lockfile::load(ctx.fs, &layout.planning_dir().join("codev.lock"))?;
    let stale_pins: Vec<String> = lock
        .map(|lock| {
            lock.sources
                .into_iter()
                .filter(|e| {
                    !declared
                        .iter()
                        .any(|d| d.git == e.git && d.git_ref == e.git_ref)
                })
                .map(|e| format!("{}@{}", e.git, e.git_ref))
                .collect()
        })
        .unwrap_or_default();

    let repo = integration::RepoFacts::probe(ctx.fs, root);
    let claude_md_section = ctx
        .fs
        .read_to_string(&root.join(codev_agents::claude::CLAUDE_MD))
        .is_ok_and(|text| managed::has_section(&text));

    let report = engine_validate::validate_all(ctx.fs, ctx.env, layout, &resolved)?;
    let findings = report.items.iter().flat_map(|i| i.findings.iter());
    let validate_errors = findings.clone().filter(|f| f.is_error()).count();
    let validate_warnings = findings
        .filter(|f| f.finding.severity == codev_core::parser::ast::Severity::Warning)
        .count();

    let cached = engine_update::cached_notice(ctx.fs, ctx.env, ctx.clock, ctx.version);
    let facts = DoctorFacts {
        rules_present: resolved.rules.values().any(|blocks| !blocks.is_empty()),
        language_set: project.language.is_some(),
        unknown_config_keys: resolved
            .unknown_keys
            .iter()
            .map(|u| u.key.clone())
            .collect(),
        roles: roles_facts(ctx, layout, &resolved),
        skills: ClaudeCode::skill_states(ctx.fs, root, &selected, ctx.version),
        main_specs: specs::list(ctx.fs, layout).len(),
        accepted_decisions,
        git_sources,
        stale_pins,
        claude_md_section,
        ci_validating_files: repo.ci_files_running_validate(),
        is_github: repo.is_github,
        is_gitlab: repo.is_gitlab,
        pr_template_section: repo
            .pr_template
            .as_ref()
            .is_some_and(|(_, text)| managed::has_section(text)),
        validate_errors,
        validate_warnings,
        changelog: changelog_state(ctx, layout),
        // The cache only: doctor stays offline.
        version: VersionFacts {
            running: ctx.version.to_string(),
            requirement: version_requirement(ctx, resolved.codev_version.as_deref()),
            newer_release: cached
                .as_ref()
                .filter(|n| n.available)
                .map(|n| n.latest.clone()),
            latest_known: cached.map(|n| n.latest),
        },
    };
    Ok(DoctorOutcome {
        root: root.to_path_buf(),
        checks: core_doctor::evaluate(&facts),
    })
}

/// The roles in effect for `config.roles`: the invalid settings when there
/// are some, otherwise the active and disabled roles. Empty when the
/// configured schema cannot be resolved — other checks report that.
fn roles_facts(ctx: &Ctx, layout: &Layout, config: &config::ResolvedConfig) -> RolesFacts {
    let invalid: Vec<String> = engine_roles::project_findings(ctx.fs, layout, config)
        .into_iter()
        .map(|finding| finding.role)
        .collect();
    if !invalid.is_empty() {
        return RolesFacts {
            invalid,
            ..RolesFacts::default()
        };
    }
    let Ok(resolved) = engine_roles::resolve(ctx.fs, layout, config, None) else {
        return RolesFacts::default();
    };
    // Without a change, an `on_domain` role is inactive: it is listed on
    // its own, with the domains that would activate it.
    let mut facts = RolesFacts::default();
    for resolved in &resolved.roles {
        let id = resolved.role.id.clone();
        match resolved.when {
            Activation::Always => facts.active.push(id),
            Activation::Never => facts.disabled.push(id),
            Activation::OnDomain => facts
                .on_domain
                .push(format!("{id} ({})", resolved.role.domains.join(", "))),
        }
    }
    facts
}

/// Number of missing doctor checks in the current project, for the one-line
/// hint of `codev status`. `None` when doctor cannot run: a hint never
/// breaks the command it decorates.
pub fn doctor_missing_from_cwd(ctx: &Ctx) -> Option<usize> {
    doctor(ctx).ok().map(|d| d.missing())
}

// ─────────────────────────────── helpers ───────────────────────────────

/// Resolves which change to act on.
///
/// Without an explicit name: if there is only one active change, that is the
/// one — the common case, and requiring the name would be ceremony. Beyond
/// that, refuse and list the candidates rather than pick one at random.
fn resolve_change(
    fs: &dyn FileSystem,
    layout: &Layout,
    requested: Option<&str>,
) -> Result<ChangeId> {
    if let Some(name) = requested {
        return Ok(ChangeId::parse(name)?);
    }

    let active = change::list(fs, layout);
    match active.len() {
        1 => Ok(active.into_iter().next().expect("exactly one element")),
        0 => Err(
            Failure::new("no_active_change", "no active change in this project")
                .with_fix("create one with `codev new change <name>`"),
        ),
        _ => {
            let names: Vec<String> = active.iter().map(ToString::to_string).collect();
            Err(Failure::new(
                "ambiguous_change",
                format!(
                    "several active changes: {} — specify which one",
                    names.join(", ")
                ),
            )
            .with_fix(format!("`--change {}`", names[0])))
        }
    }
}

fn absolute(ctx: &Ctx, path: &str) -> Result<PathBuf> {
    let candidate = Path::new(path);
    let joined = if candidate.is_absolute() {
        candidate.to_path_buf()
    } else {
        let cwd = ctx.env.current_dir().map_err(|e| {
            Failure::new(
                "unreadable",
                format!("cannot read the current directory: {e}"),
            )
        })?;
        cwd.join(candidate)
    };
    Ok(clean(joined))
}

/// Removes the useless components of a path.
///
/// Without this, `codev init` run without an argument prints "/my/project/.":
/// the path is correct, but a user reading it wonders what that dot is doing
/// there. No canonicalization — the folder may not exist yet.
fn clean(path: PathBuf) -> PathBuf {
    use std::path::Component;
    let mut out = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                out.pop();
            }
            other => out.push(other),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use codev_engine::ports::{
        FixedClock, FixedEnv, MemoryExecutable, MemoryFileSystem, MemoryHttp, MockProcessRunner,
    };

    /// Default `init` options for the tests: non-interactive (--yes), probe
    /// disabled (deterministic results, no real manifest to detect in a
    /// default MemoryFileSystem).
    fn test_init_opts() -> crate::init_prompts::InitOptions {
        crate::init_prompts::InitOptions {
            yes: true,
            no_detect: true,
            preset: None,
            language: None,
            ..Default::default()
        }
    }

    struct Harness {
        fs: MemoryFileSystem,
        env: FixedEnv,
        clock: FixedClock,
        runner: MockProcessRunner,
        http: MemoryHttp,
        exe: MemoryExecutable,
    }

    impl Harness {
        fn new() -> Self {
            let mut env = FixedEnv::at("/p");
            env.vars.insert("HOME".into(), "/home".into());
            Self {
                fs: MemoryFileSystem::new(),
                env,
                clock: FixedClock("2026-09-08".into()),
                runner: MockProcessRunner::new(),
                // Offline by default: every test that does not seed a
                // response proves the commands survive without network.
                http: MemoryHttp::new(),
                exe: MemoryExecutable::at("/home/.local/bin/codev", "codev 0.1.0"),
            }
        }

        fn with_http(mut self, http: MemoryHttp) -> Self {
            self.http = http;
            self
        }

        fn with_exe(mut self, exe: MemoryExecutable) -> Self {
            self.exe = exe;
            self
        }

        fn with(mut self, path: &str, contents: &str) -> Self {
            self.fs = self.fs.with_file(path, contents);
            self
        }

        fn with_runner(mut self, runner: MockProcessRunner) -> Self {
            self.runner = runner;
            self
        }

        fn ctx(&self) -> Ctx<'_> {
            Ctx {
                fs: &self.fs,
                env: &self.env,
                clock: &self.clock,
                runner: &self.runner,
                http: &self.http,
                exe: &self.exe,
                version: "0.1.0",
                target: "aarch64-apple-darwin",
            }
        }
    }

    #[test]
    fn init_creates_the_structure_and_the_skills() {
        let h = Harness::new();
        let outcome = init(&h.ctx(), ".", false, &test_init_opts()).unwrap();

        assert_eq!(outcome.root, PathBuf::from("/p"));
        assert_eq!(
            outcome.skills,
            [
                "codev-propose",
                "codev-explore",
                "codev-onboard",
                "codev-apply",
                "codev-sync",
                "codev-archive",
                "codev-update",
                "codev-configure"
            ]
        );
        assert!(h.fs.read("/p/_codev/config.yaml").is_some());
        assert!(
            h.fs.read("/p/.claude/skills/codev-propose/SKILL.md")
                .is_some_and(|c| c.contains("name: codev-propose"))
        );
        assert!(outcome.changed_anything());
    }

    #[test]
    fn init_counts_the_generated_config_among_the_created_files() {
        let h = Harness::new();
        let outcome = init(&h.ctx(), ".", false, &test_init_opts()).unwrap();

        let config = PathBuf::from("/p/_codev/config.yaml");
        assert_eq!(
            outcome.created.iter().filter(|p| **p == config).count(),
            1,
            "created: {:?}",
            outcome.created
        );
        assert!(!outcome.untouched.contains(&config));
        // 8 skills + config.yaml + 5 `.gitkeep` + CLAUDE.md.
        assert_eq!(outcome.created.len(), 15, "{:?}", outcome.created);
    }

    #[test]
    fn rerunning_init_changes_nothing() {
        let h = Harness::new();
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        let second = init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        assert!(
            !second.changed_anything(),
            "created: {:?}, updated: {:?}",
            second.created,
            second.updated
        );
    }

    #[test]
    fn init_respects_already_configured_workflows() {
        let h = Harness::new().with("/p/_codev/config.yaml", "workflows:\n  - explore\n");
        let outcome = init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        assert_eq!(outcome.skills, ["codev-explore"]);
        assert!(
            h.fs.read("/p/.claude/skills/codev-propose/SKILL.md")
                .is_none()
        );
    }

    #[test]
    fn init_yes_with_probe_generates_config_with_provenance() {
        // Realistic scenario: Rust workspace project + Atlassian .mcp.json.
        // `codev init --yes` (without --no-detect) must produce a
        // _codev/config.yaml that reflects the detection, with provenance
        // comments.
        const CARGO_WS: &str = r#"
[workspace]
members = ["a", "b"]
[workspace.package]
edition = "2024"
"#;
        const MCP: &str = r#"{
            "mcpServers": {
                "claude.ai Atlassian Rovo": { "url": "https://mcp.atlassian.com/" }
            }
        }"#;
        let h = Harness::new()
            .with("/p/Cargo.toml", CARGO_WS)
            .with("/p/.mcp.json", MCP);

        // --yes alone (no --no-detect), so that the probe runs.
        let opts = crate::init_prompts::InitOptions {
            yes: true,
            no_detect: false,
            preset: None,
            language: None,
            ..Default::default()
        };
        let outcome = init(&h.ctx(), ".", false, &opts).unwrap();

        // 8 skills installed (full default — 7 cycle workflows + configure).
        assert_eq!(outcome.skills.len(), 8);
        assert!(outcome.skills.iter().any(|s| s == "codev-apply"));
        assert!(outcome.skills.iter().any(|s| s == "codev-configure"));

        // The config.yaml exists and carries the MCP tool + the detected
        // context.
        let cfg =
            h.fs.read("/p/_codev/config.yaml")
                .expect("config.yaml written");
        assert!(
            cfg.contains("mcp__claude_ai_Atlassian_Rovo__getJiraIssue"),
            "config must contain the normalized tool_id: {cfg}"
        );
        assert!(
            cfg.contains("detected from .mcp.json"),
            "config must cite the MCP provenance: {cfg}"
        );
        assert!(
            cfg.contains("Rust workspace"),
            "config must cite the detected stack: {cfg}"
        );
        assert!(
            cfg.contains("detected from Cargo.toml"),
            "config must cite the stack provenance: {cfg}"
        );
    }

    #[test]
    fn init_writes_the_language_detected_from_the_locale() {
        let mut h = Harness::new();
        h.env.vars.insert("LANG".into(), "fr_FR.UTF-8".into());
        let opts = crate::init_prompts::InitOptions {
            yes: true,
            no_detect: false,
            preset: None,
            language: None,
            ..Default::default()
        };
        init(&h.ctx(), ".", false, &opts).unwrap();

        let cfg =
            h.fs.read("/p/_codev/config.yaml")
                .expect("config.yaml written");
        assert!(
            cfg.contains("# detected from LANG=fr_FR.UTF-8\nlanguage: fr\n"),
            "{cfg}"
        );
    }

    #[test]
    fn init_language_flag_wins_over_the_locale() {
        let mut h = Harness::new();
        h.env.vars.insert("LANG".into(), "fr_FR.UTF-8".into());
        let opts = crate::init_prompts::InitOptions {
            yes: true,
            no_detect: false,
            preset: None,
            language: Some("de".into()),
            ..Default::default()
        };
        init(&h.ctx(), ".", false, &opts).unwrap();

        let cfg =
            h.fs.read("/p/_codev/config.yaml")
                .expect("config.yaml written");
        assert!(cfg.contains("language: de\n"), "{cfg}");
        assert!(!cfg.contains("language: fr"), "{cfg}");
    }

    #[test]
    fn init_yes_without_detection_produces_a_minimal_config() {
        // --no-detect skips the probe; the config.yaml only has the schema
        // and the workflows.
        let h = Harness::new();
        let outcome = init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        assert_eq!(outcome.skills.len(), 8);
        let cfg = h.fs.read("/p/_codev/config.yaml").unwrap();
        assert!(cfg.contains("schema: spec-driven"));
        assert!(cfg.contains("- propose"));
        assert!(!cfg.contains("mcp:"), "no mcp: without detection: {cfg}");
        assert!(!cfg.contains("context: |"), "no context without detection");
    }

    #[test]
    fn update_requires_an_existing_project() {
        let h = Harness::new();
        let err = update(&h.ctx(), false).unwrap_err();
        assert_eq!(err.code, "no_codev_root");
    }

    #[test]
    fn the_full_cycle_leads_to_instructions() {
        // The first vertical slice, end to end: init, new change, status,
        // instructions.
        let h = Harness::new();
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();

        let created = new_change(&h.ctx(), "add-auth", None, Some("Add auth".into())).unwrap();
        assert_eq!(created.schema_name, "spec-driven");
        assert!(
            h.fs.read("/p/_codev/changes/add-auth/change.yaml")
                .is_some_and(|c| c.contains("created: 2026-09-08"))
        );

        // Without `--change`: a single active change, hence no ambiguity.
        let outcome = status(&h.ctx(), None).unwrap();
        assert_eq!(outcome.status.change.as_str(), "add-auth");
        assert!(!outcome.status.planning_complete);

        let instr = artifact_instructions(&h.ctx(), None, None).unwrap();
        assert_eq!(instr.artifact_id, "proposal");
        assert!(instr.template.is_some());
        assert!(instr.instruction.is_some());
    }

    #[test]
    fn rejects_an_invalid_change_name() {
        let h = Harness::new();
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        let before = h.fs.paths();
        let err = new_change(&h.ctx(), "Add Auth", None, None).unwrap_err();
        assert_eq!(err.code, "invalid_change_id");
        assert!(err.message.contains("space"), "{}", err.message);
        assert_eq!(h.fs.paths(), before, "a refused name writes nothing");
    }

    #[test]
    fn new_change_refuses_the_reserved_name_archive() {
        let h = Harness::new().with(
            "/p/_codev/changes/archive/2026-09-01-old/proposal.md",
            "# Proposal",
        );
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        let before = h.fs.paths();
        let err = new_change(&h.ctx(), "archive", None, None).unwrap_err();
        assert_eq!(err.code, "invalid_change_id");
        assert!(err.message.contains("reserved"), "{}", err.message);
        assert_eq!(h.fs.paths(), before, "a refused name writes nothing");
    }

    /// A project with a `spec-driven` change `add-auth` and `config` as
    /// its `_codev/config.yaml`.
    fn roles_project(config: &str, change_yaml: &str) -> Harness {
        Harness::new()
            .with("/p/_codev/config.yaml", config)
            .with("/p/_codev/changes/add-auth/change.yaml", change_yaml)
    }

    fn roles_json(h: &Harness, at: Option<&str>, change: Option<&str>) -> serde_json::Value {
        let outcome = roles(&h.ctx(), at, change, false, false).unwrap();
        serde_json::to_value(crate::contract::RolesV1::from(&outcome)).unwrap()
    }

    #[test]
    fn roles_at_a_step_as_json() {
        let h = roles_project("", "schema: spec-driven\n");
        let json = roles_json(&h, Some("plan.trace"), Some("add-auth"));
        assert_eq!(json["changeName"], "add-auth");
        assert_eq!(json["at"], "plan.trace");
        let roles = json["roles"].as_array().unwrap();
        assert_eq!(roles.len(), 1);
        let tracer = &roles[0];
        assert_eq!(tracer["id"], "tracer");
        assert_eq!(tracer["kind"], "step");
        assert_eq!(tracer["at"], "plan.trace");
        assert_eq!(tracer["steps"], serde_json::json!(["plan.trace"]));
        assert!(tracer["artifact"].is_null());
        assert_eq!(tracer["domains"], serde_json::json!([]));
        assert_eq!(tracer["when"], "always");
        assert_eq!(tracer["whenOrigin"], "default");
        assert_eq!(tracer["activatedBy"], "always");
        assert!(tracer["knowledge"].is_null(), "the tracer has no knowledge");
        let criteria = tracer["doneWhen"].as_array().unwrap();
        assert!(!criteria.is_empty());
        assert!(criteria.iter().all(|c| c["origin"] == "built-in"));
    }

    #[test]
    fn roles_at_plan_write_are_the_artifact_roles() {
        let h = roles_project("", "schema: spec-driven\n");
        let json = roles_json(&h, Some("plan.write"), None);
        assert!(json["changeName"].is_null());
        let pairs: Vec<(String, String)> = json["roles"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| {
                (
                    r["id"].as_str().unwrap().to_string(),
                    r["artifact"].as_str().unwrap().to_string(),
                )
            })
            .collect();
        let expected = [
            ("product", "proposal"),
            ("qa", "specs"),
            ("architect", "design"),
            ("lead", "tasks"),
        ]
        .map(|(id, artifact)| (id.to_string(), artifact.to_string()));
        assert_eq!(pairs, expected);
        assert_eq!(
            json["roles"][0]["doneWhen"][0]["origin"],
            "schema:spec-driven"
        );
    }

    #[test]
    fn roles_without_at_lists_every_role() {
        let h = roles_project("roles:\n  newcomer: never\n", "schema: spec-driven\n");
        let json = roles_json(&h, None, None);
        let roles = json["roles"].as_array().unwrap();
        let ids: Vec<&str> = roles.iter().map(|r| r["id"].as_str().unwrap()).collect();
        assert_eq!(
            ids,
            [
                "analyst",
                "investigator",
                "product",
                "qa",
                "architect",
                "lead",
                "tracer",
                "skeptic",
                "security",
                "privacy",
                "accessibility",
                "performance",
                "ops",
                "migration",
                "developer",
                "newcomer"
            ]
        );
        let newcomer = &roles[15];
        assert_eq!(newcomer["when"], "never");
        assert_eq!(newcomer["whenOrigin"], "project");
        // An inactive role is served neither its knowledge nor a reason.
        assert!(newcomer["knowledge"].is_null() && newcomer["activatedBy"].is_null());
        for expert in &roles[8..14] {
            assert_eq!(expert["when"], "on_domain", "{}", expert["id"]);
            assert_eq!(expert["at"], "plan.challenge");
            assert_eq!(
                expert["steps"],
                serde_json::json!(["plan.challenge", "code.review"])
            );
            assert!(expert["activatedBy"].is_null());
        }
        let locked: Vec<&str> = roles
            .iter()
            .filter(|r| r["disableable"] == false)
            .map(|r| r["id"].as_str().unwrap())
            .collect();
        assert_eq!(
            locked,
            ["analyst", "product", "qa", "architect", "lead", "developer"]
        );
    }

    /// `roles_project` with a proposal for `add-auth` holding `domains`.
    fn roles_project_with_domains(config: &str, change_yaml: &str, domains: &str) -> Harness {
        roles_project(config, change_yaml).with(
            "/p/_codev/changes/add-auth/proposal.md",
            &format!("# Proposal: Auth\n\n## Domains\n\n{domains}\n"),
        )
    }

    fn ids_of(json: &serde_json::Value) -> Vec<String> {
        json["roles"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| r["id"].as_str().unwrap().to_string())
            .collect()
    }

    #[test]
    fn roles_serve_the_experts_of_the_ticked_domains() {
        let h = roles_project_with_domains(
            "",
            "schema: spec-driven\n",
            "- [x] security\n- [x] personal-data\n- [ ] performance",
        );
        let json = roles_json(&h, Some("plan.challenge"), Some("add-auth"));
        assert_eq!(ids_of(&json), ["skeptic", "security", "privacy"]);
        let security = &json["roles"][1];
        assert_eq!(security["activatedBy"], "domain:security");
        assert_eq!(json["roles"][2]["activatedBy"], "domain:personal-data");
        let knowledge = security["knowledge"].as_str().unwrap();
        let sections: Vec<&str> = knowledge
            .lines()
            .filter_map(|l| l.strip_prefix("## "))
            .collect();
        assert_eq!(
            sections,
            ["Scope", "Signals", "By stack", "References", "Reporting"]
        );
        for reference in ["OWASP Top 10", "ASVS", "CWE"] {
            assert!(knowledge.contains(reference), "{reference}");
        }
        let review = roles_json(&h, Some("code.review"), Some("add-auth"));
        assert_eq!(review["roles"][0]["at"], "code.review");
    }

    #[test]
    fn roles_without_a_change_serve_no_expert() {
        let h = roles_project_with_domains("", "schema: spec-driven\n", "- [x] security");
        assert_eq!(
            ids_of(&roles_json(&h, Some("code.review"), None)),
            ["newcomer"]
        );
        let none = roles_project_with_domains("", "schema: spec-driven\n", "None");
        assert_eq!(
            ids_of(&roles_json(&none, Some("plan.challenge"), Some("add-auth"))),
            ["skeptic"]
        );
        let forced = roles_project_with_domains(
            "roles:\n  accessibility: never\n",
            "schema: spec-driven\nroles:\n  performance: always\n",
            "- [x] user-interface",
        );
        let json = roles_json(&forced, Some("code.review"), Some("add-auth"));
        assert_eq!(ids_of(&json), ["performance", "newcomer"]);
        assert_eq!(json["roles"][0]["activatedBy"], "change");
    }

    #[test]
    fn roles_refuse_to_disable_the_analyst() {
        let h = roles_project("roles:\n  analyst: never\n", "schema: spec-driven\n");
        let err = roles(&h.ctx(), Some("plan.understand"), None, false, false).unwrap_err();
        assert_eq!(err.code, "role_not_disableable");
        assert!(err.message.contains("analyst"), "{}", err.message);
    }

    #[test]
    fn roles_follow_the_change_override_and_the_language() {
        let h = roles_project(
            "language: fr\nroles:\n  newcomer: never\n",
            "schema: spec-driven\nroles:\n  newcomer: always\n",
        );
        let with_change = roles_json(&h, Some("code.review"), Some("add-auth"));
        assert_eq!(with_change["roles"][0]["whenOrigin"], "change");
        assert_eq!(with_change["roles"][0]["name"], "Nouvel arrivant");
        let project = roles_json(&h, Some("code.review"), None);
        assert!(project["roles"].as_array().unwrap().is_empty());
    }

    #[test]
    fn roles_rejects_an_unknown_step() {
        // The failure's document shape: `roles_failure_keeps_the_shape` in
        // main.rs.
        let h = roles_project("", "schema: spec-driven\n");
        let err = roles(&h.ctx(), Some("plan.review"), None, false, false).unwrap_err();
        assert_eq!(err.code, "unknown_step");
        assert!(err.message.contains("code.review"), "{}", err.message);
    }

    // ─────────────── suggested domains and stacks ───────────────

    const LOGIN_PATCH: &str = "diff --git a/src/auth/session.rs b/src/auth/session.rs\n\
--- /dev/null\n\
+++ b/src/auth/session.rs\n\
@@ -0,0 +1,2 @@\n\
+fn login() {\n\
+}\n";

    /// A git repository whose branch adds `src/auth/session.rs`.
    fn login_repository() -> MockProcessRunner {
        use codev_engine::diff::doubles::{records, repository};
        repository(
            None,
            &records(&["2\t0\tsrc/auth/session.rs"]),
            "",
            LOGIN_PATCH,
        )
    }

    fn roles_change_json(h: &Harness, at: Option<&str>, all_stacks: bool) -> serde_json::Value {
        let outcome = roles(&h.ctx(), at, Some("add-auth"), false, all_stacks).unwrap();
        serde_json::to_value(crate::contract::RolesV1::from(&outcome)).unwrap()
    }

    fn asked_for_a_diff(runner: &MockProcessRunner) -> bool {
        runner
            .calls()
            .iter()
            .any(|(_, args)| args.iter().any(|a| a == "--numstat"))
    }

    #[test]
    fn roles_json_suggests_security_for_an_auth_path() {
        let h = roles_project_with_domains("", "schema: spec-driven\n", "- [ ] performance")
            .with_runner(login_repository());
        let json = roles_change_json(&h, None, false);
        assert_eq!(json["diff"]["available"], true);
        assert_eq!(json["diff"]["base"], "main");
        assert_eq!(json["diff"]["files"], 1);
        assert_eq!(json["diff"]["scanned"], 1);
        assert_eq!(json["diff"]["truncated"], false);
        let suggestions = json["suggestions"].as_array().unwrap();
        assert_eq!(suggestions.len(), 1, "{suggestions:?}");
        let security = &suggestions[0];
        assert_eq!(security["domain"], "security");
        assert_eq!(security["state"], "unticked");
        assert!(security["justification"].is_null());
        assert_eq!(security["matchCount"], 1);
        let first = &security["matches"][0];
        assert_eq!(first["source"], "security");
        assert_eq!(first["kind"], "paths");
        assert_eq!(first["trigger"], "**/auth/**");
        assert_eq!(first["path"], "src/auth/session.rs");
        assert!(first["line"].is_null());
    }

    #[test]
    fn roles_outside_git_report_no_diff() {
        // The harness's runner answers nothing: no git repository.
        let h = roles_project_with_domains("", "schema: spec-driven\n", "None");
        let json = roles_change_json(&h, Some("plan.challenge"), false);
        assert_eq!(json["diff"]["available"], false);
        let reason = json["diff"]["reason"].as_str().unwrap();
        assert!(reason.contains("not in a git repository"), "{reason}");
        assert!(json["diff"]["base"].is_null());
        assert_eq!(json["suggestions"], serde_json::json!([]));
    }

    #[test]
    fn roles_without_a_change_have_no_diff() {
        let h = roles_project("", "schema: spec-driven\n").with_runner(login_repository());
        let json = roles_json(&h, Some("plan.challenge"), None);
        assert!(json["diff"].is_null());
        assert_eq!(json["suggestions"], serde_json::json!([]));
        assert!(!asked_for_a_diff(&h.runner), "no change, no git");
    }

    #[test]
    fn roles_suggestions_are_stable_and_write_nothing() {
        let h = roles_project_with_domains("", "schema: spec-driven\n", "None")
            .with_runner(login_repository());
        let before = h.fs.paths();
        let first = roles_change_json(&h, Some("code.review"), false);
        let second = roles_change_json(&h, Some("code.review"), false);
        assert_eq!(first, second);
        assert_eq!(h.fs.paths(), before);
    }

    #[test]
    fn roles_serve_by_stack_for_the_detected_languages() {
        let h = roles_project_with_domains("", "schema: spec-driven\n", "- [x] security")
            .with("/p/Cargo.toml", "[package]\nname = \"x\"\n");
        let json = roles_change_json(&h, Some("code.review"), false);
        assert_eq!(
            json["stacks"],
            serde_json::json!({ "detected": ["Rust"], "filter": "detected" })
        );
        let security = json["roles"]
            .as_array()
            .unwrap()
            .iter()
            .find(|r| r["id"] == "security")
            .unwrap();
        let by_stack = security["knowledge"]
            .as_str()
            .unwrap()
            .split("## By stack")
            .nth(1)
            .unwrap()
            .split("\n## ")
            .next()
            .unwrap()
            .to_string();
        assert!(by_stack.contains("- **Rust**"), "{by_stack}");
        for other in [
            "**Python**",
            "**Go**",
            "**Java**",
            "**TypeScript/JavaScript**",
        ] {
            assert!(!by_stack.contains(other), "{other} in {by_stack}");
        }
        let none = roles_project_with_domains("", "schema: spec-driven\n", "- [x] security");
        let json = roles_change_json(&none, Some("code.review"), false);
        assert_eq!(
            json["stacks"],
            serde_json::json!({ "detected": [], "filter": "none-detected" })
        );
        assert!(
            json["roles"][0]["knowledge"]
                .as_str()
                .unwrap()
                .contains("**Python**")
        );
    }

    #[test]
    fn roles_all_stacks_serve_every_language() {
        let h = roles_project_with_domains("", "schema: spec-driven\n", "- [x] security")
            .with("/p/Cargo.toml", "[package]\nname = \"x\"\n");
        let json = roles_change_json(&h, Some("code.review"), true);
        assert_eq!(json["stacks"]["filter"], "all-stacks");
        assert_eq!(json["stacks"]["detected"], serde_json::json!(["Rust"]));
        let knowledge = json["roles"][0]["knowledge"].as_str().unwrap();
        for language in [
            "**Rust**",
            "**Python**",
            "**Go**",
            "**Java**",
            "**TypeScript/JavaScript**",
        ] {
            assert!(knowledge.contains(language), "{language}");
        }
    }

    #[test]
    fn roles_ignore_an_invalid_project_trigger_with_a_warning() {
        let h = roles_project_with_domains(
            "triggers:\n  - domain: secrets\n    paths: [\"src/**\"]\n",
            "schema: spec-driven\n",
            "None",
        )
        .with_runner(login_repository());
        let json = roles_change_json(&h, None, false);
        let codes: Vec<&str> = json["status"]
            .as_array()
            .unwrap()
            .iter()
            .map(|s| s["code"].as_str().unwrap())
            .collect();
        assert_eq!(codes, ["invalid_trigger"]);
        let domains: Vec<&str> = json["suggestions"]
            .as_array()
            .unwrap()
            .iter()
            .map(|s| s["domain"].as_str().unwrap())
            .collect();
        assert_eq!(domains, ["security"], "the built-in triggers still run");
    }

    fn suggested_findings(report: &ValidateReport) -> Vec<(String, u32, String)> {
        report
            .items
            .iter()
            .flat_map(|item| item.findings.iter())
            .filter(|f| f.finding.code == "domain_suggested")
            .map(|f| {
                (
                    codev_core::layout::slashed(&f.path),
                    f.finding.line,
                    f.finding.message.clone(),
                )
            })
            .collect()
    }

    #[test]
    fn validate_warns_on_an_unticked_suggested_domain() {
        let h = roles_project_with_domains("", "schema: spec-driven\n", "- [ ] security")
            .with_runner(login_repository());
        let report = validate(&h.ctx(), ValidateArgs::Item("add-auth".into())).unwrap();
        let found = suggested_findings(&report);
        assert_eq!(found.len(), 1, "{found:?}");
        let (path, line, message) = &found[0];
        assert_eq!(path, "_codev/changes/add-auth/proposal.md");
        assert_eq!(*line, 5, "the line of `- [ ] security`");
        for needle in ["`security`", "**/auth/**", "src/auth/session.rs"] {
            assert!(message.contains(needle), "{needle}: {message}");
        }
        // A warning: `--strict` turns it into a failing exit code.
        assert!(
            report
                .items
                .iter()
                .flat_map(|item| item.findings.iter())
                .filter(|f| f.finding.code == "domain_suggested")
                .all(|f| !f.is_error())
        );
        assert!(report.has_warnings());
    }

    #[test]
    fn validate_is_silent_on_a_justified_or_ticked_suggestion() {
        for domains in ["- [ ] security — only renames a variable", "- [x] security"] {
            let h = roles_project_with_domains("", "schema: spec-driven\n", domains)
                .with_runner(login_repository());
            let report = validate(&h.ctx(), ValidateArgs::All).unwrap();
            assert!(suggested_findings(&report).is_empty(), "{domains}");
        }
    }

    #[test]
    fn validate_outside_git_reports_no_suggestion() {
        let h = roles_project_with_domains("", "schema: spec-driven\n", "None");
        let report = validate(&h.ctx(), ValidateArgs::Changes).unwrap();
        assert!(suggested_findings(&report).is_empty());
        let specs_only = roles_project_with_domains("", "schema: spec-driven\n", "None")
            .with_runner(login_repository());
        validate(&specs_only.ctx(), ValidateArgs::Specs).unwrap();
        assert!(
            !asked_for_a_diff(&specs_only.runner),
            "`--specs` never runs git"
        );
    }

    #[test]
    fn archive_does_not_ask_git_for_a_diff() {
        let h = reviewed_change(&security_finding(
            "high",
            "  - status: fixed\n    by: developer\n    ref: src/auth.rs uses the CSPRNG\n  - status: fixed\n    by: newcomer\n",
        ))
        .with_runner(login_repository());
        archive(&h.ctx(), None).unwrap();
        assert!(!asked_for_a_diff(&h.runner));
    }

    #[test]
    fn doctor_does_not_ask_git_for_a_diff() {
        let h = roles_project_with_domains("", "schema: spec-driven\n", "- [ ] security")
            .with_runner(login_repository());
        let _ = doctor(&h.ctx());
        assert!(!asked_for_a_diff(&h.runner));
    }

    #[test]
    fn status_fails_with_unknown_role() {
        let h = roles_project("roles:\n  reviewer: never\n", "schema: spec-driven\n");
        let err = status(&h.ctx(), Some("add-auth")).unwrap_err();
        assert_eq!(err.code, "unknown_role");
        assert!(err.message.contains("reviewer"), "{}", err.message);
    }

    #[test]
    fn instructions_fail_when_an_artifact_role_is_disabled() {
        let h = roles_project("roles:\n  qa: never\n", "schema: spec-driven\n");
        let err = artifact_instructions(&h.ctx(), Some("specs"), Some("add-auth")).unwrap_err();
        assert_eq!(err.code, "role_not_disableable");
        assert!(err.message.contains("qa"), "{}", err.message);
        // Extending its criteria is fine.
        let h = roles_project(
            "roles:\n  qa:\n    done_when: [Every error code is named]\n",
            "schema: spec-driven\n",
        );
        assert!(artifact_instructions(&h.ctx(), Some("specs"), Some("add-auth")).is_ok());
    }

    #[test]
    fn status_refuses_the_archive_folder_as_a_change() {
        let h = Harness::new().with(
            "/p/_codev/changes/archive/2026-09-01-old/proposal.md",
            "# Proposal",
        );
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        let err = status(&h.ctx(), Some("archive")).unwrap_err();
        assert_eq!(err.code, "invalid_change_id");
        assert!(err.message.contains("reserved"), "{}", err.message);
    }

    #[test]
    fn status_infers_the_single_change_despite_the_archive() {
        let h = Harness::new().with(
            "/p/_codev/changes/archive/2026-09-01-old/proposal.md",
            "# Proposal",
        );
        project_with_change(&h);
        let outcome = status(&h.ctx(), None).unwrap();
        assert_eq!(outcome.status.change.as_str(), "add-auth");
    }

    #[test]
    fn configured_language_reads_the_project_configuration() {
        let h = Harness::new();
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        write_helper(&h, "/p/_codev/config.yaml", "language: fr\n");
        assert_eq!(configured_language(&h.ctx()), "fr");
    }

    #[test]
    fn configured_language_defaults_to_en_outside_a_project() {
        let h = Harness::new();
        assert_eq!(configured_language(&h.ctx()), "en");
    }

    #[test]
    fn rejects_an_already_existing_change() {
        let h = Harness::new();
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        new_change(&h.ctx(), "add-auth", None, Some("First goal".into())).unwrap();
        let metadata = "/p/_codev/changes/add-auth/change.yaml";
        let before = h.fs.read(metadata);
        let err = new_change(&h.ctx(), "add-auth", None, Some("Other".into())).unwrap_err();
        assert_eq!(err.code, "change_exists");
        assert_eq!(h.fs.read(metadata), before, "the existing change is kept");
    }

    #[test]
    fn rejects_an_unknown_schema_before_creating_the_folder() {
        let h = Harness::new();
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        let err = new_change(&h.ctx(), "add-auth", Some("imaginary"), None).unwrap_err();
        assert_eq!(err.code, "schema_not_found");
        assert!(
            h.fs.read("/p/_codev/changes/add-auth/change.yaml")
                .is_none(),
            "no folder may be left behind after a failure"
        );
    }

    #[test]
    fn status_without_active_change_points_to_creation() {
        let h = Harness::new();
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        let err = status(&h.ctx(), None).unwrap_err();
        assert_eq!(err.code, "no_active_change");
        assert!(err.fix.is_some_and(|f| f.contains("codev new change")));
    }

    #[test]
    fn status_with_several_changes_refuses_to_guess() {
        let h = Harness::new();
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        new_change(&h.ctx(), "add-auth", None, None).unwrap();
        new_change(&h.ctx(), "fix-bug", None, None).unwrap();

        let err = status(&h.ctx(), None).unwrap_err();
        assert_eq!(err.code, "ambiguous_change");
        assert!(err.message.contains("add-auth") && err.message.contains("fix-bug"));
    }

    #[test]
    fn lists_the_changes_and_the_specs() {
        let h = Harness::new().with("/p/_codev/specs/user-auth/spec.md", "# spec");
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        new_change(&h.ctx(), "add-auth", None, None).unwrap();

        let changes = list_changes(&h.ctx()).unwrap();
        assert_eq!(
            changes
                .changes
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>(),
            ["add-auth"]
        );

        let specs = list_specs(&h.ctx()).unwrap();
        assert_eq!(specs.specs, ["user-auth"]);
    }

    #[test]
    fn lists_the_schemas_with_their_flow() {
        let h = Harness::new();
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        let outcome = list_schemas(&h.ctx()).unwrap();

        assert_eq!(outcome.schemas.len(), 1);
        assert_eq!(outcome.schemas[0].name, "spec-driven");
        assert_eq!(outcome.schemas[0].origin, "built-in");
        assert_eq!(
            outcome.schemas[0].flow,
            ["proposal", "specs", "design", "tasks"]
        );
    }

    #[test]
    fn sync_with_a_single_active_change_is_implicit() {
        // A single active change → sync without a name works.
        let h = Harness::new();
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        new_change(&h.ctx(), "add-auth", None, None).unwrap();
        write_helper(
            &h,
            "/p/_codev/changes/add-auth/specs/user-auth/spec.md",
            "## Purpose\n\nAuth.\n\n## ADDED Requirements\n\n### Requirement: Login\nThe system SHALL emit a token.\n\n#### Scenario: OK\n- **WHEN** login\n- **THEN** token\n",
        );
        let outcome = sync(&h.ctx(), None).unwrap();
        assert_eq!(outcome.change, "add-auth");
        assert_eq!(outcome.created.len(), 1);
    }

    #[test]
    fn archive_with_validation_error_fails_with_the_validation_failed_code() {
        let h = Harness::new();
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        new_change(&h.ctx(), "buggy", None, None).unwrap();
        write_helper(
            &h,
            "/p/_codev/changes/buggy/specs/x/spec.md",
            // Duplicate → validate reports it.
            "## Purpose\n\nx.\n\n## ADDED Requirements\n\n### Requirement: A\nThe system SHALL a.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n\n### Requirement: A\nThe system SHALL a.\n\n#### Scenario: T\n- **WHEN** c\n- **THEN** d\n",
        );
        let err = archive(&h.ctx(), None).unwrap_err();
        assert_eq!(err.code, "validation_failed");
        assert!(
            err.message.contains("codev validate buggy"),
            "{}",
            err.message
        );
        // The change has not moved.
        assert!(h.fs.read("/p/_codev/changes/buggy/change.yaml").is_some());
    }

    // ─────────────── review ───────────────

    const DELTA: &str = "## Purpose\n\nUser authentication.\n\n## ADDED Requirements\n\n### Requirement: Login\nThe system SHALL emit a token.\n\n#### Scenario: OK\n- **WHEN** login\n- **THEN** token\n";

    /// A change `add-auth` with a delta, `src/auth.rs`, and `review` as its
    /// record (none when empty).
    fn reviewed_change(review: &str) -> Harness {
        let h = Harness::new();
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        new_change(&h.ctx(), "add-auth", None, None).unwrap();
        write_helper(
            &h,
            "/p/_codev/changes/add-auth/specs/user-auth/spec.md",
            DELTA,
        );
        write_helper(
            &h,
            "/p/src/auth.rs",
            "fn login() {\n    let token = random();\n}\n",
        );
        if !review.is_empty() {
            write_helper(&h, "/p/_codev/changes/add-auth/review.md", review);
        }
        h
    }

    fn security_finding(confidence: &str, answers: &str) -> String {
        format!(
            "## plan.challenge\n\n### skeptic\n\nNo finding.\n\n## code.review\n\n### newcomer\n\n#### R-newcomer-1 A random token\n\n```yaml\nseverity: high\nconfidence: {confidence}\nlocation: src/auth.rs:2\nevidence: let token = random();\nrecommendation: Use the CSPRNG.\nhistory:\n  - status: open\n    by: newcomer\n{answers}```\n"
        )
    }

    #[test]
    fn review_json_lists_findings_awaiting_and_reported() {
        let h = reviewed_change(&security_finding(
            "low",
            "  - status: disputed\n    by: developer\n    note: seeded by the OS\n  - status: disputed\n    by: newcomer\n    note: not for tokens\n",
        ));
        let outcome = review(&h.ctx(), None).unwrap();
        assert_eq!(outcome.exit_code(), 0);
        let json = serde_json::to_value(crate::contract::ReviewV1::from(&outcome)).unwrap();
        assert_eq!(json["changeName"], "add-auth");
        assert_eq!(json["path"], "_codev/changes/add-auth/review.md");
        assert_eq!(json["exists"], true);
        let finding = &json["findings"][0];
        assert_eq!(finding["id"], "R-newcomer-1");
        assert_eq!(finding["status"], "disputed");
        assert_eq!(finding["awaiting"], "human");
        assert_eq!(finding["reported"], false);
        assert_eq!(finding["history"][1]["note"], "seeded by the OS");
        assert_eq!(json["archive"]["ready"], false);
        assert_eq!(json["archive"]["blockers"][0]["code"], "review_unresolved");
        assert_eq!(json["steps"][1]["step"], "code.review");
        assert_eq!(json["steps"][1]["roles"][0]["state"], "findings");
    }

    #[test]
    fn review_without_a_record_succeeds() {
        let h = reviewed_change("");
        let outcome = review(&h.ctx(), Some("add-auth")).unwrap();
        assert_eq!(outcome.exit_code(), 0);
        let json = serde_json::to_value(crate::contract::ReviewV1::from(&outcome)).unwrap();
        assert_eq!(json["exists"], false);
        assert_eq!(json["findings"], serde_json::json!([]));
        assert_eq!(
            json["archive"]["blockers"][0]["code"],
            "review_record_missing"
        );
        let text = crate::render::review(&outcome);
        assert!(text.contains("No review record yet"), "{text}");
    }

    #[test]
    fn review_exits_1_on_a_record_error() {
        let h = reviewed_change(
            &security_finding("high", "").replace("severity: high", "severity: critical"),
        );
        let outcome = review(&h.ctx(), None).unwrap();
        assert_eq!(outcome.exit_code(), 1);
        let json = serde_json::to_value(crate::contract::ReviewV1::from(&outcome)).unwrap();
        assert_eq!(json["issues"][0]["code"], "review_invalid");
        assert!(json["issues"][0]["line"].as_u64().unwrap() > 1);
        let text = crate::render::review(&outcome);
        assert!(
            text.contains("review_invalid") && text.contains("critical"),
            "{text}"
        );
    }

    #[test]
    fn archive_json_refusal_carries_review_blocked() {
        let h = reviewed_change(&security_finding("high", ""));
        let err = archive(&h.ctx(), None).unwrap_err();
        assert_eq!(err.code, "review_blocked");
        assert!(err.message.contains("R-newcomer-1"), "{}", err.message);
        assert!(
            h.fs.read("/p/_codev/changes/add-auth/change.yaml")
                .is_some()
        );
        // Fixed, then verified: the change goes.
        let h = reviewed_change(&security_finding(
            "high",
            "  - status: fixed\n    by: developer\n    ref: src/auth.rs uses the CSPRNG\n  - status: fixed\n    by: newcomer\n",
        ));
        let outcome = archive(&h.ctx(), None).unwrap();
        assert!(
            h.fs.read(outcome.moved_to.join("review.md")).is_some(),
            "the record moves with the change"
        );
    }

    #[test]
    fn validate_clean_project_produces_no_finding() {
        let h = Harness::new();
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        new_change(&h.ctx(), "add-auth", None, None).unwrap();
        // A well-formed delta under the change.
        write_helper(
            &h,
            "/p/_codev/changes/add-auth/specs/user-auth/spec.md",
            "## Purpose\n\nUser authentication.\n\n## ADDED Requirements\n\n### Requirement: Login\nThe system SHALL emit a token.\n\n#### Scenario: OK\n- **WHEN** login\n- **THEN** token\n",
        );

        let report = validate(&h.ctx(), ValidateArgs::All).unwrap();
        assert!(
            !report.has_errors(),
            "no finding expected; got: {:#?}",
            report
        );
        // 1 change + 1 decisions item (always present, even without ADRs).
        assert_eq!(report.items.len(), 2);
    }

    #[test]
    fn validate_catches_parser_and_rule_findings() {
        let h = Harness::new();
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        new_change(&h.ctx(), "buggy", None, None).unwrap();
        // A delta where SHALL is missing (rule E1) and the scenario is
        // malformed (parser rule): both findings must surface.
        write_helper(
            &h,
            "/p/_codev/changes/buggy/specs/x/spec.md",
            "## Purpose\n\nCapability for testing the rules.\n\n## ADDED Requirements\n\n### Requirement: X\nThe system does x.\n\n### Scenario: Malformed\n- **WHEN** a\n- **THEN** b\n",
        );

        let report = validate(&h.ctx(), ValidateArgs::All).unwrap();
        assert!(report.has_errors());

        let codes: Vec<&str> = report
            .items
            .iter()
            .flat_map(|i| i.findings.iter().map(|f| f.finding.code))
            .collect();
        assert!(
            codes.contains(&"requirement_no_shall"),
            "rule E1 expected; codes: {codes:?}"
        );
        assert!(
            codes.contains(&"scenario_wrong_heading_level"),
            "parser rule expected; codes: {codes:?}"
        );
    }

    #[test]
    fn validate_zero_delta_without_marker_fails() {
        // Rule E3: a change with no delta and no skip_specs fails.
        let h = Harness::new();
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        new_change(&h.ctx(), "refactor", None, None).unwrap();

        let report = validate(&h.ctx(), ValidateArgs::Changes).unwrap();
        assert!(report.has_errors());
        assert!(
            report.items[0]
                .findings
                .iter()
                .any(|f| f.finding.code == "zero_delta_without_marker")
        );
    }

    #[test]
    fn validate_specs_scope_ignores_the_changes() {
        // A change in error, but only the specs are requested: nothing to
        // report, exit 0.
        let h = Harness::new();
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        new_change(&h.ctx(), "refactor", None, None).unwrap(); // zero-delta

        let report = validate(&h.ctx(), ValidateArgs::Specs).unwrap();
        assert!(!report.has_errors());
        assert!(report.items.is_empty());
    }

    #[test]
    fn validate_unknown_item_fails_with_a_useful_code() {
        let h = Harness::new();
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        let err = validate(&h.ctx(), ValidateArgs::Item("ghost".into())).unwrap_err();
        assert_eq!(err.code, "unknown_item");
        assert!(err.fix.is_some_and(|f| f.contains("codev list")));
    }

    const VALID_MAIN_SPEC: &str = "# User auth Specification\n\n## Purpose\n\nUser authentication.\n\n## Requirements\n\n### Requirement: Login\nThe system SHALL emit a token.\n\n#### Scenario: OK\n- **WHEN** login\n- **THEN** token\n";

    /// A project with one valid main spec and a stray Markdown file whose
    /// name ends with `spec.md`.
    fn project_with_a_stray_spec_file() -> Harness {
        let h = Harness::new()
            .with("/p/_codev/specs/user-auth/spec.md", VALID_MAIN_SPEC)
            .with("/p/_codev/specs/notes/myspec.md", "# Notes\n");
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        h
    }

    #[test]
    fn validate_specs_ignores_a_stray_markdown_file() {
        let h = project_with_a_stray_spec_file();
        let report = validate(&h.ctx(), ValidateArgs::Specs).unwrap();
        let names: Vec<&str> = report.items.iter().map(|i| i.name.as_str()).collect();
        assert_eq!(names, ["user-auth"]);
        assert!(
            !report.has_errors() && !report.has_warnings(),
            "{report:#?}"
        );
    }

    #[test]
    fn validate_all_ignores_a_stray_markdown_file() {
        let h = project_with_a_stray_spec_file();
        let report = validate(&h.ctx(), ValidateArgs::All).unwrap();
        assert!(
            !report.has_errors() && !report.has_warnings(),
            "{report:#?}"
        );
        assert!(
            report.items.iter().all(|i| !i.name.contains("notes")),
            "{report:#?}"
        );
    }

    #[test]
    fn validate_does_not_find_a_stray_file_by_name() {
        let h = project_with_a_stray_spec_file();
        let err = validate(&h.ctx(), ValidateArgs::Item("notes/my".into())).unwrap_err();
        assert_eq!(err.code, "unknown_item");
    }

    /// Small helper that inserts a file into the harness's in-memory FS.
    ///
    /// Lives here rather than in `Harness` because it is only useful to the
    /// `validate` tests — the other commands add no manual content.
    fn write_helper(h: &Harness, path: &str, contents: &str) {
        use codev_engine::FileSystem;
        h.fs.write(std::path::Path::new(path), contents).unwrap();
    }

    #[test]
    fn a_broken_custom_schema_is_reported_without_hiding_the_others() {
        let h = Harness::new().with(
            "/p/_codev/schemas/broken/schema.yaml",
            "name: broken\nartifacts:\n  - id: a\n    generates: a.md\n    requires: [ghost]\napply:\n  requires: [a]\n  tracks: a.md\n",
        );
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        let outcome = list_schemas(&h.ctx()).unwrap();

        assert_eq!(outcome.schemas.len(), 1, "spec-driven stays listed");
        assert_eq!(outcome.schemas[0].name, "spec-driven");
        assert_eq!(outcome.warnings.len(), 1);
        assert_eq!(outcome.warnings[0].code, "schema_unusable");
        assert!(
            outcome.warnings[0].message.contains("`broken`"),
            "{}",
            outcome.warnings[0].message
        );
    }

    // ─────────────── decision seal ───────────────

    #[test]
    fn decision_new_accepted_seals_the_entry_and_exposes_the_hash() {
        // A `decision new --status accepted` creates the ADR AND the seal entry; the hash
        // surfaces in the outcome for the JSON contract.
        let h = Harness::new();
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        let outcome = decision_new(&h.ctx(), "A first choice", "accepted").unwrap();
        assert!(outcome.body_sha256.is_some());
        assert!(outcome.body_sha256.as_ref().unwrap().starts_with("sha256:"));

        // The seal file exists and references 0001.
        use codev_engine::FileSystem;
        let seal_content =
            h.fs.read_to_string(std::path::Path::new("/p/_codev/decisions/seal.yaml"))
                .unwrap();
        assert!(seal_content.contains("0001"));
    }

    #[test]
    fn decision_new_proposed_neither_seals_nor_exposes_a_hash() {
        // A `proposed` status commits to no immutability — no seal, no hash
        // in the outcome.
        let h = Harness::new();
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        let outcome = decision_new(&h.ctx(), "Lead", "proposed").unwrap();
        assert!(outcome.body_sha256.is_none());

        use codev_engine::FileSystem;
        assert!(
            !h.fs
                .exists(std::path::Path::new("/p/_codev/decisions/seal.yaml"))
        );
    }

    #[test]
    fn decision_seal_is_a_noop_after_decision_new() {
        // `decision new` already seals the ADR; a `decision seal` right
        // after must be a silent no-op.
        let h = Harness::new();
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        decision_new(&h.ctx(), "A first choice", "accepted").unwrap();
        let outcome = decision_seal(&h.ctx(), "0001", false).unwrap();
        assert!(outcome.was_noop);
        assert!(!outcome.was_forced);
    }

    #[test]
    fn decision_seal_refuses_without_force_after_the_body_changed() {
        let h = Harness::new();
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        decision_new(&h.ctx(), "A first choice", "accepted").unwrap();

        // Corrupt the ADR's body after sealing.
        use codev_engine::FileSystem;
        let adr_path = std::path::Path::new("/p/_codev/decisions/0001-a-first-choice.md");
        let source = h.fs.read_to_string(adr_path).unwrap();
        let modified = source.replace("## Context", "## Context MODIFIED IN PLACE");
        assert_ne!(modified, source, "the substitution must change the body");
        h.fs.write(adr_path, &modified).unwrap();

        let err = decision_seal(&h.ctx(), "0001", false).unwrap_err();
        assert_eq!(err.code, "seal_conflict");
    }

    #[test]
    fn decision_seal_with_force_rewrites_after_a_change() {
        let h = Harness::new();
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        decision_new(&h.ctx(), "A first choice", "accepted").unwrap();

        use codev_engine::FileSystem;
        let adr_path = std::path::Path::new("/p/_codev/decisions/0001-a-first-choice.md");
        let source = h.fs.read_to_string(adr_path).unwrap();
        let modified = source.replace("## Context", "## Context MODIFIED IN PLACE");
        assert_ne!(modified, source, "the substitution must change the body");
        h.fs.write(adr_path, &modified).unwrap();

        let outcome = decision_seal(&h.ctx(), "0001", true).unwrap();
        assert!(!outcome.was_noop);
        assert!(outcome.was_forced);
    }

    // ─────────────── decision accept ───────────────

    fn seal_yaml(h: &Harness) -> Option<String> {
        use codev_engine::FileSystem;
        h.fs.read_to_string(std::path::Path::new("/p/_codev/decisions/seal.yaml"))
            .ok()
    }

    /// A proposed decision whose body the user has written by hand — the
    /// situation `decision accept` is meant for.
    fn written_proposed_decision(h: &Harness) -> std::path::PathBuf {
        use codev_engine::FileSystem;
        let created = decision_new(&h.ctx(), "A first choice", "proposed").unwrap();
        let path = std::path::PathBuf::from("/p/_codev/decisions/0001-a-first-choice.md");
        assert_eq!(created.path, path);
        let source = h.fs.read_to_string(&path).unwrap();
        let written = source.replace("## Context", "## Context\n\nThe real context.");
        assert_ne!(written, source);
        h.fs.write(&path, &written).unwrap();
        path
    }

    #[test]
    fn decision_accept_seals_a_proposed_decision() {
        use codev_engine::FileSystem;
        let h = Harness::new();
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        let path = written_proposed_decision(&h);
        let before = h.fs.read_to_string(&path).unwrap();

        let outcome = decision_accept(&h.ctx(), "0001").unwrap();

        assert_eq!(outcome.decision.id, "0001");
        assert_eq!(outcome.decision.status, "accepted");
        assert!(outcome.decision.in_effect);
        assert_eq!(outcome.path, path);
        let after = h.fs.read_to_string(&path).unwrap();
        assert!(after.contains("status: accepted"));
        assert_eq!(
            codev_core::decisions::body_slice(&after).unwrap(),
            codev_core::decisions::body_slice(&before).unwrap(),
            "the body stays byte for byte identical"
        );
        let seal = seal_yaml(&h).expect("seal.yaml written");
        assert!(seal.contains("0001"));
        assert!(seal.contains(&outcome.body_sha256));
    }

    #[test]
    fn decision_accept_then_validate_is_clean() {
        // The whole point: write the proposed decision, accept it, and
        // validate neither reports a mismatch nor asks for a seal.
        let h = Harness::new();
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        written_proposed_decision(&h);
        decision_accept(&h.ctx(), "0001").unwrap();

        let report = validate(&h.ctx(), ValidateArgs::All).unwrap();
        let codes: Vec<&str> = report
            .items
            .iter()
            .flat_map(|i| i.findings.iter().map(|f| f.finding.code))
            .collect();
        assert!(
            !codes.contains(&"decision_seal_mismatch") && !codes.contains(&"decision_unsealed"),
            "no seal finding expected; codes: {codes:?}"
        );
    }

    #[test]
    fn decision_accept_refuses_an_accepted_decision() {
        let h = Harness::new();
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        decision_new(&h.ctx(), "Settled", "accepted").unwrap();
        let seal_before = seal_yaml(&h);

        let err = decision_accept(&h.ctx(), "0001").unwrap_err();

        assert_eq!(err.code, "decision_not_proposed");
        assert!(err.message.contains("supersede"));
        assert_eq!(seal_yaml(&h), seal_before, "seal.yaml unchanged");
    }

    #[test]
    fn decision_accept_refuses_an_inherited_decision() {
        let h = Harness::new()
            .with("/p/_codev/config.yaml", "inherits:\n  - path: ~/shared\n")
            .with(
                "/home/shared/_codev/decisions/0100.md",
                "---\nid: \"0100\"\ntitle: Shared\nstatus: proposed\ndate: 2026-09-08\n---\n\n## Context\n\nx\n",
            );
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        let err = decision_accept(&h.ctx(), "path:~/shared/0100").unwrap_err();
        assert_eq!(err.code, "cannot_accept_inherited");
        assert!(seal_yaml(&h).is_none(), "nothing written");
    }

    #[test]
    fn decision_accept_refuses_an_unknown_id() {
        let h = Harness::new();
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        let err = decision_accept(&h.ctx(), "0042").unwrap_err();
        assert_eq!(err.code, "unknown_decision_id");
    }

    #[test]
    fn decision_accept_writes_nothing_when_the_seal_file_is_invalid() {
        use codev_engine::FileSystem;
        let h = Harness::new();
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        let path = written_proposed_decision(&h);
        let adr_before = h.fs.read_to_string(&path).unwrap();
        write_helper(&h, "/p/_codev/decisions/seal.yaml", "not: [a, seal");

        let err = decision_accept(&h.ctx(), "0001").unwrap_err();

        assert_eq!(err.code, "seal_invalid");
        assert_eq!(h.fs.read_to_string(&path).unwrap(), adr_before);
        assert_eq!(seal_yaml(&h).as_deref(), Some("not: [a, seal"));
    }

    // ─────────────── decision supersede ───────────────

    /// Codes of every decision or seal finding `validate` reports — empty
    /// means the decisions are clean.
    fn decision_seal_findings(h: &Harness) -> Vec<&'static str> {
        validate(&h.ctx(), ValidateArgs::All)
            .unwrap()
            .items
            .iter()
            .flat_map(|i| i.findings.iter().map(|f| f.finding.code))
            .filter(|code| code.starts_with("decision_") || code.starts_with("seal_"))
            .collect()
    }

    fn read(h: &Harness, path: &str) -> String {
        use codev_engine::FileSystem;
        h.fs.read_to_string(std::path::Path::new(path)).unwrap()
    }

    #[test]
    fn decision_supersede_creates_a_proposed_decision_and_keeps_the_old_one_in_effect() {
        let h = Harness::new();
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        decision_new(&h.ctx(), "Old choice", "accepted").unwrap();
        let old_before = read(&h, "/p/_codev/decisions/0001-old-choice.md");
        let seal_before = seal_yaml(&h);

        let outcome = decision_supersede(&h.ctx(), "0001", "New choice").unwrap();

        assert_eq!(outcome.new_decision.id, "0002");
        assert_eq!(outcome.new_decision.status, "proposed");
        assert_eq!(outcome.new_decision.supersedes, vec!["0001"]);
        assert!(!outcome.new_decision.in_effect);
        assert_eq!(outcome.old_qualified_id, "project/0001");
        assert_eq!(
            read(&h, "/p/_codev/decisions/0001-old-choice.md"),
            old_before,
            "the old decision is not modified"
        );
        assert_eq!(seal_yaml(&h), seal_before, "seal.yaml unchanged");
        let old = decision_list(&h.ctx())
            .unwrap()
            .decisions
            .into_iter()
            .find(|d| d.id == "0001")
            .unwrap();
        assert!(old.in_effect);
        assert!(old.superseded_by.is_none());
    }

    #[test]
    fn decision_supersede_refuses_a_decision_that_is_not_accepted() {
        let h = Harness::new();
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        decision_new(&h.ctx(), "Tentative", "proposed").unwrap();
        let err = decision_supersede(&h.ctx(), "0001", "X").unwrap_err();
        assert_eq!(err.code, "predecessor_not_accepted");
        use codev_engine::FileSystem;
        assert!(
            !h.fs
                .exists(std::path::Path::new("/p/_codev/decisions/0002-x.md"))
        );
    }

    #[test]
    fn decision_supersede_then_accept_supersedes_the_old_one_and_validates_clean() {
        // End to end: supersede → validate clean → write the body → accept
        // → old superseded, new sealed → validate clean.
        use codev_engine::FileSystem;
        let h = Harness::new();
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        decision_new(&h.ctx(), "Old choice", "accepted").unwrap();
        let old_path = "/p/_codev/decisions/0001-old-choice.md";
        let old_before = read(&h, old_path);
        let old_seal = codev_engine::decisions_actions::read_seal_file(&h.fs, &Layout::new("/p"))
            .unwrap()
            .find("0001")
            .cloned()
            .unwrap();

        decision_supersede(&h.ctx(), "0001", "New choice").unwrap();
        assert!(decision_seal_findings(&h).is_empty());

        // The user writes the new decision's body — no seal to trip over.
        let new_path = std::path::Path::new("/p/_codev/decisions/0002-new-choice.md");
        let source = h.fs.read_to_string(new_path).unwrap();
        h.fs.write(
            new_path,
            &source.replace("## Context", "## Context\n\nWhy it changes."),
        )
        .unwrap();
        assert!(decision_seal_findings(&h).is_empty());

        let accepted = decision_accept(&h.ctx(), "0002").unwrap();

        assert!(accepted.decision.in_effect);
        assert_eq!(accepted.superseded.len(), 1);
        assert_eq!(accepted.superseded[0].qualified_id, "project/0001");
        assert_eq!(accepted.superseded[0].path, std::path::Path::new(old_path));
        let old_after = read(&h, old_path);
        assert!(old_after.contains("status: superseded"));
        assert_eq!(
            codev_core::decisions::body_slice(&old_after).unwrap(),
            codev_core::decisions::body_slice(&old_before).unwrap(),
        );
        let seal =
            codev_engine::decisions_actions::read_seal_file(&h.fs, &Layout::new("/p")).unwrap();
        assert_eq!(seal.find("0001"), Some(&old_seal), "old seal untouched");
        assert_eq!(
            seal.find("0002").map(|s| s.body_sha256.as_str()),
            Some(accepted.body_sha256.as_str())
        );
        let old = decision_list(&h.ctx())
            .unwrap()
            .decisions
            .into_iter()
            .find(|d| d.id == "0001")
            .unwrap();
        assert!(!old.in_effect);
        assert_eq!(old.superseded_by.as_deref(), Some("project/0002"));
        assert!(
            decision_seal_findings(&h).is_empty(),
            "{:?}",
            decision_seal_findings(&h)
        );
    }

    #[test]
    fn decision_accept_refuses_a_second_supersession_of_the_same_decision() {
        let h = Harness::new();
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        decision_new(&h.ctx(), "Old choice", "accepted").unwrap();
        decision_supersede(&h.ctx(), "0001", "First successor").unwrap();
        decision_supersede(&h.ctx(), "0001", "Second successor").unwrap();
        decision_accept(&h.ctx(), "0002").unwrap();
        let second = "/p/_codev/decisions/0003-second-successor.md";
        let second_before = read(&h, second);
        let seal_before = seal_yaml(&h);

        let err = decision_accept(&h.ctx(), "0003").unwrap_err();

        assert_eq!(err.code, "predecessor_not_accepted");
        assert!(err.message.contains("`0001`"), "{}", err.message);
        assert!(err.message.contains("`superseded`"), "{}", err.message);
        assert!(err.message.contains("project/0002"), "{}", err.message);
        assert_eq!(read(&h, second), second_before);
        assert_eq!(seal_yaml(&h), seal_before);
    }

    // ─────────────── decision promote ───────────────

    const DESIGN_WITH_BLOCK: &str = "\
# Design: add-auth

## Decisions

### Decision: Use JWT

The rationale for the choice.

## End
";

    #[test]
    fn decision_promote_creates_a_proposed_adr_and_references_the_design() {
        let h = Harness::new();
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        new_change(&h.ctx(), "add-auth", None, None).unwrap();
        // A design.md with a decision block.
        use codev_engine::FileSystem;
        h.fs.write(
            std::path::Path::new("/p/_codev/changes/add-auth/design.md"),
            DESIGN_WITH_BLOCK,
        )
        .unwrap();

        let outcome = decision_promote(&h.ctx(), "add-auth", "Use JWT").unwrap();
        assert_eq!(outcome.decision.id, "0001");
        assert_eq!(outcome.decision.status, "proposed");
        assert!(!outcome.decision.in_effect);
        assert_eq!(outcome.source_change, "add-auth");
        assert!(seal_yaml(&h).is_none(), "a proposed ADR is not sealed");

        // The ADR exists.
        assert!(h.fs.exists(std::path::Path::new("/p/_codev/decisions/0001-use-jwt.md")));
        // The design was rewritten with the reference.
        let design =
            h.fs.read_to_string(std::path::Path::new("/p/_codev/changes/add-auth/design.md"))
                .unwrap();
        assert!(design.contains("### Decision: Use JWT\n\n> Promoted to ADR **0001**"));
        assert!(!design.contains("The rationale for the choice."));
    }

    #[test]
    fn decision_promote_rework_then_accept_validates_clean() {
        // The promoted body is meant to be reworked: doing so before
        // `decision accept` must never trip the seal.
        use codev_engine::FileSystem;
        let h = Harness::new();
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        new_change(&h.ctx(), "add-auth", None, None).unwrap();
        h.fs.write(
            std::path::Path::new("/p/_codev/changes/add-auth/design.md"),
            DESIGN_WITH_BLOCK,
        )
        .unwrap();
        decision_promote(&h.ctx(), "add-auth", "Use JWT").unwrap();

        let adr = std::path::Path::new("/p/_codev/decisions/0001-use-jwt.md");
        let source = h.fs.read_to_string(adr).unwrap();
        let reworked = source.replace(
            "## Consequences\n",
            "## Consequences\n\nTokens expire; refresh is needed.\n",
        );
        assert_ne!(reworked, source);
        h.fs.write(adr, &reworked).unwrap();
        assert!(decision_seal_findings(&h).is_empty());

        let accepted = decision_accept(&h.ctx(), "0001").unwrap();
        assert_eq!(accepted.decision.status, "accepted");
        assert!(accepted.superseded.is_empty());
        assert!(
            decision_seal_findings(&h).is_empty(),
            "{:?}",
            decision_seal_findings(&h)
        );
    }

    #[test]
    fn decision_promote_rejects_a_missing_change() {
        let h = Harness::new();
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        let err = decision_promote(&h.ctx(), "ghost", "X").unwrap_err();
        assert_eq!(err.code, "unknown_change");
    }

    #[test]
    fn decision_promote_rejects_a_missing_design() {
        let h = Harness::new();
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        new_change(&h.ctx(), "add-auth", None, None).unwrap();
        // No design.md created.
        let err = decision_promote(&h.ctx(), "add-auth", "X").unwrap_err();
        assert_eq!(err.code, "design_missing");
    }

    #[test]
    fn decision_promote_rejects_a_missing_title() {
        let h = Harness::new();
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        new_change(&h.ctx(), "add-auth", None, None).unwrap();
        use codev_engine::FileSystem;
        h.fs.write(
            std::path::Path::new("/p/_codev/changes/add-auth/design.md"),
            DESIGN_WITH_BLOCK,
        )
        .unwrap();
        let err = decision_promote(&h.ctx(), "add-auth", "Ghost").unwrap_err();
        assert_eq!(err.code, "decision_heading_not_found");
    }

    #[test]
    fn decision_promote_rejects_an_archived_change() {
        // Simulate an archived change by creating the folder directly under
        // `archive/<date>-<name>/`.
        let h = Harness::new().with(
            "/p/_codev/changes/archive/2026-09-01-old/design.md",
            DESIGN_WITH_BLOCK,
        );
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        let err = decision_promote(&h.ctx(), "old", "Use JWT").unwrap_err();
        assert_eq!(err.code, "cannot_promote_from_archived");
    }

    // ─────────────── decision deviate ───────────────

    fn inherited_adr(id: &str) -> String {
        format!(
            "---\nid: \"{id}\"\ntitle: Source choice\nstatus: accepted\ndate: 2026-09-08\n---\n\n## Context\n\nx\n"
        )
    }

    #[test]
    fn decision_deviate_creates_a_proposed_adr_without_seal() {
        let h = Harness::new()
            .with("/p/_codev/config.yaml", "inherits:\n  - path: ~/shared\n")
            .with(
                "/home/shared/_codev/decisions/0100.md",
                &inherited_adr("0100"),
            );
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        let outcome = decision_deviate(&h.ctx(), "path:~/shared/0100", "Our alternative").unwrap();

        assert_eq!(outcome.target_qualified_id, "path:~/shared/0100");
        assert_eq!(outcome.decision.id, "0001");
        assert_eq!(outcome.decision.status, "proposed");
        assert_eq!(outcome.decision.deviates_from, vec!["path:~/shared/0100"]);
        assert!(
            seal_yaml(&h).is_none(),
            "a proposed deviation is not sealed"
        );
    }

    #[test]
    fn decision_deviate_has_no_effect_until_accepted() {
        let h = Harness::new()
            .with("/p/_codev/config.yaml", "inherits:\n  - path: ~/shared\n")
            .with(
                "/home/shared/_codev/decisions/0100.md",
                &inherited_adr("0100"),
            );
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        decision_deviate(&h.ctx(), "path:~/shared/0100", "Our alt").unwrap();

        let find = |qualified: &str| {
            decision_list(&h.ctx())
                .unwrap()
                .decisions
                .into_iter()
                .find(|d| d.qualified_id == qualified)
                .expect("decision listed")
        };
        // Proposed: the inherited decision is still in effect.
        let inherited = find("path:~/shared/0100");
        assert!(inherited.in_effect);
        assert!(inherited.deviated_by.is_none());
        assert!(!find("project/0001").in_effect);
        assert!(decision_seal_findings(&h).is_empty());

        // Accepted: the deviation takes effect.
        let accepted = decision_accept(&h.ctx(), "0001").unwrap();
        assert!(accepted.decision.in_effect);
        let inherited = find("path:~/shared/0100");
        assert!(!inherited.in_effect);
        assert_eq!(inherited.deviated_by.as_deref(), Some("project/0001"));
        assert!(seal_yaml(&h).is_some_and(|s| s.contains("0001")));
        assert!(decision_seal_findings(&h).is_empty());
    }

    #[test]
    fn decision_deviate_rejects_a_local_one_and_points_to_supersede() {
        let h = Harness::new();
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        decision_new(&h.ctx(), "A local one", "accepted").unwrap();
        let err = decision_deviate(&h.ctx(), "project/0001", "…").unwrap_err();
        assert_eq!(err.code, "cannot_deviate_from_local");
        assert!(err.message.contains("supersede"));
    }

    #[test]
    fn decision_deviate_rejects_an_unknown_target() {
        let h = Harness::new();
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        let err = decision_deviate(&h.ctx(), "path:~/unknown/0100", "…").unwrap_err();
        assert_eq!(err.code, "unknown_decision_id");
    }

    #[test]
    fn decision_list_exposes_deviated_by_on_the_inherited_one() {
        let h = Harness::new()
            .with("/p/_codev/config.yaml", "inherits:\n  - path: ~/shared\n")
            .with(
                "/home/shared/_codev/decisions/0100.md",
                &inherited_adr("0100"),
            );
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        decision_deviate(&h.ctx(), "path:~/shared/0100", "Our alt").unwrap();
        decision_accept(&h.ctx(), "0001").unwrap();
        let list = decision_list(&h.ctx()).unwrap();

        let inherited = list
            .decisions
            .iter()
            .find(|d| d.qualified_id == "path:~/shared/0100")
            .expect("inherited decision present in the listing");
        assert_eq!(inherited.deviated_by.as_deref(), Some("project/0001"));
        assert!(!inherited.in_effect);

        let local = list
            .decisions
            .iter()
            .find(|d| d.qualified_id == "project/0001")
            .expect("local decision present");
        assert_eq!(local.deviates_from, vec!["path:~/shared/0100"]);
        assert!(local.in_effect);
    }

    // ─────────────── validate --strict ───────────────

    #[test]
    fn validate_clean_project_has_neither_error_nor_warning() {
        // Baseline for strict mode: without findings, neither `has_errors`
        // nor `has_warnings` is raised.
        let h = Harness::new();
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        new_change(&h.ctx(), "add-auth", None, None).unwrap();
        // A single artifact is enough for validate_change to report no error
        // (simple proposal).
        use codev_engine::FileSystem;
        h.fs.write(
            std::path::Path::new("/p/_codev/changes/add-auth/proposal.md"),
            "# Proposal\n\n## Why\n\nT\n\n## What Changes\n\n- x\n\n## Domains\n\nNone\n\n## Release note\n\nNone\n",
        )
        .unwrap();
        h.fs
            .write(
                std::path::Path::new("/p/_codev/changes/add-auth/specs/x/spec.md"),
                "## Purpose\n\nx.\n\n## ADDED Requirements\n\n### Requirement: X\nThe system SHALL x.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n",
            )
            .unwrap();
        let report = validate(&h.ctx(), ValidateArgs::All).unwrap();
        assert!(!report.has_errors());
        assert!(!report.has_warnings(), "{:#?}", report);
    }

    #[test]
    fn validate_with_unsealed_adr_has_warnings_but_no_errors() {
        // A local `accepted` ADR without seal.yaml → `decision_unsealed`
        // warning. It is the typical case where `--strict` flips the CLI's
        // exit code.
        let h = Harness::new().with(
            "/p/_codev/decisions/0001.md",
            "---\nid: \"0001\"\ntitle: T\nstatus: accepted\ndate: 2026-09-08\n---\n\n## Context\n\nx\n",
        );
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        let report = validate(&h.ctx(), ValidateArgs::All).unwrap();
        assert!(!report.has_errors(), "warnings only");
        assert!(report.has_warnings(), "at least one warning expected");
    }

    #[test]
    fn validate_with_seal_mismatch_has_an_error() {
        // A mismatch is an error, independent of strict mode.
        let h = Harness::new();
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        decision_new(&h.ctx(), "A first choice", "accepted").unwrap();
        // Corrupt the ADR's body after sealing.
        use codev_engine::FileSystem;
        let adr_path = std::path::Path::new("/p/_codev/decisions/0001-a-first-choice.md");
        let source = h.fs.read_to_string(adr_path).unwrap();
        let modified = source.replace("## Context", "## Context ALTERED");
        h.fs.write(adr_path, &modified).unwrap();

        let report = validate(&h.ctx(), ValidateArgs::All).unwrap();
        assert!(report.has_errors(), "mismatch → error");
        // The mismatch is not a warning; has_warnings may be false or true
        // depending on the other findings, but the exit code would have
        // flipped on `has_errors` alone, without depending on strict.
    }

    // ─────────────── init --inherit, integration, update, doctor ───────────────

    fn git_ok(stdout: &str) -> codev_engine::ProcessOutput {
        codev_engine::ProcessOutput {
            stdout: stdout.as_bytes().to_vec(),
            stderr: Vec::new(),
            exit_code: 0,
        }
    }

    fn reachable_git() -> MockProcessRunner {
        MockProcessRunner::new()
            .with_response("git", &["--version"], git_ok("git version 2.45.0\n"))
            .with_response(
                "git",
                &["ls-remote"],
                git_ok("9f2c1ab7d00d\trefs/heads/main\n"),
            )
            .with_response("git", &["init"], git_ok(""))
            .with_response("git", &["-C"], git_ok(""))
    }

    fn opts_with(inherits: &[&str]) -> crate::init_prompts::InitOptions {
        crate::init_prompts::InitOptions {
            inherits: inherits.iter().map(|s| s.to_string()).collect(),
            ..test_init_opts()
        }
    }

    #[test]
    fn init_writes_inherits_and_pins_git_sources() {
        let h = Harness::new()
            .with("/home/shared/_codev/config.yaml", "context: |\n  Shared\n")
            .with_runner(reachable_git());
        let outcome = init(
            &h.ctx(),
            ".",
            false,
            &opts_with(&[
                "git@github.com:acme/conventions.git@v2",
                "~/shared",
                "../home/shared",
            ]),
        )
        .unwrap();

        let config = h.fs.read("/p/_codev/config.yaml").unwrap();
        assert!(
            config.contains(
                "# set with codev init --inherit\ninherits:\n  - git: git@github.com:acme/conventions.git\n    ref: v2\n  - path: ~/shared\n  - path: /home/shared\n"
            ),
            "{config}"
        );
        let lock = h.fs.read("/p/_codev/codev.lock").unwrap();
        assert!(lock.contains("commit = \"9f2c1ab7d00d\""), "{lock}");
        assert!(lock.contains("ref = \"v2\""), "{lock}");
        assert!(matches!(
            outcome.pins.as_slice(),
            [engine_sources::PinChange::Added { .. }]
        ));
        assert!(
            outcome
                .warnings
                .iter()
                .all(|w| w.code != "inherit_pin_failed")
        );
        // The written config reads back.
        config::resolve(&h.fs, &h.env, &Layout::new("/p")).unwrap();
    }

    #[test]
    fn init_refuses_a_missing_inherit_path_before_writing() {
        let h = Harness::new();
        for bad in ["./does-not-exist", "", "git@github.com:acme/c.git@"] {
            let err = init(&h.ctx(), ".", false, &opts_with(&[bad])).unwrap_err();
            assert_eq!(err.code, "invalid_inherit", "{bad}");
        }
        assert!(h.fs.paths().is_empty(), "{:?}", h.fs.paths());
    }

    #[test]
    fn init_keeps_an_existing_config_and_warns_inherit_not_written() {
        let original = "schema: spec-driven\n";
        let h = Harness::new()
            .with("/p/_codev/config.yaml", original)
            .with("/home/shared/_codev/config.yaml", "");
        let outcome = init(&h.ctx(), ".", false, &opts_with(&["/home/shared"])).unwrap();
        assert_eq!(h.fs.read("/p/_codev/config.yaml").unwrap(), original);
        let warning = outcome
            .warnings
            .iter()
            .find(|w| w.code == "inherit_not_written")
            .expect("inherit_not_written");
        assert!(warning.message.replace('\\', "/").contains("/home/shared"));
    }

    #[test]
    fn init_survives_a_pin_failure() {
        let runner = MockProcessRunner::new()
            .with_response("git", &["--version"], git_ok("git version 2.45.0\n"))
            .with_response(
                "git",
                &["ls-remote"],
                codev_engine::ProcessOutput {
                    stdout: Vec::new(),
                    stderr: b"fatal: unable to access 'https://unreachable.example/'".to_vec(),
                    exit_code: 128,
                },
            );
        let h = Harness::new().with_runner(runner);
        let outcome = init(
            &h.ctx(),
            ".",
            false,
            &opts_with(&["https://unreachable.example/acme/c.git"]),
        )
        .unwrap();
        assert!(
            h.fs.read("/p/_codev/config.yaml")
                .unwrap()
                .contains("git: https://unreachable.example/acme/c.git")
        );
        assert!(h.fs.read("/p/_codev/codev.lock").is_none());
        assert!(
            h.fs.read("/p/.claude/skills/codev-propose/SKILL.md")
                .is_some()
        );
        assert!(
            outcome
                .warnings
                .iter()
                .any(|w| w.code == "inherit_pin_failed" && w.message.contains("unable to access"))
        );
    }

    fn action_of(outcome: &SetupOutcome, step: integration::Step) -> integration::StepAction {
        outcome
            .integration
            .iter()
            .find(|s| s.step == step)
            .unwrap_or_else(|| panic!("no step {step:?}: {:?}", outcome.integration))
            .action
    }

    #[test]
    fn init_writes_the_three_integration_files_on_github() {
        use integration::{Step, StepAction};
        let h = Harness::new().with("/p/.github/dependabot.yml", "version: 2\n");
        let outcome = init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        for step in [Step::ClaudeMd, Step::Ci, Step::PrTemplate] {
            assert_eq!(action_of(&outcome, step), StepAction::Created, "{step:?}");
        }
        let claude = h.fs.read("/p/CLAUDE.md").unwrap();
        assert!(claude.contains("/codev-propose") && claude.contains("/codev-apply"));
        let workflow = h.fs.read("/p/.github/workflows/codev.yml").unwrap();
        assert!(workflow.contains("CODEV_VERSION=0.1.0"));
        assert!(workflow.contains("codev validate --strict"));
        let template = h.fs.read("/p/.github/PULL_REQUEST_TEMPLATE.md").unwrap();
        assert!(template.contains("codev change"));
        assert!(template.contains("`codev validate --strict` passes"));
    }

    #[test]
    fn init_outside_github_writes_only_claude_md_and_says_why() {
        use integration::{Step, StepAction};
        let h = Harness::new();
        let outcome = init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        assert_eq!(action_of(&outcome, Step::ClaudeMd), StepAction::Created);
        assert_eq!(action_of(&outcome, Step::Ci), StepAction::Skipped);
        assert_eq!(action_of(&outcome, Step::PrTemplate), StepAction::Skipped);
        assert!(!h.fs.exists(Path::new("/p/.github")));
    }

    #[test]
    fn init_appends_to_an_existing_claude_md() {
        let user = "# Team rules\n\nAlways write tests.\n";
        let h = Harness::new().with("/p/CLAUDE.md", user);
        let outcome = init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        assert_eq!(
            action_of(&outcome, integration::Step::ClaudeMd),
            integration::StepAction::Appended
        );
        let written = h.fs.read("/p/CLAUDE.md").unwrap();
        assert!(written.starts_with(user));
        assert!(managed::has_section(&written));
    }

    #[test]
    fn init_no_integration_writes_none() {
        let h = Harness::new().with("/p/.github/dependabot.yml", "version: 2\n");
        let opts = crate::init_prompts::InitOptions {
            no_claude_md: true,
            no_ci: true,
            no_pr_template: true,
            ..test_init_opts()
        };
        let outcome = init(&h.ctx(), ".", false, &opts).unwrap();
        assert!(outcome.integration.is_empty());
        assert!(h.fs.read("/p/CLAUDE.md").is_none());
        assert!(h.fs.read("/p/.github/workflows/codev.yml").is_none());
        assert!(h.fs.read("/p/.github/PULL_REQUEST_TEMPLATE.md").is_none());
    }

    #[test]
    fn init_no_ci_skips_only_the_workflow() {
        let h = Harness::new().with("/p/.github/dependabot.yml", "version: 2\n");
        let opts = crate::init_prompts::InitOptions {
            no_ci: true,
            ..test_init_opts()
        };
        init(&h.ctx(), ".", false, &opts).unwrap();
        assert!(h.fs.read("/p/CLAUDE.md").is_some());
        assert!(h.fs.read("/p/.github/PULL_REQUEST_TEMPLATE.md").is_some());
        assert!(h.fs.read("/p/.github/workflows/codev.yml").is_none());
    }

    #[test]
    fn rerunning_init_with_integration_is_idempotent() {
        let h = Harness::new().with("/p/.github/dependabot.yml", "version: 2\n");
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        let second = init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        assert!(!second.changed_anything(), "{:?}", second.updated);
        assert!(
            second
                .integration
                .iter()
                .all(|s| s.action == integration::StepAction::Unchanged)
        );
    }

    #[test]
    fn update_refreshes_the_claude_md_section_and_adds_nothing_when_opted_out() {
        let stale = format!(
            "Team intro.\n\n{}\nTeam outro.\n",
            managed::render_section("old codev text")
        );
        let h = Harness::new()
            .with("/p/_codev/config.yaml", "schema: spec-driven\n")
            .with("/p/CLAUDE.md", &stale)
            .with("/p/.github/dependabot.yml", "version: 2\n");
        let outcome = update(&h.ctx(), false).unwrap();
        let written = h.fs.read("/p/CLAUDE.md").unwrap();
        assert!(written.starts_with("Team intro.\n\n<!-- codev:begin"));
        assert!(written.ends_with("<!-- codev:end -->\n\nTeam outro.\n"));
        assert!(!written.contains("old codev text"));
        assert_eq!(
            action_of(&outcome, integration::Step::ClaudeMd),
            integration::StepAction::Refreshed
        );
        // GitHub project without workflow nor template: update adds neither.
        assert!(h.fs.read("/p/.github/workflows/codev.yml").is_none());
        assert!(h.fs.read("/p/.github/PULL_REQUEST_TEMPLATE.md").is_none());
        let again = update(&h.ctx(), false).unwrap();
        assert!(!again.changed_anything());
    }

    #[test]
    fn init_records_the_doctor_count_for_the_hint() {
        let h = Harness::new();
        let outcome = init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        // rules, specs, ci are missing on a fresh project outside GitHub.
        assert_eq!(outcome.doctor_missing, Some(3));
    }

    fn check<'a>(outcome: &'a DoctorOutcome, id: &str) -> &'a Check {
        outcome
            .checks
            .iter()
            .find(|c| c.id == id)
            .unwrap_or_else(|| panic!("no check {id}"))
    }

    #[test]
    fn doctor_on_a_fresh_project_reports_the_expected_statuses() {
        let h = Harness::new();
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        let outcome = doctor(&h.ctx()).unwrap();
        assert_eq!(check(&outcome, "config.rules").status, CheckStatus::Missing);
        assert_eq!(check(&outcome, "specs").status, CheckStatus::Missing);
        assert_eq!(check(&outcome, "decisions").status, CheckStatus::Warning);
        assert_eq!(check(&outcome, "ci").status, CheckStatus::Missing);
        for id in [
            "config.language",
            "config.roles",
            "skills",
            "sources",
            "codev_version",
            "claude_md",
            "validate",
        ] {
            assert_eq!(check(&outcome, id).status, CheckStatus::Ok, "{id}");
        }
        assert!(outcome.checks.iter().all(|c| c.id != "pr_template"));
    }

    #[test]
    fn doctor_lists_an_expert_forced_by_the_project() {
        let h = Harness::new();
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        let config = h.fs.read("/p/_codev/config.yaml").unwrap();
        let h = h.with(
            "/p/_codev/config.yaml",
            &format!("{config}\nroles:\n  security: always\n  ops: never\n"),
        );
        let outcome = doctor(&h.ctx()).unwrap();
        let message = &check(&outcome, "config.roles").message;
        let (active, rest) = message.split_once("; on domain: ").unwrap();
        let (on_domain, disabled) = rest.split_once("; disabled: ").unwrap();
        assert!(active.contains("security"), "{message}");
        assert!(
            !on_domain.contains("security") && !on_domain.contains("ops"),
            "{message}"
        );
        assert_eq!(disabled, "ops", "{message}");
    }

    #[test]
    fn doctor_lists_the_roles_in_effect() {
        let h = Harness::new();
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        let config = h.fs.read("/p/_codev/config.yaml").unwrap();
        let h = h.with(
            "/p/_codev/config.yaml",
            &format!("{config}\nroles:\n  newcomer: never\n"),
        );
        let outcome = doctor(&h.ctx()).unwrap();
        let roles = check(&outcome, "config.roles");
        assert_eq!(roles.status, CheckStatus::Ok);
        assert!(
            roles
                .message
                .contains("active roles: analyst, investigator, product"),
            "{}",
            roles.message
        );
        assert!(
            roles.message.ends_with("disabled: newcomer"),
            "{}",
            roles.message
        );
        assert!(
            roles
                .message
                .contains("on domain: security (security), privacy (personal-data)"),
            "{}",
            roles.message
        );

        let h = h.with(
            "/p/_codev/config.yaml",
            "language: en\nroles:\n  reviewer: never\n",
        );
        let outcome = doctor(&h.ctx()).unwrap();
        assert_eq!(check(&outcome, "config.roles").code, "roles_invalid");
        assert_eq!(check(&outcome, "validate").status, CheckStatus::Missing);
    }

    #[test]
    fn doctor_reports_outdated_skills_and_unpinned_sources() {
        let h = Harness::new()
            .with(
                "/p/_codev/config.yaml",
                "language: en\nworkflows: [propose]\ninherits:\n  - git: https://example.com/c.git\n    ref: main\n",
            )
            .with(
                "/p/.claude/skills/codev-propose/SKILL.md",
                "---\nname: codev-propose\nmetadata:\n  version: \"0.0.1\"\n---\nbody\n",
            );
        let outcome = doctor(&h.ctx()).unwrap();
        assert_eq!(check(&outcome, "skills").code, "skills_outdated");
        assert_eq!(check(&outcome, "sources").code, "source_unpinned");
    }

    #[test]
    fn doctor_recognizes_cargo_run_validate_in_ci() {
        let h = Harness::new()
            .with("/p/_codev/config.yaml", "language: en\n")
            .with(
                "/p/.github/workflows/ci.yml",
                "jobs:\n  d:\n    steps:\n      - run: cargo run --locked --quiet --bin codev -- validate --strict\n",
            );
        let outcome = doctor(&h.ctx()).unwrap();
        assert_eq!(check(&outcome, "ci").status, CheckStatus::Ok);
        assert_eq!(
            check(&outcome, "pr_template").code,
            "pr_template_section_missing"
        );
    }

    #[test]
    fn doctor_writes_nothing() {
        let h = Harness::new();
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        let before: Vec<(PathBuf, Option<String>)> =
            h.fs.paths()
                .into_iter()
                .map(|p| {
                    let c = h.fs.read(&p);
                    (p, c)
                })
                .collect();
        doctor(&h.ctx()).unwrap();
        let after: Vec<(PathBuf, Option<String>)> =
            h.fs.paths()
                .into_iter()
                .map(|p| {
                    let c = h.fs.read(&p);
                    (p, c)
                })
                .collect();
        assert_eq!(before, after);
        assert!(h.runner.calls().is_empty());
    }

    #[test]
    fn doctor_outside_a_project_fails_with_no_codev_root() {
        let h = Harness::new();
        let err = doctor(&h.ctx()).err().expect("doctor must fail");
        assert_eq!(err.code, "no_codev_root");
    }

    // ─────────────── changelog ───────────────

    fn archived(name: &str, note: Option<&str>) -> (String, String) {
        let mut proposal =
            format!("# Proposal: {name}\n\n## Why\n\nx\n\n## What Changes\n\n- what {name}\n");
        if let Some(note) = note {
            proposal.push_str(&format!("\n## Release note\n\n{note}\n"));
        }
        (
            format!("/p/_codev/changes/archive/{name}/proposal.md"),
            proposal,
        )
    }

    fn listing(names: &[&str]) -> String {
        names
            .iter()
            .map(|n| format!("_codev/changes/archive/{n}\n"))
            .collect()
    }

    /// `2026-01-01-a` is tagged `v1.0.0`; `2026-02-01-b` and `2026-03-01-c`
    /// came after.
    fn changelog_project() -> Harness {
        let mut h = Harness::new().with("/p/_codev/config.yaml", "language: en\n");
        for (name, note) in [
            ("2026-01-01-a", Some("- Added: a.")),
            (
                "2026-02-01-b",
                Some("- Fixed: b.\n- Added: **BREAKING** b."),
            ),
            ("2026-03-01-c", Some("None")),
        ] {
            let (path, proposal) = archived(name, note);
            h = h.with(&path, &proposal);
        }
        h.with_runner(
            MockProcessRunner::new()
                .with_response("git", &["rev-parse"], git_ok("true\n"))
                .with_response("git", &["tag"], git_ok("v1.0.0\n"))
                .with_response(
                    "git",
                    &["ls-tree", "-d", "--name-only", "v1.0.0"],
                    git_ok(&listing(&["2026-01-01-a"])),
                ),
        )
    }

    fn changelog_args() -> ChangelogArgs {
        ChangelogArgs::default()
    }

    #[test]
    fn changelog_lists_the_changes_since_the_latest_tag() {
        let h = changelog_project();
        let outcome =
            changelog(&h.ctx(), ChangelogFormat::KeepAChangelog, &changelog_args()).unwrap();
        assert_eq!(outcome.since.as_deref(), Some("v1.0.0"));
        assert_eq!(
            outcome.rendered,
            "## [Unreleased]\n\n### Added\n\n- **BREAKING** b.\n\n### Fixed\n\n- b.\n"
        );
        assert!(outcome.warnings.is_empty(), "{:?}", outcome.warnings);
        assert!(outcome.written.is_none());
        let markdown = changelog(&h.ctx(), ChangelogFormat::Markdown, &changelog_args()).unwrap();
        assert!(markdown.rendered.starts_with("### Added"));
    }

    #[test]
    fn changelog_excludes_the_configured_changes() {
        let h = changelog_project().with(
            "/p/_codev/config.yaml",
            "language: en\nchangelog:\n  exclude:\n    - 2026-02-01-b\n",
        );
        let outcome =
            changelog(&h.ctx(), ChangelogFormat::KeepAChangelog, &changelog_args()).unwrap();
        assert!(outcome.entries.is_empty());
        assert!(outcome.warnings.is_empty());
    }

    #[test]
    fn changelog_reports_fallback_and_inconsistency_warnings() {
        let (path, proposal) = archived("2026-04-01-old", None);
        let (path2, proposal2) = archived("2026-05-01-drop", Some("- Added: x."));
        let h = changelog_project()
            .with(&path, &proposal)
            .with(&path2, &proposal2)
            .with(
                "/p/_codev/changes/archive/2026-05-01-drop/specs/s/spec.md",
                "## REMOVED Requirements\n\n### Requirement: Old\n**Reason**: gone\n**Migration**: none\n",
            );
        let outcome =
            changelog(&h.ctx(), ChangelogFormat::KeepAChangelog, &changelog_args()).unwrap();
        let codes: Vec<&str> = outcome.warnings.iter().map(|w| w.code).collect();
        assert_eq!(codes, ["release_note_missing", "release_note_inconsistent"]);
        assert!(
            outcome
                .rendered
                .contains("### Changed\n\n- what 2026-04-01-old\n")
        );
    }

    #[test]
    fn changelog_release_heading() {
        let h = changelog_project();
        let args = ChangelogArgs {
            release: Some("v1.2.0".into()),
            date: Some("2026-10-20".into()),
            ..changelog_args()
        };
        let outcome = changelog(&h.ctx(), ChangelogFormat::KeepAChangelog, &args).unwrap();
        assert!(outcome.rendered.starts_with("## [1.2.0] - 2026-10-20\n"));
        // The date defaults to today's.
        let args = ChangelogArgs {
            release: Some("1.2.0".into()),
            ..changelog_args()
        };
        let outcome = changelog(&h.ctx(), ChangelogFormat::KeepAChangelog, &args).unwrap();
        assert!(outcome.rendered.starts_with("## [1.2.0] - 2026-09-08\n"));
    }

    #[test]
    fn changelog_rejects_an_invalid_version_or_date() {
        let h = changelog_project();
        let args = ChangelogArgs {
            release: Some("next".into()),
            ..changelog_args()
        };
        let err = changelog(&h.ctx(), ChangelogFormat::KeepAChangelog, &args)
            .err()
            .unwrap();
        assert_eq!(err.code, "invalid_release_version");
        let args = ChangelogArgs {
            release: Some("1.0.0".into()),
            date: Some("20/10/2026".into()),
            ..changelog_args()
        };
        let err = changelog(&h.ctx(), ChangelogFormat::KeepAChangelog, &args)
            .err()
            .unwrap();
        assert_eq!(err.code, "invalid_date");
    }

    fn write_args() -> ChangelogArgs {
        ChangelogArgs {
            write: true,
            ..changelog_args()
        }
    }

    #[test]
    fn changelog_write_creates_then_is_idempotent() {
        let h = changelog_project();
        let outcome = changelog(&h.ctx(), ChangelogFormat::KeepAChangelog, &write_args()).unwrap();
        assert_eq!(
            outcome.written,
            Some((PathBuf::from("/p/CHANGELOG.md"), ChangelogWrite::Created))
        );
        let text = read(&h, "/p/CHANGELOG.md");
        assert!(text.starts_with("# Changelog\n"));
        assert!(text.contains("## [Unreleased]\n\n### Added\n\n- **BREAKING** b."));
        let again = changelog(&h.ctx(), ChangelogFormat::KeepAChangelog, &write_args()).unwrap();
        assert_eq!(again.written.unwrap().1, ChangelogWrite::Unchanged);
        assert_eq!(read(&h, "/p/CHANGELOG.md"), text);
    }

    #[test]
    fn changelog_write_keeps_hand_written_content() {
        let hand = "# Changelog\n\n## [Unreleased]\n\n- curated.\n\n## [1.0.0] - 2026-01-01\n\n- first.\n\n[1.0.0]: https://example.com/v1.0.0\n";
        let h = changelog_project().with("/p/CHANGELOG.md", hand);
        let outcome = changelog(&h.ctx(), ChangelogFormat::KeepAChangelog, &write_args()).unwrap();
        assert_eq!(outcome.written.unwrap().1, ChangelogWrite::Updated);
        let text = read(&h, "/p/CHANGELOG.md");
        assert_eq!(text.matches("## [Unreleased]").count(), 1);
        assert!(text.contains("<!-- codev:end -->\n\n- curated.\n\n## [1.0.0]"));
        assert!(text.ends_with("[1.0.0]: https://example.com/v1.0.0\n"));
    }

    #[test]
    fn changelog_write_release_rotates() {
        let h = changelog_project();
        changelog(&h.ctx(), ChangelogFormat::KeepAChangelog, &write_args()).unwrap();
        let args = ChangelogArgs {
            release: Some("1.1.0".into()),
            date: Some("2026-10-20".into()),
            ..write_args()
        };
        changelog(&h.ctx(), ChangelogFormat::KeepAChangelog, &args).unwrap();
        let text = read(&h, "/p/CHANGELOG.md");
        assert_eq!(
            codev_core::changelog::file::managed_block(&text).as_deref(),
            Some("## [Unreleased]")
        );
        assert!(text.contains("<!-- codev:end -->\n\n## [1.1.0] - 2026-10-20\n\n### Added"));
    }

    #[test]
    fn changelog_write_refuses_markdown() {
        let h = changelog_project();
        let err = changelog(&h.ctx(), ChangelogFormat::Markdown, &write_args())
            .err()
            .unwrap();
        assert_eq!(err.code, "invalid_argument");
        assert!(!h.fs.paths().contains(&PathBuf::from("/p/CHANGELOG.md")));
    }

    #[test]
    fn changelog_write_refuses_malformed_markers_without_writing() {
        let broken =
            "# Changelog\n\n<!-- codev:begin -->\na\n<!-- codev:begin -->\n<!-- codev:end -->\n";
        let h = changelog_project().with("/p/CHANGELOG.md", broken);
        let err = changelog(&h.ctx(), ChangelogFormat::KeepAChangelog, &write_args())
            .err()
            .unwrap();
        assert_eq!(err.code, "changelog_malformed");
        assert_eq!(read(&h, "/p/CHANGELOG.md"), broken);
    }

    #[test]
    fn changelog_unknown_ref_fails() {
        let h = changelog_project();
        let args = ChangelogArgs {
            since: Some("v9.9.9".into()),
            ..changelog_args()
        };
        // No mock response for that ref: git refuses it.
        let err = changelog(&h.ctx(), ChangelogFormat::KeepAChangelog, &args)
            .err()
            .unwrap();
        assert_eq!(err.code, "unknown_ref");
    }

    #[test]
    fn changelog_json_carries_entries_and_status() {
        let (path, proposal) = archived("2026-04-01-old", None);
        let h = changelog_project().with(&path, &proposal);
        let outcome = changelog(&h.ctx(), ChangelogFormat::Json, &changelog_args()).unwrap();
        let json = serde_json::to_value(crate::contract::ChangelogV1::from(&outcome)).unwrap();
        assert_eq!(json["since"], "v1.0.0");
        assert_eq!(json["until"], serde_json::Value::Null);
        let first = &json["entries"][0];
        assert_eq!(first["category"], "Added");
        assert_eq!(first["text"], "**BREAKING** b.");
        assert_eq!(first["breaking"], true);
        assert_eq!(first["change"], "2026-02-01-b");
        assert_eq!(first["archivePath"], "_codev/changes/archive/2026-02-01-b");
        assert_eq!(first["source"], "release_note");
        assert_eq!(json["entries"][1]["source"], "what_changes");
        assert_eq!(json["status"][0]["code"], "release_note_missing");
        assert!(
            json["markdown"]
                .as_str()
                .unwrap()
                .starts_with("## [Unreleased]")
        );
    }

    #[test]
    fn validate_strict_flags_a_missing_release_note() {
        let h = Harness::new();
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        new_change(&h.ctx(), "add-auth", None, None).unwrap();
        write_helper(
            &h,
            "/p/_codev/changes/add-auth/proposal.md",
            "# Proposal\n\n## Why\n\nT\n",
        );
        write_helper(
            &h,
            "/p/_codev/changes/add-auth/specs/x/spec.md",
            "## Purpose\n\nx.\n\n## ADDED Requirements\n\n### Requirement: X\nThe system SHALL x.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n",
        );
        let report = validate(&h.ctx(), ValidateArgs::All).unwrap();
        // A warning: the exit code only flips under --strict.
        assert!(!report.has_errors());
        assert!(report.has_warnings());
    }

    #[test]
    fn doctor_reports_a_missing_changelog() {
        let h = changelog_project();
        let outcome = doctor(&h.ctx()).unwrap();
        let changelog = check(&outcome, "changelog");
        assert_eq!(changelog.status, CheckStatus::Warning);
        assert_eq!(changelog.code, "changelog_missing");
    }

    #[test]
    fn doctor_changelog_is_ok_after_write() {
        let h = changelog_project();
        changelog(&h.ctx(), ChangelogFormat::KeepAChangelog, &write_args()).unwrap();
        let outcome = doctor(&h.ctx()).unwrap();
        assert_eq!(check(&outcome, "changelog").status, CheckStatus::Ok);
        // Only local, read-only git queries.
        for (program, args) in h.runner.calls() {
            assert_eq!(program, "git");
            assert!(
                ["rev-parse", "tag", "ls-tree"].contains(&args[0].as_str()),
                "{args:?}"
            );
        }
    }

    #[test]
    fn doctor_changelog_is_outdated_after_a_new_archive() {
        let h = changelog_project();
        changelog(&h.ctx(), ChangelogFormat::KeepAChangelog, &write_args()).unwrap();
        let (path, proposal) = archived("2026-04-01-new", Some("- Fixed: new."));
        write_helper(&h, &path, &proposal);
        let outcome = doctor(&h.ctx()).unwrap();
        let changelog = check(&outcome, "changelog");
        assert_eq!(changelog.code, "changelog_outdated");
        assert_eq!(changelog.fix.as_deref(), Some("codev changelog --write"));
    }

    #[test]
    fn release_workflow_generates_the_body_with_codev_changelog() {
        // Publishing cannot run without a tag: the workflow is checked
        // statically instead.
        let workflow = include_str!("../../../.github/workflows/release.yml");
        let flat: String = workflow.split_whitespace().collect::<Vec<_>>().join(" ");
        for needle in [
            "fetch-depth: 0",
            "stage=\"codev-${version}-x86_64-unknown-linux-musl\"",
            "changelog --until \"$GITHUB_REF_NAME\" --format markdown",
            "body_path: release-notes.md",
            "generate_release_notes: true",
        ] {
            assert!(flat.contains(needle), "release.yml must contain `{needle}`");
        }
    }

    // ─────────────── update notice, codev_version, self-update ───────────────

    const UPDATE_CACHE: &str = "/home/.cache/codev/update-check.json";

    fn release_json(version: &str) -> String {
        format!(
            r#"{{"tag_name": "v{version}", "html_url": "https://github.com/mairistem/codev/releases/tag/v{version}"}}"#
        )
    }

    fn cache_knowing(h: &Harness, latest: &str) -> String {
        codev_core::update::UpdateCache {
            checked_at: h.clock.now_unix(),
            latest: Some(latest.into()),
            url: Some(codev_core::update::release_page(latest)),
            ..Default::default()
        }
        .render()
    }

    /// An initialized project with one change, `add-auth`.
    fn project_with_change(h: &Harness) {
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        new_change(&h.ctx(), "add-auth", None, None).unwrap();
    }

    fn status_json(outcome: &StatusOutcome) -> serde_json::Value {
        serde_json::to_value(
            crate::contract::StatusV1::new(
                &outcome.status,
                &outcome.planning_home,
                &outcome.change_root,
                &outcome.warnings,
            )
            .with_update(outcome.update.as_ref(), outcome.requirement.as_ref()),
        )
        .unwrap()
    }

    #[test]
    fn status_json_carries_update_and_version_requirement() {
        let h = Harness::new()
            .with("/p/_codev/config.yaml", "codev_version: \">=0.2\"\n")
            .with_http(
                MemoryHttp::new()
                    .with_response(codev_core::update::RELEASES_API, release_json("0.6.0")),
            );
        project_with_change(&h);
        let json = status_json(&status(&h.ctx(), Some("add-auth")).unwrap());
        assert_eq!(
            json["update"],
            serde_json::json!({
                "current": "0.1.0",
                "latest": "0.6.0",
                "url": "https://github.com/mairistem/codev/releases/tag/v0.6.0",
                "available": true,
                "snoozed": false,
            })
        );
        assert_eq!(
            json["versionRequirement"],
            serde_json::json!({ "requirement": ">=0.2", "satisfied": false })
        );
        assert_eq!(h.http.calls().len(), 1);
    }

    #[test]
    fn status_json_omits_update_when_unknown() {
        // Offline and no cache: no `update`, no `versionRequirement`, and
        // the rest of the document is intact.
        let h = Harness::new();
        project_with_change(&h);
        let json = status_json(&status(&h.ctx(), Some("add-auth")).unwrap());
        assert!(json.get("update").is_none(), "{json}");
        assert!(json.get("versionRequirement").is_none(), "{json}");
        assert_eq!(json["changeName"], "add-auth");
        // In CI, not even a request.
        let mut ci = Harness::new().with_http(
            MemoryHttp::new()
                .with_response(codev_core::update::RELEASES_API, release_json("0.6.0")),
        );
        ci.env.vars.insert("CI".into(), "true".into());
        project_with_change(&ci);
        let json = status_json(&status(&ci.ctx(), Some("add-auth")).unwrap());
        assert!(json.get("update").is_none(), "{json}");
        assert!(ci.http.calls().is_empty());
    }

    #[test]
    fn status_human_prints_the_update_line_unless_snoozed() {
        let h = Harness::new();
        let cache = cache_knowing(&h, "0.6.0");
        let h = h.with(UPDATE_CACHE, &cache);
        project_with_change(&h);
        let notice = status(&h.ctx(), Some("add-auth")).unwrap().update.unwrap();
        assert!(notice.should_announce());
        assert_eq!(
            notice.line(),
            "codev 0.6.0 is available (you have 0.1.0) — run `codev self-update`"
        );

        // "Later": snoozed, the JSON says so and the line is gone.
        let outcome = self_update(
            &h.ctx(),
            &SelfUpdateArgs {
                snooze: true,
                ..SelfUpdateArgs::default()
            },
            &mut |_, _| true,
        )
        .unwrap();
        assert_eq!(outcome.action.as_str(), "snoozed");
        let outcome = status(&h.ctx(), Some("add-auth")).unwrap();
        let notice = outcome.update.as_ref().unwrap();
        assert!(notice.snoozed && !notice.should_announce());
        assert_eq!(status_json(&outcome)["update"]["snoozed"], true);
        assert!(h.http.calls().is_empty(), "the cache was recent");
    }

    #[test]
    fn status_json_never_carries_the_doctor_hint() {
        let h = Harness::new();
        project_with_change(&h);
        assert!(doctor_missing_from_cwd(&h.ctx()).is_some_and(|n| n > 0));
        let json = status_json(&status(&h.ctx(), Some("add-auth")).unwrap());
        assert!(!json.to_string().contains("codev doctor"), "{json}");
    }

    #[test]
    fn other_commands_never_request_the_release_api() {
        let h = Harness::new().with_http(
            MemoryHttp::new()
                .with_response(codev_core::update::RELEASES_API, release_json("0.6.0")),
        );
        project_with_change(&h);
        validate(&h.ctx(), ValidateArgs::All).unwrap();
        list_changes(&h.ctx()).unwrap();
        list_specs(&h.ctx()).unwrap();
        doctor(&h.ctx()).unwrap();
        artifact_instructions(&h.ctx(), None, Some("add-auth")).unwrap();
        update(&h.ctx(), false).unwrap();
        assert!(h.http.calls().is_empty(), "{:?}", h.http.calls());
    }

    #[test]
    fn an_invalid_codev_version_fails_status() {
        let h = Harness::new();
        project_with_change(&h);
        h.fs.write(
            Path::new("/p/_codev/config.yaml"),
            "codev_version: latest\n",
        )
        .unwrap();
        let err = status(&h.ctx(), Some("add-auth")).unwrap_err();
        assert_eq!(err.code, "invalid_codev_version");
    }

    #[test]
    fn doctor_reports_an_unsatisfied_codev_version() {
        let h = Harness::new().with("/p/_codev/config.yaml", "codev_version: \">=0.7\"\n");
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        let outcome = doctor(&h.ctx()).unwrap();
        let version = check(&outcome, "codev_version");
        assert_eq!(version.status, CheckStatus::Missing);
        assert_eq!(version.code, "codev_version_unsatisfied");
        assert_eq!(version.fix.as_deref(), Some("codev self-update"));
    }

    #[test]
    fn doctor_warns_about_a_cached_newer_release_without_a_request() {
        let h = Harness::new();
        let cache = cache_knowing(&h, "0.6.0");
        let h = h.with(UPDATE_CACHE, &cache).with_http(
            MemoryHttp::new()
                .with_response(codev_core::update::RELEASES_API, release_json("0.9.0")),
        );
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        let outcome = doctor(&h.ctx()).unwrap();
        let version = check(&outcome, "codev_version");
        assert_eq!(version.status, CheckStatus::Warning);
        assert_eq!(version.code, "update_available");
        assert!(version.message.contains("0.6.0"), "{}", version.message);
        assert!(h.http.calls().is_empty());
    }

    // ─────────────── forward-compatible configuration ───────────────

    fn with_config(h: &Harness, yaml: &str) {
        h.fs.write(Path::new("/p/_codev/config.yaml"), yaml)
            .unwrap();
    }

    #[test]
    fn status_warns_on_an_unknown_config_key_and_quotes_the_requirement() {
        let h = Harness::new();
        project_with_change(&h);
        with_config(&h, "codev_version: \">=0.9\"\nreview_policy: strict\n");
        let outcome = status(&h.ctx(), Some("add-auth")).unwrap();
        assert_eq!(outcome.status.change.as_str(), "add-auth");
        let json = status_json(&outcome);
        let entry = json["status"]
            .as_array()
            .unwrap()
            .iter()
            .find(|e| e["code"] == "unknown_config_key")
            .unwrap_or_else(|| panic!("{json}"));
        let message = entry["message"].as_str().unwrap();
        assert!(message.contains("`review_policy`"), "{message}");
        assert!(message.contains("\">=0.9\""), "{message}");
        assert!(message.contains("codev self-update"), "{message}");
    }

    #[test]
    fn status_warns_on_a_misspelled_top_level_key() {
        let h = Harness::new();
        project_with_change(&h);
        with_config(&h, "shema: spec-driven\n");
        let outcome = status(&h.ctx(), Some("add-auth")).unwrap();
        assert_eq!(outcome.status.schema_name, "spec-driven");
        let warning = outcome
            .warnings
            .iter()
            .find(|w| w.code == "unknown_config_key")
            .unwrap();
        assert!(warning.message.contains("`shema`"), "{}", warning.message);
        assert!(
            warning.message.contains("newer codev"),
            "{}",
            warning.message
        );
    }

    #[test]
    fn update_installs_skills_despite_an_unknown_mcp_key() {
        let h = Harness::new().with(
            "/p/_codev/config.yaml",
            "mcp:\n  jira_tool: mcp__a__getJiraIssue\n  design_tool: mcp__b__get\n",
        );
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        let outcome = update(&h.ctx(), false).unwrap();
        assert!(
            outcome
                .warnings
                .iter()
                .any(|w| w.code == "unknown_config_key" && w.message.contains("`mcp.design_tool`")),
            "{:?}",
            outcome.warnings
        );
        let skill =
            h.fs.read_to_string(Path::new("/p/.claude/skills/codev-propose/SKILL.md"))
                .unwrap();
        assert!(skill.contains("mcp__a__getJiraIssue"));
    }

    #[test]
    fn status_fails_on_an_unknown_changelog_key_quoting_the_requirement() {
        let h = Harness::new();
        project_with_change(&h);
        with_config(
            &h,
            "codev_version: \">=0.9\"\nchangelog:\n  since: v1.0.0\n",
        );
        let err = status(&h.ctx(), Some("add-auth")).unwrap_err();
        assert_eq!(err.code, "invalid");
        assert!(err.message.contains("since"), "{}", err.message);
        assert!(err.message.contains("\">=0.9\""), "{}", err.message);
        assert!(err.message.contains("codev self-update"), "{}", err.message);
    }

    #[test]
    fn validate_reports_an_unknown_config_key_as_a_warning() {
        let h = Harness::new();
        project_with_change(&h);
        with_config(
            &h,
            "schema: spec-driven\nlanguage: en\nreview_policy: strict\n",
        );
        let report = validate(&h.ctx(), ValidateArgs::All).unwrap();
        let item = report
            .items
            .iter()
            .find(|i| i.kind == codev_engine::validate::ItemKind::Config)
            .expect("a config item");
        assert_eq!(item.findings.len(), 1);
        assert_eq!(item.findings[0].finding.code, "unknown_config_key");
        assert_eq!(item.findings[0].finding.line, 3);
        // `--strict` fails on warnings, plain `validate` on errors only.
        assert!(item.has_warnings() && !item.has_errors(), "{item:?}");
        let json = serde_json::to_value(crate::contract::ValidateReportV1::from(&report)).unwrap();
        assert!(json.to_string().contains("\"kind\":\"config\""), "{json}");
        // Scoped runs leave the configuration out.
        let changes = validate(&h.ctx(), ValidateArgs::Changes).unwrap();
        assert!(
            changes
                .items
                .iter()
                .all(|i| i.kind != codev_engine::validate::ItemKind::Config)
        );
    }

    #[test]
    fn validate_has_no_config_item_when_clean() {
        let h = Harness::new();
        project_with_change(&h);
        let report = validate(&h.ctx(), ValidateArgs::All).unwrap();
        assert!(
            report
                .items
                .iter()
                .all(|i| i.kind != codev_engine::validate::ItemKind::Config)
        );
    }

    #[test]
    fn doctor_reports_an_unknown_config_key() {
        let h = Harness::new().with(
            "/p/_codev/config.yaml",
            "language: en\nreview_policy: strict\n",
        );
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        let outcome = doctor(&h.ctx()).unwrap();
        let keys = check(&outcome, "config.keys");
        assert_eq!(keys.status, CheckStatus::Warning);
        assert_eq!(keys.code, "unknown_config_key");
        assert!(keys.message.contains("`review_policy`"), "{}", keys.message);
        assert!(
            keys.fix
                .as_deref()
                .unwrap()
                .starts_with("correct or remove"),
            "{:?}",
            keys.fix
        );
    }

    #[test]
    fn doctor_fixes_a_codev_above_the_requirement_with_an_explicit_version() {
        // The harness runs codev 0.1.0.
        let h = Harness::new();
        let cache = cache_knowing(&h, "0.0.8");
        let h = h.with(UPDATE_CACHE, &cache).with(
            "/p/_codev/config.yaml",
            "codev_version: \">=0.0.5, <0.1\"\n",
        );
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        let version = check(&doctor(&h.ctx()).unwrap(), "codev_version").clone();
        assert_eq!(version.code, "codev_version_unsatisfied");
        assert_eq!(
            version.fix.as_deref(),
            Some("codev self-update --version 0.0.8")
        );
    }

    #[test]
    fn self_update_json_reports_the_action() {
        let asset = codev_core::update::asset_name("0.6.0", "aarch64-apple-darwin").unwrap();
        let archive = b"archive".to_vec();
        let h = Harness::new()
            .with_http(
                MemoryHttp::new()
                    .with_response(codev_core::update::RELEASES_API, release_json("0.6.0"))
                    .with_response(
                        &codev_core::update::download_url("0.6.0", "SHA256SUMS"),
                        format!("{}  {asset}\n", codev_core::update::sha256_hex(&archive)),
                    )
                    .with_response(
                        &codev_core::update::download_url("0.6.0", &asset),
                        archive.clone(),
                    ),
            )
            .with_exe(
                MemoryExecutable::at("/home/.local/bin/codev", "codev 0.1.0").with_file(
                    format!(
                        "{}/codev-0.6.0-aarch64-apple-darwin/codev",
                        codev_engine::ports::MEMORY_TEMP_DIR
                    ),
                    "codev 0.6.0",
                ),
            )
            .with_runner(
                MockProcessRunner::new()
                    .with_response("tar", &[], ok_process())
                    .with_response("/home/.local/bin/codev", &["update"], ok_process()),
            );
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        let outcome = self_update(&h.ctx(), &SelfUpdateArgs::default(), &mut |_, _| true).unwrap();
        let json = serde_json::to_value(crate::contract::SelfUpdateV1::from(&outcome)).unwrap();
        assert_eq!(json["action"], "updated");
        assert_eq!(json["current"], "0.1.0");
        assert_eq!(json["latest"], "0.6.0");
        assert_eq!(json["installMethod"], "standalone");
        assert_eq!(json["projectUpdated"], true);
        assert_eq!(json["hint"], serde_json::Value::Null);
        assert_eq!(json["status"], serde_json::json!([]));
        assert_eq!(
            h.exe.read("/home/.local/bin/codev").as_deref(),
            Some(&b"codev 0.6.0"[..])
        );
    }

    fn ok_process() -> codev_engine::ProcessOutput {
        codev_engine::ProcessOutput {
            stdout: Vec::new(),
            stderr: Vec::new(),
            exit_code: 0,
        }
    }

    #[test]
    fn self_update_json_failure_keeps_the_shape() {
        let h = Harness::new();
        let err = self_update(
            &h.ctx(),
            &SelfUpdateArgs {
                check: true,
                ..SelfUpdateArgs::default()
            },
            &mut |_, _| true,
        )
        .unwrap_err();
        assert_eq!(err.code, "update_check_failed");
        let json = crate::contract::failure(
            crate::contract::self_update_shape("0.1.0"),
            &err.code,
            &err.message,
        );
        assert_eq!(json["latest"], serde_json::Value::Null);
        assert_eq!(json["status"][0]["code"], "update_check_failed");
    }

    #[test]
    fn self_update_snooze_records_and_silences_the_notice() {
        let h = Harness::new();
        let cache = cache_knowing(&h, "0.6.0");
        let h = h.with(UPDATE_CACHE, &cache);
        let outcome = self_update(
            &h.ctx(),
            &SelfUpdateArgs {
                snooze: true,
                ..SelfUpdateArgs::default()
            },
            &mut |_, _| true,
        )
        .unwrap();
        assert_eq!(outcome.action.as_str(), "snoozed");
        assert!(!update_notice(&h.ctx()).unwrap().should_announce());

        // Up to date: nothing to snooze, still a success.
        let h = Harness::new();
        let cache = cache_knowing(&h, "0.1.0");
        let h = h.with(UPDATE_CACHE, &cache);
        let outcome = self_update(
            &h.ctx(),
            &SelfUpdateArgs {
                snooze: true,
                ..SelfUpdateArgs::default()
            },
            &mut |_, _| true,
        )
        .unwrap();
        assert_eq!(outcome.action.as_str(), "nothing_to_snooze");
    }

    #[test]
    fn windows_cleanup_removes_the_previous_binary() {
        let exe = MemoryExecutable::at("C:/Users/me/AppData/Local/Programs/codev/codev.exe", "new")
            .with_file(
                "C:/Users/me/AppData/Local/Programs/codev/codev.old.exe",
                "old",
            );
        codev_engine::update::cleanup_previous_binary(&exe);
        assert!(
            exe.read("C:/Users/me/AppData/Local/Programs/codev/codev.old.exe")
                .is_none()
        );
    }

    // ─────────────── changes: new change, list, status ───────────────

    #[test]
    fn new_change_writes_schema_date_and_goal_only() {
        let h = Harness::new();
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        let before = h.fs.paths();
        let outcome = new_change(
            &h.ctx(),
            "add-auth",
            None,
            Some("Add authentication".into()),
        )
        .unwrap();

        let metadata = PathBuf::from("/p/_codev/changes/add-auth/change.yaml");
        assert_eq!(
            h.fs.read(&metadata).unwrap(),
            "schema: spec-driven\ncreated: 2026-09-08\ngoal: Add authentication\n"
        );
        let added: Vec<PathBuf> =
            h.fs.paths()
                .into_iter()
                .filter(|p| !before.contains(p))
                .collect();
        assert_eq!(
            added,
            std::slice::from_ref(&metadata),
            "change.yaml is the only file"
        );
        assert_eq!(outcome.created, [metadata]);
        assert_eq!(outcome.schema_name, "spec-driven");
        assert_eq!(
            outcome.change_root,
            PathBuf::from("/p/_codev/changes/add-auth")
        );

        let rendered = crate::render::new_change(&outcome);
        assert!(rendered.contains("Change `add-auth` created"), "{rendered}");
        assert!(rendered.contains("Schema    spec-driven"), "{rendered}");
        assert!(
            rendered.contains("codev instructions --change add-auth"),
            "{rendered}"
        );
    }

    #[test]
    fn new_change_ignores_a_blank_goal() {
        let h = Harness::new();
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        new_change(&h.ctx(), "add-auth", None, Some("  ".into())).unwrap();
        let metadata = h.fs.read("/p/_codev/changes/add-auth/change.yaml").unwrap();
        assert!(!metadata.contains("goal"), "{metadata}");
    }

    #[test]
    fn status_of_a_fresh_change_has_only_the_proposal_ready() {
        let h = Harness::new();
        project_with_change(&h);
        let json = status_json(&status(&h.ctx(), Some("add-auth")).unwrap());

        let artifact = |id: &str| {
            json["artifacts"]
                .as_array()
                .unwrap()
                .iter()
                .find(|a| a["id"] == id)
                .unwrap_or_else(|| panic!("artifact `{id}` missing"))
                .clone()
        };
        assert_eq!(artifact("proposal")["status"], "ready");
        for id in ["specs", "design"] {
            assert_eq!(artifact(id)["status"], "blocked");
            assert_eq!(artifact(id)["missingDeps"], serde_json::json!(["proposal"]));
        }
        assert_eq!(artifact("tasks")["status"], "blocked");
        assert_eq!(
            artifact("tasks")["missingDeps"],
            serde_json::json!(["specs", "design"])
        );
        assert_eq!(json["applyRequires"], serde_json::json!(["tasks"]));
        assert_eq!(json["isPlanningComplete"], false);
    }

    #[test]
    fn status_after_the_proposal_shows_specs_and_design_ready() {
        let h = Harness::new();
        project_with_change(&h);
        write_helper(&h, "/p/_codev/changes/add-auth/proposal.md", "# Proposal\n");

        let rendered = crate::render::status(&status(&h.ctx(), Some("add-auth")).unwrap().status);
        for line in [
            "Change: add-auth",
            "Schema: spec-driven",
            "[x] proposal",
            "[ ] specs",
            "[ ] design",
            "[-] tasks (waiting on: specs, design)",
            "Planning: 1/4 artifacts",
            "Next:     specs",
        ] {
            assert!(rendered.contains(line), "missing `{line}` in:\n{rendered}");
        }
    }

    #[test]
    fn status_resolves_the_project_root_from_a_subfolder() {
        let mut h = Harness::new();
        project_with_change(&h);
        h.env.cwd = PathBuf::from("/p/src");

        let outcome = status(&h.ctx(), None).unwrap();
        assert_eq!(outcome.planning_home, PathBuf::from("/p"));
        assert_eq!(
            outcome.change_root,
            PathBuf::from("/p/_codev/changes/add-auth")
        );
    }

    #[test]
    fn instructions_refuse_an_invalid_change_name() {
        let h = Harness::new();
        project_with_change(&h);
        let err = artifact_instructions(&h.ctx(), None, Some("Bad")).unwrap_err();
        assert_eq!(err.code, "invalid_change_id");
    }

    // ─────────────── instructions ───────────────

    #[test]
    fn instructions_for_the_next_artifact_keep_the_glob_and_write_nothing() {
        let h = Harness::new();
        project_with_change(&h);
        write_helper(&h, "/p/_codev/changes/add-auth/proposal.md", "# Proposal\n");
        let before = h.fs.paths();

        let instr = artifact_instructions(&h.ctx(), None, Some("add-auth")).unwrap();
        assert_eq!(instr.artifact_id, "specs");
        assert_eq!(
            instr.resolved_output_path,
            PathBuf::from("/p/_codev/changes/add-auth/specs/**/*.md")
        );
        assert_eq!(h.fs.paths(), before, "instructions write nothing");
    }

    #[test]
    fn human_instructions_list_the_sections_in_order() {
        let h = Harness::new();
        project_with_change(&h);
        write_helper(
            &h,
            "/p/_codev/config.yaml",
            "context: |\n  Rust project.\nrules:\n  specs:\n    - Observable behavior only.\n",
        );
        write_helper(&h, "/p/_codev/changes/add-auth/proposal.md", "# Proposal\n");

        let instr = artifact_instructions(&h.ctx(), Some("specs"), Some("add-auth")).unwrap();
        let rendered = crate::render::instructions(&instr);
        assert!(
            rendered.starts_with("Artifact: specs (change `add-auth`, schema spec-driven)"),
            "{rendered}"
        );
        let mut last = 0;
        for section in [
            "Write to: ",
            "Language: en",
            "Read before writing:",
            "[x] proposal",
            "Project context:",
            "(project) Rust project.",
            "Rules for this artifact:",
            "(project) Observable behavior only.",
            "Instruction:",
            "Template:",
            "Unlocks: tasks",
        ] {
            let at = rendered[last..]
                .find(section)
                .unwrap_or_else(|| panic!("`{section}` missing or out of order in:\n{rendered}"));
            last += at;
        }
    }

    #[test]
    fn instructions_of_an_unknown_artifact_name_it_and_the_schema() {
        let h = Harness::new();
        project_with_change(&h);
        let err = artifact_instructions(&h.ctx(), Some("sketch"), Some("add-auth")).unwrap_err();
        assert_eq!(err.code, "unknown_artifact");
        assert!(
            err.message.contains("sketch") && err.message.contains("spec-driven"),
            "{}",
            err.message
        );
    }

    // ─────────────── schemas ───────────────

    const LITE_SCHEMA: &str = "name: lite\nartifacts:\n  - id: brief\n    generates: brief.md\n  - id: plan\n    generates: tasks.md\n    requires: [brief]\napply:\n  requires: [plan]\n  tracks: tasks.md\n";

    #[test]
    fn schemas_list_a_project_schema_with_its_flow_before_the_built_in_one() {
        let h = Harness::new().with("/p/_codev/schemas/lite/schema.yaml", LITE_SCHEMA);
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        let outcome = list_schemas(&h.ctx()).unwrap();

        let listed: Vec<(&str, &str, Vec<&str>)> = outcome
            .schemas
            .iter()
            .map(|s| {
                (
                    s.name.as_str(),
                    s.origin,
                    s.flow.iter().map(String::as_str).collect(),
                )
            })
            .collect();
        assert_eq!(
            listed,
            [
                ("lite", "project", vec!["brief", "plan"]),
                (
                    "spec-driven",
                    "built-in",
                    vec!["proposal", "specs", "design", "tasks"]
                ),
            ]
        );
        let rendered = crate::render::schemas(&outcome);
        assert!(
            rendered.starts_with(
                "Available schemas:\n  lite (project)\n    brief → plan\n  spec-driven (built-in)\n"
            ),
            "{rendered}"
        );
    }

    #[test]
    fn status_uses_the_project_spec_driven_schema() {
        let h = Harness::new().with(
            "/p/_codev/schemas/spec-driven/schema.yaml",
            "name: spec-driven\nartifacts:\n  - id: proposal\n    generates: proposal.md\napply:\n  requires: [proposal]\n  tracks: proposal.md\n",
        );
        project_with_change(&h);
        let outcome = status(&h.ctx(), Some("add-auth")).unwrap();
        let ids: Vec<&str> = outcome
            .status
            .artifacts
            .iter()
            .map(|a| a.id.as_str())
            .collect();
        assert_eq!(ids, ["proposal"]);
    }

    #[test]
    fn new_change_takes_the_schema_from_the_flag_then_the_configuration() {
        let h = Harness::new().with("/p/_codev/schemas/lite/schema.yaml", LITE_SCHEMA);
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        write_helper(&h, "/p/_codev/config.yaml", "schema: lite\n");

        let quick = new_change(&h.ctx(), "quick", None, None).unwrap();
        assert_eq!(quick.schema_name, "lite");
        assert!(
            h.fs.read("/p/_codev/changes/quick/change.yaml")
                .is_some_and(|c| c.contains("schema: lite"))
        );

        let full = new_change(&h.ctx(), "full", Some("spec-driven"), None).unwrap();
        assert_eq!(full.schema_name, "spec-driven");
        assert!(
            h.fs.read("/p/_codev/changes/full/change.yaml")
                .is_some_and(|c| c.contains("schema: spec-driven"))
        );
    }

    /// A valid schema whose `name:` differs from the folder it lives in.
    const MISNAMED_SCHEMA: &str = "name: quick-flow\nartifacts:\n  - id: brief\n    generates: brief.md\napply:\n  requires: [brief]\n  tracks: brief.md\n";

    #[test]
    fn new_change_records_the_folder_of_a_schema_named_differently() {
        let h = Harness::new().with("/p/_codev/schemas/lite/schema.yaml", MISNAMED_SCHEMA);
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();

        new_change(&h.ctx(), "quick", Some("lite"), None).unwrap();
        let metadata = h.fs.read("/p/_codev/changes/quick/change.yaml").unwrap();
        assert!(metadata.contains("schema: lite"), "{metadata}");

        // The change must be usable afterwards: the recorded name resolves.
        let outcome = status(&h.ctx(), Some("quick")).unwrap();
        let ids: Vec<&str> = outcome
            .status
            .artifacts
            .iter()
            .map(|a| a.id.as_str())
            .collect();
        assert_eq!(ids, ["brief"]);
    }

    #[test]
    fn status_and_instructions_name_the_schema_by_its_folder() {
        let h = Harness::new()
            .with("/p/_codev/schemas/lite/schema.yaml", MISNAMED_SCHEMA)
            .with("/p/_codev/changes/quick/change.yaml", "schema: lite\n");
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();

        let outcome = status(&h.ctx(), Some("quick")).unwrap();
        assert_eq!(outcome.status.schema_name, "lite");
        let instructions = artifact_instructions(&h.ctx(), None, Some("quick")).unwrap();
        assert_eq!(instructions.schema_name, "lite");
        let err = artifact_instructions(&h.ctx(), Some("sketch"), Some("quick")).unwrap_err();
        assert!(err.message.contains("`lite`"), "{}", err.message);
    }

    #[test]
    fn schemas_warn_when_the_name_differs_from_the_folder() {
        let h = Harness::new().with("/p/_codev/schemas/lite/schema.yaml", MISNAMED_SCHEMA);
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        let outcome = list_schemas(&h.ctx()).unwrap();

        let names: Vec<(&str, &str)> = outcome
            .schemas
            .iter()
            .map(|s| (s.name.as_str(), s.origin))
            .collect();
        assert_eq!(names, [("lite", "project"), ("spec-driven", "built-in")]);
        assert_eq!(outcome.warnings.len(), 1, "{:?}", outcome.warnings);
        let warning = &outcome.warnings[0];
        assert_eq!(warning.code, "schema_name_mismatch");
        assert!(
            warning.message.contains("`lite`") && warning.message.contains("`quick-flow`"),
            "{}",
            warning.message
        );
    }

    #[test]
    fn schemas_with_matching_names_raise_no_warning() {
        let h = Harness::new().with("/p/_codev/schemas/lite/schema.yaml", LITE_SCHEMA);
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        let outcome = list_schemas(&h.ctx()).unwrap();
        assert!(outcome.warnings.is_empty(), "{:?}", outcome.warnings);
    }

    #[test]
    fn changing_the_configured_schema_leaves_existing_changes_alone() {
        let h = Harness::new().with("/p/_codev/schemas/lite/schema.yaml", LITE_SCHEMA);
        project_with_change(&h);
        write_helper(&h, "/p/_codev/config.yaml", "schema: lite\n");

        let outcome = status(&h.ctx(), Some("add-auth")).unwrap();
        assert_eq!(outcome.status.schema_name, "spec-driven");
        let ids: Vec<&str> = outcome
            .status
            .artifacts
            .iter()
            .map(|a| a.id.as_str())
            .collect();
        assert_eq!(ids, ["proposal", "specs", "design", "tasks"]);
    }

    // ─────────────── decision show ───────────────

    #[test]
    fn decision_show_prints_the_header_then_the_file() {
        let adr = "---\nid: \"0001\"\ntitle: Foundation\nstatus: accepted\ndate: 2026-09-08\ntags: [arch]\n---\n\n## Context\n\nWhy.\n";
        let h = Harness::new().with("/p/_codev/decisions/0001-foundation.md", adr);
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();

        let outcome = decision_show(&h.ctx(), "0001").unwrap();
        assert_eq!(
            outcome.decision.path,
            PathBuf::from("_codev/decisions/0001-foundation.md")
        );
        assert_eq!(outcome.content, adr);
        let rendered = crate::render::decision_show(&outcome);
        assert!(
            rendered.starts_with(
                "ID:     0001 (project/0001)\nTitle:  Foundation\nStatus: accepted\nDate:   2026-09-08\nTags:   arch\n"
            ),
            "{rendered}"
        );
        let file_line = format!("File:   {}\n\n", outcome.decision.path.display());
        assert!(rendered.contains(&file_line), "{rendered}");
        assert!(rendered.ends_with(adr), "the file follows unchanged");
    }

    fn colliding_decisions() -> Harness {
        let h = Harness::new()
            .with("/p/_codev/config.yaml", "inherits:\n  - path: ~/shared\n")
            .with(
                "/p/_codev/decisions/0007-local.md",
                "---\nid: \"0007\"\ntitle: Local choice\nstatus: accepted\ndate: 2026-09-08\n---\n\nx\n",
            )
            .with(
                "/home/shared/_codev/decisions/0007-shared.md",
                &inherited_adr("0007"),
            );
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        h
    }

    #[test]
    fn decision_show_refuses_an_ambiguous_id_and_lists_the_candidates() {
        let h = colliding_decisions();
        let err = decision_show(&h.ctx(), "0007")
            .err()
            .expect("an ambiguous id is refused");
        assert_eq!(err.code, "ambiguous_decision_id");
        assert!(
            err.message.contains("project/0007") && err.message.contains("path:~/shared/0007"),
            "{}",
            err.message
        );
    }

    #[test]
    fn decision_show_accepts_a_qualified_id() {
        let h = colliding_decisions();
        let outcome = decision_show(&h.ctx(), "path:~/shared/0007").unwrap();
        assert_eq!(outcome.decision.origin, "path:~/shared");
        assert_eq!(outcome.decision.title, "Source choice");
        let local = decision_show(&h.ctx(), "project/0007").unwrap();
        assert_eq!(local.decision.title, "Local choice");
    }

    #[test]
    fn decision_show_of_an_inherited_decision_gives_its_origin_and_absolute_path() {
        let h = Harness::new()
            .with("/p/_codev/config.yaml", "inherits:\n  - path: /shared\n")
            .with(
                "/shared/_codev/decisions/0100-shared.md",
                &inherited_adr("0100"),
            );
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();

        let outcome = decision_show(&h.ctx(), "0100").unwrap();
        assert_eq!(outcome.decision.origin, "path:/shared");
        assert_eq!(outcome.decision.qualified_id, "path:/shared/0100");
        assert_eq!(
            outcome.decision.path,
            PathBuf::from("/shared/_codev/decisions/0100-shared.md")
        );
        assert_eq!(outcome.content, inherited_adr("0100"));
    }

    #[test]
    fn decision_show_refuses_an_unknown_id() {
        let h = Harness::new();
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        let err = decision_show(&h.ctx(), "9999")
            .err()
            .expect("an unknown id is refused");
        assert_eq!(err.code, "unknown_decision_id");
        assert!(
            err.message.contains("codev decision list"),
            "{}",
            err.message
        );
    }

    // ─────────────── update: skills ───────────────

    fn skill_path(workflow: &str) -> PathBuf {
        PathBuf::from(format!("/p/.claude/skills/codev-{workflow}/SKILL.md"))
    }

    #[test]
    fn update_creates_missing_and_rewrites_outdated_skills() {
        let h = Harness::new();
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        h.fs.remove_file(&skill_path("explore")).unwrap();
        let sync = h.fs.read(skill_path("sync")).unwrap();
        write_helper(
            &h,
            "/p/.claude/skills/codev-sync/SKILL.md",
            &sync.replace("version: \"0.1.0\"", "version: \"0.0.1\""),
        );

        let outcome = update(&h.ctx(), false).unwrap();
        assert!(
            outcome.created.contains(&skill_path("explore")),
            "{:?}",
            outcome.created
        );
        assert!(
            outcome.updated.contains(&skill_path("sync")),
            "{:?}",
            outcome.updated
        );
        for workflow in ["explore", "sync"] {
            assert!(
                h.fs.read(skill_path(workflow))
                    .is_some_and(|c| c.contains("version: \"0.1.0\"")),
                "`{workflow}` carries the running version"
            );
        }
    }

    #[test]
    fn a_second_update_has_nothing_to_do() {
        let h = Harness::new();
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        update(&h.ctx(), false).unwrap();

        let again = update(&h.ctx(), false).unwrap();
        assert!(again.created.is_empty() && again.updated.is_empty());
        let rendered = crate::render::setup(&again, false);
        assert!(rendered.starts_with("codev updated in "), "{rendered}");
        assert!(
            rendered.contains("Nothing to do, everything is already in place."),
            "{rendered}"
        );
    }

    #[test]
    fn update_force_rewrites_a_hand_edited_skill() {
        let h = Harness::new();
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        let rendered = h.fs.read(skill_path("apply")).unwrap();
        write_helper(
            &h,
            "/p/.claude/skills/codev-apply/SKILL.md",
            &format!("{rendered}\nMy own rule.\n"),
        );

        let kept = update(&h.ctx(), false).unwrap();
        assert_eq!(kept.preserved, [skill_path("apply")]);

        let forced = update(&h.ctx(), true).unwrap();
        assert!(
            forced.updated.contains(&skill_path("apply")),
            "{:?}",
            forced.updated
        );
        assert!(forced.preserved.is_empty());
        assert_eq!(h.fs.read(skill_path("apply")).unwrap(), rendered);
    }

    #[test]
    fn update_ignores_an_unknown_workflow_with_a_warning() {
        let h = Harness::new();
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        write_helper(&h, "/p/_codev/config.yaml", "workflows: [propose, bogus]\n");

        let outcome = update(&h.ctx(), false).unwrap();
        assert_eq!(outcome.skills, ["codev-propose"]);
        assert!(
            outcome
                .warnings
                .iter()
                .any(|w| w.code == "unknown_workflow" && w.message.contains("`bogus`")),
            "{:?}",
            outcome.warnings
        );
    }

    #[test]
    fn update_keeps_the_skill_of_a_workflow_no_longer_listed() {
        let h = Harness::new();
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        let apply_skill = h.fs.read(skill_path("apply"));
        assert!(apply_skill.is_some(), "the default catalog installs apply");
        write_helper(&h, "/p/_codev/config.yaml", "workflows: [propose]\n");

        update(&h.ctx(), false).unwrap();
        assert_eq!(h.fs.read(skill_path("apply")), apply_skill);
    }

    #[test]
    fn update_recreates_a_deleted_codev_folder() {
        let h = Harness::new().with("/p/_codev/config.yaml", "schema: spec-driven\n");
        let outcome = update(&h.ctx(), false).unwrap();
        assert!(
            outcome
                .created
                .contains(&PathBuf::from("/p/_codev/specs/.gitkeep")),
            "{:?}",
            outcome.created
        );
        assert!(
            outcome
                .untouched
                .contains(&PathBuf::from("/p/_codev/config.yaml"))
        );
        assert_eq!(
            h.fs.read("/p/_codev/config.yaml").unwrap(),
            "schema: spec-driven\n"
        );
    }
}
