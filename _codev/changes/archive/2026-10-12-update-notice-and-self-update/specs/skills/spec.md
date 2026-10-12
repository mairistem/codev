## ADDED Requirements

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
