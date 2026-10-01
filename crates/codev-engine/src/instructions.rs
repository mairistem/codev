use std::path::PathBuf;

use codev_core::decisions::DecisionStatus;
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
    /// Structural keywords of the template stay in English whatever it is.
    pub language: String,
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
    /// True when the change has disabled this artifact: it must **not** be
    /// created.
    pub skipped: bool,
    pub warnings: Vec<Warning>,
}

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

    Ok(Instructions {
        change: ctx.change.clone(),
        schema_name: ctx.schema.name().to_string(),
        artifact_id: artifact.id.clone(),
        description: artifact.description.clone(),
        resolved_output_path: change_dir.join(&artifact.generates),
        instruction: artifact.instruction.clone(),
        template: ctx.schema.template(fs, artifact)?,
        language: config.language.clone(),
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
        skipped: state == ArtifactState::Skipped,
        warnings,
    })
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

    #[test]
    fn spec_driven_instructions_start_with_a_role_block() {
        let fs = project(&[(
            "/p/_codev/changes/add-auth/change.yaml",
            "schema: spec-driven",
        )]);
        for (artifact, role, criterion) in [
            ("proposal", "Role: product owner", "**BREAKING**"),
            ("specs", "Role: QA analyst", "edge-case scenario"),
            ("design", "Role: architect", "alternative it rejects"),
            ("tasks", "Role: tech lead", "every spec scenario is covered"),
        ] {
            let instruction = instructions(&fs, Some(artifact))
                .unwrap()
                .instruction
                .unwrap_or_default();
            assert!(
                instruction.starts_with(role),
                "`{artifact}` must start with `{role}`"
            );
            let (_, criteria) = instruction
                .split_once("Done when:")
                .unwrap_or_else(|| panic!("`{artifact}` must carry a `Done when:` list"));
            assert!(
                criteria.contains(criterion),
                "`{artifact}` criteria must mention `{criterion}`"
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
}
