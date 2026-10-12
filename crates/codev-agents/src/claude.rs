use std::path::{Path, PathBuf};

use codev_core::doctor::SkillState;
use codev_core::managed::SpliceMode;
use codev_core::{Plan, WriteMode};
use codev_engine::FileSystem;
use codev_engine::integration::{self, IntegrationPlan, Step};

use crate::target::{AgentTarget, SkillsPlan};
use crate::workflows::Workflow;

/// Claude Code.
///
/// A skill placed in `.claude/skills/<name>/SKILL.md` is discovered
/// automatically **and** can be invoked by the user by typing `/<name>`. That
/// is why codev does not generate separate command files: they would be two
/// files to keep consistent for a single workflow.
/// See `_codev/decisions/0004-single-identity-for-skill-and-command.md`.
///
/// The instance carries a `RenderCtx`: the CLI feeds it from the project
/// config (`_codev/config.yaml.mcp.jira_tool`, …), and it feeds placeholder
/// substitution at rendering time.
#[derive(Debug, Default)]
pub struct ClaudeCode {
    ctx: RenderCtx,
}

impl ClaudeCode {
    /// Creates an instance with an empty render context — used when no
    /// project config is available (tests, low-level scaffolding path).
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an instance with the given context. This is the path used by
    /// the CLI, fed by `ResolvedConfig.mcp`.
    pub fn with_ctx(ctx: RenderCtx) -> Self {
        Self { ctx }
    }
}

/// Render context for a skill's frontmatter.
///
/// Carries the project-specific information that placeholder substitution
/// needs. Today: the name of the Jira MCP tool. A future MCP (Design,
/// Confluence…) will add its own field.
///
/// `Default` returns all fields as `None` — used by the tests and the
/// "project without a configured MCP" path.
#[derive(Debug, Clone, Default)]
pub struct RenderCtx {
    /// Name of the Jira MCP tool to inject into the skills. Fed by
    /// `_codev/config.yaml.mcp.jira_tool`.
    pub jira_mcp_tool: Option<String>,
}

/// Text placeholder substituted in `allowed_tools` and in the body of each
/// workflow. Chosen so that it does not occur naturally in a markdown body.
const JIRA_MCP_PLACEHOLDER: &str = "{{JIRA_MCP_TOOL}}";

/// Fallback shown in the body when no Jira MCP is configured — the agent
/// reading the skill sees clearly that ticket detection will not succeed.
const JIRA_MCP_FALLBACK_BODY: &str = "(Jira MCP not configured)";

/// Substitutes the `{{JIRA_MCP_TOOL}}` placeholder according to the config.
///
/// Two passes when the config is absent: first remove
/// `, {{JIRA_MCP_TOOL}}` (with the comma preceding it, so that
/// `allowed_tools` does not end with `", "`), then replace whatever remains
/// with the text fallback (useful in the body).
pub fn substitute_jira_mcp(source: &str, jira_tool: Option<&str>) -> String {
    match jira_tool {
        Some(tool) => source.replace(JIRA_MCP_PLACEHOLDER, tool),
        None => source
            .replace(&format!(", {JIRA_MCP_PLACEHOLDER}"), "")
            .replace(JIRA_MCP_PLACEHOLDER, JIRA_MCP_FALLBACK_BODY),
    }
}

const SKILLS_ROOT: &str = ".claude/skills";

/// Skill name prefix. `propose` becomes `codev-propose`, invocable as
/// `/codev-propose`.
const SKILL_PREFIX: &str = "codev";

impl ClaudeCode {
    pub fn ctx(&self) -> &RenderCtx {
        &self.ctx
    }

    pub fn skill_name(workflow_id: &str) -> String {
        format!("{SKILL_PREFIX}-{workflow_id}")
    }

    pub fn skill_file(project_root: &Path, workflow_id: &str) -> PathBuf {
        project_root
            .join(SKILLS_ROOT)
            .join(Self::skill_name(workflow_id))
            .join("SKILL.md")
    }

    /// Renders the complete `SKILL.md`: generated frontmatter, body as is.
    ///
    /// `ctx` carries the project-specific substitutions (Jira MCP name, …).
    /// A `RenderCtx::default()` renders "without MCP" — the placeholder is
    /// removed cleanly and replaced by an informative fallback in the body.
    pub fn render(workflow: &Workflow, version: &str, ctx: &RenderCtx) -> String {
        let tools = substitute_jira_mcp(workflow.allowed_tools, ctx.jira_mcp_tool.as_deref());
        let body = substitute_jira_mcp(workflow.body, ctx.jira_mcp_tool.as_deref());
        format!(
            "---\n\
             name: {name}\n\
             description: {description}\n\
             allowed-tools: {tools}\n\
             license: MIT\n\
             metadata:\n\
             \x20 generator: codev\n\
             \x20 version: \"{version}\"\n\
             ---\n\
             \n\
             {body}",
            name = Self::skill_name(workflow.id),
            description = yaml_scalar(workflow.description),
            tools = yaml_scalar(&tools),
            body = body.trim_end(),
        )
    }
}

impl AgentTarget for ClaudeCode {
    fn id(&self) -> &'static str {
        "claude"
    }

    fn label(&self) -> &'static str {
        "Claude Code"
    }

    fn detect(&self, fs: &dyn FileSystem, project_root: &Path) -> bool {
        fs.exists(&project_root.join(".claude")) || fs.exists(&project_root.join("CLAUDE.md"))
    }

    fn plan_skills(
        &self,
        fs: &dyn FileSystem,
        project_root: &Path,
        workflows: &[&Workflow],
        version: &str,
        force: bool,
    ) -> SkillsPlan {
        let mut plan = Plan::new();
        let mut preserved = Vec::new();

        for workflow in workflows {
            let path = Self::skill_file(project_root, workflow.id);
            let desired = Self::render(workflow, version, &self.ctx);

            // Same stamped version, different content: someone edited the
            // file by hand. Overwriting it would lose their work without
            // warning; report it and move on.
            let edited_by_hand = !force
                && fs.exists(&path)
                && fs.read_to_string(&path).is_ok_and(|current| {
                    current != desired && stamped_version(&current).as_deref() == Some(version)
                });
            if edited_by_hand {
                preserved.push(path);
                continue;
            }

            plan.dir(path.parent().unwrap_or(project_root));
            plan.write(path, desired, WriteMode::Overwrite);
        }

        SkillsPlan { plan, preserved }
    }
}

/// The `CLAUDE.md` file Claude Code reads at the project root.
pub const CLAUDE_MD: &str = "CLAUDE.md";

impl ClaudeCode {
    /// The body of codev's managed section in `CLAUDE.md`.
    ///
    /// Written for Claude Code, in English whatever the artifact language:
    /// short, imperative, naming only skills that are installed.
    pub fn claude_md_body(skills: &[String]) -> String {
        let has = |name: &str| skills.iter().any(|s| s == name);
        let implement = if has("codev-apply") {
            "then implement it with `/codev-apply`"
        } else {
            "then implement it by following the change's `tasks.md`"
        };
        let propose = if has("codev-propose") {
            "start with `/codev-propose`"
        } else {
            "start by creating a codev change (`codev new change <name>`)"
        };
        let mut body = String::from("## Spec-driven development with codev\n\n");
        body.push_str("This repository follows spec-driven development with codev.\n\n");
        body.push_str("- `_codev/specs/` describes the current behavior and is authoritative.\n");
        body.push_str(
            "- `_codev/decisions/` holds accepted architecture decisions: follow them, do not re-debate them.\n",
        );
        body.push_str(&format!(
            "- For any change of behavior, {propose}, {implement}. Do not modify behavior without a codev change.\n"
        ));
        body.push_str("- Trivial fixes (typos, documentation wording) are exempt.\n");
        body.push_str("- Run `codev validate` before committing.\n");
        if !skills.is_empty() {
            let list: Vec<String> = skills.iter().map(|s| format!("`/{s}`")).collect();
            body.push_str(&format!("\nInstalled skills: {}.\n", list.join(", ")));
        }
        body
    }

    /// Plans codev's section in `CLAUDE.md`: created or appended by `init`
    /// (`SpliceMode::Init`), only refreshed by `update`
    /// (`SpliceMode::Refresh`).
    pub fn plan_claude_md(
        fs: &dyn FileSystem,
        project_root: &Path,
        skills: &[String],
        mode: SpliceMode,
    ) -> IntegrationPlan {
        let existing = fs.read_to_string(&project_root.join(CLAUDE_MD)).ok();
        integration::plan_managed_markdown(
            project_root,
            Path::new(CLAUDE_MD),
            existing.as_deref(),
            &Self::claude_md_body(skills),
            mode,
            Step::ClaudeMd,
        )
    }

    /// The state of each skill of `workflows`: missing, stamped with
    /// another version, or current. Read-only — for `codev doctor`.
    pub fn skill_states(
        fs: &dyn FileSystem,
        project_root: &Path,
        workflows: &[&Workflow],
        version: &str,
    ) -> Vec<(String, SkillState)> {
        workflows
            .iter()
            .map(|w| {
                let state = match fs.read_to_string(&Self::skill_file(project_root, w.id)) {
                    Err(_) => SkillState::Missing,
                    Ok(contents) => match stamped_version(&contents) {
                        Some(v) if v == version => SkillState::Current,
                        other => SkillState::Outdated(other),
                    },
                };
                (Self::skill_name(w.id), state)
            })
            .collect()
    }
}

/// Renders a scalar that is safe for YAML frontmatter.
///
/// Descriptions contain colons and commas, which change the meaning of a
/// bare scalar. Always quote rather than guess case by case.
fn yaml_scalar(raw: &str) -> String {
    let escaped = raw.replace('\\', "\\\\").replace('"', "\\\"");
    format!("\"{escaped}\"")
}

/// Extracts the version stamped in a `SKILL.md` frontmatter.
///
/// This stamp is what distinguishes "file from an earlier version, to
/// regenerate" from "file of the current version that the user modified, to
/// preserve".
fn stamped_version(contents: &str) -> Option<String> {
    if !contents.starts_with("---") {
        return None;
    }
    contents
        .lines()
        .skip(1)
        .take_while(|line| *line != "---")
        .find_map(|line| line.trim().strip_prefix("version:"))
        .map(|value| value.trim().trim_matches('"').to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use codev_engine::ports::MemoryFileSystem;

    use crate::workflows;

    const VERSION: &str = "0.1.0";

    fn propose() -> &'static Workflow {
        workflows::find("propose").unwrap()
    }

    #[test]
    fn the_skill_name_is_the_slash_command() {
        assert_eq!(ClaudeCode::skill_name("propose"), "codev-propose");
        assert_eq!(
            ClaudeCode::skill_file(Path::new("/p"), "propose"),
            PathBuf::from("/p/.claude/skills/codev-propose/SKILL.md")
        );
    }

    #[test]
    fn the_frontmatter_is_complete_and_the_body_intact() {
        let rendered = ClaudeCode::render(propose(), VERSION, &RenderCtx::default());

        assert!(rendered.starts_with("---\n"));
        assert!(rendered.contains("name: codev-propose\n"));
        // Rendered without a configured MCP: the placeholder and the comma
        // preceding it have been removed.
        assert!(
            rendered.contains("allowed-tools: \"Bash(codev:*), Read, Write, Edit, Glob, Grep\"\n")
        );
        assert!(rendered.contains("  version: \"0.1.0\"\n"));
        assert!(
            rendered.contains("Planning boundary"),
            "the workflow body must be present as is"
        );
    }

    #[test]
    fn every_skill_frontmatter_is_valid_yaml() {
        // The guardrail that matters: a description containing a colon, an
        // em dash or an apostrophe must not produce a frontmatter that
        // Claude Code refuses to read — and the error would be silent, the
        // skill simply missing.
        for workflow in workflows::CATALOG {
            let rendered = ClaudeCode::render(workflow, VERSION, &RenderCtx::default());
            let frontmatter = rendered
                .split("---\n")
                .nth(1)
                .unwrap_or_else(|| panic!("`{}` has no frontmatter", workflow.id));

            let parsed: serde_norway::Value = serde_norway::from_str(frontmatter)
                .unwrap_or_else(|e| panic!("unreadable frontmatter for `{}`: {e}", workflow.id));

            assert_eq!(
                parsed["name"].as_str(),
                Some(ClaudeCode::skill_name(workflow.id).as_str()),
                "the skill name must match its folder"
            );
            assert_eq!(parsed["metadata"]["version"].as_str(), Some(VERSION));
            assert!(parsed["description"].as_str().is_some_and(|d| d.len() > 60));
        }
    }

    #[test]
    fn the_description_is_quoted_because_it_contains_yaml_punctuation() {
        let rendered = ClaudeCode::render(propose(), VERSION, &RenderCtx::default());
        let line = rendered
            .lines()
            .find(|l| l.starts_with("description:"))
            .unwrap();
        assert!(
            line.starts_with("description: \"") && line.ends_with('"'),
            "{line}"
        );
    }

    #[test]
    fn installs_the_requested_skills() {
        let fs = MemoryFileSystem::new();
        let workflows = vec![propose()];
        let planned =
            ClaudeCode::new().plan_skills(&fs, Path::new("/p"), &workflows, VERSION, false);

        assert_eq!(planned.plan.writes.len(), 1);
        assert_eq!(
            planned.plan.writes[0].path,
            PathBuf::from("/p/.claude/skills/codev-propose/SKILL.md")
        );
        assert!(planned.preserved.is_empty());
    }

    #[test]
    fn regenerates_a_skill_from_an_earlier_version() {
        let older = ClaudeCode::render(propose(), "0.0.1", &RenderCtx::default());
        let fs =
            MemoryFileSystem::new().with_file("/p/.claude/skills/codev-propose/SKILL.md", older);

        let planned =
            ClaudeCode::new().plan_skills(&fs, Path::new("/p"), &[propose()], VERSION, false);

        assert_eq!(planned.plan.writes.len(), 1, "the update must write");
        assert!(planned.preserved.is_empty());
    }

    #[test]
    fn preserves_a_skill_edited_by_hand() {
        let edited = format!(
            "{}\n\nMy own instruction added at the end.\n",
            ClaudeCode::render(propose(), VERSION, &RenderCtx::default())
        );
        let fs =
            MemoryFileSystem::new().with_file("/p/.claude/skills/codev-propose/SKILL.md", edited);

        let planned =
            ClaudeCode::new().plan_skills(&fs, Path::new("/p"), &[propose()], VERSION, false);

        assert!(
            planned.plan.writes.is_empty(),
            "a file edited by hand must not be overwritten"
        );
        assert_eq!(
            planned.preserved,
            [PathBuf::from("/p/.claude/skills/codev-propose/SKILL.md")]
        );
    }

    #[test]
    fn force_overwrites_even_an_edited_skill() {
        let edited = format!(
            "{}\n\nMy instruction.\n",
            ClaudeCode::render(propose(), VERSION, &RenderCtx::default())
        );
        let fs =
            MemoryFileSystem::new().with_file("/p/.claude/skills/codev-propose/SKILL.md", edited);

        let planned =
            ClaudeCode::new().plan_skills(&fs, Path::new("/p"), &[propose()], VERSION, true);

        assert_eq!(planned.plan.writes.len(), 1);
        assert!(planned.preserved.is_empty());
    }

    #[test]
    fn an_already_compliant_skill_is_replanned_harmlessly() {
        // The plan contains it, but execution will recognize it as identical
        // and write nothing: `apply::execute` is the one that decides.
        let fs = MemoryFileSystem::new().with_file(
            "/p/.claude/skills/codev-propose/SKILL.md",
            ClaudeCode::render(propose(), VERSION, &RenderCtx::default()),
        );
        let planned =
            ClaudeCode::new().plan_skills(&fs, Path::new("/p"), &[propose()], VERSION, false);
        assert_eq!(planned.plan.writes.len(), 1);
        assert!(planned.preserved.is_empty());
    }

    #[test]
    fn reads_the_version_stamp() {
        assert_eq!(
            stamped_version(&ClaudeCode::render(
                propose(),
                "1.2.3",
                &RenderCtx::default()
            ))
            .as_deref(),
            Some("1.2.3")
        );
        assert_eq!(stamped_version("no frontmatter"), None);
        assert_eq!(
            stamped_version("---\nname: x\n---\nversion: 9.9.9\n"),
            None,
            "a version outside the frontmatter does not count"
        );
    }

    #[test]
    fn detects_claude_code_from_its_traces() {
        let root = Path::new("/p");
        assert!(!ClaudeCode::new().detect(&MemoryFileSystem::new(), root));
        assert!(ClaudeCode::new().detect(
            &MemoryFileSystem::new().with_file("/p/CLAUDE.md", "#"),
            root
        ));
        assert!(ClaudeCode::new().detect(
            &MemoryFileSystem::new().with_file("/p/.claude/settings.json", "{}"),
            root
        ));
    }

    // ─────────────── substitute_jira_mcp ───────────────

    #[test]
    fn substitute_with_tool_replaces_the_placeholder() {
        let src = "Bash(codev:*), Read, {{JIRA_MCP_TOOL}}";
        let out = substitute_jira_mcp(src, Some("mcp__foo__bar"));
        assert_eq!(out, "Bash(codev:*), Read, mcp__foo__bar");
    }

    #[test]
    fn substitute_without_tool_removes_placeholder_and_comma_in_allowed_tools() {
        // The preceding comma must go too, otherwise `allowed_tools` would
        // end with `", "` — malformed.
        let src = "Bash(codev:*), Read, Write, Grep, {{JIRA_MCP_TOOL}}";
        let out = substitute_jira_mcp(src, None);
        assert_eq!(out, "Bash(codev:*), Read, Write, Grep");
    }

    #[test]
    fn substitute_without_tool_uses_the_fallback_in_the_body() {
        // Without a preceding comma (the typical body case), the placeholder
        // is replaced by the informative mention.
        let src = "Call the `{{JIRA_MCP_TOOL}}` tool with the ticket id.";
        let out = substitute_jira_mcp(src, None);
        assert_eq!(
            out,
            "Call the `(Jira MCP not configured)` tool with the ticket id."
        );
    }

    #[test]
    fn substitute_without_placeholder_changes_nothing() {
        let src = "Bash(codev:*), Read";
        assert_eq!(substitute_jira_mcp(src, None), src);
        assert_eq!(substitute_jira_mcp(src, Some("mcp__x__y")), src);
    }

    #[test]
    fn render_with_jira_tool_substitutes_everywhere() {
        let ctx = RenderCtx {
            jira_mcp_tool: Some("mcp__x__y".into()),
        };
        let rendered = ClaudeCode::render(propose(), VERSION, &ctx);
        assert!(!rendered.contains("{{JIRA_MCP_TOOL}}"));
        assert!(!rendered.contains("(Jira MCP not configured)"));
        assert!(rendered.contains("mcp__x__y"));
    }

    #[test]
    fn render_without_jira_tool_removes_placeholder_and_uses_fallback() {
        let rendered = ClaudeCode::render(propose(), VERSION, &RenderCtx::default());
        assert!(!rendered.contains("{{JIRA_MCP_TOOL}}"));
        // The body cites the fallback (at least one occurrence).
        assert!(rendered.contains("(Jira MCP not configured)"));
        // `allowed-tools` does not end with an orphan comma.
        let line = rendered
            .lines()
            .find(|l| l.starts_with("allowed-tools:"))
            .unwrap();
        assert!(!line.ends_with(", \""));
        assert!(line.ends_with('"'));
    }

    fn names(list: &[&str]) -> Vec<String> {
        list.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn claude_md_section_names_propose_apply_and_the_installed_skills() {
        let body =
            ClaudeCode::claude_md_body(&names(&["codev-propose", "codev-explore", "codev-apply"]));
        assert!(
            body.contains("`_codev/specs/` describes the current behavior and is authoritative")
        );
        assert!(body.contains("`_codev/decisions/`"));
        assert!(
            body.contains("start with `/codev-propose`, then implement it with `/codev-apply`")
        );
        assert!(body.contains("Do not modify behavior without a codev change"));
        assert!(body.contains("Trivial fixes"));
        assert!(body.contains("`codev validate`"));
        assert!(
            body.contains("Installed skills: `/codev-propose`, `/codev-explore`, `/codev-apply`.")
        );

        let fs = MemoryFileSystem::new();
        let planned = ClaudeCode::plan_claude_md(
            &fs,
            Path::new("/p"),
            &names(&["codev-propose"]),
            SpliceMode::Init,
        );
        codev_engine::apply::execute(&planned.plan, &fs).unwrap();
        let written = fs.read("/p/CLAUDE.md").unwrap();
        assert!(codev_core::managed::has_section(&written));
    }

    #[test]
    fn claude_md_falls_back_to_tasks_when_apply_is_not_installed() {
        let body = ClaudeCode::claude_md_body(&names(&["codev-propose", "codev-explore"]));
        assert!(body.contains("following the change's `tasks.md`"));
        assert!(!body.contains("/codev-apply"));
    }

    #[test]
    fn claude_md_refresh_keeps_user_text_and_adds_nothing_when_absent() {
        let fs = MemoryFileSystem::new().with_file("/p/CLAUDE.md", "# Team notes\n");
        let planned = ClaudeCode::plan_claude_md(
            &fs,
            Path::new("/p"),
            &names(&["codev-propose"]),
            SpliceMode::Refresh,
        );
        assert!(planned.plan.is_empty() && planned.steps.is_empty());
    }

    #[test]
    fn skill_states_report_missing_outdated_and_current() {
        let propose = workflows::find("propose").unwrap();
        let explore = workflows::find("explore").unwrap();
        let apply = workflows::find("apply").unwrap();
        let fs = MemoryFileSystem::new()
            .with_file(
                "/p/.claude/skills/codev-propose/SKILL.md",
                ClaudeCode::render(propose, VERSION, &RenderCtx::default()),
            )
            .with_file(
                "/p/.claude/skills/codev-explore/SKILL.md",
                ClaudeCode::render(explore, "0.0.9", &RenderCtx::default()),
            );
        let states =
            ClaudeCode::skill_states(&fs, Path::new("/p"), &[propose, explore, apply], VERSION);
        assert_eq!(
            states,
            [
                ("codev-propose".to_string(), SkillState::Current),
                (
                    "codev-explore".to_string(),
                    SkillState::Outdated(Some("0.0.9".into()))
                ),
                ("codev-apply".to_string(), SkillState::Missing),
            ]
        );
    }
}
