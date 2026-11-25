## MODIFIED Requirements

### Requirement: Skill `onboard` presents compagnon and recommends the next action

The compagnon catalog SHALL expose an `onboard` workflow — installed under
`.claude/skills/compagnon-onboard/SKILL.md`, invocable as
`/compagnon-onboard` — whose role is to introduce compagnon to a user
discovering it, in three blocks:

1. A short description of compagnon (two or three sentences).
2. The current state of the project — repository initialized or not,
   number of main specs, number of indexed local decisions, active
   changes listed by name, and **number of archived changes**
   (displayed only if non-zero, so as not to clutter the output on a
   new project).
3. The recommended next action, adapted to the state:
   - `_compagnon/` missing → `compagnon init`.
   - Project initialized, no change → **invite reading `README.md` to
     get a feel for the project**, then `/compagnon-propose <idea>`;
     `/compagnon-explore <topic>` is still mentioned as an alternative if
     the user has a question but no idea for an action yet.
   - One active change whose planning is incomplete →
     `/compagnon-propose <that-change>` to continue it.
   - One active change whose planning is complete →
     `/compagnon-apply <that-change>`.
   - Several active changes → list them and let the user choose.

The skill MUST be **strictly read-only**: `allowed-tools` limited to
`Bash(compagnon:*), Read, Glob`. No `Write`, no `Edit`, no general
`Bash`.

#### Scenario: Role documented in the catalog

- **GIVEN** the compagnon workflow catalog
- **WHEN** the `onboard` workflow is resolved
- **THEN** its entry exists (`find("onboard").is_some()`)
- **AND** its `allowed_tools` is exactly
  `"Bash(compagnon:*), Read, Glob"`
- **AND** its `allowed_tools` does NOT contain general `Bash`
  (invariant rule: only `apply` has it)
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
- **AND** that number matches the number of folders of the form
  `<date>-<name>/` under `_compagnon/changes/archive/`

#### Scenario: "Here you have" block adds no archived line on a new project

- **GIVEN** a freshly initialized project, without any archived
  change
- **WHEN** the user runs `/compagnon-onboard`
- **THEN** the "here you have" block **does not display** an
  "archived changes" line — the output stays short and uncluttered

#### Scenario: Default recommendation cites README.md

- **GIVEN** an initialized project without any active change
- **WHEN** the user runs `/compagnon-onboard`
- **THEN** the "what's next" block invites reading `README.md` before
  creating a change
- **AND** cites `/compagnon-propose <an-idea>` as the actionable step and
  mentions `/compagnon-explore <topic>` as an alternative
