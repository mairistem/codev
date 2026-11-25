# Shell Completions Specification

## Purpose

Provide a mechanism for users to enable completion of `compagnon`
commands in their shell, without writing or maintaining a script by
hand — the tool generates the script from the existing `clap`
declaration.

## Requirements

### Requirement: `compagnon completions <shell>` prints a completion script to stdout

The `compagnon` binary SHALL expose a `completions <shell>` subcommand
where `<shell>` is one of the five standard `clap_complete` values:
`bash`, `zsh`, `fish`, `powershell`, `elvish`.

The command MUST:

- write the script to **stdout only** — no system file is touched;
- return exit code **0** on success;
- refuse with a non-zero exit code and a clear message if `<shell>`
  is not one of the five recognized values.

The script MUST match the current structure of the Compagnon
commands — every new subcommand added to `Cli` (for example
`decision promote`, `decision deviate`) appears in the script
without manual intervention.

#### Scenario: zsh generation is non-empty and mentions Compagnon

- **GIVEN** a `compagnon` binary of the current version
- **WHEN** the user runs `compagnon completions zsh`
- **THEN** stdout carries non-empty output (≥ 500 characters)
- **AND** that output contains the name `compagnon` at least once
- **AND** the exit code is 0

#### Scenario: Support for the five clap_complete shells

- **GIVEN** a `compagnon` binary of the current version
- **WHEN** the user successively runs `compagnon completions bash`,
  `compagnon completions zsh`, `compagnon completions fish`,
  `compagnon completions powershell`, `compagnon completions elvish`
- **THEN** each call produces non-empty output on stdout
- **AND** each call returns exit code 0

#### Scenario: Unknown shell rejected

- **GIVEN** a `compagnon` binary of the current version
- **WHEN** the user runs `compagnon completions nushell`
- **THEN** no script is printed to stdout
- **AND** an error message names `nushell` and recalls the list
  of recognized shells
- **AND** the exit code is non-zero

### Requirement: The `completions` command is read-only

`compagnon completions <shell>` MUST NOT write to the file system,
contact the network, or read `_compagnon/config.yaml` or any other
project state. It does not depend on an initialized repository: it
works from any current directory, including outside any Compagnon root.

#### Scenario: Works outside a Compagnon repository

- **GIVEN** a user in a directory that has no `_compagnon/`
- **WHEN** they run `compagnon completions bash`
- **THEN** the script is printed normally
- **AND** no error message mentions `_compagnon`
