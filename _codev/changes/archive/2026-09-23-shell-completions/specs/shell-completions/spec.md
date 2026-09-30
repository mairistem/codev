## Purpose

Provide a mechanism for users to enable completion of `codev`
commands in their shell, without having to write or maintain a
script by hand — the tool generates the script from the existing
`clap` declaration.

## ADDED Requirements

### Requirement: `codev completions <shell>` prints a completion script to stdout

The `codev` binary SHALL expose a `completions <shell>` subcommand
where `<shell>` is one of the five standard `clap_complete`
values: `bash`, `zsh`, `fish`, `powershell`, `elvish`.

The command MUST:

- write the script to **stdout only** — no system file
  is touched;
- return exit code **0** on success;
- refuse with a non-zero exit code and a clear message if `<shell>`
  is not one of the five recognized values.

The script MUST match the current structure of the codev
commands — every new subcommand added to `Cli` (for example
`decision promote`, `decision deviate`) appears in the script without
manual intervention.

#### Scenario: zsh generation is non-empty and mentions codev

- **GIVEN** a `codev` binary of the current version
- **WHEN** the user runs `codev completions zsh`
- **THEN** stdout carries non-empty output (≥ 500 characters)
- **AND** that output contains the name `codev` at least once
- **AND** the exit code is 0

#### Scenario: Support for the five clap_complete shells

- **GIVEN** a `codev` binary of the current version
- **WHEN** the user successively runs `codev completions bash`,
  `codev completions zsh`, `codev completions fish`,
  `codev completions powershell`, `codev completions elvish`
- **THEN** each call produces non-empty output on stdout
- **AND** each call returns exit code 0

#### Scenario: Unknown shell refused

- **GIVEN** a `codev` binary of the current version
- **WHEN** the user runs `codev completions nushell`
- **THEN** no script is printed to stdout
- **AND** an error message names `nushell` and recalls the list
  of recognized shells
- **AND** the exit code is non-zero

### Requirement: The `completions` command is purely read-only

`codev completions <shell>` MUST NOT write to the file
system, contact the network, or read `_codev/config.yaml` or
any other project state. It is independent of an initialized
repository: it works in any current directory,
including outside any codev root.

#### Scenario: Works outside a codev repository

- **GIVEN** a user in a directory that has no `_codev/`
- **WHEN** they run `codev completions bash`
- **THEN** the script is printed normally
- **AND** no error message mentions `_codev`
