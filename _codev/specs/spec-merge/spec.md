# Spec Merge Specification

## Purpose

Bring a delta into the main specs without collateral damage: what the
delta describes changes; everything else — comments, order, whitespace,
free-form sections — stays exactly as it was, down to the character. Close
the cycle of a change with a chronological move into the archive.

## Requirements

### Requirement: Semantic merge per operation

A sync SHALL apply each operation of a delta according to its own
semantics: `ADDED` inserts at the end of the `## Requirements` section,
`MODIFIED` replaces the block of the requirement with the same name down to
the character, `REMOVED` deletes the entire block, `RENAMED` retitles only
the `### Requirement:` header without touching the body.

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

### Requirement: Preservation of unmentioned content

A sync MUST leave strictly unchanged everything the delta does not
mention: the other requirements, HTML comments, fenced code blocks,
free-form sections after `## Requirements`, and even the whitespace between
requirements.

#### Scenario: A free-form section after Requirements survives a sync

- **GIVEN** a main spec that contains, after `## Requirements`, a
  `## Notes` section with a paragraph
- **AND** a delta that does not mention that section
- **WHEN** the user runs `codev sync <change>`
- **THEN** the `## Notes` section is present, down to the character, in the
  file after the sync

#### Scenario: An HTML comment in an untouched requirement survives

- **GIVEN** a requirement `Login` containing an HTML comment in its
  description
- **AND** a delta that does not touch `Login`
- **WHEN** the user runs `codev sync <change>`
- **THEN** the HTML comment of `Login` remains identical down to the character

### Requirement: Creation of a main spec for a new capability

When the delta targets a capability that does not yet have a main spec
under `_codev/specs/`, a sync SHALL create the file `_codev/specs/<path>/spec.md`
from the delta's `## Purpose` and its `ADDED` requirements.

#### Scenario: New capability created from Purpose and ADDED

- **GIVEN** no main spec under `_codev/specs/user-auth/`
- **AND** a delta carrying `## Purpose` and an `ADDED: Login` requirement
- **WHEN** the user runs `codev sync <change>`
- **THEN** the file `_codev/specs/user-auth/spec.md` now exists
- **AND** it starts with the delta's `## Purpose` section
- **AND** it contains the delta's `Login` requirement under `## Requirements`

#### Scenario: New capability without Purpose is rejected

- **GIVEN** no main spec under `_codev/specs/x/`
- **AND** a delta carrying only `ADDED` operations without `## Purpose`
- **WHEN** the user runs `codev sync <change>`
- **THEN** no write takes place
- **AND** an error message names the capability, states that `## Purpose` is
  required for a new capability, and recalls the stable code
  `new_capability_without_purpose`

### Requirement: Atomicity of the merge plan

A sync or an archive MUST validate its complete plan — every main spec to
rewrite, every main spec to create, every move — before performing any
write at all. A single unresolved operation blocks all the others.

#### Scenario: MODIFIED on a missing requirement rejects the whole sync

- **GIVEN** a delta containing two `MODIFIED` operations, one on `Login`,
  which exists, the other on `Ghost`, which does not exist in the main spec
- **WHEN** the user runs `codev sync <change>`
- **THEN** no write takes place on the main spec
- **AND** an error message names `Ghost`, states that the main spec does
  not contain it, and recalls the stable code `modified_target_missing`

#### Scenario: REMOVED on the last requirement rejects the operation

- **GIVEN** a main spec containing a single requirement
- **AND** a delta containing a `REMOVED` on that requirement
- **WHEN** the user runs `codev sync <change>`
- **THEN** no write takes place
- **AND** an error message points to the stable code
  `would_leave_spec_without_requirement` and explains that
  `retire_capabilities` is not yet supported

### Requirement: Sync leaves the change active; archive moves it

A sync SHALL leave the change directory where it is; an archive MUST move
it to `_codev/changes/archive/<date>-<name>/`, where `<date>` is the local
date in `YYYY-MM-DD` format and `<name>` is the change identifier.

#### Scenario: Sync does not move the change

- **GIVEN** a change `add-auth` whose merge succeeds
- **WHEN** the user runs `codev sync add-auth`
- **THEN** the `_codev/changes/add-auth/` directory still exists at that location
- **AND** its contents are identical to what they were before the sync

#### Scenario: Archive moves the change to the dated archive

- **GIVEN** a change `add-auth` whose merge succeeds on `2026-09-08`
- **WHEN** the user runs `codev archive add-auth`
- **THEN** `_codev/changes/add-auth/` no longer exists
- **AND** `_codev/changes/archive/2026-09-08-add-auth/` exists, with all the
  files of the change preserved

### Requirement: Validation pre-flight before archive

`codev archive` MUST refuse to act if `codev validate <change>` reports at
least one error; it MUST do so without performing the merge, without
writing, and without moving anything.

#### Scenario: A change with a validation error is not archived

- **GIVEN** a change in which a delta has a `duplicate_requirement`
- **WHEN** the user runs `codev archive <change>`
- **THEN** no main spec is modified
- **AND** the change remains active at its original location
- **AND** the error message names the code `validation_failed` and invites
  the user to run `codev validate <change>` to see the details

#### Scenario: A sync does not require full validation

- **GIVEN** a change containing a well-formed delta but also a requirement
  without `SHALL` (an error finding reported by validate)
- **WHEN** the user runs `codev sync <change>`
- **THEN** the merge takes place; the sync pre-flight is limited to the
  invariants strictly required for the merge (existence of the `MODIFIED`
  targets, presence of a Purpose for a new capability)

### Requirement: Report with a stable contract

When `--json` is requested, sync and archive MUST write exactly one JSON
document to stdout whose shape is frozen per version, listing the files
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
- **THEN** the JSON document additionally contains the `movedTo` field with
  the path under `changes/archive/<date>-<name>/`

### Requirement: Atomic deletion of an emptied spec under `retire_capabilities`

When a change carries `retire_capabilities: true` in its
`change.yaml`, and a `## REMOVED Requirements` delta removes **all**
the requirements of a main spec, `codev sync` MUST write a plan that
**deletes** the file `_codev/specs/<capability>/spec.md` in a single
atomic operation, rather than leaving it empty or rejecting it.

Without the marker, the behavior remains the current one: rejection with
the stable code `would_leave_spec_without_requirement`.

#### Scenario: Retiring a capability with the marker → file deleted

- **GIVEN** a project containing a main spec
  `_codev/specs/user-auth/spec.md` with a single requirement `Login`
- **AND** a change whose `change.yaml` carries `retire_capabilities: true`
- **AND** a delta `_codev/changes/<c>/specs/user-auth/spec.md` that
  lists the `Login` requirement under `## REMOVED Requirements`
- **WHEN** the user runs `codev sync <c>`
- **THEN** the file `_codev/specs/user-auth/spec.md` no longer exists
  after execution
- **AND** the sync report lists the file in `deleted[]`

#### Scenario: Retiring a capability without the marker → rejection

- **GIVEN** the same context, but **without** `retire_capabilities: true`
- **WHEN** the user runs `codev sync <c>`
- **THEN** no write or deletion takes place in `_codev/specs/`
- **AND** the error message names the stable code
  `would_leave_spec_without_requirement`

#### Scenario: `retire_capabilities` has no effect when a requirement remains

- **GIVEN** a main spec with the requirements `Login` and `Logout`
- **AND** a `retire_capabilities: true` change whose delta removes
  only `Login`
- **WHEN** the user runs `codev sync <c>`
- **THEN** the file `_codev/specs/<capability>/spec.md` still exists,
  still contains `Logout`, and does not appear in `deleted[]`
- **AND** the marker triggered nothing beyond a normal merge

### Requirement: `deletions` is a first-class plan operation

The core `Plan` type MUST carry a `deletions: Vec<PathBuf>` field
distinct from `writes` and `moves`, and the shell MUST apply the
deletions in a deterministic order: **after** the writes and **before**
the moves. A `Plan` without deletions keeps the historical behavior
bit-for-bit.

#### Scenario: The plan exposes deletions separately

- **GIVEN** a `plan_sync` on a change that retires a capability
- **WHEN** the inspector looks at the produced `Plan`
- **THEN** the entry for the file to delete appears in `plan.deletions`
- **AND** does not appear in `plan.writes` (no write of an empty string)

#### Scenario: Execution order — deletions after writes

- **GIVEN** a plan that both modifies a spec `A` (write) and
  deletes a spec `B` (deletion)
- **WHEN** the shell executes the plan
- **THEN** the write to `A` is applied before the deletion of `B`
- **AND** the deletion of `B` is applied before any `move`

### Requirement: The `sync` and `archive` JSON contract exposes `deleted`

The JSON report of `codev sync --json` (contract `SyncReportV1`) and of
`codev archive --json` (contract `ArchiveReportV1`) MUST carry an
additive field `deleted: Vec<String>`, always present, empty in the
common case. The field contains the absolute paths of the main specs
deleted by the change, in a deterministic order.

#### Scenario: `sync --json` with a retired capability

- **GIVEN** a project with a `user-auth` spec, and a
  `retire_capabilities: true` change that removes its only requirement
- **WHEN** the user runs `codev sync <c> --json`
- **THEN** the JSON document carries `"deleted": ["<abs>/…/user-auth/spec.md"]`
- **AND** `updated`, `created`, `unchanged` do NOT mention that path

#### Scenario: `deleted` always present, empty by default

- **GIVEN** a change without deletions
- **WHEN** the user runs `codev sync <c> --json`
- **THEN** the JSON document carries `"deleted": []`
