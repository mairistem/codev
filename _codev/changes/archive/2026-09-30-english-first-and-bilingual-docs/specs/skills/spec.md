## ADDED Requirements

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

## MODIFIED Requirements

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
