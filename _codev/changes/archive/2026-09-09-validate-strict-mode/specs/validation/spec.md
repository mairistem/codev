## ADDED Requirements

### Requirement: Strict mode propagates warnings to the exit code

`codev validate [--all|--changes|--specs|<item>] [--strict]` MUST
return a non-zero exit code as soon as at least one finding is
emitted, **whatever its severity**, when `--strict` is present.
Without `--strict`, the behavior stays unchanged: exit code 1 only in
the presence of at least one `Error`.

Strict mode **does not modify** the severity of findings in the
report; it changes **only** the exit-code decision rule. The human
rendering (text and JSON) stays identical.

#### Scenario: `--strict` on a clean report → exit 0

- **GIVEN** a project for which `codev validate --all` reports no
  finding
- **WHEN** the user runs `codev validate --all --strict`
- **THEN** the command's exit code is 0

#### Scenario: `--strict` on a warning → non-zero exit

- **GIVEN** a project containing an unsealed `accepted` local ADR,
  which reports a `decision_unsealed` (warning) on validate
- **WHEN** the user runs `codev validate --strict`
- **THEN** the command's exit code is non-zero (1)
- **AND** the human rendering still shows the `warning` line (not
  `error`) for this finding — the displayed severity is preserved

#### Scenario: Without `--strict`, a warning does not flip the exit code

- **GIVEN** the same context
- **WHEN** the user runs `codev validate` (without `--strict`)
- **THEN** the command's exit code is 0
- **AND** the warning still appears in the output

#### Scenario: An error stays blocking, even without `--strict`

- **GIVEN** a project where a local ADR was edited in place after
  sealing, which reports a `decision_seal_mismatch` (error)
- **WHEN** the user runs `codev validate`
- **THEN** the command's exit code is non-zero (1) — strict mode is
  not needed for errors

### Requirement: `hasWarnings` exposed in the JSON contract

The JSON report `ValidateReportV1` MUST carry an additive field
`hasWarnings: bool` — `true` as soon as at least one finding of
severity `Warning` is present, `false` otherwise. The field is
**always** present in the output, regardless of its value or of
strict mode.

This field is **informational**: a consumer who wants to decide
outside strict mode uses it. The exit code remains the official
signal.

#### Scenario: `hasWarnings: true` on a report with a warning

- **GIVEN** a project that reports a `decision_unsealed` warning
- **WHEN** the user runs `codev validate --json`
- **THEN** the JSON document carries `"hasWarnings": true` at the root

#### Scenario: `hasWarnings: false` on a clean report

- **GIVEN** a project without any finding
- **WHEN** the user runs `codev validate --all --json`
- **THEN** the JSON document carries `"hasWarnings": false`

#### Scenario: `hasWarnings` independent of strict mode

- **GIVEN** a project that reports a warning and no error
- **WHEN** the user runs `codev validate --strict --json`
- **THEN** the JSON document carries `"hasWarnings": true`
- **AND** the command's exit code is non-zero — the two signals
  coexist without contradicting each other
