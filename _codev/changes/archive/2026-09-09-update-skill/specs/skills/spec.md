## ADDED Requirements

### Requirement: Skill `update` revises a planning artifact

The codev catalog SHALL expose an `update` workflow — installed under
`.claude/skills/codev-update/SKILL.md`, invocable as `/codev-update` —
whose role is to revise an already-written planning artifact
(proposal, specs, design or tasks) of an active change, based on a
free-form description given by the user.

#### Scenario: Revising a design after a new constraint

- **GIVEN** an active change whose `design.md` cites a technical decision X
- **WHEN** the user types `/codev-update design "replace X with Y
  because of constraint Z"`
- **THEN** the skill reads `design.md`, applies the requested revision,
  and writes the new version
- **AND** reruns `codev validate <change>` at the end of processing

#### Scenario: Implicit resolution when a single change is active

- **GIVEN** a project with a single active change
- **WHEN** the user types `/codev-update proposal "reduce the
  scope"`
- **THEN** the skill implicitly resolves the active change
- **AND** applies the revision to that change's `proposal.md`

#### Scenario: Ambiguity about the change to revise

- **GIVEN** two active changes
- **WHEN** the user types `/codev-update tasks "…"` without naming a
  change
- **THEN** the skill asks the user which one to revise, listing the
  two names
- **AND** writes nothing before getting the answer

### Requirement: Skill `update` announces the ripple before acting

When the requested revision of one artifact makes another
inconsistent, the skill MUST report it to the user and propose the
fix before writing it, rather than leaving the main spec, the design
or the task list in silent disagreement.

#### Scenario: Removing a capability from the proposal ripples onto specs

- **GIVEN** a `proposal.md` declaring two new capabilities `a` and
  `b`, and a file `specs/b/spec.md` already written
- **WHEN** the user types `/codev-update proposal "remove capability
  b — out of scope after all"`
- **THEN** the skill applies the revision to `proposal.md`
- **AND** reports to the user that `specs/b/spec.md` becomes orphaned
- **AND** proposes to delete that file or to call
  `/codev-update specs …` to adjust it
- **AND** does not write this second modification without confirmation

#### Scenario: A revision without ripple applies without additional confirmation

- **GIVEN** a revision that only touches `design.md` with no
  consequence on the other artifacts
- **WHEN** the user types `/codev-update design "…"`
- **THEN** the skill applies the revision without asking for
  additional confirmation

### Requirement: Skill `update` stays within the planning boundary

The `update` workflow MUST limit itself to the files under
`_codev/changes/<name>/` and MUST NOT:

- modify project code;
- create a missing artifact (proposal, specs, design, tasks) — that is
  what `/codev-propose` does;
- touch an already-archived change under `changes/archive/`.

#### Scenario: Refusal to write a missing artifact

- **GIVEN** a change whose `design.md` does not exist yet
- **WHEN** the user types `/codev-update design "add decision
  Z"`
- **THEN** the skill refuses
- **AND** explicitly invites the user to run `/codev-propose` to
  create the artifact

#### Scenario: Refusal of an archived change

- **GIVEN** a change living under `changes/archive/2026-09-09-<name>/`
- **WHEN** the user types `/codev-update proposal --change
  <archived-name>`
- **THEN** the skill refuses
- **AND** reminds that an archived change is history; correcting it
  requires un-archiving it by hand
