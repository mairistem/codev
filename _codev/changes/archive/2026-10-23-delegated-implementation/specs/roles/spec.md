## MODIFIED Requirements

### Requirement: A role may carry the model its sub-agent runs on

Each role of the catalog SHALL carry a model: `opus`, `sonnet`, `haiku`,
or none, meaning the model of the session that runs the skill. No built-in
role sets one: every role runs on the session's model until the project
chooses another.

A role's mapping under `roles:` in `_codev/config.yaml` MAY set `model:`
to one of `opus`, `sonnet`, `haiku`, or `inherit` for none; it replaces
the role's default. Any other value SHALL be reported with the code
`invalid_role_model`, naming the role, the file and the accepted values:
`codev validate` reports it as an error finding, and `codev status`,
`codev instructions` and `codev roles` fail with exit code 1 and that
code, as for the other invalid role settings. A change's `change.yaml`
sets no model. A model set on a role that serves no reviewing step
(`plan.trace`, `plan.challenge`, `code.review`) and is not the
`developer` never applies: it SHALL be reported with the warning
`role_model_unused`, naming the role. Only the `developer`'s model is
used at `code.implement`: a project role declared at that step that
carries a model is warned too.

With `--json`, `codev roles` SHALL give each role `model` (`opus`,
`sonnet`, `haiku` or `null`) and `modelOrigin` (`default` or
`project`). A reviewing role's model applies only where it runs as its
own sub-agent, in the `isolated` review mode; in the `inline` mode every
reviewing role runs in the session's model. The `developer`'s model, at
`code.implement`, applies in either mode: the `apply` skill then hands
the tasks to a sub-agent on that model.

#### Scenario: The default models

- **GIVEN** a project with no `roles:` key
- **WHEN** the user runs `codev roles --json`
- **THEN** `skeptic`, `privacy` and `architect` have `model: null` and
  `modelOrigin: "default"`

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

#### Scenario: A model on a role that is never a sub-agent

- **GIVEN** a configuration with `roles: { product: { model: sonnet } }`
- **WHEN** the user runs `codev validate --json`
- **THEN** the `config` item holds a warning `role_model_unused` naming
  `product`

#### Scenario: A model on another role at code.implement

- **GIVEN** a project role `pair-programmer` declared at `code.implement`
  with `model: sonnet`
- **WHEN** the user runs `codev validate --json`
- **THEN** the `config` item holds a warning `role_model_unused` naming
  `pair-programmer`

#### Scenario: A model on the developer is used

- **GIVEN** a configuration with `roles: { developer: { model: sonnet } }`
- **WHEN** the user runs `codev validate --json` and `codev roles --at
  code.implement --json`
- **THEN** no `role_model_unused` warning is reported, and the
  `developer` has `model: "sonnet"`
