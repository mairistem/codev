# Review Record Specification

## Purpose

Keep, for each change, a record of what the reviewing roles found — with a
severity, the evidence and the answer each finding received, and who
decided it — that codev checks deterministically and that gates archiving.

## Requirements

### Requirement: Findings are graded on a shared severity and confidence scale

Every finding SHALL carry a severity among:

- `high` — The change would break a requirement, lose or expose data, or
  open a vulnerability, and must not be archived until it is fixed or a
  human accepts the risk.
- `medium` — A real defect or risk with a limited impact or a workaround,
  to fix in this change or to defer deliberately.
- `low` — An improvement, such as readability, consistency or a minor
  cost, that does not decide whether the change is correct.

and a confidence among:

- `high` — The evidence shows the problem directly.
- `low` — The problem depends on something the reviewer could not see or
  verify.

A finding with a low confidence is kept in the record and answered like
any other, but is not presented to the human unless a decision is asked
of them. The scales MUST be the same in the skills, the documentation and
the checks; any other value is a `review_invalid` error.

#### Scenario: The scales reach the skills

- **GIVEN** the codev workflow catalog
- **WHEN** the `propose` and `apply` skills are rendered
- **THEN** each carries the three severities and the two confidences with
  their definitions

#### Scenario: An unknown severity

- **GIVEN** a review record with a finding whose severity is `critical`
- **WHEN** the user runs `codev validate <change>`
- **THEN** a `review_invalid` error names `critical` and the valid values

### Requirement: The review record has a layout both humans and codev read

A change's review record SHALL be the file `review.md` of the change's
directory. Its layout:

- an optional first line `# Review record: <change>`, and an optional
  format marker `<!-- codev-review: <n> -->`; a record without a marker is
  format 1;
- one level-2 heading per step, `## plan.challenge` or `## code.review`;
- under a step, one level-3 heading per role, `### <role-id>`;
- under a role, either the line `No finding.`, or the line
  `Disabled by configuration.`, or one or more findings;
- a finding is a level-4 heading `#### <id> <title>` followed by one
  fenced `yaml` block holding its fields.

Other lines are free text, ignored by the checks. Line endings may be
`\n` or `\r\n`. Validation SHALL report as `review_invalid` errors, each at
its line: a finding outside a role, a role outside a step, a step or a
role written twice, a role with neither a finding nor one of the two
lines, a finding whose block is missing or is not valid YAML. A step other
than the two above, or a role the change's catalog does not know, is the
warning `review_unknown_section`, and is otherwise ignored.

#### Scenario: A well-formed record

- **GIVEN** an active change whose `review.md` has a `## plan.challenge`
  step with `### skeptic` saying `No finding.` and `### security` holding
  one well-formed finding
- **WHEN** the user runs `codev validate <change>`
- **THEN** no review finding is reported

#### Scenario: A role section with nothing in it

- **GIVEN** a record whose `### skeptic` section holds only free text
- **WHEN** the user runs `codev validate <change>`
- **THEN** a `review_invalid` error at the line of `### skeptic` says to
  write `No finding.` when the role found nothing

#### Scenario: A finding block that is not YAML

- **GIVEN** a record whose finding block cannot be parsed as YAML
- **WHEN** the user runs `codev validate <change>`
- **THEN** a `review_invalid` error is reported at the line of the block

#### Scenario: A misspelled step

- **GIVEN** a record with a `## code.reveiw` section
- **WHEN** the user runs `codev validate <change>`
- **THEN** a `review_unknown_section` warning names `code.reveiw`

#### Scenario: Windows line endings

- **GIVEN** a well-formed record saved with `\r\n` line endings
- **WHEN** the user runs `codev validate <change>`
- **THEN** no review finding is reported

### Requirement: A finding states where, what and why, with a verbatim quote

A finding's block SHALL hold `severity`, `confidence`, `location`,
`evidence`, `recommendation` and `history`. Its id SHALL be
`R-<role-id>-<n>`, where `<role-id>` is the role of its section and `<n>` a
positive integer, unique in the record: an id, once written, is never
reused or renumbered. The `location` is either `<path>:<line>` or
`<path>:<start>-<end>`, a path relative to the project root, for code; or
`<file>#<heading>`, a file relative to the change's directory and the text
of one of its headings, for the plan. `\` is read as `/`; the path is then
judged as text, the same on every operating system: it MUST NOT start
with `/`, its first segment MUST NOT hold `:` (a drive or a scheme), and
no segment may be empty, `.` or `..`. The `evidence` is the cited text,
copied verbatim — whole lines or a fragment of each — never empty.

Validation SHALL report as `review_invalid` errors a missing or empty
field, a malformed id or location, and a duplicate id; and as the warning
`review_unknown_field` a field it does not know, which is otherwise
ignored.

#### Scenario: Both kinds of location

- **GIVEN** a finding located at `src/db.rs:12-14` and another at
  `specs/auth/spec.md#Requirement: Login`, each with its fields
- **WHEN** the user runs `codev validate <change>`
- **THEN** no `review_invalid` error is reported for either

#### Scenario: A missing evidence

- **GIVEN** a finding without `evidence`
- **WHEN** the user runs `codev validate <change>`
- **THEN** a `review_invalid` error names the finding and `evidence`

#### Scenario: An id that does not match its role

- **GIVEN** a finding `R-security-1` under `### privacy`
- **WHEN** the user runs `codev validate <change>`
- **THEN** a `review_invalid` error says the id must start with
  `R-privacy-`

#### Scenario: Two findings with the same id

- **GIVEN** two findings `R-security-1`, one under each step
- **WHEN** the user runs `codev validate <change>`
- **THEN** a `review_invalid` error is reported at the second one

#### Scenario: A location outside the project

- **GIVEN** a finding whose location is `../secrets.txt:1`, and another
  at `C:\secrets.txt:1`
- **WHEN** the user runs `codev validate <change>`
- **THEN** a `review_invalid` error is reported for each
- **AND** neither file is read

#### Scenario: A field from a newer codev

- **GIVEN** a finding with an extra field `category: injection`
- **WHEN** the user runs `codev validate <change>`
- **THEN** a `review_unknown_field` warning names `category`

### Requirement: Each finding keeps the history of its answers and who gave them

A finding's `history` SHALL be a list of entries, each with a `status` and
a `by` — a role id, `human`, or `human:<name>` — oldest first; the
finding's status is its last entry's. The first entry MUST be `open` by
the finding's role. The allowed transitions, and what each entry requires:

| From | To | Requires |
|---|---|---|
| `open` | `fixed` | `ref`: the commit or the file change |
| `open`, `disputed` | `disputed` | `note`: the argument |
| `open`, `disputed` | `accepted-risk` | `note`: the justification, and `by` a human |
| `open`, `disputed` | `deferred` | `to`: the change or the issue it goes to, and `by` a human when the severity is `high` |
| `disputed` | `rejected` | `by` the finding's role or a human |
| `disputed` | `fixed` | `ref` |
| `fixed` | `fixed` | `by` the finding's role: the fix is verified |
| `fixed` | `open` | `by` the finding's role, and `note`: why the fix does not hold |

Validation SHALL report an empty history, a first entry that is not
`open` by the finding's role, an unknown status and a transition outside
the table as `review_invalid` errors; and a missing requirement of an
entry as the error `review_decision_incomplete`.

#### Scenario: An accepted risk decided by the author

- **GIVEN** a finding whose last entry is `accepted-risk` by `developer`
  with a note
- **WHEN** the user runs `codev validate <change>`
- **THEN** a `review_decision_incomplete` error says an accepted risk is
  decided by a human

#### Scenario: A fix without a reference

- **GIVEN** a finding whose last entry is `fixed` with no `ref`
- **WHEN** the user runs `codev validate <change>`
- **THEN** a `review_decision_incomplete` error names `ref`

#### Scenario: A high finding deferred by the author

- **GIVEN** a `high` finding deferred by `developer` to `harden-sessions`
- **WHEN** the user runs `codev validate <change>`
- **THEN** a `review_decision_incomplete` error says a high finding is
  deferred by a human

#### Scenario: The author rejects a finding

- **GIVEN** a finding whose history goes from `disputed` by `developer` to
  `rejected` by `developer`
- **WHEN** the user runs `codev validate <change>`
- **THEN** a `review_invalid` error says only the reviewer or a human
  rejects a finding

#### Scenario: A complete loop

- **GIVEN** a finding `open` by `security`, `disputed` by `developer` with
  an argument, then `rejected` by `security`
- **WHEN** the user runs `codev validate <change>`
- **THEN** no review finding is reported

### Requirement: The evidence of an unresolved finding is found in the current tree

For each finding whose status is `open`, `disputed`, `accepted-risk` or
`deferred`, validation SHALL check the location against the files as they
are now, without git and without the network:

- a code location: the file exists, the cited lines exist, and each of
  the evidence's non-blank lines is contained in one of consecutive
  non-blank lines of the file, in order, comparing lines with their
  surrounding whitespace trimmed, runs of whitespace reduced to one space
  and `\r` ignored; the match MUST start no more than 5 lines before the
  cited start and no more than 5 lines after the cited end;
- a plan location: the file exists in the change's directory, holds a
  heading with that text (case-insensitive), and the evidence appears in
  that heading's section.

A missing file, line or heading is the error `review_location_missing`.
Evidence found nowhere is the error `review_evidence_not_found`. Evidence
found only outside the tolerance is the warning `review_evidence_moved`,
naming the line where it is. A file that is not UTF-8 text, holds a NUL
byte, or is larger than 4 MiB is not compared: the warning
`review_evidence_unchecked` says so. A `fixed` or `rejected` finding is not checked: its code is expected
to have changed.

#### Scenario: Evidence at the cited lines

- **GIVEN** an `open` finding located at `src/db.rs:12-13` whose evidence
  is the text of line 12, indented differently, and the start of line 13
- **WHEN** the user runs `codev validate <change>`
- **THEN** no review finding is reported

#### Scenario: Evidence a few lines away

- **GIVEN** an `open` finding located at `src/db.rs:12` whose evidence is
  now at line 15
- **WHEN** the user runs `codev validate <change>`
- **THEN** no review finding is reported

#### Scenario: Evidence that moved further

- **GIVEN** an `open` finding located at `src/db.rs:12` whose evidence is
  now at line 40
- **WHEN** the user runs `codev validate <change>`
- **THEN** a `review_evidence_moved` warning names line 40

#### Scenario: Invented evidence

- **GIVEN** a `disputed` finding whose evidence appears nowhere in the
  cited file
- **WHEN** the user runs `codev validate <change>`
- **THEN** a `review_evidence_not_found` error is reported
- **AND** the exit code is 1

#### Scenario: A line beyond the end of the file

- **GIVEN** an `open` finding located at `src/db.rs:400` in a file of 50
  lines
- **WHEN** the user runs `codev validate <change>`
- **THEN** a `review_location_missing` error is reported

#### Scenario: A plan finding in its section

- **GIVEN** an `open` finding located at
  `design.md#Decision: Tokens are stored` whose evidence is a sentence of
  that decision
- **WHEN** the user runs `codev validate <change>`
- **THEN** no review finding is reported

#### Scenario: A binary file

- **GIVEN** an `open` finding located in a file holding NUL bytes
- **WHEN** the user runs `codev validate <change>`
- **THEN** a `review_evidence_unchecked` warning is reported

#### Scenario: A fixed finding whose code is gone

- **GIVEN** a `fixed` finding whose cited file was deleted by the fix
- **WHEN** the user runs `codev validate <change>`
- **THEN** no review finding is reported

### Requirement: Every role active at a recorded step has its section

When the record has a step's section, validation SHALL report the error
`review_role_missing` for each role `codev roles --at <step> --change
<change>` returns that has no section under it, or whose section says
`Disabled by configuration.`. A role inactive at the step may have a
section or none.

#### Scenario: An active expert without a section

- **GIVEN** a change whose proposal ticks `security`, and whose record's
  `## plan.challenge` holds only `### skeptic`
- **WHEN** the user runs `codev validate <change>`
- **THEN** a `review_role_missing` error names `security` and
  `plan.challenge`

#### Scenario: A role disabled by configuration

- **GIVEN** a configuration with `roles: { newcomer: never }` and a record
  whose `## code.review` holds `### newcomer` saying
  `Disabled by configuration.`
- **WHEN** the user runs `codev validate <change>`
- **THEN** no review finding is reported

### Requirement: `codev review` shows the record and what is left to decide

`codev review [--change <name>] [--json]` SHALL read the change's record
and print, per step and per role, the findings with their severity,
status and location, and, for each finding, who must act next: the
author for an `open` finding, the reviewer for a `fixed` finding not yet
verified or a `disputed` one the reviewer has not answered, a human for a
dispute the reviewer maintained. It SHALL say which findings are
reported to the human (confidence `high`), list the record's defects as
`codev validate` does, and say whether the record lets the change be
archived, naming each reason it does not. The change is picked as by the
other commands taking `--change`. Without a record, the command succeeds
and says so. The exit code is 1 when the record has an error, 0
otherwise. With `--json`, it prints a `ReviewV1` document.

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

### Requirement: `codev archive` refuses a change whose review is not settled

Before moving a change, and after the validation pre-flight, `codev
archive` SHALL refuse with the stable code `review_blocked`, writing and
moving nothing, when, for `plan.challenge` or `code.review`, at least one
role is active for the change and the record does not exist
(`review_record_missing`) or has no section for that step
(`review_step_missing`); or when a `high` finding's status is `open` or
`disputed` (`review_unresolved`); or when the record's format is newer
than this codev reads (`review_format_unsupported`). The message names
each reason and points to `codev review`. A step with no active role
needs no section. `codev sync` is not affected.

#### Scenario: A high finding still open

- **GIVEN** a change whose record holds a `high` finding `open`
- **WHEN** the user runs `codev archive --change <change> --json`
- **THEN** the command fails with the code `review_blocked`
- **AND** the message names the finding and `codev review`
- **AND** the change is still in `_codev/changes/`

#### Scenario: An accepted risk lets the change go

- **GIVEN** a change whose only `high` finding is `accepted-risk` by
  `human` with a justification, and whose record has both steps
- **WHEN** the user runs `codev archive`
- **THEN** the change is archived with its `review.md`

#### Scenario: No record for a reviewed step

- **GIVEN** a change whose roles are active at `code.review` and that has
  no `review.md`
- **WHEN** the user runs `codev archive`
- **THEN** the command fails with `review_blocked`, naming
  `review_record_missing`

#### Scenario: Every review role turned off

- **GIVEN** a change whose `change.yaml` sets `skeptic` and `newcomer` to
  `never` and whose proposal ticks no domain, without `review.md`
- **WHEN** the user runs `codev archive`
- **THEN** the change is archived

#### Scenario: Sync does not look at the record

- **GIVEN** a change with a `high` finding `open`
- **WHEN** the user runs `codev sync`
- **THEN** the specs are merged

### Requirement: The record is read by future versions and older ones keep working

A record whose format marker is greater than the format this codev reads
SHALL NOT be checked: validation reports the warning
`review_format_unsupported` naming the format, and `codev archive` refuses
it, telling the user to update codev. A codev that predates the record
ignores `review.md`; that cannot be tested from this codev, and the
design records why it holds.

#### Scenario: A record from a newer codev

- **GIVEN** a record starting with `<!-- codev-review: 2 -->`
- **WHEN** the user runs `codev validate <change>`
- **THEN** a `review_format_unsupported` warning is reported and no other
  review finding

### Requirement: The record is not a planning artifact

The review record SHALL NOT change the state `codev status` reports for
the change's artifacts, and `codev sync` SHALL leave it in place. `codev
archive` moves it with the change; an archived record is never checked
again, and `codev changelog` does not read it.

#### Scenario: Status ignores the record

- **GIVEN** a change with a `review.md` and no other artifact
- **WHEN** the user runs `codev status --json`
- **THEN** the artifacts' states are those of a change without `review.md`

#### Scenario: The changelog ignores the record

- **GIVEN** an archived change whose folder holds a `review.md`
- **WHEN** the user runs `codev changelog`
- **THEN** the release notes are those of its proposal alone

#### Scenario: The archive keeps the record

- **GIVEN** a change whose review is settled
- **WHEN** the user runs `codev archive`
- **THEN** `review.md` is in the dated archive folder
