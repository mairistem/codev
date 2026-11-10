## ADDED Requirements

### Requirement: Findings that ask for opposed fixes are linked, and only a human settles them

A finding MAY carry `conflicts_with`, the list of the ids of the findings
of the same record whose fixes cannot be applied together with its own.
Validation SHALL report as `review_invalid`, at the finding's block, a
`conflicts_with` that is not a non-empty list of finding ids, that names
an id twice, that names the finding itself, that names an id no finding
of the record carries, or that names a finding which does not name this
one back: links are mutual. A record without `conflicts_with` stays
valid, and its findings are linked to none. Links may join findings of
different roles and of different steps.

A link between two findings is settled once either of them holds a
human's answer: an entry of its `history` by a human (`human` or
`human:<name>`), or a `conflict_decision`. A finding MAY carry
`conflict_decision`, a mapping with `by` and `note`, which records the
human's choice whatever the statuses of the linked findings. Validation
SHALL report as `review_invalid` a `conflict_decision` on a finding
without `conflicts_with`, or one that is not a mapping. It SHALL report
as `review_decision_incomplete` a `conflict_decision` whose `by` is not a
human or whose `note` is missing or empty.

A link is unsettled otherwise. It waits for the human when at least one
of its two findings has confidence `high`, and each of its two findings
then awaits a human, whatever its own status. A finding none of whose
links waits for the human awaits whom its own history says. A link
between two findings with confidence `low` does not wait for the human.

`codev review --json` SHALL give each finding `conflictsWith`, the ids
as written, or an empty list without the field. The human output SHALL
name, under a linked finding, the findings it conflicts with. The
record's format stays 1: a record that uses `conflicts_with` or
`conflict_decision` carries the marker `<!-- codev-review: 1 -->` and
draws no `review_format_unsupported` warning.

#### Scenario: Two opposed findings, linked both ways

- **GIVEN** a record whose `### performance` holds `R-performance-1`
  with `conflicts_with: [R-skeptic-2]`, and whose `### skeptic` holds
  `R-skeptic-2` with `conflicts_with: [R-performance-1]`, both `open`
  with confidence `high`
- **WHEN** the user runs `codev review --json`
- **THEN** `R-performance-1` has `conflictsWith: ["R-skeptic-2"]` and
  `R-skeptic-2` has `conflictsWith: ["R-performance-1"]`
- **AND** both have `awaiting: "human"`
- **AND** `codev validate <change>` reports no review finding

#### Scenario: A link that is not mutual

- **GIVEN** a record where `R-performance-1` has `conflicts_with:
  [R-skeptic-2]` and `R-skeptic-2` has no `conflicts_with`
- **WHEN** the user runs `codev validate <change>`
- **THEN** a `review_invalid` error at the block of `R-performance-1`
  says that `R-skeptic-2` does not list it back

#### Scenario: A link to a finding that is not in the record

- **GIVEN** a finding with `conflicts_with: [R-skeptic-9]`, and no
  finding `R-skeptic-9` in the record
- **WHEN** the user runs `codev validate <change>`
- **THEN** a `review_invalid` error at its block names `R-skeptic-9`

#### Scenario: A malformed link

- **GIVEN** a finding with `conflicts_with: R-performance-1`, a second
  with `conflicts_with: [R-skeptic-2, R-skeptic-2]`, and a third,
  `R-ops-1`, with `conflicts_with: [R-ops-1]`
- **WHEN** the user runs `codev validate <change>`
- **THEN** a `review_invalid` error is reported at each of the three
  blocks

#### Scenario: The author fixed one side and the other role gave way

- **GIVEN** linked findings `R-performance-1` and `R-skeptic-2`, both
  with confidence `high`, where `R-performance-1` is `fixed` by the
  author then by `performance`, and `R-skeptic-2` is `disputed` by the
  author then `rejected` by `skeptic`
- **WHEN** the user runs `codev review --json`
- **THEN** both findings have `awaiting: "human"`

#### Scenario: A medium finding deferred by the author alone

- **GIVEN** linked findings, both `medium` with confidence `high`, one
  `open` and the other `deferred` by the author to `later-change`
- **WHEN** the user runs `codev review --json`
- **THEN** both findings have `awaiting: "human"`

#### Scenario: The human chose

- **GIVEN** linked findings `R-performance-1`, `open`, and
  `R-skeptic-2`, `accepted-risk` by `human:Ludovic` with a note
- **WHEN** the user runs `codev review --json`
- **THEN** `R-performance-1` has `awaiting: "author"`, and
  `R-skeptic-2` has `awaiting: null`

#### Scenario: A human's choice on a finding that takes no more entries

- **GIVEN** linked findings `R-performance-1`, `fixed` by the author and
  verified by `performance`, and `R-skeptic-2`, `rejected` by `skeptic`
  after the author's dispute, where `R-performance-1` carries
  `conflict_decision` with `by: human:Ludovic` and a note
- **WHEN** the user runs `codev review --json`
- **THEN** both findings have `awaiting: null`

#### Scenario: A conflict decision taken by the author

- **GIVEN** a linked finding whose `conflict_decision` has `by:
  developer`, or no `note`
- **WHEN** the user runs `codev validate <change>`
- **THEN** a `review_decision_incomplete` error at its block says the
  decision is a human's, with a note

#### Scenario: A conflict decision without a link

- **GIVEN** a finding with `conflict_decision` and no `conflicts_with`
- **WHEN** the user runs `codev validate <change>`
- **THEN** a `review_invalid` error at its block names `conflicts_with`

#### Scenario: An earlier choice does not settle a later link

- **GIVEN** at `plan.challenge`, `R-performance-1` linked to
  `R-skeptic-2`, which is `accepted-risk` by `human:Ludovic`, and
  `R-performance-1` `fixed` and verified; and at `code.review`, a new
  finding `R-security-1`, confidence `high`, linked to `R-performance-1`
- **WHEN** the user runs `codev review --json`
- **THEN** `R-security-1` and `R-performance-1` have `awaiting: "human"`
- **AND** `R-skeptic-2` has `awaiting: null`

#### Scenario: A conflict of low-confidence findings

- **GIVEN** linked findings, both with confidence `low` and `open`
- **WHEN** the user runs `codev review --json`
- **THEN** both have `awaiting: "author"`

#### Scenario: A low-confidence finding linked to a confident one

- **GIVEN** linked findings `R-performance-1` with confidence `low` and
  `R-skeptic-2` with confidence `high`, both `open`
- **WHEN** the user runs `codev review --json`
- **THEN** both have `awaiting: "human"`, and `R-performance-1` keeps
  `reported: false`

#### Scenario: The human output names the link

- **GIVEN** linked findings `R-performance-1` and `R-skeptic-2`, both
  `open` with confidence `high`
- **WHEN** the user runs `codev review`
- **THEN** the output names `R-skeptic-2` under `R-performance-1` as a
  finding it conflicts with, and the reverse
- **AND** both are listed under `Waiting for an answer` with `human`

#### Scenario: A record without links

- **GIVEN** a record whose findings carry no `conflicts_with`
- **WHEN** the user runs `codev review --json`
- **THEN** each finding has `conflictsWith: []` and the `awaiting` its
  history gives

## MODIFIED Requirements

### Requirement: `codev review` shows the record and what is left to decide

`codev review [--change <name>] [--json]` SHALL read the change's record
and print, per step and per role, the findings with their severity,
status and location, and, for each finding, who must act next: the
author for an `open` finding, the reviewer for a `fixed` finding not yet
verified or a `disputed` one the reviewer has not answered, a human for a
dispute the reviewer maintained, and a human for every finding with an
unsettled link that waits for the human, whatever its own status, as
the requirement on opposed findings defines it. It SHALL say which findings are
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

#### Scenario: A fixed finding with an unsettled link is for the human

- **GIVEN** a finding `fixed` by `developer` then by `performance`, with
  confidence `high`, linked by `conflicts_with` to an `open` finding of
  the skeptic, and no human's answer on either
- **WHEN** the user runs `codev review --json`
- **THEN** the fixed finding's `awaiting` is `human`, not `null`

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

### Requirement: `codev archive` refuses a change whose review is not settled

Before moving a change, and after the validation pre-flight, `codev
archive` SHALL refuse with the stable code `review_blocked`, writing and
moving nothing, when, for `plan.challenge` or `code.review`, at least one
role is active for the change and the record does not exist
(`review_record_missing`) or has no section for that step
(`review_step_missing`); or when a `high` finding's status is `open` or
`disputed` (`review_unresolved`); or when an unsettled link joins a
`high` finding with confidence `high`, whatever that finding's status
(`review_conflict_unsettled`, naming the two linked findings); or when
the record's format is newer than this codev reads
(`review_format_unsupported`). An unsettled link between `medium` or
`low` findings does not block: it waits for the human as a maintained
`medium` dispute does. A human can settle any link, whatever the
statuses, with a `conflict_decision`. The message names each reason and
points to `codev review`. A step with no active role needs no section.
`codev sync` is not affected.

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

#### Scenario: A high conflict settled without a human

- **GIVEN** a change whose record links `R-performance-1`, `high`,
  `fixed` by the author and verified by `performance`, to
  `R-skeptic-2`, `high`, `rejected` by `skeptic` after the author's
  dispute, both with confidence `high`, and no human's answer on either
- **WHEN** the user runs `codev archive --change <change> --json`
- **THEN** the command fails with the code `review_blocked`
- **AND** the reasons include `review_conflict_unsettled`, naming
  `R-performance-1` and `R-skeptic-2`

#### Scenario: A conflict decision lets the change go

- **GIVEN** the record of the previous scenario, where `R-performance-1`
  now carries `conflict_decision` with `by: human:Ludovic` and a note
- **WHEN** the user runs `codev archive`
- **THEN** the change is archived

#### Scenario: A medium conflict does not block

- **GIVEN** a change whose record links two `medium` findings with
  confidence `high`, both `open`, and holds no other unsettled finding
- **WHEN** the user runs `codev archive`
- **THEN** the change is archived

#### Scenario: A high conflict the human settled

- **GIVEN** a change whose record links `R-performance-1`, `high`,
  `fixed` and verified, to `R-skeptic-2`, `high`, `accepted-risk` by
  `human:Ludovic` with a note
- **WHEN** the user runs `codev archive`
- **THEN** the change is archived

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

