## ADDED Requirements

### Requirement: A schema artifact may declare its role

An artifact of a `schema.yaml` MAY declare a `role:` mapping with an `id`,
a `name` mapping from language codes to display names, a `question` and a
`done_when` list. The schema SHALL be refused with `schema_unreadable` when
the mapping holds an unknown key or misses `id`, `name` or `question`, and
with `schema_invalid`, naming the schema and the reason, when:

- a role id is not kebab-case;
- two artifacts declare the same role id;
- a role id is the id of a built-in step role;
- a role's `name` has no `en` entry.

An artifact without `role:` has no role, and the rest of the schema is
unaffected.

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
