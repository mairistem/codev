## MODIFIED Requirements

### Requirement: `compagnon validate` reports unknown configuration keys

When it validates the whole project — no scope, or `--all` — and
`_compagnon/config.yaml` holds keys the running compagnon ignores — top-level,
`mcp`, `review` or a role's mapping under `roles:` — `compagnon validate`
SHALL add to its report an item of kind `config`, named `config`, whose path
is `_compagnon/config.yaml`, carrying one finding of severity `warning` and
code `unknown_config_key` per unknown key, at the line of that key. The
same item carries the `schema_unknown_key` findings of the project's
schemas, each located in its schema's file. The item MUST be absent when
neither the configuration nor a project schema holds an unknown key, and
from the reports of `--changes`, `--specs` and of a named item. Like any
warning, it fails `compagnon validate --strict` and not `compagnon validate`.

#### Scenario: Strict validation with an unknown key

- **GIVEN** a project whose `_compagnon/config.yaml` holds the unknown
  top-level key `review_policy` on line 3, and no other finding
- **WHEN** the user runs `compagnon validate --strict --json`
- **THEN** the report holds an item of kind `config` with one `warning`
  finding `unknown_config_key` at line 3 of `_compagnon/config.yaml`
- **AND** the exit code is 1

#### Scenario: Non-strict validation with an unknown key

- **GIVEN** the same project
- **WHEN** the user runs `compagnon validate`
- **THEN** the warning is reported and the exit code is 0

#### Scenario: Clean configuration

- **GIVEN** a project whose `_compagnon/config.yaml` and project schemas hold
  only known keys
- **WHEN** the user runs `compagnon validate --all --json`
- **THEN** no item of kind `config` is reported
