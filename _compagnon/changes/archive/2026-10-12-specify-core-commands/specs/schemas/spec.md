## Purpose

Let a project use the built-in `spec-driven` workflow or define its own, as
plain data files, and make the workflow of each change explicit and stable
once the change is created.

## ADDED Requirements

### Requirement: `compagnon schemas` lists the usable schemas

`compagnon schemas` SHALL list the project schemas — each folder
`_compagnon/schemas/<name>/` holding a `schema.yaml`, named after the folder, in
alphabetical order, with origin `project` — then the built-in `spec-driven`
with origin `built-in`, unless a project schema named `spec-driven` already
replaces it. Each schema SHALL come with its flow: its artifact ids in
dependency order, ties broken by declaration order.

A project schema that cannot be used SHALL NOT prevent listing the others:
it is left out of the list and reported as a warning `schema_unusable`
naming it and the reason. The command exits with code 0 in that case.

The human output SHALL print `Available schemas:`, then for each schema a
line `<name> (<origin>)` followed by its flow joined with `→`. With
`--json`, the document SHALL be `{ "schemas": [{ "name", "origin", "flow" }],
"root": <project root>, "status": [...] }`.

#### Scenario: Only the built-in schema

- **GIVEN** a project without `_compagnon/schemas/<name>/schema.yaml`
- **WHEN** the user runs `compagnon schemas --json`
- **THEN** `schemas` is one entry `spec-driven`, origin `built-in`, flow
  `["proposal", "specs", "design", "tasks"]`

#### Scenario: A project schema is listed before the built-in one

- **GIVEN** a valid project schema `lite` with the artifacts `brief` then
  `plan`, `plan` requiring `brief`
- **WHEN** the user runs `compagnon schemas`
- **THEN** the output lists `lite (project)` with `brief → plan`, then
  `spec-driven (built-in)`

#### Scenario: A broken project schema is reported, the others listed

- **GIVEN** a project schema `broken` whose artifact requires an artifact
  that does not exist
- **WHEN** the user runs `compagnon schemas --json`
- **THEN** the exit code is 0
- **AND** `schemas` lists `spec-driven` only
- **AND** `status` holds a warning `schema_unusable` naming `broken`

#### Scenario: A project schema replaces the built-in one

- **GIVEN** a project schema folder `_compagnon/schemas/spec-driven/`
- **WHEN** the user runs `compagnon schemas --json`
- **THEN** `spec-driven` appears once, with origin `project`

### Requirement: A schema is resolved from the project before the built-in one

Wherever a schema is needed by name, compagnon SHALL use
`_compagnon/schemas/<name>/schema.yaml` when it exists, and the built-in schema
only when the name is `spec-driven` and the project does not define it. A
project schema's templates SHALL be read from its `templates/` folder. A
name found nowhere SHALL fail with `schema_not_found`, whose message points
to `compagnon schemas`.

#### Scenario: The project's `spec-driven` wins

- **GIVEN** a project schema `spec-driven` with a single artifact
  `proposal`
- **WHEN** the user runs `compagnon status --change add-auth` on a change using
  `spec-driven`
- **THEN** only the `proposal` artifact is listed

#### Scenario: Unknown schema

- **WHEN** the user runs `compagnon new change add-auth --schema imaginary`
- **THEN** the command fails with `schema_not_found` and a message naming
  `compagnon schemas`

### Requirement: A schema file is validated before use

A `schema.yaml` SHALL be refused with `schema_unreadable` when it is not
valid YAML or holds a key compagnon does not know, and with `schema_invalid`,
naming the schema and the reason, when:

- `name` is empty or not kebab-case;
- `artifacts` is empty;
- an artifact id is not kebab-case or is declared twice;
- an artifact has an empty `generates`;
- an artifact requires an artifact that does not exist, or itself;
- the dependencies form a cycle — the message names the artifacts
  involved;
- `apply.requires` is empty or names an artifact that does not exist;
- `apply.tracks` is empty.

`version` defaults to `1`. A command that needs a refused schema fails with
its code; `compagnon schemas` reports it as `schema_unusable`.

#### Scenario: A minimal valid schema

- **GIVEN** a schema with a name, two artifacts, `apply.requires` and
  `apply.tracks`, and no `version`
- **WHEN** it is read
- **THEN** it is accepted with version `1`

#### Scenario: An unknown key

- **GIVEN** a schema holding the key `artefacts`
- **WHEN** it is read
- **THEN** it is refused with `schema_unreadable`

#### Scenario: Structural defects

- **WHEN** a schema requires a missing artifact, declares an artifact twice,
  declares no artifact, has an empty `apply.requires`, or has a cycle
  between `a` and `b`
- **THEN** each is refused with `schema_invalid`
- **AND** the cycle message names `a` and `b`

### Requirement: The schema of a change is chosen once, at creation

`compagnon new change` SHALL choose the schema from `--schema` when given,
otherwise from the `schema:` key of `_compagnon/config.yaml`, otherwise
`spec-driven`. The chosen schema MUST be resolved and valid before anything
is written: otherwise the command fails with the schema's error and creates
no folder. The schema is recorded in the change's `change.yaml`, and every
later command uses the recorded schema: changing `schema:` in the
configuration does not affect existing changes. A change folder without
`change.yaml` uses the `schema:` of the configuration, or `spec-driven`.

#### Scenario: The configured default

- **GIVEN** a configuration with `schema: lite` and a valid project schema
  `lite`
- **WHEN** the user runs `compagnon new change quick`
- **THEN** its `change.yaml` holds `schema: lite`

#### Scenario: `--schema` wins over the configuration

- **GIVEN** a configuration with `schema: lite`
- **WHEN** the user runs `compagnon new change full --schema spec-driven`
- **THEN** its `change.yaml` holds `schema: spec-driven`

#### Scenario: An unknown schema creates nothing

- **WHEN** the user runs `compagnon new change add-auth --schema imaginary`
- **THEN** the command fails with `schema_not_found`
- **AND** `_compagnon/changes/add-auth/` does not exist

#### Scenario: Changing the default leaves existing changes alone

- **GIVEN** a change created with `spec-driven`
- **WHEN** the configuration's `schema:` is changed to `lite`
- **THEN** `compagnon status` for that change still lists the four
  `spec-driven` artifacts

#### Scenario: A hand-made change folder

- **GIVEN** a change folder holding only `proposal.md`, no `change.yaml`, in
  a project without `schema:`
- **WHEN** the user runs `compagnon status --change hand-made --json`
- **THEN** `schemaName` is `spec-driven`
