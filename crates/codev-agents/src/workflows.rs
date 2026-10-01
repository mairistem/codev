use codev_engine::Warning;

/// A workflow: the body of a skill, plus what it takes to declare it.
///
/// The body is a markdown file from `assets/workflows/`, included at compile
/// time. It stays **data**: improving it does not require touching the
/// code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Workflow {
    pub id: &'static str,
    /// What the agent reads to decide whether this skill applies. It is the
    /// most important sentence in the file: a vague description and the skill
    /// never triggers.
    pub description: &'static str,
    /// The tools the workflow needs to use.
    ///
    /// Also serves as a guardrail: `explore` does not get `Write`, which makes
    /// its promise to write nothing structural rather than declarative.
    pub allowed_tools: &'static str,
    pub body: &'static str,
}

/// The workflows shipped by this version.
///
/// The catalog only contains what the CLI can actually serve: a workflow
/// whose commands do not exist yet would produce a skill that fails in front
/// of the user. Adding a workflow means adding an asset file and an entry
/// here.
pub const CATALOG: &[Workflow] = &[
    Workflow {
        id: "propose",
        description: "Create a codev change and write all its planning artifacts in one pass \
                      — proposal, specs, design, tasks. Use when the user describes what they \
                      want to build or fix and a plan ready for implementation is needed. \
                      Modifies no code. Detects a ticket identifier (pattern [A-Z]{2,}-\\d+) \
                      mentioned in the prompt and enriches the proposal through the Jira MCP \
                      configured for the project, if available.",
        // `{{JIRA_MCP_TOOL}}` at the end of the list: a placeholder substituted
        // at rendering time with the name of the Jira MCP tool declared in
        // `_codev/config.yaml.mcp.jira_tool`. Without config, the placeholder
        // (and the comma preceding it) are removed cleanly — see
        // `codev-agents::claude::substitute_jira_mcp`. The skill stays
        // read-only on Jira: a single declared tool, never a write.
        allowed_tools: "Bash(codev:*), Read, Write, Edit, Glob, Grep, {{JIRA_MCP_TOOL}}",
        body: include_str!("../../../assets/workflows/propose.md"),
    },
    Workflow {
        id: "explore",
        description: "Explore an idea, investigate a problem or clarify a need before \
                      creating a codev change. Use when the request is vague, when several \
                      approaches need comparing, or when it is not yet clear what to build. \
                      Writes no files.",
        allowed_tools: "Bash(codev:*), Read, Glob, Grep",
        body: include_str!("../../../assets/workflows/explore.md"),
    },
    Workflow {
        id: "apply",
        description: "Implement the tasks of an already planned codev change: read tasks.md, \
                      work through each unchecked box in order, checking them off along the \
                      way. Modifies project code. Does not modify other changes, does not \
                      archive and does not sync — those steps stay explicit on the user's side.",
        // General `Bash` in addition to `Bash(codev:*)`: tasks cite
        // verification commands (cargo, git, npm, python…) that must be
        // runnable. The `Bash(codev:*)` prefix stays first to document the
        // main usage.
        allowed_tools: "Bash(codev:*), Read, Write, Edit, Glob, Grep, Bash",
        body: include_str!("../../../assets/workflows/apply.md"),
    },
    Workflow {
        id: "sync",
        description: "Merge the deltas of an already planned codev change into the main \
                      specs, without moving the change. Use when a new capability must appear \
                      in the specs before another change consumes it, or to review the merge \
                      before archiving. Does not archive.",
        // `Read` in addition to `Bash(codev:*)` so the skill can re-read
        // `tasks.md` if the user asks a context question — without ever
        // writing. No general `Bash`: the only effective action goes through
        // `codev`.
        allowed_tools: "Bash(codev:*), Read",
        body: include_str!("../../../assets/workflows/sync.md"),
    },
    Workflow {
        id: "archive",
        description: "Close a codev change: merge its deltas into the main specs and move the \
                      folder to the dated archive. Refuses to act if validation reports \
                      errors, and then points to `codev validate` for the details.",
        allowed_tools: "Bash(codev:*), Read",
        body: include_str!("../../../assets/workflows/archive.md"),
    },
    Workflow {
        id: "update",
        description: "Revise an already written planning artifact of an active codev change — \
                      proposal, specs, design or tasks — while keeping it consistent with the \
                      other artifacts. Modifies no project code, creates no missing artifact, \
                      touches no archived change.",
        // Direct markdown editing via Edit/Write, no general Bash: the skill
        // revises text, it does not run tests. The rule "only `apply` has
        // general Bash" still holds.
        allowed_tools: "Bash(codev:*), Read, Write, Edit, Glob, Grep",
        body: include_str!("../../../assets/workflows/update.md"),
    },
    Workflow {
        id: "onboard",
        description: "Introduce codev to a user discovering it: what the tool does, the \
                      current state of the project, and the recommended next action. Strictly \
                      read-only — modifies and creates nothing.",
        // Read-only: the role is to guide, never to act on the user's behalf.
        // No Write, no Edit, no Grep (known paths), no general Bash — the
        // rule "only `apply` has general Bash" still holds.
        allowed_tools: "Bash(codev:*), Read, Glob",
        body: include_str!("../../../assets/workflows/onboard.md"),
    },
    Workflow {
        id: "configure",
        description: "Enrich a project's `_codev/config.yaml` by analyzing its code: read \
                      README, CONTRIBUTING, docs and a sample of sources, then propose a \
                      detailed `context:` and per-artifact `rules:`. Shows a diff, writes only \
                      on confirmation. Never touches the workflows, the MCPs or the schema.",
        // Targeted editing of a single file (_codev/config.yaml) via Edit,
        // with Read/Glob/Grep to explore the project. No general Bash: the
        // skill runs no tests, no git, no external tool — the rule "only
        // `apply` has general Bash" still holds.
        allowed_tools: "Bash(codev:*), Read, Write, Edit, Glob, Grep",
        body: include_str!("../../../assets/workflows/configure.md"),
    },
];

/// The workflows installed when the configuration names none.
///
/// The default catalog covers the **entire** codev cycle: a user who
/// installs the tool gets everything needed to propose, implement, validate
/// and archive in one go — plus `configure`, the recommended entry point
/// after `codev init` to enrich `_codev/config.yaml`. A project that wants
/// to restrict the list declares `workflows:` explicitly in its
/// `_codev/config.yaml` (opt-out path).
pub const DEFAULT_WORKFLOWS: &[&str] = &[
    "propose",
    "explore",
    "onboard",
    "apply",
    "sync",
    "archive",
    "update",
    "configure",
];

pub fn find(id: &str) -> Option<&'static Workflow> {
    CATALOG.iter().find(|w| w.id == id)
}

/// Resolves the requested list into known workflows.
///
/// An unknown identifier produces a warning, not an error: a typo in
/// `config.yaml` must not prevent the other skills from being installed, but
/// it must not go unnoticed either.
pub fn select(requested: Option<&[String]>) -> (Vec<&'static Workflow>, Vec<Warning>) {
    let mut warnings = Vec::new();
    let ids: Vec<String> = match requested {
        Some(ids) => ids.to_vec(),
        None => DEFAULT_WORKFLOWS.iter().map(|s| s.to_string()).collect(),
    };

    let mut workflows = Vec::new();
    for id in ids {
        match find(&id) {
            Some(workflow) if !workflows.contains(&workflow) => workflows.push(workflow),
            Some(_) => {}
            None => warnings.push(Warning::new(
                "unknown_workflow",
                format!(
                    "unknown workflow `{id}` ignored; this version provides: {}",
                    CATALOG.iter().map(|w| w.id).collect::<Vec<_>>().join(", ")
                ),
            )),
        }
    }
    (workflows, warnings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_workflow_has_a_usable_body_and_description() {
        for workflow in CATALOG {
            assert!(
                workflow.body.len() > 200,
                "the body of `{}` is suspiciously short",
                workflow.id
            );
            assert!(
                workflow.description.len() > 60,
                "the description of `{}` is too vague to trigger the skill",
                workflow.id
            );
            assert!(
                workflow.allowed_tools.contains("Bash(codev:*)"),
                "`{}` must be able to call the CLI",
                workflow.id
            );
        }
    }

    #[test]
    fn explore_cannot_write() {
        // Its "writes no files" promise must be structural.
        let explore = find("explore").unwrap();
        assert!(!explore.allowed_tools.contains("Write"));
        assert!(!explore.allowed_tools.contains("Edit"));
    }

    #[test]
    fn cycle_completion_skills_present_and_restricted() {
        // The two cycle-closing skills: present, with `allowed-tools`
        // strictly `Bash(codev:*), Read` — no general `Bash`, no
        // `Write`/`Edit`. This makes the skill unable to modify a file on
        // its own: everything goes through the CLI.
        for id in ["sync", "archive"] {
            let workflow = find(id).unwrap_or_else(|| panic!("`{id}` must be in the CATALOG"));
            assert_eq!(
                workflow.allowed_tools, "Bash(codev:*), Read",
                "`{id}` must stick to Bash(codev:*), Read"
            );
            assert!(!workflow.allowed_tools.contains("Write"));
            assert!(!workflow.allowed_tools.contains("Edit"));
            assert!(!workflow.allowed_tools.ends_with(", Bash"));
        }
    }

    #[test]
    fn sync_and_archive_cite_the_contract_fields() {
        // The skills explicitly name the public contract fields they
        // consume. A field rename in `contract.rs` must surface here via
        // `grep`, rather than become a silent failure at runtime.
        let sync = find("sync").unwrap();
        for field in [
            "SyncReportV1",
            "changeName",
            "created",
            "updated",
            "unchanged",
        ] {
            assert!(
                sync.body.contains(field),
                "sync must cite `{field}` by name"
            );
        }
        let archive = find("archive").unwrap();
        for field in ["ArchiveReportV1", "movedTo", "validation_failed", "status"] {
            assert!(
                archive.body.contains(field),
                "archive must cite `{field}` by name"
            );
        }
    }

    #[test]
    fn update_is_in_the_catalog_with_the_right_tools() {
        let update = find("update").expect("update must be in the CATALOG");
        // Markdown editing: Write and Edit required. Bash(codev:*) only —
        // no general Bash, the rule "only apply has it" is preserved.
        assert_eq!(
            update.allowed_tools,
            "Bash(codev:*), Read, Write, Edit, Glob, Grep"
        );
        assert!(!update.allowed_tools.ends_with(", Bash"));
    }

    #[test]
    fn update_states_its_boundaries() {
        // Tracer for an accidental rename or removal of the guardrails in
        // the skill body.
        let update = find("update").unwrap();
        for boundary in ["not modify", "archived"] {
            assert!(
                update.body.contains(boundary),
                "the `update` body must state the boundary `{boundary}`"
            );
        }
    }

    #[test]
    fn apply_is_in_the_catalog_with_the_right_tools() {
        let apply = find("apply").expect("apply must be in the CATALOG");
        // The `Bash(codev:*)` prefix first — main usage — and general
        // `Bash` last — for verification commands. It is the ONLY workflow
        // to request the latter, a guardrail against silently widening it
        // to others.
        assert!(apply.allowed_tools.starts_with("Bash(codev:*)"));
        assert!(apply.allowed_tools.contains(", Bash"));
        assert!(apply.allowed_tools.contains("Edit"));

        let others_with_general_bash = CATALOG
            .iter()
            .filter(|w| w.id != "apply")
            .filter(|w| w.allowed_tools.ends_with(", Bash") || w.allowed_tools == "Bash")
            .count();
        assert_eq!(
            others_with_general_bash, 0,
            "only `apply` may have general Bash — otherwise the restriction promise is lost"
        );
    }

    #[test]
    fn without_request_installs_the_default_catalog() {
        // The default catalog covers the 8 workflows of the codev cycle: a
        // user who just installed the tool gets everything needed to
        // propose, implement, validate and archive in one go, plus
        // `configure` to enrich the config. A project that wants to restrict
        // the list goes through an explicit `workflows:` in
        // `_codev/config.yaml` — opt-out path.
        let (workflows, warnings) = select(None);
        let ids: Vec<_> = workflows.iter().map(|w| w.id).collect();
        assert_eq!(
            ids,
            [
                "propose",
                "explore",
                "onboard",
                "apply",
                "sync",
                "archive",
                "update",
                "configure"
            ]
        );
        assert_eq!(ids.as_slice(), DEFAULT_WORKFLOWS);
        assert!(warnings.is_empty());
    }

    #[test]
    fn configure_is_in_the_catalog_with_the_right_tools() {
        // `configure` has Write and Edit (it modifies a file), but NOT
        // general Bash — it runs no tests; only `apply` has that right.
        let configure = find("configure").expect("configure must be in the CATALOG");
        assert!(configure.allowed_tools.contains("Write"));
        assert!(configure.allowed_tools.contains("Edit"));
        assert!(!configure.allowed_tools.ends_with(", Bash"));
        assert!(configure.allowed_tools.starts_with("Bash(codev:*)"));
        // The body must explicitly cite the preserved fields — it is the
        // skill's structural promise.
        assert!(
            configure.body.contains("schema") && configure.body.contains("workflows"),
            "the body must list the preserved fields (schema, workflows, mcp, inherits)"
        );
    }

    #[test]
    fn opt_out_restriction_via_explicit_workflows() {
        // A project that declares `workflows: [propose]` gets only
        // `propose`, never the others. It is the counterpart of the broad
        // default: opt-out by explicit declaration.
        let requested = vec!["propose".to_string()];
        let (workflows, warnings) = select(Some(&requested));
        let ids: Vec<_> = workflows.iter().map(|w| w.id).collect();
        assert_eq!(ids, ["propose"]);
        assert!(warnings.is_empty());
    }

    #[test]
    fn onboard_is_in_the_catalog_with_the_right_tools() {
        let onboard = find("onboard").expect("onboard must be in the CATALOG");
        // Read-only: Bash(codev:*), Read, Glob — no Write/Edit, no general
        // Bash, no Grep (known paths).
        assert_eq!(onboard.allowed_tools, "Bash(codev:*), Read, Glob");
        assert!(!onboard.allowed_tools.contains("Write"));
        assert!(!onboard.allowed_tools.contains("Edit"));
        assert!(!onboard.allowed_tools.ends_with(", Bash"));
        // The body must mention the "thin config → /codev-configure" branch
        // — a tracer that it is not removed by accident in a rework.
        assert!(
            onboard.body.contains("/codev-configure"),
            "the onboard body must cite /codev-configure as the recommendation on a thin config"
        );
        assert!(
            onboard.body.contains("thin"),
            "the onboard body must describe thin detection"
        );
        assert!(
            !onboard.body.contains("200 characters") && !onboard.body.contains("< 200"),
            "the onboard body must no longer mention the removed 200-character threshold"
        );
    }

    #[test]
    fn onboard_is_in_the_default_catalog() {
        // `onboard` is part of the default catalog — that is its very role:
        // welcoming a user who has configured nothing.
        assert!(DEFAULT_WORKFLOWS.contains(&"onboard"));
    }

    #[test]
    fn onboard_cites_its_three_blocks() {
        // Tracer for an accidental rework of the body: the skill promises
        // three blocks (description, state, next step/action). These
        // keywords must stay present.
        let onboard = find("onboard").unwrap();
        for keyword in ["codev is", "here you have", "what's next"] {
            assert!(
                onboard
                    .body
                    .to_lowercase()
                    .contains(&keyword.to_lowercase()),
                "the `onboard` body must cite the block `{keyword}`"
            );
        }
    }

    #[test]
    fn an_unknown_workflow_warns_without_blocking_the_others() {
        let requested = vec!["propose".to_string(), "teleportation".to_string()];
        let (workflows, warnings) = select(Some(&requested));
        assert_eq!(
            workflows.iter().map(|w| w.id).collect::<Vec<_>>(),
            ["propose"]
        );
        assert_eq!(warnings.len(), 1);
        assert_eq!(warnings[0].code, "unknown_workflow");
        assert!(
            warnings[0].message.contains("propose"),
            "the message must list what exists"
        );
    }

    #[test]
    fn deduplicates_a_repeated_request() {
        let requested = vec!["propose".to_string(), "propose".to_string()];
        let (workflows, _) = select(Some(&requested));
        assert_eq!(workflows.len(), 1);
    }

    // ─────────────── Atlassian MCP on propose (first integration) ───────────────

    #[test]
    fn propose_uses_a_placeholder_for_the_jira_mcp() {
        // The CATALOG no longer hardcodes any MCP name: rendering
        // substitutes `{{JIRA_MCP_TOOL}}` from the project config. Locks in
        // the placeholder's presence in both expected places:
        // `allowed_tools` and body.
        let propose = find("propose").expect("propose must be in the CATALOG");
        assert!(
            propose.allowed_tools.contains("{{JIRA_MCP_TOOL}}"),
            "allowed_tools must contain the placeholder for substitution"
        );
        assert!(
            propose.body.contains("{{JIRA_MCP_TOOL}}"),
            "the body must cite the placeholder so that substitution \
             injects the tool name into the agent's instructions"
        );
    }

    #[test]
    fn propose_does_not_hardcode_an_mcp_name() {
        // Guardrail against a hardcoding regression. No `mcp__...` string
        // may appear in the CATALOG — everything goes through the
        // placeholder.
        let propose = find("propose").unwrap();
        assert!(
            !propose.allowed_tools.contains("mcp__"),
            "no MCP name may be hardcoded in allowed_tools"
        );
        assert!(
            !propose.body.contains("mcp__"),
            "no MCP name may be hardcoded in the body"
        );
    }

    #[test]
    fn propose_body_describes_ticket_detection() {
        // Tracer for an accidental rework of the body: the pattern and
        // placeholder keywords must stay present so that the logic described
        // stays visible to the agent reading the skill.
        let propose = find("propose").unwrap();
        for keyword in ["[A-Z]{2,}-\\d+", "{{JIRA_MCP_TOOL}}"] {
            assert!(
                propose.body.contains(keyword),
                "the `propose` body must cite `{keyword}`"
            );
        }
    }

    #[test]
    fn propose_body_carries_the_review_steps() {
        // The review steps and the output contract of the plan are what the
        // agent follows; a rework of the body must not drop them silently.
        let propose = find("propose").unwrap();
        for keyword in [
            "### 6. Check traceability",
            "### 7. Challenge the plan",
            "edge-case",
            "Impact touches them",
            "Points to challenge",
            "No point to challenge found",
            "it never adds scope",
            "capped at five",
        ] {
            assert!(
                propose.body.contains(keyword),
                "the `propose` body must cite `{keyword}`"
            );
        }
    }
}
