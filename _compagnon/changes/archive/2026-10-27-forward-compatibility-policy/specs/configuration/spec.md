## MODIFIED Requirements

### Requirement: `codev validate` reports unknown configuration keys

When it validates the whole project — no scope, or `--all` — and
`_codev/config.yaml` holds keys the running codev ignores — top-level,
`mcp`, `review` or a role's mapping under `roles:` — `codev validate`
SHALL add to its report an item of kind `config`, named `config`, whose path
is `_codev/config.yaml`, carrying one finding of severity `warning` and
code `unknown_config_key` per unknown key, at the line of that key. The
same item carries the `schema_unknown_key` findings of the project's
schemas, each located in its schema's file. The item MUST be absent when
neither the configuration nor a project schema holds an unknown key, and
from the reports of `--changes`, `--specs` and of a named item. Like any
warning, it fails `codev validate --strict` and not `codev validate`.

#### Scenario: Strict validation with an unknown key

- **GIVEN** a project whose `_codev/config.yaml` holds the unknown
  top-level key `review_policy` on line 3, and no other finding
- **WHEN** the user runs `codev validate --strict --json`
- **THEN** the report holds an item of kind `config` with one `warning`
  finding `unknown_config_key` at line 3 of `_codev/config.yaml`
- **AND** the exit code is 1

#### Scenario: Non-strict validation with an unknown key

- **GIVEN** the same project
- **WHEN** the user runs `codev validate`
- **THEN** the warning is reported and the exit code is 0

#### Scenario: Clean configuration

- **GIVEN** a project whose `_codev/config.yaml` and project schemas hold
  only known keys
- **WHEN** the user runs `codev validate --all --json`
- **THEN** no item of kind `config` is reported
