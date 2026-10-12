## MODIFIED Requirements

### Requirement: `codev instructions` fails with stable codes

`codev instructions` SHALL fail with exit code 1 and:

- `unknown_artifact` when `ARTIFACT` is not an artifact of the change's
  schema, naming the artifact and the schema;
- `no_artifact_ready` when `ARTIFACT` is omitted and no artifact is
  `ready`, pointing to `codev status --change <name>`;
- `template_not_found` when the artifact declares a template that its
  project schema's `templates/` folder does not hold;
- the codes of the change choice specified by the `changes` capability.

With `--json`, the document carries the code in `status`, with `artifact`,
`changeName` and the other single values `null`, the arrays empty, and
`language` set as on success — the `language:` of the project
configuration, `en` when absent — or `en` when the configuration cannot be
read, so that the failure keeps every field of the success document.

#### Scenario: Unknown artifact

- **WHEN** the user runs `codev instructions sketch --change add-auth --json`
- **THEN** the exit code is 1
- **AND** `status[0].code` is `unknown_artifact`, its message names
  `sketch` and `spec-driven`, and `artifact` is `null`

#### Scenario: The failure document keeps the configured language

- **GIVEN** a configuration with `language: fr`
- **WHEN** the user runs `codev instructions sketch --change add-auth --json`
- **THEN** the exit code is 1
- **AND** `language` is `fr`

#### Scenario: The failure document outside a project

- **GIVEN** a directory outside any codev project
- **WHEN** the user runs `codev instructions --json`
- **THEN** `status[0].code` is `no_codev_root`
- **AND** `language` is `en`

#### Scenario: Nothing left to write

- **GIVEN** a change where every artifact is written
- **WHEN** the user runs `codev instructions --change add-auth`
- **THEN** the command fails with `no_artifact_ready`

#### Scenario: A template missing from a project schema

- **GIVEN** a project schema whose artifact `brief` declares
  `template: brief.md` and no `templates/brief.md` file
- **WHEN** the user runs `codev instructions brief --change quick`
- **THEN** the command fails with `template_not_found`
