## Purpose

Describes the contract of the skills that codev installs in Claude Code:
their name, what they must do, what they are not allowed to do, and how
their frontmatter guarantees these promises. Entries are added as the
changes that introduce each workflow land — one ADDED per workflow.

## ADDED Requirements

### Requirement: Skill `apply` guides the implementation of a change

The codev catalog SHALL expose an `apply` workflow — installed under
`.claude/skills/codev-apply/SKILL.md`, invocable as `/codev-apply` — whose
role is to process the unchecked tasks of a change's `tasks.md`, in file
order, checking each box as it goes.

#### Scenario: Implementing a change with a single active one

- **GIVEN** a project with a single active change whose `tasks.md` has two
  unchecked tasks
- **WHEN** the user types `/codev-apply`
- **THEN** the skill implicitly resolves the active change
- **AND** implements the first task then checks it
- **AND** implements the second task then checks it

#### Scenario: Resuming after an interruption

- **GIVEN** a `tasks.md` where the first task is already checked `- [x]` and
  the second is not
- **WHEN** the user types `/codev-apply`
- **THEN** the skill skips the already-checked task
- **AND** starts with the first unchecked task

#### Scenario: Ambiguity requires an explicit choice

- **GIVEN** two active changes
- **WHEN** the user types `/codev-apply` without a name
- **THEN** the skill asks which one to apply, listing both names

### Requirement: Skill `apply` respects the change's boundaries

The `apply` workflow MUST confine itself to what is needed to check off the
tasks of the named change: it MUST NOT modify other changes, MUST NOT
archive or sync on its own, and MUST stop as soon as a task is ambiguous
or blocked rather than guess.

#### Scenario: Refusal to archive from apply

- **GIVEN** a change whose tasks are all checked
- **WHEN** the user types `/codev-apply`
- **THEN** the skill reports that the change is ready to be archived
- **AND** explicitly invites running `/codev-archive` or `codev archive`
  as a separate next step

#### Scenario: Ambiguous task interrupts the flow

- **GIVEN** a `tasks.md` containing a task whose wording admits several
  interpretations that would materially change the result
- **WHEN** the skill reaches that task
- **THEN** the skill asks the user for clarification before implementing
- **AND** does not check the task until the clarification is obtained

### Requirement: Frontmatter contract of a codev skill

Every skill shipped by codev MUST carry a valid YAML frontmatter whose
`name` matches the name of the `.claude/skills/<name>/` folder, whose
`allowed-tools` field includes at least `Bash(codev:*)`, and whose
`metadata.version` matches the version of the binary that generated it.

#### Scenario: Frontmatter parsed by a third-party YAML reader

- **GIVEN** a skill shipped by the current version of the binary
- **WHEN** its frontmatter is extracted and passed to a standard YAML parser
- **THEN** the parser returns `name`, `allowed-tools` and `metadata.version`
  without error
- **AND** `metadata.version` equals the version the binary announces

#### Scenario: Manual edit detected on update

- **GIVEN** a skill whose body a user has edited by hand, without
  changing its version
- **WHEN** the user reruns `codev update` without `--force`
- **THEN** the skill is not overwritten
- **AND** the update report flags it as preserved
