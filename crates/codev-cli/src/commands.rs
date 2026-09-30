use std::fmt;
use std::path::{Path, PathBuf};

use codev_agents::claude::ClaudeCode;
use codev_agents::target::AgentTarget;
use codev_agents::workflows;
use codev_core::{ChangeId, ChangeStatus, CoreError, Layout, Plan, WriteMode};
use codev_engine::apply::{self, Applied};
use codev_engine::archive as engine_archive;
use codev_engine::archive::ArchiveOutcome;
use codev_engine::config;
use codev_engine::decisions as engine_decisions;
use codev_engine::decisions_actions as engine_actions;
use codev_engine::instructions::Instructions;
use codev_engine::metadata::ChangeMetadata;
use codev_engine::sources as engine_sources;
use codev_engine::sync as engine_sync;
use codev_engine::sync::SyncOutcome;
use codev_engine::validate as engine_validate;
use codev_engine::validate::ValidateReport;
use codev_engine::{
    Clock, EngineError, Env, FileSystem, ProcessRunner, RealProcessRunner, Warning,
};
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
    pub version: &'a str,
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

    // Writes the generated config.yaml ONLY if missing — an existing file is
    // never touched; the user's configuration stays in charge. It goes
    // through a plan like every other write, so that it is counted among
    // the created files.
    let mut config_written = Applied::default();
    if !ctx.fs.exists(&config_path) {
        let detected = if opts.no_detect {
            codev_core::detect::Detected::empty()
        } else {
            codev_engine::detect::run(ctx.fs, ctx.env, layout.project_root())
        };
        let choices = crate::init_prompts::run(&detected, opts)
            .map_err(|e| Failure::new("prompt_failed", e.to_string()))?;
        let generated = codev_core::config::from_detected(&detected, &choices);
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
    let mut outcome = install_skills(ctx, &layout, force)?;
    outcome.absorb(config_written);
    outcome.absorb(scaffolded);
    // The scaffold reports the config written above as untouched: it is
    // listed once, as created.
    let created = outcome.created.clone();
    outcome.untouched.retain(|p| !created.contains(p));
    Ok(outcome)
}

/// Regenerates the skills of an already initialized project.
///
/// Also runs the scaffolding plan: in "create only what is missing" mode, it
/// restores a deleted folder without touching anything else.
pub fn update(ctx: &Ctx, force: bool) -> Result<SetupOutcome> {
    let layout = root::discover_from_cwd(ctx.fs, ctx.env)?;
    let scaffolded = apply::execute(&scaffold::plan_init(&layout), ctx.fs)?;
    let mut outcome = install_skills(ctx, &layout, force)?;
    outcome.absorb(scaffolded);
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
}

pub fn status(ctx: &Ctx, requested: Option<&str>) -> Result<StatusOutcome> {
    let layout = root::discover_from_cwd(ctx.fs, ctx.env)?;
    let config = config::resolve(ctx.fs, ctx.env, &layout)?;
    let change = resolve_change(ctx.fs, &layout, requested)?;
    let ctxt = change::load(ctx.fs, &layout, &config, change.clone())?;

    Ok(StatusOutcome {
        status: change::status(ctx.fs, &layout, &ctxt)?,
        planning_home: layout.project_root().to_path_buf(),
        change_root: layout.change_dir(&change),
        warnings: config.warnings,
    })
}

// ─────────────────────────────── instructions ───────────────────────────────

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
            Ok(resolved) => summaries.push(SchemaSummary {
                name,
                origin: origin.label(),
                flow: resolved
                    .graph
                    .topological_order()
                    .iter()
                    .map(|a| a.id.clone())
                    .collect(),
            }),
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

    match scope {
        ValidateArgs::All => Ok(engine_validate::validate_all(
            ctx.fs, ctx.env, &layout, &config,
        )?),
        ValidateArgs::Changes => {
            let mut items = Vec::new();
            for change_id in change::list(ctx.fs, &layout) {
                items.push(engine_validate::validate_change(
                    ctx.fs, &layout, &config, &change_id,
                )?);
            }
            Ok(ValidateReport {
                root: layout.project_root().to_path_buf(),
                items,
            })
        }
        ValidateArgs::Specs => {
            let mut items = Vec::new();
            for capability in specs::list(ctx.fs, &layout) {
                items.push(engine_validate::validate_spec(
                    ctx.fs,
                    &layout,
                    &capability,
                )?);
            }
            Ok(ValidateReport {
                root: layout.project_root().to_path_buf(),
                items,
            })
        }
        ValidateArgs::Item(name) => {
            let item = resolve_validate_item(ctx.fs, &layout, &config, &name)?;
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
    let seal_file = engine_actions::read_seal_file(ctx.fs, &layout).map_err(action_to_failure)?;
    let plan = engine_actions::plan_supersede(
        &index,
        &seal_file,
        old_id,
        new_title,
        &today,
        &layout,
        |path| ctx.fs.read_to_string(path),
    )
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
    pub body_sha256: String,
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
    let seal_file = engine_actions::read_seal_file(ctx.fs, &layout).map_err(action_to_failure)?;

    let plan = engine_actions::plan_deviate(&index, &seal_file, target, new_title, &today, &layout)
        .map_err(action_to_failure)?;

    let target_qualified_id = plan.target_qualified_id.clone();
    let new_path = plan.new_path.clone();
    let body_sha256 = plan.body_sha256.clone();
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
        body_sha256,
    })
}

#[derive(Debug)]
pub struct DecisionPromotedOutcome {
    pub root: PathBuf,
    pub decision: DecisionSummary,
    pub path: PathBuf,
    pub body_sha256: String,
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
    let seal_file = engine_actions::read_seal_file(ctx.fs, &layout).map_err(action_to_failure)?;
    let today = ctx.clock.today();

    let plan = engine_actions::plan_promote(
        &index,
        &seal_file,
        change_name,
        heading,
        &design_source,
        design_path.clone(),
        &today,
        &layout,
    )
    .map_err(action_to_failure)?;

    let new_path = plan.new_path.clone();
    let body_sha256 = plan.body_sha256.clone();
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
        body_sha256,
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
    // `sources update` is the only command that drives `git`.
    let runner: &dyn ProcessRunner = &RealProcessRunner;
    let outcome =
        engine_sources::run_sources_update(ctx.fs, ctx.env, runner, ctx.clock, &layout, &sources)?;
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
    use codev_engine::ports::{FixedClock, FixedEnv, MemoryFileSystem};

    /// Default `init` options for the tests: non-interactive (--yes), probe
    /// disabled (deterministic results, no real manifest to detect in a
    /// default MemoryFileSystem).
    fn test_init_opts() -> crate::init_prompts::InitOptions {
        crate::init_prompts::InitOptions {
            yes: true,
            no_detect: true,
            preset: None,
            language: None,
        }
    }

    struct Harness {
        fs: MemoryFileSystem,
        env: FixedEnv,
        clock: FixedClock,
    }

    impl Harness {
        fn new() -> Self {
            let mut env = FixedEnv::at("/p");
            env.vars.insert("HOME".into(), "/home".into());
            Self {
                fs: MemoryFileSystem::new(),
                env,
                clock: FixedClock("2026-09-08".into()),
            }
        }

        fn with(mut self, path: &str, contents: &str) -> Self {
            self.fs = self.fs.with_file(path, contents);
            self
        }

        fn ctx(&self) -> Ctx<'_> {
            Ctx {
                fs: &self.fs,
                env: &self.env,
                clock: &self.clock,
                version: "0.1.0",
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
        // 8 skills + config.yaml + 5 `.gitkeep`.
        assert_eq!(outcome.created.len(), 14, "{:?}", outcome.created);
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
        let err = new_change(&h.ctx(), "Add Auth", None, None).unwrap_err();
        assert_eq!(err.code, "invalid_change_id");
    }

    #[test]
    fn rejects_an_already_existing_change() {
        let h = Harness::new();
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        new_change(&h.ctx(), "add-auth", None, None).unwrap();
        let err = new_change(&h.ctx(), "add-auth", None, None).unwrap_err();
        assert_eq!(err.code, "change_exists");
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
        assert_eq!(outcome.warnings.len(), 1);
        assert_eq!(outcome.warnings[0].code, "schema_unusable");
    }

    // ─────────────── decision seal ───────────────

    #[test]
    fn decision_new_seals_the_entry_and_exposes_the_hash() {
        // A `decision new` creates the ADR AND the seal entry; the hash
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

    // ─────────────── decision promote ───────────────

    const DESIGN_WITH_BLOCK: &str = "\
# Design: add-auth

## Decisions

### Decision: Use JWT

The rationale for the choice.

## End
";

    #[test]
    fn decision_promote_creates_an_adr_and_references_the_design() {
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
        assert_eq!(outcome.source_change, "add-auth");
        assert!(outcome.body_sha256.starts_with("sha256:"));

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
    fn decision_deviate_creates_a_local_adr_and_seals_it() {
        let h = Harness::new()
            .with("/p/_codev/config.yaml", "inherits:\n  - path: ~/shared\n")
            .with(
                "/home/shared/_codev/decisions/0100.md",
                &inherited_adr("0100"),
            );
        init(&h.ctx(), ".", false, &test_init_opts()).unwrap();
        let outcome = decision_deviate(&h.ctx(), "path:~/shared/0100", "Our alternative").unwrap();

        assert_eq!(outcome.target_qualified_id, "path:~/shared/0100");
        assert!(outcome.body_sha256.starts_with("sha256:"));
        assert_eq!(outcome.decision.id, "0001");
        assert_eq!(outcome.decision.deviates_from, vec!["path:~/shared/0100"]);

        // The seal exists and references 0001.
        use codev_engine::FileSystem;
        let seal_content =
            h.fs.read_to_string(std::path::Path::new("/p/_codev/decisions/seal.yaml"))
                .unwrap();
        assert!(seal_content.contains("0001"));
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
            "# Proposal\n\n## Why\n\nT\n\n## What Changes\n\n- x\n",
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
}
