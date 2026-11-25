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
embedded proposal template MUST carry the `## Release note` section.

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

#### Scenario: A custom schema gets no role block

- **GIVEN** a change using a project schema whose artifact instruction has
  no role block
- **WHEN** the user runs `codev instructions <artifact> --change <name>
  --json`
- **THEN** `instruction` is the schema's text, with no `Role:` line added

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
