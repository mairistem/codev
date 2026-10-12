## MODIFIED Requirements

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
