## ADDED Requirements

### Requirement: The specs validated are the capabilities listed

`codev validate` — without argument, with `--all` or with `--specs` — SHALL
validate exactly the main specs of the capabilities that `codev list
--specs` lists, and `codev validate <name>` SHALL find a spec under the same
names. A file under `_codev/specs/` that is not a capability's `spec.md`
SHALL be ignored without finding: it is neither validated nor reported, and
it never makes the command fail.

#### Scenario: A stray Markdown file is ignored

- **GIVEN** a project with a valid main spec `_codev/specs/user-auth/spec.md`
  and a file `_codev/specs/notes/myspec.md`
- **WHEN** the user runs `codev validate --specs --json`
- **THEN** the exit code is 0
- **AND** `items` holds one spec item, `user-auth`, and no item mentions
  `notes`

#### Scenario: The whole project with a stray file

- **GIVEN** the same project
- **WHEN** the user runs `codev validate`
- **THEN** the exit code is 0

#### Scenario: A stray file is not an item

- **GIVEN** the same project
- **WHEN** the user runs `codev validate notes/my`
- **THEN** the command fails with `unknown_item`
