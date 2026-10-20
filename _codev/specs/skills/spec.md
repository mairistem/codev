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
`metadata.version` matches the version of the binary that generated it,
and whose `metadata.checksum` is the SHA-256, in hexadecimal, of the
skill's whole content with that `checksum` line left out. A skill whose
content no longer matches its checksum has been edited by hand.

#### Scenario: Frontmatter parsed by a third-party YAML reader

- **GIVEN** a skill shipped by the current version of the binary
- **WHEN** its frontmatter is extracted and passed to a standard YAML parser
- **THEN** the parser returns `name`, `allowed-tools`, `metadata.version`
  and `metadata.checksum` without error
- **AND** `metadata.version` equals the version the binary reports

#### Scenario: Hand edit detected on update

- **GIVEN** a skill whose body a user has edited by hand, without
  changing its version
- **WHEN** the user runs `codev update` again without `--force`
- **THEN** the skill is not overwritten
- **AND** the update report flags it as preserved

#### Scenario: The checksum matches the content

- **GIVEN** a skill as the binary renders it
- **WHEN** the SHA-256 of its content without the `checksum` line is
  computed
- **THEN** it equals `metadata.checksum`

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

### Requirement: The default schema gives each artifact a role with "Done when" criteria

Each artifact of the embedded `spec-driven` schema SHALL declare its role as
data — an id, a display name, the question it owns and a list of "done
when" criteria — which `codev instructions <artifact> --json` returns in
its `role` field, with the origin `schema:spec-driven` on each criterion.
The roles are:

- `proposal` — `product` (product owner), owns why and what;
- `specs` — `qa` (QA analyst), owns observable behavior;
- `design` — `architect`, owns how;
- `tasks` — `lead` (tech lead), owns sequencing and verification.

The tech lead's criteria MUST require that the documentation affected by
the change is updated in a task, or that the plan states none is affected;
that every spec scenario is covered by an automated test written in a task,
a manual check being acceptable only when automation is impossible and with
the reason stated; and that tests follow the project's existing test suite,
the testing conventions of its language, and its `rules`. The `proposal`
instruction MUST ask the Impact section to name the affected documentation,
MUST describe a `## Domains` section, after `## Impact`: the six domains
`security`, `personal-data`, `user-interface`, `performance`, `operations`
and `migration` as a list to tick, or the single line `None`, and MUST
describe a `## Release note` section, after `## Domains`: bullets
`- <Category>: <text>` with Category among `Added`, `Changed`,
`Deprecated`, `Removed`, `Fixed` and `Security`, breaking items starting
with `**BREAKING**`, or the single line `None` for an internal change. The
product owner's criteria MUST require that the release note is written for
end users, categorized, with breaking items marked, or `None`, and that
the Domains section ticks every domain the change touches, or says `None`.
The embedded proposal template MUST carry the `## Domains` section, with
the six domains unticked, and the `## Release note` section. The
architect's criteria MUST require that the approach is the simplest that
satisfies the specs, and that a newcomer can follow the design without
knowing its history.

The instructions of the default schema MUST NOT restate the role: they no
longer start with a `Role:` line or a `Done when:` list. A custom
schema's instructions MUST be returned as written; its artifacts carry a
role only when the schema declares one.

#### Scenario: The proposal carries the product owner role

- **GIVEN** a change using the `spec-driven` schema
- **WHEN** the user runs `codev instructions proposal --change <name> --json`
- **THEN** `role.id` is `product` and `role.name` is `Product owner`
- **AND** `role.doneWhen` includes marking every breaking change with
  **BREAKING**
- **AND** `instruction` asks the Impact section to name the affected
  documentation, and does not start with `Role:`
- **AND** `role.doneWhen` requires a release note written for end users,
  categorized, with breaking items marked, or `None`
- **AND** `template` has a `## Release note` section after `## Impact`

#### Scenario: The proposal lists the domains it touches

- **GIVEN** a change using the `spec-driven` schema
- **WHEN** the user runs `codev instructions proposal --change <name> --json`
- **THEN** `template` has a `## Domains` section between `## Impact` and
  `## Release note`, listing the six domains unticked
- **AND** `instruction` describes the section and its `None`
- **AND** `role.doneWhen` requires every domain the change touches to be
  ticked, or `None`

#### Scenario: The tasks criteria cover every spec scenario

- **GIVEN** a change using the `spec-driven` schema
- **WHEN** the user runs `codev instructions tasks --change <name> --json`
- **THEN** `role.id` is `lead`
- **AND** `role.doneWhen` requires every spec scenario to be covered by at
  least one task's verification

#### Scenario: The tasks criteria require documentation and automated tests

- **GIVEN** a change using the `spec-driven` schema
- **WHEN** the user runs `codev instructions tasks --change <name> --json`
- **THEN** `role.doneWhen` requires the affected documentation to be
  updated in a task, or the plan to state that none is affected
- **AND** it requires every spec scenario to be covered by an automated
  test, a manual check only when automation is impossible, with the reason
- **AND** it requires tests to follow the project's existing test suite,
  the conventions of its language and its `rules`

#### Scenario: The design criteria require the simplest approach

- **GIVEN** a change using the `spec-driven` schema
- **WHEN** the user runs `codev instructions design --change <name> --json`
- **THEN** `role.id` is `architect`
- **AND** `role.doneWhen` requires the approach to be the simplest that
  satisfies the specs, and the design to be followable by a newcomer
  without knowing its history

#### Scenario: A custom schema gets no role added

- **GIVEN** a change using a project schema whose artifact declares no role
- **WHEN** the user runs `codev instructions <artifact> --change <name>
  --json`
- **THEN** `instruction` is the schema's text, with no `Role:` line added
- **AND** `role` is `null`

### Requirement: The `propose` skill checks traceability before presenting the plan

Once every required artifact is written and before showing the final
status, the `propose` skill SHALL re-read the artifacts from disk and check
that:

- every capability listed in the proposal has its spec file, and every spec
  file is listed in the proposal;
- every requirement has a nominal scenario and an error or edge-case
  scenario, or states why none applies;
- every scenario is covered by a task whose verification refers to it and
  names an automated test, or says why automation is impossible;
- a task updates the documentation when the proposal's Impact touches
  documented behavior, or the plan states that none is affected;
- the release note agrees with the plan: every **BREAKING** item of What
  Changes, and every REMOVED requirement or MODIFIED requirement that
  breaks users, has a breaking item, and an internal change says `None`;
- every task names a concrete verification.

The skill MUST fix the gaps it finds directly in the artifacts. The check
MUST NOT modify any code file.

The skill SHALL take this check's criteria from
`codev roles --at plan.trace --change <name> --json` — the `tracer`'s,
followed by the project's extra criteria — and apply them all. When no
role is active at `plan.trace`, the skill MUST skip the check and its
summary MUST say that the traceability check is disabled by configuration.

#### Scenario: A requirement without an error scenario gets one

- **GIVEN** a plan whose spec has a requirement with only a nominal scenario
- **WHEN** `/codev-propose` reaches the traceability check
- **THEN** the requirement gains an error or edge-case scenario, or a stated
  reason why none applies
- **AND** a task's verification covers the new scenario

#### Scenario: A spec file missing from the proposal is listed

- **GIVEN** a plan with a spec file whose capability is not listed in the
  proposal
- **WHEN** `/codev-propose` reaches the traceability check
- **THEN** the proposal lists the capability, or the spec file is removed
  if the capability is out of scope

#### Scenario: A plan without a documentation task gets one

- **GIVEN** a plan whose Impact changes a documented command and whose
  tasks update no documentation
- **WHEN** `/codev-propose` reaches the traceability check
- **THEN** the tasks gain a documentation task, or the plan states why no
  documentation is affected

#### Scenario: A scenario verified only by hand gets an automated test

- **GIVEN** a plan whose only task covering a scenario is verified by a
  manual check, with no reason given
- **WHEN** `/codev-propose` reaches the traceability check
- **THEN** the covering task names an automated test, or states why
  automation is impossible

#### Scenario: A breaking change missing from the release note is added

- **GIVEN** a plan whose What Changes marks an item **BREAKING** and whose
  release note has no breaking item
- **WHEN** `/codev-propose` reaches the traceability check
- **THEN** the release note gains the breaking item

#### Scenario: A complete plan is left unchanged

- **GIVEN** a plan where capabilities, spec files, scenarios, automated
  tests, documentation tasks, task verifications and the release note all
  cover each other
- **WHEN** `/codev-propose` reaches the traceability check
- **THEN** no artifact is modified by the check

#### Scenario: The traceability check disabled by configuration

- **GIVEN** a configuration with `roles: { tracer: never }`
- **WHEN** `/codev-propose` has written the artifacts
- **THEN** no traceability check runs
- **AND** the summary says the traceability check is disabled by
  configuration

### Requirement: The `propose` skill challenges the plan and reports points to challenge

After the traceability check, the `propose` skill SHALL re-read the
artifacts as reviewers who did not write them: every role
`codev roles --at plan.challenge --change <name> --json` returns — the
`skeptic` and the experts the change's domains activate — each along its
criteria, the project's extra criteria included, and using its
`knowledge` when it has one. The `skeptic`'s grid covers the need (does
the change solve the Why, is there a simpler solution or one without
code), the scope (anything nobody asked for, an obvious case missing), the
specs (error and edge cases, each requirement verifiable), the decisions
(a silent contradiction with a decision in effect), the assumptions (the
one that would bring the plan down if wrong) and the tasks (each one
verifiable), the domains (every domain the plan touches is ticked in the
proposal's `## Domains`, so that its expert reviews it), as well as the
project's `rules:` received through `codev instructions`. The `skeptic`
challenges the plan's logic and evidence; it MUST NOT review a domain
itself — security, privacy, compatibility and migration, operations,
performance, accessibility — which belongs to the expert that domain
activates. An expert asks whether the design addresses its domain.

Each finding MUST be written to the change's review record under
`## plan.challenge`, then answered as the review record's answer loop
describes: the author fixes what is unambiguous directly in the artifacts
or disputes it, the reviewer re-checks once, and what remains is for the
human. The pass MUST NOT add scope.

The final summary of the skill MUST end with a "Points to challenge" block:
at most five items in total across the roles, taken from the findings the
record reports to the human (confidence `high`) that were not rejected,
ranked by severity, each on one line tagged with the id of the role that
raised it and giving its finding id, its status, what, why it matters and
the artifact concerned. When the pass finds nothing worth raising, the
block MUST say "No point to challenge found" instead of inventing points.

When no role is active at `plan.challenge`, the skill MUST skip the pass,
and the "Points to challenge" block MUST say that the skeptic is disabled
by configuration instead of listing points.

#### Scenario: Points to challenge are reported

- **GIVEN** a plan whose design rests on an unverified assumption
- **WHEN** `/codev-propose` presents the plan
- **THEN** the summary contains a "Points to challenge" block naming the
  assumption, why it matters and the artifact concerned, tagged `skeptic`
- **AND** the same point is a finding of `review.md` under
  `## plan.challenge`, with its status

#### Scenario: At most five points

- **GIVEN** a plan in which the skeptic finds four weak points and the
  `security` expert four more
- **WHEN** `/codev-propose` presents the plan
- **THEN** the "Points to challenge" block lists at most five items in
  total, the most severe first, each tagged with its role id
- **AND** all eight are findings of the record

#### Scenario: An active expert challenges the design

- **GIVEN** a change whose proposal ticks `personal-data` and whose design
  stores a new e-mail field with no retention rule
- **WHEN** `/codev-propose` challenges the plan
- **THEN** the missing retention rule is a finding of the record under
  `### privacy`, fixed in the design or raised as a point to challenge

#### Scenario: Nothing to challenge

- **GIVEN** a plan in which the contrarian pass finds nothing worth raising
- **WHEN** `/codev-propose` presents the plan
- **THEN** the summary says "No point to challenge found"
- **AND** lists no invented point
- **AND** each role's section of the record says `No finding.`

#### Scenario: The skeptic leaves a domain to its expert

- **GIVEN** a plan whose proposal ticks `operations`, and whose design
  calls a new provider without stating a timeout
- **WHEN** the contrarian pass runs
- **THEN** the missing timeout is not a finding of the `skeptic`
- **AND** the `skeptic`'s criteria name the domains as the experts' own

#### Scenario: A touched domain left unticked

- **GIVEN** a plan whose design stores a new e-mail field, and whose
  proposal leaves `personal-data` unticked with no reason
- **WHEN** the contrarian pass runs
- **THEN** the `skeptic` records a finding at `proposal.md#Domains`
  saying that the privacy expert will not review the plan

#### Scenario: The pass does not add scope

- **GIVEN** a plan for which the reviewer thinks of a useful but unrequested
  feature
- **WHEN** the contrarian pass runs
- **THEN** the feature is not added to the artifacts

#### Scenario: The skeptic disabled by configuration

- **GIVEN** a configuration with `roles: { skeptic: never }` and a change
  that ticks no domain
- **WHEN** `/codev-propose` presents the plan
- **THEN** no contrarian pass has run
- **AND** the "Points to challenge" block says the skeptic is disabled by
  configuration

#### Scenario: A project criterion joins the grid

- **GIVEN** a configuration where `skeptic` has the extra criterion
  `The rollback plan is stated`
- **WHEN** the contrarian pass runs on a plan with no rollback plan
- **THEN** the missing rollback plan is fixed or raised as a point to
  challenge

### Requirement: The `onboard` skill relays what `codev doctor` reports missing

When the repository is initialized, the `onboard` skill SHALL run
`codev doctor --json` and, in its "here you have" block, list the checks
whose `status` is `missing` — at most five, one line each, with their
`fix` verbatim — or say that the setup is complete when none is missing.
Warnings MAY be summarized as a count. The recommended next action of the
"what's next" block is unchanged.

The skill MUST stay read-only: it never runs a fix itself, `codev init`
and `codev update` included, and its `allowed-tools` stay
`Bash(codev:*), Read, Glob`. If `codev doctor` fails or is not available,
the skill continues without the list.

#### Scenario: Missing checks are listed with their fix

- **GIVEN** an initialized project whose `codev doctor --json` reports
  `config.rules` and `specs` as `missing`
- **WHEN** the user runs `/codev-onboard`
- **THEN** the "here you have" block lists both checks with their fix
  (`/codev-configure`, `/codev-propose`)
- **AND** no file of the project is modified

#### Scenario: A complete setup is said in one line

- **GIVEN** an initialized project where `codev doctor --json` reports no
  `missing` check
- **WHEN** the user runs `/codev-onboard`
- **THEN** the "here you have" block says the codev setup is complete

#### Scenario: Doctor unavailable

- **GIVEN** an initialized project where `codev doctor --json` fails
- **WHEN** the user runs `/codev-onboard`
- **THEN** the skill still shows its three blocks, without the list of
  missing checks

#### Scenario: The skill text carries the doctor step

- **GIVEN** the codev workflow catalog
- **WHEN** the `onboard` workflow is resolved
- **THEN** its body names `codev doctor --json`
- **AND** its `allowed_tools` is still exactly `"Bash(codev:*), Read, Glob"`

### Requirement: The `apply` skill tests by the project's conventions and verifies before checking off

When it implements a task, the `apply` skill SHALL write tests that follow
the project's existing test suite first, then the conventions of the
project's language and the project's `rules`. It MUST run the project's
test command, found from the project's `context` or `rules`, its manifest or
its CI configuration, and MUST NOT invent one. It MUST check a task off only
once the task's verification passes. Before reporting the change as done, it
MUST run the project's full test suite once and report a failure instead of
the completion summary.

#### Scenario: Tests follow the existing suite

- **GIVEN** a project whose tests live in a dedicated directory with a
  naming pattern
- **WHEN** `/codev-apply` implements a task that adds behavior
- **THEN** the new tests are placed and named like the existing ones

#### Scenario: A failing verification keeps the task unchecked

- **GIVEN** a task whose cited test fails after implementation
- **WHEN** `/codev-apply` verifies the task
- **THEN** the task stays `- [ ]` until the test passes

#### Scenario: The full suite runs before the change is reported done

- **GIVEN** a change whose tasks are all checked
- **WHEN** `/codev-apply` finishes
- **THEN** it has run the project's full test suite once
- **AND** a failure is reported instead of the "ready to be archived"
  summary

#### Scenario: No test command can be found

- **GIVEN** a project whose `context`, `rules`, manifest and CI
  configuration name no test command
- **WHEN** `/codev-apply` needs to run the tests
- **THEN** it asks the user for the command rather than guessing one

### Requirement: Skills offer to update codev at the start of an action

The `propose`, `apply`, `update`, `sync` and `archive` skills SHALL read
the `update` and `versionRequirement` objects of the first
`codev status --json` they run, and only that one, before acting:

- when `versionRequirement.satisfied` is `false`, tell the user in one
  line that the project requires codev `<requirement>` while
  `<update.current>` runs, then ask the question below;
- otherwise, when `update.available` is `true` and `update.snoozed` is
  `false`, tell the user in one line that codev `<update.latest>` is
  available, with `update.url`, then ask the question below;
- otherwise, say nothing about updates.

The question offers two answers, "Update now" and "Later". "Update now"
runs `codev self-update --yes`, reports its outcome in one line, then
resumes the original action. "Later" runs `codev self-update --snooze`
for an available update — never for an unsatisfied requirement — and
resumes the original action. The question MUST NOT be asked again during
the same action, nor in the middle of one: `/codev-apply` asks it before
implementing its first task, never between tasks. The skills' allowed
tools are unchanged — `codev self-update` runs under `Bash(codev:*)`. The
`explore`, `onboard` and `configure` skills never ask it.

#### Scenario: Update available at the start of `/codev-propose`

- **GIVEN** `codev status --json` reports `update.available: true` and
  `update.snoozed: false`
- **WHEN** `/codev-propose` reads it
- **THEN** the skill names the new version and its release URL in one
  line and asks "Update now" or "Later" before writing any artifact

#### Scenario: Update now, then resume

- **GIVEN** the user answers "Update now"
- **WHEN** the skill continues
- **THEN** it runs `codev self-update --yes`
- **AND** resumes the action it was invoked for

#### Scenario: Later snoozes and resumes

- **GIVEN** the user answers "Later" to an available update
- **WHEN** the skill continues
- **THEN** it runs `codev self-update --snooze`
- **AND** resumes the action it was invoked for

#### Scenario: Snoozed or unknown, nothing is said

- **GIVEN** `codev status --json` has no `update` object, or
  `update.snoozed: true`, and no unsatisfied `versionRequirement`
- **WHEN** a skill reads it
- **THEN** the skill says nothing about updates

#### Scenario: Team requirement not met

- **GIVEN** `versionRequirement.satisfied: false`
- **WHEN** `/codev-apply` reads it
- **THEN** the skill names the requirement and the running version, and
  asks the same question before the first task
- **AND** "Later" does not snooze anything

#### Scenario: Only the five action skills carry the notice

- **GIVEN** the codev workflow catalog
- **WHEN** the skills are rendered
- **THEN** the `propose`, `apply`, `update`, `sync` and `archive` skills
  carry the update notice, and `explore`, `onboard` and `configure` do
  not
- **AND** no skill's allowed tools change

### Requirement: `codev update` regenerates the skills of the configured workflows

`codev update` SHALL write one skill per workflow listed by `workflows:` in
`_codev/config.yaml`, or per workflow of the default catalog when the key is
absent, at `.claude/skills/codev-<workflow>/SKILL.md`, rendered by the
running codev:

- a missing skill is created;
- a skill identical to the rendering is left untouched;
- a skill stamped with another codev version is rewritten;
- among the skills stamped with the running version, one whose content
  matches its own checksum is codev's rendering and is rewritten; one
  whose content no longer matches its checksum is edited by hand and is
  preserved; one without a checksum, rendered before checksums, is
  preserved as edited by hand unless it differs from codev's rendering
  only by the review mode, as the review-mode requirement says, the
  checksum line left out of the comparison;
- with `--force`, every skill is rewritten.

An unknown workflow name SHALL be ignored with a warning `unknown_workflow`
listing the workflows this codev provides; the other skills are written.
The skill of a workflow no longer listed SHALL be left in place: `codev
update` deletes no skill. `codev update` SHALL also recreate the missing
folders of `_codev/` and a missing `_codev/config.yaml`, without
overwriting any existing file. Its handling of codev's managed sections is
specified by the `init` capability. Outside a codev project it fails with
`no_codev_root` and writes nothing.

The human output SHALL start with `codev updated in <project root>`, list
the installed skills, then say how many files were created and updated, or
`Nothing to do, everything is already in place.`, and finally list the
preserved skills with a hint to rerun with `--force`. With `--json`, the
document SHALL carry `root`, `created`, `updated`, `untouched`,
`preserved`, `skills` (the names of the configured skills), `integration`
and `status` (warnings). The command exits with code 0.

#### Scenario: Missing and outdated skills are written

- **GIVEN** an initialized project whose `codev-explore` skill was deleted
  and whose `codev-sync` skill is stamped with an earlier codev version
- **WHEN** the user runs `codev update --json`
- **THEN** `created` lists the `codev-explore` skill and `updated` the
  `codev-sync` skill
- **AND** both carry the running codev's version stamp

#### Scenario: A second run changes nothing

- **GIVEN** a project where `codev update` just ran
- **WHEN** the user runs `codev update`
- **THEN** `created` and `updated` are empty
- **AND** the human output says `Nothing to do, everything is already in
  place.`

#### Scenario: `--force` rewrites a hand-edited skill

- **GIVEN** a project whose `codev-apply` skill was edited by hand without
  changing its version stamp
- **WHEN** the user runs `codev update --force --json`
- **THEN** `updated` lists the `codev-apply` skill and `preserved` is empty
- **AND** the skill is back to what codev renders

#### Scenario: An unknown workflow is ignored with a warning

- **GIVEN** a configuration with `workflows: [propose, bogus]`
- **WHEN** the user runs `codev update --json`
- **THEN** `skills` is `["codev-propose"]`
- **AND** `status` holds a warning `unknown_workflow` naming `bogus`

#### Scenario: A workflow removed from the configuration keeps its skill

- **GIVEN** a project where every default skill is installed, then
  `workflows: [propose]` in the configuration
- **WHEN** the user runs `codev update`
- **THEN** `.claude/skills/codev-apply/SKILL.md` still exists, unchanged

#### Scenario: A deleted `_codev/` folder is recreated

- **GIVEN** an initialized project whose `_codev/specs/` folder was deleted
- **WHEN** the user runs `codev update --json`
- **THEN** `created` lists `_codev/specs/.gitkeep`
- **AND** `_codev/config.yaml` is listed in `untouched`, unchanged

#### Scenario: Outside a project

- **GIVEN** a folder with no `_codev/` in it or its parents
- **WHEN** the user runs `codev update --json`
- **THEN** the exit code is 1 and `status[0].code` is `no_codev_root`
- **AND** `root` is `null` and `skills` is empty

#### Scenario: A skill rendered by an earlier build of the same version

- **GIVEN** a project whose `codev-apply` skill was rendered by a build
  of the same version whose skill text differed, untouched since, with a
  `checksum` line matching its content
- **WHEN** the user runs `codev update --json`
- **THEN** `updated` lists the `codev-apply` skill and `preserved` is
  empty

#### Scenario: A hand edit of a checksummed skill

- **GIVEN** a project whose `codev-apply` skill a user edited by hand
  after codev rendered it with its checksum
- **WHEN** the user runs `codev update --json`
- **THEN** `preserved` lists the `codev-apply` skill, left as edited

#### Scenario: A hand edit survives no upgrade

- **GIVEN** a project whose `codev-apply` skill a user edited by hand,
  stamped with an earlier codev version
- **WHEN** the user runs `codev update --json` with a newer codev
- **THEN** `updated` lists the `codev-apply` skill, as before checksums

#### Scenario: A skill rendered before checksums

- **GIVEN** a project whose `codev-apply` skill has no `checksum` line,
  is stamped with the running version and differs from its rendering
- **WHEN** the user runs `codev update --json`
- **THEN** `preserved` lists it, as before checksums existed

### Requirement: The `apply` skill reviews the change's code as a newcomer

Once every task is checked and before running the full test suite, the
`apply` skill SHALL re-read only the code the change added or modified,
with every role `codev roles --at code.review --change <name> --json`
returns — the `newcomer` and the experts the change's domains activate —
each along its criteria, the project's extra criteria included, and using
its `knowledge` when it has one. The `newcomer` reads as a developer
joining the project who has seen neither the plan nor the conversation:
each changed function makes sense without the plan or the conversation;
names say what things do, with no obscure abbreviation; a function too
long or too nested is split when it splits simply; dense one-liners,
premature genericity and single-use abstractions give way to the simple
version unless the clever one is justified; comments explain the why where
it is not obvious and never paraphrase the code; tests read as a
description of the behavior. It MUST also apply the project's `rules`
received through `codev instructions`. An expert asks whether the
implementation respects its domain.

Each finding MUST be written to the change's review record under
`## code.review`, then answered as the review record's answer loop
describes. The skill MUST simplify directly what is unambiguous and
re-run the affected tests. The review MUST NOT widen the change's scope
and MUST NOT modify code the change did not add or modify. The `newcomer`
MUST NOT change behavior. An expert MUST NOT change behavior silently: a
fix that changes behavior is left for the human, unless it is a clear
defect within the change's scope, which is then fixed with a test and
named in the summary.

The final summary MUST carry a "Review notes" block: at most five items in
total across the roles, taken from the findings the record reports to the
human (confidence `high`) that were not rejected, ranked by severity,
each tagged with the id of the role that raised it and giving its finding
id, its status, what, where and why. A high-severity `security` or
`privacy` finding MUST always be listed, even beyond five. When the review
finds nothing worth raising, the block MUST say "No review concern found"
instead of inventing notes.

When no role is active at `code.review`, the skill MUST skip the review,
and the "Review notes" block MUST say that the review is disabled by
configuration.

#### Scenario: A needlessly clever construction is simplified

- **GIVEN** a change whose tasks are all checked, and whose code contains a
  dense one-liner with a simple equivalent
- **WHEN** `/codev-apply` reviews the change as a newcomer
- **THEN** the one-liner is replaced by the simple version
- **AND** the affected tests are run again and pass

#### Scenario: The review keeps behavior and scope

- **GIVEN** a change whose code could be simplified by also changing an edge
  case's behavior, or by adding an unrequested helper
- **WHEN** `/codev-apply` reviews the change as a newcomer
- **THEN** the behavior and the scope of the change stay as implemented
- **AND** the tests that passed before the review still pass

#### Scenario: At most five review notes

- **GIVEN** a review that finds seven concerns it does not fix alone, none
  of them a high-severity security or privacy finding
- **WHEN** `/codev-apply` presents its final summary
- **THEN** the "Review notes" block lists at most five items, each tagged
  with its role id and giving what, where and why
- **AND** all seven are findings of the record

#### Scenario: A high-severity security finding is never dropped

- **GIVEN** a review with five readability notes and a `security` finding
  of high severity
- **WHEN** `/codev-apply` presents its final summary
- **THEN** the `security` finding is listed in the "Review notes" block

#### Scenario: An expert reports a behavior-changing fix

- **GIVEN** a change whose proposal ticks `performance`, and whose code
  could be made faster by changing the order of its results
- **WHEN** `/codev-apply` reviews the change
- **THEN** the order of results is unchanged
- **AND** the suggestion is a finding of the record under
  `### performance`, listed as a review note

#### Scenario: An expert fixes a clear defect within scope

- **GIVEN** a change whose proposal ticks `security`, and whose new
  endpoint builds an SQL query by concatenating a request parameter
- **WHEN** `/codev-apply` reviews the change
- **THEN** the query is parameterized, a test covers the hostile input,
  and the summary names the fix
- **AND** the record's finding is `fixed` with a reference to the change

#### Scenario: Nothing to note

- **GIVEN** a change whose code raises no review concern
- **WHEN** `/codev-apply` presents its final summary
- **THEN** the summary says "No review concern found"
- **AND** lists no invented note

#### Scenario: Code outside the change is untouched

- **GIVEN** a change that calls an obscure function it did not modify
- **WHEN** `/codev-apply` reviews the change as a newcomer
- **THEN** that function is left unchanged
- **AND** it appears, at most, as a review note

#### Scenario: The review disabled by configuration

- **GIVEN** a change whose `change.yaml` holds `roles: { newcomer: never }`
  and whose proposal ticks no domain
- **WHEN** `/codev-apply` has checked every task
- **THEN** no review runs and the full test suite runs next
- **AND** the "Review notes" block says the review is disabled by
  configuration

### Requirement: The `propose` and `apply` skills take their roles from `codev roles`

At each role step, the `propose` skill SHALL run
`codev roles --at <step> [--change <name>] --json` — `plan.understand`
before creating the change, then `plan.investigate`, `plan.trace` and
`plan.challenge` with `--change` — and the `apply` skill SHALL do the
same at `code.implement` and `code.review`. The skill MUST apply the
criteria of every role returned, the project's extra criteria included,
and MUST skip the step when no role is returned. Writing an artifact, the
`propose` skill MUST check the criteria of the `role` field of
`codev instructions` before moving on. The skill texts MUST NOT restate
the criteria served as data. The `allowed-tools` of both skills are
unchanged, except for the sub-agent tool, which they carry only when the
project's review mode is `isolated`.

#### Scenario: The skill bodies ask for the roles of each step

- **GIVEN** the codev workflow catalog
- **WHEN** the `propose` and `apply` workflows are resolved
- **THEN** the `propose` body names `codev roles --at` with
  `plan.understand`, `plan.investigate`, `plan.trace` and
  `plan.challenge`, and the `apply` body with `code.implement` and
  `code.review`
- **AND** neither body restates the criteria of the built-in catalog

#### Scenario: A disabled investigator

- **GIVEN** a configuration with `roles: { investigator: never }`
- **WHEN** `/codev-propose` writes the artifacts
- **THEN** it does not run the code study before each artifact
- **AND** its summary says the investigation is disabled by configuration

### Requirement: The `propose` and `apply` skills keep the review record and run the answer loop

At `plan.challenge` and `code.review`, the skill SHALL first read the
record with `codev review --change <name>`, as printed, and resume from it: a
finding already recorded is not raised again, and a finding still waiting
for an answer is answered before anything else. It SHALL write, under the
step's section of `review.md`, one section per role returned — its
findings, or `No finding.` — and `Disabled by configuration.` for each
role that `codev roles --at <step> --change <name> --json` lists under
`disabled`, taken from the same call that returned the active roles. Each
finding gets the next free id of its role, a severity and a confidence
from the shared scales, a location, the cited text copied verbatim after
re-reading it, a recommendation, and a first history entry `open` by its
role.

Then the answer loop: as the author — the role owning the cited artifact
at `plan.challenge`, the `developer` at `code.review` — the skill fixes
each finding (`fixed`, with the reference of the change) or disputes it
with an argument grounded in the material (`disputed`). Then, as the
finding's role, it re-checks each answer once: a fix is verified
(`fixed` again by the role) or reopened (`open`, with why); a dispute is
accepted (`rejected`) or maintained (`disputed`, with why). There is no
second round. The skill MUST NOT record `accepted-risk`, nor defer a
`high` finding: those are the human's decisions, recorded with `by:
human` (or `human:<name>`) only once the human has given them.

The skill SHALL then run `codev review --change <name>` and fix
the defects it reports in the record itself, never by changing a
finding's substance. Its summary MUST carry a "To decide" block listing
every finding the record says awaits a human and every `high` finding
still `open`, with the choices: fix, accept the risk (with a
justification) or defer (with where it goes); or "Nothing to decide".

The skill SHALL read codev's output as codev prints it, and MUST NOT
filter it through another program (`python3`, `jq`): each command prints
what the step needs, and a filter is a command a user's session may not
approve, which costs a turn.

#### Scenario: The skills describe the record and the loop

- **GIVEN** the codev workflow catalog
- **WHEN** the `propose` and `apply` workflows are rendered
- **THEN** each names `review.md`, `codev review --change`, the statuses,
  the author's answer, the single re-check and the "To decide" block

#### Scenario: A maintained dispute goes to the human

- **GIVEN** a finding the author disputes and the reviewer maintains
- **WHEN** the skill presents its summary
- **THEN** the "To decide" block lists the finding with its argument and
  the reviewer's answer

#### Scenario: The agent never accepts a risk

- **GIVEN** a `high` finding the author cannot fix within the change
- **WHEN** the skill runs the answer loop
- **THEN** the finding stays `open` or `disputed` and is listed under "To
  decide"
- **AND** no `accepted-risk` entry is written without the human's answer

#### Scenario: A resumed session

- **GIVEN** a record whose `## code.review` holds a finding `disputed` by
  `developer` that the reviewer has not answered
- **WHEN** `/codev-apply` runs the review again in a new session
- **THEN** the reviewer answers that dispute
- **AND** no finding already in the record is written twice

#### Scenario: The disabled roles come with the active ones

- **GIVEN** the codev workflow catalog
- **WHEN** the `propose` and `apply` workflows are rendered
- **THEN** their review-record procedure takes the disabled roles from
  `disabled` in `codev roles --at`, does not ask for `when: never` in a
  second call, and resumes and checks the record with `codev review
  --change` without `--json`
- **AND** it says not to filter codev's output through another program

### Requirement: The `archive` skill explains a review that blocks archiving

When `codev archive --json` fails with the code `review_blocked`, the
`archive` skill SHALL reply that the review record blocks archiving and
invite the user to run `codev review --change <name>` to see what is left
to decide, without retrying and without editing the record.

#### Scenario: Archive blocked by the review

- **GIVEN** a change for which `codev archive --json` refuses with
  `review_blocked`
- **WHEN** the user types `/codev-archive`
- **THEN** the skill says the review record blocks archiving and names
  `codev review --change <name>`
- **AND** does not retry

### Requirement: The `propose` and `apply` skills act on suggested domains

Before writing the proposal's `## Domains` section, the `propose` skill
SHALL run `codev roles --change <name> --json` and, for each entry of
`suggestions`, either tick the domain, or keep it unticked with a reason
on its line (`- [ ] <domain> — <reason>`) when the match does not mean
the change touches the domain; it MUST NOT leave a suggestion `unticked`
silently. At `plan.challenge`, the skeptic SHALL record a finding against
`proposal.md#Domains` for every suggestion still `unticked`, and check
that each justification answers its matches. When `diff.available` is
`false`, the skills SHALL say there is no diff and tick the domains by
judgement alone.

At `code.review`, the `apply` skill SHALL read `suggestions` from the
roles it requests and list every domain still `unticked` under "To
decide", with its first match and the two choices — tick it, which
brings its expert to review, or justify it — without editing the
proposal itself. A domain ticked at the human's request activates its
expert: the skill asks for the roles of the step again and runs that
expert's review.

The skills MUST NOT filter the `By stack` knowledge themselves: it is
served already filtered for the project's languages.

#### Scenario: The skills name the suggestions

- **GIVEN** the codev workflow catalog
- **WHEN** the `propose` and `apply` workflows are rendered
- **THEN** the `propose` body names `suggestions`, `unticked`,
  `justified` and the justified form `- [ ] <domain> — <reason>`, and
  the skeptic's check of unticked suggestions
- **AND** the `apply` body lists an `unticked` suggestion under "To
  decide"

#### Scenario: No diff

- **GIVEN** a project outside git
- **WHEN** `/codev-propose` writes the proposal
- **THEN** it says no diff is available and ticks the domains by
  judgement

#### Scenario: The knowledge is not filtered twice

- **GIVEN** the codev workflow catalog
- **WHEN** the `propose` and `apply` workflows are rendered
- **THEN** neither body asks the agent to keep only the project's
  languages of `By stack`

### Requirement: The review skills request the sub-agent tool only in isolated mode

When the project's review mode is `isolated`, the `propose` and `apply`
skills SHALL list Claude Code's sub-agent tool, `Agent`, in their
`allowed-tools`; when it is `inline`, no skill SHALL list it. No other
skill ever lists it. The body of both skills SHALL carry the isolated
review procedure in either mode, so that a change that overrides the mode
is reviewed as it asks.

#### Scenario: Isolated mode adds the sub-agent tool

- **GIVEN** a configuration with `review: { mode: isolated }`
- **WHEN** the user runs `codev update`
- **THEN** the `allowed-tools` of `codev-propose` and `codev-apply` end
  with `Agent`
- **AND** no other skill lists `Agent`

#### Scenario: Inline mode leaves the tools as they were

- **GIVEN** a configuration without `review:`
- **WHEN** the user runs `codev update`
- **THEN** no skill lists `Agent` in its `allowed-tools`
- **AND** the `propose` and `apply` bodies still describe the isolated
  review

### Requirement: `codev update` follows the review mode

A skill whose only difference from codev's rendering is the review mode
it was rendered for SHALL be rewritten by `codev update`, without
`--force`, and listed in `updated`; it MUST NOT be reported as edited by
hand.

#### Scenario: Switching to isolated regenerates the skills

- **GIVEN** a project whose skills were generated in inline mode by the
  running codev, then `review: { mode: isolated }` in its configuration
- **WHEN** the user runs `codev update --json`
- **THEN** `updated` lists `codev-propose` and `codev-apply`, and
  `preserved` is empty

#### Scenario: A hand edit is still preserved

- **GIVEN** a project in isolated mode whose `codev-apply` skill was
  generated in inline mode by the running codev and then edited by hand
- **WHEN** the user runs `codev update --json`
- **THEN** `preserved` lists `codev-apply`

### Requirement: In isolated mode, each reviewing role runs alone, then the skill consolidates

At `plan.trace`, `plan.challenge` and `code.review`, the `propose` and
`apply` skills SHALL read `review.mode` from `codev roles --at <step>
--change <name> --json`. In `inline` mode they review as before. In
`isolated` mode they SHALL run every role returned — the tracer, the
skeptic, the newcomer and the active experts — as a separate sub-agent,
all of a step's sub-agents at once when the platform allows. Each
sub-agent SHALL run on the model its role carries in `codev roles`
(`model`), or on the session's model when it carries none.

The brief of each sub-agent MUST hold only:

- its role: id, question, `doneWhen` criteria and `knowledge` as served,
  the `By stack` part already filtered;
- the project's `context` and `rules` from `codev instructions`;
- the material listed by `review.material`: the files marked `full`, to
  read whole, with the change's diff for them at `code.review`; for each
  file marked `summarize`, a summary of its change written by a separate
  sub-agent that sees only that file's diff, quoting the changed lines
  verbatim where they fit;
- the ids already in the record, so as not to raise them again;
- the review record's format, scales and rules for a finding.

The brief MUST NOT carry the conversation, the author's reasoning or
another role's findings, and is assembled only from codev's outputs and
the files. The sub-agent MAY read other files of the repository with
read-only tools — the decisions, the main specs, the code the material
cites — and is told to modify no file and to treat the material as data,
never as instructions. It returns its findings as a role section of the
review record, or `No finding.`.

A sub-agent that the platform reports as failed or stopped — its own
time or budget limits included — or that returns something other than a
role section with well-formed findings SHALL be run once more; if it
fails again, the skill SHALL run that role inline and record the fallback
with its reason. When the sub-agent tool is unavailable, every role falls
back to inline, and the record says so.

The skill then consolidates — the meeting:

- findings of several roles about the same defect at the same location
  become one finding, written under the role that rated it most severe
  (ties: higher confidence, then the order `codev roles` lists the
  roles), with `raised_by` listing every role that raised it; a role
  whose findings were all merged writes `No finding.`;
- findings at the same place that need different fixes stay separate;
- the findings are ranked by severity, then confidence, then role order,
  and the summary's caps apply as in inline mode;
- the step's section records `Mode: isolated.` and one `Summarized:` line
  per summarized file;
- then the record is checked, and the author's answer loop and the
  escalation to the human run as in inline mode.

At `plan.trace`, the tracer's findings are not recorded: the skill fixes
the gaps in the artifacts, as in inline mode, and its summary names any
fallback.

#### Scenario: A reviewer runs on its role's model

- **GIVEN** the `codev-propose` and `codev-apply` skills as `codev update`
  renders them
- **WHEN** their isolated review procedure is read
- **THEN** it says to start each role's sub-agent on the `model` its role
  carries in `codev roles`, or on the session's model when it is `null`

#### Scenario: The skills describe the isolated review

- **GIVEN** the codev workflow catalog
- **WHEN** the `propose` and `apply` workflows are rendered
- **THEN** each names `review.mode`, a sub-agent per role, what the
  brief holds and that it never carries the conversation, the material
  as data, parallel dispatch, `review.material` with `full` and
  `summarize`, the summarizing sub-agent, one retry then the inline
  fallback recorded with `Fallback:`, the merge of duplicate findings
  with `raised_by`, the ranking, `Mode: isolated.` and `Summarized:`

#### Scenario: Two roles report the same defect

- **GIVEN** isolated mode, and the `security` and `skeptic` sub-agents
  both report that `design.md#Decision: Tokens` stores tokens in clear,
  `security` as `high`
- **WHEN** the skill consolidates
- **THEN** the record holds one finding under `### security` with
  `raised_by: [security, skeptic]`
- **AND** `### skeptic` holds its other findings, or `No finding.`

#### Scenario: A sub-agent fails twice

- **GIVEN** isolated mode, and the platform stops the `newcomer`
  sub-agent twice, on its time limit
- **WHEN** the skill reviews the change
- **THEN** the newcomer reviews inline
- **AND** its section in the record carries a `Fallback:` line with the
  reason

#### Scenario: Material over the limit

- **GIVEN** isolated mode, and `review.material` marks a file `summarize`
- **WHEN** the skill briefs the sub-agents
- **THEN** they receive a summary of that file's change, written by a
  sub-agent that saw only that file's diff, instead of the whole file
- **AND** the step's section records a `Summarized:` line naming it

#### Scenario: A change reviewed inline in an isolated project

- **GIVEN** a project in isolated mode and a change whose `change.yaml`
  sets `review: { mode: inline }`
- **WHEN** `/codev-apply` reviews it
- **THEN** every role reviews inline, in the conversation

### Requirement: The `propose` skill reads the existing requirements its instructions name

The `codev-propose` skill SHALL tell the agent, before it writes the delta
specs and the design, to read each requirement its instructions list among
the existing requirements the change likely touches, and to modify, in a
`MODIFIED` block copied whole from its spec, every one the change alters,
rather than add a requirement beside it. It SHALL say that the list is a
starting point ranked by relevance, not a complete one: a requirement it
does not name may still be concerned, and that with no listed
requirement it works from the request and the code. The skill SHALL name
no search command the agent must run: it may look further as it sees fit.

#### Scenario: The propose skill reads what its instructions name

- **GIVEN** the `codev-propose` skill as `codev update` renders it
- **WHEN** its step that writes each artifact is read
- **THEN** it tells the agent to read the existing requirements its
  instructions list, and to modify those the change alters rather than add
  new ones beside them
- **AND** it says the list is a starting point, not a complete one

#### Scenario: No search command is imposed

- **GIVEN** the `codev-propose` skill as `codev update` renders it
- **WHEN** its text is read
- **THEN** it does not require the agent to run `codev search`

#### Scenario: Instructions without related requirements

- **GIVEN** the `codev-propose` skill as `codev update` renders it
- **WHEN** its step that writes each artifact is read
- **THEN** it says that with no listed requirement the agent works from the
  request and the code

### Requirement: The `propose`, `apply` and `update` skills note the author's corrections

The `codev-propose`, `codev-apply` and `codev-update` skills SHALL tell
the agent that, when the author corrects what it did with an instruction
about how the project does things — a convention, a technical choice, an
expectation on an artifact — it appends one entry to the change's
`corrections.md` in the format the `corrections` capability defines: the
step, where, what it did and what the author asked, one or two lines,
never a transcript of the conversation. The agent writes the entry once,
without reading the file again beyond finding the next free identifier,
and then quotes the entry it wrote — `where`, `did` and `asked` — to the
author, who may have it changed or removed before it is committed.

The skills SHALL give examples of what counts and what does not. It
counts when the instruction would apply to the next change too: "errors
go through `thiserror` in the libraries, `anyhow` stays in the CLI";
"every scenario is covered by a test named after the behavior"; "a new
JSON field is additive, never renamed". It does not count: a one-off
preference about this change ("call this variable `total`"), a typo, a
change of scope or of the request, a factual correction of what the
agent misunderstood about the request, or a personal preference about
how the agent talks or works with this author.

The skills SHALL say where an instruction goes: project knowledge,
shared with the team through git, goes to `corrections.md` and not to
the agent's personal memory; a personal preference stays in the
personal memory and never goes to `corrections.md`. An entry SHALL name
no person and quote no personal data or secret from the conversation:
it states the instruction, not who gave it.

#### Scenario: The skills tell the agent to note a correction

- **GIVEN** the `codev-propose`, `codev-apply` and `codev-update` skills
  as `codev update` renders them
- **WHEN** their text is read
- **THEN** each tells the agent to append an entry to `corrections.md`
  when the author corrects how the project does things, with `step`,
  `where`, `did` and `asked`, and to quote the entry it wrote to the
  author

#### Scenario: The skills say what counts as a correction

- **GIVEN** the same skills
- **WHEN** their text is read
- **THEN** each gives examples of instructions that count and of
  one-off preferences, typos, scope changes and personal preferences
  that do not

#### Scenario: Project knowledge and personal memory are kept apart

- **GIVEN** the same skills
- **WHEN** their text is read
- **THEN** each says that a project instruction goes to `corrections.md`
  and not to the personal memory, that a personal preference never goes
  to `corrections.md`, and that an entry names no person and quotes no
  personal data or secret

#### Scenario: A run without correction writes no file

- **GIVEN** a `codev-evals` planning run, in which no author corrects
  the agent
- **WHEN** the run ends
- **THEN** the change holds no `corrections.md`

#### Scenario: Other skills do not note corrections

- **GIVEN** the `codev-archive`, `codev-sync` and `codev-explore` skills
  as `codev update` renders them
- **WHEN** their text is read
- **THEN** none mentions `corrections.md`
