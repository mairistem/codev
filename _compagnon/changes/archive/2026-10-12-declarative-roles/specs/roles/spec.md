## Purpose

Define the roles the skills play — who owns which question at which workflow
step, and the "done when" criteria they check — as data that codev serves,
that a project can switch on or off and extend, and that a change can
override.

## ADDED Requirements

### Requirement: codev knows a catalog of roles bound to workflow steps

codev SHALL know a catalog of roles. Each role has an id, a display name in
English and in French, the question it owns, its "done when" criteria, and
the workflow step it serves. The steps, in workflow order, are
`plan.understand`, `plan.investigate`, `plan.write`, `plan.trace`,
`plan.challenge`, `code.implement` and `code.review`.

The catalog is made of:

- the built-in step roles, embedded in codev: `analyst`
  (`plan.understand`), `investigator` (`plan.investigate`), `tracer`
  (`plan.trace`), `skeptic` (`plan.challenge`), `developer`
  (`code.implement`) and `newcomer` (`code.review`);
- the artifact roles declared by the schema in use, all at `plan.write`:
  with the default `spec-driven` schema, `product` (`proposal`), `qa`
  (`specs`), `architect` (`design`) and `lead` (`tasks`).

The display names are: Analyst / Analyste, Investigator / Enquêteur,
Product owner / Product owner, QA analyst / Analyste QA, Architect /
Architecte, Tech lead / Tech lead, Tracer / Contrôleur de traçabilité,
Skeptic / Contradicteur, Developer / Développeur, Newcomer / Nouvel
arrivant. Every role is active (`always`) by default. Artifact roles and
the `developer` cannot be disabled.

#### Scenario: The default catalog

- **GIVEN** a project using the `spec-driven` schema and no `roles:` key
- **WHEN** the user runs `codev roles --json`
- **THEN** `roles` lists `analyst`, `investigator`, `product`, `qa`,
  `architect`, `lead`, `tracer`, `skeptic`, `developer` and `newcomer`, in
  that order, each with `when: "always"`
- **AND** `product`, `qa`, `architect`, `lead` and `developer` have
  `disableable: false`

#### Scenario: A custom schema without roles

- **GIVEN** a project whose schema declares no artifact role
- **WHEN** the user runs `codev roles --json`
- **THEN** `roles` lists only the six built-in step roles

### Requirement: The project configures roles under `roles:`

`_codev/config.yaml` SHALL accept a top-level `roles:` mapping from a role
id to either an activation (`always` or `never`), or a mapping with an
optional `when:` (the same values, `always` when absent) and an optional
`done_when:` list. The `done_when:` entries SHALL be added after the role's
own criteria, never replacing them, each with the origin `project`; blank
entries are ignored. Any other key inside a role's mapping MUST make the
configuration invalid, like an unknown key of `changelog`. The `roles:` key
of an inherited source's configuration MUST be ignored.

A change's `change.yaml` SHALL accept a `roles:` mapping from a role id to
an activation, which overrides the project's activation of that role for
this change only.

#### Scenario: A role disabled by the project

- **GIVEN** a configuration with `roles: { newcomer: never }`
- **WHEN** the user runs `codev roles --at code.review --json`
- **THEN** `roles` is empty
- **AND** `codev roles --json` lists `newcomer` with `when: "never"` and
  `whenOrigin: "project"`

#### Scenario: Extra criteria are added after the role's own

- **GIVEN** a configuration where `skeptic` has `done_when:` with the entry
  `The rollback plan is stated`
- **WHEN** the user runs `codev roles --at plan.challenge --json`
- **THEN** the `skeptic` entry's `doneWhen` holds its built-in criteria,
  each with the origin `built-in`, followed by `The rollback plan is
  stated` with the origin `project`

#### Scenario: A change overrides the activation

- **GIVEN** a configuration with `roles: { newcomer: never }` and a change
  `add-auth` whose `change.yaml` holds `roles: { newcomer: always }`
- **WHEN** the user runs `codev roles --at code.review --change add-auth
  --json`
- **THEN** `roles` holds `newcomer` with `whenOrigin: "change"`
- **AND** without `--change`, `roles` is empty

#### Scenario: An unknown key inside a role

- **GIVEN** a configuration where `skeptic` holds the key `done-when`
- **WHEN** the user runs `codev status`
- **THEN** the command fails with the code `invalid`, naming the key

#### Scenario: Roles of an inherited source are ignored

- **GIVEN** a project inheriting from a source whose configuration holds
  `roles: { skeptic: never }`
- **WHEN** the user runs `codev roles --at plan.challenge --json`
- **THEN** `roles` holds `skeptic`

### Requirement: Invalid role settings are reported with stable codes

A role setting, in `_codev/config.yaml` or in a change's `change.yaml`,
SHALL be reported:

- with `unknown_role` when its id is neither a built-in step role nor an
  artifact role of the schema in use;
- with `invalid_role_activation` when its activation is neither `always`
  nor `never`;
- with `role_not_disableable` when it sets `never` on an artifact role or
  on the `developer`.

The message MUST name the role and the file. `codev validate` SHALL report
each one as an error finding with its code — under the `config` item for
`_codev/config.yaml`, under the change's item for its `change.yaml`. Project
settings are checked against the configured schema; a change's settings
against the change's schema. `codev status`, `codev instructions` and
`codev roles` SHALL fail with exit code 1 and the code of the first
invalid setting, the project's before the change's.

#### Scenario: An unknown role id

- **GIVEN** a configuration with `roles: { reviewer: never }`
- **WHEN** the user runs `codev validate --json`
- **THEN** the `config` item holds an error `unknown_role` naming
  `reviewer`
- **AND** `codev status --change add-auth --json` fails with
  `status[0].code` `unknown_role`

#### Scenario: An invalid activation

- **GIVEN** a configuration with `roles: { skeptic: sometimes }`
- **WHEN** the user runs `codev validate`
- **THEN** it reports the error `invalid_role_activation` naming `skeptic`
  and the accepted values `always` and `never`

#### Scenario: Disabling an artifact role

- **GIVEN** a configuration with `roles: { qa: never }`
- **WHEN** the user runs `codev instructions specs --change add-auth`
- **THEN** the command fails with `role_not_disableable` naming `qa`
- **AND** `roles: { qa: { done_when: ["..."] } }` is accepted

#### Scenario: An invalid setting in a change

- **GIVEN** a change `add-auth` whose `change.yaml` holds
  `roles: { developer: never }`
- **WHEN** the user runs `codev validate add-auth --json`
- **THEN** the item of `add-auth` holds an error `role_not_disableable`
  naming `developer` and `change.yaml`

### Requirement: `codev roles` serves the roles of a step

`codev roles [--at <step>] [--change <name>] [--json]` SHALL resolve the
roles against the change's schema and its `change.yaml` with `--change`,
otherwise against the configured schema and the project configuration
alone. With `--at`, it SHALL list the roles active at that step; without
it, every role of the catalog with its activation. It reads files, writes
none, and exits with code 0.

With `--json`, the document SHALL carry `changeName` (`null` without
`--change`), `schemaName`, `at` (`null` without `--at`), `language`,
`roles` and `status`. Each role SHALL carry `id`, `name` (its display name
in the configured `language`, English when the role has none in that
language), `kind` (`step` or `artifact`), `at`, `artifact` (the artifact id
of an artifact role, `null` otherwise), `question`, `when` (`always` or
`never`), `whenOrigin` (`default`, `project` or `change`), `disableable`,
and `doneWhen`, a list of criteria each with its `origin` (`built-in`,
`schema:<name>` or `project`) and its `text`.

Without `--json`, with `--at`, it SHALL print each active role's id and
name, its question, and its criteria each prefixed with its origin in
parentheses — or a line saying no role is active at that step. Without
`--at`, it SHALL print one line per role: id, name, step or artifact, and
activation with its origin when not the default, marking the roles that
cannot be disabled.

An `--at` that is not one of the steps SHALL fail with `unknown_step`,
naming the valid steps. With `--json`, a failure keeps the document's
shape, with `roles` empty.

#### Scenario: The roles of a step, as JSON

- **GIVEN** a project with no `roles:` key and a change `add-auth`
- **WHEN** the user runs `codev roles --at plan.trace --change add-auth
  --json`
- **THEN** `roles` holds one entry, `tracer`, with `kind: "step"`,
  `at: "plan.trace"`, `artifact: null`, `when: "always"`,
  `whenOrigin: "default"` and its criteria with the origin `built-in`

#### Scenario: Artifact roles at `plan.write`

- **WHEN** the user runs `codev roles --at plan.write --json`
- **THEN** `roles` holds `product`, `qa`, `architect` and `lead`, with
  `artifact` `proposal`, `specs`, `design` and `tasks`, and criteria with
  the origin `schema:spec-driven`

#### Scenario: Display names follow the configured language

- **GIVEN** a configuration with `language: fr`
- **WHEN** the user runs `codev roles --at plan.challenge --json`
- **THEN** the `skeptic` entry has `name: "Contradicteur"`
- **AND** with `language: de`, it has `name: "Skeptic"`

#### Scenario: An unknown step

- **WHEN** the user runs `codev roles --at plan.review --json`
- **THEN** the exit code is 1 and `status[0].code` is `unknown_step`
- **AND** its message lists the valid steps and `roles` is empty

#### Scenario: Human output of every role

- **GIVEN** a configuration with `roles: { newcomer: never }`
- **WHEN** the user runs `codev roles`
- **THEN** it prints one line per role, the `newcomer` line saying `never`
  and `project`, and the `product` line saying it cannot be disabled

### Requirement: `roles` is a known configuration key

`roles` SHALL be a known top-level key of `_codev/config.yaml`: it never
produces an `unknown_config_key` warning.

#### Scenario: No warning for `roles`

- **GIVEN** a configuration with `roles: { newcomer: never }`
- **WHEN** the user runs `codev status --json`
- **THEN** `status` holds no `unknown_config_key` warning
