## ADDED Requirements

### Requirement: Skill `propose` detects mentioned Jira tickets and enriches the proposal

The catalog's `propose` workflow SHALL, before resolving the change
name, scan the user's prompt to spot a ticket identifier matching the
regular pattern `[A-Z]{2,}-\d+` (for example `PROJ-123`, `PROJ-42`).

On detection of at least one ticket, the skill MUST:

1. **Attempt to call** the MCP tool `mcp__claude_ai_Atlassian__getJiraIssue`
   with the identifier of the ticket **mentioned earliest in the
   prompt** (a single ticket per invocation; the others are just named
   in the proposal, not fetched).
2. **On success**: inject the content (title, description, status,
   type) into the writing context, and show the ticket at the top of
   the change's `proposal.md`, as a quote line
   `> Source: ticket **<ID>** — "<title>" (<status>)`.
3. **On failure — MCP tool unavailable in the session**: display an
   informational message to the user ("Ticket <ID> is mentioned but
   no Atlassian MCP is available in this session — the proposal will
   be written without its content"), then continue the usual flow
   without the ticket's content. The final proposal still mentions
   the ticket at the top, with what the skill knows (the ID alone).
4. **When there is no pattern**: behavior bit-identical to today — no
   MCP call, no message.

The `allowed-tools` of the `propose` workflow MUST declare
`mcp__claude_ai_Atlassian__getJiraIssue`. No other Atlassian MCP tool
is needed for this capability — the skill never writes to Jira and
does no JQL search.

#### Scenario: Ticket mentioned, MCP available → ticket cited at the top

- **GIVEN** a user types `/codev-propose add JWT for PROJ-123`
- **AND** the MCP `mcp__claude_ai_Atlassian__getJiraIssue` is
  available in the session
- **AND** the ticket `PROJ-123` exists, with the title "Authenticate
  users with JWT" and the status "In Progress"
- **WHEN** the skill runs
- **THEN** a call `mcp__claude_ai_Atlassian__getJiraIssue({issueIdOrKey: "PROJ-123"})`
  is issued
- **AND** the `proposal.md` of the created change contains at the top
  a line `> Source: ticket **PROJ-123** — "Authenticate users with
  JWT" (In Progress)`
- **AND** the body of the proposal (Why, What Changes…) uses the
  ticket's content to be more precise than with the prompt alone

#### Scenario: Ticket mentioned, MCP unavailable → proposal anyway

- **GIVEN** a user types `/codev-propose add JWT for PROJ-123`
- **AND** no Atlassian MCP is available in the session
- **WHEN** the skill runs
- **THEN** the skill displays an informational message naming
  `PROJ-123` and saying that no Atlassian MCP is available
- **AND** the proposal is created anyway, with `PROJ-123` mentioned at
  its top as a bare ID ("Source: ticket **PROJ-123** — content not
  fetched")
- **AND** the body of the proposal is written from the user's prompt
  alone

#### Scenario: No ticket mentioned → unchanged behavior

- **GIVEN** a user types `/codev-propose add
  authentication`
- **WHEN** the skill runs
- **THEN** no MCP call is issued
- **AND** no ticket-related message appears
- **AND** the proposal is written exactly as before this change

#### Scenario: Several tickets mentioned → only the first is fetched

- **GIVEN** a user types `/codev-propose fix PROJ-123
  and PROJ-456`
- **AND** the MCP is available
- **WHEN** the skill runs
- **THEN** an MCP call is issued for `PROJ-123` only
- **AND** the proposal cites `PROJ-123` at the top with its fetched
  content
- **AND** the proposal cites `PROJ-456` as a second linked ticket,
  without its content fetched ("other mentioned ticket(s):
  PROJ-456")

#### Scenario: `allowed-tools` does declare the Atlassian MCP

- **GIVEN** the `Workflow { id: "propose", … }` entry of the CATALOG
- **WHEN** its `allowed_tools` is inspected
- **THEN** the string contains
  `mcp__claude_ai_Atlassian__getJiraIssue`
- **AND** does NOT contain any other `mcp__claude_ai_Atlassian__*` tool
  (strict rule: the skill only needs to read a ticket)
