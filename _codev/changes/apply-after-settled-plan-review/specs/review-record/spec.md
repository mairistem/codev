## ADDED Requirements

### Requirement: `codev status` says whether the plan review lets the implementation start

When the change has a `review.md`, `codev status --change <name> --json`
SHALL carry a `planReview` object with `settled`, a boolean, and
`blockers`, a list whose entries each have a stable `code` and a
`message`. Without `review.md`, the document SHALL have no `planReview`
key, and a consumer reads its absence as a plan review that blocks
nothing. The plan review SHALL be unsettled, each reason a blocker,
when:

- a finding of the record's `plan.challenge` section has severity `high`
  and its status is `open` or `disputed` — code `review_unresolved`, the
  message naming the finding's id, its title and its status;
- a finding of the record's `plan.challenge` section has severity `high`
  and its last entry is `accepted-risk` or `deferred` but the history
  does not satisfy the record's rules for that decision — not given by
  `human` or `human:<name>`, an `accepted-risk` without its `note`, a
  `deferred` without its `to` — code `review_decision_incomplete`, the
  message naming the finding and what is missing;
- the record's format is newer than this codev reads — code
  `review_format_unsupported`, the message telling the user to update
  codev.

It SHALL be settled otherwise, and in particular when the record has no
`plan.challenge` section, when the
plan's `high` findings are all `fixed`, `rejected`, or `accepted-risk`
or `deferred` as the record's rules require, and whatever the findings of the `code.review` section are.
A missing record or step is not a blocker here, whether roles are active
or not: `codev archive` still refuses it as its own requirement says.
The human output of `codev status` SHALL add, after its planning lines
and only when the plan review is unsettled, a line starting with
`Plan review: not settled`, naming each blocker's finding or reason and
`codev review --change <name>`. The exit code stays 0.

#### Scenario: A high plan finding still open

- **GIVEN** a change whose record holds, under `## plan.challenge`, the
  finding `R-skeptic-1` with severity `high` and the history `open` by
  `skeptic`
- **WHEN** the user runs `codev status --change <change> --json`
- **THEN** `planReview.settled` is `false`
- **AND** `planReview.blockers` holds one entry with the code
  `review_unresolved` whose message names `R-skeptic-1`
- **AND** the exit code is 0

#### Scenario: A dispute the reviewer maintained

- **GIVEN** a `high` finding at `plan.challenge` `disputed` by `product`,
  then `disputed` by `skeptic` with a note
- **WHEN** the user runs `codev status --change <change> --json`
- **THEN** `planReview.settled` is `false` with a `review_unresolved`
  blocker naming the finding

#### Scenario: A human accepted the risk

- **GIVEN** a `high` finding at `plan.challenge` whose last entry is
  `accepted-risk` by `human` with a justification
- **WHEN** the user runs `codev status --change <change> --json`
- **THEN** `planReview.settled` is `true` and `blockers` is empty

#### Scenario: A human deferred the finding

- **GIVEN** a `high` finding at `plan.challenge` whose last entry is
  `deferred` by `human:Ludovic` with `to: some-later-change`
- **WHEN** the user runs `codev status --change <change> --json`
- **THEN** `planReview.settled` is `true`

#### Scenario: A risk accepted by an agent does not settle the plan

- **GIVEN** a `high` finding at `plan.challenge` whose last entry is
  `accepted-risk` by `lead`, without a `note`
- **WHEN** the user runs `codev status --change <change> --json`
- **THEN** `planReview.settled` is `false` with a blocker of code
  `review_decision_incomplete` naming the finding

#### Scenario: Code findings do not unsettle the plan

- **GIVEN** a record whose `plan.challenge` section has `No finding.` for
  every role and whose `code.review` section holds a `high` finding
  `open`
- **WHEN** the user runs `codev status --change <change> --json`
- **THEN** `planReview.settled` is `true`

#### Scenario: A medium plan finding does not unsettle the plan

- **GIVEN** a `medium` finding at `plan.challenge` still `open`
- **WHEN** the user runs `codev status --change <change> --json`
- **THEN** `planReview.settled` is `true`

#### Scenario: No record, roles active

- **GIVEN** a change whose roles are active at `plan.challenge` and that
  has no `review.md`
- **WHEN** the user runs `codev status --change <change> --json`
- **THEN** the document has no `planReview` key
- **AND** `codev archive --change <change>` still refuses with
  `review_blocked`, naming `review_record_missing`

#### Scenario: A record without a plan step

- **GIVEN** a record holding only a `code.review` section
- **WHEN** the user runs `codev status --change <change> --json`
- **THEN** `planReview.settled` is `true` and `blockers` is empty

#### Scenario: A record from a newer codev

- **GIVEN** a record starting with `<!-- codev-review: 2 -->`
- **WHEN** the user runs `codev status --change <change> --json`
- **THEN** `planReview.settled` is `false` with one blocker of code
  `review_format_unsupported` whose message says to update codev

#### Scenario: The human output names what is left

- **GIVEN** a change whose record holds `R-skeptic-1`, `high` and `open`,
  at `plan.challenge`
- **WHEN** the user runs `codev status --change <change>`
- **THEN** the output has a line starting with `Plan review: not
  settled` naming `R-skeptic-1` and `codev review --change <change>`
- **AND** a change whose plan review is settled has no such line
