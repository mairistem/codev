## ADDED Requirements

### Requirement: The `propose` and `apply` skills take their roles from `codev roles`

At each role step, the `propose` skill SHALL run
`codev roles --at <step> [--change <name>] --json` — `plan.understand`
before creating the change, then `plan.investigate`, `plan.trace` and
`plan.challenge` with `--change` — and the `apply` skill SHALL do the
same at `code.implement` and `code.review`. The skill MUST apply the
criteria of every role returned, the project's extra criteria included,
and MUST skip the step when no role is returned. Writing an artifact, the
`propose` skill MUST check the criteria of the `role` field of
`codev instructions` before moving on. The skill texts MUST NOT restate
the criteria served as data. The `allowed-tools` of both skills are
unchanged.

#### Scenario: The skill bodies ask for the roles of each step

- **GIVEN** the codev workflow catalog
- **WHEN** the `propose` and `apply` workflows are resolved
- **THEN** the `propose` body names `codev roles --at` with
  `plan.understand`, `plan.investigate`, `plan.trace` and
  `plan.challenge`, and the `apply` body with `code.implement` and
  `code.review`
- **AND** neither body restates the criteria of the built-in catalog

#### Scenario: A disabled investigator

- **GIVEN** a configuration with `roles: { investigator: never }`
- **WHEN** `/codev-propose` writes the artifacts
- **THEN** it does not run the code study before each artifact
- **AND** its summary says the investigation is disabled by configuration

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
and MUST describe a `## Release note` section, after `## Impact`: bullets
`- <Category>: <text>` with Category among `Added`, `Changed`,
`Deprecated`, `Removed`, `Fixed` and `Security`, breaking items starting
with `**BREAKING**`, or the single line `None` for an internal change. The
product owner's criteria MUST require that the release note is written for
end users, categorized, with breaking items marked, or `None`. The
embedded proposal template MUST carry the `## Release note` section. The
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

### Requirement: The `propose` skill checks traceability before presenting the plan

Once every required artifact is written and before showing the final
status, the `propose` skill SHALL re-read the artifacts from disk and check
that:

- every capability listed in the proposal has its spec file, and every spec
  file is listed in the proposal;
- every requirement has a nominal scenario and an error or edge-case
  scenario, or states why none applies;
- every scenario is covered by a task whose verification refers to it and
  names an automated test, or says why automation is impossible;
- a task updates the documentation when the proposal's Impact touches
  documented behavior, or the plan states that none is affected;
- the release note agrees with the plan: every **BREAKING** item of What
  Changes, and every REMOVED requirement or MODIFIED requirement that
  breaks users, has a breaking item, and an internal change says `None`;
- every task names a concrete verification.

The skill MUST fix the gaps it finds directly in the artifacts. The check
MUST NOT modify any code file.

The skill SHALL take this check's criteria from
`codev roles --at plan.trace --change <name> --json` — the `tracer`'s,
followed by the project's extra criteria — and apply them all. When no
role is active at `plan.trace`, the skill MUST skip the check and its
summary MUST say that the traceability check is disabled by configuration.

#### Scenario: A requirement without an error scenario gets one

- **GIVEN** a plan whose spec has a requirement with only a nominal scenario
- **WHEN** `/codev-propose` reaches the traceability check
- **THEN** the requirement gains an error or edge-case scenario, or a stated
  reason why none applies
- **AND** a task's verification covers the new scenario

#### Scenario: A spec file missing from the proposal is listed

- **GIVEN** a plan with a spec file whose capability is not listed in the
  proposal
- **WHEN** `/codev-propose` reaches the traceability check
- **THEN** the proposal lists the capability, or the spec file is removed
  if the capability is out of scope

#### Scenario: A plan without a documentation task gets one

- **GIVEN** a plan whose Impact changes a documented command and whose
  tasks update no documentation
- **WHEN** `/codev-propose` reaches the traceability check
- **THEN** the tasks gain a documentation task, or the plan states why no
  documentation is affected

#### Scenario: A scenario verified only by hand gets an automated test

- **GIVEN** a plan whose only task covering a scenario is verified by a
  manual check, with no reason given
- **WHEN** `/codev-propose` reaches the traceability check
- **THEN** the covering task names an automated test, or states why
  automation is impossible

#### Scenario: A breaking change missing from the release note is added

- **GIVEN** a plan whose What Changes marks an item **BREAKING** and whose
  release note has no breaking item
- **WHEN** `/codev-propose` reaches the traceability check
- **THEN** the release note gains the breaking item

#### Scenario: A complete plan is left unchanged

- **GIVEN** a plan where capabilities, spec files, scenarios, automated
  tests, documentation tasks, task verifications and the release note all
  cover each other
- **WHEN** `/codev-propose` reaches the traceability check
- **THEN** no artifact is modified by the check

#### Scenario: The traceability check disabled by configuration

- **GIVEN** a configuration with `roles: { tracer: never }`
- **WHEN** `/codev-propose` has written the artifacts
- **THEN** no traceability check runs
- **AND** the summary says the traceability check is disabled by
  configuration

### Requirement: The `propose` skill challenges the plan and reports points to challenge

After the traceability check, the `propose` skill SHALL re-read the
artifacts as a reviewer who did not write them, along a fixed grid: the
need (does the change solve the Why, is there a simpler solution or one
without code), the scope (anything nobody asked for, an obvious case
missing), the specs (error and edge cases, each requirement verifiable),
the decisions (a silent contradiction with a decision in effect), the
assumptions (the one that would bring the plan down if wrong) and the tasks
(each one verifiable). It MUST also apply, only when the proposal's Impact
touches them, the lenses security & privacy, compatibility & migration,
operability, performance, and accessibility & UX, as well as the project's
`rules:` received through `codev instructions`.

The pass MUST fix what is unambiguous directly in the artifacts and record
what needs a human decision in the Open Questions of `design.md`, or in the
proposal when there is no design. It MUST NOT add scope.

The final summary of the skill MUST end with a "Points to challenge" block:
at most five items, ranked by impact, each on one line giving what, why it
matters and the artifact concerned. When the pass finds nothing worth
raising, the block MUST say "No point to challenge found" instead of
inventing points.

The grid is the criteria `codev roles --at plan.challenge --change <name>
--json` returns — the `skeptic`'s, followed by the project's extra
criteria. When no role is active at `plan.challenge`, the skill MUST skip
the pass, and the "Points to challenge" block MUST say that the skeptic is
disabled by configuration instead of listing points.

#### Scenario: Points to challenge are reported

- **GIVEN** a plan whose design rests on an unverified assumption
- **WHEN** `/codev-propose` presents the plan
- **THEN** the summary contains a "Points to challenge" block naming the
  assumption, why it matters and the artifact concerned
- **AND** the same point is recorded in the Open Questions of `design.md`

#### Scenario: At most five points

- **GIVEN** a plan in which the contrarian pass finds eight weak points
- **WHEN** `/codev-propose` presents the plan
- **THEN** the "Points to challenge" block lists at most five items, the
  ones with the highest impact first

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

- **GIVEN** a configuration with `roles: { skeptic: never }`
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
`apply` skill SHALL re-read only the code the change added or modified, as a
developer joining the project who has seen neither the plan nor the
conversation, along a fixed grid: each changed function makes sense without
the plan or the conversation; names say what things do, with no obscure
abbreviation; a function too long or too nested is split when it splits
simply; dense one-liners, premature genericity and single-use abstractions
give way to the simple version unless the clever one is justified; comments
explain the why where it is not obvious and never paraphrase the code; tests
read as a description of the behavior. It MUST also apply the project's
`rules` received through `codev instructions`.

The skill MUST simplify directly what is unambiguous and re-run the affected
tests. The review MUST NOT change behavior, MUST NOT widen the change's
scope and MUST NOT modify code the change did not add or modify.

The final summary MUST carry a "Readability notes" block listing what the
review did not simplify alone: at most three items, each giving what, where
and why. When the review finds nothing worth raising, the block MUST say "No
readability concern found" instead of inventing notes.

The grid is the criteria `codev roles --at code.review --change <name>
--json` returns — the `newcomer`'s, followed by the project's extra
criteria. When no role is active at `code.review`, the skill MUST skip the
review, and the "Readability notes" block MUST say that the newcomer review
is disabled by configuration.

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

#### Scenario: At most three readability notes

- **GIVEN** a review that finds five concerns it does not simplify alone
- **WHEN** `/codev-apply` presents its final summary
- **THEN** the "Readability notes" block lists at most three items, each
  with what, where and why

#### Scenario: Nothing to note

- **GIVEN** a change whose code raises no readability concern
- **WHEN** `/codev-apply` presents its final summary
- **THEN** the summary says "No readability concern found"
- **AND** lists no invented note

#### Scenario: Code outside the change is untouched

- **GIVEN** a change that calls an obscure function it did not modify
- **WHEN** `/codev-apply` reviews the change as a newcomer
- **THEN** that function is left unchanged
- **AND** it appears, at most, as a readability note

#### Scenario: The newcomer review disabled by configuration

- **GIVEN** a change whose `change.yaml` holds `roles: { newcomer: never }`
- **WHEN** `/codev-apply` has checked every task
- **THEN** no newcomer review runs and the full test suite runs next
- **AND** the "Readability notes" block says the newcomer review is
  disabled by configuration
