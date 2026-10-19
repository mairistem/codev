## MODIFIED Requirements

### Requirement: `codev review` shows the record and what is left to decide

`codev review [--change <name>] [--json]` SHALL read the change's record
and print, per step and per role, the findings with their severity,
status and location, and, for each finding, who must act next: the
author for an `open` finding, the reviewer for a `fixed` finding not yet
verified or a `disputed` one the reviewer has not answered, a human for a
dispute the reviewer maintained. It SHALL say which findings are
reported to the human (confidence `high`), list the record's defects as
`codev validate` does, and say whether the record lets the change be
archived, naming each reason it does not. It SHALL also list the entries
of the change's `corrections.md`, each with its identifier, step,
location and what the author asked, under a section `Corrections`
printed only when there is one, followed by the corrections file's
defects; the corrections never decide whether the change can be archived,
their defects are never listed among the record's, and they do not
change the exit code, which `codev validate` already reports. The change
is picked as by the other commands taking `--change`. Without a record,
the command succeeds and says so. The exit code is 1 when the record has
an error, 0 otherwise. With `--json`, it prints a `ReviewV1` document,
whose `corrections` array holds one entry per correction with `id`,
`title`, `step`, `where`, `did` and `asked`, and whose
`correctionsIssues` array holds the corrections file's defects; both are
empty without a corrections file.

#### Scenario: A maintained dispute is for the human

- **GIVEN** a finding `disputed` by `developer`, then `disputed` by
  `security` with a note
- **WHEN** the user runs `codev review --json`
- **THEN** the finding's `awaiting` is `human`

#### Scenario: A low-confidence finding is not reported

- **GIVEN** a finding with confidence `low`
- **WHEN** the user runs `codev review --json`
- **THEN** the finding's `reported` is `false`

#### Scenario: No record yet

- **GIVEN** an active change without `review.md`
- **WHEN** the user runs `codev review --change <change> --json`
- **THEN** `exists` is `false`, `findings` is empty
- **AND** the exit code is 0

#### Scenario: A record with an error

- **GIVEN** a record with a `review_invalid` error
- **WHEN** the user runs `codev review`
- **THEN** the error is listed with its line
- **AND** the exit code is 1

#### Scenario: The corrections of a change

- **GIVEN** an active change whose `corrections.md` holds the entry
  `C-1` with `step: apply`, and no `review.md`
- **WHEN** the user runs `codev review --change <change> --json`
- **THEN** `corrections` holds `C-1` with its `step`, `where`, `did` and
  `asked`
- **AND** the human output prints a `Corrections` section with `C-1`

#### Scenario: Corrections do not block the archive

- **GIVEN** a change whose review is settled and whose `corrections.md`
  holds two entries
- **WHEN** the user runs `codev review`
- **THEN** it says the record lets the change be archived

#### Scenario: A corrections file with an error

- **GIVEN** an active change without `review.md` whose `corrections.md`
  holds an entry without `did`
- **WHEN** the user runs `codev review --change <change>`
- **THEN** the `corrections_invalid` error is listed with its line under
  `Corrections`, not under the record's issues, and `correctionsIssues`
  holds it with `--json`
- **AND** the exit code is 0

#### Scenario: Corrections from a newer codev

- **GIVEN** a `corrections.md` starting with `<!-- codev-corrections: 2 -->`
- **WHEN** the user runs `codev review --change <change> --json`
- **THEN** `corrections` is an empty array and `correctionsIssues` holds
  the `corrections_format_unsupported` warning

#### Scenario: A change without corrections

- **GIVEN** an active change without `corrections.md`
- **WHEN** the user runs `codev review --change <change>`
- **THEN** the output has no `Corrections` section, and `corrections` is
  an empty array with `--json`
