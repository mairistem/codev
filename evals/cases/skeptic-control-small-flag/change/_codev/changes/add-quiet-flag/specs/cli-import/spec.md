## Purpose

Lets scripts silence the CLI's progress output.

## ADDED Requirements

### Requirement: `--quiet` silences progress

`ledger import --quiet` SHALL print no progress line on standard error; errors SHALL still be printed, and the exit code SHALL be unchanged.

#### Scenario: A quiet import

- **WHEN** the user runs `ledger import --quiet data.csv` and the import succeeds
- **THEN** nothing is printed and the exit code is 0

#### Scenario: A quiet import that fails

- **WHEN** the user runs `ledger import --quiet missing.csv`
- **THEN** the error is printed on standard error and the exit code is 1
