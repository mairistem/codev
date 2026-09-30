# Design: `codev completions <shell>`

## Context

See `proposal.md`. A purely additive change, a few lines of code
that delegate to `clap_complete`.

## Decisions

### Decision: `clap_complete` dependency, no manual generation

`clap_complete` is the official companion of `clap` — maintained by
the same maintainers, versioned in parallel. Generating the scripts by
hand would take long, would let the docs drift, and everything would
have to be redone each time a command is added.

**Rejected alternative**: a bootstrap script that parses `--help` and
generates the completion. Fragile — the `--help` format can change,
shell grammars vary, and we would be writing code that is already part
of the ecosystem.

### Decision: five shells, not a limited selection

`clap_complete::Shell` lists five shells (`bash`, `zsh`, `fish`,
`powershell`, `elvish`). Supporting them all costs **nothing** in
code — one clap enum, a single-arm match. On the other hand, refusing
`fish` or `elvish` up front would create a legitimate complaint in
a few months.

### Decision: dedicated subcommand, not a global flag

Two options:

| Option | Pro | Con |
|---|---|---|
| **A. `codev --completions <shell>`** global flag | Short to type | Breaks the "verb-noun" pattern of the rest of the CLI |
| **B. `codev completions <shell>`** subcommand | Consistent with `codev list`, `codev decision …` | Two more characters |

**Chosen: B.** Consistency wins.

### Decision: no automatic installation

`codev init` will **not** try to guess the user's shell or
write into `~/.zshrc` or similar. The pattern would be
too magical: each shell has its location, its conventions,
sometimes a separate completions folder (`~/.zfunc/`), sometimes an
`autoload`. A failure would leave an orphan entry in the user's
config file — impossible to undo cleanly.

The command's help message (`codev completions --help`) MUST
list the recommended procedure per shell — short, copy-pasteable,
unambiguous:

- **bash**: `codev completions bash > ~/.local/share/bash-completion/completions/codev`
- **zsh**: `codev completions zsh > "${fpath[1]}/_codev"` then
  `compinit`
- **fish**: `codev completions fish > ~/.config/fish/completions/codev.fish`
- **powershell**: `codev completions powershell | Out-String |
  Invoke-Expression` (or redirect to `$PROFILE`)
- **elvish**: the official Elvish docs take care of the rest

### Decision: the command has no `--json` flag

The output is a shell script — no structured JSON is possible or
useful. The command sits outside codev's global JSON contract:
`fail(json, shape, err)` is not called on this branch, and there
is no `Vec<StatusEntry>` to return. The public contract remains
honored — nothing added, nothing removed in `codev-cli::contract::v1`.

## Risks / Trade-offs

- **`clap_complete` may evolve and break** — shared maintainers
  with `clap`, semver respected. Version 4.x as long as we are on 4.x
  for clap.
- **Static completion — no active change names** — to
  complete `codev status --change <TAB>` with the real names, we
  would have to generate dynamically at each call. `clap_complete`
  supports this via a custom `ValueEnum` + `PossibleValue` — deferrable,
  not in this batch.

## Migration Plan

None. Additive, opt-in feature.
