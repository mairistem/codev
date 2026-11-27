## ADDED Requirements

### Requirement: The skills suggest the next step in plain words first

Whenever the `propose`, `apply`, `update`, `sync`, `archive` or `configure`
skill tells the user what to do next — the next step after it finishes, the
way out of a refusal, the fix for a problem it reports — it SHALL first give
the words to say to Claude, in plain language and in the language the user
writes in, then MAY give the slash command or the `compagnon` command as an
optional shortcut. Every command these skills named before this requirement
that exists — a skill of the catalog or a `compagnon` command — MUST still be
named, as the shortcut; a command that names no skill, such as
`/compagnon-continue`, is dropped. A command that is not a next step for
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

### Requirement: A reply ends with the one request to say next

When one of the six skills ends its reply with a next step, the reply
SHALL close on that step alone, on its last line: one plain request, quoted,
in the user's language, followed by its shortcut — one request, not a list
of options — so that Claude Code's suggested next prompt, drawn from the
conversation, is likely to be that request. A refusal or a summary that
leaves several choices to the user names them before that last line.

#### Scenario: The shared rule asks for one closing request

- **GIVEN** the shared rule for suggesting next steps, as the six skills
  render it
- **WHEN** the test suite of `compagnon-agents` runs
- **THEN** it asks to end the reply with the next step alone on the last
  line, one quoted request with its shortcut

#### Scenario: Propose closes on the request to build the plan

- **GIVEN** the propose skill as Compagnon renders it
- **WHEN** the test suite of `compagnon-agents` runs
- **THEN** its summary ends with a quoted plain request to build the plan,
  with `/compagnon-apply` as the shortcut

### Requirement: Skill descriptions name the plain requests that start them

The description of the `propose`, `apply`, `update`, `sync`, `archive` and
`configure` skills — the text Claude Code matches
against a request to choose a skill — SHALL name, besides what the skill
does, examples of plain requests that should start it, at least one in
English and one in French. The French requests follow the lexicon of the
French documentation (modification, compétence, spécification, not change,
skill, spec), and the shared rule tells the skills to use that lexicon when
they say a request in French. Each description MUST stay within 1024
characters.

#### Scenario: Every description names English and French requests

- **GIVEN** the six workflows of the catalog named by this requirement
- **WHEN** the test suite of `compagnon-agents` runs
- **THEN** each description holds at least one quoted English request and
  one quoted French request
- **AND** none exceeds 1024 characters

#### Scenario: A French request holding an anglicism is caught

- **GIVEN** a description whose French request is « archive le change »
- **WHEN** the check runs on it
- **THEN** it fails naming the workflow and « change »

#### Scenario: A description without a French request is caught

- **GIVEN** a description whose only example request is in English
- **WHEN** the check runs on it
- **THEN** it fails naming the workflow

## MODIFIED Requirements

### Requirement: The `sync` and `archive` skills rely on the JSON contract

The `sync` and `archive` workflows MUST invoke the CLI with `--json` and
read the structured form (`SyncReportV1`, `ArchiveReportV1`) rather than
the human output — it is the public contract that Compagnon guarantees stable
within its current version, and it is what makes the skill's output
reliable.

#### Scenario: Structured rendering of creations and updates

- **GIVEN** a change whose merge creates one main spec and updates
  another
- **WHEN** the user types `/compagnon-sync`
- **THEN** the output names the two distinctly — the created file and the
  updated file — each on its own line

#### Scenario: Archive refusal detected through a stable code

- **GIVEN** a change for which `compagnon archive --json` refuses with the code
  `validation_failed` in its `status` array
- **WHEN** the user types `/compagnon-archive`
- **THEN** the skill detects the stable code in the JSON
- **AND** says, in the user's language, that the change has errors and
  offers to show them, then gives `compagnon validate <name>` as the
  shortcut — the same sentence every time, with no detail of its own
- **AND** does not parse the human message (which may be reworded without
  notice)
