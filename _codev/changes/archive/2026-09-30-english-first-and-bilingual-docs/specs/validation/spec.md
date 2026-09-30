## ADDED Requirements

### Requirement: A delta is checked against its main spec

For each delta of a change, validation SHALL read the main spec it targets
and report as an error `rename_source_missing` every `RENAMED` whose `FROM`
names no requirement of that spec — unless the `TO` requirement already
exists there, which means an earlier sync already applied the rename.

#### Scenario: Rename of an unknown requirement

- **GIVEN** a main spec without any requirement named `Remember Me`
- **AND** a delta renaming `Remember Me` to `Stay Signed In`
- **WHEN** the user runs `codev validate <change>`
- **THEN** an error `rename_source_missing` is reported
- **AND** the exit code is non-zero

#### Scenario: Rename already applied by a sync

- **GIVEN** a delta renaming `Remember Me` to `Stay Signed In`
- **AND** a main spec that already contains `Stay Signed In` and no longer
  contains `Remember Me`
- **WHEN** the user runs `codev validate <change>`
- **THEN** no `rename_source_missing` is reported
