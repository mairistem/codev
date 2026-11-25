## RENAMED Requirements

- FROM: `### Requirement: Validation pre-flight before archive`
- TO: `### Requirement: Validation pre-flight before sync and archive`

## MODIFIED Requirements

### Requirement: Validation pre-flight before sync and archive

`compagnon sync` and `compagnon archive` MUST refuse to act if
`compagnon validate <change>` reports at least one error; they MUST do so
without performing the merge, without writing, and without moving anything,
and report the stable code `validation_failed`.

#### Scenario: A change with a validation error is not archived

- **GIVEN** a change in which a delta has a `duplicate_requirement`
- **WHEN** the user runs `compagnon archive <change>`
- **THEN** no main spec is modified
- **AND** the change remains active at its original location
- **AND** the error message names the code `validation_failed` and invites
  the user to run `compagnon validate <change>` to see the details

#### Scenario: A sync with a validation error merges nothing

- **GIVEN** a change containing a well-formed delta but also a requirement
  without `SHALL` (an error finding reported by validate)
- **WHEN** the user runs `compagnon sync --change <change>`
- **THEN** no main spec is modified
- **AND** the JSON error carries the code `validation_failed`
