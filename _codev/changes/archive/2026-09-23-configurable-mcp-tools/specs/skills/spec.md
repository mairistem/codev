## MODIFIED Requirements

### Requirement: Skill `propose` detects mentioned Jira tickets and enriches the proposal

The catalog's `propose` workflow SHALL, before resolving the change
name, scan the user's prompt for a ticket identifier that matches
the regular pattern `[A-Z]{2,}-\d+` (for example `PROJ-123`,
`PROJ-42`).

On detection of at least one ticket, the skill MUST:

1. **Attempt to call** the MCP tool whose name is declared at the
   project level via `_codev/config.yaml.mcp.jira_tool`. This name is
   injected at skill installation time into the `allowed-tools`
   frontmatter and into the body — the user/organization chooses
   **its** MCP (`mcp__claude_ai_Atlassian__getJiraIssue`,
   `mcp__claude_ai_Atlassian_Rovo__getJiraIssue`, or another one)
   without touching codev's source code.
2. **A single call per invocation**, on the identifier of the ticket
   **mentioned earliest in the prompt**; the others are just named.
3. **On success**: inject the content (title, description, status,
   type) into the writing context, and show the ticket at the top of
   `proposal.md` as a quotation line
   `> Source: ticket **<ID>** — "<title>" (<status>)`.
4. **On failure — MCP tool unavailable in the session or not
   configured at the project level**: show an informational message
   to the user, then continue the usual flow without the ticket
   content. The proposal still cites the ticket at the top ("Source:
   ticket **<ID>** — content not retrieved").
5. **When no pattern is present**: behavior bit-identical to
   today — no MCP call, no message.

The workflow CATALOG MUST use a textual placeholder
`{{JIRA_MCP_TOOL}}` in `allowed_tools` and in the body of the
`propose` workflow, in place of a hardcoded MCP name. The
placeholder is substituted when the installation frontmatter is
rendered by `ClaudeCode::render`.

When `_codev/config.yaml.mcp.jira_tool` is **absent** or empty, the
rendering MUST:

- cleanly remove `{{JIRA_MCP_TOOL}}` from `allowed_tools` **and** the
  comma that precedes it (so as not to leave a malformed
  `allowed-tools` that would end with `", "`);
- replace each occurrence in the body with the string
  `(Jira MCP not configured)` — the skill remains installed and
  functional in other respects, but no longer calls any MCP.

#### Scenario: `mcp.jira_tool` configuration present → functional skill

- **GIVEN** a project whose `_codev/config.yaml` declares
  `mcp: { jira_tool: mcp__claude_ai_Atlassian_Rovo__getJiraIssue }`
- **WHEN** the user runs `codev update`
- **THEN** the `.claude/skills/codev-propose/SKILL.md` file carries
  `allowed-tools: "Bash(codev:*), Read, Write, Edit, Glob, Grep, mcp__claude_ai_Atlassian_Rovo__getJiraIssue"`
- **AND** the skill body cites
  `mcp__claude_ai_Atlassian_Rovo__getJiraIssue` where the CATALOG
  contains `{{JIRA_MCP_TOOL}}`
- **AND** no `{{…}}` remains in the installed file

#### Scenario: `mcp.jira_tool` configuration absent → clean fallback

- **GIVEN** a project whose `_codev/config.yaml` has no `mcp:` block
- **WHEN** the user runs `codev update`
- **THEN** the `.claude/skills/codev-propose/SKILL.md` file carries
  `allowed-tools: "Bash(codev:*), Read, Write, Edit, Glob, Grep"`
  (without the placeholder and without a trailing comma)
- **AND** the body replaces `{{JIRA_MCP_TOOL}}` with the mention
  `(Jira MCP not configured)`
- **AND** no `{{…}}` remains in the installed file

#### Scenario: Ticket mentioned, MCP configured and available → ticket cited at the top

- **GIVEN** a user types `/codev-propose add JWT for PROJ-123`
- **AND** the project has configured `mcp.jira_tool:
  mcp__claude_ai_Atlassian_Rovo__getJiraIssue`
- **AND** that MCP is available in the Claude Code session
- **AND** the ticket `PROJ-123` exists
- **WHEN** the skill runs
- **THEN** a call `mcp__claude_ai_Atlassian_Rovo__getJiraIssue({issueIdOrKey: "PROJ-123"})`
  is made
- **AND** the `proposal.md` of the created change contains at the top
  a line `> Source: ticket **PROJ-123** — "<title>" (<status>)`

#### Scenario: Ticket mentioned, MCP absent → proposal anyway

- **GIVEN** a user types `/codev-propose fix PROJ-123`
- **AND** no Jira MCP is configured (`mcp.jira_tool` absent or
  declared tool unavailable)
- **WHEN** the skill runs
- **THEN** the skill shows an informational message naming
  `PROJ-123` and indicating that the Jira MCP is not active
- **AND** the proposal is created anyway, with `PROJ-123` mentioned
  at the top along with "content not retrieved"

#### Scenario: No ticket mentioned → unchanged behavior

- **GIVEN** a user types `/codev-propose add
  authentication`
- **WHEN** the skill runs
- **THEN** no MCP call is made
- **AND** no ticket-related message appears
- **AND** the proposal is written exactly as before this batch
