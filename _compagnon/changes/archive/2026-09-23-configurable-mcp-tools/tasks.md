# Tasks

## 1. Project config — new `mcp:` field

- [x] 1.1 Add the `McpConfig` struct in
      `codev-engine::config`:
      ```rust
      #[derive(Debug, Clone, Default, Deserialize)]
      #[serde(deny_unknown_fields)]
      pub struct McpConfig {
          #[serde(default)]
          pub jira_tool: Option<String>,
      }
      ```
- [x] 1.2 Add the field `pub mcp: McpConfig` (with
      `#[serde(default)]`) to `ProjectConfig`.
- [x] 1.3 Add the field `pub mcp: McpConfig` to `ResolvedConfig`,
      fed by the project only (no inherited merge).
- [x] 1.4 Tests in `config`: parse a YAML with `mcp:` → correct
      value; parse a YAML without `mcp:` → empty default; parse a
      YAML with an unknown field in `mcp:` → `serde` error.

## 2. Skill rendering — placeholder + substitution

- [x] 2.1 Add `pub struct RenderCtx { pub jira_mcp_tool:
      Option<String> }` in `codev-agents::claude` (or a dedicated
      module if cleaner).
- [x] 2.2 Extend `ClaudeCode::render` to accept a `&RenderCtx`
      argument. Signature before:
      `render(workflow: &Workflow, version: &str)`. Signature after:
      `render(workflow: &Workflow, version: &str, ctx: &RenderCtx)`.
- [x] 2.3 New internal function `substitute_jira_mcp(source:
      &str, jira_tool: Option<&str>) -> String` that applies the
      design's rule (removes `, {{JIRA_MCP_TOOL}}` before the
      fallback). Called on `allowed_tools` **and** `body` before
      assembly.
- [x] 2.4 Dedicated tests on `substitute_jira_mcp`:
      - Some(tool) → `{{JIRA_MCP_TOOL}}` replaced by the name.
      - None + placeholder preceded by `, ` in `allowed_tools` →
        placeholder AND comma removed.
      - None + placeholder alone in the body → replaced by
        `(Jira MCP not configured)`.
      - No placeholder in the source → source unchanged.

## 3. CATALOG — uses the placeholder

- [x] 3.1 In `codev-agents::workflows::CATALOG`, `propose` entry:
      `allowed_tools: "Bash(codev:*), Read, Write, Edit, Glob, Grep, {{JIRA_MCP_TOOL}}"`.
      Remove the hardcoded mention of
      `mcp__claude_ai_Atlassian__getJiraIssue`.
- [x] 3.2 Edit `assets/workflows/propose.md`: wherever the name
      `mcp__claude_ai_Atlassian__getJiraIssue` is cited, replace it
      with `{{JIRA_MCP_TOOL}}`.
- [x] 3.3 Update the doc comment of the `propose` entry — no longer
      say "Atlassian MCP" specifically, but "the Jira MCP configured
      at the project level".

## 4. Invariant tests — adapt and replace

- [x] 4.1 Remove the obsolete tests:
      `propose_declare_le_mcp_atlassian_get_jira_issue`,
      `propose_ne_declare_pas_dautre_mcp_atlassian`. Rationale:
      the CATALOG's `allowed_tools` no longer cites any MCP name.
- [x] 4.2 New test
      `propose_utilise_un_placeholder_pour_le_mcp_jira`:
      `find("propose").unwrap().allowed_tools.contains("{{JIRA_MCP_TOOL}}")`.
- [x] 4.3 Adapt `propose_cite_la_detection_de_ticket_dans_son_body`
      to check for the presence of `{{JIRA_MCP_TOOL}}` in the body
      (instead of the hardcoded name).
- [x] 4.4 New test in `codev-agents::claude::tests`:
      `render_avec_jira_tool_configure_substitue_partout` — checks
      that after `render` with `RenderCtx { jira_mcp_tool:
      Some("mcp__x__y".into()) }`, neither `{{JIRA_MCP_TOOL}}` nor
      `(Jira MCP not configured)` remains, and that the name does
      appear.
- [x] 4.5 New test
      `render_sans_jira_tool_retire_placeholder_et_virgule`: after
      `render` with `jira_mcp_tool: None`, the rendered
      `allowed_tools` ends with `Grep"` (no comma or MCP), and the
      body contains `(Jira MCP not configured)`.

## 5. Wiring — the CLI passes the context

- [x] 5.1 `codev-cli::commands::update` builds a `RenderCtx`
      from `ResolvedConfig.mcp.jira_tool` and passes it to
      `ClaudeCode::render`.
- [x] 5.2 `codev-cli::commands::init` does the same (in the
      scaffolding path that installs the skills).
- [x] 5.3 Integration test `init_installe_skill_avec_mcp_configure`:
      in a `Harnais::neuf` that writes a `config.yaml` with `mcp:`,
      run `init`, check that the installed SKILL.md carries the
      right name.
- [x] 5.4 Integration test
      `init_installe_skill_sans_mcp_configure`: without an `mcp:`
      block, the installed SKILL.md carries `allowed-tools` without
      the MCP suffix and a body with the mention "(Jira MCP not
      configured)".

## 6. Scaffold — commented example

- [x] 6.1 `DEFAULT_CONFIG` of `codev-engine::scaffold` gains a
      commented example block:
      ```yaml
      # Name of the Jira MCP tool to use when a ticket is
      # mentioned in a skill's prompt. Uncomment and replace
      # with the exact name of the MCP available in your
      # Claude Code session.
      # mcp:
      #   jira_tool: mcp__claude_ai_Atlassian_Rovo__getJiraIssue
      ```
      Placed after the commented `inherits:` block.

## 7. Repository migration

- [x] 7.1 Edit the repository's `_codev/config.yaml` — add:
      ```yaml
      mcp:
        jira_tool: mcp__claude_ai_Atlassian_Rovo__getJiraIssue
      ```
- [x] 7.2 `cargo install --path crates/codev-cli`
- [x] 7.3 `codev update --force` — the propose SKILL.md now carries
      the Rovo name.
- [x] 7.4 Manual check: `head -5
      .claude/skills/codev-propose/SKILL.md` — the `allowed-tools`
      does contain `mcp__claude_ai_Atlassian_Rovo__getJiraIssue`,
      and no `{{…}}` remains.

## 8. Workspace integration

- [x] 8.1 `cargo test --workspace` stays green, gains at least 8
      new tests (config + substitute + render + init integration).
- [x] 8.2 `cargo clippy --workspace --all-targets` stays free of
      warnings.
- [x] 8.3 `codev validate --strict --all` stays green.
