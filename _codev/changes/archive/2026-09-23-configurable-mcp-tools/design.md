# Design: configure MCP names at the project level

## Context

See `proposal.md`. The first real use showed that MCP names are
**specific to the Claude Code environment**. This change removes the
names from the source code and moves them to `_codev/config.yaml`.

## Goals / Non-Goals

This design covers the configuration structure, the placeholder
substitution mechanics, the propagation across the crates, and the
fallback when no MCP is configured. It does not cover other MCPs
(Design, Confluence…), cross-project inheritance of the `mcp:`
block, or a rendering debug mode.

## Decisions

### Decision: `mcp:` is a typed struct, not a `HashMap<String, String>`

Two options:

| Option | Pro | Con |
|---|---|---|
| **A. Free-form `HashMap<String, String>`** | Zero-cost extension, each skill reads its key | Invites putting anything in; silent naming errors; `deny_unknown_fields` useless |
| **B. `struct McpConfig { jira_tool: Option<String>, … }`** | Known fields, serde validation, IDE autocompletion, warnings on typos | Each new MCP requires a PR on the codev side |

**Chosen: B.** Consistent with the philosophy of the rest of
`ProjectConfig` (`inherits`, `rules`) — typed fields,
`deny_unknown_fields`. Adding a field per future MCP is a negligible
cost, and typing makes config errors immediately visible.

### Decision: placeholder substitution via `str::replace`, no template engine

Rendering a `SKILL.md` is simple: a few static substitutions
(`{{FRONTMATTER}}` already exists for decisions). Introducing a
template engine (handlebars, tera) would be disproportionate.

**Rejected alternative**: `handlebars-rust`. Useful one day if the
substitutions become conditional/complex; today,
`s.replace("{{JIRA_MCP_TOOL}}", tool)` is enough. Can be deferred.

### Decision: the placeholder is removed **along with the comma that precedes it** in `allowed_tools`

Without this precaution, `allowed_tools` would end with `", "` when
the config is absent — Claude Code would probably reject the
frontmatter. The substitution is done in two steps:

```rust
fn substitute_jira_mcp(source: &str, jira_tool: Option<&str>) -> String {
    match jira_tool {
        Some(tool) => source.replace("{{JIRA_MCP_TOOL}}", tool),
        None => source
            .replace(", {{JIRA_MCP_TOOL}}", "")   // in allowed_tools
            .replace("{{JIRA_MCP_TOOL}}", "(Jira MCP not configured)"), // elsewhere
    }
}
```

Order matters: first remove `, {{JIRA_MCP_TOOL}}` (with the comma)
before the fallback on `{{JIRA_MCP_TOOL}}` alone. Otherwise the first
`replace` without the comma would leave an orphan `", "`.

### Decision: `RenderCtx` carries the single field, not an extensible struct

For V1: `RenderCtx { jira_mcp_tool: Option<String> }`. If another
MCP comes along later, a field is added. A `HashMap<String,
Option<String>>` would be too generic for a narrow need.

### Decision: the MCP configuration is project-specific, not inherited

A project that declares `inherits: [{path: ~/shared}]` does **not**
inherit the source's `mcp:` block. Each project configures its own
MCP — because the MCP name depends on the user's Claude Code
configuration, not on the source project.

**Rejected alternative**: propagate `mcp:` through inheritance. A
source shared between several teams would force all those teams to
use the same MCP, which is not what we want.

### Decision: the fallback shows `(Jira MCP not configured)` in the body

When `mcp.jira_tool` is absent, the agent reading the body sees
`(Jira MCP not configured)` where the tool name would have been.
This tells it clearly:

1. That it does **not** have to try to call an MCP (which does not
   exist).
2. That if the user mentions a ticket, it must show the documented
   informational message and write without the content.

**Rejected alternative**: leave the body with the raw placeholder
`{{JIRA_MCP_TOOL}}`. Rejected — the agent would have no reference
point and could hallucinate a tool name.

## Risks / Trade-offs

- **Naive substitution breaks if a user accidentally writes
  `{{JIRA_MCP_TOOL}}` in their skill body.** → **Accepted
  trade-off**: the CATALOG is Rust code maintained by us; nobody
  writes a skill body by hand. If one day the catalog becomes
  editable project by project, the substitution will become a source
  of ambiguity — we will replace it with a real template engine.
- **The MCP name depends on a Claude Code convention we have no
  control over.** If Claude Code renames its tools again, the project
  config will break. → **Mitigation**: the informational message
  documents what happens; the user updates their `config.yaml` in
  one line. No code change.
- **Two configs declaring the same MCP differ by one character.** →
  **Accepted trade-off**: strict validation (serde
  `deny_unknown_fields` on `McpConfig` + `String` type) catches YAML
  parsing but not typos in the name itself. A sanity test could
  check that the name matches `^mcp__[a-zA-Z0-9_]+$` — can be
  deferred.

## Migration Plan

For this repository:

1. Edit `_codev/config.yaml` — add:
   ```yaml
   mcp:
     jira_tool: mcp__claude_ai_Atlassian_Rovo__getJiraIssue
   ```
2. `cargo install --path crates/codev-cli`
3. `codev update --force` — the installed SKILL.md now carries the
   Rovo name.
4. Check: `head -5 .claude/skills/codev-propose/SKILL.md` shows
   `allowed-tools: "..., mcp__claude_ai_Atlassian_Rovo__getJiraIssue"`.

For a project discovering codev:

- With no MCP connected or configured: the skill installs with
  `allowed-tools` without an MCP and a body that says "(Jira MCP not
  configured)". Behavior bit-identical to the one before
  `propose-detects-jira-tickets`.
- With an MCP connected: add the `mcp:` block in `config.yaml` (the
  scaffold's `DEFAULT_CONFIG` contains a commented example), rerun
  `codev update --force`.
