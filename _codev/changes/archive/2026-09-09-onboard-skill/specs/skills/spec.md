## ADDED Requirements

### Requirement: Skill `onboard` presents codev and recommends the next action

The codev catalog SHALL expose an `onboard` workflow — installed under
`.claude/skills/codev-onboard/SKILL.md`, invocable as
`/codev-onboard` — whose role is to introduce codev to a user
discovering it, in three blocks:

1. A short description of codev (two or three sentences).
2. The current state of the project — repository initialized or not,
   number of main specs, number of indexed local decisions, active
   changes listed by name.
3. The recommended next action, adapted to the state:
   - `_codev/` absent → `codev init`.
   - Project initialized, no change → `/codev-propose <idea>`.
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

### Requirement: `onboard` is part of the default catalog

The `DEFAULT_WORKFLOWS` array of `codev-agents::workflows` MUST
contain `onboard`, alongside `propose` and `explore`. A user who runs
`codev init` on a new project, with no `workflows:` key in their
`config.yaml`, therefore gets `/codev-onboard` available immediately.

The other opt-in workflows (`apply`, `sync`, `archive`, `update`)
stay out of the default catalog — including them requires an explicit
declaration.

#### Scenario: Default catalog includes onboard

- **GIVEN** a project whose `config.yaml` has no `workflows:` key
- **WHEN** `select(None)` is called on the catalog
- **THEN** the list of returned `id`s is exactly
  `["propose", "explore", "onboard"]`
- **AND** no warning is emitted

#### Scenario: Other opt-ins stay opt-in

- **GIVEN** the same context
- **WHEN** the default catalog is inspected
- **THEN** none of `["apply", "sync", "archive", "update"]` appears —
  including them still requires an explicit declaration
