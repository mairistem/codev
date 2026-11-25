## MODIFIED Requirements

### Requirement: Skill `onboard` presents codev and recommends the next action

The codev catalog SHALL expose an `onboard` workflow — installed under
`.claude/skills/codev-onboard/SKILL.md`, invocable as
`/codev-onboard` — whose role is to introduce codev to a user
discovering it, in three blocks:

1. A short description of codev (two or three sentences).
2. The current state of the project — repository initialized or not,
   number of main specs, number of indexed local decisions, active
   changes listed by name, and **number of archived changes**
   (displayed only if non-zero, so as not to clutter the output on a
   new project).
3. The recommended next action, adapted to the state:
   - `_codev/` missing → `codev init`.
   - Project initialized, no change → **invite reading `README.md` to
     get a feel for the project**, then `/codev-propose <idea>`;
     `/codev-explore <topic>` is still mentioned as an alternative if
     the user has a question but no idea for an action yet.
   - One active change whose planning is incomplete →
     `/codev-propose <that-change>` to continue it.
   - One active change whose planning is complete →
     `/codev-apply <that-change>`.
   - Several active changes → list them and let the user choose.

The skill MUST be **strictly read-only**: `allowed-tools` limited to
`Bash(codev:*), Read, Glob`. No `Write`, no `Edit`, no general
`Bash`.

#### Scenario: Role documented in the catalog

- **GIVEN** the codev workflow catalog
- **WHEN** the `onboard` workflow is resolved
- **THEN** its entry exists (`find("onboard").is_some()`)
- **AND** its `allowed_tools` is exactly
  `"Bash(codev:*), Read, Glob"`
- **AND** its `allowed_tools` does NOT contain general `Bash`
  (invariant rule: only `apply` has it)
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

#### Scenario: "Here you have" block mentions archived changes when there are some

- **GIVEN** a project containing at least one change in
  `_codev/changes/archive/`
- **WHEN** the user runs `/codev-onboard`
- **THEN** the "here you have" block contains a line indicating the
  number of archived changes
- **AND** that number matches the number of folders of the form
  `<date>-<name>/` under `_codev/changes/archive/`

#### Scenario: "Here you have" block adds no archived line on a new project

- **GIVEN** a freshly initialized project, without any archived
  change
- **WHEN** the user runs `/codev-onboard`
- **THEN** the "here you have" block **does not display** an
  "archived changes" line — the output stays short and uncluttered

#### Scenario: Default recommendation cites README.md

- **GIVEN** an initialized project without any active change
- **WHEN** the user runs `/codev-onboard`
- **THEN** the "what's next" block invites reading `README.md` before
  creating a change
- **AND** cites `/codev-propose <an-idea>` as the actionable step and
  mentions `/codev-explore <topic>` as an alternative
