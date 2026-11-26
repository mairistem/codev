## MODIFIED Requirements

### Requirement: The skills suggest the next step in plain words first

Whenever the `propose`, `apply`, `update`, `sync`, `archive`, `configure`,
`explore` or `onboard` skill tells the user what to do next — the next step after it finishes, the
way out of a refusal, the fix for a problem it reports — it SHALL first give
the words to say to Claude, in plain language and in the language the user
writes in, then MAY give the slash command or the `compagnon` command as an
optional shortcut. Every command these skills named before this requirement
that exists — a skill of the catalog or a `compagnon` command — MUST still be
named, as the shortcut; a command that names no skill, such as
`/compagnon-continue`, is dropped. A command that is not a next step for
the user — a file marker such as `# written by /compagnon-configure`, or a
command the skill runs itself, or the `fix` of a line of `compagnon doctor`'s
output that `onboard` relays verbatim, after a few plain words of its own
saying what is missing — is not concerned.

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

#### Scenario: Every slash command in the eight skills is a shortcut

- **GIVEN** the bodies of the eight skills, as Compagnon renders them
- **WHEN** the test suite of `compagnon-agents` runs
- **THEN** each includes the shared rule for suggesting next steps
- **AND** every line naming a `/compagnon-…` command presents it as a
  shortcut, apart from the `# written by /compagnon-configure` marker and
  the doctor fix lines `onboard` relays

#### Scenario: Onboard recommends configuring in plain words

- **GIVEN** an initialized project with a thin configuration and no
  active change
- **WHEN** the user asks Claude to show them around Compagnon
- **THEN** the recommended next action invites the user to ask Claude to
  configure Compagnon for the project, in their language
- **AND** gives `/compagnon-configure` after it, as a shortcut

#### Scenario: Explore offers to plan in plain words

- **GIVEN** an exploration that crystallizes into a change
- **WHEN** the explore skill concludes
- **THEN** it invites the user to ask Claude to plan that change, in
  their language
- **AND** gives `/compagnon-propose` after it, as a shortcut

### Requirement: A reply ends with the one request to say next

When one of the eight skills ends its reply with a next step, the reply
SHALL close on that step alone, on its last line: one plain request, quoted,
in the user's language, followed by its shortcut — one request, not a list
of options — so that Claude Code's suggested next prompt, drawn from the
conversation, is likely to be that request. A refusal or a summary that
leaves several choices to the user names them before that last line, each
with its request and shortcut; when the next step is that choice, the last
line asks which one, in plain words, without a shortcut.

#### Scenario: The shared rule asks for one closing request

- **GIVEN** the shared rule for suggesting next steps, as the eight skills
  render it
- **WHEN** the test suite of `compagnon-agents` runs
- **THEN** it asks to end the reply with the next step alone on the last
  line, one quoted request with its shortcut

#### Scenario: Onboard with several active changes

- **GIVEN** a project with two active changes
- **WHEN** the user asks Claude to show them around Compagnon
- **THEN** the reply lists both changes, each with a plain request and its
  shortcut
- **AND** its last line asks which one to continue, without a shortcut

#### Scenario: Propose closes on the request to build the plan

- **GIVEN** the propose skill as Compagnon renders it
- **WHEN** the test suite of `compagnon-agents` runs
- **THEN** its summary ends with a quoted plain request to build the plan,
  with `/compagnon-apply` as the shortcut

### Requirement: Skill descriptions name the plain requests that start them

The description of the `propose`, `apply`, `update`, `sync`, `archive`,
`configure`, `explore` and `onboard` skills — the text Claude Code matches
against a request to choose a skill — SHALL name, besides what the skill
does, examples of plain requests that should start it, at least one in
English and one in French. The French requests follow the lexicon of the
French documentation (modification, compétence, spécification, not change,
skill, spec), and the shared rule tells the skills to use that lexicon when
they say a request in French. Each description MUST stay within 1024
characters.

#### Scenario: Every description names English and French requests

- **GIVEN** the eight workflows of the catalog named by this requirement
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
