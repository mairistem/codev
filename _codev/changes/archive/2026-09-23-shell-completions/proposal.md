# Proposal: generate shell completions via `codev completions`

## Why

A user who types `codev de<Tab>` in their shell today gets
nothing: no completion on subcommands (`decision`,
`deviate`, `deviated`…), on active change names, or on
flags. It is a small, constant friction on a tool used
dozens of times a day.

`clap` (used by `codev-cli`) exposes an official companion,
`clap_complete`, which generates completion scripts for bash, zsh,
fish, powershell and elvish from the existing `#[derive(Parser)]`
declaration. Nothing to write by hand — the output follows the
subcommands as they evolve, with no possible drift.

## What Changes

- **New `codev completions <shell>` subcommand** — prints
  the completion script for the given shell to stdout.
  `<shell>` accepts the five standard `clap_complete` values
  (`bash`, `zsh`, `fish`, `powershell`, `elvish`).
- **Installation documentation** — the subcommand's `--help`
  lists the recommended procedure per shell (redirecting to the right
  file, `source`, etc.).
- **New `clap_complete = "4"` dependency** in
  `crates/codev-cli/Cargo.toml` — aligned with the major version of
  `clap` already in use.
- **No automatic scaffolding** — the command touches no
  system file; it is up to the user to redirect the output wherever
  they want. Consistent with the codev philosophy ("the skill/command
  guides, the user acts").

## Capabilities

### New Capabilities

- `shell-completions` — describes the contract of the
  `codev completions` subcommand: supported shells, output format, no
  side effects.

### Modified Capabilities

None.

### Removed Capabilities

None.

## Impact

- **Code**:
  - New variant `Command::Completions { shell: Shell }` in
    `crates/codev-cli/src/main.rs`, where `Shell` is
    `clap_complete::Shell`.
  - A `match` arm that calls
    `clap_complete::generate(shell, &mut Cli::command(), "codev",
    &mut io::stdout())`.
  - A test that checks that generation for each of the five
    shells produces non-empty output and contains the name `codev`.
- **JSON contract**: nothing. The output is shell script, not
  JSON. The command has no `--json` flag (it would make no sense).
- **Files written**: none — stdout output only.
- **Migration**: none. Purely additive feature.
- **Out of scope**:
  - **Dynamic completion on active change names** — requires
    a runtime lookup; `clap_complete` generates static output. Deferrable.
  - **Auto-installation in `codev init`** — too magical, depends
    on the user's shell configuration.
  - **Support for exotic shells** (nushell, xonsh) — outside the five
    of `clap_complete`. Deferrable if there is a real need.
