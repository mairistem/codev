## MODIFIED Requirements

### Requirement: The project configures roles under `roles:`

`_codev/config.yaml` SHALL accept a top-level `roles:` mapping from a role
id to either an activation (`always`, `never` or `on_domain`), or a mapping
with an optional `when:` (the same values, the role's default when absent),
an optional `done_when:` list and an optional `model:` (see the model
requirement). The `done_when:` entries SHALL be added
after the role's own criteria, never replacing them, each with the origin
`project`; blank entries are ignored. Any other key inside a role's mapping
SHALL be ignored with an `unknown_config_key` warning naming it
`roles.<id>.<key>`, as a key of `mcp` is, and `codev validate` and
`codev doctor` report it the same way; the rest of the role's mapping
still applies. A misspelled key is therefore a warning, which
`codev validate --strict` turns into a failure.
The `roles:` key of an inherited source's configuration MUST be ignored,
whatever it holds: a key this codev does not know there, written for a
newer codev, never keeps the source's context and rules from being read.

A change's `change.yaml` SHALL accept a `roles:` mapping from a role id to
an activation, which overrides the project's activation of that role for
this change only.

#### Scenario: A role disabled by the project

- **GIVEN** a configuration with `roles: { newcomer: never }`
- **WHEN** the user runs `codev roles --at code.review --json`
- **THEN** `roles` holds no `newcomer`
- **AND** `codev roles --json` lists `newcomer` with `when: "never"` and
  `whenOrigin: "project"`

#### Scenario: Extra criteria are added after the role's own

- **GIVEN** a configuration where `skeptic` has `done_when:` with the entry
  `The rollback plan is stated`
- **WHEN** the user runs `codev roles --at plan.challenge --json`
- **THEN** the `skeptic` entry's `doneWhen` holds its built-in criteria,
  each with the origin `built-in`, followed by `The rollback plan is
  stated` with the origin `project`

#### Scenario: A change overrides the activation

- **GIVEN** a configuration with `roles: { newcomer: never }` and a change
  `add-auth` whose `change.yaml` holds `roles: { newcomer: always }`
- **WHEN** the user runs `codev roles --at code.review --change add-auth
  --json`
- **THEN** `roles` holds `newcomer` with `whenOrigin: "change"`
- **AND** without `--change`, `roles` holds no `newcomer`

#### Scenario: An unknown key inside a role

- **GIVEN** a configuration where `skeptic` holds the key `cadence:
  weekly` and `model: sonnet`
- **WHEN** the user runs `codev roles --json`
- **THEN** the command succeeds and `skeptic` has `model: "sonnet"`
- **AND** `status` holds an `unknown_config_key` warning naming
  `roles.skeptic.cadence` and `codev self-update`

#### Scenario: A misspelled key inside a role fails strict validation

- **GIVEN** a configuration where `skeptic` holds the key `done-when` on
  line 4, and no other finding
- **WHEN** the user runs `codev validate --strict --json`
- **THEN** the `config` item holds a `warning` finding
  `unknown_config_key` naming `roles.skeptic.done-when` at line 4
- **AND** the exit code is 1

#### Scenario: A source's role setting from a newer codev

- **GIVEN** a project inheriting from a source whose configuration holds
  `roles: { skeptic: { cadence: weekly } }` and a context
- **WHEN** the user runs `codev instructions proposal --change add-auth --json`
- **THEN** the source's context is in `context`, and no error is reported

#### Scenario: Roles of an inherited source are ignored

- **GIVEN** a project inheriting from a source whose configuration holds
  `roles: { skeptic: never }`
- **WHEN** the user runs `codev roles --at plan.challenge --json`
- **THEN** `roles` holds `skeptic`
