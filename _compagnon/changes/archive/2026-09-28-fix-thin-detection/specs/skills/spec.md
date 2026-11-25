## MODIFIED Requirements

### Requirement: Skill `onboard` presents codev and recommends the next action

The codev catalog SHALL expose an `onboard` workflow — installed under
`.claude/skills/codev-onboard/SKILL.md`, invocable as `/codev-onboard`
— whose role is to present codev to a user discovering it, in three
blocks:

1. A short description of codev (two or three sentences).
2. The current state of the project — repository initialized or not,
   number of main specs, number of indexed local decisions, active
   changes listed by name, and **number of archived changes**
   (displayed only if non-zero, so as not to clutter the output on a
   new project).
3. The recommended next action, adapted to the state:
   - `_codev/` missing → `codev init`.
   - Project initialized, no change, **and `_codev/config.yaml`
     thin** (no entry in `rules:`) → `/codev-configure` first, with a
     sentence explaining the benefit ("Claude will enrich the config
     based on the project"), then `/codev-propose <idea>` second.
   - Project initialized, no change, non-thin config → **invite the
     user to read `README.md` to get a feel for the project**, then
     `/codev-propose <idea>`; `/codev-explore <topic>` remains
     mentioned as an alternative.
   - One active change whose planning is incomplete →
     `/codev-propose <that-change>` to continue it.
   - One active change whose planning is complete →
     `/codev-apply <that-change>`.
   - Several active changes → list them and let the user choose.

The skill MUST be **strictly read-only**: `allowed-tools` limited to
`Bash(codev:*), Read, Glob`. No `Write`, no `Edit`, no general `Bash`.

#### Scenario: Role documented in the catalog

- **GIVEN** the codev workflow catalog
- **WHEN** the `onboard` workflow is resolved
- **THEN** its entry exists (`find("onboard").is_some()`)
- **AND** its `allowed_tools` is exactly
  `"Bash(codev:*), Read, Glob"`
- **AND** its `allowed_tools` does NOT contain general `Bash`
  (invariant rule: only `apply` has it)
- **AND** its `body` mentions the three blocks (description, state,
  recommended action)

#### Scenario: Skill installed by a `codev update`

- **GIVEN** a project whose `config.yaml` has `workflows: [propose,
  explore, apply, sync, archive, update, onboard]`
- **WHEN** the user runs `codev update`
- **THEN** the file `.claude/skills/codev-onboard/SKILL.md` is
  created
- **AND** its YAML frontmatter is valid and carries the expected
  description

#### Scenario: "Here's what you have" block mentions archived changes when there are some

- **GIVEN** a project containing at least one change in
  `_codev/changes/archive/`
- **WHEN** the user runs `/codev-onboard`
- **THEN** the "here you have" block contains a line stating
  the number of archived changes
- **AND** that number matches the number of folders of the form
  `<date>-<name>/` under `_codev/changes/archive/`

#### Scenario: "Here's what you have" block adds no archived line on a new project

- **GIVEN** a freshly initialized project, without any archived
  change
- **WHEN** the user runs `/codev-onboard`
- **THEN** the "here you have" block **does not display** an
  "archived changes" line — the output stays short and uncluttered

#### Scenario: Configure recommendation when the config has no rules

- **GIVEN** an initialized project with no active change, whose
  `_codev/config.yaml` has no entry in `rules:`
- **WHEN** the user runs `/codev-onboard`
- **THEN** the "what's next" block mentions `/codev-configure` first,
  with a sentence on the expected benefit
- **AND** mentions `/codev-propose <idea>` second

#### Scenario: Default recommendation mentions README.md when the config has rules

- **GIVEN** an initialized project with no active change whose
  `_codev/config.yaml` carries at least one entry in `rules:`
- **WHEN** the user runs `/codev-onboard`
- **THEN** the "what's next" block invites the user to read
  `README.md` before creating a change
- **AND** mentions as actionable `/codev-propose <an-idea>` and
  mentions `/codev-explore <topic>` as an alternative
