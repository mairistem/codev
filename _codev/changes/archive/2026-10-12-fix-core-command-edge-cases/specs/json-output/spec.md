## Purpose

Keep the JSON documents of every command identical whatever the operating
system, so that a skill or a script reads them without knowing where codev
runs.

## ADDED Requirements

### Requirement: Paths in JSON documents use `/` separators on every OS

Every path a command writes into a JSON document — absolute or relative,
alone or in a list, in a success or a failure document — SHALL use `/` as
its only separator, on every operating system: every `\` becomes `/`. On
Windows, a path keeps its drive prefix; on Linux and macOS, an absolute path
starts with a single `/`. The other
string fields of a document — messages, schema `generates` patterns,
identifiers — are not paths and are written as they are.

This applies in particular to `root`, `planningHome`, `changeRoot`,
`created`, `updated`, `untouched`, `preserved`, `resolvedOutputPath`, the
`path` of dependencies, decisions and findings, `movedTo`, `resolvedPath`,
and the `path` of `codev self-update` and of `codev changelog --write`.

#### Scenario: Status of a change on Windows

- **GIVEN** a project at `C:\work\acme` on Windows with a change `add-auth`
- **WHEN** the user runs `codev status --change add-auth --json`
- **THEN** `planningHome` is `C:/work/acme`
- **AND** `changeRoot` is `C:/work/acme/_codev/changes/add-auth`

#### Scenario: Instructions on Windows

- **GIVEN** the same project, whose `proposal.md` is written
- **WHEN** the user runs `codev instructions specs --change add-auth --json`
- **THEN** `resolvedOutputPath` is
  `C:/work/acme/_codev/changes/add-auth/specs/**/*.md`
- **AND** the `path` of the `proposal` dependency is
  `C:/work/acme/_codev/changes/add-auth/proposal.md`

#### Scenario: Files written on Windows

- **GIVEN** the same project
- **WHEN** the user runs `codev new change fix-bug --json`, then
  `codev update --json`
- **THEN** every entry of `created` and `updated`, and `changeRoot` and
  `root`, contain no `\`

#### Scenario: Unix paths are unchanged

- **GIVEN** a project at `/home/you/acme` on Linux or macOS
- **WHEN** the user runs `codev status --change add-auth --json`
- **THEN** `planningHome` is `/home/you/acme`, with a single leading `/`
