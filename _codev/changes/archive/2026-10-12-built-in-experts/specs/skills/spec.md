## MODIFIED Requirements

### Requirement: The default schema gives each artifact a role with "Done when" criteria

Each artifact of the embedded `spec-driven` schema SHALL declare its role as
data — an id, a display name, the question it owns and a list of "done
when" criteria — which `codev instructions <artifact> --json` returns in
its `role` field, with the origin `schema:spec-driven` on each criterion.
The roles are:

- `proposal` — `product` (product owner), owns why and what;
- `specs` — `qa` (QA analyst), owns observable behavior;
- `design` — `architect`, owns how;
- `tasks` — `lead` (tech lead), owns sequencing and verification.

The tech lead's criteria MUST require that the documentation affected by
the change is updated in a task, or that the plan states none is affected;
that every spec scenario is covered by an automated test written in a task,
a manual check being acceptable only when automation is impossible and with
the reason stated; and that tests follow the project's existing test suite,
the testing conventions of its language, and its `rules`. The `proposal`
instruction MUST ask the Impact section to name the affected documentation,
MUST describe a `## Domains` section, after `## Impact`: the six domains
`security`, `personal-data`, `user-interface`, `performance`, `operations`
and `migration` as a list to tick, or the single line `None`, and MUST
describe a `## Release note` section, after `## Domains`: bullets
`- <Category>: <text>` with Category among `Added`, `Changed`,
`Deprecated`, `Removed`, `Fixed` and `Security`, breaking items starting
with `**BREAKING**`, or the single line `None` for an internal change. The
product owner's criteria MUST require that the release note is written for
end users, categorized, with breaking items marked, or `None`, and that
the Domains section ticks every domain the change touches, or says `None`.
The embedded proposal template MUST carry the `## Domains` section, with
the six domains unticked, and the `## Release note` section. The
architect's criteria MUST require that the approach is the simplest that
satisfies the specs, and that a newcomer can follow the design without
knowing its history.

The instructions of the default schema MUST NOT restate the role: they no
longer start with a `Role:` line or a `Done when:` list. A custom
schema's instructions MUST be returned as written; its artifacts carry a
role only when the schema declares one.

#### Scenario: The proposal carries the product owner role

- **GIVEN** a change using the `spec-driven` schema
- **WHEN** the user runs `codev instructions proposal --change <name> --json`
- **THEN** `role.id` is `product` and `role.name` is `Product owner`
- **AND** `role.doneWhen` includes marking every breaking change with
  **BREAKING**
- **AND** `instruction` asks the Impact section to name the affected
  documentation, and does not start with `Role:`
- **AND** `role.doneWhen` requires a release note written for end users,
  categorized, with breaking items marked, or `None`
- **AND** `template` has a `## Release note` section after `## Impact`

#### Scenario: The proposal lists the domains it touches

- **GIVEN** a change using the `spec-driven` schema
- **WHEN** the user runs `codev instructions proposal --change <name> --json`
- **THEN** `template` has a `## Domains` section between `## Impact` and
  `## Release note`, listing the six domains unticked
- **AND** `instruction` describes the section and its `None`
- **AND** `role.doneWhen` requires every domain the change touches to be
  ticked, or `None`

#### Scenario: The tasks criteria cover every spec scenario

- **GIVEN** a change using the `spec-driven` schema
- **WHEN** the user runs `codev instructions tasks --change <name> --json`
- **THEN** `role.id` is `lead`
- **AND** `role.doneWhen` requires every spec scenario to be covered by at
  least one task's verification

#### Scenario: The tasks criteria require documentation and automated tests

- **GIVEN** a change using the `spec-driven` schema
- **WHEN** the user runs `codev instructions tasks --change <name> --json`
- **THEN** `role.doneWhen` requires the affected documentation to be
  updated in a task, or the plan to state that none is affected
- **AND** it requires every spec scenario to be covered by an automated
  test, a manual check only when automation is impossible, with the reason
- **AND** it requires tests to follow the project's existing test suite,
  the conventions of its language and its `rules`

#### Scenario: The design criteria require the simplest approach

- **GIVEN** a change using the `spec-driven` schema
- **WHEN** the user runs `codev instructions design --change <name> --json`
- **THEN** `role.id` is `architect`
- **AND** `role.doneWhen` requires the approach to be the simplest that
  satisfies the specs, and the design to be followable by a newcomer
  without knowing its history

#### Scenario: A custom schema gets no role added

- **GIVEN** a change using a project schema whose artifact declares no role
- **WHEN** the user runs `codev instructions <artifact> --change <name>
  --json`
- **THEN** `instruction` is the schema's text, with no `Role:` line added
- **AND** `role` is `null`

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

The pass MUST fix what is unambiguous directly in the artifacts and record
what needs a human decision in the Open Questions of `design.md`, or in the
proposal when there is no design. It MUST NOT add scope.

The final summary of the skill MUST end with a "Points to challenge" block:
at most five items in total across the roles, ranked by impact, each on one
line tagged with the id of the role that raised it and giving what, why it
matters and the artifact concerned. When the pass finds nothing worth
raising, the block MUST say "No point to challenge found" instead of
inventing points.

When no role is active at `plan.challenge`, the skill MUST skip the pass,
and the "Points to challenge" block MUST say that the skeptic is disabled
by configuration instead of listing points.

#### Scenario: Points to challenge are reported

- **GIVEN** a plan whose design rests on an unverified assumption
- **WHEN** `/codev-propose` presents the plan
- **THEN** the summary contains a "Points to challenge" block naming the
  assumption, why it matters and the artifact concerned, tagged `skeptic`
- **AND** the same point is recorded in the Open Questions of `design.md`

#### Scenario: At most five points

- **GIVEN** a plan in which the skeptic finds four weak points and the
  `security` expert four more
- **WHEN** `/codev-propose` presents the plan
- **THEN** the "Points to challenge" block lists at most five items in
  total, the ones with the highest impact first, each tagged with its role
  id

#### Scenario: An active expert challenges the design

- **GIVEN** a change whose proposal ticks `personal-data` and whose design
  stores a new e-mail field with no retention rule
- **WHEN** `/codev-propose` challenges the plan
- **THEN** the missing retention rule is fixed or raised as a point to
  challenge tagged `privacy`

#### Scenario: Nothing to challenge

- **GIVEN** a plan in which the contrarian pass finds nothing worth raising
- **WHEN** `/codev-propose` presents the plan
- **THEN** the summary says "No point to challenge found"
- **AND** lists no invented point

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

The skill MUST simplify directly what is unambiguous and re-run the affected
tests. The review MUST NOT widen the change's scope and MUST NOT modify code
the change did not add or modify. The `newcomer` MUST NOT change behavior.
An expert MUST NOT change behavior silently: a fix that changes behavior is
reported as a note, unless it is a clear defect within the change's scope,
which is then fixed with a test and named in the summary.

The final summary MUST carry a "Review notes" block listing what the
review did not fix alone: at most five items in total across the roles,
each tagged with the id of the role that raised it and giving what, where
and why. A high-severity `security` or `privacy` finding MUST always be
listed, even beyond five. When the review finds nothing worth raising, the
block MUST say "No review concern found" instead of inventing notes.

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
- **AND** the suggestion appears as a review note tagged `performance`

#### Scenario: An expert fixes a clear defect within scope

- **GIVEN** a change whose proposal ticks `security`, and whose new
  endpoint builds an SQL query by concatenating a request parameter
- **WHEN** `/codev-apply` reviews the change
- **THEN** the query is parameterized, a test covers the hostile input,
  and the summary names the fix

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
