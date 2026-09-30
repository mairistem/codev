# Validation Specification

## Purpose

Provide a reliable, localized verdict on a change or a main spec:
everything that `archive` and `sync` would refuse to write must be reported
here, upstream, with the line concerned and a stable code that a consumer
can test against.

## Requirements

### Requirement: Structural rules beyond the parser

The validator MUST reject as errors the defects that the parser lets
through because they require inspecting the content of a requirement or a
spec: a requirement without a `SHALL` or `MUST` keyword in its description,
a requirement with no scenario, a main spec with no extractable
requirement.

#### Scenario: Requirement without SHALL or MUST

- **GIVEN** an `## ADDED Requirements` delta with a `### Requirement: X`
  whose descriptive text is "The system does something"
- **WHEN** the validator inspects the file
- **THEN** a finding with code `requirement_no_shall` reports the line of
  the header and names the requirement `X`

#### Scenario: Requirement with no scenario at all

- **GIVEN** an `## ADDED Requirements` delta with a `### Requirement: Y`
  followed by its descriptive text but no `#### Scenario:`
- **WHEN** the validator inspects the file
- **THEN** a finding with code `requirement_no_scenario` reports the line
  of the requirement

#### Scenario: Main spec with no requirement

- **GIVEN** a main spec `_codev/specs/x/spec.md` with `## Purpose`
  but whose `## Requirements` is empty
- **WHEN** the validator inspects the spec
- **THEN** a finding with code `spec_no_requirement` reports that the spec
  has no extractable requirement

### Requirement: Consistency between sections of the same delta

The validator MUST detect inconsistencies between the four operations of
the same delta: the same requirement cannot appear in two sections at once,
a `RENAMED.TO` cannot coincide with an `ADDED` of the same name, and a
`MODIFIED` cannot reference the old name of a `RENAMED`.

#### Scenario: Requirement present in both ADDED and MODIFIED

- **GIVEN** a delta containing both `## ADDED Requirements` with
  `### Requirement: Z` and `## MODIFIED Requirements` with `### Requirement: Z`
- **WHEN** the validator inspects the delta
- **THEN** a finding with code `cross_section_conflict` names the requirement
  `Z`, the two sections involved and their respective lines

#### Scenario: RENAMED.TO collides with an ADDED of the same name

- **GIVEN** a delta containing `## ADDED Requirements` with
  `### Requirement: New Name` and `## RENAMED Requirements` with
  `FROM: Old Name` / `TO: New Name`
- **WHEN** the validator inspects the delta
- **THEN** a finding with code `rename_target_collision` reports the collision
  on `New Name`

#### Scenario: MODIFIED references the old name of a RENAMED

- **GIVEN** a delta containing `## MODIFIED Requirements` with
  `### Requirement: Old Name` and `## RENAMED Requirements` with
  `FROM: Old Name` / `TO: New Name`
- **WHEN** the validator inspects the delta
- **THEN** a finding with code `modified_uses_old_name` asks for
  `New Name` to be used in the MODIFIED section

### Requirement: Explicit zero-delta rule

A change must either produce at least one spec delta, or explicitly declare
that it will produce none; the validator MUST reject cases that contradict
this rule.

#### Scenario: Change with no delta and no skip_specs

- **GIVEN** a change whose `specs/` directory is empty and whose
  `change.yaml` does not set `skip_specs: true`
- **WHEN** the validator inspects the change
- **THEN** a finding with code `zero_delta_without_marker` asks either to
  add a delta or to declare `skip_specs: true`

#### Scenario: skip_specs declared but specs exist

- **GIVEN** a change whose `change.yaml` declares `skip_specs: true` and
  whose `specs/` directory contains at least one `.md` file
- **WHEN** the validator inspects the change
- **THEN** a finding with code `skip_specs_conflict` asks to remove
  `skip_specs: true` or to delete the files in the `specs/` directory

### Requirement: Overall verdict and exit code

On request, the validator MUST produce an overall verdict on a single item,
on all changes, on all specs, or on both; it MUST distinguish a run with no
errors from a run that reports some.

#### Scenario: Validating a named item

- **GIVEN** an initialized project with a change `add-auth`
- **WHEN** the user runs `codev validate add-auth`
- **THEN** the report covers only that change and its exit code reflects
  the presence or absence of errors

#### Scenario: Batch validation

- **GIVEN** a project containing two changes and one main spec
- **WHEN** the user runs `codev validate --all`
- **THEN** the report covers all three items in a single document

#### Scenario: No errors, exit code zero

- **GIVEN** a well-formed change with no error finding
- **WHEN** the user runs `codev validate add-auth`
- **THEN** the process exits with code `0`

#### Scenario: At least one error, non-zero exit code

- **GIVEN** a change containing at least one finding of severity `Error`
- **WHEN** the user runs `codev validate add-auth`
- **THEN** the process exits with code `1`

### Requirement: JSON report with a stable contract

When `--json` is requested, the validator MUST write exactly one JSON
document to stdout whose shape is frozen per version: the list of validated
items, and for each one its list of findings with `code`, `severity`,
`path`, `line`, `message`, plus a root-level `status` array where execution
errors land (root not found, unknown item).

#### Scenario: JSON output of a successful run

- **GIVEN** a well-formed change
- **WHEN** the user runs `codev validate add-auth --json`
- **THEN** stdout carries a single JSON document containing the list of
  items, each with an empty `findings` array, and an empty root `status`

#### Scenario: JSON output when the root cannot be found

- **GIVEN** a directory outside any `_codev/` root
- **WHEN** the user runs `codev validate --json`
- **THEN** stdout carries a single JSON document with the shape of the
  report, with its item lists empty, and a root `status` carrying an error
  entry with a stable code

### Requirement: Strict mode propagates warnings to the exit code

`codev validate [--all|--changes|--specs|<item>] [--strict]` MUST
return a non-zero exit code as soon as at least one finding is emitted,
**whatever its severity**, when `--strict` is present. Without
`--strict`, the behavior is unchanged: exit code 1 only when at least
one `Error` is present.

Strict mode **does not change** the severity of findings in the
report; it changes **only** the exit code decision rule. The human
rendering (text and JSON) remains identical.

#### Scenario: `--strict` on a clean report → exit 0

- **GIVEN** a project for which `codev validate --all` reports no
  finding
- **WHEN** the user runs `codev validate --all --strict`
- **THEN** the exit code of the command is 0

#### Scenario: `--strict` on a warning → non-zero exit

- **GIVEN** a project containing an unsealed `accepted` local ADR,
  which reports a `decision_unsealed` (warning) on validate
- **WHEN** the user runs `codev validate --strict`
- **THEN** the exit code of the command is non-zero (1)
- **AND** the human rendering still carries the `warning` line (not
  `error`) for this finding — the displayed severity is preserved

#### Scenario: Without `--strict`, a warning does not flip the exit code

- **GIVEN** the same context
- **WHEN** the user runs `codev validate` (without `--strict`)
- **THEN** the exit code of the command is 0
- **AND** the warning still appears in the output

#### Scenario: An error remains blocking, even without `--strict`

- **GIVEN** a project in which a local ADR was edited in place after
  sealing, which reports a `decision_seal_mismatch` (error)
- **WHEN** the user runs `codev validate`
- **THEN** the exit code of the command is non-zero (1) — strict mode
  is not needed for errors

### Requirement: `hasWarnings` exposed in the JSON contract

The `ValidateReportV1` JSON report MUST carry an additive field
`hasWarnings: bool` — `true` as soon as at least one finding of severity
`Warning` is present, `false` otherwise. The field is **always** present
in the output, regardless of its value or of strict mode.

This field is **informational**: a consumer that wants to decide outside
strict mode uses it. The exit code remains the official signal.

#### Scenario: `hasWarnings: true` on a report with a warning

- **GIVEN** a project that reports a `decision_unsealed` warning
- **WHEN** the user runs `codev validate --json`
- **THEN** the JSON document carries `"hasWarnings": true` at the root

#### Scenario: `hasWarnings: false` on a clean report

- **GIVEN** a project with no finding
- **WHEN** the user runs `codev validate --all --json`
- **THEN** the JSON document carries `"hasWarnings": false`

#### Scenario: `hasWarnings` independent of strict mode

- **GIVEN** a project that reports a warning and no error
- **WHEN** the user runs `codev validate --strict --json`
- **THEN** the JSON document carries `"hasWarnings": true`
- **AND** the exit code of the command is non-zero — both signals
  coexist without contradicting each other
