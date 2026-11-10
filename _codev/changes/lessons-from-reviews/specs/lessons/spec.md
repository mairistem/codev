## Purpose

The lessons ledger keeps, for a project, every lesson the `/codev-configure`
lessons mode has considered — where it went, or why it went nowhere —
with the archived findings and corrections it comes from, so that a lesson
is never proposed twice and its origin stays traceable.

## ADDED Requirements

### Requirement: The lessons ledger has a layout both humans and codev read

A project's lessons ledger SHALL be the file `_codev/lessons.md`. Its
layout:

- an optional first line `# Lessons`, then the format marker
  `<!-- codev-lessons: <n> -->`, required: a file without it is not a
  ledger, and codev neither checks nor reads it;
- one entry per pass of the lessons mode: a level-3 heading
  `### P-<n> <title>`, followed by one fenced `yaml` block holding
  `read`, the non-empty list of the archived change folders the pass
  read;
- one entry per lesson: a level-3 heading `### L-<n> <title>`, followed by
  one fenced `yaml` block holding:
  - `outcome`, one of `written` (written to its destination),
    `handed-off` (an automatable check left to a change to propose or to
    an active change), `covered` (a decision, rule, context line or
    active change already holds it), `declined` (the author refused it)
    or `set-aside` (not a lesson for the project);
  - `destination`, one of `check`, `decision`, `rule:specs`,
    `rule:design`, `rule:tasks`, `context` or `none`;
  - `where`, what holds the lesson — a decision id, a configuration key,
    a change name — required for `written`, `handed-off` and `covered`;
  - `origin`, a non-empty list of `<archived change folder>/<id>`, the
    folder a single path segment (no separator, no `.` or `..`), each
    id a finding id (`R-<role>-<n>`) or a correction id (`C-<n>`);
  - `reason`, required for `declined` and `set-aside`;
  - `pass`, the `P-<n>` entry of the pass that considered it.

The identifiers of each kind are positive integers unique in the file.
Other lines are free text. Line endings may be `\n` or `\r\n`.

#### Scenario: A well-formed ledger

- **GIVEN** a `_codev/lessons.md` starting with
  `<!-- codev-lessons: 1 -->`, holding `### P-1 First pass` whose
  `read` lists ten archived change folders, and
  `### L-1 Paths read from the project stay inside it` with
  `outcome: written`, `destination: decision`, `where: "0013"`, two
  origins and `pass: P-1`
- **WHEN** the user runs `codev validate`
- **THEN** no `lessons_*` finding is reported

#### Scenario: Windows line endings

- **GIVEN** the same file saved with `\r\n` line endings
- **WHEN** the user runs `codev validate`
- **THEN** no `lessons_*` finding is reported

#### Scenario: A file without the marker is not read

- **GIVEN** a `_codev/lessons.md` holding free notes and no
  `<!-- codev-lessons: 1 -->` marker
- **WHEN** the user runs `codev validate` and `codev review --archived
  --json`
- **THEN** no `lessons_*` finding is reported and no served entry is
  marked as considered

### Requirement: The entries of the lessons ledger are checked

When `_codev/lessons.md` holds the format marker, validation SHALL report,
each at its line:

- as the error `lessons_invalid`: an entry heading that is neither
  `### L-<n> <title>` nor `### P-<n> <title>`, an identifier used twice,
  an entry without its `yaml` block, a block that is not valid YAML, a
  missing or empty field, a field of the wrong type, an `outcome` or a
  `destination` outside its values, a `where` missing for an outcome that
  requires it, a `reason` missing for `declined` or `set-aside`, an
  `origin` item not of the form `<folder>/R-<role>-<n>` or
  `<folder>/C-<n>` with a single-segment folder, a `read` item that is
  not a single path segment, a `pass` naming no `P-<n>` entry of the
  file;
- as the warning `lessons_unknown_field`: a field the format does not
  know, which is otherwise ignored;
- as the warning `lessons_origin_unknown`: an `origin` item or a `read`
  item whose folder is not a change of `_codev/changes/archive/`.

A project without `_codev/lessons.md` gets no lessons finding.

#### Scenario: A declined lesson without its reason

- **GIVEN** a ledger entry `L-2` with `outcome: declined` and no `reason`
- **WHEN** the user runs `codev validate`
- **THEN** a `lessons_invalid` error at the entry's line names `reason`

#### Scenario: An unknown destination

- **GIVEN** a ledger entry with `destination: wiki`
- **WHEN** the user runs `codev validate`
- **THEN** a `lessons_invalid` error names `wiki` and the seven accepted
  destinations

#### Scenario: A malformed origin

- **GIVEN** a ledger entry whose `origin` holds `review-record R-security-3`
- **WHEN** the user runs `codev validate`
- **THEN** a `lessons_invalid` error names that item and the expected
  form

#### Scenario: An origin that leaves the archive

- **GIVEN** a ledger entry whose `origin` holds `../../secrets/R-skeptic-1`
- **WHEN** the user runs `codev validate`
- **THEN** a `lessons_invalid` error names that item, and no file outside
  `_codev/changes/archive/` is read for it

#### Scenario: An origin from a change that is not archived

- **GIVEN** a ledger entry whose `origin` holds
  `2026-01-01-never-archived/R-skeptic-1`
- **WHEN** the user runs `codev validate`
- **THEN** a `lessons_origin_unknown` warning names
  `2026-01-01-never-archived`, and no error

#### Scenario: A field the format does not know

- **GIVEN** a well-formed lesson entry that also holds `owner: platform`
- **WHEN** the user runs `codev validate`
- **THEN** a `lessons_unknown_field` warning names `owner`, and no error

### Requirement: The lessons ledger is read by future versions and older ones keep working

A ledger whose format marker is greater than the format this codev reads
SHALL NOT be checked or read: validation reports the warning
`lessons_format_unsupported` naming the format, `codev review --archived`
marks no entry as considered and lists the same warning, and `codev
archive --json` gives `lessons` as `null`. A codev that predates the
ledger ignores `_codev/lessons.md`, since it reads no file of `_codev/`
it does not know; that cannot be tested from this codev, and the design
records why it holds.

#### Scenario: A ledger from a newer codev

- **GIVEN** a `_codev/lessons.md` starting with `<!-- codev-lessons: 2 -->`
- **WHEN** the user runs `codev validate`
- **THEN** a `lessons_format_unsupported` warning is reported and no
  other lessons finding

### Requirement: `codev archive` tells whether lessons may be waiting

After a successful archive, `codev archive --json` SHALL carry, in its
`ArchiveReportV1` document, an additive object `lessons` with:

- `changes`: the number of archived changes that no pass of the ledger
  lists in its `read` — every archived change without a ledger or
  without a pass —, whatever they hold;
- `corrections`: the number of corrections of those changes that no
  lesson's `origin` names;
- `window`: the smallest `n` such that the last `n` archived changes,
  ordered by folder name, include every change counted in `changes`:
  the window a pass needs to read all of them;
- `waiting`: `true` when `changes` is at least 10 or `corrections` is at
  least 1, `false` otherwise.

Only the corrections files of the changes counted are read; no review
record is read for `lessons`. `lessons` SHALL be `null`
when the ledger cannot be read (a newer format or a `lessons_invalid`
error). The human output of `codev archive` SHALL NOT change. Computing
`lessons` SHALL never make the archive fail.

#### Scenario: Ten changes since the last pass

- **GIVEN** a ledger whose passes read every archived change but the ten
  newest, the tenth being the change just archived
- **WHEN** the user runs `codev archive --change <change> --json`
- **THEN** `lessons.changes` is 10, `lessons.window` is 10 and
  `lessons.waiting` is `true`

#### Scenario: Changes archived the same day, in any name order

- **GIVEN** a ledger whose pass read `2026-10-26-zeta`, and
  `2026-10-26-alpha` archived later that day with a correction `C-1`
- **WHEN** the user archives `alpha` with `--json`
- **THEN** `lessons.changes` is 1, `lessons.corrections` is 1,
  `lessons.window` is 2 and `lessons.waiting` is `true`

#### Scenario: A correction not yet considered

- **GIVEN** a project without `_codev/lessons.md`, and a change whose
  `corrections.md` holds `C-1`
- **WHEN** the user archives it with `codev archive --change <change>
  --json`
- **THEN** `lessons.corrections` is 1 and `lessons.waiting` is `true`

#### Scenario: Nothing waiting

- **GIVEN** a ledger whose passes read every archived change, and a
  change without corrections
- **WHEN** the user archives it with `--json`
- **THEN** `lessons.changes` is 1, `lessons.corrections` is 0 and
  `lessons.waiting` is `false`

#### Scenario: An unreadable ledger

- **GIVEN** a ledger starting with `<!-- codev-lessons: 2 -->`
- **WHEN** the user archives a change with `--json`
- **THEN** the archive succeeds and `lessons` is `null`
