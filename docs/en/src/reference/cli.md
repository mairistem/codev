# Command-line interface

This chapter documents every `codev` command. The help text of each command is
reproduced exactly as `codev <command> --help` prints it; run that command to
see the help of the version you have installed.

```bash
codev --help
```

```text
codev adds a thin layer of specs to a repository so that you and your agent agree on what must be built before a single line of code is written.

The commands below run in your terminal. The workflows are invoked in the Claude Code chat: /codev-propose, /codev-explore.

Usage: codev <COMMAND>

Commands:
  init          Initialize codev in a project and install the Claude Code skills
  update        Regenerate the skills after a codev upgrade
  new           Create a new item
  list          List active changes, or specified capabilities with --specs
  status        Show the state of a change's artifacts
  instructions  Print everything needed to write an artifact
  schemas       List the available workflow schemas
  decision      Create, accept, inspect and supersede architecture decisions
  sources       Manage inherited sources (local paths and remote git repositories)
  sync          Merge a change's deltas into the main specs without archiving
  archive       Merge, then move a change to the dated archive
  docs          Open the codev documentation in the browser
  completions   Generate a shell completion script for local installation
  validate      Check changes and specs for structural errors and consistency
  help          Print this message or the help of the given subcommand(s)

Options:
  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## Conventions

These rules hold for every command except `docs` and `completions`, which do
not read the project:

- **Project discovery.** codev looks for `_codev/` in the current directory,
  then in each parent directory. Run commands from anywhere inside the
  project. Outside a project, commands fail with `no_codev_root`.
- **`--json`.** Every command that reads or writes the project accepts
  `--json`. stdout then carries exactly one JSON document — on failure too —
  and human messages are not printed. See [JSON output](json-output.md).
- **Exit status.** `0` on success, `1` on failure. For `codev validate`, see
  [its section](#codev-validate).
- **Errors.** In human mode, errors go to stderr as `error: …`, often followed
  by a `help: …` line with the fix.
- **Choosing a change.** Commands that take `--change <CHANGE>` infer it when
  the project has exactly one active change, and fail with `ambiguous_change`
  when there are several.

## Project

### codev init

```text
Initialize codev in a project and install the Claude Code skills

Usage: codev init [OPTIONS] [PATH]

Arguments:
  [PATH]
          Project folder (default: the current folder)

Options:
      --force
          Rewrite skills even if they were edited by hand

  -y, --yes
          Accept all defaults without prompting (defaults + detected values)

      --no-detect
          Disable the environment probe (useful for reproducible tests)

      --preset <PRESET>
          Preselect the answer to the workflows question

          Possible values:
          - full:    All 8 workflows
          - minimal: propose, explore, onboard, configure
          - custom:  Pick the workflows one by one

      --language <CODE>
          Language of the prose skills write in artifacts, as an ISO 639 code (`en`, `fr`, `pt-BR`…). Default: detected from the locale, otherwise `en`

      --json


  -h, --help
          Print help (see a summary with '-h')
```

Initializes codev in `PATH` (the current folder by default): creates the
`_codev/` tree, writes `_codev/config.yaml`, and installs the skills into
`.claude/skills/`. See [What `codev init` does](../quickstart.md#what-codev-init-does)
for the probe and the questions.

- `--yes` skips every question and applies the defaults and detected values.
  Without a terminal on stdin (CI, pipes), `--yes` is implied.
- `--preset` answers the workflows question: `full` installs all eight
  workflows, `minimal` installs `propose`, `explore`, `onboard` and
  `configure`, `custom` lets you pick them one by one (and falls back to
  `full` without a terminal).
- `--language` sets the `language:` key. Without it, the language comes from
  the locale, else `en`. See [Artifact language](../guides/artifact-language.md).
- `--no-detect` disables the probe of manifests, license, CI, locale and MCP
  servers.

When `_codev/config.yaml` already exists, it is left untouched: `codev init`
only creates missing folders and installs or refreshes the skills. Skills you
edited by hand are preserved unless you pass `--force`.

### codev update

```text
Regenerate the skills after a codev upgrade

Usage: codev update [OPTIONS]

Options:
      --force  Rewrite skills even if they were edited by hand
      --json
  -h, --help   Print help
```

Regenerates the skills listed by `workflows:` in the configuration (all eight
when the key is absent), for example after upgrading codev or changing the
`mcp:` configuration. A skill whose content differs from what this version
generates, but which carries this version's stamp, was edited by hand: it is
left in place and reported, unless you pass `--force`. Skills removed from
`workflows:` are not deleted.

### codev docs

```text
Open the codev documentation in the browser

Three forms:

- default: writes `codev-docs-<version>.html` to the system temp folder and opens it in the browser; - `--write <PATH>`: writes the HTML to the given path, opens nothing (for sharing — email, Confluence, shared drive); - `--print`: prints the markdown source to stdout (to pipe into `less`, `bat` or an LLM).

Usage: codev docs [OPTIONS]

Options:
      --print
          Print the markdown source to stdout (does not open anything)

      --write <PATH>
          Write the HTML to the given path (does not open anything)

      --lang <LANG>
          Language of the documentation

          Possible values:
          - en: English
          - fr: French

          [default: en]

  -h, --help
          Print help (see a summary with '-h')
```

Renders this documentation as one self-contained HTML page — inline CSS, no
network access — and opens it in your browser. The page is written to
`codev-docs-<version>.html` in the system temporary folder. `--write` writes
it elsewhere without opening it, for sharing; `--print` prints the Markdown
source. `--lang` selects the English (`en`, the default) or French (`fr`)
edition. `codev docs` works anywhere, including outside a codev project.

```bash
codev docs --lang fr --write ~/Downloads/codev-manual-fr.html
```

### codev completions

```text
Generate a shell completion script for local installation

The output goes to stdout — redirect it to your shell's completions folder. Typical setups:

- bash: `codev completions bash > ~/.local/share/bash-completion/completions/codev` - zsh: `codev completions zsh > "${fpath[1]}/_codev"`, then `compinit` - fish: `codev completions fish > ~/.config/fish/completions/codev.fish` - powershell: `codev completions powershell | Out-String | Invoke-Expression`

The command touches no file; it does not read `_codev/` either and works in any directory.

Usage: codev completions <SHELL>

Arguments:
  <SHELL>
          Target shell: bash, zsh, fish, powershell, elvish

          [possible values: bash, elvish, fish, powershell, zsh]

Options:
  -h, --help
          Print help (see a summary with '-h')
```

See [Shell completions](../installation.md#shell-completions).

## Changes

### codev new change

```text
Create a change

Usage: codev new change [OPTIONS] <NAME>

Arguments:
  <NAME>  Name in kebab-case (`add-user-auth`)

Options:
      --schema <SCHEMA>  Workflow schema to use
      --goal <GOAL>      Goal, kept in the change's metadata
      --json
  -h, --help             Print help
```

Creates `_codev/changes/<NAME>/` with a `change.yaml` recording the schema,
the creation date and the goal. The name must be kebab-case: lowercase
letters, digits and single hyphens. The schema defaults to the `schema:` key
of the configuration, else `spec-driven`. Fails with `change_exists` if the
change already exists.

```bash
codev new change add-dark-mode --goal "Let users switch the UI to a dark theme"
```

### codev list

```text
List active changes, or specified capabilities with --specs

Usage: codev list [OPTIONS]

Options:
      --specs
      --json
  -h, --help   Print help
```

Lists the active changes, or with `--specs` the capabilities that have a main
spec under `_codev/specs/`.

### codev status

```text
Show the state of a change's artifacts

Usage: codev status [OPTIONS]

Options:
      --change <CHANGE>  Change name (inferred when there is only one)
      --json
  -h, --help             Print help
```

Shows the state of each artifact of a change — done `[x]`, ready `[ ]`,
blocked `[-]` or skipped `[~]` — and whether planning is complete. Fails with
`no_active_change` when the project has no active change.

### codev instructions

```text
Print everything needed to write an artifact

Usage: codev instructions [OPTIONS] [ARTIFACT]

Arguments:
  [ARTIFACT]  Artifact identifier (default: the next one to write)

Options:
      --change <CHANGE>
      --json
  -h, --help             Print help
```

Prints everything an agent needs to write one artifact: where to write it, the
schema's instruction, the template, the artifact language, the project
context and rules with their origin, the dependencies to read first, and what
the artifact unlocks. For the `design` artifact, it also lists the
architecture decisions in effect. Without `ARTIFACT`, it picks the first
artifact that is ready to write.

The skills call it with `--json`; see
[`codev instructions`](json-output.md#instructions).

### codev sync

```text
Merge a change's deltas into the main specs without archiving

Usage: codev sync [OPTIONS]

Options:
      --change <CHANGE>  Change name (inferred when there is only one)
      --json
  -h, --help             Print help
```

Validates the change, refuses if validation reports an error, then merges its
deltas into the main specs, without moving the change. Running it again
reports the specs as unchanged. See [Sync](../workflow.md#sync).

### codev archive

```text
Merge, then move a change to the dated archive

Usage: codev archive [OPTIONS]

Options:
      --change <CHANGE>  Change name (inferred when there is only one)
      --json
  -h, --help             Print help
```

Validates the change, refuses if validation reports an error, merges its
deltas into the main specs, then moves it to
`_codev/changes/archive/<YYYY-MM-DD>-<name>/`. See
[Archive](../workflow.md#archive).

### codev validate

```text
Check changes and specs for structural errors and consistency

Usage: codev validate [OPTIONS] [ITEM]

Arguments:
  [ITEM]  Name of a specific change or spec capability

Options:
      --all      Validate all active changes and all main specs
      --changes  Validate all active changes
      --specs    Validate all main specs
      --strict   Treat any finding (warnings included) as a reason for a non-zero exit code. Useful for CI and automation
      --json
  -h, --help     Print help
```

Without `ITEM` or a flag, validates everything: every active change, every
main spec, and the decision seals. `ITEM` is the name of an active change or
the path of a capability (`ui/theme`).

Exit status:

| Result | Without `--strict` | With `--strict` |
|---|---|---|
| No finding | 0 | 0 |
| Warnings only | 0 | 1 |
| At least one error | 1 | 1 |

```text
change add-dark-mode — _codev/changes/add-dark-mode
  error   _codev/changes/add-dark-mode/change.yaml:1: zero_delta_without_marker — no delta file under `specs/` and `skip_specs: true` is not declared; add a delta or set `skip_specs: true` in change.yaml

decisions decisions — _codev/decisions
  ✓ no issues

2 item(s) validated, 1 finding(s) including 1 error(s).
```

Each finding has a stable code. See
[Validation codes](file-formats.md#validation-codes).

### codev schemas

```text
List the available workflow schemas

Usage: codev schemas [OPTIONS]

Options:
      --json
  -h, --help  Print help
```

Lists the project's schemas from `_codev/schemas/` and the built-in
`spec-driven`, with the order of their artifacts. See
[Custom schemas](../guides/custom-schemas.md).

## Decisions

### codev decision

```text
Create, accept, inspect and supersede architecture decisions

Usage: codev decision <COMMAND>

Commands:
  list       List local and inherited decisions
  show       Show a specific decision
  new        Create a new local decision
  accept     Accept a proposed decision: set its status to `accepted` and seal it
  supersede  Supersede a decision: mark it `superseded` and create a new one
  seal       Add or rewrite the seal of a local decision
  deviate    Record a local deviation from an inherited decision
  promote    Promote a `### Decision: <title>` block of a `design.md` to an ADR
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

Decisions are identified by their short id (`0007`) or their qualified id
(`project/0007`, `path:~/shared/0100`,
`git:git@github.com:acme/shared.git/0100`). See
[Decision](../concepts.md#decision).

### codev decision list

```text
List local and inherited decisions

Usage: codev decision list [OPTIONS]

Options:
      --json
  -h, --help  Print help
```

Lists local and inherited decisions with their status. Decisions in effect
are marked `•`, the others `–`; a superseded decision names the one that
replaced it.

### codev decision show

```text
Show a specific decision

Usage: codev decision show [OPTIONS] <ID>

Arguments:
  <ID>  Short (`0007`) or qualified (`path:~/shared/0100`) identifier

Options:
      --json
  -h, --help  Print help
```

### codev decision new

```text
Create a new local decision

The decision is created `proposed` and unsealed, so its body can be written freely; `codev decision accept` then accepts and seals it.

Usage: codev decision new [OPTIONS] <TITLE>

Arguments:
  <TITLE>
          Free-form title — slugified for the file name

Options:
      --status <STATUS>
          Initial status; `accepted` seals the body right away, `proposed` leaves it editable until `codev decision accept`

          [default: proposed]

      --json


  -h, --help
          Print help (see a summary with '-h')
```

Creates `_codev/decisions/NNNN-<slug>.md` from the ADR template, with the next
free number. With the default `proposed` status, the decision is not sealed:
write its body, then run `codev decision accept` to accept and seal it.
`--status accepted` seals it immediately, for a decision whose text is already
final.

### codev decision accept

```text
Accept a proposed decision: set its status to `accepted` and seal it

Rewrites the frontmatter status of a local `proposed` decision and records the hash of its body in `seal.yaml`, in the same plan — both are written or neither is. The body is left untouched. Inherited decisions and decisions that are not `proposed` are refused.

Usage: codev decision accept [OPTIONS] <ID>

Arguments:
  <ID>
          Short identifier (`0007`) — inherited ones (`path:` / `git:`) are refused

Options:
      --json


  -h, --help
          Print help (see a summary with '-h')
```

Sets the status of a local `proposed` decision to `accepted` and records the
SHA-256 of its body in `_codev/decisions/seal.yaml`, in the same plan: both are
written or neither is. Only the frontmatter changes. An inherited decision is
refused with `cannot_accept_inherited`, a decision that is not `proposed` with
`decision_not_proposed`; to replace an accepted decision, use
`codev decision supersede`.

### codev decision supersede

```text
Supersede a decision: mark it `superseded` and create a new one

Usage: codev decision supersede [OPTIONS] <OLD_ID> <NEW_TITLE>

Arguments:
  <OLD_ID>     Identifier of the decision to supersede (short or qualified)
  <NEW_TITLE>  Title of the new decision

Options:
      --json
  -h, --help  Print help
```

Creates a new accepted decision with `supersedes` pointing at `OLD_ID`, and
sets the old decision's status to `superseded`. The old body is not modified,
so its seal stays valid. To depart from an inherited decision, use
`codev decision deviate`.

### codev decision seal

```text
Add or rewrite the seal of a local decision

Without `--force`: refuses if a seal exists and the body has changed. With `--force`: rewrites the seal (use after a deliberate edit of the body).

Usage: codev decision seal [OPTIONS] <ID>

Arguments:
  <ID>
          Short identifier (`0007`) — inherited ones (`path:` / `git:`) are refused

Options:
      --force
          Rewrite an existing seal even if the body has changed

      --json


  -h, --help
          Print help (see a summary with '-h')
```

Records the SHA-256 of the decision's body in `_codev/decisions/seal.yaml`.
Sealing an already sealed, unchanged decision does nothing.

### codev decision deviate

```text
Record a local deviation from an inherited decision

Creates a local `accepted` ADR that explicitly references the inherited decision being departed from. The inherited decision stays visible in `codev decision list`, but disappears from the instructions injected into the `design` artifact. To deviate from a local decision, use `codev decision supersede`.

Usage: codev decision deviate [OPTIONS] <TARGET> <NEW_TITLE>

Arguments:
  <TARGET>
          Qualified identifier of the inherited decision to set aside

          Example: `path:~/shared/0100` or `git:git@github.com:acme/shared.git/0100`.

  <NEW_TITLE>
          Free-form title of the local deviation — slugified

Options:
      --json


  -h, --help
          Print help (see a summary with '-h')
```

### codev decision promote

```text
Promote a `### Decision: <title>` block of a `design.md` to an ADR

Extracts the block's content, creates a sealed local ADR, and replaces the block's body with a textual reference to the new ADR. Refuses an archived change.

Usage: codev decision promote [OPTIONS] <CHANGE> <TITLE>

Arguments:
  <CHANGE>
          Name of the active change whose `design.md` holds the block

  <TITLE>
          Exact title of the block to promote (what follows `Decision: `)

Options:
      --json


  -h, --help
          Print help (see a summary with '-h')
```

```bash
codev decision promote add-audit-log "Append-only audit table"
```

## Inherited sources

### codev sources

```text
Manage inherited sources (local paths and remote git repositories)

Usage: codev sources <COMMAND>

Commands:
  list    List all declared sources with their state
  update  Resolve refs, download, and update `_codev/codev.lock`
  show    Show the details of a specific source
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

See [Inherited sources](../guides/inherited-sources.md).

### codev sources list

```text
List all declared sources with their state

Usage: codev sources list [OPTIONS]

Options:
      --json
  -h, --help  Print help
```

### codev sources show

```text
Show the details of a specific source

Usage: codev sources show [OPTIONS] <TARGET>

Arguments:
  <TARGET>  URL (`git:` source) or path (`path:` source)

Options:
      --json
  -h, --help  Print help
```

### codev sources update

```text
Resolve refs, download, and update `_codev/codev.lock`

Usage: codev sources update [OPTIONS]

Options:
      --json
  -h, --help  Print help
```

The only command that uses the network. Requires `git`.
