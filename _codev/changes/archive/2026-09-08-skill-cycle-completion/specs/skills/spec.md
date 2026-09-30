## Purpose

Describes the contract of the skills that codev installs in Claude Code:
their name, what they must do, what they are not allowed to do, and how
their frontmatter guarantees these promises. Entries are added as the
changes that introduce each workflow land — one ADDED per workflow.

## ADDED Requirements

### Requirement: Skill `sync` merges a change's delta without moving it

The codev catalog SHALL expose a `sync` workflow — installed under
`.claude/skills/codev-sync/SKILL.md`, invocable as `/codev-sync` — whose
role is to bring a change's deltas into the main specs, leaving the active
change in place.

#### Scenario: Sync of a single active change

- **GIVEN** a project with a single active change whose planning is
  complete and which carries an ADDED delta on a new capability
- **WHEN** the user types `/codev-sync`
- **THEN** the skill implicitly resolves the active change
- **AND** runs `codev sync <name>`
- **AND** summarizes for the user the main specs created or updated

#### Scenario: Silent second sync

- **GIVEN** an already synced change, none of whose main specs has changed
  since
- **WHEN** the user types `/codev-sync` a second time
- **THEN** the skill reports that there is nothing to do
- **AND** does not write again

#### Scenario: Sync never moves

- **GIVEN** a change whose merge succeeds
- **WHEN** the user types `/codev-sync`
- **THEN** the `_codev/changes/<name>/` folder still exists at its
  original location

#### Scenario: Sync invites to archive after a change

- **GIVEN** a change whose merge modified at least one main spec
  (created or updated)
- **WHEN** the user types `/codev-sync`
- **THEN** the final rendering contains a line inviting to `/codev-archive`
  to close the cycle, worded without an order

#### Scenario: Sync without a change does not invite

- **GIVEN** a change whose merge is a no-op (all main specs are already
  up to date)
- **WHEN** the user types `/codev-sync`
- **THEN** the final rendering reports the absence of change
- **AND** does NOT suggest archiving — there is nothing new to propagate

### Requirement: Skill `archive` closes a change with a strict pre-flight

The catalog SHALL expose an `archive` workflow — installed under
`.claude/skills/codev-archive/SKILL.md`, invocable as `/codev-archive` —
whose role is to merge the delta then move the change to
`_codev/changes/archive/<date>-<name>/`. The skill MUST refuse to act if
`codev archive` reports a failed validation pre-flight.

#### Scenario: Archive of a validated change

- **GIVEN** a change whose planning is complete and which passes
  `codev validate`
- **WHEN** the user types `/codev-archive`
- **THEN** the skill runs `codev archive <name>`
- **AND** summarizes for the user the main specs touched
- **AND** names the dated archive destination

#### Scenario: Archive refused for a validation error

- **GIVEN** a change one of whose deltas contains an error reported by
  `codev validate` (for example, a duplicated requirement)
- **WHEN** the user types `/codev-archive`
- **THEN** the skill does not insist
- **AND** explicitly invites the user to run `codev validate <name>`
  to see the details
- **AND** does not try to guess or fix the error

### Requirement: Skills `sync` and `archive` rely on the JSON contract

The `sync` and `archive` workflows MUST invoke the CLI with `--json` and
read the structured shape (`SyncReportV1`, `ArchiveReportV1`) rather than
the human output — it is the public contract that codev guarantees stable
in its current version, and it is what makes the skill's rendering
reliable.

#### Scenario: Structured rendering of creations and updates

- **GIVEN** a change whose merge creates one main spec and updates
  another
- **WHEN** the user types `/codev-sync`
- **THEN** the rendering names the two distinctly — the created file and
  the updated file — each on its own line

#### Scenario: Archive refusal detected by stable code

- **GIVEN** a change for which `codev archive --json` refuses with the code
  `validation_failed` in its `status` array
- **WHEN** the user types `/codev-archive`
- **THEN** the skill detects the stable code in the JSON
- **AND** says exactly: "The change has errors. Run `codev validate
  <name>` to see the details."
- **AND** does not parse the human message (which may be reworded without
  notice)

### Requirement: Skills `sync` and `archive` do not request general Bash

The `sync` and `archive` workflows MUST limit themselves to `Bash(codev:*)`
and read tools in their `allowed-tools` frontmatter — they run no
verification command other than those of the codev binary, unlike `apply`,
which must be able to run project tests.

#### Scenario: General Bash does not appear

- **GIVEN** the `sync` skill shipped by the current version
- **WHEN** its frontmatter is inspected
- **THEN** the `allowed-tools` string does not contain plain `Bash` at the
  end of the list, only the `Bash(codev:*)` prefix
- **AND** the same rule holds for `archive`
