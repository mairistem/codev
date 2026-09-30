# Skills Specification

## Purpose

Describes the contract of the skills that codev installs into Claude Code:
their name, what they must do, what they are not allowed to do, and how
their frontmatter guarantees these promises. Entries are added as the
changes that introduce each workflow land — one ADDED per workflow.

## Requirements

### Requirement: The `apply` skill guides the implementation of a change

The codev catalog SHALL expose an `apply` workflow — installed under
`.claude/skills/codev-apply/SKILL.md`, invocable as `/codev-apply` — whose
role is to process the unchecked tasks of a change's `tasks.md`, in file
order, checking each box as it goes.

#### Scenario: Implementing a change when only one is active

- **GIVEN** a project with a single active change whose `tasks.md` carries
  two unchecked tasks
- **WHEN** the user types `/codev-apply`
- **THEN** the skill implicitly resolves the active change
- **AND** implements the first task, then checks it
- **AND** implements the second task, then checks it

#### Scenario: Resuming after an interruption

- **GIVEN** a `tasks.md` in which the first task is already checked `- [x]`
  and the second is not
- **WHEN** the user types `/codev-apply`
- **THEN** the skill skips the already checked task
- **AND** starts with the first unchecked task

#### Scenario: Ambiguity requires an explicit choice

- **GIVEN** two active changes
- **WHEN** the user types `/codev-apply` without a name
- **THEN** the skill asks which one to apply, listing both names

### Requirement: The `apply` skill respects the boundaries of the change

The `apply` workflow MUST confine itself to what is necessary to check off
the tasks of the named change: it MUST NOT modify other changes, MUST NOT
archive or sync on its own, and MUST stop as soon as a task is ambiguous
or blocked rather than guess.

#### Scenario: Refusal to archive from apply

- **GIVEN** a change whose tasks are all checked
- **WHEN** the user types `/codev-apply`
- **THEN** the skill reports that the change is ready to be archived
- **AND** explicitly invites the user to run `/codev-archive` or
  `codev archive` as a separate next step

#### Scenario: An ambiguous task interrupts the flow

- **GIVEN** a `tasks.md` containing a task whose wording admits several
  interpretations that would materially change the outcome
- **WHEN** the skill reaches that task
- **THEN** the skill asks the user for clarification before
  implementing
- **AND** does not check the task until the clarification is obtained

### Requirement: Frontmatter contract of a codev skill

Every skill shipped by codev MUST carry a valid YAML frontmatter whose
`name` matches the name of the `.claude/skills/<name>/` directory, whose
`allowed-tools` field includes at least `Bash(codev:*)`, and whose
`metadata.version` matches the version of the binary that generated it.

#### Scenario: Frontmatter parsed by a third-party YAML reader

- **GIVEN** a skill shipped by the current version of the binary
- **WHEN** its frontmatter is extracted and passed to a standard YAML parser
- **THEN** the parser returns `name`, `allowed-tools` and `metadata.version`
  without error
- **AND** `metadata.version` equals the version the binary reports

#### Scenario: Hand edit detected on update

- **GIVEN** a skill whose body a user has edited by hand, without
  changing its version
- **WHEN** the user runs `codev update` again without `--force`
- **THEN** the skill is not overwritten
- **AND** the update report flags it as preserved

### Requirement: The `sync` skill merges a change's delta without moving it

The codev catalog SHALL expose a `sync` workflow — installed under
`.claude/skills/codev-sync/SKILL.md`, invocable as `/codev-sync` — whose
role is to bring a change's deltas into the main specs, while leaving the
change active where it is.

#### Scenario: Sync of a single active change

- **GIVEN** a project with a single active change whose planning is
  complete and which carries an ADDED delta on a new capability
- **WHEN** the user types `/codev-sync`
- **THEN** the skill implicitly resolves the active change
- **AND** runs `codev sync <name>`
- **AND** summarizes for the user the main specs created or updated

#### Scenario: Second sync is silent

- **GIVEN** a change already synced, none of whose main specs has changed
  since
- **WHEN** the user types `/codev-sync` a second time
- **THEN** the skill reports that there is nothing to do
- **AND** does not trigger any new write

#### Scenario: Sync never moves

- **GIVEN** a change whose merge succeeds
- **WHEN** the user types `/codev-sync`
- **THEN** the `_codev/changes/<name>/` directory still exists at its
  original location

#### Scenario: Sync suggests archiving after a change

- **GIVEN** a change whose merge modified at least one main spec
  (created or updated)
- **WHEN** the user types `/codev-sync`
- **THEN** the final output contains a line suggesting `/codev-archive`
  to close the cycle, phrased without insistence

#### Scenario: Sync without changes does not suggest anything

- **GIVEN** a change whose merge is a no-op (all main specs are already
  up to date)
- **WHEN** the user types `/codev-sync`
- **THEN** the final output reports the absence of changes
- **AND** does NOT suggest archiving — there is nothing new to propagate

#### Scenario: Sync refused by validation

- **GIVEN** a change that `codev validate` reports with an error
- **WHEN** the user types `/codev-sync`
- **THEN** the skill replies that the change has errors and points to
  `codev validate <name>`
- **AND** no main spec is modified

### Requirement: The `archive` skill closes a change with a strict pre-flight

The catalog SHALL expose an `archive` workflow — installed under
`.claude/skills/codev-archive/SKILL.md`, invocable as `/codev-archive` —
whose role is to merge the delta and then move the change to
`_codev/changes/archive/<date>-<name>/`. The skill MUST refuse to act if
`codev archive` reports a failed validation pre-flight.

#### Scenario: Archiving a validated change

- **GIVEN** a change whose planning is complete and which passes
  `codev validate`
- **WHEN** the user types `/codev-archive`
- **THEN** the skill runs `codev archive <name>`
- **AND** summarizes for the user the main specs touched
- **AND** names the dated archive destination

#### Scenario: Archive refused because of a validation error

- **GIVEN** a change in which a delta contains an error reported by
  `codev validate` (for example, a duplicate requirement)
- **WHEN** the user types `/codev-archive`
- **THEN** the skill does not insist
- **AND** explicitly invites the user to run `codev validate <name>`
  to see the details
- **AND** does not attempt to guess or fix the error

### Requirement: The `sync` and `archive` skills rely on the JSON contract

The `sync` and `archive` workflows MUST invoke the CLI with `--json` and
read the structured form (`SyncReportV1`, `ArchiveReportV1`) rather than
the human output — it is the public contract that codev guarantees stable
within its current version, and it is what makes the skill's output
reliable.

#### Scenario: Structured rendering of creations and updates

- **GIVEN** a change whose merge creates one main spec and updates
  another
- **WHEN** the user types `/codev-sync`
- **THEN** the output names the two distinctly — the created file and the
  updated file — each on its own line

#### Scenario: Archive refusal detected through a stable code

- **GIVEN** a change for which `codev archive --json` refuses with the code
  `validation_failed` in its `status` array
- **WHEN** the user types `/codev-archive`
- **THEN** the skill detects the stable code in the JSON
- **AND** says exactly: "The change has errors. Run `codev validate
  <name>` to see the details."
- **AND** does not parse the human message (which may be reworded without
  notice)

### Requirement: The `sync` and `archive` skills do not request general Bash

The `sync` and `archive` workflows MUST limit themselves to `Bash(codev:*)`
and read-only tools in their `allowed-tools` frontmatter — they run no
verification command other than those of the codev binary, unlike `apply`,
which must be able to run project tests.

#### Scenario: General Bash does not appear

- **GIVEN** the `sync` skill shipped by the current version
- **WHEN** its frontmatter is inspected
- **THEN** the `allowed-tools` string does not contain a bare `Bash` at the
  end of the list, only the `Bash(codev:*)` prefix
- **AND** the same rule holds for `archive`

### Requirement: The `update` skill revises a planning artifact

The codev catalog SHALL expose an `update` workflow — installed under
`.claude/skills/codev-update/SKILL.md`, invocable as `/codev-update` —
whose role is to revise an already written planning artifact (proposal,
specs, design or tasks) of an active change, based on a free-form
description given by the user.

#### Scenario: Revising a design after a new constraint

- **GIVEN** an active change whose `design.md` cites a technical decision X
- **WHEN** the user types `/codev-update design "replace X with Y because
  of constraint Z"`
- **THEN** the skill reads `design.md`, applies the requested revision, and
  writes the new version
- **AND** runs `codev validate <change>` again at the end

#### Scenario: Implicit resolution when a single change is active

- **GIVEN** a project with a single active change
- **WHEN** the user types `/codev-update proposal "reduce the
  scope"`
- **THEN** the skill implicitly resolves the active change
- **AND** applies the revision to that change's `proposal.md`

#### Scenario: Ambiguity about which change to revise

- **GIVEN** two active changes
- **WHEN** the user types `/codev-update tasks "…"` without naming a
  change
- **THEN** the skill asks the user which one to revise, listing both
  names
- **AND** writes nothing before getting the answer

### Requirement: The `update` skill announces the ripple effect before acting

When the requested revision of one artifact makes another inconsistent,
the skill MUST report it to the user and propose the correction before
writing it, rather than leaving the main spec, the design or the task list
silently out of agreement.

#### Scenario: Removing a capability from the proposal ripples onto specs

- **GIVEN** a `proposal.md` declaring two new capabilities `a` and
  `b`, and an already written `specs/b/spec.md` file
- **WHEN** the user types `/codev-update proposal "remove capability
  b — out of scope after all"`
- **THEN** the skill applies the revision to `proposal.md`
- **AND** reports to the user that `specs/b/spec.md` becomes orphaned
- **AND** offers to delete that file or to call
  `/codev-update specs …` to adjust it
- **AND** does not write that second modification without confirmation

#### Scenario: A revision without ripple applies without additional confirmation

- **GIVEN** a revision that touches only `design.md`, with no consequence
  for the other artifacts
- **WHEN** the user types `/codev-update design "…"`
- **THEN** the skill applies the revision without asking for additional
  confirmation

### Requirement: The `update` skill stays within the planning boundary

The `update` workflow MUST limit itself to files under
`_codev/changes/<name>/` and MUST NOT:

- modify project code;
- create a missing artifact (proposal, specs, design, tasks) — that is
  what `/codev-propose` does;
- touch a change already archived under `changes/archive/`.

#### Scenario: Refusal to write a missing artifact

- **GIVEN** a change whose `design.md` does not exist yet
- **WHEN** the user types `/codev-update design "add decision
  Z"`
- **THEN** the skill refuses
- **AND** explicitly points to `/codev-propose` to create the artifact

#### Scenario: Refusal of an archived change

- **GIVEN** a change that lives under `changes/archive/2026-09-09-<name>/`
- **WHEN** the user types `/codev-update proposal --change
  <archived-name>`
- **THEN** the skill refuses
- **AND** recalls that an archived change is history; correcting it
  requires un-archiving it by hand

### Requirement: The `onboard` skill introduces codev and recommends the next action

The codev catalog SHALL expose an `onboard` workflow — installed
under `.claude/skills/codev-onboard/SKILL.md`, invocable as
`/codev-onboard` — whose role is to introduce codev to a user
discovering it, in three blocks:

1. A short description of codev (two or three sentences).
2. The current state of the project — repository initialized or not,
   number of main specs, number of indexed local decisions, active
   changes listed by name, and **number of archived changes** (shown
   only when non-zero, so as not to clutter the output on a new
   project).
3. The recommended next action, adapted to the state:
   - `_codev/` missing → `codev init`.
   - Project initialized, no change, **and `_codev/config.yaml`
     thin** (no entry in `rules:`) → `/codev-configure` first, with a
     sentence explaining the benefit ("Claude will enrich the config
     from the project"), then `/codev-propose <idea>` second.
   - Project initialized, no change, config not thin → **invite the
     user to read `README.md` to get a feel for the project**, then
     `/codev-propose <idea>`; `/codev-explore <topic>` remains
     mentioned as an alternative.
   - An active change whose planning is incomplete →
     `/codev-propose <that-change>` to continue it.
   - An active change whose planning is complete →
     `/codev-apply <that-change>`.
   - Several active changes → list them and let the user choose.

The skill MUST be **strictly read-only**: `allowed-tools` limited
to `Bash(codev:*), Read, Glob`. Neither `Write`, nor `Edit`, nor
general `Bash`.

#### Scenario: Role documented in the catalog

- **GIVEN** the codev workflow catalog
- **WHEN** the `onboard` workflow is resolved
- **THEN** its entry exists (`find("onboard").is_some()`)
- **AND** its `allowed_tools` is exactly
  `"Bash(codev:*), Read, Glob"`
- **AND** its `allowed_tools` does NOT contain general `Bash` (invariant
  rule: only `apply` has it)
- **AND** its `body` cites the three blocks (description, state,
  recommended action)

#### Scenario: Skill installed by a `codev update`

- **GIVEN** a project whose `config.yaml` has `workflows: [propose,
  explore, apply, sync, archive, update, onboard]`
- **WHEN** the user runs `codev update`
- **THEN** the file `.claude/skills/codev-onboard/SKILL.md` is
  created
- **AND** its YAML frontmatter is valid and carries the expected
  description

#### Scenario: The "here you have" block mentions archived changes when there are some

- **GIVEN** a project containing at least one change in
  `_codev/changes/archive/`
- **WHEN** the user runs `/codev-onboard`
- **THEN** the "here you have" block contains a line stating the
  number of archived changes
- **AND** that number matches the number of directories of the form
  `<date>-<name>/` under `_codev/changes/archive/`

#### Scenario: The "here you have" block adds no archived line on a new project

- **GIVEN** a freshly initialized project, with no archived
  change
- **WHEN** the user runs `/codev-onboard`
- **THEN** the "here you have" block **does not show** an
  "archived changes" line — the output stays short and uncluttered

#### Scenario: Configure recommendation when the config has no rules

- **GIVEN** an initialized project with no active change, whose
  `_codev/config.yaml` has no entry in `rules:`
- **WHEN** the user runs `/codev-onboard`
- **THEN** the "what's next" block cites `/codev-configure` first,
  with a sentence about the expected benefit
- **AND** mentions `/codev-propose <idea>` second

#### Scenario: Default recommendation cites README.md when the config has rules

- **GIVEN** an initialized project with no active change whose
  `_codev/config.yaml` carries at least one entry in `rules:`
- **WHEN** the user runs `/codev-onboard`
- **THEN** the "what's next" block invites the user to read `README.md`
  before creating a change
- **AND** cites `/codev-propose <an-idea>` as the actionable step and
  mentions `/codev-explore <topic>` as an alternative

### Requirement: `onboard` is part of the default catalog

The `DEFAULT_WORKFLOWS` array of `codev-agents::workflows` MUST
contain the **complete list of the 8 codev workflows**: `propose`,
`explore`, `onboard`, `apply`, `sync`, `archive`, `update` and
`configure`. A user who runs `codev init --yes` (or from a
non-interactive pipe) on a new project, without a `workflows:` key in
their `config.yaml`, therefore gets all available skills
immediately — including `configure`, the recommended entry point
after init.

A project that wants to restrict the catalog MUST declare an explicit
`workflows:` key with a chosen subset — this is the opt-out path,
instead of the former opt-in path.

This switch solves a discoverability problem: under the former default
(3 workflows), a user who typed `/codev-apply` after
`/codev-propose` could not find the skill and assumed it did not
exist.

#### Scenario: The default catalog includes the 8 workflows

- **GIVEN** a project whose `config.yaml` has no `workflows:`
  key
- **WHEN** `select(None)` is called on the catalog
- **THEN** the list of returned `id`s is exactly
  `["propose", "explore", "onboard", "apply", "sync", "archive", "update", "configure"]`
- **AND** no warning is emitted

#### Scenario: Opt-out restriction through explicit workflows

- **GIVEN** a project whose `config.yaml` contains
  `workflows: [propose, explore, onboard]`
- **WHEN** `select` is called with that list
- **THEN** only those three skills are returned
- **AND** `apply`, `sync`, `archive`, `update`, `configure` are
  **not** installed

### Requirement: The `propose` skill detects mentioned Jira tickets and enriches the proposal

The catalog's `propose` workflow SHALL, before resolving the change
name, scan the user's prompt for a ticket identifier matching the
regular expression `[A-Z]{2,}-\d+` (for example `PROJ-123`,
`ABC-42`).

When at least one ticket is detected, the skill MUST:

1. **Attempt to call** the MCP tool whose name is declared on the
   project side via `_codev/config.yaml.mcp.jira_tool`. This name is
   injected when the skill is installed, into the `allowed-tools`
   frontmatter and into the body — the user/organization chooses
   **its own** MCP (`mcp__claude_ai_Atlassian__getJiraIssue`,
   `mcp__claude_ai_Atlassian_Rovo__getJiraIssue`, or another) without
   touching codev's source code.
2. **A single call per invocation**, on the ticket identifier
   **mentioned earliest in the prompt**; the others are merely
   named.
3. **On success**: inject the content (title, description, status,
   type) into the drafting context, and place the ticket at the top of
   `proposal.md` as a quote line
   `> Source: ticket **<ID>** — "<title>" (<status>)`.
4. **On failure — MCP tool unavailable in the session or not
   configured on the project side**: display an informational message
   to the user, then continue the usual flow without the ticket's
   content. The proposal still cites the ticket at the top
   ("Source: ticket **<ID>** — content not retrieved").
5. **When no pattern is found**: behavior bit-for-bit identical to
   today — no MCP call, no message.

The workflow CATALOG MUST use a textual placeholder
`{{JIRA_MCP_TOOL}}` in `allowed_tools` and in the body of the
`propose` workflow, instead of a hardcoded MCP name. The placeholder
is substituted when the installation frontmatter is rendered by
`ClaudeCode::render`.

When `_codev/config.yaml.mcp.jira_tool` is **absent** or empty,
rendering MUST:

- cleanly remove `{{JIRA_MCP_TOOL}}` from `allowed_tools` **and** the
  comma preceding it (so as not to leave a malformed `allowed-tools`
  ending with `", "`);
- replace every occurrence in the body with the string
  `(Jira MCP not configured)` — the skill remains installed and
  functional in every other respect, but no longer calls any MCP.

#### Scenario: `mcp.jira_tool` configuration present → functional skill

- **GIVEN** a project whose `_codev/config.yaml` declares
  `mcp: { jira_tool: mcp__claude_ai_Atlassian_Rovo__getJiraIssue }`
- **WHEN** the user runs `codev update`
- **THEN** the file `.claude/skills/codev-propose/SKILL.md` carries
  `allowed-tools: "Bash(codev:*), Read, Write, Edit, Glob, Grep, mcp__claude_ai_Atlassian_Rovo__getJiraIssue"`
- **AND** the skill body cites
  `mcp__claude_ai_Atlassian_Rovo__getJiraIssue` wherever the CATALOG
  contains `{{JIRA_MCP_TOOL}}`
- **AND** no `{{…}}` remains in the installed file

#### Scenario: `mcp.jira_tool` configuration absent → clean fallback

- **GIVEN** a project whose `_codev/config.yaml` has no `mcp:` block
- **WHEN** the user runs `codev update`
- **THEN** the file `.claude/skills/codev-propose/SKILL.md` carries
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
  is emitted
- **AND** the `proposal.md` of the created change contains at the top a line
  `> Source: ticket **PROJ-123** — "<title>" (<status>)`

#### Scenario: Ticket mentioned, MCP absent → proposal anyway

- **GIVEN** a user types `/codev-propose fix PROJ-123`
- **AND** no Jira MCP is configured (`mcp.jira_tool` absent or the
  declared tool unavailable)
- **WHEN** the skill runs
- **THEN** the skill displays an informational message naming
  `PROJ-123` and stating that the Jira MCP is not active
- **AND** the proposal is created anyway, with `PROJ-123` mentioned at
  the top along with "content not retrieved"

#### Scenario: No ticket mentioned → unchanged behavior

- **GIVEN** a user types `/codev-propose add
  authentication`
- **WHEN** the skill runs
- **THEN** no MCP call is emitted
- **AND** no ticket-related message appears
- **AND** the proposal is drafted exactly as before this batch of changes

### Requirement: Skills write artifact prose in the configured language

The skills that write planning content — `propose`, `update` and
`configure` — SHALL write their prose in the language set by `language:` in
`_codev/config.yaml` (`en` when absent), whatever language the conversation
is in. `codev instructions --json` MUST expose that language as `language`.
Structural keywords that codev parses — template headings such as `## Why`
or `### Requirement:`, delta section headings, `**WHEN**` / `**THEN**`,
`SHALL` / `MUST` — MUST stay in English.

#### Scenario: French prose, English structure

- **GIVEN** a project whose config contains `language: fr`
- **WHEN** the user runs `/codev-propose` in an English conversation
- **THEN** the proposal's sentences are written in French
- **AND** its headings are `## Why`, `## What Changes`, `## Capabilities`
  and `## Impact`
