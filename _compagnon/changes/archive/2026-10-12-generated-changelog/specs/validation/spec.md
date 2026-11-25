## ADDED Requirements

### Requirement: The release note of an active change is checked

For each active change whose `proposal.md` exists, validation SHALL report
the warning `release_note_missing` when the proposal has no
`## Release note` section, and the error `release_note_invalid` — with the
line concerned — for a bullet whose category is not one of `Added`,
`Changed`, `Deprecated`, `Removed`, `Fixed`, `Security`, for a bullet with
no text, and for a section that holds neither bullets nor `None`. Archived
changes MUST NOT be checked: their history is not rewritten.

#### Scenario: Valid release note

- **GIVEN** an active change whose release note is
  `- Added: a flag.` and `- Changed: **BREAKING** a default.`
- **WHEN** the user runs `codev validate <change>`
- **THEN** no `release_note_*` finding is reported

#### Scenario: Internal change

- **GIVEN** an active change whose release note is `None`
- **WHEN** the user runs `codev validate <change>`
- **THEN** no `release_note_*` finding is reported

#### Scenario: Missing section

- **GIVEN** an active change whose proposal has no `## Release note`
  section
- **WHEN** the user runs `codev validate <change>`
- **THEN** a `release_note_missing` warning is reported
- **AND** the exit code is 0 without `--strict`

#### Scenario: Unknown category

- **GIVEN** an active change whose release note has `- Improved: speed.`
- **WHEN** the user runs `codev validate <change>`
- **THEN** a `release_note_invalid` error names `Improved` and its line
- **AND** the exit code is 1

#### Scenario: Empty bullet

- **GIVEN** an active change whose release note has `- Fixed:` with no text
- **WHEN** the user runs `codev validate <change>`
- **THEN** a `release_note_invalid` error is reported

#### Scenario: Archived changes are not checked

- **GIVEN** a project whose only change without a release note is archived
- **WHEN** the user runs `codev validate`
- **THEN** no `release_note_*` finding is reported
