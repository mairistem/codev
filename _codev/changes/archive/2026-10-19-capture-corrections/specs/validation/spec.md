## ADDED Requirements

### Requirement: The corrections of an active change are checked

For each active change whose `corrections.md` exists, validation SHALL
report, located at the file's lines, the findings the `corrections`
capability defines: `corrections_invalid` as an error;
`corrections_unknown_field`, `corrections_too_long` and
`corrections_format_unsupported` as warnings. With `--strict`, those
warnings fail validation as every other warning does. Archived changes
MUST NOT be checked.

#### Scenario: A change without corrections

- **GIVEN** an active change without `corrections.md`
- **WHEN** the user runs `codev validate <change>`
- **THEN** no `corrections_*` finding is reported

#### Scenario: A corrections error fails validation

- **GIVEN** an active change whose `corrections.md` holds an entry
  without `did`
- **WHEN** the user runs `codev validate <change> --json`
- **THEN** a `corrections_invalid` error is reported with the path of
  `corrections.md` and its line
- **AND** the exit code is 1

#### Scenario: A warning under strict mode

- **GIVEN** an active change whose only finding is a
  `corrections_unknown_field` warning
- **WHEN** the user runs `codev validate <change> --strict`
- **THEN** the exit code is 1
