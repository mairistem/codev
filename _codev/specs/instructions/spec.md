# Instructions Specification

## Purpose

Give the person or the skill writing an artifact everything it needs in one
call — where to write, the schema's guidance and template, the language,
the project's context and rules, what to read first and what comes next —
so that codev assembles the context and never writes an artifact itself.

## Requirements

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

### Requirement: Context and rules come as blocks with their origin, from general to specific

`context` SHALL hold the non-blank `context:` of each inherited source, in
the order the sources are declared, then the project's; `rules` SHALL hold,
in the same order, the non-blank entries of `rules.<artifact>` for the
artifact described only. Each block SHALL carry its `origin` — `project`,
`path:<declared path>` or `git:<url>` — and its `text`, trimmed. An artifact
without rules gets an empty `rules` array; the context applies to every
artifact.

#### Scenario: Inherited blocks come before the project's

- **GIVEN** a project inheriting from `~/shared`, whose configuration has a
  context and a rule for `specs`, and a project configuration with its own
  context and rule for `specs`
- **WHEN** the user runs `codev instructions specs --change add-auth --json`
- **THEN** `context` is the block of `path:~/shared` then the block of
  `project`
- **AND** `rules` is the shared rule then the project rule

#### Scenario: Rules of other artifacts are left out

- **GIVEN** a configuration with rules for `specs` only, and a context
- **WHEN** the user runs `codev instructions proposal --change add-auth --json`
- **THEN** `rules` is empty
- **AND** `context` holds the project block

### Requirement: A skipped artifact is reported as not to be created

When the change's `skip_specs` disables the requested artifact,
`codev instructions` SHALL succeed with `skipped: true` and a warning
`artifact_skipped` saying its files must not be created. The human output
SHALL then print the artifact, its output path and its language, followed
by a line saying the artifact is disabled by `skip_specs` and must not be
created, and nothing else.

#### Scenario: Instructions for a skipped spec artifact

- **GIVEN** a change with `skip_specs: true`
- **WHEN** the user runs `codev instructions specs --change refactor --json`
- **THEN** `skipped` is `true` and `status` holds the warning
  `artifact_skipped`
- **AND** the human output says the artifact must not be created and shows
  no instruction or template

### Requirement: The human output of `codev instructions` reads top to bottom

Without `--json`, `codev instructions` SHALL print, in this order: the line
`Artifact: <id> (change `<name>`, schema <schema>)`, `Write to:` with the
resolved output path, `Language:` with the language; then, each only when
not empty, `Read before writing:` with one line per dependency marked `[x]`
when done and `[ ]` otherwise, `Project context:` and
`Rules for this artifact:` with one line per block prefixed by its origin in
parentheses, `Decisions in effect:`, `Instruction:`, `Template:`, and
`Unlocks:`.

#### Scenario: Human instructions for the specs artifact

- **GIVEN** a configuration with a context and a rule for `specs`, and a
  change whose `proposal.md` exists
- **WHEN** the user runs `codev instructions specs --change add-auth`
- **THEN** the output starts with `Artifact: specs (change `add-auth`,
  schema spec-driven)`
- **AND** it shows `[x] proposal`, the context and the rule each prefixed
  with `(project)`, the instruction, the template and `Unlocks: tasks`,
  in that order

#### Scenario: No empty section

- **GIVEN** a configuration without context nor rules
- **WHEN** the user runs `codev instructions proposal --change add-auth`
- **THEN** the output has no `Read before writing:`, `Project context:`,
  `Rules for this artifact:` nor `Decisions in effect:` heading

### Requirement: `codev instructions` fails with stable codes

`codev instructions` SHALL fail with exit code 1 and:

- `unknown_artifact` when `ARTIFACT` is not an artifact of the change's
  schema, naming the artifact and the schema;
- `no_artifact_ready` when `ARTIFACT` is omitted and no artifact is
  `ready`, pointing to `codev status --change <name>`;
- `template_not_found` when the artifact declares a template that its
  project schema's `templates/` folder does not hold;
- the codes of the change choice specified by the `changes` capability.

With `--json`, the document carries the code in `status`, with `artifact`,
`changeName` and the other single values `null`, the arrays empty, and
`language` set as on success — the `language:` of the project
configuration, `en` when absent — or `en` when the configuration cannot be
read, so that the failure keeps every field of the success document.

#### Scenario: Unknown artifact

- **WHEN** the user runs `codev instructions sketch --change add-auth --json`
- **THEN** the exit code is 1
- **AND** `status[0].code` is `unknown_artifact`, its message names
  `sketch` and `spec-driven`, and `artifact` is `null`

#### Scenario: The failure document keeps the configured language

- **GIVEN** a configuration with `language: fr`
- **WHEN** the user runs `codev instructions sketch --change add-auth --json`
- **THEN** the exit code is 1
- **AND** `language` is `fr`

#### Scenario: The failure document outside a project

- **GIVEN** a directory outside any codev project
- **WHEN** the user runs `codev instructions --json`
- **THEN** `status[0].code` is `no_codev_root`
- **AND** `language` is `en`

#### Scenario: Nothing left to write

- **GIVEN** a change where every artifact is written
- **WHEN** the user runs `codev instructions --change add-auth`
- **THEN** the command fails with `no_artifact_ready`

#### Scenario: A template missing from a project schema

- **GIVEN** a project schema whose artifact `brief` declares
  `template: brief.md` and no `templates/brief.md` file
- **WHEN** the user runs `codev instructions brief --change quick`
- **THEN** the command fails with `template_not_found`
