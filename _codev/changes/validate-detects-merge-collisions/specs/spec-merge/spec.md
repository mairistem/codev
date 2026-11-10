## MODIFIED Requirements

### Requirement: Semantic merge per operation

A sync SHALL apply each operation of a delta according to its own
semantics: `ADDED` inserts at the end of the `## Requirements` section,
`MODIFIED` replaces the block of the requirement with the same name down to
the character, `REMOVED` deletes the entire block, `RENAMED` retitles only
the `### Requirement:` header without touching the body. The operations
SHALL compose: a requirement `ADDED` after a requirement that the same
delta modifies or renames lands after that requirement's whole new text,
never inside it, even when that requirement ends the section. A `REMOVED`
whose requirement the spec no longer holds SHALL be skipped, as an
`ADDED` already present is: the outcome it asks for is already there,
after its own earlier sync or another change's removal.

#### Scenario: ADDED appears at the end of Requirements

- **GIVEN** a main spec `user-auth` containing `## Requirements` with
  a requirement `Login`
- **AND** an `ADDED` delta carrying a requirement `Two-Factor Authentication`
  with its scenario
- **WHEN** the user runs `codev sync <change>`
- **THEN** the main spec now contains `Login` first, then
  `Two-Factor Authentication`
- **AND** the `Login` block — its header, its description, its scenarios —
  is identical down to the character to what it was

#### Scenario: MODIFIED replaces the requirement block without touching the others

- **GIVEN** a main spec containing two requirements `Login` and
  `Session Expiration`, in that order
- **AND** a `MODIFIED` delta carrying `Session Expiration` with a new
  scenario
- **WHEN** the user runs `codev sync <change>`
- **THEN** the `Session Expiration` block has been replaced by the delta's
  version
- **AND** the `Login` block and the whitespace separating the two
  requirements remain identical down to the character

#### Scenario: REMOVED deletes the entire requirement block

- **GIVEN** a main spec containing `Login` and `Remember Me`
- **AND** a `REMOVED` delta carrying `Remember Me`
- **WHEN** the user runs `codev sync <change>`
- **THEN** the main spec no longer contains any trace of `Remember Me`
- **AND** the `Login` block remains identical down to the character

#### Scenario: RENAMED retitles the header and nothing else

- **GIVEN** a main spec containing `Session Expiration` with its
  scenarios
- **AND** a `RENAMED` delta mapping `Session Expiration` to `Session Timeout`
- **WHEN** the user runs `codev sync <change>`
- **THEN** the requirement header has become `### Requirement: Session Timeout`
- **AND** the description and scenarios of that requirement are identical
  down to the character

#### Scenario: The last requirement modified and another added

- **GIVEN** a main spec whose `## Requirements` ends with `Login`
- **AND** a delta that modifies `Login`, making it longer, and adds
  `Export`
- **WHEN** the user runs `codev sync <change>`
- **THEN** the main spec holds the delta's `Login` whole, then `Export`
- **AND** no line of `Login` is cut, and a second sync changes nothing

#### Scenario: REMOVED of a requirement already gone is skipped

- **GIVEN** a main spec holding `Login` and no longer holding `Remember Me`,
  removed by another change already merged
- **AND** a change whose delta removes `Remember Me`
- **WHEN** the user runs `codev archive <change>`
- **THEN** the change is archived
- **AND** the `Login` block is identical down to the character

### Requirement: Validation pre-flight before sync and archive

`codev sync` and `codev archive` MUST refuse to act if
`codev validate <change>` reports at least one error; they MUST do so
without performing the merge, without writing, and without moving anything,
and report the stable code `validation_failed`. An `ADDED` requirement that
the main spec already holds with a different content is such an error: it
is refused, never skipped. So is a main spec the change's deltas target
that defines a requirement twice or holds a conflict marker: the merge
would not know which copy to edit.

#### Scenario: A change with a validation error is not archived

- **GIVEN** a change in which a delta has a `duplicate_requirement`
- **WHEN** the user runs `codev archive <change>`
- **THEN** no main spec is modified
- **AND** the change remains active at its original location
- **AND** the error message names the code `validation_failed` and invites
  the user to run `codev validate <change>` to see the details

#### Scenario: A sync with a validation error merges nothing

- **GIVEN** a change containing a well-formed delta but also a requirement
  without `SHALL` (an error finding reported by validate)
- **WHEN** the user runs `codev sync --change <change>`
- **THEN** no main spec is modified
- **AND** the JSON error carries the code `validation_failed`

#### Scenario: An addition another branch already made is not archived

- **GIVEN** a change whose delta adds `Export`
- **AND** a main spec that already holds an `Export` requirement with a
  different description
- **WHEN** the user runs `codev archive <change>`
- **THEN** the main spec is unchanged and the change remains active
- **AND** the error carries the code `validation_failed`

#### Scenario: Re-syncing an addition already synced stays possible

- **GIVEN** a change whose delta adds `Export`, already synced, so that the
  main spec holds the same `Export`
- **WHEN** the user runs `codev archive <change>`
- **THEN** the change is archived
- **AND** the main spec holds `Export` once

#### Scenario: A change targeting a spec with a duplicate is not archived

- **GIVEN** a main spec holding `Login` twice, left by a text merge
- **AND** a change whose delta modifies `Login`
- **WHEN** the user runs `codev archive <change>`
- **THEN** the main spec is unchanged and the change remains active
- **AND** the error carries the code `validation_failed`
