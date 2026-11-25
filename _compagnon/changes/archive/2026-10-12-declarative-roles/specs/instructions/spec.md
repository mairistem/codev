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
when done and `[ ]` otherwise, `Project context:` and
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

