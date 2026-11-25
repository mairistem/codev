## Purpose

Provide a reliable, located verdict on a change or a main spec: everything
that `archive` and `sync` would refuse to write must be reported here,
upstream, with the line concerned and a stable code that a consumer can
test.

## ADDED Requirements

### Requirement: Structural rules beyond the parser

The validator MUST reject as errors the defects that the parser lets
through because they require inspecting the content of a requirement or a
spec: a requirement without a `SHALL` or `MUST` keyword in its
description, a requirement with no scenario, a main spec without any
extractable requirement.

#### Scenario: Requirement without SHALL or MUST

- **GIVEN** a `## ADDED Requirements` delta with a `### Requirement: X`
  whose descriptive text is "The system does something"
- **WHEN** the validator inspects the file
- **THEN** a finding with code `requirement_no_shall` reports the heading
  line and names the requirement `X`

#### Scenario: Requirement without any scenario

- **GIVEN** a `## ADDED Requirements` delta with a `### Requirement: Y`
  followed by its descriptive text but by no `#### Scenario:`
- **WHEN** the validator inspects the file
- **THEN** a finding with code `requirement_no_scenario` reports the
  requirement's line

#### Scenario: Main spec without a requirement

- **GIVEN** a main spec `_codev/specs/x/spec.md` with `## Purpose`
  but whose `## Requirements` is empty
- **WHEN** the validator inspects the spec
- **THEN** a finding with code `spec_no_requirement` reports that the spec
  has no extractable requirement

### Requirement: Consistency between sections of the same delta

The validator MUST detect inconsistencies between the four operations of
the same delta: the same requirement cannot appear in two sections at
once, a `RENAMED.TO` cannot coincide with a same-named `ADDED`, and a
`MODIFIED` cannot reference the old name of a `RENAMED`.

#### Scenario: Requirement present in ADDED and MODIFIED

- **GIVEN** a delta containing both `## ADDED Requirements` with
  `### Requirement: Z` and `## MODIFIED Requirements` with `### Requirement: Z`
- **WHEN** the validator inspects the delta
- **THEN** a finding with code `cross_section_conflict` names the
  requirement `Z`, the two sections involved and their respective lines

#### Scenario: RENAMED.TO collides with a same-named ADDED

- **GIVEN** a delta containing `## ADDED Requirements` with
  `### Requirement: New Name` and `## RENAMED Requirements` with
  `FROM: Old Name` / `TO: New Name`
- **WHEN** the validator inspects the delta
- **THEN** a finding with code `rename_target_collision` reports the
  collision on `New Name`

#### Scenario: MODIFIED references the old name of a RENAMED

- **GIVEN** a delta containing `## MODIFIED Requirements` with
  `### Requirement: Old Name` and `## RENAMED Requirements` with
  `FROM: Old Name` / `TO: New Name`
- **WHEN** the validator inspects the delta
- **THEN** a finding with code `modified_uses_old_name` asks to use
  `New Name` in the MODIFIED section

### Requirement: Explicit zero-delta rule

A change has to either produce at least one spec delta, or explicitly
declare that it will produce none; the validator MUST reject the cases
that contradict this rule.

#### Scenario: Change without any delta and without skip_specs

- **GIVEN** a change whose `specs/` folder is empty and whose
  `change.yaml` does not set `skip_specs: true`
- **WHEN** the validator inspects the change
- **THEN** a finding with code `zero_delta_without_marker` asks either to
  add a delta, or to declare `skip_specs: true`

#### Scenario: skip_specs declared but specs exist

- **GIVEN** a change whose `change.yaml` declares `skip_specs: true` and
  whose `specs/` folder contains at least one `.md` file
- **WHEN** the validator inspects the change
- **THEN** a finding with code `skip_specs_conflict` asks to remove
  `skip_specs: true` or to delete the files in the `specs/` folder

### Requirement: Overall verdict and exit code

On demand, the validator MUST produce an overall verdict on a single
item, on all changes, on all specs, or on both; it MUST distinguish a run
without any error from a run that reports some.

#### Scenario: Validation of a named item

- **GIVEN** an initialized project with a change `add-auth`
- **WHEN** the user runs `codev validate add-auth`
- **THEN** the report concerns only this change and its exit code reflects
  the presence or absence of errors

#### Scenario: Batch validation

- **GIVEN** a project containing two changes and one main spec
- **WHEN** the user runs `codev validate --all`
- **THEN** the report covers the three items in a single document

#### Scenario: No error, exit code zero

- **GIVEN** a well-formed change, without an error finding
- **WHEN** the user runs `codev validate add-auth`
- **THEN** the process exits with code `0`

#### Scenario: At least one error, non-zero exit code

- **GIVEN** a change containing at least one finding of severity `Error`
- **WHEN** the user runs `codev validate add-auth`
- **THEN** the process exits with code `1`

### Requirement: JSON report with a stable contract

On `--json` request, the validator MUST write to stdout exactly one JSON
document whose shape is frozen per version: list of validated items, and
for each its list of findings with `code`, `severity`, `path`, `line`,
`message`, plus a `status` array at the root where execution errors land
(root not found, unknown item).

#### Scenario: JSON output of a successful run

- **GIVEN** a well-formed change
- **WHEN** the user runs `codev validate add-auth --json`
- **THEN** stdout carries a single JSON document containing the list of
  items, each with an empty `findings` array, and an empty root `status`

#### Scenario: JSON output when the root cannot be found

- **GIVEN** a folder outside any `_codev/` root
- **WHEN** the user runs `codev validate --json`
- **THEN** stdout carries a single JSON document with the report's shape,
  with its item lists empty, and a root `status` carrying an error entry
  with a stable code
