## ADDED Requirements

### Requirement: The `archive` skill invites a lessons pass when lessons may be waiting

After a successful archive, when the `ArchiveReportV1` document's
`lessons.waiting` is `true`, the `archive` skill SHALL end its report with
a single line inviting the user to run
`/codev-configure lessons <lessons.window>`, naming `lessons.changes` and
`lessons.corrections`. When `lessons` is absent,
`null`, or its `waiting` is `false`, the report SHALL be unchanged. The
skill SHALL add no step, run no other command and request no other tool
for it.

#### Scenario: The skill carries the invitation

- **GIVEN** the `codev-archive` skill as `codev update` renders it
- **WHEN** its text is read
- **THEN** it ends the success report with one line inviting
  `/codev-configure lessons` when `lessons.waiting` is `true`
- **AND** its `allowed-tools` are those of the previous release

#### Scenario: Nothing waiting

- **GIVEN** an archive whose JSON has `lessons.waiting: false`
- **WHEN** the user types `/codev-archive`
- **THEN** the report holds no line about lessons

#### Scenario: An older binary

- **GIVEN** an archive whose JSON has no `lessons` field
- **WHEN** the user types `/codev-archive`
- **THEN** the report holds no line about lessons
