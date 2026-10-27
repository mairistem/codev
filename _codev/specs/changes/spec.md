# Changes Specification

## Purpose

Create changes, list them, and tell where each one stands in its planning,
so that a person or a skill always knows which artifact to write next and
whether implementation can start.

## Requirements

### Requirement: `codev new change` creates a change and its metadata

`codev new change <name>` SHALL create the folder `_codev/changes/<name>/`
holding a single file, `change.yaml`, with:

- `schema` — the schema of the change, chosen as the `schemas` capability
  describes;
- `created` — the current date, `YYYY-MM-DD`;
- `goal` — the text of `--goal`, only when one is given and it is not
  blank.

The human output SHALL say `Change `<name>` created`, give the folder's
location and the schema, and end with a next step naming
`codev instructions --change <name>`. With `--json`, the document SHALL
carry `changeName`, `schemaName`, `changeRoot` (the absolute folder path),
`created` (the files written) and `status` (configuration warnings, if
any). The command exits with code 0.

#### Scenario: A change with a goal

- **GIVEN** an initialized project dated 2026-09-08 with no `schema:` key
- **WHEN** the user runs `codev new change add-auth --goal "Add authentication"`
- **THEN** `_codev/changes/add-auth/change.yaml` holds `schema: spec-driven`,
  `created: 2026-09-08` and `goal: Add authentication`
- **AND** it is the only file in the folder
- **AND** the output names `codev instructions --change add-auth`

#### Scenario: A blank goal is not recorded

- **WHEN** the user runs `codev new change add-auth --goal "  "`
- **THEN** `change.yaml` has no `goal` key

#### Scenario: JSON document of a created change

- **WHEN** the user runs `codev new change add-auth --json`
- **THEN** stdout is one JSON document with `changeName: "add-auth"`,
  `schemaName: "spec-driven"`, a `changeRoot` ending with
  `_codev/changes/add-auth`, `created` listing its `change.yaml`, and an
  empty `status`

### Requirement: A change name is kebab-case and unique

A change name MUST consist of lowercase ASCII letters, digits and hyphens,
MUST NOT start or end with a hyphen, and MUST NOT contain two consecutive
hyphens; it MAY start with a digit. The name `archive` is reserved: it is
the folder holding the archived changes, and it is never a valid change
name. `codev new change` SHALL refuse any other name with the error code
`invalid_change_id` and a message naming the first rule broken (empty,
leading or trailing hyphen, consecutive hyphens, space, underscore,
uppercase letter, the offending character, or the reserved name). It SHALL
refuse a name whose folder already exists with the error code
`change_exists`. A refused command writes nothing and exits with code 1.

#### Scenario: Names accepted

- **WHEN** the user creates the changes `add-auth`, `100-add-feature` and
  `v2-migration`
- **THEN** each one is created

#### Scenario: An invalid name is refused with the rule it breaks

- **WHEN** the user runs `codev new change "Add Auth"`
- **THEN** the command fails with `invalid_change_id` and a message saying
  the name contains a space
- **AND** `_codev/changes/` is unchanged

#### Scenario: The reserved name `archive` is refused

- **GIVEN** a project with archived changes
- **WHEN** the user runs `codev new change archive`
- **THEN** the command fails with `invalid_change_id` and a message saying
  `archive` is reserved for the archived changes
- **AND** `_codev/changes/` is unchanged

#### Scenario: Names that only contain `archive` are accepted

- **WHEN** the user creates the changes `archive-old-specs` and
  `fix-archive`
- **THEN** each one is created

#### Scenario: An existing name is refused

- **GIVEN** an active change `add-auth`
- **WHEN** the user runs `codev new change add-auth`
- **THEN** the command fails with `change_exists`
- **AND** the existing `change.yaml` is unchanged

### Requirement: `codev list` lists the active changes and, with `--specs`, the capabilities

`codev list` SHALL list the active changes — the folders of
`_codev/changes/` whose name is a valid change name, which excludes the
reserved `archive` — in alphabetical order. A folder whose name is not a
valid change name is ignored without error. When there is none, the human
output SHALL say so and name `codev new change <name>`. With `--json`, the
document SHALL be `{ "changes": [...], "root": <project root>, "status": []
}`.

`codev list --specs` SHALL list the capabilities that have a main spec: the
path, relative to `_codev/specs/`, of each folder holding a file named
exactly `spec.md`, nested folders included, in alphabetical order. Any other
file is not a spec and is ignored without error — including a Markdown file
whose name merely ends with `spec.md`, such as `myspec.md`. A `spec.md`
placed directly in `_codev/specs/` names no capability. When there is none,
the human output SHALL say that specs appear when a change is synced or
archived. With `--json`, the document SHALL be `{ "specs": [...], "root":
<project root>, "status": [] }`.

#### Scenario: Active changes, archive excluded

- **GIVEN** the changes `fix-bug` and `add-auth`, an archived change, and a
  folder `Notes_2026` under `_codev/changes/`
- **WHEN** the user runs `codev list --json`
- **THEN** `changes` is `["add-auth", "fix-bug"]`

#### Scenario: No active change

- **GIVEN** a project without active change
- **WHEN** the user runs `codev list`
- **THEN** the output names `codev new change`
- **AND** the exit code is 0

#### Scenario: Capabilities, nested ones included

- **GIVEN** main specs `_codev/specs/user-auth/spec.md` and
  `_codev/specs/identity/sso/spec.md`, and a file
  `_codev/specs/user-auth/README.md`
- **WHEN** the user runs `codev list --specs --json`
- **THEN** `specs` is `["identity/sso", "user-auth"]`

#### Scenario: A Markdown file named like a spec is not a capability

- **GIVEN** a main spec `_codev/specs/user-auth/spec.md` and the files
  `_codev/specs/notes/myspec.md` and `_codev/specs/draftspec.md`
- **WHEN** the user runs `codev list --specs --json`
- **THEN** `specs` is `["user-auth"]`

#### Scenario: No capability yet

- **GIVEN** a project without main spec
- **WHEN** the user runs `codev list --specs`
- **THEN** the output says that specs appear when a change is synced or
  archived

### Requirement: `codev status` derives each artifact's state from the files present

`codev status` SHALL report, for each artifact of the change's schema, in
the schema's dependency order (ties broken by declaration order), a state
computed only from the files present in the change folder — codev keeps no
state file:

- `done` — its output exists; for a glob output such as `specs/**/*.md`, at
  least one file matches;
- `skipped` — the change sets `skip_specs: true` and the artifact writes
  under `specs/`; it counts as satisfied for the artifacts requiring it;
- `ready` — every artifact it requires is `done` or `skipped`;
- `blocked` — otherwise, with `missingDeps` naming the required artifacts
  not yet satisfied.

`applyRequires` SHALL be the schema's `apply.requires`, and
`isPlanningComplete` SHALL be `true` only when every artifact reachable from
`applyRequires` through the `requires` edges is `done` or `skipped` — an
artifact listed in `applyRequires` being `done` is not enough.

With `--json`, the document SHALL carry `changeName`, `schemaName`,
`planningHome` (the project root), `changeRoot` (the change folder),
`applyRequires`, `isPlanningComplete`, `artifacts` — each with `id`,
`outputPath` (the schema's `generates`, verbatim), `status`, `requires` and
`missingDeps` — and `status` (configuration warnings). The human output
SHALL print `Change:` and `Schema:`, one line per artifact marked `[x]`
done, `[ ]` ready, `[-]` blocked with `(waiting on: …)`, or `[~]` skipped
with `(disabled by skip_specs)`, then `Planning: <satisfied>/<total>
artifacts` and either `Next:` with the first ready artifact or
`Planning is complete.`. The command exits with code 0.

#### Scenario: A fresh change has only its first artifact ready

- **GIVEN** a `spec-driven` change with no artifact written
- **WHEN** the user runs `codev status --change add-auth --json`
- **THEN** `proposal` is `ready`, `specs` and `design` are `blocked` with
  `missingDeps: ["proposal"]`, and `tasks` is `blocked` with
  `missingDeps: ["specs", "design"]`
- **AND** `applyRequires` is `["tasks"]` and `isPlanningComplete` is
  `false`

#### Scenario: A written artifact unlocks the next ones

- **GIVEN** a `spec-driven` change whose `proposal.md` exists
- **WHEN** the user runs `codev status --change add-auth`
- **THEN** the output shows `[x] proposal`, `[ ] specs`, `[ ] design` and
  `[-] tasks (waiting on: specs, design)`
- **AND** it ends with `Planning: 1/4 artifacts` and `Next:     specs`

#### Scenario: A nested spec delta satisfies the glob

- **GIVEN** a change holding `proposal.md` and
  `specs/identity/user-auth/spec.md`
- **WHEN** the user runs `codev status --change add-auth --json`
- **THEN** `specs` is `done`

#### Scenario: `skip_specs` skips the spec artifact

- **GIVEN** a change with `skip_specs: true` whose `proposal.md` and
  `design.md` exist
- **WHEN** the user runs `codev status --change refactor`
- **THEN** `specs` is shown `[~]` and `(disabled by skip_specs)`
- **AND** `tasks` is `ready`

#### Scenario: Writing the tracked file first does not complete planning

- **GIVEN** a `spec-driven` change where only `tasks.md` exists
- **WHEN** the user runs `codev status --change add-auth --json`
- **THEN** `tasks` is `done`
- **AND** `isPlanningComplete` is `false`

#### Scenario: Every artifact written

- **GIVEN** a change where every artifact of its schema is written
- **WHEN** the user runs `codev status --change add-auth`
- **THEN** `isPlanningComplete` is `true` in the JSON
- **AND** the human output says `Planning is complete.` and has no `Next:`
  line

#### Scenario: Resolved paths for the skills

- **GIVEN** a project at `/p` with a change `add-auth`
- **WHEN** the user runs `codev status --json` from `/p/src`
- **THEN** `planningHome` is `/p` and `changeRoot` is
  `/p/_codev/changes/add-auth`

### Requirement: The commands taking `--change` pick the change predictably

`codev status`, `codev instructions`, `codev sync` and `codev archive` SHALL
act on the change named by `--change` when it is given. A name that is not a
valid change name — the reserved `archive` included — fails with
`invalid_change_id`; a valid name with no folder fails with
`unknown_change`, whose message points to `codev list`. Without `--change`:

- with exactly one active change, that change is used;
- with none, the command fails with `no_active_change` and the fix
  `create one with `codev new change <name>``;
- with several, the command fails with `ambiguous_change`, a message
  listing every active change, and a fix suggesting `--change` with the
  first one — it never picks one.

The archive folder is never an active change: it is never inferred nor
counted among the candidates.

A failure exits with code 1; with `--json`, the document keeps the
command's fields, set to `null` or empty, and carries the code in `status`.

#### Scenario: The single active change is inferred

- **GIVEN** one active change `add-auth`
- **WHEN** the user runs `codev status`
- **THEN** the status of `add-auth` is shown

#### Scenario: The archive folder is not an active change

- **GIVEN** one active change `add-auth` and an archived change
- **WHEN** the user runs `codev status`
- **THEN** the status of `add-auth` is shown

#### Scenario: No active change

- **GIVEN** a project without active change
- **WHEN** the user runs `codev status`
- **THEN** the command fails with `no_active_change` and the help names
  `codev new change`

#### Scenario: Several active changes

- **GIVEN** the active changes `add-auth` and `fix-bug`
- **WHEN** the user runs `codev status --json`
- **THEN** the exit code is 1
- **AND** `status[0].code` is `ambiguous_change`, its message names
  `add-auth` and `fix-bug`, and `changeName` is `null`

#### Scenario: A named change that does not exist

- **WHEN** the user runs `codev status --change ghost`
- **THEN** the command fails with `unknown_change` and a message naming
  `codev list`

#### Scenario: A named change with an invalid name

- **WHEN** the user runs `codev instructions --change Bad`
- **THEN** the command fails with `invalid_change_id`

#### Scenario: The archive folder named as a change

- **GIVEN** a project with archived changes
- **WHEN** the user runs `codev status --change archive --json`
- **THEN** the exit code is 1
- **AND** `status[0].code` is `invalid_change_id`, its message says
  `archive` is reserved, and `changeName` is `null`

### Requirement: Unknown keys of `change.yaml` warn instead of failing

When a change's `change.yaml` holds a top-level key, or a key under its
`review:` mapping, that the running codev does not know, every command
that reads the change SHALL ignore that key, use the rest of the file, and
report one warning with the code `change_unknown_key` per unknown key. The
warning MUST name the key (`review.<key>` for a key of `review`) and the
change's `change.yaml`, say that the change may have been written by a
newer codev, quote the project's `codev_version` requirement when
`_codev/config.yaml` sets one, and point to `codev self-update`, or to
correcting the key if it is a typo. Commands that report configuration
warnings MUST report it the same way, in their human output and in the
`status` array of their JSON output.

`codev validate` SHALL report each unknown key as a finding of severity
`warning` and code `change_unknown_key` on the change's item, at the line
of the key in `change.yaml`; like any warning, it fails
`codev validate --strict` and not `codev validate`. A value that is not
one the key accepts — a `review.mode` or a role activation, for instance —
still fails as its own requirement says: a value cannot be ignored without
guessing what the change meant.

#### Scenario: An unknown top-level key

- **GIVEN** a project with `codev_version: ">=0.6"` and a change `add-auth`
  whose `change.yaml` holds `schema: spec-driven` and the unknown key
  `ticket: JVS-12`
- **WHEN** the user runs `codev status --change add-auth --json`
- **THEN** the command succeeds and reports the change's artifacts
- **AND** `status` holds a `change_unknown_key` warning whose message
  names `ticket`, `change.yaml`, quotes `>=0.6` and names
  `codev self-update`

#### Scenario: An unknown key under `review`

- **GIVEN** a change `add-auth` whose `change.yaml` holds
  `review: { mode: isolated, budget: 3 }`
- **WHEN** the user runs `codev roles --at code.review --change add-auth
  --json`
- **THEN** `review.mode` is `isolated` and `review.origin` is `change`
- **AND** `status` holds a `change_unknown_key` warning naming
  `review.budget`

#### Scenario: Strict validation catches a misspelled key

- **GIVEN** a change `add-auth` with a valid delta, whose `change.yaml`
  holds `skip_spec: true` on line 3, and no other finding
- **WHEN** the user runs `codev validate add-auth --strict --json`
- **THEN** the item of `add-auth` holds a `warning` finding
  `change_unknown_key` naming `skip_spec` at line 3 of its `change.yaml`
- **AND** the exit code is 1

#### Scenario: Non-strict validation with an unknown key

- **GIVEN** a change `add-auth` with a valid delta, whose `change.yaml`
  holds the unknown key `ticket: JVS-12`, and no other finding
- **WHEN** the user runs `codev validate add-auth`
- **THEN** the warning is reported and the exit code is 0
