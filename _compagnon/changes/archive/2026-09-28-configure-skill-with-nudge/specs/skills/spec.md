## MODIFIED Requirements

### Requirement: Skill `onboard` presents compagnon and recommends the next action

The compagnon catalog SHALL expose an `onboard` workflow — installed
under `.claude/skills/compagnon-onboard/SKILL.md`, invocable as
`/compagnon-onboard` — whose role is to present compagnon to a user
discovering it, in three blocks:

1. A short description of compagnon (two or three sentences).
2. The current state of the project — repository initialized or not, number of main
   specs, number of indexed local decisions, active changes
   listed by name, and **number of archived changes** (shown
   only if non-zero, so as not to clutter the output on a
   new project).
3. The recommended next action, adapted to the state:
   - `_compagnon/` absent → `compagnon init`.
   - Project initialized, no change, **and `_compagnon/config.yaml`
     thin** (context < 200 characters and empty `rules:`) →
     `/compagnon-configure` first, with a sentence explaining the
     benefit ("Claude will enrich the config from the project"),
     then `/compagnon-propose <idea>` second.
   - Project initialized, no change, non-thin config → **invite the
     user to read `README.md` to get a feel for the project**, then
     `/compagnon-propose <idea>`; `/compagnon-explore <topic>` remains
     mentioned as an alternative.
   - One active change whose planning is incomplete →
     `/compagnon-propose <this-change>` to continue it.
   - One active change whose planning is complete →
     `/compagnon-apply <this-change>`.
   - Several active changes → list them and let the user
     choose.

The skill MUST be **strictly read-only**: `allowed-tools` limited
to `Bash(compagnon:*), Read, Glob`. No `Write`, no `Edit`, no general
`Bash`.

#### Scenario: Role documented in the catalog

- **GIVEN** the compagnon workflow catalog
- **WHEN** the `onboard` workflow is resolved
- **THEN** its entry exists (`find("onboard").is_some()`)
- **AND** its `allowed_tools` is exactly
  `"Bash(compagnon:*), Read, Glob"`
- **AND** its `allowed_tools` does NOT contain general `Bash` (invariant
  rule: only `apply` has it)
- **AND** its `body` cites the three blocks (description, state,
  recommended action)

#### Scenario: Skill installed by a `compagnon update`

- **GIVEN** a project whose `config.yaml` has `workflows: [propose,
  explore, apply, sync, archive, update, onboard]`
- **WHEN** the user runs `compagnon update`
- **THEN** the file `.claude/skills/compagnon-onboard/SKILL.md` is
  created
- **AND** its YAML frontmatter is valid and carries the expected
  description

#### Scenario: "Here you have" block mentions archived changes when there are some

- **GIVEN** a project containing at least one change in
  `_compagnon/changes/archive/`
- **WHEN** the user runs `/compagnon-onboard`
- **THEN** the "here you have" block contains a line indicating the
  number of archived changes
- **AND** this number matches the number of folders of the form
  `<date>-<name>/` under `_compagnon/changes/archive/`

#### Scenario: "Here you have" block adds no archived line on a new project

- **GIVEN** a freshly initialized project, with no archived
  change
- **WHEN** the user runs `/compagnon-onboard`
- **THEN** the "here you have" block **does not display** an
  "archived changes" line — the output stays short and uncluttered

#### Scenario: Configure recommendation when the config is thin

- **GIVEN** an initialized project with no active change, whose
  `_compagnon/config.yaml` has a `context:` shorter than 200 characters and
  empty `rules:`
- **WHEN** the user runs `/compagnon-onboard`
- **THEN** the "what's next" block cites `/compagnon-configure` first,
  with a sentence about the expected benefit
- **AND** mentions `/compagnon-propose <idea>` second

#### Scenario: Default recommendation cites README.md when the config is not thin

- **GIVEN** an initialized project with no active change whose
  `_compagnon/config.yaml` has a `context:` ≥ 200 characters OR non-empty
  `rules:`
- **WHEN** the user runs `/compagnon-onboard`
- **THEN** the "what's next" block invites the user to read `README.md` before
  creating a change
- **AND** cites `/compagnon-propose <an-idea>` as the actionable step and mentions
  `/compagnon-explore <topic>` as an alternative

### Requirement: `onboard` is part of the default catalog

The `DEFAULT_WORKFLOWS` array of `compagnon-agents::workflows` MUST
contain the **complete list of the 8 workflows** of compagnon: `propose`,
`explore`, `onboard`, `apply`, `sync`, `archive`, `update` and
`configure`. A user who runs `compagnon init --yes` (or from a
non-interactive pipe) on a new project, without a `workflows:` key in
their `config.yaml`, therefore gets all available skills
immediately — including `configure`, the recommended entry point
after init.

A project that wants to restrict the catalog MUST declare an explicit
`workflows:` key with a chosen subset — this is the opt-out path,
rather than the former opt-in path.

This switch solves a discovery problem: under the former default
(3 workflows), a user who typed `/compagnon-apply` after
`/compagnon-propose` did not find the skill and thought it
did not exist.

#### Scenario: Default catalog includes the 8 workflows

- **GIVEN** a project whose `config.yaml` has no
  `workflows:` key
- **WHEN** `select(None)` is called on the catalog
- **THEN** the list of returned `id`s is exactly
  `["propose", "explore", "onboard", "apply", "sync", "archive", "update", "configure"]`
- **AND** no warning is emitted

#### Scenario: Opt-out restriction via explicit workflows

- **GIVEN** a project whose `config.yaml` contains
  `workflows: [propose, explore, onboard]`
- **WHEN** `select` is called with this list
- **THEN** only these three skills are returned
- **AND** `apply`, `sync`, `archive`, `update`, `configure` are
  **not** installed
