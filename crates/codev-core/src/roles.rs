//! Roles: who owns which question at which step of the workflow, the
//! "done when" criteria it checks there, and the knowledge it needs to
//! check them well.
//!
//! Roles are data. The built-in roles come from `assets/roles/<id>.md`,
//! embedded here: a YAML frontmatter with the catalog data, then the
//! role's knowledge as markdown. The artifact roles come from the schema in
//! use. This module merges them into one catalog, checks the project's and
//! the change's settings against it, and decides which roles are active —
//! all without touching a file (decision 0001): the engine reads the files
//! and hands their content over.

use std::collections::BTreeMap;

use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_norway::Value;

use crate::schema::Schema;
use crate::triggers::{self, TriggerGroup};

/// The workflow steps a role can serve, in workflow order. Listing roles
/// follows this order, so a change reads from the analyst to the newcomer.
pub const STEPS: &[&str] = &[
    "plan.understand",
    "plan.investigate",
    "plan.write",
    "plan.trace",
    "plan.challenge",
    "code.implement",
    "code.review",
];

/// The step of every artifact role: writing the planning artifacts.
pub const ARTIFACT_STEP: &str = "plan.write";

/// The origin of a criterion that comes from the built-in catalog.
pub const BUILTIN_ORIGIN: &str = "built-in";

/// The origin of a criterion, or an activation, set in `_codev/config.yaml`.
pub const PROJECT_ORIGIN: &str = "project";

pub const UNKNOWN_ROLE: &str = "unknown_role";
pub const INVALID_ROLE_ACTIVATION: &str = "invalid_role_activation";
pub const ROLE_NOT_DISABLEABLE: &str = "role_not_disableable";
pub const INVALID_ROLE_MODEL: &str = "invalid_role_model";
/// A warning, not an error: a model on a role that never runs as its own
/// sub-agent has no effect, and a project is never blocked for that.
pub const ROLE_MODEL_UNUSED: &str = "role_model_unused";

/// The steps whose roles run as their own sub-agent in the `isolated`
/// review mode: the only steps where a role's model applies.
pub const REVIEWING_STEPS: &[&str] = &["plan.trace", "plan.challenge", "code.review"];

/// The one role whose model applies outside a review: the `apply` skill
/// hands the tasks to a sub-agent on it at `code.implement`.
pub const DEVELOPER: &str = "developer";

/// What a project writes under `roles:` to keep a role on the session's
/// model.
pub const INHERIT_MODEL: &str = "inherit";

/// The sections a role's knowledge may use, in this order.
pub const KNOWLEDGE_SECTIONS: &[&str] =
    &["Scope", "Signals", "By stack", "References", "Reporting"];

/// The built-in roles, one file each, in catalog order: the order of this
/// list is the order of the roles within a step.
const BUILTIN_ROLE_FILES: &[&str] = &[
    include_str!("../../../assets/roles/analyst.md"),
    include_str!("../../../assets/roles/investigator.md"),
    include_str!("../../../assets/roles/tracer.md"),
    include_str!("../../../assets/roles/skeptic.md"),
    include_str!("../../../assets/roles/security.md"),
    include_str!("../../../assets/roles/privacy.md"),
    include_str!("../../../assets/roles/accessibility.md"),
    include_str!("../../../assets/roles/performance.md"),
    include_str!("../../../assets/roles/ops.md"),
    include_str!("../../../assets/roles/migration.md"),
    include_str!("../../../assets/roles/developer.md"),
    include_str!("../../../assets/roles/newcomer.md"),
];

/// Display names by language code (`en`, `fr`…). `en` is always present.
pub type Names = BTreeMap<String, String>;

/// The `role:` of a schema artifact, as written in `schema.yaml`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ArtifactRole {
    pub id: String,
    pub name: Names,
    pub question: String,
    #[serde(default)]
    pub done_when: Vec<String>,
}

/// The frontmatter of a built-in role file.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RoleFrontmatter {
    id: String,
    name: Names,
    at: OneOrMany,
    question: String,
    #[serde(default)]
    required: bool,
    #[serde(default)]
    domains: Vec<String>,
    #[serde(default)]
    done_when: Vec<String>,
    /// The model the role's sub-agent runs on; absent for the session's.
    #[serde(default)]
    model: Option<String>,
    /// Read by [`triggers::read_groups`], which names the group and the
    /// field of a defect.
    #[serde(default)]
    triggers: Option<Value>,
}

/// `at: plan.trace` or `at: [plan.challenge, code.review]`.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum OneOrMany {
    One(String),
    Many(Vec<String>),
}

impl OneOrMany {
    fn into_vec(self) -> Vec<String> {
        match self {
            Self::One(step) => vec![step],
            Self::Many(steps) => steps,
        }
    }
}

/// Parses a built-in role file: the YAML frontmatter between two `---`
/// lines, then the knowledge, trimmed. A role with no knowledge has an
/// empty body.
pub fn parse_role_file(source: &str) -> Result<Role, String> {
    let source = source.replace("\r\n", "\n");
    let rest = source
        .strip_prefix("---\n")
        .ok_or("a role file starts with a `---` line")?;
    let (frontmatter, body) = match rest.split_once("\n---\n") {
        Some(parts) => parts,
        None => rest
            .strip_suffix("\n---")
            .map(|frontmatter| (frontmatter, ""))
            .ok_or("the frontmatter of a role file ends with a `---` line")?,
    };
    let data: RoleFrontmatter =
        serde_norway::from_str(frontmatter).map_err(|err| err.to_string())?;
    let body = body.trim();
    let model = match data.model.as_deref() {
        None => None,
        Some(text) => Some(Model::parse(text).ok_or_else(|| {
            format!(
                "role `{}` has the model `{text}`; use {}",
                data.id,
                Model::ALLOWED
            )
        })?),
    };
    let read = triggers::read_groups(data.triggers.as_ref());
    if let Some(problem) = read.problems.first() {
        return Err(problem.message.clone());
    }
    Ok(Role {
        criteria: criteria(&data.done_when, BUILTIN_ORIGIN),
        id: data.id,
        names: data.name,
        kind: RoleKind::Step,
        steps: data.at.into_vec(),
        question: data.question,
        disableable: !data.required,
        domains: data.domains,
        knowledge: (!body.is_empty()).then(|| body.to_string()),
        triggers: read.groups,
        model,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoleKind {
    Step,
    /// The role that owns this artifact of the schema.
    Artifact(String),
}

/// One "done when" criterion, with where it comes from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Criterion {
    /// `built-in`, `schema:<name>` or `project`.
    pub origin: String,
    pub text: String,
}

/// A role of the catalog, before any setting is applied.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Role {
    pub id: String,
    pub names: Names,
    pub kind: RoleKind,
    /// The steps the role serves, each one of [`STEPS`], in workflow order.
    pub steps: Vec<String>,
    pub question: String,
    pub criteria: Vec<Criterion>,
    /// False for artifact roles and for the built-in roles marked
    /// `required`.
    pub disableable: bool,
    /// The domains that activate an `on_domain` role; empty for a role
    /// bound to none. See [`crate::domains::DOMAINS`].
    pub domains: Vec<String>,
    /// The role's knowledge, as markdown; `None` when it has none.
    pub knowledge: Option<String>,
    /// The evidence in a change's diff that suggests a domain; empty for a
    /// role that carries none.
    pub triggers: Vec<TriggerGroup>,
    /// The model the role's sub-agent runs on by default; `None` keeps the
    /// session's model.
    pub model: Option<Model>,
}

impl Role {
    /// The display name in `language`, English when the role has none in
    /// that language.
    pub fn name(&self, language: &str) -> &str {
        self.names
            .get(language)
            .or_else(|| self.names.get("en"))
            .map_or(self.id.as_str(), String::as_str)
    }

    /// The first step the role serves: where it appears in the catalog.
    pub fn first_step(&self) -> &str {
        self.steps.first().map_or("", String::as_str)
    }

    pub fn serves(&self, step: &str) -> bool {
        self.steps.iter().any(|s| s == step)
    }

    /// True when the role serves a step where it runs as its own sub-agent
    /// in the `isolated` mode, so that a model applies to it.
    pub fn reviews(&self) -> bool {
        REVIEWING_STEPS.iter().any(|step| self.serves(step))
    }

    /// `on_domain` for a role bound to domains, `always` otherwise.
    pub fn default_activation(&self) -> Activation {
        if self.domains.is_empty() {
            Activation::Always
        } else {
            Activation::OnDomain
        }
    }
}

/// The built-in roles, in catalog order.
pub fn builtin_step_roles() -> Vec<Role> {
    // Parsed once per process: checking the experts' triggers compiles
    // their expressions, which is worth doing once, not per call.
    static ROLES: std::sync::OnceLock<Vec<Role>> = std::sync::OnceLock::new();
    ROLES
        .get_or_init(|| {
            // The files are embedded: a parse failure is a build bug, which
            // the test `the_builtin_catalog_has_the_step_roles_and_the_experts_at_their_steps`
            // catches before any release.
            BUILTIN_ROLE_FILES
                .iter()
                .map(|source| parse_role_file(source).expect("assets/roles/*.md are valid"))
                .collect()
        })
        .clone()
}

/// True when `id` is the id of a built-in role.
pub fn is_builtin_step_role(id: &str) -> bool {
    builtin_step_roles().iter().any(|role| role.id == id)
}

/// The full catalog for a schema: the built-in roles and the schema's
/// artifact roles, ordered by first step, then by catalog or schema order.
pub fn catalog(schema: &Schema, schema_name: &str) -> Vec<Role> {
    let origin = format!("schema:{schema_name}");
    let mut roles = builtin_step_roles();
    for artifact in &schema.artifacts {
        if let Some(role) = &artifact.role {
            roles.push(Role {
                id: role.id.clone(),
                names: role.name.clone(),
                kind: RoleKind::Artifact(artifact.id.clone()),
                steps: vec![ARTIFACT_STEP.to_string()],
                question: role.question.clone(),
                criteria: criteria(&role.done_when, &origin),
                disableable: false,
                domains: Vec::new(),
                knowledge: None,
                triggers: Vec::new(),
                model: None,
            });
        }
    }
    // A stable sort keeps the catalog order within a step.
    roles.sort_by_key(|role| step_rank(role.first_step()));
    roles
}

fn step_rank(step: &str) -> usize {
    STEPS.iter().position(|s| *s == step).unwrap_or(STEPS.len())
}

fn criteria(texts: &[String], origin: &str) -> Vec<Criterion> {
    texts
        .iter()
        .map(|text| text.trim())
        .filter(|text| !text.is_empty())
        .map(|text| Criterion {
            origin: origin.to_string(),
            text: text.to_string(),
        })
        .collect()
}

/// Whether a role runs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Activation {
    Always,
    Never,
    /// Active when the change's proposal ticks one of the role's domains.
    OnDomain,
}

impl Activation {
    pub fn parse(text: &str) -> Option<Self> {
        match text {
            "always" => Some(Self::Always),
            "never" => Some(Self::Never),
            "on_domain" => Some(Self::OnDomain),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Always => "always",
            Self::Never => "never",
            Self::OnDomain => "on_domain",
        }
    }
}

/// The model a role's sub-agent runs on: the aliases Claude Code's
/// sub-agent tool accepts. A full model id would tie the configuration to a
/// release.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Model {
    Opus,
    Sonnet,
    Haiku,
}

impl Model {
    /// The values a role's frontmatter accepts, for the messages.
    pub const ALLOWED: &'static str = "`opus`, `sonnet` or `haiku`";

    pub fn parse(text: &str) -> Option<Self> {
        match text {
            "opus" => Some(Self::Opus),
            "sonnet" => Some(Self::Sonnet),
            "haiku" => Some(Self::Haiku),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Opus => "opus",
            Self::Sonnet => "sonnet",
            Self::Haiku => "haiku",
        }
    }
}

/// What a project's `model:` asks for. Only a project may write `inherit`,
/// to go back to the session's model.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ProjectModel {
    /// `inherit`: the session's model.
    Inherit,
    /// `opus`, `sonnet` or `haiku`.
    Use(Model),
}

impl ProjectModel {
    /// The values a project's `model:` accepts, for the messages.
    const ALLOWED: &'static str = "`opus`, `sonnet`, `haiku` or `inherit`";

    /// `None` when the text is not one of [`Self::ALLOWED`].
    fn parse(text: &str) -> Option<Self> {
        if text == INHERIT_MODEL {
            return Some(Self::Inherit);
        }
        Model::parse(text).map(Self::Use)
    }

    /// The role's model once this setting applies: none for `inherit`.
    fn model(self) -> Option<Model> {
        match self {
            Self::Inherit => None,
            Self::Use(model) => Some(model),
        }
    }
}

/// Who set a role's model. A change sets none.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelOrigin {
    Default,
    Project,
}

impl ModelOrigin {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Project => "project",
        }
    }
}

/// Who set a role's activation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhenOrigin {
    Default,
    Project,
    Change,
}

impl WhenOrigin {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Project => "project",
            Self::Change => "change",
        }
    }
}

/// Why an active role is active.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActivatedBy {
    /// The role's default activation, `always`.
    Always,
    /// `always`, set in `_codev/config.yaml`.
    Project,
    /// `always`, set in the change's `change.yaml`.
    Change,
    /// `on_domain`, and the proposal ticks this domain.
    Domain(String),
}

impl ActivatedBy {
    /// `always`, `project`, `change` or `domain:<name>`.
    pub fn label(&self) -> String {
        match self {
            Self::Always => "always".to_string(),
            Self::Project => "project".to_string(),
            Self::Change => "change".to_string(),
            Self::Domain(domain) => format!("domain:{domain}"),
        }
    }
}

/// A role entry of `roles:`, as written: `skeptic: never`, or a mapping
/// with `when`, `done_when` and `model`.
///
/// The activation and the model stay text: an unexpected value is reported
/// by [`check_settings`] with `invalid_role_activation` or
/// `invalid_role_model`, naming the accepted values, rather than as a YAML
/// type error.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RoleSetting {
    pub when: Option<String>,
    pub done_when: Vec<String>,
    /// `opus`, `sonnet`, `haiku` or `inherit` when valid.
    pub model: Option<String>,
}

/// The mapping form of a role entry. Strict: a misspelled `done-when`
/// would otherwise drop criteria without a word.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RoleSettingMapping {
    #[serde(default)]
    when: Option<Value>,
    #[serde(default)]
    done_when: Vec<String>,
    #[serde(default)]
    model: Option<Value>,
}

impl<'de> Deserialize<'de> for RoleSetting {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = Value::deserialize(deserializer)?;
        if value.is_mapping() {
            let mapping: RoleSettingMapping =
                serde_norway::from_value(value).map_err(D::Error::custom)?;
            return Ok(Self {
                when: mapping.when.as_ref().map(scalar_text),
                done_when: mapping.done_when,
                model: mapping.model.as_ref().map(scalar_text),
            });
        }
        Ok(Self {
            when: Some(scalar_text(&value)),
            done_when: Vec::new(),
            model: None,
        })
    }
}

/// The activation of a role in a change's `change.yaml`: a scalar only.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoleActivation(pub String);

impl<'de> Deserialize<'de> for RoleActivation {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = Value::deserialize(deserializer)?;
        if value.is_mapping() || value.is_sequence() {
            return Err(D::Error::custom(
                "a role of change.yaml takes `always` or `never`",
            ));
        }
        Ok(Self(scalar_text(&value)))
    }
}

impl Serialize for RoleActivation {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.0)
    }
}

/// A YAML scalar as the text the user wrote: `false` stays `false`, so
/// that the message quotes it back.
fn scalar_text(value: &Value) -> String {
    match value {
        Value::String(text) => text.clone(),
        Value::Bool(flag) => flag.to_string(),
        Value::Number(number) => number.to_string(),
        Value::Null => String::new(),
        other => serde_norway::to_string(other)
            .unwrap_or_default()
            .trim()
            .to_string(),
    }
}

/// An invalid role setting.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoleFinding {
    /// `unknown_role`, `invalid_role_activation`, `role_not_disableable`
    /// or `invalid_role_model`; `role_model_unused` from
    /// [`unused_models`].
    pub code: &'static str,
    /// The role id, as written.
    pub role: String,
    pub message: String,
}

/// Checks settings against a catalog, in id order. `file` names the file
/// in the messages.
pub fn check_settings(
    catalog: &[Role],
    settings: &BTreeMap<String, RoleSetting>,
    file: &str,
) -> Vec<RoleFinding> {
    let mut findings = Vec::new();
    for (id, setting) in settings {
        let finding = |code, message: String| RoleFinding {
            code,
            role: id.clone(),
            message,
        };
        let Some(role) = catalog.iter().find(|role| &role.id == id) else {
            let known: Vec<&str> = catalog.iter().map(|role| role.id.as_str()).collect();
            findings.push(finding(
                UNKNOWN_ROLE,
                format!(
                    "{file}: `{id}` is not a role; the roles are {}",
                    known.join(", ")
                ),
            ));
            continue;
        };
        if let Some(model) = &setting.model
            && ProjectModel::parse(model).is_none()
        {
            findings.push(finding(
                INVALID_ROLE_MODEL,
                format!(
                    "{file}: role `{id}` has the model `{model}`; use {}",
                    ProjectModel::ALLOWED
                ),
            ));
        }
        let Some(when) = &setting.when else { continue };
        match Activation::parse(when) {
            None => findings.push(finding(
                INVALID_ROLE_ACTIVATION,
                format!(
                    "{file}: role `{id}` has the activation `{when}`; use `always`, `never` or `on_domain`"
                ),
            )),
            Some(Activation::OnDomain) if role.domains.is_empty() => findings.push(finding(
                INVALID_ROLE_ACTIVATION,
                format!(
                    "{file}: role `{id}` is bound to no domain, so `on_domain` would never activate it; use `always` or `never`"
                ),
            )),
            Some(Activation::Never) if !role.disableable => findings.push(finding(
                ROLE_NOT_DISABLEABLE,
                format!(
                    "{file}: role `{id}` cannot be disabled; remove `never`, its criteria can still be extended with `done_when`"
                ),
            )),
            Some(_) => {}
        }
    }
    findings
}

/// Whether the project's settings give the developer a model — `opus`,
/// `sonnet` or `haiku`, not `inherit` — so that the `apply` skill hands
/// the tasks to a sub-agent on it.
pub fn developer_has_model(settings: &BTreeMap<String, RoleSetting>) -> bool {
    settings
        .get(DEVELOPER)
        .and_then(|setting| setting.model.as_deref())
        .and_then(ProjectModel::parse)
        .and_then(ProjectModel::model)
        .is_some()
}

/// The warnings of settings against a catalog, in id order: a valid model
/// on a role that serves no reviewing step and is not the developer, which
/// never applies.
pub fn unused_models(
    catalog: &[Role],
    settings: &BTreeMap<String, RoleSetting>,
    file: &str,
) -> Vec<RoleFinding> {
    settings
        .iter()
        .filter(|(_, setting)| {
            setting
                .model
                .as_deref()
                .is_some_and(|model| ProjectModel::parse(model).is_some())
        })
        .filter_map(|(id, _)| {
            let role = catalog.iter().find(|role| &role.id == id)?;
            (!role.reviews() && role.id != DEVELOPER).then(|| RoleFinding {
                code: ROLE_MODEL_UNUSED,
                role: id.clone(),
                message: format!(
                    "{file}: role `{id}` never runs as its own sub-agent (it serves no reviewing step: {}, and only the `{DEVELOPER}` implements in one), so its `model` never applies; remove it",
                    REVIEWING_STEPS.join(", ")
                ),
            })
        })
        .collect()
}

/// A role with its settings applied.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedRole {
    pub role: Role,
    pub when: Activation,
    pub when_origin: WhenOrigin,
    /// Why the role is active; `None` when it is not.
    pub activated_by: Option<ActivatedBy>,
    /// The model the role's sub-agent runs on; `None` for the session's.
    pub model: Option<Model>,
    pub model_origin: ModelOrigin,
}

impl ResolvedRole {
    pub fn is_active(&self) -> bool {
        self.activated_by.is_some()
    }
}

/// The settings that apply to a resolution, and the files they come from
/// (for the messages).
#[derive(Debug, Clone, Copy)]
pub struct Settings<'a> {
    pub project: &'a BTreeMap<String, RoleSetting>,
    pub project_file: &'a str,
    pub change: &'a BTreeMap<String, RoleSetting>,
    pub change_file: &'a str,
    /// The domains the change's proposal ticks; `None` without a change,
    /// or when its proposal lists none: `on_domain` roles stay inactive.
    pub domains: Option<&'a [String]>,
}

/// Applies the role's default, then the project's settings, then the
/// change's, to the catalog.
///
/// Fails with the first invalid setting, the project's before the
/// change's: a command cannot serve roles it cannot trust.
pub fn resolve(
    catalog: Vec<Role>,
    settings: Settings<'_>,
) -> Result<Vec<ResolvedRole>, RoleFinding> {
    let mut findings = check_settings(&catalog, settings.project, settings.project_file);
    findings.extend(check_settings(
        &catalog,
        settings.change,
        settings.change_file,
    ));
    if let Some(first) = findings.into_iter().next() {
        return Err(first);
    }

    Ok(catalog
        .into_iter()
        .map(|mut role| {
            let mut when = role.default_activation();
            let mut when_origin = WhenOrigin::Default;
            let mut model = role.model;
            let mut model_origin = ModelOrigin::Default;
            if let Some(project) = settings.project.get(&role.id) {
                if let Some(chosen) = project.model.as_deref().and_then(ProjectModel::parse) {
                    model = chosen.model();
                    model_origin = ModelOrigin::Project;
                }
                role.criteria
                    .extend(criteria(&project.done_when, PROJECT_ORIGIN));
                if let Some(activation) = project.when.as_deref().and_then(Activation::parse) {
                    when = activation;
                    when_origin = WhenOrigin::Project;
                }
            }
            if let Some(activation) = settings
                .change
                .get(&role.id)
                .and_then(|change| change.when.as_deref())
                .and_then(Activation::parse)
            {
                when = activation;
                when_origin = WhenOrigin::Change;
            }
            let activated_by = activated_by(&role, when, when_origin, settings.domains);
            ResolvedRole {
                role,
                when,
                when_origin,
                activated_by,
                model,
                model_origin,
            }
        })
        .collect())
}

/// Why a role with this activation is active, if it is.
fn activated_by(
    role: &Role,
    when: Activation,
    origin: WhenOrigin,
    domains: Option<&[String]>,
) -> Option<ActivatedBy> {
    match when {
        Activation::Never => None,
        Activation::Always => Some(match origin {
            WhenOrigin::Default => ActivatedBy::Always,
            WhenOrigin::Project => ActivatedBy::Project,
            WhenOrigin::Change => ActivatedBy::Change,
        }),
        Activation::OnDomain => {
            let ticked = domains.unwrap_or_default();
            role.domains
                .iter()
                .find(|domain| ticked.contains(domain))
                .map(|domain| ActivatedBy::Domain(domain.clone()))
        }
    }
}

/// The heading of the knowledge section [`filter_by_stack`] filters.
const BY_STACK: &str = "## By stack";

/// `knowledge` with its `By stack` section keeping only the items for
/// `languages` (as [`crate::detect::stack::LANGUAGES`] names them).
///
/// An item is a `- **Label**: …` bullet and runs to the next one. It is
/// kept when its label names a language of `languages` — `TypeScript/
/// JavaScript` and `Rust, Python, Go, Java` name each of theirs — or names
/// no language codev detects. The section is left out when no item
/// remains. Empty `languages` keep everything: nothing was detected.
pub fn filter_by_stack(knowledge: &str, languages: &[&str]) -> String {
    if languages.is_empty() {
        return knowledge.to_string();
    }
    let lines: Vec<&str> = knowledge.lines().collect();
    let Some(start) = lines.iter().position(|line| line.trim_end() == BY_STACK) else {
        return knowledge.to_string();
    };
    let end = lines[start + 1..]
        .iter()
        .position(|line| line.starts_with("## "))
        .map_or(lines.len(), |offset| start + 1 + offset);

    // The lines before the first item (a blank line, an introduction), then
    // the items, each with its continuation lines.
    let body = &lines[start + 1..end];
    let first_item = body
        .iter()
        .position(|line| line.starts_with("- **"))
        .unwrap_or(body.len());
    let mut items: Vec<Vec<&str>> = Vec::new();
    for line in &body[first_item..] {
        if line.starts_with("- **") || items.is_empty() {
            items.push(vec![line]);
        } else if let Some(item) = items.last_mut() {
            item.push(line);
        }
    }
    let kept: Vec<Vec<&str>> = items
        .into_iter()
        .filter(|item| item_is_for(item[0], languages))
        .map(|mut item| {
            while item.last().is_some_and(|line| line.trim().is_empty()) {
                item.pop();
            }
            item
        })
        .collect();

    let mut out: Vec<&str> = lines[..start].to_vec();
    if !kept.is_empty() {
        out.push(lines[start]);
        out.extend(&body[..first_item]);
        out.extend(kept.iter().flatten());
    }
    // One blank line before the next section, whatever was kept.
    while out.last().is_some_and(|line| line.trim().is_empty()) {
        out.pop();
    }
    if end < lines.len() {
        out.push("");
    }
    out.extend(&lines[end..]);
    out.join("\n").trim_end().to_string()
}

/// True when the bullet's bold label names one of `languages`, or names
/// no language codev detects.
fn item_is_for(bullet: &str, languages: &[&str]) -> bool {
    let label = bullet
        .strip_prefix("- **")
        .and_then(|rest| rest.split("**").next())
        .unwrap_or("");
    let named: Vec<&str> = label
        .split(['/', ','])
        .map(str::trim)
        .filter_map(|name| {
            crate::detect::stack::LANGUAGES
                .iter()
                .find(|known| known.eq_ignore_ascii_case(name))
                .copied()
        })
        .collect();
    named.is_empty() || named.iter().any(|name| languages.contains(name))
}

/// The roles active at `step`, in catalog order.
pub fn active_at<'a>(roles: &'a [ResolvedRole], step: &str) -> Vec<&'a ResolvedRole> {
    roles
        .iter()
        .filter(|resolved| resolved.role.serves(step) && resolved.is_active())
        .collect()
}

/// The roles serving `step` that the configuration turned off: whose
/// activation, once the project's then the change's settings are applied,
/// is `never`. An `on_domain` role whose domain is not ticked is inactive,
/// not disabled: the review record expects nothing of it.
pub fn disabled_at<'a>(roles: &'a [ResolvedRole], step: &str) -> Vec<&'a ResolvedRole> {
    roles
        .iter()
        .filter(|resolved| resolved.role.serves(step) && resolved.when == Activation::Never)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SCHEMA_WITH_ROLES: &str = r#"
name: demo
artifacts:
  - id: proposal
    generates: proposal.md
    role:
      id: product
      name: { en: Product owner, fr: Product owner }
      question: Why this change?
      done_when: [Why is stated]
  - id: tasks
    generates: tasks.md
    requires: [proposal]
    role:
      id: lead
      name: { en: Tech lead }
      question: In what order?
      done_when: [Each task is verified]
apply:
  requires: [tasks]
  tracks: tasks.md
"#;

    const SCHEMA_WITHOUT_ROLES: &str = r#"
name: bare
artifacts:
  - id: brief
    generates: brief.md
apply:
  requires: [brief]
  tracks: brief.md
"#;

    fn demo_catalog() -> Vec<Role> {
        catalog(&Schema::parse(SCHEMA_WITH_ROLES).unwrap(), "demo")
    }

    fn ids(roles: &[Role]) -> Vec<&str> {
        roles.iter().map(|role| role.id.as_str()).collect()
    }

    fn settings(yaml: &str) -> BTreeMap<String, RoleSetting> {
        serde_norway::from_str(yaml).unwrap()
    }

    fn resolve_with(
        project: &BTreeMap<String, RoleSetting>,
        change: &BTreeMap<String, RoleSetting>,
    ) -> Result<Vec<ResolvedRole>, RoleFinding> {
        resolve_for_domains(project, change, None)
    }

    fn resolve_for_domains(
        project: &BTreeMap<String, RoleSetting>,
        change: &BTreeMap<String, RoleSetting>,
        domains: Option<&[String]>,
    ) -> Result<Vec<ResolvedRole>, RoleFinding> {
        resolve(
            demo_catalog(),
            Settings {
                project,
                project_file: "_codev/config.yaml",
                change,
                change_file: "change.yaml",
                domains,
            },
        )
    }

    fn active_ids(roles: &[ResolvedRole], step: &str) -> Vec<String> {
        active_at(roles, step)
            .iter()
            .map(|r| r.role.id.clone())
            .collect()
    }

    fn ticked(domains: &[&str]) -> Vec<String> {
        domains.iter().map(|d| d.to_string()).collect()
    }

    const EXPERTS: [(&str, &str); 6] = [
        ("security", "security"),
        ("privacy", "personal-data"),
        ("accessibility", "user-interface"),
        ("performance", "performance"),
        ("ops", "operations"),
        ("migration", "migration"),
    ];

    fn role<'a>(roles: &'a [ResolvedRole], id: &str) -> &'a ResolvedRole {
        roles.iter().find(|r| r.role.id == id).unwrap()
    }

    #[test]
    fn the_builtin_catalog_has_the_step_roles_and_the_experts_at_their_steps() {
        let roles = builtin_step_roles();
        let pairs: Vec<(&str, String)> = roles
            .iter()
            .map(|role| (role.id.as_str(), role.steps.join(" ")))
            .collect();
        let expert = "plan.challenge code.review".to_string();
        assert_eq!(
            pairs,
            [
                ("analyst", "plan.understand".to_string()),
                ("investigator", "plan.investigate".to_string()),
                ("tracer", "plan.trace".to_string()),
                ("skeptic", "plan.challenge".to_string()),
                ("security", expert.clone()),
                ("privacy", expert.clone()),
                ("accessibility", expert.clone()),
                ("performance", expert.clone()),
                ("ops", expert.clone()),
                ("migration", expert),
                ("developer", "code.implement".to_string()),
                ("newcomer", "code.review".to_string()),
            ]
        );
        for (id, domain) in EXPERTS {
            let role = roles.iter().find(|role| role.id == id).unwrap();
            assert_eq!(role.domains, [domain], "{id}");
            assert!(crate::domains::DOMAINS.contains(&domain));
        }
        for role in &roles {
            assert!(
                !role.steps.is_empty()
                    && role.steps.iter().all(|step| STEPS.contains(&step.as_str())),
                "{}",
                role.id
            );
            assert!(role.names.contains_key("en") && role.names.contains_key("fr"));
            assert!(!role.question.is_empty() && !role.criteria.is_empty());
            assert!(role.criteria.iter().all(|c| c.origin == "built-in"));
        }
        let required: Vec<&str> = roles
            .iter()
            .filter(|role| !role.disableable)
            .map(|role| role.id.as_str())
            .collect();
        assert_eq!(required, ["analyst", "developer"]);
    }

    #[test]
    fn default_catalog_lists_sixteen_roles_in_workflow_order() {
        let yaml = include_str!("../../../assets/schemas/spec-driven/schema.yaml");
        let roles = catalog(&Schema::parse(yaml).unwrap(), "spec-driven");
        assert_eq!(
            ids(&roles),
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
                "newcomer",
            ]
        );
        let artifacts: Vec<&RoleKind> = roles[2..6].iter().map(|role| &role.kind).collect();
        assert_eq!(
            artifacts,
            ["proposal", "specs", "design", "tasks"]
                .map(|a| RoleKind::Artifact(a.to_string()))
                .iter()
                .collect::<Vec<_>>()
        );
        assert!(roles[2..6].iter().all(|role| {
            role.steps == ["plan.write"]
                && !role.disableable
                && role
                    .criteria
                    .iter()
                    .all(|c| c.origin == "schema:spec-driven")
        }));
    }

    #[test]
    fn a_schema_without_roles_adds_only_built_in_roles() {
        let roles = catalog(&Schema::parse(SCHEMA_WITHOUT_ROLES).unwrap(), "bare");
        assert_eq!(roles.len(), 12);
        assert!(roles.iter().all(|role| role.kind == RoleKind::Step));
    }

    #[test]
    fn every_role_but_the_experts_is_active_by_default() {
        let roles = resolve_with(&BTreeMap::new(), &BTreeMap::new()).unwrap();
        for r in &roles {
            assert_eq!(r.when_origin, WhenOrigin::Default);
            if r.role.domains.is_empty() {
                assert_eq!(r.when, Activation::Always, "{}", r.role.id);
                assert_eq!(r.activated_by, Some(ActivatedBy::Always));
            } else {
                assert_eq!(r.when, Activation::OnDomain, "{}", r.role.id);
                assert_eq!(r.activated_by, None, "{}", r.role.id);
            }
        }
    }

    #[test]
    fn experts_are_active_only_for_their_ticked_domain() {
        let domains = ticked(&["security", "personal-data"]);
        let roles =
            resolve_for_domains(&BTreeMap::new(), &BTreeMap::new(), Some(&domains)).unwrap();
        assert_eq!(
            active_ids(&roles, "plan.challenge"),
            ["skeptic", "security", "privacy"]
        );
        assert_eq!(
            active_ids(&roles, "code.review"),
            ["security", "privacy", "newcomer"]
        );
        assert_eq!(
            role(&roles, "privacy").activated_by,
            Some(ActivatedBy::Domain("personal-data".into()))
        );
        assert_eq!(
            role(&roles, "privacy")
                .activated_by
                .as_ref()
                .unwrap()
                .label(),
            "domain:personal-data"
        );
    }

    #[test]
    fn on_domain_roles_are_inactive_without_domains() {
        for domains in [None, Some(Vec::new())] {
            let roles = resolve_for_domains(&BTreeMap::new(), &BTreeMap::new(), domains.as_deref())
                .unwrap();
            assert_eq!(active_ids(&roles, "plan.challenge"), ["skeptic"]);
            assert_eq!(active_ids(&roles, "code.review"), ["newcomer"]);
        }
    }

    #[test]
    fn a_change_forces_an_expert() {
        let roles = resolve_for_domains(
            &BTreeMap::new(),
            &settings("performance: always\n"),
            Some(&[]),
        )
        .unwrap();
        let performance = role(&roles, "performance");
        assert!(performance.is_active());
        assert_eq!(performance.activated_by, Some(ActivatedBy::Change));
    }

    #[test]
    fn a_project_turns_an_expert_off() {
        let domains = ticked(&["user-interface"]);
        let roles = resolve_for_domains(
            &settings("accessibility: never\n"),
            &BTreeMap::new(),
            Some(&domains),
        )
        .unwrap();
        assert!(!active_ids(&roles, "code.review").contains(&"accessibility".to_string()));
    }

    #[test]
    fn activated_by_names_the_origin_of_an_always() {
        let roles =
            resolve_with(&settings("security: always\n"), &settings("ops: always\n")).unwrap();
        assert_eq!(
            role(&roles, "security").activated_by,
            Some(ActivatedBy::Project)
        );
        assert_eq!(role(&roles, "ops").activated_by, Some(ActivatedBy::Change));
        assert_eq!(
            role(&roles, "skeptic")
                .activated_by
                .as_ref()
                .unwrap()
                .label(),
            "always"
        );
    }

    #[test]
    fn on_domain_on_a_role_without_domains_is_reported() {
        let found = check_settings(
            &demo_catalog(),
            &settings("skeptic: on_domain\nsecurity: on_domain\n"),
            "_codev/config.yaml",
        );
        assert_eq!(found.len(), 1, "{found:?}");
        assert_eq!(found[0].code, "invalid_role_activation");
        assert_eq!(found[0].role, "skeptic");
        assert!(
            found[0].message.contains("no domain"),
            "{}",
            found[0].message
        );
    }

    #[test]
    fn project_criteria_follow_the_role_own() {
        let project = settings(
            "skeptic:\n  when: always\n  done_when: [The rollback plan is stated, '  ']\n",
        );
        let roles = resolve_with(&project, &BTreeMap::new()).unwrap();
        let skeptic = role(&roles, "skeptic");
        let last = skeptic.role.criteria.last().unwrap();
        assert_eq!(last.text, "The rollback plan is stated");
        assert_eq!(last.origin, "project");
        let builtin = skeptic.role.criteria.len() - 1;
        assert!(
            skeptic.role.criteria[..builtin]
                .iter()
                .all(|c| c.origin == "built-in")
        );
        assert_eq!(skeptic.when_origin, WhenOrigin::Project);
    }

    #[test]
    fn a_role_disabled_by_the_project_is_not_active_at_its_step() {
        let roles = resolve_with(&settings("newcomer: never\n"), &BTreeMap::new()).unwrap();
        assert!(active_at(&roles, "code.review").is_empty());
        assert_eq!(role(&roles, "newcomer").when_origin, WhenOrigin::Project);
    }

    #[test]
    fn a_change_overrides_the_project_activation() {
        let roles = resolve_with(
            &settings("newcomer: never\n"),
            &settings("newcomer: always\n"),
        )
        .unwrap();
        let newcomer = active_at(&roles, "code.review");
        assert_eq!(newcomer.len(), 1);
        assert_eq!(newcomer[0].when_origin, WhenOrigin::Change);
    }

    #[test]
    fn an_unknown_role_id_is_reported() {
        let err = resolve_with(&settings("reviewer: never\n"), &BTreeMap::new()).unwrap_err();
        assert_eq!(err.code, "unknown_role");
        assert_eq!(err.role, "reviewer");
        assert!(
            err.message.contains("_codev/config.yaml"),
            "{}",
            err.message
        );
    }

    #[test]
    fn an_activation_other_than_always_never_or_on_domain_is_reported() {
        for yaml in [
            "skeptic: sometimes\n",
            "skeptic: false\n",
            "skeptic: { when: off }\n",
        ] {
            let found = check_settings(&demo_catalog(), &settings(yaml), "_codev/config.yaml");
            assert_eq!(found.len(), 1, "{yaml}");
            assert_eq!(found[0].code, "invalid_role_activation");
            assert!(
                found[0]
                    .message
                    .contains("`always`, `never` or `on_domain`"),
                "{}",
                found[0].message
            );
        }
    }

    #[test]
    fn artifact_roles_the_analyst_and_the_developer_cannot_be_disabled() {
        for id in ["product", "lead", "analyst", "developer"] {
            let found = check_settings(
                &demo_catalog(),
                &settings(&format!("{id}: never\n")),
                "change.yaml",
            );
            assert_eq!(found[0].code, "role_not_disableable", "{id}");
            assert!(found[0].message.contains("change.yaml"));
        }
        // Their criteria can still be extended.
        let extended = settings("product:\n  done_when: [The ticket is cited]\n");
        let roles = resolve_with(&extended, &BTreeMap::new()).unwrap();
        assert_eq!(
            role(&roles, "product").role.criteria.last().unwrap().text,
            "The ticket is cited"
        );
    }

    #[test]
    fn the_project_finding_comes_before_the_change_one() {
        let err = resolve_with(
            &settings("skeptic: sometimes\n"),
            &settings("ghost: always\n"),
        )
        .unwrap_err();
        assert_eq!(err.code, "invalid_role_activation");
    }

    #[test]
    fn an_unknown_key_inside_a_role_is_refused() {
        let err =
            serde_norway::from_str::<BTreeMap<String, RoleSetting>>("skeptic:\n  done-when: [x]\n")
                .unwrap_err();
        assert!(err.to_string().contains("done-when"), "{err}");
    }

    #[test]
    fn no_built_in_role_carries_a_model() {
        // Measured on the isolated path, no role kept its quality on Sonnet
        // for a saving worth it — see "The reviewers' models" in
        // docs/en/src/roles.md: every role runs on the session's model
        // unless the project says otherwise.
        let roles = resolve_with(&BTreeMap::new(), &BTreeMap::new()).unwrap();
        for r in &roles {
            assert_eq!(r.model, None, "{}", r.role.id);
            assert_eq!(r.model_origin, ModelOrigin::Default, "{}", r.role.id);
        }
    }

    #[test]
    fn a_project_sets_a_role_s_model() {
        let project = settings(
            "privacy: { model: sonnet }\nskeptic: { model: inherit }\nops: { when: always, model: haiku }\n",
        );
        let roles = resolve_with(&project, &BTreeMap::new()).unwrap();
        let privacy = role(&roles, "privacy");
        assert_eq!(privacy.model, Some(Model::Sonnet));
        assert_eq!(privacy.model_origin, ModelOrigin::Project);
        assert_eq!(
            privacy.when_origin,
            WhenOrigin::Default,
            "only the model is set"
        );
        let skeptic = role(&roles, "skeptic");
        assert_eq!(skeptic.model, None);
        assert_eq!(skeptic.model_origin, ModelOrigin::Project);
        assert_eq!(role(&roles, "ops").model, Some(Model::Haiku));
        assert_eq!(role(&roles, "tracer").model_origin, ModelOrigin::Default);
    }

    #[test]
    fn an_unknown_model_is_invalid() {
        for yaml in ["skeptic: { model: gpt }\n", "skeptic: { model: Sonnet }\n"] {
            let found = check_settings(&demo_catalog(), &settings(yaml), "_codev/config.yaml");
            assert_eq!(found.len(), 1, "{yaml}");
            assert_eq!(found[0].code, "invalid_role_model");
            assert_eq!(found[0].role, "skeptic");
            assert!(
                found[0]
                    .message
                    .contains("`opus`, `sonnet`, `haiku` or `inherit`"),
                "{}",
                found[0].message
            );
            assert!(found[0].message.contains("_codev/config.yaml"));
        }
        let err =
            resolve_with(&settings("skeptic: { model: gpt }\n"), &BTreeMap::new()).unwrap_err();
        assert_eq!(err.code, "invalid_role_model");
    }

    #[test]
    fn a_model_off_the_review_steps_is_a_warning() {
        let project = settings(
            "developer: { model: sonnet }\nproduct: { model: opus }\nprivacy: { model: sonnet }\nanalyst: { model: gpt }\n",
        );
        let found = unused_models(&demo_catalog(), &project, "_codev/config.yaml");
        let roles: Vec<(&str, &str)> = found.iter().map(|f| (f.code, f.role.as_str())).collect();
        assert_eq!(
            roles,
            [("role_model_unused", "product")],
            "an invalid model is an error, not also a warning; the developer's model is used"
        );
        assert!(found[0].message.contains("product"), "{}", found[0].message);
        // A warning never keeps the roles from resolving.
        let valid = settings("developer: { model: sonnet }\n");
        assert!(check_settings(&demo_catalog(), &valid, "_codev/config.yaml").is_empty());
        assert!(resolve_with(&valid, &BTreeMap::new()).is_ok());
    }

    #[test]
    fn a_change_activation_is_a_scalar() {
        let ok: BTreeMap<String, RoleActivation> =
            serde_norway::from_str("newcomer: always\nskeptic: false\n").unwrap();
        assert_eq!(ok["skeptic"], RoleActivation("false".into()));
        assert!(
            serde_norway::from_str::<BTreeMap<String, RoleActivation>>(
                "newcomer: { when: always }\n"
            )
            .is_err()
        );
    }

    #[test]
    fn names_follow_the_language_then_english() {
        let skeptic = builtin_step_roles()
            .into_iter()
            .find(|role| role.id == "skeptic")
            .unwrap();
        assert_eq!(skeptic.name("fr"), "Contradicteur");
        assert_eq!(skeptic.name("en"), "Skeptic");
        assert_eq!(skeptic.name("de"), "Skeptic");
    }

    fn criteria_text(id: &str) -> String {
        let role = builtin_step_roles()
            .into_iter()
            .find(|role| role.id == id)
            .unwrap();
        let texts: Vec<String> = role.criteria.into_iter().map(|c| c.text).collect();
        texts.join(" ")
    }

    #[test]
    fn step_role_criteria_carry_the_review_grids() {
        // The grids the skills used to restate now live in the catalog:
        // dropping one of them must fail here.
        for (id, needles) in [
            (
                "analyst",
                &[
                    "materially change the scope",
                    "acceptance criteria",
                    "reasonable assumption",
                ][..],
            ),
            (
                "investigator",
                &[
                    "nearby tests",
                    "Stay read-only",
                    "proportionate to the change",
                    "Report contradictions with existing specs",
                    "\"explore the code\"",
                ][..],
            ),
            (
                "tracer",
                &[
                    "has its spec file",
                    "edge-case scenario",
                    "names an automated test",
                    "why automation is impossible",
                    "updates the documentation",
                    "none is affected",
                    "release note agrees with the plan",
                    "**BREAKING** item of What Changes",
                    "REMOVED",
                    "says `None`",
                    "verification is concrete",
                ][..],
            ),
            (
                "skeptic",
                &[
                    "**Need**",
                    "**Scope**",
                    "**Specs**",
                    "**Decisions**",
                    "**Assumptions**",
                    "**Tasks**",
                    "**Domains**",
                    "is the expert's to review",
                    "`rules`",
                ][..],
            ),
            (
                "developer",
                &[
                    "existing test suite",
                    "conventions of its language",
                    "the manifest or the CI configuration",
                    "never guess one",
                    "ask the user",
                    "nothing more",
                    "acceptance criterion",
                ][..],
            ),
            (
                "newcomer",
                &[
                    "**Without context**",
                    "**Names**",
                    "**Size and nesting**",
                    "**Cleverness**",
                    "**Comments**",
                    "**Tests** read as a description of the behavior",
                    "The project's `rules`",
                ][..],
            ),
        ] {
            let text = criteria_text(id);
            for needle in needles {
                assert!(text.contains(needle), "`{id}` must carry `{needle}`");
            }
        }
    }

    fn builtin(id: &str) -> Role {
        builtin_step_roles()
            .into_iter()
            .find(|role| role.id == id)
            .unwrap()
    }

    /// The body of a knowledge section, up to the next `## ` heading.
    fn section_of<'a>(knowledge: &'a str, title: &str) -> &'a str {
        let heading = format!("## {title}\n");
        let start = knowledge
            .find(&heading)
            .unwrap_or_else(|| panic!("missing section `{title}`"))
            + heading.len();
        let rest = &knowledge[start..];
        &rest[..rest.find("\n## ").unwrap_or(rest.len())]
    }

    #[test]
    fn every_expert_says_what_it_does_not_report() {
        // Each expert's Reporting names what a confident finding must show.
        for (id, shows) in [
            ("security", "a finding names who sends what"),
            ("privacy", "Name the data and\nwhere it ends up"),
            ("accessibility", "State the step a user cannot take"),
            (
                "performance",
                "A cost you cannot state as a function of growth",
            ),
            ("ops", "Name the failure"),
            ("migration", "Name the\nexisting thing that breaks"),
        ] {
            let knowledge = builtin(id).knowledge.unwrap();
            assert!(
                section_of(&knowledge, "Scope").contains("Do not report:"),
                "`{id}`: Scope lists what it does not report"
            );
            assert!(
                section_of(&knowledge, "Reporting").contains(shows),
                "`{id}`: Reporting says what a finding shows: `{shows}`"
            );
        }
    }

    #[test]
    fn the_skeptic_leaves_domains_to_the_experts() {
        let criteria = criteria_text("skeptic");
        assert!(criteria.contains("**Domains**"), "{criteria}");
        assert!(!criteria.contains("lenses"), "{criteria}");
        let knowledge = builtin("skeptic").knowledge.unwrap();
        let scope = section_of(&knowledge, "Scope");
        for domain in [
            "security",
            "privacy",
            "migration",
            "operations",
            "performance",
            "accessibility",
        ] {
            assert!(
                scope.contains(domain),
                "the skeptic's Scope names `{domain}`"
            );
        }
        assert!(scope.contains("`proposal.md#Domains`"));
        assert!(scope.contains("irreversible step"));
        assert!(!knowledge.contains("Hidden migration"));
        assert!(section_of(&knowledge, "Signals").contains("**Domain left to nobody**"));
        assert!(section_of(&knowledge, "Reporting").contains("confidence `low`"));
    }

    #[test]
    fn the_newcomer_groups_findings_by_function() {
        let knowledge = builtin("newcomer").knowledge.unwrap();
        let scope = section_of(&knowledge, "Scope");
        assert!(scope.contains("missing tests"), "{scope}");
        assert!(scope.contains("Do not report:"), "{scope}");
        let reporting = section_of(&knowledge, "Reporting");
        assert!(
            reporting.contains("One finding per function"),
            "{reporting}"
        );
    }

    #[test]
    fn a_role_file_splits_frontmatter_and_knowledge() {
        let role = parse_role_file(
            "---\r\nid: demo\r\nname: { en: Demo }\r\nat: plan.trace\r\nquestion: Q?\r\ndone_when: [A]\r\n---\r\n\r\n## Signals\r\n\r\n- one\r\n",
        )
        .unwrap();
        assert_eq!(role.id, "demo");
        assert_eq!(role.steps, ["plan.trace"]);
        assert_eq!(role.criteria[0].text, "A");
        assert_eq!(role.knowledge.as_deref(), Some("## Signals\n\n- one"));
        assert!(role.disableable && role.domains.is_empty());

        let bare = parse_role_file(
            "---\nid: bare\nname: { en: Bare }\nat: plan.trace\nquestion: Q?\n---\n",
        )
        .unwrap();
        assert_eq!(bare.knowledge, None);
    }

    #[test]
    fn a_role_file_without_frontmatter_is_refused() {
        assert!(parse_role_file("id: demo\n").is_err());
        assert!(parse_role_file("---\nid: demo\n").is_err());
        let unknown = parse_role_file(
            "---\nid: demo\nname: { en: Demo }\nat: plan.trace\nquestion: Q?\nwhen: always\n---\n",
        )
        .unwrap_err();
        assert!(unknown.contains("when"), "{unknown}");
    }

    #[test]
    fn at_accepts_one_step_or_a_list() {
        let role = parse_role_file(
            "---\nid: demo\nname: { en: Demo }\nat: [plan.challenge, code.review]\nquestion: Q?\ndomains: [security]\n---\n",
        )
        .unwrap();
        assert_eq!(role.steps, ["plan.challenge", "code.review"]);
        assert_eq!(role.first_step(), "plan.challenge");
        assert!(role.serves("code.review") && !role.serves("plan.trace"));
        assert_eq!(role.default_activation(), Activation::OnDomain);
    }

    #[test]
    fn every_expert_carries_triggers_for_its_own_domain() {
        let roles = builtin_step_roles();
        for role in &roles {
            let is_expert = EXPERTS.iter().any(|(id, _)| *id == role.id);
            if !is_expert {
                assert!(role.triggers.is_empty(), "`{}` carries triggers", role.id);
                continue;
            }
            assert!(
                !role.triggers.is_empty(),
                "`{}` carries no trigger",
                role.id
            );
            for group in &role.triggers {
                assert_eq!(group.domain, role.domains[0], "`{}`", role.id);
            }
        }
        // `parse_role_file` refuses a group that does not compile: the
        // catalog parsing above is the check that every glob and pattern
        // compiles.
        let bad = parse_role_file(
            "---\nid: demo\nname: { en: Demo }\nat: plan.trace\nquestion: Q?\ntriggers:\n  - domain: security\n    patterns: ['(']\n---\n",
        );
        assert!(bad.unwrap_err().contains("`(`"));
    }

    #[test]
    fn security_triggers_name_the_dependency_manifests() {
        let security = builtin_step_roles()
            .into_iter()
            .find(|r| r.id == "security")
            .unwrap();
        let manifests: Vec<&String> = security
            .triggers
            .iter()
            .flat_map(|g| &g.manifests)
            .collect();
        for name in [
            "Cargo.toml",
            "package.json",
            "pyproject.toml",
            "requirements.txt",
            "go.mod",
            "pom.xml",
            "build.gradle",
        ] {
            assert!(
                manifests.iter().any(|m| *m == name),
                "security names {name}"
            );
        }
    }

    const KNOWLEDGE: &str = "## Signals\n\n- one\n\n## By stack\n\n- **Rust**: rust\n  pitfall.\n- **TypeScript/JavaScript**: ts.\n- **Rust, Python, Go, Java**: templates.\n- **Python**: py.\n- **SQL**: any.\n\n## References\n\n- ref";

    fn by_stack(knowledge: &str) -> Vec<&str> {
        knowledge
            .split("## By stack")
            .nth(1)
            .map(|rest| rest.split("\n## ").next().unwrap())
            .unwrap_or("")
            .lines()
            .filter_map(|line| line.strip_prefix("- **"))
            .map(|rest| rest.split("**").next().unwrap())
            .collect()
    }

    #[test]
    fn filter_by_stack_keeps_detected_languages() {
        let filtered = filter_by_stack(KNOWLEDGE, &["Rust"]);
        assert_eq!(
            by_stack(&filtered),
            ["Rust", "Rust, Python, Go, Java", "SQL"]
        );
        assert!(
            filtered.contains("- **Rust**: rust\n  pitfall.\n"),
            "{filtered}"
        );
        assert!(filtered.starts_with("## Signals\n\n- one\n\n## By stack\n\n"));
        assert!(
            filtered.ends_with("\n\n## References\n\n- ref"),
            "{filtered}"
        );
    }

    #[test]
    fn an_item_naming_several_languages_is_kept_for_any() {
        let filtered = filter_by_stack(KNOWLEDGE, &["Python"]);
        assert_eq!(
            by_stack(&filtered),
            ["Rust, Python, Go, Java", "Python", "SQL"]
        );
        let typescript = filter_by_stack(KNOWLEDGE, &["TypeScript", "JavaScript"]);
        assert_eq!(by_stack(&typescript), ["TypeScript/JavaScript", "SQL"]);
    }

    #[test]
    fn an_item_naming_no_known_language_is_kept() {
        assert!(by_stack(&filter_by_stack(KNOWLEDGE, &["Go"])).contains(&"SQL"));
    }

    #[test]
    fn the_section_is_left_out_when_nothing_remains() {
        let knowledge =
            "## Signals\n\n- one\n\n## By stack\n\n- **Rust**: r.\n\n## References\n\n- ref";
        assert_eq!(
            filter_by_stack(knowledge, &["Go"]),
            "## Signals\n\n- one\n\n## References\n\n- ref"
        );
        let last = "## Signals\n\n- one\n\n## By stack\n\n- **Rust**: r.\n";
        assert_eq!(filter_by_stack(last, &["Go"]), "## Signals\n\n- one");
    }

    #[test]
    fn no_language_keeps_every_item() {
        assert_eq!(filter_by_stack(KNOWLEDGE, &[]), KNOWLEDGE);
        assert_eq!(
            filter_by_stack("## Signals\n\n- one", &["Rust"]),
            "## Signals\n\n- one"
        );
    }

    /// The `## ` headings of a knowledge text, in order.
    fn sections(knowledge: &str) -> Vec<&str> {
        knowledge
            .lines()
            .filter_map(|line| line.strip_prefix("## "))
            .collect()
    }

    #[test]
    fn knowledge_uses_only_the_five_sections_in_order() {
        for role in builtin_step_roles() {
            let Some(knowledge) = &role.knowledge else {
                continue;
            };
            let found = sections(knowledge);
            assert!(
                !found.is_empty(),
                "`{}`: knowledge without a section",
                role.id
            );
            let ranks: Vec<usize> = found
                .iter()
                .map(|title| {
                    KNOWLEDGE_SECTIONS
                        .iter()
                        .position(|known| known == title)
                        .unwrap_or_else(|| panic!("`{}`: unknown section `{title}`", role.id))
                })
                .collect();
            assert!(
                ranks.windows(2).all(|pair| pair[0] < pair[1]),
                "`{}`: sections out of order: {found:?}",
                role.id
            );
        }
        for id in ["skeptic", "newcomer"] {
            let role = builtin_step_roles()
                .into_iter()
                .find(|r| r.id == id)
                .unwrap();
            assert!(role.knowledge.is_some(), "`{id}` carries knowledge");
        }
        let newcomer = builtin_step_roles()
            .into_iter()
            .find(|r| r.id == "newcomer")
            .unwrap();
        assert!(sections(newcomer.knowledge.as_deref().unwrap()).contains(&"By stack"));
    }

    #[test]
    fn every_expert_has_the_five_sections_and_its_references() {
        let references: [(&str, &[&str]); 6] = [
            ("security", &["OWASP Top 10", "ASVS", "CWE"]),
            ("privacy", &["GDPR", "Art. 5", "Art. 25"]),
            ("accessibility", &["WCAG 2.2", "RGAA 4"]),
            ("performance", &["profiler", "EXPLAIN"]),
            ("ops", &["Site Reliability Engineering", "Twelve-Factor"]),
            ("migration", &["Semantic Versioning", "expand and contract"]),
        ];
        for (id, needles) in references {
            let role = builtin_step_roles()
                .into_iter()
                .find(|r| r.id == id)
                .unwrap();
            let knowledge = role.knowledge.as_deref().unwrap_or_default();
            assert_eq!(sections(knowledge), KNOWLEDGE_SECTIONS, "`{id}`");
            let by_stack = knowledge
                .split("## By stack")
                .nth(1)
                .and_then(|rest| rest.split("\n## ").next())
                .unwrap();
            for stack in ["Rust", "TypeScript/JavaScript", "Python", "Go", "Java"] {
                assert!(by_stack.contains(stack), "`{id}`: By stack names {stack}");
            }
            let refs = knowledge.split("## References").nth(1).unwrap();
            for needle in needles {
                assert!(refs.contains(needle), "`{id}` references `{needle}`");
            }
        }
    }

    fn disabled_ids(roles: &[ResolvedRole], step: &str) -> Vec<String> {
        disabled_at(roles, step)
            .iter()
            .map(|r| r.role.id.clone())
            .collect()
    }

    #[test]
    fn the_roles_set_to_never_are_disabled_at_their_step() {
        let roles = resolve_with(&settings("newcomer: never"), &BTreeMap::new()).unwrap();
        assert_eq!(disabled_ids(&roles, "code.review"), ["newcomer"]);
        assert!(disabled_ids(&roles, "plan.challenge").is_empty());
        assert_eq!(role(&roles, "newcomer").when_origin, WhenOrigin::Project);
        assert!(!active_ids(&roles, "code.review").contains(&"newcomer".to_string()));
    }

    #[test]
    fn a_role_turned_back_on_by_the_change_is_not_disabled() {
        let roles =
            resolve_with(&settings("newcomer: never"), &settings("newcomer: always")).unwrap();
        assert!(disabled_ids(&roles, "code.review").is_empty());
        assert!(active_ids(&roles, "code.review").contains(&"newcomer".to_string()));
    }

    #[test]
    fn an_inactive_expert_is_not_disabled() {
        let roles = resolve_with(&BTreeMap::new(), &BTreeMap::new()).unwrap();
        assert!(disabled_ids(&roles, "code.review").is_empty());
        assert!(!active_ids(&roles, "code.review").contains(&"security".to_string()));
    }

    #[test]
    fn a_model_on_the_developer_is_used() {
        let project = settings("developer: { model: sonnet }\n");
        assert!(unused_models(&demo_catalog(), &project, "_codev/config.yaml").is_empty());
    }

    #[test]
    fn a_model_on_another_role_at_code_implement_is_unused() {
        // A project role declared at the developer's step: no sub-agent
        // ever runs it, so its model never applies.
        let mut catalog = demo_catalog();
        let mut pair = catalog
            .iter()
            .find(|role| role.id == DEVELOPER)
            .unwrap()
            .clone();
        pair.id = "pair-programmer".into();
        catalog.push(pair);
        let project = settings("pair-programmer: { model: sonnet }\n");
        let found = unused_models(&catalog, &project, "_codev/config.yaml");
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].role, "pair-programmer");
    }

    #[test]
    fn only_a_real_model_on_the_developer_delegates() {
        assert!(developer_has_model(&settings(
            "developer: { model: sonnet }\n"
        )));
        assert!(!developer_has_model(&settings(
            "developer: { model: inherit }\n"
        )));
        assert!(!developer_has_model(&settings(
            "skeptic: { model: sonnet }\n"
        )));
        assert!(!developer_has_model(&BTreeMap::new()));
    }
}
