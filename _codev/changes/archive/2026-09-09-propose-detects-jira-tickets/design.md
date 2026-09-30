# Design: `codev-propose` detects and enriches a Jira ticket

## Context

See `proposal.md`. First real wiring of an MCP into a codev skill.
The design crystallizes the six choices made during exploration.

## Goals / Non-Goals

This design covers: where detection sits in the flow, the format of
the citation at the top of the proposal, the fallback strategy
without an MCP, and the invariant test that locks in `allowed-tools`.
It does not cover other MCPs (Design, GitHub…), a multi-ticket mode,
or project configuration of the pattern.

## Decisions

### Decision: detection lives in the markdown body, not in the CLI

Direct alignment with the MCP strategy recorded in memory: the `codev`
CLI stays **MCP-agnostic**. It knows neither Jira nor Atlassian; it
manages `_codev/`, the artifact graph, the seal, the merge, the
archive. External integrations live at the Claude Code skill level —
it is the agent that reads the body, spots the pattern, calls the
MCP, and writes the proposal.

**Consequence**: nothing to change in the Rust crates on the runtime
side. Only the skill's markdown body and the CATALOG's
`allowed-tools` move. A future integration of another MCP will follow
the same template.

**Rejected alternative**: a `codev new change --from-ticket
PROJ-123` flag that, on the CLI side, would call the MCP through a
dedicated port. Rejected because it would force the core to know
about MCPs and would break agnosticism.

### Decision: exact MCP name hardcoded — `mcp__claude_ai_Atlassian__getJiraIssue`

The official Claude Atlassian MCP exposes some thirty tools; we only
declare the one strictly needed to read a ticket. The skill never
writes to Jira, does no JQL search, and touches neither Confluence
nor Compass. The restriction is a safeguard: if a bad prompt ever
tries to use the skill to write, the tool call will fail at the
harness level rather than silently comply.

**Rejected alternative**: `mcp__claude_ai_Atlassian__*` (glob). More
permissive, but leaves the door open to unplanned calls. Refused on
the security principle of "minimum viable".

### Decision: citation at the top of the proposal, not in `change.yaml`

The ticket is **human information** — its natural place is the
proposal, which the reviewer reads first. Putting it in
`change.yaml` (e.g. `source_ticket: PROJ-123`) would have required
extending `ChangeMetadata` (new field), parsing it, rendering it in
status… code for a case that lives perfectly well in markdown.

**Chosen format** — first line of the proposal under the title:

```
# Proposal: <title>

> Source: ticket **PROJ-123** — "Authenticate users with JWT" (In Progress)

## Why
[…]
```

If the MCP failed: `> Source: ticket **PROJ-123** — content not
fetched`. The shape stays stable; only the suffix changes.

### Decision: the pattern is the generic `[A-Z]{2,}-\d+`, not configurable

Every Atlassian organization uses an uppercase prefix of 2+ letters
followed by a hyphen and a number. A lone `A-1` would be ambiguous
(a reference to an Excel cell?) — the 2+ letter constraint avoids that
false positive. Adding project configuration
(`ticket_pattern: "PROJ-\\d+"`) would cost a new `ChangeMetadata` or
`ProjectConfig` field for marginal benefit — false positives on
`[A-Z]{2,}-\d+` are rare in practice.

**Deferrable**: if a project regularly reports false positives, we
will add `_codev/config.yaml.ticket_pattern`.

### Decision: without an MCP, an informational message — not silence

A user who mentions `PROJ-123` expects it to count for something.
Staying silent would leave them in the dark. The message costs one
sentence, clarifies the skill's behavior, and does not prevent
execution.

**Alignment** with the codev philosophy: "report, never block
silently" — a pattern already applied in `sync` (invites to archive),
`validate` (emits warnings), `deviate` (announces the ripple).

### Decision: one ticket fetched, the others just named

Handling every mentioned ticket would multiply MCP calls and weigh
down the proposal. The rule "first detected = main source" is simple
and predictable. The other tickets remain mentioned at the top
(`other mentioned ticket(s): PROJ-456`) for traceability — a reader of
the proposal can look them up by hand.

**Deferrable**: an `--all-tickets` mode or multiple MCP calls if the
pattern becomes recurrent.

## Risks / Trade-offs

- **The user does not expect the skill to call an MCP.** In an
  environment where several MCPs are connected, an unexpected call
  can be surprising. → **Mitigation**: the informational message when
  the MCP succeeds ("Ticket PROJ-123 fetched via the Atlassian MCP,
  injected into the proposal context") makes the call visible.
- **Pattern false positive** — an identifier that looks like a ticket
  without being one (e.g. `TODO-42` in a comment in the prompt). →
  **Accepted trade-off**: the MCP will probably return `not_found`,
  and the skill falls back to "ticket mentioned, content not fetched".
  The worst case is a useless warning.
- **The ticket content is too large** — a Jira description can be
  long. → **Accepted trade-off**: the MCP returns structured JSON; the
  skill summarizes what it injects into the prompt (title, status,
  description truncated if needed). This is the skill's editorial
  responsibility, not code logic.
- **The MCP returns an authentication error** (expired token, private
  space). → **Behavior**: treated as "MCP unavailable", with a
  specific informational message ("authentication required").

## Migration Plan

None. Users without a connected MCP see nothing change as long as
they do not mention a ticket. On the first ticket mention without an
MCP, an informational message appears — but the proposal still comes
out.

To enable the new capability, it is enough to connect the Atlassian
MCP in the user's Claude Code config (independent of codev) — then
rerun `codev update --force` so that the new version of the skill is
installed with its extended `allowed-tools`.
