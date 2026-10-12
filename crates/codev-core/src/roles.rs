//! Roles: who owns which question at which step of the workflow, and the
//! "done when" criteria it checks there.
//!
//! Roles are data. The step roles come from `assets/roles.yaml`, embedded
//! here; the artifact roles come from the schema in use. This module
//! merges them into one catalog, checks the project's and the change's
//! settings against it, and decides which roles are active — all without
//! touching a file (decision 0001): the engine reads the files and hands
//! their content over.

use std::collections::BTreeMap;

use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_norway::Value;

use crate::schema::Schema;

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

const BUILTIN_CATALOG_YAML: &str = include_str!("../../../assets/roles.yaml");

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

/// A step role, as written in `assets/roles.yaml`.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct StepRole {
    id: String,
    name: Names,
    at: String,
    question: String,
    #[serde(default)]
    required: bool,
    #[serde(default)]
    done_when: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct CatalogFile {
    roles: Vec<StepRole>,
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
    /// One of [`STEPS`].
    pub at: String,
    pub question: String,
    pub criteria: Vec<Criterion>,
    /// False for artifact roles and for the step roles marked `required`.
    pub disableable: bool,
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
}

/// The built-in step roles, in catalog order.
pub fn builtin_step_roles() -> Vec<Role> {
    // The file is embedded: a parse failure is a build bug, which the
    // test `the_builtin_catalog_has_the_six_step_roles_at_their_steps`
    // catches before any release.
    let file: CatalogFile =
        serde_norway::from_str(BUILTIN_CATALOG_YAML).expect("assets/roles.yaml is valid");
    file.roles
        .into_iter()
        .map(|role| Role {
            criteria: criteria(&role.done_when, BUILTIN_ORIGIN),
            id: role.id,
            names: role.name,
            kind: RoleKind::Step,
            at: role.at,
            question: role.question,
            disableable: !role.required,
        })
        .collect()
}

/// True when `id` is the id of a built-in step role.
pub fn is_builtin_step_role(id: &str) -> bool {
    builtin_step_roles().iter().any(|role| role.id == id)
}

/// The full catalog for a schema: the built-in step roles and the
/// schema's artifact roles, ordered by step, then by catalog or schema
/// order.
pub fn catalog(schema: &Schema, schema_name: &str) -> Vec<Role> {
    let origin = format!("schema:{schema_name}");
    let mut roles = builtin_step_roles();
    for artifact in &schema.artifacts {
        if let Some(role) = &artifact.role {
            roles.push(Role {
                id: role.id.clone(),
                names: role.name.clone(),
                kind: RoleKind::Artifact(artifact.id.clone()),
                at: ARTIFACT_STEP.to_string(),
                question: role.question.clone(),
                criteria: criteria(&role.done_when, &origin),
                disableable: false,
            });
        }
    }
    // A stable sort keeps the catalog order within a step.
    roles.sort_by_key(|role| step_rank(&role.at));
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
}

impl Activation {
    pub fn parse(text: &str) -> Option<Self> {
        match text {
            "always" => Some(Self::Always),
            "never" => Some(Self::Never),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Always => "always",
            Self::Never => "never",
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

/// A role entry of `roles:`, as written: `skeptic: never`, or a mapping
/// with `when` and `done_when`.
///
/// The activation stays text: an unexpected value is reported by
/// [`check_settings`] with `invalid_role_activation`, naming the accepted
/// values, rather than as a YAML type error.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RoleSetting {
    pub when: Option<String>,
    pub done_when: Vec<String>,
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
            });
        }
        Ok(Self {
            when: Some(scalar_text(&value)),
            done_when: Vec::new(),
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
    /// `unknown_role`, `invalid_role_activation` or `role_not_disableable`.
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
        let Some(when) = &setting.when else { continue };
        match Activation::parse(when) {
            None => findings.push(finding(
                INVALID_ROLE_ACTIVATION,
                format!(
                    "{file}: role `{id}` has the activation `{when}`; use `always` or `never`"
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

/// A role with its settings applied.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedRole {
    pub role: Role,
    pub when: Activation,
    pub when_origin: WhenOrigin,
}

impl ResolvedRole {
    pub fn is_active(&self) -> bool {
        self.when == Activation::Always
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
}

/// Applies the project's settings, then the change's, to the catalog.
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
            let mut when = Activation::Always;
            let mut when_origin = WhenOrigin::Default;
            if let Some(project) = settings.project.get(&role.id) {
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
            ResolvedRole {
                role,
                when,
                when_origin,
            }
        })
        .collect())
}

/// The roles active at `step`, in catalog order.
pub fn active_at<'a>(roles: &'a [ResolvedRole], step: &str) -> Vec<&'a ResolvedRole> {
    roles
        .iter()
        .filter(|resolved| resolved.role.at == step && resolved.is_active())
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
        resolve(
            demo_catalog(),
            Settings {
                project,
                project_file: "_codev/config.yaml",
                change,
                change_file: "change.yaml",
            },
        )
    }

    fn role<'a>(roles: &'a [ResolvedRole], id: &str) -> &'a ResolvedRole {
        roles.iter().find(|r| r.role.id == id).unwrap()
    }

    #[test]
    fn the_builtin_catalog_has_the_six_step_roles_at_their_steps() {
        let roles = builtin_step_roles();
        let pairs: Vec<(&str, &str)> = roles
            .iter()
            .map(|role| (role.id.as_str(), role.at.as_str()))
            .collect();
        assert_eq!(
            pairs,
            [
                ("analyst", "plan.understand"),
                ("investigator", "plan.investigate"),
                ("tracer", "plan.trace"),
                ("skeptic", "plan.challenge"),
                ("developer", "code.implement"),
                ("newcomer", "code.review"),
            ]
        );
        for role in &roles {
            assert!(STEPS.contains(&role.at.as_str()), "{}", role.id);
            assert!(role.names.contains_key("en") && role.names.contains_key("fr"));
            assert!(!role.question.is_empty() && !role.criteria.is_empty());
            assert!(role.criteria.iter().all(|c| c.origin == "built-in"));
        }
        let required: Vec<&str> = roles
            .iter()
            .filter(|role| !role.disableable)
            .map(|role| role.id.as_str())
            .collect();
        assert_eq!(required, ["developer"]);
    }

    #[test]
    fn default_catalog_lists_ten_roles_in_workflow_order() {
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
            role.at == "plan.write"
                && !role.disableable
                && role
                    .criteria
                    .iter()
                    .all(|c| c.origin == "schema:spec-driven")
        }));
    }

    #[test]
    fn a_schema_without_roles_adds_only_step_roles() {
        let roles = catalog(&Schema::parse(SCHEMA_WITHOUT_ROLES).unwrap(), "bare");
        assert_eq!(roles.len(), 6);
        assert!(roles.iter().all(|role| role.kind == RoleKind::Step));
    }

    #[test]
    fn every_role_is_active_by_default() {
        let roles = resolve_with(&BTreeMap::new(), &BTreeMap::new()).unwrap();
        assert!(
            roles
                .iter()
                .all(|r| r.when == Activation::Always && r.when_origin == WhenOrigin::Default)
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
    fn an_activation_other_than_always_or_never_is_reported() {
        for yaml in [
            "skeptic: sometimes\n",
            "skeptic: false\n",
            "skeptic: { when: off }\n",
        ] {
            let found = check_settings(&demo_catalog(), &settings(yaml), "_codev/config.yaml");
            assert_eq!(found.len(), 1, "{yaml}");
            assert_eq!(found[0].code, "invalid_role_activation");
            assert!(
                found[0].message.contains("`always` or `never`"),
                "{}",
                found[0].message
            );
        }
    }

    #[test]
    fn artifact_roles_and_the_developer_cannot_be_disabled() {
        for id in ["product", "lead", "developer"] {
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
                    "Impact touches them",
                    "accessibility & UX",
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
}
