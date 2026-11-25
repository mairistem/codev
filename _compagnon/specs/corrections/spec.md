# Corrections Specification

## Purpose

The corrections file keeps, in a change, the instructions the author gave
the agent about how the project does things, so that they outlive the
session and can later become rules, decisions or checks.

## Requirements

### Requirement: The corrections file has a layout both humans and codev read

A change's corrections SHALL live in the file `corrections.md` of the
change's directory. Its layout:

- an optional first line `# Corrections: <change>`, then the format
  marker `<!-- codev-corrections: <n> -->`, required: a file without it is
  not a corrections file, and codev neither checks nor lists it, so that a
  file of the same name the project already keeps is left alone;
- one entry per correction: a level-3 heading `### C-<n> <title>`, `<n>`
  a positive integer unique in the file, followed by one fenced `yaml`
  block;
- the block holds four fields, each a string: `step`, one of `propose`,
  `apply` or `update`; `where`, the file or artifact concerned; `did`,
  what the agent had done; `asked`, what the author asked instead.

Other lines are free text. Line endings may be `\n` or `\r\n`.

#### Scenario: A well-formed corrections file

- **GIVEN** an active change whose `corrections.md` starts with the
  marker `<!-- codev-corrections: 1 -->` and holds an entry
  `### C-1 Typed errors in the libraries` with `step: apply`,
  `where: crates/codev-engine/src/search.rs`, `did` and `asked`
- **WHEN** the user runs `codev validate <change>`
- **THEN** no `corrections_*` finding is reported

#### Scenario: Windows line endings

- **GIVEN** the same file saved with `\r\n` line endings
- **WHEN** the user runs `codev validate <change>`
- **THEN** no `corrections_*` finding is reported

#### Scenario: A file without the marker is not read

- **GIVEN** an active change whose `corrections.md` holds free notes with
  `### ` headings and no `<!-- codev-corrections: 1 -->` marker
- **WHEN** the user runs `codev validate <change>` and `codev review
  --change <change> --json`
- **THEN** no `corrections_*` finding is reported and `corrections` is
  an empty array

### Requirement: The entries of the corrections file are checked

For each active change whose `corrections.md` exists, validation SHALL
report, each at its line:

- as the error `corrections_invalid`: an entry heading that is not
  `### C-<n> <title>`, an identifier used twice, an entry without its
  `yaml` block, a block that is not valid YAML, a missing or empty field,
  a field that is not a string, a `step` other than `propose`, `apply`
  or `update`;
- as the warning `corrections_unknown_field`: a field the format does not
  know, which is otherwise ignored;
- as the warning `corrections_too_long`: a `did` or `asked` longer than
  300 characters, since an entry states an instruction and never
  transcribes the conversation.

A change without `corrections.md` gets no corrections finding. Archived
changes MUST NOT be checked.

#### Scenario: A missing field

- **GIVEN** an entry whose block has no `asked`
- **WHEN** the user runs `codev validate <change>`
- **THEN** a `corrections_invalid` error at the entry's line names
  `asked`

#### Scenario: An unknown step

- **GIVEN** an entry with `step: archive`
- **WHEN** the user runs `codev validate <change>`
- **THEN** a `corrections_invalid` error names the three accepted steps

#### Scenario: Two entries with one identifier

- **GIVEN** two entries headed `### C-1`
- **WHEN** the user runs `codev validate <change>`
- **THEN** a `corrections_invalid` error at the second one says the
  identifier is already used

#### Scenario: A field the format does not know

- **GIVEN** a well-formed entry that also holds `category: style`
- **WHEN** the user runs `codev validate <change>`
- **THEN** a `corrections_unknown_field` warning names `category`, and no
  error

#### Scenario: A transcript instead of an instruction

- **GIVEN** an entry whose `asked` holds 450 characters
- **WHEN** the user runs `codev validate <change>`
- **THEN** a `corrections_too_long` warning is reported at the entry

#### Scenario: Archived corrections are not checked

- **GIVEN** a project whose only invalid `corrections.md` is in an
  archived change
- **WHEN** the user runs `codev validate`
- **THEN** no `corrections_*` finding is reported

### Requirement: The corrections file is read by future versions and older ones keep working

A file whose format marker is greater than the format this codev reads
SHALL NOT be checked: validation reports the warning
`corrections_format_unsupported` naming the format, and `codev review`
lists no entry from it. A codev that predates the corrections file
ignores `corrections.md`; that cannot be tested from this codev, and the
design records why it holds.

#### Scenario: A file from a newer codev

- **GIVEN** a `corrections.md` starting with `<!-- codev-corrections: 2 -->`
- **WHEN** the user runs `codev validate <change>`
- **THEN** a `corrections_format_unsupported` warning is reported and no
  other corrections finding

### Requirement: The corrections file is not a planning artifact

`corrections.md` SHALL NOT change the state `codev status` reports for the
change's artifacts, `codev sync` SHALL leave it in place, and `codev
changelog` SHALL NOT read it. `codev archive` moves it with the change.
What the entries say never blocks the archive; a file with a
`corrections_invalid` error blocks it as every validation error does.

#### Scenario: Status ignores the corrections

- **GIVEN** a change with a `corrections.md` and no other artifact
- **WHEN** the user runs `codev status --json`
- **THEN** the artifacts' states are those of a change without
  `corrections.md`

#### Scenario: The archive keeps the corrections

- **GIVEN** a change ready to be archived, with a `corrections.md`
- **WHEN** the user runs `codev archive`
- **THEN** `corrections.md` is in the dated archive folder

#### Scenario: Sync leaves the corrections in place

- **GIVEN** a change with delta specs and a `corrections.md`
- **WHEN** the user runs `codev sync <change>`
- **THEN** `corrections.md` is still in the change's folder

#### Scenario: An invalid corrections file blocks the archive

- **GIVEN** a change ready to be archived whose `corrections.md` holds an
  entry without `asked`
- **WHEN** the user runs `codev archive`
- **THEN** the archive is refused as for any validation error, and
  `codev validate <change>` names `corrections.md`

#### Scenario: The changelog ignores the corrections

- **GIVEN** an archived change whose folder holds a `corrections.md`
- **WHEN** the user runs `codev changelog`
- **THEN** the release notes are those of its proposal alone
