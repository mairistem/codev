## MODIFIED Requirements

### Requirement: Semantic merge per operation

A sync SHALL apply each operation of a delta according to its own
semantics: `ADDED` inserts at the end of the `## Requirements` section,
`MODIFIED` replaces the block of the requirement with the same name down to
the character, `REMOVED` deletes the entire block, `RENAMED` retitles only
the `### Requirement:` header without touching the body. The operations
SHALL compose: a requirement `ADDED` after a requirement that the same
delta modifies or renames lands after that requirement's whole new text,
never inside it, even when that requirement ends the section.

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
