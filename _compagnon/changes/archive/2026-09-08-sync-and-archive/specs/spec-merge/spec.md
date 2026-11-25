## Purpose

Bring a delta into the main specs without collateral damage: what the
delta describes changes; everything else — comments, order, spacing, free
sections — stays character for character what it was. Close a change's
cycle with a chronological move to the archive.

## ADDED Requirements

### Requirement: Semantic merge per operation

A sync SHALL apply each operation of a delta according to its own
semantics: `ADDED` inserts at the end of the `## Requirements` section,
`MODIFIED` replaces the block of the same-named requirement character for
character, `REMOVED` deletes the whole block, `RENAMED` retitles only the
`### Requirement:` heading without touching the body.

#### Scenario: ADDED appears at the end of Requirements

- **GIVEN** a main spec `user-auth` containing `## Requirements` with
  a requirement `Login`
- **AND** an `ADDED` delta carrying a requirement `Two-Factor Authentication`
  with its scenario
- **WHEN** the user runs `codev sync <change>`
- **THEN** the main spec now contains `Login` first then
  `Two-Factor Authentication`
- **AND** the `Login` block — its heading, its description, its scenarios —
  is identical character for character to what it was

#### Scenario: MODIFIED replaces the requirement's block without touching the others

- **GIVEN** a main spec containing two requirements `Login` and
  `Session Expiration`, in that order
- **AND** a `MODIFIED` delta carrying `Session Expiration` with a new
  scenario
- **WHEN** the user runs `codev sync <change>`
- **THEN** the `Session Expiration` block has been replaced by the delta's
  version
- **AND** the `Login` block and the spacing separating the two
  requirements stay identical character for character

#### Scenario: REMOVED deletes the requirement's whole block

- **GIVEN** a main spec containing `Login` and `Remember Me`
- **AND** a `REMOVED` delta carrying `Remember Me`
- **WHEN** the user runs `codev sync <change>`
- **THEN** the main spec no longer contains any trace of `Remember Me`
- **AND** the `Login` block stays identical character for character

#### Scenario: RENAMED retitles the heading and nothing else

- **GIVEN** a main spec containing `Session Expiration` with its
  scenarios
- **AND** a `RENAMED` delta mapping `Session Expiration` to `Session Timeout`
- **WHEN** the user runs `codev sync <change>`
- **THEN** the requirement's heading has become `### Requirement: Session Timeout`
- **AND** the description and scenarios of this requirement are identical
  character for character

### Requirement: Preservation of unmentioned content

A sync MUST leave strictly unchanged everything the delta does not
mention: the other requirements, HTML comments, fenced code blocks, free
sections after `## Requirements`, and even the spacing between
requirements.

#### Scenario: A free section after Requirements survives a sync

- **GIVEN** a main spec that contains, after `## Requirements`, a
  `## Notes` section with a paragraph
- **AND** a delta that does not mention this section
- **WHEN** the user runs `codev sync <change>`
- **THEN** the `## Notes` section is present character for character in
  the file after sync

#### Scenario: An HTML comment in an untouched requirement survives

- **GIVEN** a requirement `Login` containing an HTML comment in its
  description
- **AND** a delta that does not touch `Login`
- **WHEN** the user runs `codev sync <change>`
- **THEN** the HTML comment of `Login` stays identical character for
  character

### Requirement: Creation of a main spec for a new capability

When the delta targets a capability that does not yet have a main spec
under `_codev/specs/`, a sync SHALL create the file
`_codev/specs/<path>/spec.md` from the delta's `## Purpose` and its
`ADDED` requirements.

#### Scenario: New capability created from Purpose and ADDED

- **GIVEN** no main spec under `_codev/specs/user-auth/`
- **AND** a delta carrying `## Purpose` and a requirement `ADDED: Login`
- **WHEN** the user runs `codev sync <change>`
- **THEN** the file `_codev/specs/user-auth/spec.md` now exists
- **AND** it starts with the delta's `## Purpose` section
- **AND** it contains the delta's `Login` requirement under
  `## Requirements`

#### Scenario: New capability without Purpose is refused

- **GIVEN** no main spec under `_codev/specs/x/`
- **AND** a delta carrying only `ADDED` entries without `## Purpose`
- **WHEN** the user runs `codev sync <change>`
- **THEN** no write takes place
- **AND** an error message names the capability, states that `## Purpose`
  is required for a new capability, and recalls the stable code
  `new_capability_without_purpose`

### Requirement: Atomicity of the merge plan

A sync or an archive MUST validate its complete plan — each main spec to
rewrite, each main spec to create, each move — before performing the
slightest write. A single unresolved operation prevents all the others.

#### Scenario: MODIFIED on a missing requirement refuses the whole sync

- **GIVEN** a delta containing two `MODIFIED`, one on `Login` which exists,
  the other on `Phantom` which does not exist in the main spec
- **WHEN** the user runs `codev sync <change>`
- **THEN** no write takes place on the main spec
- **AND** an error message names `Phantom`, states that the main spec
  does not contain it, and recalls the stable code `modified_target_missing`

#### Scenario: REMOVED on the last requirement refuses the operation

- **GIVEN** a main spec containing only a single requirement
- **AND** a delta containing a `REMOVED` on that requirement
- **WHEN** the user runs `codev sync <change>`
- **THEN** no write takes place
- **AND** an error message points to the stable code
  `would_leave_spec_without_requirement` and explains that
  `retire_capabilities` is not supported yet

### Requirement: Sync leaves the change active; archive moves it

A sync SHALL leave the change's folder in place; an archive MUST move it
to `_codev/changes/archive/<date>-<name>/`, where `<date>` is the local
date in `YYYY-MM-DD` format and `<name>` the change's identifier.

#### Scenario: Sync does not move the change

- **GIVEN** a change `add-auth` whose merge succeeds
- **WHEN** the user runs `codev sync add-auth`
- **THEN** the folder `_codev/changes/add-auth/` still exists at that
  location
- **AND** its content is identical to what it was before the sync

#### Scenario: Archive moves the change to the dated archive

- **GIVEN** a change `add-auth` whose merge succeeds on date `2026-09-08`
- **WHEN** the user runs `codev archive add-auth`
- **THEN** `_codev/changes/add-auth/` no longer exists
- **AND** `_codev/changes/archive/2026-09-08-add-auth/` exists, with all
  the change's files preserved

### Requirement: Validation pre-flight before archive

`codev archive` MUST refuse to act if `codev validate <change>` reports at
least one error; it MUST do so without proceeding with the merge, without
writing, and without moving.

#### Scenario: A change with a validation error is not archived

- **GIVEN** a change one of whose deltas has a `duplicate_requirement`
- **WHEN** the user runs `codev archive <change>`
- **THEN** no main spec is modified
- **AND** the change stays active at its original location
- **AND** the error message names the code `validation_failed` and invites
  running `codev validate <change>` to see the details

#### Scenario: A sync does not require full validation

- **GIVEN** a change containing a well-formed delta but also a requirement
  without `SHALL` (error finding reported by validate)
- **WHEN** the user runs `codev sync <change>`
- **THEN** the merge takes place; the sync pre-flight is limited to the
  invariants strictly necessary for the merge (existence of the `MODIFIED`
  targets, presence of a Purpose for a new capability)

### Requirement: Report with a stable contract

On `--json` request, sync and archive MUST write to stdout exactly one
JSON document whose shape is frozen per version, listing the files
written, created, or moved, with a root `status` array for execution
errors.

#### Scenario: JSON report of a successful sync

- **GIVEN** a change whose merge touches a single existing main spec
- **WHEN** the user runs `codev sync <change> --json`
- **THEN** stdout carries a single JSON document containing the `updated`
  field with the path of the modified main spec, an empty `created` field,
  and an empty root `status`

#### Scenario: JSON report of a successful archive

- **GIVEN** a change whose archive succeeds
- **WHEN** the user runs `codev archive <change> --json`
- **THEN** the JSON document additionally contains the `movedTo` field
  with the path under `changes/archive/<date>-<name>/`
