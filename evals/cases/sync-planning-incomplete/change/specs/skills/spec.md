## RENAMED Requirements

- FROM: `### Requirement: The review skills request the sub-agent tool only in isolated mode`
- TO: `### Requirement: Skills request the sub-agent tool only when a sub-agent may run`

## MODIFIED Requirements

### Requirement: The `apply` skill guides the implementation of a change

The codev catalog SHALL expose an `apply` workflow — installed under
`.claude/skills/codev-apply/SKILL.md`, invocable as `/codev-apply` — whose
role is to process the unchecked tasks of a change's `tasks.md`, in file
order, checking each box as it goes.

When `codev roles --at code.implement` gives the `developer` a model, the
skill SHALL hand the tasks to one sub-agent started on that model, with
the change's name, the developer's criteria, the project's `rules` and
`context` for the tasks, and any instruction the session itself runs
under, such as a run without a user: the sub-agent works through the
tasks as the skill's own steps say — study, implement, verify, check
off, stop on a material ambiguity or a technical blocker — and reports
the tasks it checked and, if it stopped, the task with the
interpretations or the blocker. The session re-reads `tasks.md`; it
relays a stop to the user as the skill says, and once the user answers —
or, in a run without a user, once the session has made the most
reasonable assumption — it starts a new sub-agent on the developer's
model for the remaining tasks, with that answer; the session never
implements a task itself while the developer has a model, but in the
fallback below. When every task is checked, the session goes on to the
review and the summary itself. Without a model on the `developer`, the
skill implements in the session.

A sub-agent fails when the platform cannot start it or stops it, or when
its answer reports neither checked tasks nor a stop; a reported stop is
not a failure. A failed sub-agent is retried once, the new one first
inspecting the working tree for work left on the first unchecked task;
after a second failure the session implements the remaining tasks itself
and says so in its summary.

#### Scenario: Implementing a change when only one is active

- **GIVEN** a project with a single active change whose `tasks.md` carries
  two unchecked tasks
- **WHEN** the user types `/codev-apply`
- **THEN** the skill implicitly resolves the active change
- **AND** implements the first task, then checks it
- **AND** implements the second task, then checks it

#### Scenario: Resuming after an interruption

- **GIVEN** a `tasks.md` in which the first task is already checked `- [x]`
  and the second is not
- **WHEN** the user types `/codev-apply`
- **THEN** the skill skips the already checked task
- **AND** starts with the first unchecked task

#### Scenario: Ambiguity requires an explicit choice

- **GIVEN** two active changes
- **WHEN** the user types `/codev-apply` without a name
- **THEN** the skill asks which one to apply, listing both names

#### Scenario: The developer's model implements the tasks

- **GIVEN** a project with `roles: { developer: { model: sonnet } }` and a
  change with three unchecked tasks
- **WHEN** the user types `/codev-apply`
- **THEN** the skill starts one sub-agent on `sonnet` that implements and
  checks the three tasks
- **AND** the session reviews the change's code and writes the summary

#### Scenario: The sub-agent stops on an ambiguity

- **GIVEN** a project whose `developer` has a model, and a sub-agent that
  stops on task 2.1 with two interpretations
- **WHEN** the user picks one
- **THEN** the session starts a new sub-agent on the developer's model for
  task 2.1 and the tasks after it, with the user's choice

#### Scenario: The sub-agent cannot be started

- **GIVEN** a project whose `developer` has a model, and a platform that
  fails to start the sub-agent twice
- **WHEN** the user types `/codev-apply`
- **THEN** the session implements the remaining tasks itself and its
  summary says the developer's model could not be used

#### Scenario: Without a model, nothing changes

- **GIVEN** a project whose `developer` has no model
- **WHEN** the user types `/codev-apply`
- **THEN** the skill implements the tasks in the session, without a
  sub-agent

### Requirement: Skills request the sub-agent tool only when a sub-agent may run

When the project's review mode is `isolated`, the `propose` and `apply`
skills SHALL list Claude Code's sub-agent tool, `Agent`, in their
`allowed-tools`; when the project's `developer` has a model, the `apply`
skill SHALL list it too, whatever the review mode. Otherwise no skill
SHALL list it, and no other skill ever lists it. The body of both skills SHALL carry the isolated
review procedure in either mode, so that a change that overrides the mode
is reviewed as it asks.

#### Scenario: Isolated mode adds the sub-agent tool

- **GIVEN** a configuration with `review: { mode: isolated }`
- **WHEN** the user runs `codev update`
- **THEN** the `allowed-tools` of `codev-propose` and `codev-apply` end
  with `Agent`
- **AND** no other skill lists `Agent`

#### Scenario: Inline mode leaves the tools as they were

- **GIVEN** a configuration without `review:`
- **WHEN** the user runs `codev update`
- **THEN** no skill lists `Agent` in its `allowed-tools`
- **AND** the `propose` and `apply` bodies still describe the isolated
  review

#### Scenario: A developer model adds the sub-agent tool to apply

- **GIVEN** a configuration without `review:` and with `roles: {
  developer: { model: sonnet } }`
- **WHEN** the user runs `codev update`
- **THEN** the `allowed-tools` of `codev-apply` end with `Agent`
- **AND** `codev-propose` does not list it
