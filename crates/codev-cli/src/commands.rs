use std::fmt;
use std::path::{Path, PathBuf};

use codev_agents::claude::ClaudeCode;
use codev_agents::target::AgentTarget;
use codev_agents::workflows;
use codev_core::{ChangeId, ChangeStatus, CoreError, Layout};
use codev_engine::apply::{self, Applied};
use codev_engine::config;
use codev_engine::instructions::Instructions;
use codev_engine::metadata::ChangeMetadata;
use codev_engine::archive as engine_archive;
use codev_engine::archive::ArchiveOutcome;
use codev_engine::sync as engine_sync;
use codev_engine::sync::SyncOutcome;
use codev_engine::decisions as engine_decisions;
use codev_engine::decisions_actions as engine_actions;
use codev_engine::sources as engine_sources;
use codev_engine::validate as engine_validate;
use codev_engine::validate::ValidateReport;
use codev_engine::{change, instructions, root, scaffold, schemas, specs};
use codev_engine::{
    Clock, EngineError, Env, FileSystem, ProcessRunner, RealProcessRunner, Warning,
};

/// Un échec de commande, réduit à ce dont les deux sorties ont besoin : un code
/// stable pour le JSON, un message lisible pour le terminal.
#[derive(Debug)]
pub struct Failure {
    pub code: String,
    pub message: String,
    /// La correction à proposer, quand elle est connue.
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

/// Les ports, réunis. Le seul objet que les commandes reçoivent du monde
/// extérieur.
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
}

/// Initialise codev dans un projet.
///
/// Deux temps, et l'ordre compte : on écrit d'abord la structure — ce qui crée
/// `config.yaml` s'il manque — puis on **relit** la configuration pour décider
/// des skills. Un projet qui avait déjà choisi ses workflows garde donc son
/// choix, puisque le scaffolding n'écrase rien.
pub fn init(ctx: &Ctx, path: &str, force: bool) -> Result<SetupOutcome> {
    let root = absolute(ctx, path)?;
    let layout = Layout::new(&root);

    let scaffolded = apply::execute(&scaffold::plan_init(&layout), ctx.fs)?;
    let mut outcome = install_skills(ctx, &layout, force)?;
    outcome.absorb(scaffolded);
    Ok(outcome)
}

/// Régénère les skills d'un projet déjà initialisé.
///
/// Passe aussi le plan de scaffolding : en mode « ne crée que ce qui manque »,
/// il restaure un dossier supprimé sans rien toucher d'autre.
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

    // Le contexte de rendu porte la config MCP du projet : nom du tool
    // Jira à injecter dans le frontmatter, autres MCP à venir. Sans
    // config, RenderCtx est vide et le placeholder est retiré proprement.
    let target = ClaudeCode::with_ctx(codev_agents::claude::RenderCtx {
        jira_mcp_tool: config.mcp.jira_tool.clone(),
    });
    let planned = target.plan_skills(
        ctx.fs,
        layout.project_root(),
        &selected,
        ctx.version,
        force,
    );
    let applied = apply::execute(&planned.plan, ctx.fs)?;

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

    // Le schéma est résolu maintenant, pas à la première commande qui en aura
    // besoin : un nom fautif doit échouer ici, avant qu'un dossier de change ne
    // porte une métadonnée invalide.
    let schema_name = schema.unwrap_or(&config.schema).to_string();
    let resolved = schemas::resolve(ctx.fs, &layout, &schema_name)?;

    let metadata =
        ChangeMetadata::new(resolved.name(), ctx.clock.today()).with_goal(goal);
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
            // Un schéma maison cassé ne doit pas empêcher de lister les autres :
            // c'est justement la commande qu'on lance pour comprendre.
            Err(err) => warnings.push(Warning::new(
                "schema_unusable",
                format!("le schéma « {name} » ({}) est inutilisable : {err}", origin.label()),
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
        ValidateArgs::All => Ok(engine_validate::validate_all(ctx.fs, ctx.env, &layout, &config)?),
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
                items.push(engine_validate::validate_spec(ctx.fs, &layout, &capability)?);
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

/// Comment `validate` a été appelée. `Item` porte une `String` plutôt qu'un
/// `&str` parce que le nom peut venir d'un argument CLI dont le CLI est
/// propriétaire — un `&str` obligerait à propager sa durée de vie.
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
    // Un même nom peut désigner un change ET une capacité de spec — dans ce
    // cas on refuse plutôt que de deviner, en nommant les candidats.
    let changes: Vec<ChangeId> = change::list(fs, layout)
        .into_iter()
        .filter(|c| c.as_str() == name)
        .collect();
    let matching_specs: Vec<String> = specs::list(fs, layout)
        .into_iter()
        .filter(|s| s == name)
        .collect();

    match (changes.len(), matching_specs.len()) {
        (1, 0) => Ok(engine_validate::validate_change(fs, layout, config, &changes[0])?),
        (0, 1) => Ok(engine_validate::validate_spec(fs, layout, &matching_specs[0])?),
        (0, 0) => Err(Failure::new(
            "unknown_item",
            format!("aucun change ni spec ne s'appelle « {name} »"),
        )
        .with_fix("`codev list` et `codev list --specs` disent ce qui existe")),
        _ => Err(Failure::new(
            "ambiguous_item",
            format!(
                "« {name} » désigne à la fois un change et une spec ; précise avec `--changes` ou `--specs`"
            ),
        )),
    }
}

// ─────────────────────────────── sync / archive ───────────────────────────────

pub fn sync(ctx: &Ctx, requested: Option<&str>) -> Result<SyncOutcome> {
    let layout = root::discover_from_cwd(ctx.fs, ctx.env)?;
    let config = config::resolve(ctx.fs, ctx.env, &layout)?;
    let change = resolve_change(ctx.fs, &layout, requested)?;
    Ok(engine_sync::execute_sync(ctx.fs, &layout, &config, &change)?)
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

/// Description enrichie d'une entrée d'index — remonte l'état d'effet et
/// la supersession pour le rendu.
#[derive(Debug)]
pub struct DecisionSummary {
    pub id: String,
    pub qualified_id: String,
    pub title: String,
    pub status: String,
    pub date: String,
    pub tags: Vec<String>,
    pub supersedes: Vec<String>,
    /// Dérives locales — additif (K6). Vide sur les ADR antérieurs.
    pub deviates_from: Vec<String>,
    pub path: PathBuf,
    pub origin: String,
    pub in_effect: bool,
    pub superseded_by: Option<String>,
    /// Renseigné pour les entrées héritées qu'un ADR local écarte via
    /// `deviates_from` (K6). Additif — `None` sinon.
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
    let (idx, _) =
        engine_actions::resolve_old_entry(&index, id).map_err(|err| Failure {
            code: err.code().to_string(),
            message: err.to_string(),
            fix: None,
        })?;
    let summaries = build_summaries(&index, &layout);
    let entry = &index.entries[idx];
    let content = ctx.fs.read_to_string(&entry.path).map_err(|e| {
        Failure::new("unreadable", format!("{}: {e}", entry.path.display()))
    })?;
    // Rechercher le summary correspondant.
    let summary = summaries
        .into_iter()
        .find(|s| s.qualified_id == entry.qualified_id.as_str())
        .expect("l'entrée résolue vient de l'index");
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
    /// Hash du corps du nouvel ADR — `None` si le statut ne se scelle
    /// pas (proposed, deprecated, rejected).
    pub body_sha256: Option<String>,
}

pub fn decision_new(ctx: &Ctx, title: &str, status_raw: &str) -> Result<DecisionCreatedOutcome> {
    let layout = root::discover_from_cwd(ctx.fs, ctx.env)?;
    let cfg = config::resolve(ctx.fs, ctx.env, &layout)?;
    let index = engine_decisions::index(ctx.fs, ctx.env, &layout, &cfg)?;
    let status = codev_core::decisions::DecisionStatus::from_raw(status_raw);
    let today = ctx.clock.today();
    let seal_file = engine_actions::read_seal_file(ctx.fs, &layout).map_err(action_to_failure)?;
    let create_plan = engine_actions::plan_new(
        &index,
        &seal_file,
        title,
        status.clone(),
        &today,
        &layout,
    )
    .map_err(action_to_failure)?;
    apply::execute(&create_plan.plan, ctx.fs)?;

    // On ne remonte le hash que si l'ADR est effectivement scellé (statuts
    // `accepted` / `superseded`). Pour les autres, `body_sha256` reste
    // `None` — champ additif du contrat JSON.
    let body_sha256 = if matches!(
        status,
        codev_core::decisions::DecisionStatus::Accepted
            | codev_core::decisions::DecisionStatus::Superseded
    ) {
        Some(create_plan.body_sha256.clone())
    } else {
        None
    };

    // Relire pour construire un summary à jour. La comparaison se fait sur
    // l'identifiant qualifié — le `path` du summary est relatif au projet,
    // celui du plan est absolu, ils ne coïncideraient jamais tels quels.
    let new_qualified = format!("projet/{}", create_plan.new_id);
    let index_apres = engine_decisions::index(ctx.fs, ctx.env, &layout, &cfg)?;
    let summary = build_summaries(&index_apres, &layout)
        .into_iter()
        .find(|s| s.qualified_id == new_qualified)
        .ok_or_else(|| Failure::new("write_failed", "la décision créée n'a pas été relue"))?;
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

    let new_qualified = format!("projet/{new_id}");
    let index_apres = engine_decisions::index(ctx.fs, ctx.env, &layout, &cfg)?;
    let new_summary = build_summaries(&index_apres, &layout)
        .into_iter()
        .find(|s| s.qualified_id == new_qualified)
        .ok_or_else(|| Failure::new("write_failed", "la décision créée n'a pas été relue"))?;
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
    /// L'identifiant qualifié de la décision qu'on écarte — même forme
    /// que celle qu'accepte `codev decision show`.
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

    let plan = engine_actions::plan_deviate(
        &index,
        &seal_file,
        target,
        new_title,
        &today,
        &layout,
    )
    .map_err(action_to_failure)?;

    let target_qualified_id = plan.target_qualified_id.clone();
    let new_path = plan.new_path.clone();
    let body_sha256 = plan.body_sha256.clone();
    let new_id = plan.new_id.clone();

    apply::execute(&plan.plan, ctx.fs)?;

    // Re-lecture pour construire un summary à jour, exactement comme
    // decision_new/supersede.
    let new_qualified = format!("projet/{new_id}");
    let index_apres = engine_decisions::index(ctx.fs, ctx.env, &layout, &cfg)?;
    let summary = build_summaries(&index_apres, &layout)
        .into_iter()
        .find(|s| s.qualified_id == new_qualified)
        .ok_or_else(|| Failure::new("write_failed", "la dérive créée n'a pas été relue"))?;

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
    /// Le change d'où la promotion vient.
    pub source_change: String,
    /// Le `design.md` qui a été mis à jour (chemin absolu).
    pub design_path: PathBuf,
}

pub fn decision_promote(
    ctx: &Ctx,
    change_name: &str,
    heading: &str,
) -> Result<DecisionPromotedOutcome> {
    let layout = root::discover_from_cwd(ctx.fs, ctx.env)?;
    let cfg = config::resolve(ctx.fs, ctx.env, &layout)?;

    // Résolution du change — refuse d'emblée un dossier introuvable ou
    // archivé, avec un code stable dédié pour l'archivé.
    let change_id = ChangeId::parse(change_name).map_err(Failure::from)?;
    let change_dir = layout.change_dir(&change_id);
    if !ctx.fs.exists(&change_dir) {
        if is_change_in_archive(ctx.fs, &layout, change_name) {
            return Err(Failure::new(
                "cannot_promote_from_archived",
                format!(
                    "le change « {change_name} » est archivé : un design \
                     archivé est de l'histoire, la promotion se fait avant \
                     l'archive"
                ),
            ));
        }
        return Err(Failure::new(
            "unknown_change",
            format!("le change « {change_name} » n'existe pas"),
        ));
    }

    // Design.md du change — requis pour promouvoir.
    let design_path = change_dir.join("design.md");
    if !ctx.fs.exists(&design_path) {
        return Err(Failure::new(
            "design_missing",
            format!(
                "le change « {change_name} » n'a pas de `design.md` — rien à \
                 promouvoir. Crée-le d'abord avec `codev-propose` ou en \
                 éditant à la main."
            ),
        ));
    }
    let design_source = ctx.fs.read_to_string(&design_path).map_err(|e| {
        Failure::new(
            "read_failed",
            format!("lecture de {} impossible : {e}", design_path.display()),
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

    let new_qualified = format!("projet/{new_id}");
    let index_apres = engine_decisions::index(ctx.fs, ctx.env, &layout, &cfg)?;
    let summary = build_summaries(&index_apres, &layout)
        .into_iter()
        .find(|s| s.qualified_id == new_qualified)
        .ok_or_else(|| Failure::new("write_failed", "la décision promue n'a pas été relue"))?;

    Ok(DecisionPromotedOutcome {
        root: layout.project_root().to_path_buf(),
        decision: summary,
        path: new_path,
        body_sha256,
        source_change,
        design_path,
    })
}

/// Cherche un dossier `<date>-<change_name>` sous
/// `_codev/changes/archive/`. Utilisé pour distinguer « change absent »
/// de « change archivé ».
fn is_change_in_archive(fs: &dyn codev_engine::FileSystem, layout: &Layout, change_name: &str) -> bool {
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
    /// `true` si le sceau a été réécrit via `--force` (utile au rendu
    /// humain : « scellé » vs « re-scellé » vs « déjà à jour »).
    pub was_forced: bool,
}

pub fn decision_seal(ctx: &Ctx, id: &str, force: bool) -> Result<DecisionSealedOutcome> {
    let layout = root::discover_from_cwd(ctx.fs, ctx.env)?;
    let cfg = config::resolve(ctx.fs, ctx.env, &layout)?;
    let index = engine_decisions::index(ctx.fs, ctx.env, &layout, &cfg)?;
    let seal_file = engine_actions::read_seal_file(ctx.fs, &layout).map_err(action_to_failure)?;
    let today = ctx.clock.today();

    // On note s'il y avait déjà une entrée pour distinguer, au rendu, le
    // « scellé neuf » du « re-scellé avec force ».
    let had_previous_entry = seal_file.find(id).is_some();

    let plan = engine_actions::plan_seal(
        &index,
        &seal_file,
        id,
        force,
        &today,
        &layout,
        |path| ctx.fs.read_to_string(path),
    )
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

/// Convertit l'index en `DecisionSummary` avec effet et supersession.
fn build_summaries(
    index: &engine_decisions::DecisionIndex,
    layout: &Layout,
) -> Vec<DecisionSummary> {
    // Précalcule qui supersede qui : `superseded_by[id] = qualified_id` du
    // superseder.
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
    let in_effect_set: std::collections::BTreeSet<String> = index
        .in_effect
        .iter()
        .map(|q| q.as_str())
        .collect();

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
                    engine_decisions::Origin::Project => "projet".into(),
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
    // `sources update` est la seule commande qui pilote `git`.
    let runner: &dyn ProcessRunner = &RealProcessRunner;
    let outcome = engine_sources::run_sources_update(
        ctx.fs, ctx.env, runner, ctx.clock, &layout, &sources,
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
                format!("aucune source déclarée avec l'adresse « {target} »"),
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

// ─────────────────────────────── aides ───────────────────────────────

/// Résout sur quel change agir.
///
/// Sans nom explicite : s'il n'y a qu'un seul change actif, c'est celui-là — le
/// cas courant, et l'exiger serait de la cérémonie. Au-delà, on refuse en
/// listant les candidats plutôt que d'en choisir un au hasard.
fn resolve_change(
    fs: &dyn FileSystem,
    layout: &Layout,
    requested: Option<&str>,
) -> Result<ChangeId> {
    if let Some(name) = requested {
        return Ok(ChangeId::parse(name)?);
    }

    let actifs = change::list(fs, layout);
    match actifs.len() {
        1 => Ok(actifs.into_iter().next().expect("un seul élément")),
        0 => Err(Failure::new(
            "no_active_change",
            "aucun change actif dans ce projet",
        )
        .with_fix("crée-en un avec `codev new change <nom>`")),
        _ => {
            let noms: Vec<String> = actifs.iter().map(ToString::to_string).collect();
            Err(Failure::new(
                "ambiguous_change",
                format!(
                    "plusieurs changes actifs : {} — précise lequel",
                    noms.join(", ")
                ),
            )
            .with_fix(format!("`--change {}`", noms[0])))
        }
    }
}

fn absolute(ctx: &Ctx, path: &str) -> Result<PathBuf> {
    let candidate = Path::new(path);
    let joined = if candidate.is_absolute() {
        candidate.to_path_buf()
    } else {
        let cwd = ctx.env.current_dir().map_err(|e| {
            Failure::new("unreadable", format!("dossier courant illisible : {e}"))
        })?;
        cwd.join(candidate)
    };
    Ok(clean(joined))
}

/// Retire les composants inutiles d'un chemin.
///
/// Sans cela, `codev init` lancé sans argument affiche « /mon/projet/. » : le
/// chemin est correct, mais un utilisateur qui le lit se demande ce que fait ce
/// point. On ne canonicalise pas — le dossier peut ne pas encore exister.
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

    struct Harnais {
        fs: MemoryFileSystem,
        env: FixedEnv,
        clock: FixedClock,
    }

    impl Harnais {
        fn neuf() -> Self {
            let mut env = FixedEnv::at("/p");
            env.vars.insert("HOME".into(), "/home".into());
            Self {
                fs: MemoryFileSystem::new(),
                env,
                clock: FixedClock("2026-09-08".into()),
            }
        }

        fn avec(mut self, path: &str, contents: &str) -> Self {
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
    fn init_cree_la_structure_et_les_skills() {
        let h = Harnais::neuf();
        let outcome = init(&h.ctx(), ".", false).unwrap();

        assert_eq!(outcome.root, PathBuf::from("/p"));
        assert_eq!(
            outcome.skills,
            ["codev-propose", "codev-explore", "codev-onboard"]
        );
        assert!(h.fs.read("/p/_codev/config.yaml").is_some());
        assert!(h
            .fs
            .read("/p/.claude/skills/codev-propose/SKILL.md")
            .is_some_and(|c| c.contains("name: codev-propose")));
        assert!(outcome.changed_anything());
    }

    #[test]
    fn relancer_init_ne_change_rien() {
        let h = Harnais::neuf();
        init(&h.ctx(), ".", false).unwrap();
        let second = init(&h.ctx(), ".", false).unwrap();
        assert!(
            !second.changed_anything(),
            "créé : {:?}, mis à jour : {:?}",
            second.created,
            second.updated
        );
    }

    #[test]
    fn init_respecte_les_workflows_deja_configures() {
        let h = Harnais::neuf().avec("/p/_codev/config.yaml", "workflows:\n  - explore\n");
        let outcome = init(&h.ctx(), ".", false).unwrap();
        assert_eq!(outcome.skills, ["codev-explore"]);
        assert!(h.fs.read("/p/.claude/skills/codev-propose/SKILL.md").is_none());
    }

    #[test]
    fn update_exige_un_projet_existant() {
        let h = Harnais::neuf();
        let err = update(&h.ctx(), false).unwrap_err();
        assert_eq!(err.code, "no_codev_root");
    }

    #[test]
    fn le_cycle_complet_mene_a_des_instructions() {
        // La tranche verticale du lot 1, de bout en bout : init, new change,
        // status, instructions.
        let h = Harnais::neuf();
        init(&h.ctx(), ".", false).unwrap();

        let cree = new_change(&h.ctx(), "add-auth", None, Some("Ajouter l'auth".into())).unwrap();
        assert_eq!(cree.schema_name, "spec-driven");
        assert!(h
            .fs
            .read("/p/_codev/changes/add-auth/change.yaml")
            .is_some_and(|c| c.contains("created: 2026-09-08")));

        // Sans `--change` : un seul change actif, donc pas d'ambiguïté.
        let statut = status(&h.ctx(), None).unwrap();
        assert_eq!(statut.status.change.as_str(), "add-auth");
        assert!(!statut.status.planning_complete);

        let instr = artifact_instructions(&h.ctx(), None, None).unwrap();
        assert_eq!(instr.artifact_id, "proposal");
        assert!(instr.template.is_some());
        assert!(instr.instruction.is_some());
    }

    #[test]
    fn refuse_un_nom_de_change_invalide() {
        let h = Harnais::neuf();
        init(&h.ctx(), ".", false).unwrap();
        let err = new_change(&h.ctx(), "Add Auth", None, None).unwrap_err();
        assert_eq!(err.code, "invalid_change_id");
    }

    #[test]
    fn refuse_un_change_deja_existant() {
        let h = Harnais::neuf();
        init(&h.ctx(), ".", false).unwrap();
        new_change(&h.ctx(), "add-auth", None, None).unwrap();
        let err = new_change(&h.ctx(), "add-auth", None, None).unwrap_err();
        assert_eq!(err.code, "change_exists");
    }

    #[test]
    fn refuse_un_schema_inconnu_avant_de_creer_le_dossier() {
        let h = Harnais::neuf();
        init(&h.ctx(), ".", false).unwrap();
        let err = new_change(&h.ctx(), "add-auth", Some("fantaisie"), None).unwrap_err();
        assert_eq!(err.code, "schema_not_found");
        assert!(
            h.fs.read("/p/_codev/changes/add-auth/change.yaml").is_none(),
            "aucun dossier ne doit rester derrière un échec"
        );
    }

    #[test]
    fn status_sans_change_actif_oriente_vers_la_creation() {
        let h = Harnais::neuf();
        init(&h.ctx(), ".", false).unwrap();
        let err = status(&h.ctx(), None).unwrap_err();
        assert_eq!(err.code, "no_active_change");
        assert!(err.fix.is_some_and(|f| f.contains("codev new change")));
    }

    #[test]
    fn status_avec_plusieurs_changes_refuse_de_deviner() {
        let h = Harnais::neuf();
        init(&h.ctx(), ".", false).unwrap();
        new_change(&h.ctx(), "add-auth", None, None).unwrap();
        new_change(&h.ctx(), "fix-bug", None, None).unwrap();

        let err = status(&h.ctx(), None).unwrap_err();
        assert_eq!(err.code, "ambiguous_change");
        assert!(err.message.contains("add-auth") && err.message.contains("fix-bug"));
    }

    #[test]
    fn liste_les_changes_et_les_specs() {
        let h = Harnais::neuf().avec("/p/_codev/specs/user-auth/spec.md", "# spec");
        init(&h.ctx(), ".", false).unwrap();
        new_change(&h.ctx(), "add-auth", None, None).unwrap();

        let changes = list_changes(&h.ctx()).unwrap();
        assert_eq!(
            changes.changes.iter().map(ToString::to_string).collect::<Vec<_>>(),
            ["add-auth"]
        );

        let specs = list_specs(&h.ctx()).unwrap();
        assert_eq!(specs.specs, ["user-auth"]);
    }

    #[test]
    fn liste_les_schemas_avec_leur_enchainement() {
        let h = Harnais::neuf();
        init(&h.ctx(), ".", false).unwrap();
        let outcome = list_schemas(&h.ctx()).unwrap();

        assert_eq!(outcome.schemas.len(), 1);
        assert_eq!(outcome.schemas[0].name, "spec-driven");
        assert_eq!(outcome.schemas[0].origin, "intégré");
        assert_eq!(
            outcome.schemas[0].flow,
            ["proposal", "specs", "design", "tasks"]
        );
    }

    #[test]
    fn sync_un_seul_change_actif_est_implicite() {
        // Un seul change actif → sync sans nom marche.
        let h = Harnais::neuf();
        init(&h.ctx(), ".", false).unwrap();
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
    fn archive_avec_validate_erreur_echoue_code_util() {
        let h = Harnais::neuf();
        init(&h.ctx(), ".", false).unwrap();
        new_change(&h.ctx(), "buggy", None, None).unwrap();
        write_helper(
            &h,
            "/p/_codev/changes/buggy/specs/x/spec.md",
            // Doublon → validate le remonte.
            "## Purpose\n\nx.\n\n## ADDED Requirements\n\n### Requirement: A\nThe system SHALL a.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n\n### Requirement: A\nThe system SHALL a.\n\n#### Scenario: T\n- **WHEN** c\n- **THEN** d\n",
        );
        let err = archive(&h.ctx(), None).unwrap_err();
        assert_eq!(err.code, "invalid");
        assert!(err.message.contains("validation_failed"), "{}", err.message);
        // Le change n'a pas bougé.
        assert!(h.fs.read("/p/_codev/changes/buggy/change.yaml").is_some());
    }

    #[test]
    fn validate_projet_propre_ne_produit_aucun_finding() {
        let h = Harnais::neuf();
        init(&h.ctx(), ".", false).unwrap();
        new_change(&h.ctx(), "add-auth", None, None).unwrap();
        // Un delta bien formé sous le change.
        write_helper(
            &h,
            "/p/_codev/changes/add-auth/specs/user-auth/spec.md",
            "## Purpose\n\nAuthentification des utilisateurs.\n\n## ADDED Requirements\n\n### Requirement: Login\nThe system SHALL emit a token.\n\n#### Scenario: OK\n- **WHEN** login\n- **THEN** token\n",
        );

        let report = validate(&h.ctx(), ValidateArgs::All).unwrap();
        assert!(
            !report.has_errors(),
            "aucun finding attendu ; obtenu : {:#?}",
            report
        );
        // 1 change + 1 item décisions (toujours présent, même sans ADR).
        assert_eq!(report.items.len(), 2);
    }

    #[test]
    fn validate_attrape_les_findings_du_parseur_et_des_regles() {
        let h = Harnais::neuf();
        init(&h.ctx(), ".", false).unwrap();
        new_change(&h.ctx(), "buggy", None, None).unwrap();
        // Un delta où SHALL manque (règle E1) et le scénario est mal formé
        // (règle du parseur) : les deux findings doivent remonter.
        write_helper(
            &h,
            "/p/_codev/changes/buggy/specs/x/spec.md",
            "## Purpose\n\nCapacité de test des règles.\n\n## ADDED Requirements\n\n### Requirement: X\nThe system does x.\n\n### Scenario: MalForme\n- **WHEN** a\n- **THEN** b\n",
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
            "règle E1 attendue ; codes : {codes:?}"
        );
        assert!(
            codes.contains(&"scenario_wrong_heading_level"),
            "règle du parseur attendue ; codes : {codes:?}"
        );
    }

    #[test]
    fn validate_zero_delta_sans_marqueur_echoue() {
        // La règle E3 : un change sans delta et sans skip_specs échoue.
        let h = Harnais::neuf();
        init(&h.ctx(), ".", false).unwrap();
        new_change(&h.ctx(), "refactor", None, None).unwrap();

        let report = validate(&h.ctx(), ValidateArgs::Changes).unwrap();
        assert!(report.has_errors());
        assert!(report.items[0]
            .findings
            .iter()
            .any(|f| f.finding.code == "zero_delta_without_marker"));
    }

    #[test]
    fn validate_scope_specs_ignore_les_changes() {
        // Un change en erreur mais on demande seulement les specs : rien à
        // remonter, exit 0.
        let h = Harnais::neuf();
        init(&h.ctx(), ".", false).unwrap();
        new_change(&h.ctx(), "refactor", None, None).unwrap(); // zero-delta

        let report = validate(&h.ctx(), ValidateArgs::Specs).unwrap();
        assert!(!report.has_errors());
        assert!(report.items.is_empty());
    }

    #[test]
    fn validate_item_inconnu_echoue_avec_code_utile() {
        let h = Harnais::neuf();
        init(&h.ctx(), ".", false).unwrap();
        let err = validate(&h.ctx(), ValidateArgs::Item("fantome".into())).unwrap_err();
        assert_eq!(err.code, "unknown_item");
        assert!(err.fix.is_some_and(|f| f.contains("codev list")));
    }

    /// Petit helper qui insère un fichier dans le FS en mémoire du harnais.
    ///
    /// Vit ici plutôt que dans `Harnais` parce qu'il n'est utile qu'aux tests
    /// de `validate` — les autres commandes n'ajoutent pas de contenu manuel.
    fn write_helper(h: &Harnais, path: &str, contents: &str) {
        use codev_engine::FileSystem;
        h.fs.write(std::path::Path::new(path), contents).unwrap();
    }

    #[test]
    fn un_schema_maison_casse_est_signale_sans_masquer_les_autres() {
        let h = Harnais::neuf().avec(
            "/p/_codev/schemas/casse/schema.yaml",
            "name: casse\nartifacts:\n  - id: a\n    generates: a.md\n    requires: [fantome]\napply:\n  requires: [a]\n  tracks: a.md\n",
        );
        init(&h.ctx(), ".", false).unwrap();
        let outcome = list_schemas(&h.ctx()).unwrap();

        assert_eq!(outcome.schemas.len(), 1, "spec-driven reste listé");
        assert_eq!(outcome.warnings.len(), 1);
        assert_eq!(outcome.warnings[0].code, "schema_unusable");
    }

    // ─────────────── decision seal (K3) ───────────────

    #[test]
    fn decision_new_scelle_lentree_et_expose_le_hash() {
        // Un `decision new` crée l'ADR ET l'entrée de sceau ; le hash
        // remonte dans l'outcome pour le contrat JSON.
        let h = Harnais::neuf();
        init(&h.ctx(), ".", false).unwrap();
        let outcome = decision_new(&h.ctx(), "Un premier choix", "accepted").unwrap();
        assert!(outcome.body_sha256.is_some());
        assert!(outcome.body_sha256.as_ref().unwrap().starts_with("sha256:"));

        // Le fichier de sceau existe et référence 0001.
        use codev_engine::FileSystem;
        let seal_content = h
            .fs
            .read_to_string(std::path::Path::new("/p/_codev/decisions/seal.yaml"))
            .unwrap();
        assert!(seal_content.contains("0001"));
    }

    #[test]
    fn decision_new_proposed_ne_scelle_pas_ni_nexpose_de_hash() {
        // Un statut `proposed` n'engage pas d'immutabilité — pas de sceau,
        // pas de hash dans l'outcome.
        let h = Harnais::neuf();
        init(&h.ctx(), ".", false).unwrap();
        let outcome = decision_new(&h.ctx(), "Piste", "proposed").unwrap();
        assert!(outcome.body_sha256.is_none());

        use codev_engine::FileSystem;
        assert!(!h
            .fs
            .exists(std::path::Path::new("/p/_codev/decisions/seal.yaml")));
    }

    #[test]
    fn decision_seal_est_noop_apres_decision_new() {
        // `decision new` scelle déjà l'ADR ; un `decision seal` juste
        // après doit être un no-op silencieux.
        let h = Harnais::neuf();
        init(&h.ctx(), ".", false).unwrap();
        decision_new(&h.ctx(), "Un premier choix", "accepted").unwrap();
        let outcome = decision_seal(&h.ctx(), "0001", false).unwrap();
        assert!(outcome.was_noop);
        assert!(!outcome.was_forced);
    }

    #[test]
    fn decision_seal_refuse_sans_force_apres_modification_du_corps() {
        let h = Harnais::neuf();
        init(&h.ctx(), ".", false).unwrap();
        decision_new(&h.ctx(), "Un premier choix", "accepted").unwrap();

        // On corrompt le corps de l'ADR après scellement.
        use codev_engine::FileSystem;
        let adr_path = std::path::Path::new("/p/_codev/decisions/0001-un-premier-choix.md");
        let source = h.fs.read_to_string(adr_path).unwrap();
        let modifie = source.replace("## Contexte", "## Contexte MODIFIÉ EN PLACE");
        assert_ne!(modifie, source, "la substitution doit changer le corps");
        h.fs.write(adr_path, &modifie).unwrap();

        let err = decision_seal(&h.ctx(), "0001", false).unwrap_err();
        assert_eq!(err.code, "seal_conflict");
    }

    #[test]
    fn decision_seal_avec_force_reecrit_apres_modification() {
        let h = Harnais::neuf();
        init(&h.ctx(), ".", false).unwrap();
        decision_new(&h.ctx(), "Un premier choix", "accepted").unwrap();

        use codev_engine::FileSystem;
        let adr_path = std::path::Path::new("/p/_codev/decisions/0001-un-premier-choix.md");
        let source = h.fs.read_to_string(adr_path).unwrap();
        let modifie = source.replace("## Contexte", "## Contexte MODIFIÉ EN PLACE");
        assert_ne!(modifie, source, "la substitution doit changer le corps");
        h.fs.write(adr_path, &modifie).unwrap();

        let outcome = decision_seal(&h.ctx(), "0001", true).unwrap();
        assert!(!outcome.was_noop);
        assert!(outcome.was_forced);
    }

    // ─────────────── decision promote (K7) ───────────────

    const DESIGN_AVEC_BLOC: &str = "\
# Design : add-auth

## Décisions

### Décision : Utiliser JWT

Le rationale du choix.

## Fin
";

    #[test]
    fn decision_promote_cree_un_adr_et_reference_le_design() {
        let h = Harnais::neuf();
        init(&h.ctx(), ".", false).unwrap();
        new_change(&h.ctx(), "add-auth", None, None).unwrap();
        // Un design.md avec un bloc de décision.
        use codev_engine::FileSystem;
        h.fs
            .write(
                std::path::Path::new("/p/_codev/changes/add-auth/design.md"),
                DESIGN_AVEC_BLOC,
            )
            .unwrap();

        let outcome = decision_promote(&h.ctx(), "add-auth", "Utiliser JWT").unwrap();
        assert_eq!(outcome.decision.id, "0001");
        assert_eq!(outcome.source_change, "add-auth");
        assert!(outcome.body_sha256.starts_with("sha256:"));

        // L'ADR existe.
        assert!(h.fs.exists(std::path::Path::new(
            "/p/_codev/decisions/0001-utiliser-jwt.md"
        )));
        // Le design a été réécrit avec la référence.
        let design = h
            .fs
            .read_to_string(std::path::Path::new("/p/_codev/changes/add-auth/design.md"))
            .unwrap();
        assert!(design.contains("### Décision : Utiliser JWT\n\n> Promue en ADR **0001**"));
        assert!(!design.contains("Le rationale du choix."));
    }

    #[test]
    fn decision_promote_refuse_un_change_absent() {
        let h = Harnais::neuf();
        init(&h.ctx(), ".", false).unwrap();
        let err = decision_promote(&h.ctx(), "fantome", "X").unwrap_err();
        assert_eq!(err.code, "unknown_change");
    }

    #[test]
    fn decision_promote_refuse_un_design_absent() {
        let h = Harnais::neuf();
        init(&h.ctx(), ".", false).unwrap();
        new_change(&h.ctx(), "add-auth", None, None).unwrap();
        // Pas de design.md créé.
        let err = decision_promote(&h.ctx(), "add-auth", "X").unwrap_err();
        assert_eq!(err.code, "design_missing");
    }

    #[test]
    fn decision_promote_refuse_un_titre_absent() {
        let h = Harnais::neuf();
        init(&h.ctx(), ".", false).unwrap();
        new_change(&h.ctx(), "add-auth", None, None).unwrap();
        use codev_engine::FileSystem;
        h.fs
            .write(
                std::path::Path::new("/p/_codev/changes/add-auth/design.md"),
                DESIGN_AVEC_BLOC,
            )
            .unwrap();
        let err = decision_promote(&h.ctx(), "add-auth", "Fantome").unwrap_err();
        assert_eq!(err.code, "decision_heading_not_found");
    }

    #[test]
    fn decision_promote_refuse_un_change_archive() {
        // On simule un change archivé en créant directement le dossier
        // sous `archive/<date>-<name>/`.
        let h = Harnais::neuf().avec(
            "/p/_codev/changes/archive/2026-09-01-old/design.md",
            DESIGN_AVEC_BLOC,
        );
        init(&h.ctx(), ".", false).unwrap();
        let err = decision_promote(&h.ctx(), "old", "Utiliser JWT").unwrap_err();
        assert_eq!(err.code, "cannot_promote_from_archived");
    }

    // ─────────────── decision deviate (K6) ───────────────

    fn adr_heritee(id: &str) -> String {
        format!(
            "---\nid: \"{id}\"\ntitle: Choix source\nstatus: accepted\ndate: 2026-09-08\n---\n\n## Contexte\n\nx\n"
        )
    }

    #[test]
    fn decision_deviate_cree_un_adr_local_et_le_scelle() {
        let h = Harnais::neuf()
            .avec(
                "/p/_codev/config.yaml",
                "inherits:\n  - path: ~/partage\n",
            )
            .avec(
                "/home/partage/_codev/decisions/0100.md",
                &adr_heritee("0100"),
            );
        init(&h.ctx(), ".", false).unwrap();
        let outcome = decision_deviate(&h.ctx(), "path:~/partage/0100", "Notre alternative").unwrap();

        assert_eq!(outcome.target_qualified_id, "path:~/partage/0100");
        assert!(outcome.body_sha256.starts_with("sha256:"));
        assert_eq!(outcome.decision.id, "0001");
        assert_eq!(outcome.decision.deviates_from, vec!["path:~/partage/0100"]);

        // Le sceau existe et référence 0001.
        use codev_engine::FileSystem;
        let seal_content = h
            .fs
            .read_to_string(std::path::Path::new("/p/_codev/decisions/seal.yaml"))
            .unwrap();
        assert!(seal_content.contains("0001"));
    }

    #[test]
    fn decision_deviate_refuse_une_locale_avec_renvoi_vers_supersede() {
        let h = Harnais::neuf();
        init(&h.ctx(), ".", false).unwrap();
        decision_new(&h.ctx(), "Un local", "accepted").unwrap();
        let err = decision_deviate(&h.ctx(), "projet/0001", "…").unwrap_err();
        assert_eq!(err.code, "cannot_deviate_from_local");
        assert!(err.message.contains("supersede"));
    }

    #[test]
    fn decision_deviate_refuse_une_cible_inconnue() {
        let h = Harnais::neuf();
        init(&h.ctx(), ".", false).unwrap();
        let err = decision_deviate(&h.ctx(), "path:~/inconnue/0100", "…").unwrap_err();
        assert_eq!(err.code, "unknown_decision_id");
    }

    #[test]
    fn decision_list_expose_deviated_by_sur_lheritée() {
        let h = Harnais::neuf()
            .avec(
                "/p/_codev/config.yaml",
                "inherits:\n  - path: ~/partage\n",
            )
            .avec(
                "/home/partage/_codev/decisions/0100.md",
                &adr_heritee("0100"),
            );
        init(&h.ctx(), ".", false).unwrap();
        decision_deviate(&h.ctx(), "path:~/partage/0100", "Notre alt").unwrap();
        let list = decision_list(&h.ctx()).unwrap();

        let heritee = list
            .decisions
            .iter()
            .find(|d| d.qualified_id == "path:~/partage/0100")
            .expect("héritée présente dans le listing");
        assert_eq!(heritee.deviated_by.as_deref(), Some("projet/0001"));
        assert!(!heritee.in_effect);

        let locale = list
            .decisions
            .iter()
            .find(|d| d.qualified_id == "projet/0001")
            .expect("locale présente");
        assert_eq!(locale.deviates_from, vec!["path:~/partage/0100"]);
        assert!(locale.in_effect);
    }

    // ─────────────── validate --strict (E5) ───────────────

    #[test]
    fn validate_projet_propre_na_ni_erreur_ni_warning() {
        // Baseline pour le mode strict : sans finding, ni `has_errors`
        // ni `has_warnings` ne remontent.
        let h = Harnais::neuf();
        init(&h.ctx(), ".", false).unwrap();
        new_change(&h.ctx(), "add-auth", None, None).unwrap();
        // Un seul artefact suffisant pour que validate_change ne remonte pas
        // d'erreur (proposal simple).
        use codev_engine::FileSystem;
        h.fs
            .write(
                std::path::Path::new("/p/_codev/changes/add-auth/proposal.md"),
                "# Proposal\n\n## Pourquoi\n\nT\n\n## Ce qui change\n\n- x\n",
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
    fn validate_avec_adr_non_scelle_a_warnings_mais_pas_derreurs() {
        // Un ADR local `accepted` sans sceal.yaml → warning
        // `decision_unsealed`. C'est le cas typique où `--strict`
        // fera basculer l'exit code du CLI.
        let h = Harnais::neuf().avec(
            "/p/_codev/decisions/0001.md",
            "---\nid: \"0001\"\ntitle: T\nstatus: accepted\ndate: 2026-09-08\n---\n\n## Contexte\n\nx\n",
        );
        init(&h.ctx(), ".", false).unwrap();
        let report = validate(&h.ctx(), ValidateArgs::All).unwrap();
        assert!(!report.has_errors(), "warnings uniquement");
        assert!(report.has_warnings(), "au moins un warning attendu");
    }

    #[test]
    fn validate_avec_seal_mismatch_a_erreur() {
        // Un mismatch est une erreur, indépendante du mode strict.
        let h = Harnais::neuf();
        init(&h.ctx(), ".", false).unwrap();
        decision_new(&h.ctx(), "Un premier choix", "accepted").unwrap();
        // On corrompt le corps de l'ADR après scellement.
        use codev_engine::FileSystem;
        let adr_path = std::path::Path::new("/p/_codev/decisions/0001-un-premier-choix.md");
        let source = h.fs.read_to_string(adr_path).unwrap();
        let modifie = source.replace("## Contexte", "## Contexte ALTÉRÉ");
        h.fs.write(adr_path, &modifie).unwrap();

        let report = validate(&h.ctx(), ValidateArgs::All).unwrap();
        assert!(report.has_errors(), "mismatch → erreur");
        // Le mismatch n'est pas un warning ; has_warnings peut être
        // faux ou vrai selon les autres findings, mais l'exit code
        // aurait basculé sur `has_errors` seul, sans dépendre de strict.
    }
}
