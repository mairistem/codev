## MODIFIED Requirements

### Requirement: The `configure` skill enriches `_codev/config.yaml` by analyzing the project

The codev catalog SHALL expose a `configure` workflow — installed
under `.claude/skills/codev-configure/SKILL.md`, invocable as
`/codev-configure` — whose role is to enrich `_codev/config.yaml`
by analyzing the project, without modifying the workflows, the
detected MCPs, or the schema. The same skill holds a second mode,
`/codev-configure lessons`, described by its own requirements; what
follows is the default mode, run when the skill is invoked without
`lessons`.

The procedure MUST be:

1. **Read the existing file** — the project's `_codev/config.yaml`.
2. **Read-only exploration** — root `README.md`,
   `CONTRIBUTING.md` if it exists, the contents of `docs/`, a sample
   of the most frequently edited source files (via `git log --pretty=format: --name-only | sort | uniq -c | sort -rn | head`),
   and the structure of the main directories.
3. **Patch proposal** — a new value for `context:` (2 to 5
   lines focused on what matters for steering the skills — stack
   beyond the language, API conventions, error style, comment tone,
   structural library choices) and for `rules:` per artifact
   (`specs:`, `design:`, `tasks:` — 1 to 2 rules each, grounded in
   what the project actually does).
4. **Display the diff** as output, without writing.
5. **Write only on confirmation** from the user.

The skill MUST **preserve** the fields it does not touch — `schema`,
`workflows`, `mcp`, `inherits`, `codev_version`, as well as existing
provenance comments. It does not rewrite the `mcp:` key detected by
`codev init`.

The skill MUST **refuse to act** if `_codev/config.yaml` is missing —
it points the user to `codev init`.

#### Scenario: Role documented in the catalog

- **GIVEN** the codev workflow catalog
- **WHEN** the `configure` workflow is resolved
- **THEN** its entry exists (`find("configure").is_some()`)
- **AND** its `body` cites the five steps (read, exploration,
  proposal, diff, confirmation)
- **AND** its `body` explicitly lists the preserved fields
  (`schema`, `workflows`, `mcp`, `inherits`, `codev_version`)

#### Scenario: Confirmation required before writing

- **GIVEN** a project whose `_codev/config.yaml` has a thin context
- **WHEN** the user runs `/codev-configure`
- **THEN** a diff of the change is displayed
- **AND** the file is written only after explicit confirmation from
  the user

#### Scenario: Refusal when the config is missing

- **GIVEN** a project without `_codev/config.yaml`
- **WHEN** the user runs `/codev-configure`
- **THEN** the skill refuses to act
- **AND** points to `codev init`

#### Scenario: The default mode does not read the history

- **GIVEN** the codev workflow catalog
- **WHEN** the `configure` workflow is resolved
- **THEN** its `body` runs `codev review --archived` only in the
  section of the `lessons` mode

## ADDED Requirements

### Requirement: The `configure` skill has a lessons mode that reads the archived findings and corrections

Invoked as `/codev-configure lessons [<n>]`, the `configure` skill SHALL
read `codev review --archived --last <n>` (`n` 10 by default), the
decisions with `codev decision list` (in effect, proposed or
superseded), `_codev/config.yaml` with its comments, the active changes
with `codev list`, and `_codev/lessons.md` when it exists. It SHALL
refuse to act, as the default mode does, when `_codev/config.yaml` is
missing. When `codev review --archived` fails with a usage error, the
running codev predates the mode: the skill SHALL say in one line that the
lessons mode needs a newer codev and point to `codev self-update`, then
stop, without reading the archive by other means. The mode SHALL run the
update check the other skills run at the start of an action.

It SHALL group the served entries into lessons by judgement, under these
thresholds: a lesson from findings needs findings of at least two
different changes; a single correction is enough for a lesson, since its
author already judged it, but it is confirmed like any other. An entry
already marked `considered`, and a lesson the ledger records as
`declined` or `set-aside`, SHALL NOT be proposed again.

#### Scenario: A finding seen in one change only

- **GIVEN** an archive whose only finding about symbolic links is in one
  change
- **WHEN** the user runs `/codev-configure lessons`
- **THEN** no lesson about symbolic links is proposed

#### Scenario: A recurring finding

- **GIVEN** two archived changes each holding a `fixed` finding about a
  symbolic link that lets a file outside the project be read
- **WHEN** the user runs `/codev-configure lessons`
- **THEN** one lesson is proposed, naming both findings as its origin

#### Scenario: A declined lesson stays declined

- **GIVEN** a ledger lesson `declined` whose origin names two findings,
  and a third archived finding of the same kind
- **WHEN** the user runs `/codev-configure lessons`
- **THEN** the lesson is not proposed again

#### Scenario: An older binary

- **GIVEN** the codev workflow catalog
- **WHEN** the `configure` workflow is resolved
- **THEN** its lessons section says that a usage error from
  `codev review --archived` means a newer codev is needed, points to
  `codev self-update`, and stops without reading
  `_codev/changes/archive/` by other means

#### Scenario: The skill carries the thresholds

- **GIVEN** the codev workflow catalog
- **WHEN** the `configure` workflow is resolved
- **THEN** its `body` says a finding becomes a lesson when it recurs in
  at least two changes, and that one correction is enough but is
  confirmed

### Requirement: The lessons mode proposes each lesson for one destination, in a fixed order

For each lesson, the lessons mode SHALL propose a single destination, the
first that fits in this order:

1. `check` — an automatable check (a lint, a test, a CI job): the mode
   writes no code; it names the change to propose, or the active change
   whose tasks should hold it;
2. `decision` — an architecture decision, created with
   `codev decision new` as `proposed`, never an edit of an accepted
   decision; when a sealed decision proved too vague, a superseding one
   with `codev decision supersede`;
3. `rule:specs`, `rule:design` or `rule:tasks` — a rule of `rules:`, a
   lesson about code being written as a design or tasks rule; or
   `context` — a line of `context:`;
4. `none` — general expertise that belongs to codev's built-in roles,
   recorded `set-aside`, nothing written in the project.

A lesson already held by a decision in effect or proposed, a rule, a
`context:` line or an active change SHALL be shown as `covered`, naming
what holds it, and nothing is written for it but its ledger entry; a
rule SHALL never restate a decision in effect. When an artifact already
has ten rules, the mode SHALL propose merging rules rather than adding
one.

The mode SHALL show one line per lesson,
`L-<n> [<destination>] <lesson> — from <origin>, <origin>`, or
`L-<n> [covered by <what>] …` for a covered lesson, the origins as
`<archived change folder>/<id>`, followed by the diff of
every file it would write. The author may accept, decline, or move a
lesson to another destination.

#### Scenario: A lesson a lint can enforce

- **GIVEN** findings of three archived changes about functions too long
  or taking too many parameters
- **WHEN** the user runs `/codev-configure lessons`
- **THEN** the lesson is proposed with the destination `check`, naming
  the lint, and no rule is proposed for it

#### Scenario: A lesson a decision already holds

- **GIVEN** recurring findings about what an older codev does with a new
  key, and a proposed decision on compatibility with older codevs
- **WHEN** the user runs `/codev-configure lessons`
- **THEN** the lesson is shown as `covered` by that decision, and no
  rule or decision is proposed for it

#### Scenario: Too many rules on an artifact

- **GIVEN** a configuration whose `rules.tasks` holds ten rules, and a
  lesson for `rule:tasks`
- **WHEN** the user runs `/codev-configure lessons`
- **THEN** the mode proposes a merge of existing `tasks` rules that
  makes room for it, rather than an eleventh rule

#### Scenario: The author moves a lesson

- **GIVEN** a lesson proposed as `rule:design`
- **WHEN** the author asks for a decision instead
- **THEN** the diff shown again creates a proposed decision and adds no
  rule

### Requirement: The lessons mode writes only what the author agrees to, with its origin

The lessons mode SHALL write nothing before the author's explicit
agreement to the diff shown, and SHALL write only what was agreed. A rule
or a `context:` line it writes SHALL carry, on the line before it, a YAML
comment naming its lesson and its origins. A decision it creates SHALL
name the findings and corrections it comes from in its `## Context`, and
stay `proposed` for the author to accept. With the same agreement, the
mode SHALL append to `_codev/lessons.md` — creating it with its format
marker when it does not exist — one `P-<n>` entry for the pass, listing
in `read` every archived change folder it read, and one
`L-<n>` entry per lesson shown, with its outcome: `written`,
`handed-off`, `covered`, `declined` or `set-aside`. It SHALL preserve
every other field and comment of `_codev/config.yaml`, never modify an
accepted decision or a change's files, and run `codev validate` after
writing.

#### Scenario: Nothing written without agreement

- **GIVEN** a lessons pass that proposes two lessons
- **WHEN** the author answers no
- **THEN** no file of the project is modified

#### Scenario: A rule carries its origin

- **GIVEN** a lesson from two corrections, accepted for `rule:tasks`
- **WHEN** the mode writes it
- **THEN** `rules.tasks` holds the rule, preceded by a comment naming
  the lesson and both corrections
- **AND** `_codev/lessons.md` holds an `L-<n>` entry with
  `outcome: written`, `destination: rule:tasks` and both origins

#### Scenario: A declined lesson is remembered

- **GIVEN** a lesson the author declines with a reason
- **WHEN** the mode writes the ledger
- **THEN** the entry has `outcome: declined` and that reason, and nothing
  else is written for it

#### Scenario: A decision stays proposed

- **GIVEN** a lesson accepted for `decision`
- **WHEN** the mode writes it
- **THEN** a new decision with `status: proposed` names the lesson's
  origins in its context, and `_codev/decisions/seal.yaml` is unchanged
