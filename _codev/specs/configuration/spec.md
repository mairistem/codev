# Configuration Specification

## Purpose

Describes how codev reads `_codev/config.yaml` when it holds keys the
running codev does not know — typically keys added by a newer codev — so
that an older binary keeps working and explains what it needs, while typos
stay visible.

## Requirements

### Requirement: Unknown top-level and `mcp` keys warn instead of failing

When `_codev/config.yaml` holds a top-level key, or a key of the `mcp`
section, that the running codev does not know, every command that reads
the configuration SHALL ignore that key, use the rest of the configuration,
and report one warning with the code `unknown_config_key` per unknown key.
The warning MUST name the key (`mcp.<key>` for a key of `mcp`), say that the
project may need a newer codev, quote the project's `codev_version`
requirement when the file sets one, and point to `codev self-update`.
Commands that report configuration warnings, `codev status` among them,
MUST report it in their human output and in the `status` array of their
JSON output. The keys of an inherited source's configuration MUST NOT
produce this warning: only its context and rules are read.

#### Scenario: Unknown top-level key with a version requirement

- **GIVEN** a project whose `_codev/config.yaml` holds
  `codev_version: ">=0.9"` and the top-level key `review_policy`, unknown
  to the running codev, with an active change
- **WHEN** the user runs `codev status --json`
- **THEN** the command succeeds and reports the change's status
- **AND** its `status` array holds a warning with the code
  `unknown_config_key` whose message names `review_policy`, quotes
  `>=0.9` and names `codev self-update`

#### Scenario: Unknown top-level key without a version requirement

- **GIVEN** a project whose `_codev/config.yaml` holds `shema: spec-driven`
  and no `codev_version`, with an active change
- **WHEN** the user runs `codev status`
- **THEN** the command succeeds with the default schema
- **AND** a warning `unknown_config_key` names `shema` and says the project
  may need a newer codev

#### Scenario: Unknown key of the `mcp` section

- **GIVEN** a project whose `_codev/config.yaml` holds
  `mcp: {jira_tool: mcp__a__getJiraIssue, design_tool: mcp__b__get}`
- **WHEN** the user runs `codev update`
- **THEN** the skills are installed with the Jira tool `mcp__a__getJiraIssue`
- **AND** a warning `unknown_config_key` names `mcp.design_tool`

#### Scenario: Unknown key of an inherited source

- **GIVEN** a project inheriting from a local source whose
  `_codev/config.yaml` holds the unknown key `review_policy` and a context
- **WHEN** the user runs `codev status`
- **THEN** the source's context is used
- **AND** no `unknown_config_key` warning is reported

### Requirement: Unknown keys of `inherits` entries and `changelog` fail the command

An unknown key inside an `inherits` entry or inside the `changelog` section
of `_codev/config.yaml` MUST make every command that reads the
configuration fail with the code `invalid`, naming the key, because
ignoring it would silently change which source is read or what
`codev changelog --write` writes. When the file sets `codev_version`, the
error message MUST also quote the requirement and point to
`codev self-update`.

#### Scenario: Unknown key of the `changelog` section

- **GIVEN** a project whose `_codev/config.yaml` holds
  `codev_version: ">=0.9"` and `changelog: {since: v1.0.0}`
- **WHEN** the user runs `codev status`
- **THEN** the command fails with the code `invalid`
- **AND** the message names `since`, quotes `>=0.9` and names
  `codev self-update`

#### Scenario: Unknown key of an `inherits` entry

- **GIVEN** a project whose `_codev/config.yaml` declares
  `inherits: [{path: ~/shared, pin: abc}]` and no `codev_version`
- **WHEN** the user runs `codev status`
- **THEN** the command fails with the code `invalid` naming `pin`

### Requirement: `codev validate` reports unknown configuration keys

When it validates the whole project — no scope, or `--all` — and
`_codev/config.yaml` holds unknown top-level or `mcp` keys, `codev validate`
SHALL add to its report an item of kind `config`, named `config`, whose path
is `_codev/config.yaml`, carrying one finding of severity `warning` and
code `unknown_config_key` per unknown key, at the line of that key. The
item MUST be absent when the configuration holds no unknown key, and from
the reports of `--changes`, `--specs` and of a named item. Like any warning,
it fails `codev validate --strict` and not `codev validate`.

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

- **GIVEN** a project whose `_codev/config.yaml` holds only known keys
- **WHEN** the user runs `codev validate --all --json`
- **THEN** no item of kind `config` is reported

### Requirement: `review` is a known configuration key

`review` SHALL be a known top-level key of `_codev/config.yaml`: it never
produces an `unknown_config_key` warning. A key under `review:` other
than `mode` SHALL be ignored with an `unknown_config_key` warning naming
it `review.<key>`, as a key of `mcp` is, and `codev validate` and
`codev doctor` report it the same way.

#### Scenario: No warning for `review`

- **GIVEN** a configuration with `review: { mode: isolated }`
- **WHEN** the user runs `codev status --json`
- **THEN** `status` holds no `unknown_config_key` warning

#### Scenario: An unknown key under `review`

- **GIVEN** a configuration with `review: { mode: inline, budget: 3 }`
- **WHEN** the user runs `codev status --json`
- **THEN** the exit code is 0 and `status` holds an `unknown_config_key`
  warning naming `review.budget`
