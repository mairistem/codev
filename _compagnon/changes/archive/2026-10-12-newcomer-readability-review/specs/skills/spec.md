## ADDED Requirements

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

## MODIFIED Requirements

### Requirement: The default schema gives each artifact a role with "Done when" criteria

The instruction of each artifact of the embedded `spec-driven` schema, as
returned by `codev instructions <artifact> --json`, SHALL start with a role
block: a `Role:` line naming the role and what it owns, followed by a
`Done when:` list of checkable criteria. The roles are:

- `proposal` — product owner, owns why and what;
- `specs` — QA analyst, owns observable behavior;
- `design` — architect, owns how;
- `tasks` — tech lead, owns sequencing and verification.

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

The rest of each instruction MUST stay as before. A custom schema's
instructions MUST be returned as written, without any role block added by
codev.

#### Scenario: The proposal instruction starts with the product owner role

- **GIVEN** a change using the `spec-driven` schema
- **WHEN** the user runs `codev instructions proposal --change <name> --json`
- **THEN** `instruction` starts with `Role: product owner`
- **AND** it contains a `Done when:` list that includes marking every
  breaking change with **BREAKING**
- **AND** it asks the Impact section to name the affected documentation
- **AND** its `Done when:` list requires a release note written for end
  users, categorized, with breaking items marked, or `None`
- **AND** `template` has a `## Release note` section after `## Impact`

#### Scenario: The tasks criteria cover every spec scenario

- **GIVEN** a change using the `spec-driven` schema
- **WHEN** the user runs `codev instructions tasks --change <name> --json`
- **THEN** `instruction` starts with `Role: tech lead`
- **AND** its `Done when:` list requires every spec scenario to be covered
  by at least one task's verification

#### Scenario: The tasks criteria require documentation and automated tests

- **GIVEN** a change using the `spec-driven` schema
- **WHEN** the user runs `codev instructions tasks --change <name> --json`
- **THEN** its `Done when:` list requires the affected documentation to be
  updated in a task, or the plan to state that none is affected
- **AND** it requires every spec scenario to be covered by an automated
  test, a manual check only when automation is impossible, with the reason
- **AND** it requires tests to follow the project's existing test suite,
  the conventions of its language and its `rules`

#### Scenario: The design criteria require the simplest approach

- **GIVEN** a change using the `spec-driven` schema
- **WHEN** the user runs `codev instructions design --change <name> --json`
- **THEN** `instruction` starts with `Role: architect`
- **AND** its `Done when:` list requires the approach to be the simplest
  that satisfies the specs, and the design to be followable by a newcomer
  without knowing its history

#### Scenario: A custom schema gets no role block

- **GIVEN** a change using a project schema whose artifact instruction has
  no role block
- **WHEN** the user runs `codev instructions <artifact> --change <name>
  --json`
- **THEN** `instruction` is the schema's text, with no `Role:` line added
