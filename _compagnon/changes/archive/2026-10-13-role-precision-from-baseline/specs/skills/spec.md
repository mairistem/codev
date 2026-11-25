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
verifiable), the domains (every domain the plan touches is ticked in the
proposal's `## Domains`, so that its expert reviews it), as well as the
project's `rules:` received through `codev instructions`. The `skeptic`
challenges the plan's logic and evidence; it MUST NOT review a domain
itself — security, privacy, compatibility and migration, operations,
performance, accessibility — which belongs to the expert that domain
activates. An expert asks whether the design addresses its domain.

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

#### Scenario: The skeptic leaves a domain to its expert

- **GIVEN** a plan whose proposal ticks `operations`, and whose design
  calls a new provider without stating a timeout
- **WHEN** the contrarian pass runs
- **THEN** the missing timeout is not a finding of the `skeptic`
- **AND** the `skeptic`'s criteria name the domains as the experts' own

#### Scenario: A touched domain left unticked

- **GIVEN** a plan whose design stores a new e-mail field, and whose
  proposal leaves `personal-data` unticked with no reason
- **WHEN** the contrarian pass runs
- **THEN** the `skeptic` records a finding at `proposal.md#Domains`
  saying that the privacy expert will not review the plan

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
