## ADDED Requirements

### Requirement: Atomic deletion of an emptied spec when `retire_capabilities`

When a change carries `retire_capabilities: true` in its
`change.yaml`, and a `## REMOVED Requirements` delta removes **all**
the requirements of a main spec, `codev sync` MUST write a plan that
**deletes** the file `_codev/specs/<capa>/spec.md` in a single atomic
operation, rather than leaving it empty or refusing.

Without the marker, the behavior remains as today: refusal with the
stable code `would_leave_spec_without_requirement`.

#### Scenario: Removing a capability with the marker → file deleted

- **GIVEN** a project containing a main spec
  `_codev/specs/user-auth/spec.md` with a single requirement `Login`
- **AND** a change whose `change.yaml` carries `retire_capabilities: true`
- **AND** a delta `_codev/changes/<c>/specs/user-auth/spec.md` that
  lists the requirement `Login` under `## REMOVED Requirements`
- **WHEN** the user runs `codev sync <c>`
- **THEN** the file `_codev/specs/user-auth/spec.md` no longer exists
  after execution
- **AND** the sync report lists the file in `deleted[]`

#### Scenario: Removing a capability without the marker → refusal

- **GIVEN** the same context, but **without** `retire_capabilities: true`
- **WHEN** the user runs `codev sync <c>`
- **THEN** no write or deletion happens on `_codev/specs/`
- **AND** the error message names the stable code
  `would_leave_spec_without_requirement`

#### Scenario: `retire_capabilities` has no effect when a requirement remains

- **GIVEN** a main spec with the requirements `Login` and `Logout`
- **AND** a `retire_capabilities: true` change whose delta removes
  only `Login`
- **WHEN** the user runs `codev sync <c>`
- **THEN** the file `_codev/specs/<capa>/spec.md` still exists,
  still contains `Logout`, and does not appear in `deleted[]`
- **AND** the marker triggered nothing beyond a normal merge

### Requirement: `deletions` is a first-class operation of the plan

The core's `Plan` type MUST carry a `deletions: Vec<PathBuf>` field
distinct from `writes` and `moves`, and the shell MUST apply the
deletions in a deterministic order: **after** the writes and
**before** the moves. A `Plan` without deletions keeps the historical
behavior bit-identical.

#### Scenario: The plan exposes deletions separately

- **GIVEN** a `plan_sync` on a change that removes a capability
- **WHEN** the inspector looks at the produced `Plan`
- **THEN** the entry for the file to delete appears in `plan.deletions`
- **AND** does not appear in `plan.writes` (no write of an empty string)

#### Scenario: Execution order — deletions after writes

- **GIVEN** a plan that both modifies a spec `A` (write) and
  deletes a spec `B` (deletion)
- **WHEN** the shell executes the plan
- **THEN** the write to `A` is applied before the deletion of `B`
- **AND** the deletion of `B` is applied before any `move`

### Requirement: `sync` and `archive` JSON contract exposes `deleted`

The JSON report of `codev sync --json` (contract `SyncReportV1`) and
of `codev archive --json` (contract `ArchiveReportV1`) MUST carry an
additive field `deleted: Vec<String>`, always present, empty in the
usual case. The field contains the absolute paths of the main specs
deleted by the change, in a deterministic order.

#### Scenario: `sync --json` with a removed capability

- **GIVEN** a project with a `user-auth` spec, and a
  `retire_capabilities: true` change that removes its only requirement
- **WHEN** the user runs `codev sync <c> --json`
- **THEN** the JSON document carries `"deleted": ["<abs>/…/user-auth/spec.md"]`
- **AND** `updated`, `created`, `unchanged` do NOT mention that path

#### Scenario: `deleted` always present, empty by default

- **GIVEN** a change without any deletion
- **WHEN** the user runs `codev sync <c> --json`
- **THEN** the JSON document carries `"deleted": []`
