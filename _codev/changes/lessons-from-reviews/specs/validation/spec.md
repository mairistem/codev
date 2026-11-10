## ADDED Requirements

### Requirement: The lessons ledger is checked

`codev validate` without an item, and with `--all`, SHALL check
`_codev/lessons.md` when it exists and report, located at the file's
lines, the findings the `lessons` capability defines:
`lessons_invalid` as an error; `lessons_unknown_field`,
`lessons_origin_unknown` and `lessons_format_unsupported` as warnings.
With `--strict`, those warnings fail validation as every other warning
does. The ledger SHALL appear as an item of the report only when it has a
finding, so that a project whose ledger is clean, or that has none, gets
the same report as before. A run naming a change or a spec, or limited
to `--changes` or `--specs`, SHALL NOT check the ledger.

#### Scenario: A ledger error fails validation

- **GIVEN** a ledger entry with `outcome: written` and no `where`
- **WHEN** the user runs `codev validate --json`
- **THEN** a `lessons_invalid` error is reported with the path
  `_codev/lessons.md` and its line
- **AND** the exit code is 1

#### Scenario: A clean ledger adds no item

- **GIVEN** a well-formed ledger
- **WHEN** the user runs `codev validate --json`
- **THEN** no item has the path `_codev/lessons.md`

#### Scenario: A warning under strict mode

- **GIVEN** a project whose only finding is a `lessons_origin_unknown`
  warning
- **WHEN** the user runs `codev validate --strict`
- **THEN** the exit code is 1

#### Scenario: Validating one change leaves the ledger alone

- **GIVEN** a ledger with a `lessons_invalid` error and a valid active
  change `add-auth`
- **WHEN** the user runs `codev validate add-auth`
- **THEN** no `lessons_*` finding is reported and the exit code is 0
