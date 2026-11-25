## ADDED Requirements

### Requirement: The skills suggest the next step in plain words first

Whenever the `propose`, `apply`, `update`, `sync`, `archive` or `configure`
skill tells the user what to do next — the next step after it finishes, the
way out of a refusal, the fix for a problem it reports — it SHALL first give
the words to say to Claude, in plain language and in the language the user
writes in, then MAY give the slash command or the `compagnon` command as an
optional shortcut. Every command these skills named before this requirement
MUST still be named, as the shortcut. A command that is not a next step for
the user — a file marker such as `# written by /compagnon-configure`, or a
command the skill runs itself — is not concerned.

#### Scenario: Apply suggests archiving in plain words

- **GIVEN** a change whose tasks are all checked
- **WHEN** the user asks Claude, in French, to implement the change
- **THEN** the final summary invites the user to say, in French, that the
  change can be archived
- **AND** gives `/compagnon-archive` after it, as a shortcut

#### Scenario: A refusal leads with a plain request

- **GIVEN** a change whose planning is not complete
- **WHEN** the user asks Claude to implement it
- **THEN** the refusal suggests asking Claude to finish the plan, in plain
  words
- **AND** names `/compagnon-propose` as the shortcut

#### Scenario: A diagnostic command stays, as the shortcut

- **GIVEN** a change that fails validation
- **WHEN** the user asks Claude to archive it
- **THEN** the reply offers to show the errors in plain words
- **AND** still names `compagnon validate <name>`

#### Scenario: Every slash command in the six skills is a shortcut

- **GIVEN** the bodies of the six skills, as Compagnon renders them
- **WHEN** the test suite of `compagnon-agents` runs
- **THEN** each includes the shared rule for suggesting next steps
- **AND** every line naming a `/compagnon-…` command presents it as a
  shortcut, apart from the `# written by /compagnon-configure` marker

### Requirement: Skill descriptions name the plain requests that start them

The description of each of the eight skills — the text Claude Code matches
against a request to choose a skill — SHALL name, besides what the skill
does, examples of plain requests that should start it, at least one in
English and one in French. Each description MUST stay within 1024
characters.

#### Scenario: Every description names English and French requests

- **GIVEN** the catalog of the eight workflows
- **WHEN** the test suite of `compagnon-agents` runs
- **THEN** each description holds at least one quoted English request and
  one quoted French request
- **AND** none exceeds 1024 characters

#### Scenario: A description without a French request is caught

- **GIVEN** a description whose only example request is in English
- **WHEN** the check runs on it
- **THEN** it fails naming the workflow
