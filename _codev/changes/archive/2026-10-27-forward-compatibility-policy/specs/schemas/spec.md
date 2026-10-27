## MODIFIED Requirements

### Requirement: A schema file is validated before use

A `schema.yaml` SHALL be refused with `schema_unreadable` when it is not
valid YAML, and with `schema_invalid`, naming the schema and the reason,
when:

- `name` is empty or not kebab-case;
- `artifacts` is empty;
- an artifact id is not kebab-case or is declared twice;
- an artifact has an empty `generates`;
- an artifact requires an artifact that does not exist, or itself;
- the dependencies form a cycle — the message names the artifacts
  involved;
- `apply.requires` is empty or names an artifact that does not exist;
- `apply.tracks` is empty.

`version` defaults to `1`. A command that needs a refused schema fails with
its code; `codev schemas` reports it as `schema_unusable`.

A key a project schema holds that codev does not know — at the top level,
in an artifact, in `apply` or in an artifact's `role:` — SHALL be ignored,
and the rest of the schema used, with a warning `schema_unknown_key` per
key. The warning MUST name the key, the schema's file and its line, say
that the schema may be written for a newer codev, quote the project's
`codev_version` when set, and point to `codev self-update`. Commands that
report configuration warnings MUST report it the same way, in their human
output and in the `status` array of their JSON output, for the schema
they use; `codev schemas` lists the schema as usable. When it validates
the whole project, `codev validate` SHALL add one `warning` finding
`schema_unknown_key` per key, located at that line of the schema's file,
to the item of kind `config`, which is then reported. The built-in schema
holds no key codev does not know.

#### Scenario: A minimal valid schema

- **GIVEN** a schema with a name, two artifacts, `apply.requires` and
  `apply.tracks`, and no `version`
- **WHEN** it is read
- **THEN** it is accepted with version `1`

#### Scenario: An unknown key in a project schema

- **GIVEN** a project schema `lite` holding `artifacts`, `apply` and the
  unknown top-level key `gates` on line 12, and a change `quick` using it
- **WHEN** the user runs `codev status --change quick --json`
- **THEN** the command succeeds and lists the artifacts of `lite`
- **AND** `status` holds a `schema_unknown_key` warning naming `gates`,
  `_codev/schemas/lite/schema.yaml` and `codev self-update`

#### Scenario: A misspelled key in a project schema fails strict validation

- **GIVEN** a project schema `lite` whose artifact `brief` holds the key
  `require` on line 9, and no other finding
- **WHEN** the user runs `codev validate --strict --json`
- **THEN** the `config` item holds a `warning` finding `schema_unknown_key`
  naming `require`, located at line 9 of `_codev/schemas/lite/schema.yaml`
- **AND** the exit code is 1

#### Scenario: A schema that is not YAML

- **GIVEN** a project schema whose `schema.yaml` is not valid YAML
- **WHEN** a command reads it
- **THEN** it is refused with `schema_unreadable`

#### Scenario: Structural defects

- **WHEN** a schema requires a missing artifact, declares an artifact twice,
  declares no artifact, has an empty `apply.requires`, or has a cycle
  between `a` and `b`
- **THEN** each is refused with `schema_invalid`
- **AND** the cycle message names `a` and `b`

### Requirement: A schema artifact may declare its role

An artifact of a `schema.yaml` MAY declare a `role:` mapping with an `id`,
a `name` mapping from language codes to display names, a `question` and a
`done_when` list. The schema SHALL be refused with `schema_unreadable` when
the mapping misses `id`, `name` or `question`, and with `schema_invalid`,
naming the schema and the reason, when:

- a role id is not kebab-case;
- two artifacts declare the same role id;
- a role id is the id of a built-in step role;
- a role's `name` has no `en` entry.

An artifact without `role:` has no role, and the rest of the schema is
unaffected. Another key in the mapping is ignored with a
`schema_unknown_key` warning, as any unknown key of a project schema is.

#### Scenario: A schema with an artifact role

- **GIVEN** a project schema whose artifact `brief` declares
  `role: { id: editor, name: { en: Editor }, question: "Is the brief clear?", done_when: ["One page at most"] }`
- **WHEN** the user runs `codev roles --at plan.write --json`
- **THEN** `roles` holds `editor` with `artifact: "brief"` and the criterion
  `One page at most` with the origin `schema:<the schema's name>`

#### Scenario: A role id taken by a built-in step role

- **GIVEN** a project schema whose artifact declares `role: { id: skeptic, … }`
- **WHEN** a command reads the schema
- **THEN** the schema is refused with `schema_invalid`, naming `skeptic`

#### Scenario: A role without an English name

- **GIVEN** a project schema whose artifact role has `name: { fr: Rédacteur }`
- **WHEN** a command reads the schema
- **THEN** the schema is refused with `schema_invalid`, naming the role

#### Scenario: An unknown key in an artifact role

- **GIVEN** a project schema whose artifact `brief` declares
  `role: { id: editor, name: { en: Editor }, question: "Is the brief clear?", model: sonnet }`
- **WHEN** the user runs `codev roles --at plan.write --json`
- **THEN** `roles` holds `editor`
- **AND** `status` holds a `schema_unknown_key` warning naming `model`
