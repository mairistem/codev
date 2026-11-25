## MODIFIED Requirements

### Requirement: The `propose` skill challenges the plan and reports points to challenge

After the traceability check, the `propose` skill SHALL re-read the
artifacts as reviewers who did not write them: every role
`codev roles --at plan.challenge --change <name> --json` returns — the
`skeptic` and the experts the change's domains activate — each along its
criteria, the project's extra criteria included, and using its
`knowledge` when it has one. The `skeptic`'s grid covers the need (does
the change solve the Why, is there a simpler solution or one without
code), the scope (anything nobody asked for, an obvious case missing), the
specs (error and edge cases, each requirement verifiable), the decisions
(a silent contradiction with a decision in effect), the assumptions (the
one that would bring the plan down if wrong) and the tasks (each one
verifiable). It MUST also apply, only when the proposal's Impact touches
them, the lenses security & privacy, compatibility & migration,
operability, performance, and accessibility & UX, as well as the project's
`rules:` received through `codev instructions`. An expert asks whether
the design addresses its domain.

Each finding MUST be written to the change's review record under
`## plan.challenge`, then answered as the review record's answer loop
describes: the author fixes what is unambiguous directly in the artifacts
or disputes it, the reviewer re-checks once, and what remains is for the
human. The pass MUST NOT add scope.

The final summary of the skill MUST end with a "Points to challenge" block:
at most five items in total across the roles, taken from the findings the
record reports to the human (confidence `high`) that were not rejected,
ranked by severity, each on one line tagged with the id of the role that
raised it and giving its finding id, its status, what, why it matters and
the artifact concerned. When the pass finds nothing worth raising, the
block MUST say "No point to challenge found" instead of inventing points.

When no role is active at `plan.challenge`, the skill MUST skip the pass,
and the "Points to challenge" block MUST say that the skeptic is disabled
by configuration instead of listing points.

#### Scenario: Points to challenge are reported

- **GIVEN** a plan whose design rests on an unverified assumption
- **WHEN** `/codev-propose` presents the plan
- **THEN** the summary contains a "Points to challenge" block naming the
  assumption, why it matters and the artifact concerned, tagged `skeptic`
- **AND** the same point is a finding of `review.md` under
  `## plan.challenge`, with its status

#### Scenario: At most five points

- **GIVEN** a plan in which the skeptic finds four weak points and the
  `security` expert four more
- **WHEN** `/codev-propose` presents the plan
- **THEN** the "Points to challenge" block lists at most five items in
  total, the most severe first, each tagged with its role id
- **AND** all eight are findings of the record

#### Scenario: An active expert challenges the design

- **GIVEN** a change whose proposal ticks `personal-data` and whose design
  stores a new e-mail field with no retention rule
- **WHEN** `/codev-propose` challenges the plan
- **THEN** the missing retention rule is a finding of the record under
  `### privacy`, fixed in the design or raised as a point to challenge

#### Scenario: Nothing to challenge

- **GIVEN** a plan in which the contrarian pass finds nothing worth raising
- **WHEN** `/codev-propose` presents the plan
- **THEN** the summary says "No point to challenge found"
- **AND** lists no invented point
- **AND** each role's section of the record says `No finding.`

#### Scenario: A conditional lens outside the Impact is not applied

- **GIVEN** a plan whose Impact touches no user interface
- **WHEN** the contrarian pass runs
- **THEN** no accessibility & UX point is raised

#### Scenario: The pass does not add scope

- **GIVEN** a plan for which the reviewer thinks of a useful but unrequested
  feature
- **WHEN** the contrarian pass runs
- **THEN** the feature is not added to the artifacts

#### Scenario: The skeptic disabled by configuration

- **GIVEN** a configuration with `roles: { skeptic: never }` and a change
  that ticks no domain
- **WHEN** `/codev-propose` presents the plan
- **THEN** no contrarian pass has run
- **AND** the "Points to challenge" block says the skeptic is disabled by
  configuration

#### Scenario: A project criterion joins the grid

- **GIVEN** a configuration where `skeptic` has the extra criterion
  `The rollback plan is stated`
- **WHEN** the contrarian pass runs on a plan with no rollback plan
- **THEN** the missing rollback plan is fixed or raised as a point to
  challenge

### Requirement: The `apply` skill reviews the change's code as a newcomer

Once every task is checked and before running the full test suite, the
`apply` skill SHALL re-read only the code the change added or modified,
with every role `codev roles --at code.review --change <name> --json`
returns — the `newcomer` and the experts the change's domains activate —
each along its criteria, the project's extra criteria included, and using
its `knowledge` when it has one. The `newcomer` reads as a developer
joining the project who has seen neither the plan nor the conversation:
each changed function makes sense without the plan or the conversation;
names say what things do, with no obscure abbreviation; a function too
long or too nested is split when it splits simply; dense one-liners,
premature genericity and single-use abstractions give way to the simple
version unless the clever one is justified; comments explain the why where
it is not obvious and never paraphrase the code; tests read as a
description of the behavior. It MUST also apply the project's `rules`
received through `codev instructions`. An expert asks whether the
implementation respects its domain.

Each finding MUST be written to the change's review record under
`## code.review`, then answered as the review record's answer loop
describes. The skill MUST simplify directly what is unambiguous and
re-run the affected tests. The review MUST NOT widen the change's scope
and MUST NOT modify code the change did not add or modify. The `newcomer`
MUST NOT change behavior. An expert MUST NOT change behavior silently: a
fix that changes behavior is left for the human, unless it is a clear
defect within the change's scope, which is then fixed with a test and
named in the summary.

The final summary MUST carry a "Review notes" block: at most five items in
total across the roles, taken from the findings the record reports to the
human (confidence `high`) that were not rejected, ranked by severity,
each tagged with the id of the role that raised it and giving its finding
id, its status, what, where and why. A high-severity `security` or
`privacy` finding MUST always be listed, even beyond five. When the review
finds nothing worth raising, the block MUST say "No review concern found"
instead of inventing notes.

When no role is active at `code.review`, the skill MUST skip the review,
and the "Review notes" block MUST say that the review is disabled by
configuration.

#### Scenario: A needlessly clever construction is simplified

- **GIVEN** a change whose tasks are all checked, and whose code contains a
  dense one-liner with a simple equivalent
- **WHEN** `/codev-apply` reviews the change as a newcomer
- **THEN** the one-liner is replaced by the simple version
- **AND** the affected tests are run again and pass

#### Scenario: The review keeps behavior and scope

- **GIVEN** a change whose code could be simplified by also changing an edge
  case's behavior, or by adding an unrequested helper
- **WHEN** `/codev-apply` reviews the change as a newcomer
- **THEN** the behavior and the scope of the change stay as implemented
- **AND** the tests that passed before the review still pass

#### Scenario: At most five review notes

- **GIVEN** a review that finds seven concerns it does not fix alone, none
  of them a high-severity security or privacy finding
- **WHEN** `/codev-apply` presents its final summary
- **THEN** the "Review notes" block lists at most five items, each tagged
  with its role id and giving what, where and why
- **AND** all seven are findings of the record

#### Scenario: A high-severity security finding is never dropped

- **GIVEN** a review with five readability notes and a `security` finding
  of high severity
- **WHEN** `/codev-apply` presents its final summary
- **THEN** the `security` finding is listed in the "Review notes" block

#### Scenario: An expert reports a behavior-changing fix

- **GIVEN** a change whose proposal ticks `performance`, and whose code
  could be made faster by changing the order of its results
- **WHEN** `/codev-apply` reviews the change
- **THEN** the order of results is unchanged
- **AND** the suggestion is a finding of the record under
  `### performance`, listed as a review note

#### Scenario: An expert fixes a clear defect within scope

- **GIVEN** a change whose proposal ticks `security`, and whose new
  endpoint builds an SQL query by concatenating a request parameter
- **WHEN** `/codev-apply` reviews the change
- **THEN** the query is parameterized, a test covers the hostile input,
  and the summary names the fix
- **AND** the record's finding is `fixed` with a reference to the change

#### Scenario: Nothing to note

- **GIVEN** a change whose code raises no review concern
- **WHEN** `/codev-apply` presents its final summary
- **THEN** the summary says "No review concern found"
- **AND** lists no invented note

#### Scenario: Code outside the change is untouched

- **GIVEN** a change that calls an obscure function it did not modify
- **WHEN** `/codev-apply` reviews the change as a newcomer
- **THEN** that function is left unchanged
- **AND** it appears, at most, as a review note

#### Scenario: The review disabled by configuration

- **GIVEN** a change whose `change.yaml` holds `roles: { newcomer: never }`
  and whose proposal ticks no domain
- **WHEN** `/codev-apply` has checked every task
- **THEN** no review runs and the full test suite runs next
- **AND** the "Review notes" block says the review is disabled by
  configuration

## ADDED Requirements

### Requirement: The `propose` and `apply` skills keep the review record and run the answer loop

At `plan.challenge` and `code.review`, the skill SHALL first read the
record with `codev review --change <name> --json` and resume from it: a
finding already recorded is not raised again, and a finding still waiting
for an answer is answered before anything else. It SHALL write, under the
step's section of `review.md`, one section per role returned — its
findings, or `No finding.` — and `Disabled by configuration.` for each
role of the step that the project or the change sets to `never`. Each
finding gets the next free id of its role, a severity and a confidence
from the shared scales, a location, the cited text copied verbatim after
re-reading it, a recommendation, and a first history entry `open` by its
role.

Then the answer loop: as the author — the role owning the cited artifact
at `plan.challenge`, the `developer` at `code.review` — the skill fixes
each finding (`fixed`, with the reference of the change) or disputes it
with an argument grounded in the material (`disputed`). Then, as the
finding's role, it re-checks each answer once: a fix is verified
(`fixed` again by the role) or reopened (`open`, with why); a dispute is
accepted (`rejected`) or maintained (`disputed`, with why). There is no
second round. The skill MUST NOT record `accepted-risk`, nor defer a
`high` finding: those are the human's decisions, recorded with `by:
human` (or `human:<name>`) only once the human has given them.

The skill SHALL then run `codev review --change <name> --json` and fix
the defects it reports in the record itself, never by changing a
finding's substance. Its summary MUST carry a "To decide" block listing
every finding the record says awaits a human and every `high` finding
still `open`, with the choices: fix, accept the risk (with a
justification) or defer (with where it goes); or "Nothing to decide".

#### Scenario: The skills describe the record and the loop

- **GIVEN** the codev workflow catalog
- **WHEN** the `propose` and `apply` workflows are rendered
- **THEN** each names `review.md`, `codev review --change`, the statuses,
  the author's answer, the single re-check and the "To decide" block

#### Scenario: A maintained dispute goes to the human

- **GIVEN** a finding the author disputes and the reviewer maintains
- **WHEN** the skill presents its summary
- **THEN** the "To decide" block lists the finding with its argument and
  the reviewer's answer

#### Scenario: The agent never accepts a risk

- **GIVEN** a `high` finding the author cannot fix within the change
- **WHEN** the skill runs the answer loop
- **THEN** the finding stays `open` or `disputed` and is listed under "To
  decide"
- **AND** no `accepted-risk` entry is written without the human's answer

#### Scenario: A resumed session

- **GIVEN** a record whose `## code.review` holds a finding `disputed` by
  `developer` that the reviewer has not answered
- **WHEN** `/codev-apply` runs the review again in a new session
- **THEN** the reviewer answers that dispute
- **AND** no finding already in the record is written twice

### Requirement: The `archive` skill explains a review that blocks archiving

When `codev archive --json` fails with the code `review_blocked`, the
`archive` skill SHALL reply that the review record blocks archiving and
invite the user to run `codev review --change <name>` to see what is left
to decide, without retrying and without editing the record.

#### Scenario: Archive blocked by the review

- **GIVEN** a change for which `codev archive --json` refuses with
  `review_blocked`
- **WHEN** the user types `/codev-archive`
- **THEN** the skill says the review record blocks archiving and names
  `codev review --change <name>`
- **AND** does not retry
