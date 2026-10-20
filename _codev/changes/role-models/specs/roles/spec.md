## ADDED Requirements

### Requirement: A role may carry the model its sub-agent runs on

Each role of the catalog SHALL carry a model: `opus`, `sonnet`, `haiku`,
or none, meaning the model of the session that runs the skill. The
built-in defaults SHALL be `sonnet` for `tracer`, `skeptic`, `newcomer`,
`security`, `performance` and `migration`, whose findings the evaluation
suite measured unchanged on that model, and none for every other role.

A role's mapping under `roles:` in `_codev/config.yaml` MAY set `model:`
to one of `opus`, `sonnet`, `haiku`, or `inherit` for none; it replaces
the role's default. Any other value SHALL be reported with the code
`invalid_role_model`, naming the role, the file and the accepted values:
`codev validate` reports it as an error finding, and `codev status`,
`codev instructions` and `codev roles` fail with exit code 1 and that
code, as for the other invalid role settings. A change's `change.yaml`
sets no model.

With `--json`, `codev roles` SHALL give each role `model` (`opus`,
`sonnet`, `haiku` or `null`) and `modelOrigin` (`default` or
`project`). The model applies only where a role runs as its own
sub-agent, in the `isolated` review mode; in the `inline` mode every role
runs in the session's model.

#### Scenario: The default models

- **GIVEN** a project with no `roles:` key
- **WHEN** the user runs `codev roles --json`
- **THEN** `skeptic` has `model: "sonnet"` and `modelOrigin: "default"`
- **AND** `privacy` and `architect` have `model: null`

#### Scenario: A project sets a role's model

- **GIVEN** a configuration with `roles: { privacy: { model: sonnet },
  skeptic: { model: inherit } }`
- **WHEN** the user runs `codev roles --json`
- **THEN** `privacy` has `model: "sonnet"` and `skeptic` has
  `model: null`, both with `modelOrigin: "project"`

#### Scenario: An unknown model

- **GIVEN** a configuration with `roles: { skeptic: { model: gpt } }`
- **WHEN** the user runs `codev validate --json`
- **THEN** the `config` item holds an error `invalid_role_model` naming
  `skeptic` and the accepted values
- **AND** `codev roles --json` fails with exit code 1 and
  `status[0].code` `invalid_role_model`

## MODIFIED Requirements

### Requirement: The project configures roles under `roles:`

`_codev/config.yaml` SHALL accept a top-level `roles:` mapping from a role
id to either an activation (`always`, `never` or `on_domain`), or a mapping
with an optional `when:` (the same values, the role's default when absent),
an optional `done_when:` list and an optional `model:` (see the model
requirement). The `done_when:` entries SHALL be added
after the role's own criteria, never replacing them, each with the origin
`project`; blank entries are ignored. Any other key inside a role's mapping
MUST make the configuration invalid, like an unknown key of `changelog`.
The `roles:` key of an inherited source's configuration MUST be ignored.

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

- **GIVEN** a configuration where `skeptic` holds the key `done-when`
- **WHEN** the user runs `codev status`
- **THEN** the command fails with the code `invalid`, naming the key

#### Scenario: Roles of an inherited source are ignored

- **GIVEN** a project inheriting from a source whose configuration holds
  `roles: { skeptic: never }`
- **WHEN** the user runs `codev roles --at plan.challenge --json`
- **THEN** `roles` holds `skeptic`
