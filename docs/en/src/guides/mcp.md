# Jira and other MCP servers

codev's skills can pull context from external systems through the MCP servers
connected to your Claude Code session. The CLI itself never talks to them: the
skills detect what they need in your request and call the MCP tool directly.

Today, one integration is available: `/codev-propose` reads a Jira ticket.

## Jira

When your request to `/codev-propose` contains a ticket identifier — anything
matching `[A-Z]{2,}-\d+`, such as `PROJ-123` — and a Jira MCP tool is
configured, the skill:

1. calls the configured tool once, for the first identifier it found;
2. uses the ticket's title, description, status and type as context for the
   plan;
3. cites the ticket at the top of the proposal:

   ```markdown
   # Proposal: Add dark mode

   > Source: ticket **PROJ-123** — "Dark mode for the dashboard" (In Progress)

   ## Why
   ```

The integration is strictly read-only: one call to the one configured tool,
never a search, a comment or a transition. If the request mentions several
tickets, only the first is fetched; the others are listed under the citation
for traceability.

If a ticket is mentioned but the MCP server is not available in the session,
the skill says so, writes the proposal from your request alone, and cites the
ticket as "content not retrieved".

## Configuration

The name of the Jira tool depends on how the MCP server is registered in your
Claude Code setup, so it is configured per project:

```yaml
# _codev/config.yaml
mcp:
  jira_tool: mcp__atlassian__getJiraIssue
```

`codev init` fills it in for you when it finds a server whose name, command or
URL mentions Jira or Atlassian in `.mcp.json`, `.claude/settings.json` or
`~/.claude.json`:

```text
  ✓ Jira MCP detected: mcp__atlassian__getJiraIssue
    (source: .mcp.json → server "atlassian")
```

The tool name follows Claude Code's convention,
`mcp__<server name>__getJiraIssue`, with spaces and dots in the server name
replaced by underscores — for example, a connector named
`claude.ai Atlassian Rovo` gives
`mcp__claude_ai_Atlassian_Rovo__getJiraIssue`. If yours differs, look for the
exact name in the tool list of a Claude Code session and set it by hand.

At install time, codev adds the tool to the `allowed-tools` of
`codev-propose`, so the skill may call it and nothing else from that server.
After changing `mcp:`, regenerate the skills:

```bash
codev update --force
```

Without `mcp.jira_tool`, ticket detection is inert: the skill behaves exactly
as it does in a project without any MCP integration.

> **Note**
> `mcp:` is never inherited from an [inherited source](inherited-sources.md):
> tool names depend on each person's Claude Code setup, not on a shared
> repository.

## Adding another MCP integration

MCP integrations are part of codev itself: each one is a configuration key and
a placeholder in a workflow, so adding one — for example a Confluence page or
a design tool — is a contribution to codev rather than a setting. The pattern
is the same as for Jira:

1. Add a key under `mcp:` in the project configuration
   (`crates/codev-engine/src/config.rs`).
2. Carry it into the skill render context and substitute a placeholder such as
   `{{JIRA_MCP_TOOL}}` in both the `allowed-tools` and the body of the
   workflow that uses it (`crates/codev-agents/src/claude.rs`).
3. Describe the detection and the call in the workflow file under
   `assets/workflows/`, with a clean fallback when the key is absent.

The existing skill is extended rather than duplicated: there is one
`/codev-propose`, whatever the source of the request. See
[CONTRIBUTING.md](https://github.com/mairistem/codev/blob/main/CONTRIBUTING.md)
to propose one.
