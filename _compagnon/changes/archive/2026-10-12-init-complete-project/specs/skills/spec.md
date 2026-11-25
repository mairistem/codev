## ADDED Requirements

### Requirement: The `onboard` skill relays what `compagnon doctor` reports missing

When the repository is initialized, the `onboard` skill SHALL run
`compagnon doctor --json` and, in its "here you have" block, list the checks
whose `status` is `missing` — at most five, one line each, with their
`fix` verbatim — or say that the setup is complete when none is missing.
Warnings MAY be summarized as a count. The recommended next action of the
"what's next" block is unchanged.

The skill MUST stay read-only: it never runs a fix itself, `compagnon init`
and `compagnon update` included, and its `allowed-tools` stay
`Bash(compagnon:*), Read, Glob`. If `compagnon doctor` fails or is not available,
the skill continues without the list.

#### Scenario: Missing checks are listed with their fix

- **GIVEN** an initialized project whose `compagnon doctor --json` reports
  `config.rules` and `specs` as `missing`
- **WHEN** the user runs `/compagnon-onboard`
- **THEN** the "here you have" block lists both checks with their fix
  (`/compagnon-configure`, `/compagnon-propose`)
- **AND** no file of the project is modified

#### Scenario: A complete setup is said in one line

- **GIVEN** an initialized project where `compagnon doctor --json` reports no
  `missing` check
- **WHEN** the user runs `/compagnon-onboard`
- **THEN** the "here you have" block says the compagnon setup is complete

#### Scenario: Doctor unavailable

- **GIVEN** an initialized project where `compagnon doctor --json` fails
- **WHEN** the user runs `/compagnon-onboard`
- **THEN** the skill still shows its three blocks, without the list of
  missing checks

#### Scenario: The skill text carries the doctor step

- **GIVEN** the compagnon workflow catalog
- **WHEN** the `onboard` workflow is resolved
- **THEN** its body names `compagnon doctor --json`
- **AND** its `allowed_tools` is still exactly `"Bash(compagnon:*), Read, Glob"`
