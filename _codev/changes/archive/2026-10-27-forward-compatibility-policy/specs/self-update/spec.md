## ADDED Requirements

### Requirement: The commands that give a verdict refuse a codev outside `codev_version`

When `_codev/config.yaml` sets `codev_version` and the running codev does
not satisfy it, `codev validate`, `codev sync` and `codev archive` SHALL
refuse to run: they check and write nothing, exit with code 1 and report
the code `codev_version_unsatisfied`. The message MUST name the
requirement, the running version and the fix the `codev_version` check of
`codev doctor` gives for the same case — `codev self-update`, or
`codev self-update --version <X.Y.Z>` when the running codev is above the
requirement. With `--json`, the command writes its usual document, the
code in its `status` array.

Every other command keeps working and reports the requirement as before:
`versionRequirement` in `codev status --json`, the `codev_version` check of
`codev doctor`. A project without `codev_version` is unaffected, and an
invalid `codev_version` still fails with `invalid_codev_version`. A codev
that predates this requirement does not refuse; that cannot be tested from
this codev, and the design records what it does instead.

#### Scenario: Validation refused below the requirement

- **GIVEN** a project with `codev_version: ">=99"`
- **WHEN** the user runs `codev validate --json`
- **THEN** the exit code is 1 and `status[0].code` is
  `codev_version_unsatisfied`
- **AND** the message names `>=99`, the running version and
  `codev self-update`

#### Scenario: Archive refused, nothing moved

- **GIVEN** a project with `codev_version: ">=99"` and an active change
  `add-auth` that passes validation
- **WHEN** the user runs `codev archive add-auth`
- **THEN** the command fails with `codev_version_unsatisfied`
- **AND** `_codev/changes/add-auth/` is still in place and the main specs
  are unchanged

#### Scenario: Sync refused, specs unchanged

- **GIVEN** a project with `codev_version: ">=99"` and an active change
  `add-auth` with a delta
- **WHEN** the user runs `codev sync add-auth`
- **THEN** the command fails with `codev_version_unsatisfied`
- **AND** the main specs are unchanged

#### Scenario: Above the requirement, the fix names a version

- **GIVEN** a project with `codev_version: "=0.0.1"`
- **WHEN** the user runs `codev validate`
- **THEN** the command fails with `codev_version_unsatisfied`
- **AND** the message names `codev self-update --version 0.0.1`

#### Scenario: Other commands keep working

- **GIVEN** a project with `codev_version: ">=99"` and an active change
  `add-auth`
- **WHEN** the user runs `codev status --change add-auth --json`
- **THEN** the command succeeds and `versionRequirement.satisfied` is
  `false`

#### Scenario: No requirement, no refusal

- **GIVEN** a project without `codev_version`, with no finding
- **WHEN** the user runs `codev validate`
- **THEN** the exit code is 0
