# Start from a Jira ticket

<div class="skip-step">

[No Jira? Skip to Reviewing a plan](review-a-plan.md)

</div>


Your team already describes the work in Jira. Name the ticket in your
request and Claude starts the [plan](../glossary.md#plan) from it: no copy
and paste, and the plan says where it comes from.

1. **You say**: "Prepare a plan for PROJ-123".
2. **Claude reads the ticket** — its title, description, status and
   type — once, without changing anything in Jira.
3. **You get a plan that cites it** on its first line, so a reviewer sees
   which ticket it answers.

<figure class="diagram">
<div class="scroll">
<svg viewBox="0 0 760 210" role="img" aria-label="From a Jira ticket to a plan: you name ticket PROJ-123 in your request; Claude reads the ticket once, read-only; Claude writes proposal.md, whose first line cites the ticket.">
<defs><marker id="dg-mcp-arrow" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" style="fill:var(--dg-line)"/></marker></defs>
<rect x="8" y="36" width="210" height="138" rx="12" style="fill:var(--dg-box);stroke:var(--dg-you)" stroke-width="2"/>
<text x="26" y="64" class="mono" font-size="13" style="fill:var(--dg-muted)">Jira ticket</text>
<text x="26" y="92" class="mono" font-size="18" font-weight="600" style="fill:var(--dg-ink)">PROJ-123</text>
<text x="26" y="118" font-size="15" style="fill:var(--dg-ink)">Dark mode for the</text>
<text x="26" y="138" font-size="15" style="fill:var(--dg-ink)">dashboard</text>
<circle cx="32" cy="158" r="5" style="fill:var(--dg-you)"/>
<text x="44" y="163" font-size="13" style="fill:var(--dg-muted)">In Progress</text>
<line x1="222" y1="105" x2="282" y2="105" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-mcp-arrow)"/>
<text x="252" y="92" text-anchor="middle" font-size="12" style="fill:var(--dg-muted)">named</text>
<rect x="288" y="56" width="180" height="98" rx="49" style="fill:var(--dg-box);stroke:var(--dg-claude)" stroke-width="2"/>
<circle cx="318" cy="88" r="5" fill="#ff5331"/><circle cx="332" cy="88" r="5" fill="#ffab03"/><circle cx="346" cy="88" r="5" fill="#00c39b"/>
<text x="378" y="94" font-size="17" font-weight="600" style="fill:var(--dg-ink)">Claude</text>
<text x="378" y="118" text-anchor="middle" font-size="14" style="fill:var(--dg-ink)">reads it once</text>
<text x="378" y="138" text-anchor="middle" font-size="12" style="fill:var(--dg-muted)">read-only</text>
<line x1="472" y1="105" x2="532" y2="105" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-mcp-arrow)"/>
<text x="502" y="92" text-anchor="middle" font-size="12" style="fill:var(--dg-muted)">writes</text>
<rect x="538" y="24" width="214" height="162" rx="12" style="fill:var(--dg-file);stroke:var(--dg-claude)" stroke-width="2"/>
<rect x="538" y="24" width="214" height="30" rx="12" style="fill:var(--dg-box)"/>
<text x="554" y="44" class="mono" font-size="13" style="fill:var(--dg-ink)">proposal.md</text>
<text x="554" y="80" class="mono" font-size="13" font-weight="600" style="fill:var(--dg-ink)"># Proposal: Dark mode</text>
<rect x="550" y="94" width="190" height="44" rx="6" style="fill:var(--dg-box)"/>
<rect x="550" y="94" width="4" height="44" style="fill:var(--dg-you)"/>
<text x="562" y="112" class="mono" font-size="12" style="fill:var(--dg-ink)">Source: ticket PROJ-123</text>
<text x="562" y="129" class="mono" font-size="12" style="fill:var(--dg-muted)">"Dark mode for the…"</text>
<text x="554" y="162" class="mono" font-size="13" font-weight="600" style="fill:var(--dg-ink)">## Why</text>
</svg>
</div>
<figcaption>From a Jira ticket to a plan: you name the ticket, Claude reads it once without changing it, and the plan cites it on its first line.</figcaption>
</figure>

<div class="example">
<p class="example-title">Example</p>

```text
> Prepare a plan for PROJ-123
⏺ atlassian - getJiraIssue (MCP)(issueIdOrKey: "PROJ-123")
  ⎿ Dark mode for the dashboard · Story · In Progress
⏺ Write(_compagnon/changes/add-dark-mode/proposal.md)
  ⎿ Wrote 41 lines to _compagnon/changes/add-dark-mode/proposal.md
⏺ The plan is ready and cites PROJ-123 on its first line. Nothing
  changed in Jira.
```

The top of the [proposal](../glossary.md#proposal) then reads:

```markdown
# Proposal: Add dark mode

> Source: ticket **PROJ-123** — "Dark mode for the dashboard" (In Progress)

## Why
```

</div>

Any request to `/compagnon-propose` works, as long as it names a ticket:
"Plan PROJ-123, and keep the old theme as an option" adds your own words
to the ticket's.

## Set it up

Someone sets this up once per computer, and `compagnon init` usually
does it alone: it finds your Jira connector and writes its name in the
project's configuration. If a ticket name is treated as plain text, ask
the person who installed Compagnon to look below.

<details>
<summary>Under the hood: set it up</summary>

Jira reaches Claude Code through an [MCP](../glossary.md#mcp) server — a
connector to another tool. The `compagnon` command itself never talks to
Jira: Claude calls the tool during your session. Compagnon only needs the
name of its ticket-reading tool, in the project configuration:

```yaml
# _compagnon/config.yaml
mcp:
  jira_tool: mcp__atlassian__getJiraIssue
```

`compagnon init` fills it in for you when it finds a server whose name,
command or URL mentions Jira or Atlassian in `.mcp.json`,
`.claude/settings.json` or `~/.claude.json`:

```text
  ✓ Jira MCP detected: mcp__atlassian__getJiraIssue
    (source: .mcp.json → server "atlassian")
```

The tool name follows Claude Code's convention,
`mcp__<server name>__getJiraIssue`, with spaces and dots in the server name
replaced by underscores — for example, a connector named
`claude.ai Atlassian Rovo` gives
`mcp__claude_ai_Atlassian_Rovo__getJiraIssue`. If yours differs, look for
the exact name in the tool list of a Claude Code session and set it by hand.

At install time, Compagnon adds the tool to the `allowed-tools` of
`compagnon-propose`, so the skill may call it and nothing else from that
server. After changing `mcp:`, regenerate the skills:

```bash
compagnon update --force
```

Without `mcp.jira_tool`, a ticket name in your request is ordinary text:
the skill behaves exactly as in a project without any MCP integration.

> **Note**
> `mcp:` is never inherited from an [inherited source](inherited-sources.md):
> tool names depend on each person's Claude Code setup, not on a shared
> repository.

</details>

## What Claude may do with Jira

A ticket is anything matching `[A-Z]{2,}-\d+`, such as `PROJ-123`. When
your request names one and the tool is configured, `/compagnon-propose`
keeps to these rules:

- **One call**, to the one configured tool, for the first ticket named.
- **Read-only**: never a search, a comment or a status change.
- **First ticket only**: other tickets you name are listed under the
  citation, for you to look up.
- **No connection, no guess**: if the MCP server is not available in the
  session, Claude says so, writes the proposal from your request alone,
  and cites the ticket as "content not retrieved".

## Other MCP servers

Jira is the only integration today. Reading a Confluence page or a design
tool is not a setting you can add: each integration is part of Compagnon
itself, so a new one is a contribution. See
[CONTRIBUTING.md](https://github.com/mairistem/compagnon/blob/main/CONTRIBUTING.md)
to propose one.

<details>
<summary>Under the hood: adding another MCP integration</summary>

Each integration is a configuration key and a placeholder in a
[workflow](../glossary.md#workflow). The pattern is the same as for Jira:

1. Add a key under `mcp:` in the project configuration
   (`crates/compagnon-engine/src/config.rs`).
2. Carry it into the skill render context and substitute a placeholder such
   as `{{JIRA_MCP_TOOL}}` in both the `allowed-tools` and the body of the
   workflow that uses it (`crates/compagnon-agents/src/claude.rs`).
3. Describe the detection and the call in the workflow file under
   `assets/workflows/`, with a clean fallback when the key is absent.

The existing skill is extended rather than duplicated: there is one
`/compagnon-propose`, whatever the source of the request.

</details>

## Next step

[Reviewing a plan](review-a-plan.md) Read the plan Claude wrote before anything is built.
