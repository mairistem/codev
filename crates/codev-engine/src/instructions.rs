use std::path::PathBuf;

use codev_core::decisions::DecisionStatus;
use codev_core::roles::ResolvedRole;
use codev_core::{ArtifactState, ChangeId, CoreError, Layout};

use crate::change::{self, ChangeContext};
use crate::config::{Block, ResolvedConfig};
use crate::decisions::{self, Origin};
use crate::error::{EngineError, Result, Warning};
use crate::ports::{Env, FileSystem};

/// An artifact already written, to reread for orientation before writing the next one.
#[derive(Debug, Clone)]
pub struct Dependency {
    pub id: String,
    pub path: PathBuf,
    pub done: bool,
}

/// A reference to an architecture decision in effect.
///
/// Deliberately a **reference** — not the full content: the skill consuming
/// this list reads the file through `path` if it needs to, just as it already
/// does for dependencies. Duplicating the body here would bloat every
/// instruction for no benefit.
#[derive(Debug, Clone)]
pub struct DecisionRef {
    /// The short `id`, as written in the frontmatter.
    pub id: String,
    /// The qualified `origin/id` identifier — avoids collisions between the
    /// project and inherited sources.
    pub qualified_id: String,
    pub title: String,
    /// Always one of the five recognized statuses, serialized in lowercase.
    /// For a decision "in effect" it is `accepted` — exposing it keeps the
    /// consumer honest if the computation changes later.
    pub status: String,
    pub tags: Vec<String>,
    /// Path relative to the project.
    pub path: PathBuf,
    /// `"project"` or `"path:<declared path>"`.
    pub origin: String,
}

/// Everything an agent needs to know to write an artifact.
///
/// This is the centerpiece of the system: the CLI writes nothing itself, it
/// assembles the context. `instruction` comes from the schema, `template`
/// from its files, `context` and `rules` from the configuration and the
/// inherited sources.
#[derive(Debug, Clone)]
pub struct Instructions {
    pub change: ChangeId,
    pub schema_name: String,
    pub artifact_id: String,
    pub description: Option<String>,
    /// Where to write. May be a glob pattern: `instruction` then explains how
    /// to pick the concrete path.
    pub resolved_output_path: PathBuf,
    pub instruction: Option<String>,
    pub template: Option<String>,
    /// Language of the prose to write (from `language:` in the config).
    pub language: String,
    /// The language the structural keywords are written in: the catalog
    /// language of `language:`, English when the catalog does not ship it.
    /// The template is served in it.
    pub keyword_language: codev_core::keywords::Lang,
    /// Constraints for the agent, never content to copy into the file.
    pub context: Vec<Block>,
    pub rules: Vec<Block>,
    pub dependencies: Vec<Dependency>,
    pub unlocks: Vec<String>,
    /// The architecture decisions **in effect** — filled for the `design`
    /// artifact, left empty for the others. A skill producing a design
    /// reads these references and opens the files they point to before
    /// writing. The field is always present, possibly empty, so that the
    /// consumer can test `decisions.length` without a conditional
    /// branch.
    pub decisions: Vec<DecisionRef>,
    /// The existing requirements the change likely touches, ranked against
    /// the proposal — for the artifacts that require the proposal directly,
    /// once it is written; empty for the others, which read no main spec.
    pub related: Vec<crate::search::RelatedRequirement>,
    /// True when the change has disabled this artifact: it must **not** be
    /// created.
    pub skipped: bool,
    /// The role that owns the artifact, settings applied; `None` when the
    /// schema declares none.
    pub role: Option<ResolvedRole>,
    pub warnings: Vec<Warning>,
}

/// The most related requirements the instructions give: enough to point to
/// the specs a change touches, bounded so that the cost does not grow with
/// the project.
pub const RELATED_LIMIT: usize = 10;

/// Assembles the instructions for an artifact.
///
/// `artifact_id` set to `None` asks for "the next one to write", i.e. the
/// first `Ready` artifact in topological order.
pub fn for_artifact(
    fs: &dyn FileSystem,
    env: &dyn Env,
    layout: &Layout,
    ctx: &ChangeContext,
    config: &ResolvedConfig,
    artifact_id: Option<&str>,
) -> Result<Instructions> {
    let status = change::status(fs, layout, ctx)?;
    // Resolved first: an invalid role setting fails the command before
    // anything is assembled.
    let roles = crate::roles::resolve(fs, layout, config, Some(ctx))?;

    let artifact_id = match artifact_id {
        Some(id) => {
            if ctx.schema.graph.artifact(id).is_none() {
                return Err(CoreError::UnknownArtifact {
                    schema: ctx.schema.name().to_string(),
                    artifact: id.to_string(),
                }
                .into());
            }
            id.to_string()
        }
        None => status
            .artifacts
            .iter()
            .find(|a| a.state == ArtifactState::Ready)
            .map(|a| a.id.clone())
            .ok_or_else(|| EngineError::NoArtifactReady {
                change: ctx.change.to_string(),
            })?,
    };

    let artifact = ctx
        .schema
        .graph
        .artifact(&artifact_id)
        .expect("the id was just validated against the graph");
    let state = status
        .artifacts
        .iter()
        .find(|a| a.id == artifact_id)
        .map(|a| a.state)
        .expect("every artifact in the graph appears in the status");

    let change_dir = layout.change_dir(&ctx.change);
    let mut warnings = config.warnings.clone();
    if state == ArtifactState::Skipped {
        warnings.push(Warning::new(
            "artifact_skipped",
            format!(
                "artifact `{artifact_id}` is disabled by `skip_specs` in change.yaml: \
                 its files must not be created"
            ),
        ));
    }

    let dependencies = artifact
        .requires
        .iter()
        .map(|dep_id| {
            let done = status
                .artifacts
                .iter()
                .find(|a| &a.id == dep_id)
                .is_some_and(|a| a.state.satisfies_dependency());
            let generates = ctx
                .schema
                .graph
                .artifact(dep_id)
                .map(|a| a.generates.clone())
                .unwrap_or_default();
            Dependency {
                id: dep_id.clone(),
                path: change_dir.join(generates),
                done,
            }
        })
        .collect();

    // Injected decisions: only for `design`. For the other artifacts we
    // return an empty list rather than loading the index for nothing.
    let decisions = if artifact.id == "design" {
        let index = decisions::index(fs, env, layout, config)?;
        warnings.extend(
            index
                .findings
                .iter()
                .map(|f| Warning::new(f.code, f.message.clone())),
        );
        build_decision_refs(&index, layout)
    } else {
        Vec::new()
    };

    let (related, unreadable_warnings) =
        related_for(fs, layout, config, &ctx.schema.graph, artifact, &change_dir);
    warnings.extend(unreadable_warnings);

    // The keywords follow the template served: a project schema's single
    // template, written before codev shipped the language, keeps its own.
    let setting = codev_core::keywords::Lang::for_setting(&config.language);
    let template = ctx.schema.template(fs, artifact, setting)?;
    let keyword_language = template
        .as_deref()
        .and_then(codev_core::keywords::text_language)
        .unwrap_or(setting);
    Ok(Instructions {
        change: ctx.change.clone(),
        schema_name: ctx.schema.name().to_string(),
        artifact_id: artifact.id.clone(),
        description: artifact.description.clone(),
        resolved_output_path: change_dir.join(&artifact.generates),
        instruction: artifact.instruction.clone(),
        template,
        language: config.language.clone(),
        keyword_language,
        context: config.context.clone(),
        rules: config.rules_for(&artifact.id).to_vec(),
        dependencies,
        unlocks: ctx
            .schema
            .graph
            .unlocked_by(&artifact.id)
            .iter()
            .map(|id| id.to_string())
            .collect(),
        decisions,
        related,
        skipped: state == ArtifactState::Skipped,
        role: roles.for_artifact(&artifact.id).cloned(),
        warnings,
    })
}

/// The related requirements: only for the artifacts that require the
/// proposal directly — the specs and the design write deltas and choices
/// about existing behavior; the others only need the context and rules,
/// and must not pay for reading every main spec.
fn related_for(
    fs: &dyn FileSystem,
    layout: &Layout,
    config: &ResolvedConfig,
    graph: &codev_core::ArtifactGraph,
    artifact: &codev_core::Artifact,
    change_dir: &std::path::Path,
) -> (Vec<crate::search::RelatedRequirement>, Vec<Warning>) {
    let Some(proposal) = graph
        .artifacts()
        .iter()
        .find(|a| a.generates == "proposal.md")
    else {
        return (Vec::new(), Vec::new());
    };
    if !artifact.requires.contains(&proposal.id) {
        return (Vec::new(), Vec::new());
    }
    let Ok(text) = fs.read_to_string(&change_dir.join(&proposal.generates)) else {
        return (Vec::new(), Vec::new());
    };
    crate::search::related_requirements(fs, layout, &config.language, &text, RELATED_LIMIT)
}

/// Converts the index's `in_effect` entries into `DecisionRef`s ready for
/// the public contract.
///
/// The order is that of `in_effect` — which follows the natural order of
/// the entries: project first, then inherited sources, each sorted by file
/// id. Predictable and testable.
fn build_decision_refs(index: &decisions::DecisionIndex, layout: &Layout) -> Vec<DecisionRef> {
    let mut out = Vec::with_capacity(index.in_effect.len());
    for qid in &index.in_effect {
        let Some(entry) = index.entries.iter().find(|e| &e.qualified_id == qid) else {
            continue; // should not happen — `in_effect` is derived from `entries`
        };
        let status = match &entry.decision.status {
            DecisionStatus::Unknown(raw) => raw.clone(),
            other => other.as_str().to_string(),
        };
        // Path relative to the project when possible; otherwise keep the
        // absolute path (inherited source outside the repository).
        let relative_path = entry
            .path
            .strip_prefix(layout.project_root())
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|_| entry.path.clone());
        out.push(DecisionRef {
            id: entry.decision.id.clone(),
            qualified_id: qid.as_str(),
            title: entry.decision.title.clone(),
            status,
            tags: entry.decision.tags.clone(),
            path: relative_path,
            origin: match &entry.qualified_id.origin {
                Origin::Project => "project".to_string(),
                Origin::Path(raw) => format!("path:{raw}"),
                Origin::Git(url) => format!("git:{url}"),
            },
        });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ports::{FixedEnv, MemoryFileSystem};
    use codev_core::ChangeId;

    fn project(files: &[(&str, &str)]) -> MemoryFileSystem {
        let mut fs = MemoryFileSystem::new().with_file(
            "/p/_codev/config.yaml",
            "context: |\n  Stack: Rust\nrules:\n  specs:\n    - Only observable behavior\n  design:\n    - Cite the decisions\n",
        );
        for (path, contents) in files {
            fs = fs.with_file(*path, *contents);
        }
        fs
    }

    fn env() -> FixedEnv {
        let mut env = FixedEnv::at("/p");
        env.vars.insert("HOME".into(), "/home".into());
        env
    }

    fn instructions(fs: &MemoryFileSystem, artifact: Option<&str>) -> Result<Instructions> {
        let layout = Layout::new("/p");
        let e = env();
        let config = crate::config::resolve(fs, &e, &layout).unwrap();
        let ctx = change::load(fs, &layout, &config, ChangeId::parse("add-auth").unwrap()).unwrap();
        for_artifact(fs, &e, &layout, &ctx, &config, artifact)
    }

    #[test]
    fn without_a_named_artifact_returns_the_next_to_write() {
        let fs = project(&[
            (
                "/p/_codev/changes/add-auth/change.yaml",
                "schema: spec-driven",
            ),
            ("/p/_codev/changes/add-auth/proposal.md", "# Proposal"),
        ]);
        let instr = instructions(&fs, None).unwrap();
        assert_eq!(instr.artifact_id, "specs");
    }

    #[test]
    fn carries_the_schema_template_and_instruction() {
        let fs = project(&[(
            "/p/_codev/changes/add-auth/change.yaml",
            "schema: spec-driven",
        )]);
        let instr = instructions(&fs, Some("proposal")).unwrap();

        assert!(
            instr.instruction.is_some_and(|i| i.contains("WHY")),
            "the schema instruction must come through"
        );
        assert!(
            instr.template.is_some_and(|t| t.contains("## Why")),
            "the embedded template must come through"
        );
        assert_eq!(
            instr.resolved_output_path,
            PathBuf::from("/p/_codev/changes/add-auth/proposal.md")
        );
    }

    fn spec_driven() -> MemoryFileSystem {
        project(&[(
            "/p/_codev/changes/add-auth/change.yaml",
            "schema: spec-driven",
        )])
    }

    /// The artifact's role criteria, joined on one line so that an
    /// assertion survives rewrapping.
    fn role_criteria(fs: &MemoryFileSystem, artifact: &str) -> String {
        let role = instructions(fs, Some(artifact))
            .unwrap()
            .role
            .unwrap_or_else(|| panic!("`{artifact}` must carry a role"));
        role.role
            .criteria
            .iter()
            .map(|c| c.text.as_str())
            .collect::<Vec<_>>()
            .join(" ")
    }

    #[test]
    fn spec_driven_artifacts_carry_their_role_as_data() {
        let fs = spec_driven();
        for (artifact, id, name, criterion) in [
            ("proposal", "product", "Product owner", "**BREAKING**"),
            ("proposal", "product", "Product owner", "release note"),
            ("specs", "qa", "QA analyst", "edge-case scenario"),
            ("design", "architect", "Architect", "alternative it rejects"),
            (
                "tasks",
                "lead",
                "Tech lead",
                "every spec scenario is covered",
            ),
        ] {
            let instr = instructions(&fs, Some(artifact)).unwrap();
            let role = instr.role.expect("a spec-driven artifact has a role");
            assert_eq!(role.role.id, id);
            assert_eq!(role.role.name("en"), name);
            assert!(!role.role.question.is_empty());
            assert!(
                role.role
                    .criteria
                    .iter()
                    .all(|c| c.origin == "schema:spec-driven")
            );
            assert!(
                role_criteria(&fs, artifact).contains(criterion),
                "`{artifact}` criteria must mention `{criterion}`"
            );
            // The role is data now: the instruction no longer restates it.
            let instruction = instr.instruction.unwrap_or_default();
            assert!(!instruction.starts_with("Role:"), "{artifact}");
            assert!(!instruction.contains("Done when:"), "{artifact}");
        }
    }

    #[test]
    fn tasks_criteria_require_documentation_and_automated_tests() {
        let criteria = role_criteria(&spec_driven(), "tasks");
        for criterion in [
            "documentation the change affects",
            "states that none is affected",
            "covered by an automated test",
            "only when automation is impossible",
            "project's existing test suite",
            "conventions of its language",
            "the project's `rules`",
        ] {
            assert!(
                criteria.contains(criterion),
                "the tech lead criteria must mention `{criterion}`"
            );
        }
    }

    #[test]
    fn design_criteria_require_the_simplest_approach_a_newcomer_can_follow() {
        let criteria = role_criteria(&spec_driven(), "design");
        for criterion in [
            "the simplest that satisfies the specs",
            "a newcomer can follow the design without knowing its history",
        ] {
            assert!(
                criteria.contains(criterion),
                "the architect criteria must mention `{criterion}`"
            );
        }
    }

    #[test]
    fn instructions_carry_the_artifact_role_with_project_criteria() {
        let fs = MemoryFileSystem::new()
            .with_file(
                "/p/_codev/config.yaml",
                "roles:\n  qa:\n    done_when: [Every error code is named]\n",
            )
            .with_file(
                "/p/_codev/changes/add-auth/change.yaml",
                "schema: spec-driven",
            );
        let role = instructions(&fs, Some("specs")).unwrap().role.unwrap();
        assert_eq!(role.role.id, "qa");
        let last = role.role.criteria.last().unwrap();
        assert_eq!(last.text, "Every error code is named");
        assert_eq!(last.origin, "project");
        assert_eq!(role.role.criteria[0].origin, "schema:spec-driven");
    }

    #[test]
    fn an_invalid_role_setting_fails_the_instructions() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "roles:\n  qa: never\n")
            .with_file(
                "/p/_codev/changes/add-auth/change.yaml",
                "schema: spec-driven",
            );
        let err = instructions(&fs, Some("specs")).unwrap_err();
        assert_eq!(err.code(), "role_not_disableable");
    }

    #[test]
    fn proposal_impact_names_the_affected_documentation() {
        let fs = project(&[(
            "/p/_codev/changes/add-auth/change.yaml",
            "schema: spec-driven",
        )]);
        let instruction = instructions(&fs, Some("proposal"))
            .unwrap()
            .instruction
            .unwrap_or_default();
        let (_, impact) = instruction
            .split_once("- **Impact**:")
            .expect("the proposal instruction describes Impact");
        assert!(
            impact.contains("documentation to update"),
            "Impact must name the affected documentation"
        );
    }

    #[test]
    fn proposal_template_and_instruction_carry_the_release_note() {
        let fs = project(&[(
            "/p/_codev/changes/add-auth/change.yaml",
            "schema: spec-driven",
        )]);
        let instr = instructions(&fs, Some("proposal")).unwrap();
        let rest = instr.instruction.unwrap_or_default();
        assert!(
            role_criteria(&fs, "proposal").contains("release note is written for end users"),
            "the product owner criteria must require the release note"
        );
        for word in ["**Release note**", "`Added`", "`Security`", "`None`"] {
            assert!(
                rest.contains(word),
                "the instruction must describe `{word}`"
            );
        }
        let template = instr.template.unwrap_or_default();
        let impact = template.find("## Impact").expect("Impact section");
        let note = template
            .find("## Release note")
            .expect("the template carries a Release note section");
        assert!(note > impact, "Release note comes after Impact");
    }

    #[test]
    fn proposal_template_and_instruction_carry_the_domains() {
        let fs = project(&[(
            "/p/_codev/changes/add-auth/change.yaml",
            "schema: spec-driven",
        )]);
        let instr = instructions(&fs, Some("proposal")).unwrap();
        assert!(
            role_criteria(&fs, "proposal").contains("Domains ticks every domain"),
            "the product owner criteria must require the domains"
        );
        let rest = instr.instruction.unwrap_or_default();
        assert!(rest.contains("**Domains**") && rest.contains("`None`"));
        let template = instr.template.unwrap_or_default();
        let impact = template.find("## Impact").unwrap();
        let domains = template.find("## Domains").expect("a Domains section");
        let note = template.find("## Release note").unwrap();
        assert!(impact < domains && domains < note, "{template}");
        let parsed = codev_core::domains::parse(&template);
        assert!(parsed.findings.is_empty(), "{:?}", parsed.findings);
        assert!(
            parsed.value.ticked().is_empty(),
            "nothing ticked by default"
        );
        for domain in codev_core::domains::DOMAINS {
            assert!(
                template.contains(&format!("- [ ] {domain}\n")),
                "the template lists `{domain}` unticked"
            );
        }
    }

    #[test]
    fn a_project_schema_instruction_is_returned_as_written() {
        // Roles belong to the schema: codev adds none to a custom one.
        let fs = project(&[
            (
                "/p/_codev/schemas/lite/schema.yaml",
                "name: lite\nartifacts:\n  - id: brief\n    generates: brief.md\n    instruction: Write the brief.\napply:\n  requires: [brief]\n  tracks: brief.md\n",
            ),
            ("/p/_codev/changes/add-auth/change.yaml", "schema: lite"),
        ]);
        let instr = instructions(&fs, Some("brief")).unwrap();
        assert_eq!(instr.instruction.as_deref(), Some("Write the brief."));
        assert!(instr.role.is_none(), "an artifact without role has none");
    }

    #[test]
    fn injects_only_the_requested_artifacts_rules() {
        let fs = project(&[(
            "/p/_codev/changes/add-auth/change.yaml",
            "schema: spec-driven",
        )]);

        let specs = instructions(&fs, Some("specs")).unwrap();
        assert_eq!(specs.rules.len(), 1);
        assert!(specs.rules[0].text.contains("observable"));

        let proposal = instructions(&fs, Some("proposal")).unwrap();
        assert!(proposal.rules.is_empty(), "proposal has no declared rule");

        // Context, on the other hand, applies everywhere.
        assert_eq!(proposal.context.len(), 1);
        assert_eq!(proposal.context[0].origin, "project");
    }

    #[test]
    fn tells_what_the_artifact_unlocks_and_what_to_reread() {
        let fs = project(&[
            (
                "/p/_codev/changes/add-auth/change.yaml",
                "schema: spec-driven",
            ),
            ("/p/_codev/changes/add-auth/proposal.md", "# Proposal"),
        ]);
        let instr = instructions(&fs, Some("specs")).unwrap();

        assert_eq!(instr.unlocks, ["tasks"]);
        assert_eq!(instr.dependencies.len(), 1);
        assert_eq!(instr.dependencies[0].id, "proposal");
        assert!(instr.dependencies[0].done);
        assert_eq!(
            instr.dependencies[0].path,
            PathBuf::from("/p/_codev/changes/add-auth/proposal.md")
        );
    }

    #[test]
    fn instructions_carry_a_decisions_field_even_when_empty() {
        // No ADR in the project — the field exists but stays empty.
        // Always present: the consumer tests `decisions.length` without a
        // branch.
        let fs = project(&[(
            "/p/_codev/changes/add-auth/change.yaml",
            "schema: spec-driven",
        )]);
        let instr = instructions(&fs, Some("design")).unwrap();
        assert!(instr.decisions.is_empty(), "no declared ADR = empty");
    }

    #[test]
    fn design_receives_the_decisions_in_effect() {
        let fs = project(&[
            (
                "/p/_codev/changes/add-auth/change.yaml",
                "schema: spec-driven",
            ),
            ("/p/_codev/changes/add-auth/proposal.md", "# Proposal"),
            (
                "/p/_codev/decisions/0001-foundation.md",
                "---\nid: 0001\ntitle: Foundation\nstatus: accepted\ndate: 2026-09-08\n---\n\n## Context\n\nx\n",
            ),
        ]);
        let instr = instructions(&fs, Some("design")).unwrap();
        assert_eq!(instr.decisions.len(), 1);
        assert_eq!(instr.decisions[0].id, "0001");
        assert_eq!(instr.decisions[0].title, "Foundation");
        assert_eq!(instr.decisions[0].status, "accepted");
        assert_eq!(instr.decisions[0].origin, "project");
        // Path relative to the project, not absolute.
        assert_eq!(
            instr.decisions[0].path,
            PathBuf::from("_codev/decisions/0001-foundation.md")
        );
    }

    #[test]
    fn proposal_does_not_receive_the_decisions() {
        // A decision is present BUT the requested artifact is not `design`:
        // the field exists and stays empty. A future change may extend this
        // to other artifacts without breaking this test — it will have to be
        // moved, not just deleted, to keep the boundary explicit.
        let fs = project(&[
            (
                "/p/_codev/changes/add-auth/change.yaml",
                "schema: spec-driven",
            ),
            (
                "/p/_codev/decisions/0001.md",
                "---\nid: 0001\ntitle: X\nstatus: accepted\ndate: 2026-09-08\n---\n",
            ),
        ]);
        let instr = instructions(&fs, Some("proposal")).unwrap();
        assert!(
            instr.decisions.is_empty(),
            "only `design` receives the decisions in this batch"
        );
    }

    #[test]
    fn a_disabled_artifact_is_reported_and_not_to_be_created() {
        let fs = project(&[(
            "/p/_codev/changes/add-auth/change.yaml",
            "schema: spec-driven\nskip_specs: true\n",
        )]);
        let instr = instructions(&fs, Some("specs")).unwrap();
        assert!(instr.skipped);
        assert!(instr.warnings.iter().any(|w| w.code == "artifact_skipped"));
    }

    #[test]
    fn rejects_an_artifact_missing_from_the_schema() {
        let fs = project(&[(
            "/p/_codev/changes/add-auth/change.yaml",
            "schema: spec-driven",
        )]);
        let err = instructions(&fs, Some("sketch")).unwrap_err();
        assert_eq!(err.code(), "unknown_artifact");
    }

    #[test]
    fn when_everything_is_written_nothing_is_ready() {
        let fs = project(&[
            (
                "/p/_codev/changes/add-auth/change.yaml",
                "schema: spec-driven",
            ),
            ("/p/_codev/changes/add-auth/proposal.md", "x"),
            ("/p/_codev/changes/add-auth/specs/user-auth/spec.md", "x"),
            ("/p/_codev/changes/add-auth/design.md", "x"),
            ("/p/_codev/changes/add-auth/tasks.md", "x"),
        ]);
        let err = instructions(&fs, None).unwrap_err();
        assert_eq!(err.code(), "no_artifact_ready");
    }

    #[test]
    fn design_instructions_omit_deviated_decisions() {
        // An inherited decision that a local ADR deviates from NO LONGER
        // appears among the decisions injected into the `design` artifact —
        // the agent sees the deviation, not the decision it replaces.
        let local_adr = "---\nid: \"0007\"\ntitle: Local alternative\nstatus: accepted\ndate: 2026-09-08\ndeviates_from: [\"path:~/shared/0100\"]\n---\n\n## Context\n\nx\n";
        let inherited_adr = "---\nid: \"0100\"\ntitle: Source choice\nstatus: accepted\ndate: 2026-09-08\n---\n\n## Context\n\ny\n";
        let fs = project(&[
            ("/p/_codev/config.yaml", "inherits:\n  - path: ~/shared\n"),
            ("/p/_codev/decisions/0007-alternative.md", local_adr),
            (
                "/home/shared/_codev/decisions/0100-choice.md",
                inherited_adr,
            ),
            (
                "/p/_codev/changes/add-auth/change.yaml",
                "schema: spec-driven",
            ),
            (
                "/p/_codev/changes/add-auth/proposal.md",
                "# Proposal\n\n## Why\n\nT\n\n## What Changes\n\n- x\n",
            ),
            (
                "/p/_codev/changes/add-auth/specs/x/spec.md",
                "## ADDED Requirements\n\n### Requirement: X\nThe system SHALL x.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n",
            ),
        ]);
        let instr = instructions(&fs, Some("design")).unwrap();

        let ids: Vec<&str> = instr
            .decisions
            .iter()
            .map(|d| d.qualified_id.as_str())
            .collect();
        assert!(
            ids.contains(&"project/0007"),
            "the local deviation must be present"
        );
        assert!(
            !ids.contains(&"path:~/shared/0100"),
            "the deviated inherited decision must disappear; ids present: {ids:?}"
        );
    }

    #[test]
    fn configuration_warnings_carry_through_to_instructions() {
        let mut fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "inherits:\n  - path: /nowhere\n");
        fs = fs.with_file(
            "/p/_codev/changes/add-auth/change.yaml",
            "schema: spec-driven",
        );
        let instr = instructions(&fs, Some("proposal")).unwrap();
        assert!(
            instr
                .warnings
                .iter()
                .any(|w| w.code == "inherit_unresolved")
        );
    }

    #[test]
    fn dependencies_not_written_yet_are_not_done() {
        let fs = project(&[(
            "/p/_codev/changes/add-auth/change.yaml",
            "schema: spec-driven",
        )]);
        let instr = instructions(&fs, Some("tasks")).unwrap();
        let deps: Vec<(&str, bool)> = instr
            .dependencies
            .iter()
            .map(|d| (d.id.as_str(), d.done))
            .collect();
        assert_eq!(deps, [("specs", false), ("design", false)]);
    }

    #[test]
    fn instructions_carry_the_configured_language() {
        let change = (
            "/p/_codev/changes/add-auth/change.yaml",
            "schema: spec-driven",
        );
        let default = project(&[change]);
        assert_eq!(
            instructions(&default, Some("proposal")).unwrap().language,
            "en"
        );
        let french = project(&[change, ("/p/_codev/config.yaml", "language: fr\n")]);
        assert_eq!(
            instructions(&french, Some("proposal")).unwrap().language,
            "fr"
        );
    }

    #[test]
    fn a_project_schema_without_instruction_nor_template_gives_null() {
        let fs = project(&[
            (
                "/p/_codev/schemas/lite/schema.yaml",
                "name: lite\nartifacts:\n  - id: brief\n    generates: brief.md\napply:\n  requires: [brief]\n  tracks: brief.md\n",
            ),
            ("/p/_codev/changes/add-auth/change.yaml", "schema: lite"),
        ]);
        let instr = instructions(&fs, Some("brief")).unwrap();
        assert_eq!(instr.instruction, None);
        assert_eq!(instr.template, None);
    }

    #[test]
    fn a_missing_project_template_fails_with_template_not_found() {
        let fs = project(&[
            (
                "/p/_codev/schemas/lite/schema.yaml",
                "name: lite\nartifacts:\n  - id: brief\n    generates: brief.md\n    template: brief.md\napply:\n  requires: [brief]\n  tracks: brief.md\n",
            ),
            ("/p/_codev/changes/add-auth/change.yaml", "schema: lite"),
        ]);
        let err = instructions(&fs, Some("brief")).unwrap_err();
        assert_eq!(err.code(), "template_not_found");
    }

    #[test]
    fn inherited_blocks_come_before_the_project_ones() {
        let fs = project(&[
            (
                "/home/shared/_codev/config.yaml",
                "context: |\n  Shared context\nrules:\n  specs:\n    - Shared rule\n",
            ),
            (
                "/p/_codev/config.yaml",
                "inherits:\n  - path: ~/shared\ncontext: |\n  Project context\nrules:\n  specs:\n    - Project rule\n",
            ),
            (
                "/p/_codev/changes/add-auth/change.yaml",
                "schema: spec-driven",
            ),
        ]);
        let instr = instructions(&fs, Some("specs")).unwrap();
        let blocks = |blocks: &[Block]| -> Vec<(String, String)> {
            blocks
                .iter()
                .map(|b| (b.origin.clone(), b.text.clone()))
                .collect()
        };
        assert_eq!(
            blocks(&instr.context),
            [
                ("path:~/shared".to_string(), "Shared context".to_string()),
                ("project".to_string(), "Project context".to_string()),
            ]
        );
        assert_eq!(
            blocks(&instr.rules),
            [
                ("path:~/shared".to_string(), "Shared rule".to_string()),
                ("project".to_string(), "Project rule".to_string()),
            ]
        );
    }

    const DECISIONS_SPEC: &str = "# Decisions\n\n## Purpose\n\nRecord decisions.\n\n## Requirements\n\n\
### Requirement: Creating a decision through the CLI\n\n`codev decision new` SHALL create a decision file from the command line.\n\n\
#### Scenario: Create\n\n- **WHEN** asked\n- **THEN** it creates\n\n\
### Requirement: Listing decisions\n\n`codev decision list` SHALL list them.\n\n\
#### Scenario: List\n\n- **WHEN** asked\n- **THEN** it lists\n";

    const DRAFT_PROPOSAL: &str = "# Proposal: Draft decisions\n\n## Why\n\nA user wants to create a decision as a draft \
from the command line.\n\n## What Changes\n\n- `codev decision new` creates a draft decision.\n\n## Impact\n\nNone.\n";

    fn drafting(proposal: Option<&str>) -> MemoryFileSystem {
        let mut files = vec![
            (
                "/p/_codev/changes/add-auth/change.yaml",
                "schema: spec-driven",
            ),
            ("/p/_codev/specs/decisions/spec.md", DECISIONS_SPEC),
        ];
        if let Some(text) = proposal {
            files.push(("/p/_codev/changes/add-auth/proposal.md", text));
        }
        project(&files)
    }

    #[test]
    fn instructions_name_the_requirements_the_proposal_touches() {
        let fs = drafting(Some(DRAFT_PROPOSAL));
        for artifact in ["specs", "design"] {
            let instr = instructions(&fs, Some(artifact)).unwrap();
            assert_eq!(
                instr.related[0].requirement, "Creating a decision through the CLI",
                "{artifact}"
            );
            assert_eq!(instr.related[0].capability, "decisions");
            assert!(
                instr.related[0]
                    .excerpt
                    .contains("SHALL create a decision file")
            );
            assert!(instr.related.len() <= RELATED_LIMIT);
        }
    }

    #[test]
    fn the_artifacts_that_do_not_require_the_proposal_read_no_spec() {
        let fs = drafting(Some(DRAFT_PROPOSAL));
        for artifact in ["proposal", "tasks"] {
            let instr = instructions(&fs, Some(artifact)).unwrap();
            assert!(instr.related.is_empty(), "{artifact}");
        }
        assert!(
            !fs.reads().iter().any(|p| p.starts_with("/p/_codev/specs")),
            "{:?}",
            fs.reads()
        );
    }

    #[test]
    fn no_related_requirements_before_the_proposal() {
        let instr = instructions(&drafting(None), Some("specs")).unwrap();
        assert!(instr.related.is_empty());
    }

    #[test]
    fn an_empty_proposal_ranks_nothing() {
        let instr = instructions(&drafting(Some("")), Some("specs")).unwrap();
        assert!(instr.related.is_empty());
        let instr = instructions(&drafting(Some("the of a")), Some("design")).unwrap();
        assert!(instr.related.is_empty());
    }

    #[test]
    fn no_related_requirements_without_specs() {
        let fs = project(&[
            (
                "/p/_codev/changes/add-auth/change.yaml",
                "schema: spec-driven",
            ),
            ("/p/_codev/changes/add-auth/proposal.md", DRAFT_PROPOSAL),
        ]);
        assert!(instructions(&fs, Some("specs")).unwrap().related.is_empty());
    }

    /// A file system whose reads of one path fail.
    struct Unreadable<'a> {
        inner: &'a MemoryFileSystem,
        path: &'a str,
    }

    impl FileSystem for Unreadable<'_> {
        fn exists(&self, path: &std::path::Path) -> bool {
            self.inner.exists(path)
        }
        fn read_to_string(&self, path: &std::path::Path) -> std::io::Result<String> {
            if path == std::path::Path::new(self.path) {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::PermissionDenied,
                    "denied",
                ));
            }
            self.inner.read_to_string(path)
        }
        fn len(&self, path: &std::path::Path) -> std::io::Result<u64> {
            self.inner.len(path)
        }
        fn is_regular_file(&self, path: &std::path::Path) -> bool {
            self.inner.is_regular_file(path)
        }
        fn write(&self, path: &std::path::Path, contents: &str) -> std::io::Result<()> {
            self.inner.write(path, contents)
        }
        fn create_dir_all(&self, path: &std::path::Path) -> std::io::Result<()> {
            self.inner.create_dir_all(path)
        }
        fn walk_files(&self, dir: &std::path::Path) -> std::io::Result<Vec<String>> {
            self.inner.walk_files(dir)
        }
        fn list_dir(&self, dir: &std::path::Path) -> std::io::Result<Vec<String>> {
            self.inner.list_dir(dir)
        }
        fn rename(&self, from: &std::path::Path, to: &std::path::Path) -> std::io::Result<()> {
            self.inner.rename(from, to)
        }
        fn remove_file(&self, path: &std::path::Path) -> std::io::Result<()> {
            self.inner.remove_file(path)
        }
    }

    #[test]
    fn an_unreadable_spec_is_left_out_with_a_warning() {
        let mem = drafting(Some(DRAFT_PROPOSAL)).with_file("/p/_codev/specs/billing/spec.md", "x");
        let fs = Unreadable {
            inner: &mem,
            path: "/p/_codev/specs/billing/spec.md",
        };
        let layout = Layout::new("/p");
        let e = env();
        let config = crate::config::resolve(&fs, &e, &layout).unwrap();
        let ctx =
            change::load(&fs, &layout, &config, ChangeId::parse("add-auth").unwrap()).unwrap();
        let instr = for_artifact(&fs, &e, &layout, &ctx, &config, Some("specs")).unwrap();
        assert_eq!(instr.related[0].capability, "decisions");
        assert!(
            instr
                .warnings
                .iter()
                .any(|w| w.code == "spec_unreadable" && w.message.contains("billing")),
            "{:?}",
            instr.warnings
        );
    }

    #[test]
    fn french_proposal_template_holds_french_keywords() {
        let change = (
            "/p/_codev/changes/add-auth/change.yaml",
            "schema: spec-driven",
        );
        let french = project(&[change, ("/p/_codev/config.yaml", "language: fr\n")]);
        let got = instructions(&french, Some("proposal")).unwrap();
        assert_eq!(got.keyword_language.code(), "fr");
        let template = got.template.unwrap();
        assert!(template.contains("## Changements"), "{template}");
        assert!(template.contains("## Domaines"), "{template}");
    }

    #[test]
    fn french_spec_template_holds_french_keywords() {
        let change = (
            "/p/_codev/changes/add-auth/change.yaml",
            "schema: spec-driven",
        );
        let french = project(&[
            change,
            (
                "/p/_codev/changes/add-auth/proposal.md",
                "# Proposition : X\n",
            ),
            ("/p/_codev/config.yaml", "language: fr-CA\n"),
        ]);
        let template = instructions(&french, Some("specs"))
            .unwrap()
            .template
            .unwrap();
        assert!(template.contains("### Exigence :"), "{template}");
        assert!(template.contains("DOIT"), "{template}");
    }

    #[test]
    fn a_single_english_template_keeps_english_keywords() {
        let schema = "name: lite\nartifacts:\n  - id: brief\n    generates: brief.md\n    template: brief.md\napply:\n  requires: [brief]\n  tracks: brief.md\n";
        let files = |template: &'static str| {
            project(&[
                ("/p/_codev/schemas/lite/schema.yaml", schema),
                ("/p/_codev/schemas/lite/templates/brief.md", template),
                ("/p/_codev/changes/add-auth/change.yaml", "schema: lite"),
                ("/p/_codev/config.yaml", "language: fr\n"),
            ])
        };
        let english = instructions(&files("# Brief\n\n## What Changes\n"), Some("brief")).unwrap();
        assert_eq!(english.language, "fr");
        assert_eq!(english.keyword_language.code(), "en");
        let plain = instructions(&files("Free text.\n"), Some("brief")).unwrap();
        assert_eq!(plain.keyword_language.code(), "fr");
    }

    #[test]
    fn a_language_without_catalog_gets_the_english_template() {
        let change = (
            "/p/_codev/changes/add-auth/change.yaml",
            "schema: spec-driven",
        );
        let brazil = project(&[change, ("/p/_codev/config.yaml", "language: pt-BR\n")]);
        let got = instructions(&brazil, Some("proposal")).unwrap();
        assert_eq!(got.language, "pt-BR");
        assert_eq!(got.keyword_language.code(), "en");
        assert!(got.template.unwrap().contains("## What Changes"));
    }
}
