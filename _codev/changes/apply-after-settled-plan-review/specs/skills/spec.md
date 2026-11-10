## ADDED Requirements

### Requirement: The `apply` skill refuses a plan whose review is not settled

After checking that planning is complete and before its first task, the
`apply` skill SHALL read `planReview` from the `codev status --change
<name> --json` it already runs. When `planReview.settled` is `false`, it
MUST refuse to implement: it replies with the stable refusal code
`plan_review_unsettled`, names each blocker's message and code, points to
`codev review --change <name>`, and gives the ways out — fix the plan
with `/codev-update` and record the fix in `review.md`, or have a human
accept the risk (with a justification) or defer the finding (with where
it goes) in `review.md` — then stops, modifying no file and checking no
task. The skill MUST NOT write `accepted-risk` or defer the finding
itself, even in a run without a user: only a person settles it. When
`planReview` is absent from the document — the change has no
`review.md`, or the codev predates the field — the skill SHALL proceed
as before. When `isPlanningComplete` is
`false`, the skill reports the missing artifacts first, as it already
does.

#### Scenario: An open high plan finding stops the implementation

- **GIVEN** a change whose planning is complete and whose `codev status
  --json` gives `planReview.settled` `false` with a `review_unresolved`
  blocker naming `R-skeptic-1`
- **WHEN** the user types `/codev-apply`
- **THEN** the skill replies with `plan_review_unsettled`, names
  `R-skeptic-1` and `codev review --change <name>`, and lists fixing the
  plan, accepting the risk and deferring the finding
- **AND** no file is modified and no task is checked

#### Scenario: A settled plan review lets the implementation start

- **GIVEN** a change whose only `high` plan finding is `accepted-risk` by
  `human`, so that `planReview.settled` is `true`
- **WHEN** the user types `/codev-apply`
- **THEN** the skill implements the first unchecked task

#### Scenario: No record, or an older codev, without the field

- **GIVEN** a `codev status --json` document without `planReview`
- **WHEN** the `apply` skill reads it for a complete plan
- **THEN** the skill goes on to the tasks

#### Scenario: A run without a user does not settle the finding itself

- **GIVEN** a run without a user and a plan review that is not settled
- **WHEN** the `apply` skill runs
- **THEN** it refuses with `plan_review_unsettled`
- **AND** `review.md` gains no `accepted-risk` or `deferred` entry

#### Scenario: The rendered skill carries the gate

- **GIVEN** the codev workflow catalog
- **WHEN** the `apply` workflow is rendered
- **THEN** its planning check names `planReview`, `settled`,
  `plan_review_unsettled` and `codev review --change`, after the
  `isPlanningComplete` check and before the tasks are read
