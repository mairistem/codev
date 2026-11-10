## ADDED Requirements

### Requirement: Archive reports the unchecked tasks, and refuses them under `--strict`

`codev archive` SHALL count the boxes of the file named by the schema's
`apply.tracks` in the change it archives, as the `task-progress` capability
defines, once the validation pre-flight and the review gate have passed.

Without `--strict`, unchecked boxes SHALL NOT prevent the archive: the
archive proceeds, the human output adds a warning naming the number of
unchecked boxes and the tracked file, then each unchecked box's line,
trimmed, and the JSON report's `status` carries one entry of level
`warning` and code `tasks_unchecked`. The exit code stays 0.

With `--strict`, unchecked boxes MUST make the archive refuse with the code
`tasks_unchecked`, before any write: no main spec is modified and the
change stays active at its location. The message names the number of
unchecked boxes and the tracked file.

An absent tracked file, or one without any box, SHALL give neither a
warning nor a refusal. The JSON report of a successful archive SHALL carry
`tasks`, with the same shape and the same `null` as in `codev status`,
always present. Only the change being archived is evaluated: changes already
under `_codev/changes/archive/` are never read for their boxes, by
`codev archive` or by `codev validate`, and `codev validate` reports nothing
about boxes.

#### Scenario: Unchecked boxes warn and the archive proceeds

- **GIVEN** a change `add-auth` that passes validation and the review gate,
  whose `tasks.md` holds `- [x] 1.1 Parser` and `- [ ] 9.1 Tag the
  release`
- **WHEN** the user runs `codev archive --change add-auth --json`
- **THEN** the exit code is 0 and the change is moved to the dated archive
- **AND** `status` holds one entry with level `warning` and code
  `tasks_unchecked`
- **AND** `tasks.checked` is 1 and `tasks.total` is 2

#### Scenario: The human output names each unchecked box

- **GIVEN** the same change `add-auth`
- **WHEN** the user runs `codev archive --change add-auth`
- **THEN** the output warns of 1 unchecked box in `tasks.md`
- **AND** it shows the line `- [ ] 9.1 Tag the release`

#### Scenario: `--strict` refuses unchecked boxes and writes nothing

- **GIVEN** the same change `add-auth`, whose delta modifies a main spec
- **WHEN** the user runs `codev archive --change add-auth --strict --json`
- **THEN** the exit code is not 0 and `status[0].code` is
  `tasks_unchecked`
- **AND** no main spec is modified and `_codev/changes/add-auth/` still
  exists

#### Scenario: `--strict` archives a change whose boxes are all checked

- **GIVEN** a change whose `tasks.md` boxes are all checked
- **WHEN** the user runs `codev archive --change add-auth --strict --json`
- **THEN** the exit code is 0, the change is archived, and `status` holds
  no `tasks_unchecked` entry

#### Scenario: No tracked file, no warning

- **GIVEN** a change that passes validation and holds no `tasks.md`
- **WHEN** the user runs `codev archive --change add-auth --strict --json`
- **THEN** the change is archived, `tasks` is `null`, and `status` holds no
  `tasks_unchecked` entry

#### Scenario: Archived changes are never re-evaluated

- **GIVEN** a project whose `_codev/changes/archive/2026-09-28-fix-thin-detection/tasks.md`
  holds two unchecked boxes, and an active change whose boxes are all
  checked
- **WHEN** the user runs `codev validate --strict`, then
  `codev archive --change add-auth --strict`
- **THEN** neither command reports `tasks_unchecked`, and both exit with
  code 0
