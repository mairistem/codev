## ADDED Requirements

### Requirement: The template is served in the project's keyword language

`codev instructions` SHALL give as `template` the artifact's template in the
keyword language. The embedded `spec-driven` schema SHALL hold its four
templates in every language the keyword catalog ships, each writing every
catalog keyword it holds with that language's first text, and its other
headings, guidance comments and the `**WHEN**` / `**THEN**` scenario lines
in that language. Identifiers stay as codev defines them in every
language: the domain names of the proposal's domain list (`security`,
`personal-data`…), so that a template's domain list is read without
error. A project schema SHALL be read from
`templates/<keywordLanguage>/<file>` when that file exists, and from
`templates/<file>` otherwise, so that a project schema written before this
change is served unchanged.

#### Scenario: The French proposal template

- **GIVEN** a `spec-driven` change and a configuration with `language: fr`
- **WHEN** the user runs `codev instructions proposal --change add-auth --json`
- **THEN** `template` starts with `# Proposition :` and holds
  `## Changements`, `## Domaines` and `## Note de version`
- **AND** it holds no `## What Changes`

#### Scenario: The French spec template

- **GIVEN** a `spec-driven` change and a configuration with `language: fr`
- **WHEN** the user runs `codev instructions specs --change add-auth --json`
- **THEN** `template` holds `## Exigences AJOUTÉES`, `### Exigence :`,
  `#### Scénario :` and the word `DOIT`

#### Scenario: Every embedded template uses its language's keywords

- **GIVEN** the embedded templates of every catalog language
- **WHEN** codev reads each template's structural lines with the keyword
  catalog
- **THEN** every keyword it holds is recognized as the text of its key in
  the template's own language

#### Scenario: A language without a catalog gets the English template

- **GIVEN** a `spec-driven` change and a configuration with `language: pt-BR`
- **WHEN** the user runs `codev instructions proposal --change add-auth --json`
- **THEN** `language` is `pt-BR`, `keywordLanguage` is `en`
- **AND** `template` holds `## What Changes`

#### Scenario: A project schema with a template per language

- **GIVEN** a configuration with `language: fr`, and a project schema whose
  artifact `brief` declares `template: brief.md`, with both
  `templates/fr/brief.md` and `templates/brief.md`
- **WHEN** the user runs `codev instructions brief --change quick --json`
- **THEN** `template` is the content of `templates/fr/brief.md`

#### Scenario: A project schema with a single template

- **GIVEN** a configuration with `language: fr`, and a project schema whose
  artifact `brief` declares `template: brief.md`, with only
  `templates/brief.md`
- **WHEN** the user runs `codev instructions brief --change quick --json`
- **THEN** `template` is the content of `templates/brief.md`

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
  its template file in the keyword language, as "The template is served in
  the project's keyword language" says, each `null` when the schema defines
  none;
- `language` — the `language:` of the project configuration, `en` when
  absent: the language of the prose;
- `keywordLanguage` — the language the structural keywords are written in,
  as the `keyword-languages` capability resolves it from `language`;
- `keywords` — an object giving, for every key of the keyword catalog, the
  text codev writes for it in `keywordLanguage` (the first text of the
  key), and for `normative` and `none` the list of all its texts;
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
- **THEN** `language` and `keywordLanguage` are `fr`
- **AND** `keywords.requirement` is `### Exigence :` and
  `keywords.normative` is `["DOIT", "DOIVENT"]`
- **AND** without `language:` in the configuration both are `en`, and
  `keywords.requirement` is `### Requirement:`

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
resolved output path, `Language:` with the language followed by the
language of the structural keywords — `Language: fr — prose and structural
keywords in fr` when they are the same, `Language: pt-BR — prose in pt-BR,
structural keywords in en` otherwise; then, each only when
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
- **AND** with no `language:` in the configuration, its third line is
  `Language: en — prose and structural keywords in en`
- **AND** it shows `[x] proposal`, the context and the rule each prefixed
  with `(project)`, the `Role:` section naming `QA analyst (qa)`, the
  instruction, the template and `Unlocks: tasks`, in that order

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
  project schema's `templates/` folder does not hold, neither under the
  keyword language's folder nor at its root;
- the codes of the change choice specified by the `changes` capability.

With `--json`, the document carries the code in `status`, with `artifact`,
`changeName` and the other single values `null`, the arrays empty, and
`language`, `keywordLanguage` and `keywords` set as on success — from the
`language:` of the project configuration, `en` when absent — or for `en`
when the configuration cannot be read, so that the failure keeps every
field of the success document.

#### Scenario: Unknown artifact

- **WHEN** the user runs `codev instructions sketch --change add-auth --json`
- **THEN** the exit code is 1
- **AND** `status[0].code` is `unknown_artifact`, its message names
  `sketch` and `spec-driven`, and `artifact` is `null`

#### Scenario: The failure document keeps the configured language

- **GIVEN** a configuration with `language: fr`
- **WHEN** the user runs `codev instructions sketch --change add-auth --json`
- **THEN** the exit code is 1
- **AND** `language` and `keywordLanguage` are `fr`

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

- **GIVEN** a configuration with `language: fr`, and a project schema
  whose artifact `brief` declares `template: brief.md` with neither
  `templates/fr/brief.md` nor `templates/brief.md`
- **WHEN** the user runs `codev instructions brief --change quick`
- **THEN** the command fails with `template_not_found`
