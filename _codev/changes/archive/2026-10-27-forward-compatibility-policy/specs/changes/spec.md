## ADDED Requirements

### Requirement: Unknown keys of `change.yaml` warn instead of failing

When a change's `change.yaml` holds a top-level key, or a key under its
`review:` mapping, that the running codev does not know, every command
that reads the change SHALL ignore that key, use the rest of the file, and
report one warning with the code `change_unknown_key` per unknown key. The
warning MUST name the key (`review.<key>` for a key of `review`) and the
change's `change.yaml`, say that the change may have been written by a
newer codev, quote the project's `codev_version` requirement when
`_codev/config.yaml` sets one, and point to `codev self-update`, or to
correcting the key if it is a typo. Commands that report configuration
warnings MUST report it the same way, in their human output and in the
`status` array of their JSON output.

`codev validate` SHALL report each unknown key as a finding of severity
`warning` and code `change_unknown_key` on the change's item, at the line
of the key in `change.yaml`; like any warning, it fails
`codev validate --strict` and not `codev validate`. A value that is not
one the key accepts — a `review.mode` or a role activation, for instance —
still fails as its own requirement says: a value cannot be ignored without
guessing what the change meant.

#### Scenario: An unknown top-level key

- **GIVEN** a project with `codev_version: ">=0.6"` and a change `add-auth`
  whose `change.yaml` holds `schema: spec-driven` and the unknown key
  `ticket: JVS-12`
- **WHEN** the user runs `codev status --change add-auth --json`
- **THEN** the command succeeds and reports the change's artifacts
- **AND** `status` holds a `change_unknown_key` warning whose message
  names `ticket`, `change.yaml`, quotes `>=0.6` and names
  `codev self-update`

#### Scenario: An unknown key under `review`

- **GIVEN** a change `add-auth` whose `change.yaml` holds
  `review: { mode: isolated, budget: 3 }`
- **WHEN** the user runs `codev roles --at code.review --change add-auth
  --json`
- **THEN** `review.mode` is `isolated` and `review.origin` is `change`
- **AND** `status` holds a `change_unknown_key` warning naming
  `review.budget`

#### Scenario: Strict validation catches a misspelled key

- **GIVEN** a change `add-auth` with a valid delta, whose `change.yaml`
  holds `skip_spec: true` on line 3, and no other finding
- **WHEN** the user runs `codev validate add-auth --strict --json`
- **THEN** the item of `add-auth` holds a `warning` finding
  `change_unknown_key` naming `skip_spec` at line 3 of its `change.yaml`
- **AND** the exit code is 1

#### Scenario: Non-strict validation with an unknown key

- **GIVEN** a change `add-auth` with a valid delta, whose `change.yaml`
  holds the unknown key `ticket: JVS-12`, and no other finding
- **WHEN** the user runs `codev validate add-auth`
- **THEN** the warning is reported and the exit code is 0
