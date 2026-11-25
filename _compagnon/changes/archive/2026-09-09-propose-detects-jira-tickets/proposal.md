# Proposal: `codev-propose` detects and enriches a mentioned Jira ticket

## Why

This is the first real MCP integration on the codev side. Today, when
a user mentions `PROJ-123` in their `/codev-propose` prompt, the skill
treats the string as an opaque word, just like "add authentication"
— the proposal comes out **without** the ticket's context, even
though an Atlassian MCP connected to the same session could fetch it
in a single call.

This is the strategy Ludovic settled on: **future codev skills detect
MCP-triggerable patterns themselves and call the MCP directly, without
separate skills** ([[project_codev_mcp-integration]]). This change
delivers the first case — Jira / Atlassian — whose lessons will guide
the following integrations (Design, Notion, GitHub Issues…).

Decided with Ludovic after a `/codev-explore` exploration
(2026-09-09) — the 6 open questions were settled:

1. **Scope** — Jira only; a real second case will come later.
2. **MCP** — `mcp__claude_ai_Atlassian__*` (official Claude Atlassian
   MCP, available on Claude Code).
3. **Pattern** — generic `[A-Z]{2,}-\d+`, no project configuration.
4. **Fetched content** — injected into the conversation context
   **and** cited at the top of the proposal for traceability.
5. **Without a connected MCP** — informational message; the proposal
   is written anyway, without the ticket's content.
6. **Where the logic lives** — in the skill's markdown body (the codev
   CLI does not need to know about Jira).

## What Changes

- **The body of `assets/workflows/propose.md`** gains a step 0:
  detection of a `[A-Z]{2,}-\d+` pattern in the user's prompt, even
  before resolving the change name.
- **On detection**:
  - If `mcp__claude_ai_Atlassian__getJiraIssue` is available, the
    skill calls it with the ticket identifier.
  - The ticket's content (title, description, status, type) becomes
    a **context source** that the agent reads before writing the
    proposal.
  - The proposal cites the ticket at the top, in an "External
    context" section or similar, with a line such as
    `> Source: ticket **PROJ-123** — "<title>" (<status>)`.
- **Without a connected MCP**: the skill displays
  "Ticket PROJ-123 is mentioned but no Atlassian MCP is available in
  this session — the proposal will be written without its content."
  Then it continues.
- **`allowed-tools` of `propose`** gains
  `mcp__claude_ai_Atlassian__getJiraIssue`. No other MCP tool is
  added — the skill has no need to search or modify tickets.
- **No boundary change** — the skill remains unable to write outside
  `_codev/changes/<name>/`.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `skills` — new ADDED requirement describing the detection and
  enrichment behavior of `codev-propose`. The other propose
  requirements (implicit today) remain unchanged.

### Removed Capabilities

None.

## Impact

- **Code**: edit of the `assets/workflows/propose.md` body (adding a
  step 0). Extension of the `allowed-tools` of the
  `Workflow { id: "propose", … }` entry in the `CATALOG` of
  `codev-agents::workflows`. A dedicated test
  `propose_declare_le_mcp_atlassian` locks in the presence of the MCP
  in `allowed-tools`.
- **JSON contract**: nothing. Detection and the MCP call live
  entirely in the body — the codev CLI knows nothing about them.
- **Files written**: no change on disk apart from the proposal.md of
  the change in question. The skill keeps creating the change via
  `codev new change` as before.
- **Migration**: none. Without a connected Atlassian MCP, behavior is
  bit-identical to today (informational message on the first
  mentioned ticket, silent otherwise).
- **Out of scope**:
  - **Other MCPs** (Design, Notion, GitHub) — will wait for a real
    second concrete case.
  - **Ticket search** (JQL, listing) — the skill only calls
    `getJiraIssue` for a specific mentioned ID.
  - **Writing to Jira** (transitions, comments) — not in this batch;
    the proposal may suggest it as future work if useful.
  - **Multiple tickets in a single invocation** — the skill handles
    the first detected ticket; the others are just mentioned.
    Deferrable if the pattern becomes recurrent.
  - **Project configuration of the pattern** — no `ticket_pattern:`
    field in `_codev/config.yaml` for V1. The generic pattern
    `[A-Z]{2,}-\d+` is enough for all standard Atlassian orgs.
