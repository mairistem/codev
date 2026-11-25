## ADDED Requirements

### Requirement: The commands that give a verdict refuse a compagnon outside `compagnon_version`

When `_compagnon/config.yaml` sets `compagnon_version` and the running compagnon does
not satisfy it, `compagnon validate`, `compagnon sync` and `compagnon archive` SHALL
refuse to run: they check and write nothing, exit with code 1 and report
the code `compagnon_version_unsatisfied`. The message MUST name the
requirement, the running version and the fix the `compagnon_version` check of
`compagnon doctor` gives for the same case — `compagnon self-update`, or
`compagnon self-update --version <X.Y.Z>` when the running compagnon is above the
requirement. With `--json`, the command writes its usual document, the
code in its `status` array.

Every other command keeps working and reports the requirement as before:
`versionRequirement` in `compagnon status --json`, the `compagnon_version` check of
`compagnon doctor`. A project without `compagnon_version` is unaffected, and an
invalid `compagnon_version` still fails with `invalid_compagnon_version`. A compagnon
that predates this requirement does not refuse; that cannot be tested from
this compagnon, and the design records what it does instead.

#### Scenario: Validation refused below the requirement

- **GIVEN** a project with `compagnon_version: ">=99"`
- **WHEN** the user runs `compagnon validate --json`
- **THEN** the exit code is 1 and `status[0].code` is
  `compagnon_version_unsatisfied`
- **AND** the message names `>=99`, the running version and
  `compagnon self-update`

#### Scenario: Archive refused, nothing moved

- **GIVEN** a project with `compagnon_version: ">=99"` and an active change
  `add-auth` that passes validation
- **WHEN** the user runs `compagnon archive add-auth`
- **THEN** the command fails with `compagnon_version_unsatisfied`
- **AND** `_compagnon/changes/add-auth/` is still in place and the main specs
  are unchanged

#### Scenario: Sync refused, specs unchanged

- **GIVEN** a project with `compagnon_version: ">=99"` and an active change
  `add-auth` with a delta
- **WHEN** the user runs `compagnon sync add-auth`
- **THEN** the command fails with `compagnon_version_unsatisfied`
- **AND** the main specs are unchanged

#### Scenario: Above the requirement, the fix names a version

- **GIVEN** a project with `compagnon_version: "=0.0.1"`
- **WHEN** the user runs `compagnon validate`
- **THEN** the command fails with `compagnon_version_unsatisfied`
- **AND** the message names `compagnon self-update --version 0.0.1`

#### Scenario: Other commands keep working

- **GIVEN** a project with `compagnon_version: ">=99"` and an active change
  `add-auth`
- **WHEN** the user runs `compagnon status --change add-auth --json`
- **THEN** the command succeeds and `versionRequirement.satisfied` is
  `false`

#### Scenario: No requirement, no refusal

- **GIVEN** a project without `compagnon_version`, with no finding
- **WHEN** the user runs `compagnon validate`
- **THEN** the exit code is 0
