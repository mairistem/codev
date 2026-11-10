## MODIFIED Requirements

### Requirement: The project and a change set the review mode

`_codev/config.yaml` SHALL accept a `review:` mapping whose `mode` is
`inline` or `isolated`. A change's `change.yaml` SHALL accept the same
`review:` mapping. The effective mode of a change SHALL be the change's
`review.mode` when set, otherwise the project's, otherwise `inline`; its
origin is `change`, `project` or `default` accordingly. Without a change,
the effective mode is the project's, or `inline`. An inherited source's
`review:` MUST NOT be read: the mode belongs to the team writing in the
repository.

A `mode` other than `inline` or `isolated`, in either file, SHALL make
every command that reads that file fail with the stable code
`invalid_review_mode`, naming the file, the value and the allowed values.
A `review:` that is not a mapping, or a `mode` that is not text, fails
the same way. An unknown key under `review:` in `change.yaml` is ignored
with a `change_unknown_key` warning, as any unknown key of that file is.

#### Scenario: The default mode is inline

- **GIVEN** a project whose configuration has no `review:` key and a
  change `add-auth` whose `change.yaml` has none either
- **WHEN** the user runs `codev roles --at code.review --change add-auth
  --json`
- **THEN** `review.mode` is `inline` and `review.origin` is `default`

#### Scenario: The project sets isolated

- **GIVEN** a configuration with `review: { mode: isolated }`
- **WHEN** the user runs `codev roles --at plan.challenge --json`
- **THEN** `review.mode` is `isolated` and `review.origin` is `project`

#### Scenario: A change overrides the project

- **GIVEN** a configuration with `review: { mode: isolated }` and a change
  `fix-typo` whose `change.yaml` sets `review: { mode: inline }`
- **WHEN** the user runs `codev roles --at code.review --change fix-typo
  --json`
- **THEN** `review.mode` is `inline` and `review.origin` is `change`

#### Scenario: An unknown mode in the configuration

- **GIVEN** a configuration with `review: { mode: parallel }`
- **WHEN** the user runs `codev status --json`
- **THEN** the exit code is 1 and `status[0].code` is
  `invalid_review_mode`
- **AND** the message names `parallel`, `inline` and `isolated`

#### Scenario: An unknown mode in a change

- **GIVEN** a change whose `change.yaml` sets `review: { mode: alone }`
- **WHEN** the user runs `codev roles --at code.review --change <name>
  --json`
- **THEN** the exit code is 1 and `status[0].code` is
  `invalid_review_mode`, naming `change.yaml`

#### Scenario: An inherited source's mode is not read

- **GIVEN** a project inheriting a source whose configuration sets
  `review: { mode: isolated }`, and no `review:` key of its own
- **WHEN** the user runs `codev roles --at code.review --json`
- **THEN** `review.mode` is `inline` and `review.origin` is `default`
