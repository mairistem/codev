## ADDED Requirements

### Requirement: `codev instructions` names the existing requirements the change likely touches

Once the change folder holds its `proposal.md`, `codev instructions` SHALL
give, for each artifact that requires the proposal directly in the
change's schema — `specs` and `design` in `spec-driven` —, the requirements
of the project's main specs most relevant to the proposal's text — its
`Why` and `What Changes` sections, its whole text when it has neither —,
ranked as `codev search` ranks them, at most 10. With `--json`, the
document SHALL carry `related`: one entry per requirement, the most
relevant first, with `capability`, `requirement` and `excerpt`, the
excerpt at most 160 characters of the requirement's description.

`related` SHALL be an empty array for any other artifact — so that the
calls that only need the context and the rules, such as `/codev-apply`'s
and the reviews', read no main spec —, when `proposal.md` does not exist
yet, when the project has no main spec, when the proposal's text holds no
word to rank once its common words are removed, when nothing matches, and
in a failure document. A main spec that cannot be read SHALL leave it out
of the ranking and add a warning naming it to `status`; the command still
succeeds. The decisions in effect stay in `decisions` and are not
repeated in `related`.

The human section `Existing requirements this change likely touches:`
SHALL print one line per requirement as `<capability>::<requirement> —
<excerpt>`, then a line saying that one of them is read with
`codev show <capability>::<requirement>`, and that a requirement the
change alters is modified, in a `MODIFIED` block copied from its spec,
rather than added again.

#### Scenario: The specs of a change about decisions

- **GIVEN** a project whose spec `decisions` holds the requirement
  `Creating a decision through the CLI`, and a change whose `proposal.md`
  says it lets a user create a decision as a draft from the command line
- **WHEN** the user runs `codev instructions specs --change draft-decisions --json`
- **THEN** `related` lists `decisions` / `Creating a decision through the CLI`
  with an excerpt
- **AND** it holds at most 10 entries

#### Scenario: The artifacts that do not require the proposal

- **GIVEN** the same change
- **WHEN** the user runs `codev instructions proposal --change draft-decisions --json`,
  or `codev instructions tasks --change draft-decisions --json`
- **THEN** `related` is an empty array
- **AND** no main spec is read

#### Scenario: A proposal not written yet

- **GIVEN** a change whose folder holds no `proposal.md`
- **WHEN** the user runs `codev instructions specs --change draft-decisions --json`
- **THEN** `related` is an empty array

#### Scenario: A proposal with nothing to rank

- **GIVEN** a change whose `proposal.md` is empty
- **WHEN** the user runs `codev instructions specs --change draft-decisions --json`
- **THEN** `related` is an empty array and the command succeeds

#### Scenario: A main spec that cannot be read

- **GIVEN** a project whose spec `billing` cannot be read and whose spec
  `decisions` matches the proposal
- **WHEN** the user runs `codev instructions specs --change draft-decisions --json`
- **THEN** `related` lists the requirements of `decisions`
- **AND** `status` holds a warning naming `billing`, and the command exits
  with code 0

#### Scenario: The human output

- **GIVEN** the change of the first scenario
- **WHEN** the user runs `codev instructions design --change draft-decisions`
- **THEN** the output has the section `Existing requirements this change
  likely touches:` with the line
  `decisions::Creating a decision through the CLI — …`
- **AND** a line naming `codev show <capability>::<requirement>` and saying
  that a requirement the change alters is modified, not added again

#### Scenario: A project without specs

- **GIVEN** a project whose `_codev/specs/` is empty, and a change whose
  `proposal.md` exists
- **WHEN** the user runs `codev instructions specs --change first --json`
- **THEN** `related` is an empty array and the human output has no
  `Existing requirements` section

## MODIFIED Requirements

### Requirement: `codev instructions` describes one artifact of a change

`codev instructions [ARTIFACT] --change <name>` SHALL describe the named
artifact of the change's schema or, without `ARTIFACT`, the first artifact
whose state is `ready` in the schema's dependency order. With `--json`, the
document SHALL carry:

- `changeName`, `schemaName`, `artifact` and `description` (the schema's,
  or `null`);
- `resolvedOutputPath` — the change folder joined with the artifact's
  `generates`, kept as a glob when `generates` is one;
- `instruction` and `template` — the schema's instruction and the content of
  its template file, each `null` when the schema defines none;
- `language` — the `language:` of the project configuration, `en` when
  absent;
- `dependencies` — one entry per artifact it requires, with `id`, `path`
  (the change folder joined with that artifact's `generates`) and `done`,
  `true` when that artifact is `done` or `skipped`;
- `unlocks` — the artifacts that require it, in declaration order;
- `decisions` — always present; its content is specified by the
  `decisions` capability;
- `skipped` — `true` when the change's `skip_specs` disables the artifact;
- `related` — the existing requirements the change likely touches, as
  "`codev instructions` names the existing requirements the change likely
  touches" says; an empty array when there are none;
- `role` — the artifact's role as the `roles` capability resolves it
  (`id`, `name`, `kind`, `at`, `artifact`, `question`, `when`,
  `whenOrigin`, `disableable`, `doneWhen`), or `null` when the schema
  declares no role for the artifact;
- `status` — the configuration warnings.

The command reads files and writes none. It exits with code 0.

#### Scenario: The next artifact to write

- **GIVEN** a `spec-driven` change whose `proposal.md` exists
- **WHEN** the user runs `codev instructions --change add-auth --json`
- **THEN** `artifact` is `specs`
- **AND** `resolvedOutputPath` ends with `_codev/changes/add-auth/specs/**/*.md`
- **AND** `dependencies` is one entry `proposal` with `done: true` and the
  path of `proposal.md`, and `unlocks` is `["tasks"]`

#### Scenario: A named artifact carries the schema's guidance

- **GIVEN** a `spec-driven` change
- **WHEN** the user runs `codev instructions proposal --change add-auth --json`
- **THEN** `instruction` is the `proposal` instruction of the schema and
  `template` its template, which holds `## Why`
- **AND** `resolvedOutputPath` ends with `_codev/changes/add-auth/proposal.md`

#### Scenario: Dependencies not written yet

- **GIVEN** a `spec-driven` change with no artifact written
- **WHEN** the user runs `codev instructions tasks --change add-auth --json`
- **THEN** `dependencies` lists `specs` and `design`, both with
  `done: false`

#### Scenario: The configured language

- **GIVEN** a configuration with `language: fr`
- **WHEN** the user runs `codev instructions proposal --change add-auth --json`
- **THEN** `language` is `fr`
- **AND** without `language:` in the configuration it is `en`

#### Scenario: A project schema without instruction or template

- **GIVEN** a change using a project schema whose artifact `brief` declares
  neither `instruction` nor `template`
- **WHEN** the user runs `codev instructions brief --change quick --json`
- **THEN** `instruction` and `template` are `null`

#### Scenario: The artifact's role

- **GIVEN** a `spec-driven` change and a configuration where `qa` has the
  extra criterion `Every error code is named`
- **WHEN** the user runs `codev instructions specs --change add-auth --json`
- **THEN** `role.id` is `qa` and `role.question` is the QA analyst's
  question
- **AND** `role.doneWhen` ends with `Every error code is named` with the
  origin `project`, after the schema's criteria

#### Scenario: An artifact without a role

- **GIVEN** a change using a project schema whose artifact `brief` declares
  no role
- **WHEN** the user runs `codev instructions brief --change quick --json`
- **THEN** `role` is `null`

### Requirement: The human output of `codev instructions` reads top to bottom

Without `--json`, `codev instructions` SHALL print, in this order: the line
`Artifact: <id> (change `<name>`, schema <schema>)`, `Write to:` with the
resolved output path, `Language:` with the language; then, each only when
not empty, `Read before writing:` with one line per dependency marked `[x]`
when done and `[ ]` otherwise, `Existing requirements this change likely
touches:` with one line per related requirement, `Project context:` and
`Rules for this artifact:` with one line per block prefixed by its origin in
parentheses, `Role:` with the role's name and id, its question and its
criteria each prefixed by its origin in parentheses, `Decisions in effect:`,
`Instruction:`, `Template:`, and `Unlocks:`.

#### Scenario: Human instructions for the specs artifact

- **GIVEN** a configuration with a context and a rule for `specs`, and a
  change whose `proposal.md` exists
- **WHEN** the user runs `codev instructions specs --change add-auth`
- **THEN** the output starts with `Artifact: specs (change `add-auth`,
  schema spec-driven)`
- **AND** it shows `[x] proposal`, the context and the rule each prefixed
  with `(project)`, the `Role:` section naming `QA analyst (qa)`, the
  instruction, the template and `Unlocks: tasks`, in that order

#### Scenario: No empty section

- **GIVEN** a configuration without context nor rules
- **WHEN** the user runs `codev instructions proposal --change add-auth`
- **THEN** the output has no `Read before writing:`, `Project context:`,
  `Rules for this artifact:` nor `Decisions in effect:` heading
