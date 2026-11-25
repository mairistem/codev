## ADDED Requirements

### Requirement: The default schema gives each artifact a role with "Done when" criteria

The instruction of each artifact of the embedded `spec-driven` schema, as
returned by `codev instructions <artifact> --json`, SHALL start with a role
block: a `Role:` line naming the role and what it owns, followed by a
`Done when:` list of checkable criteria. The roles are:

- `proposal` — product owner, owns why and what;
- `specs` — QA analyst, owns observable behavior;
- `design` — architect, owns how;
- `tasks` — tech lead, owns sequencing and verification.

The rest of each instruction MUST stay as before. A custom schema's
instructions MUST be returned as written, without any role block added by
codev.

#### Scenario: The proposal instruction starts with the product owner role

- **GIVEN** a change using the `spec-driven` schema
- **WHEN** the user runs `codev instructions proposal --change <name> --json`
- **THEN** `instruction` starts with `Role: product owner`
- **AND** it contains a `Done when:` list that includes marking every
  breaking change with **BREAKING**

#### Scenario: The tasks criteria cover every spec scenario

- **GIVEN** a change using the `spec-driven` schema
- **WHEN** the user runs `codev instructions tasks --change <name> --json`
- **THEN** `instruction` starts with `Role: tech lead`
- **AND** its `Done when:` list requires every spec scenario to be covered
  by at least one task's verification

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
- every scenario is covered by a task whose verification refers to it;
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

#### Scenario: A complete plan is left unchanged

- **GIVEN** a plan where capabilities, spec files, scenarios and task
  verifications all cover each other
- **WHEN** `/codev-propose` reaches the traceability check
- **THEN** no artifact is modified by the check

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
