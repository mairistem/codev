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
  update        Regenerate the skills, and refresh codev's managed sections, after a codev upgrade
  self-update   Download and install the latest codev release, verified
  doctor        Check what the project's codev setup still lacks, and how to fix it
  new           Create a new item
  list          List active changes, or specified capabilities with --specs
  status        Show the state of a change's artifacts
  instructions  Print everything needed to write an artifact
  schemas       List the available workflow schemas
  decision      Create, accept, inspect and supersede architecture decisions
  sources       Manage inherited sources (local paths and remote git repositories)
  sync          Merge a change's deltas into the main specs without archiving
  archive       Merge, then move a change to the dated archive
  changelog     Generate release notes from the archived changes
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

      --inherit <SOURCE>
          Inherit conventions from a source, written under `inherits:` (repeatable). A git URL (`git@…`, `https://…`, `ssh://…`, `file://…`, or ending in `.git`), optionally suffixed with `@<ref>` (default `main`), or a local folder. Git sources are pinned in `_codev/codev.lock` right away

      --no-claude-md
          Do not write codev's section in `CLAUDE.md`

      --no-ci
          Do not write `.github/workflows/codev.yml`

      --no-pr-template
          Do not write codev's section in the pull request template

      --no-integration
          Skip all repository integration: same as `--no-claude-md --no-ci --no-pr-template`

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
- `--inherit <SOURCE>` declares an inherited source under `inherits:`,
  once per source. A value starting with `git@`, `https://`, `http://`,
  `ssh://` or `file://`, or ending in `.git`, is a git source; an `@<ref>`
  suffix picks the branch or tag (`…/conventions.git@v2`), `main` otherwise.
  Anything else is a local folder, which must exist; a relative path is
  written as an absolute one. Git sources are pinned in `_codev/codev.lock`
  right away, as `codev sources update` would; if that fails (network,
  credentials, unknown ref), init still succeeds, warns with
  `inherit_pin_failed` and prints `help: run codev sources update`. An
  invalid value fails with `invalid_inherit` before anything is written. See
  [Inherited sources](../guides/inherited-sources.md).
- `--no-claude-md`, `--no-ci` and `--no-pr-template` skip one integration
  file each; `--no-integration` skips all three. See
  [Repository integration and codev doctor](../guides/repository-integration.md).

Unless you opt out, `codev init` also writes codev's section in `CLAUDE.md`
and, on GitHub, `.github/workflows/codev.yml` and a section in the pull
request template. The summary lists each file as created, appended,
refreshed, up to date or skipped, with the reason; for a GitLab repository it
prints a job to paste into `.gitlab-ci.yml`, which codev never edits. The last
line suggests `codev doctor` when something is still missing.

When `_codev/config.yaml` already exists, it is left untouched: `codev init`
only creates missing folders, installs or refreshes the skills, and adds or
refreshes the integration files. `--inherit` sources are then not written;
the `inherit_not_written` warning names them so you can add them by hand.
Skills you edited by hand are preserved unless you pass `--force`.

### codev update

```text
Regenerate the skills, and refresh codev's managed sections, after a codev upgrade

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

`codev update` also refreshes codev's managed sections in `CLAUDE.md` and the
pull request template, and `.github/workflows/codev.yml` when its first line
is `# codev:managed` — which moves its pinned version to the running one. It
only refreshes what exists: it never creates these files nor adds a section,
so a project initialized with `--no-integration` stays as it is.

### codev self-update

```text
Download and install the latest codev release, verified

Downloads the release archive for this platform from GitHub, verifies its SHA-256 against the release's `SHA256SUMS`, replaces this binary atomically, then runs the new binary's `codev update` in the current project, if any. Never downgrades unless `--version` asks for it. A binary installed by cargo, a cargo build or a package manager is left alone: the command that upgrades it is printed instead.

Usage: codev self-update [OPTIONS]

Options:
      --version <X.Y.Z>
          Install this version (`X.Y.Z`) instead of the latest; allows a downgrade

      --check
          Only report the running and latest versions (exit code 0 whether or not an update is available)

      --snooze
          Silence the update notice for the latest known version, for 7 days

  -y, --yes
          Do not ask for confirmation

      --json
          JSON output; implies `--yes`

  -h, --help
          Print help (see a summary with '-h')
```

Installs a newer codev in place of the running one, in one command and never
behind your back:

1. resolves the target — `--version`, or the latest release, asked to
   GitHub now whatever the cache, `CI` or `CODEV_NO_UPDATE_CHECK`;
2. on a terminal, asks for confirmation unless `--yes` is given;
3. downloads `codev-<version>-<target>.tar.gz` (`.zip` on Windows) and
   `SHA256SUMS` from the GitHub release, and verifies the archive's SHA-256;
4. writes the new binary next to the running one, then renames it into place,
   so the binary's path always holds a complete binary;
5. inside a codev project, runs the new binary's `codev update`.

A binary installed by `cargo install`, built in a cargo `target/` folder, or
installed by a package manager (Homebrew, Nix, Scoop, `/usr/bin`) is left
alone: the command that upgrades it is printed and the exit status is `0`.
The latest release older than the running binary is refused
(`update_downgrade_refused`) unless you ask for it with `--version`.

`--check` prints the running and latest versions, refreshes the update cache
and exits with `0` whether or not an update is available. `--snooze` silences
the update notice of `codev status` and of the skills for the latest known
version, for 7 days; a newer release is announced again.

| Code | Meaning |
|---|---|
| `update_check_failed` | The release API cannot be reached, or answers no release |
| `update_unsupported_platform` | No published binary for this platform; build from source |
| `update_download_failed` | The archive or `SHA256SUMS` cannot be downloaded |
| `update_checksum_mismatch` | `SHA256SUMS` has no entry for the archive, or its SHA-256 differs |
| `update_extract_failed` | The archive cannot be extracted, or holds no codev binary |
| `update_not_writable` | The binary's folder cannot be written |
| `update_downgrade_refused` | The target is older than the running version, without `--version` |
| `invalid_version` | `--version` is not `X.Y.Z` |

Every failure leaves the installed binary untouched. When `codev update`
fails in the project after a successful update, the command warns
`update_project_refresh_failed` and still exits with `0`: run `codev update`
there. For `--json`, see [JSON output](json-output.md#self-update).

### codev doctor

```text
Check what the project's codev setup still lacks, and how to fix it

Read-only and offline: configuration, skills, inherited sources, the running codev version against `codev_version` and the cached update check, specs, decisions, `CLAUDE.md`, CI and pull request template, `codev validate`, and whether `CHANGELOG.md` matches the archived changes. Each check is ok, missing or a warning, with the command or skill that fixes it.

Usage: codev doctor [OPTIONS]

Options:
      --strict
          Exit with code 1 when a check is missing (warnings never fail)

      --json


  -h, --help
          Print help (see a summary with '-h')
```

Lists what the project's codev setup still lacks, grouped under
Configuration, Planning and Repository, with the command or skill that fixes
each item. It writes nothing and needs no network; the `changelog` check runs
local, read-only `git` queries.

| Check | Passes when | Fix |
|---|---|---|
| `config.rules` | `rules:` has at least one entry, inherited ones included | `/codev-configure` |
| `config.language` | `language:` is set | add `language: <code>` |
| `skills` | every configured skill is installed and generated by this codev version | `codev update` |
| `sources` | every git source is pinned in `codev.lock` and cached | `codev sources update` |
| `codev_version` | the running codev satisfies `codev_version`, and the cached update check knows no newer release — a warning only for the latter | `codev self-update` |
| `specs` | at least one main spec exists | `/codev-propose` |
| `decisions` | at least one decision is accepted — a warning only | `codev decision new` |
| `validate` | `codev validate` reports nothing | `codev validate` |
| `claude_md` | `CLAUDE.md` has the codev section | `codev init` |
| `ci` | a GitHub workflow or `.gitlab-ci.yml` runs `codev validate` | `codev init` on GitHub |
| `pr_template` | the pull request template has the codev section — GitHub only | `codev init` |
| `changelog` | `CHANGELOG.md` matches what `codev changelog` generates — a warning only | `codev changelog --write` |

The `codev_version` check reads the update cache that `codev status` keeps;
it never contacts the network.

Each line shows `✓` (ok), `✗` (missing) or `!` (warning); the last line
counts the checks and those left to fix. The exit status is `0`, unless
`--strict` is given and a check is missing: warnings never fail. See
[Repository integration and codev doctor](../guides/repository-integration.md)
and, for `--json`, [JSON output](json-output.md#doctor).

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

`--print` is meant to be piped: when the reader stops early, as in
`codev docs --print | head`, the command stops silently and exits with code 0.
On the page, notes and warnings keep their meaning: a note has a green rule, a
warning a coral one.

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
`no_active_change` when the project has no active change. In both cases, when
`codev doctor` would report a missing check, a last `hint:` line on stderr
suggests running it.

When a newer codev release is known and you have not snoozed it, stderr also
carries ``codev X.Y.Z is available (you have A.B.C) — run `codev self-update` ``.
To know, `codev status` asks GitHub for the latest release at most once every
24 hours, with a 2-second timeout, and caches the answer in
`~/.cache/codev/update-check.json`. It never checks when `CI` is set or with
`CODEV_NO_UPDATE_CHECK=1`, and a failed check is silent. No other command
checks. With `--json`, the same information is in the `update` object — see
[JSON output](json-output.md#status).

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

### codev changelog

```text
Generate release notes from the archived changes

Lists the release-note items of the changes archived after `--since` (default: the highest `v*` tag) and up to `--until` (default: the working tree), by category, breaking items first. A change archived without a release note falls back on its What Changes bullets, with a warning. `--write` keeps the `[Unreleased]` section of `CHANGELOG.md` up to date, in a managed section; with `--release`, it becomes a dated version block.

Usage: codev changelog [OPTIONS]

Options:
      --since <REF>
          Git ref the range starts after (default: the highest `v*` tag below `--until`)

      --until <REF>
          Git ref the range ends at (default: the working tree)

      --format <FORMAT>
          Output format

          Possible values:
          - keep-a-changelog: `## [Unreleased]` (or the release) and `### <Category>` sections
          - markdown:         The same without the version heading, for a release body
          - json:             The JSON contract v1

          [default: keep-a-changelog]

      --release <VERSION>
          Title the block `## [<VERSION>] - <DATE>` instead of `## [Unreleased]`

      --date <DATE>
          Release date, `YYYY-MM-DD` (default: today)

      --write
          Update `CHANGELOG.md` (created if missing); text outside codev's managed section is kept

  -h, --help
          Print help (see a summary with '-h')
```

Lists the release-note items of the changes archived in a git range. A change
is in range when its folder is in `_codev/changes/archive/` at `--until` and
was not at `--since`: git decides, not dates. The default `--since` is the
highest `v*` tag by version order — below `--until` when `--until` is a
version tag — and, without any tag, every archived change is listed.
Outside a git repository, every archived change is listed with a warning.

Items are grouped by category in Keep a Changelog order, breaking items
first, then in archive order. A change whose note is `None` adds nothing. A
change archived without a release note contributes its What Changes bullets
under `Changed`, with a `release_note_missing` warning; a change whose deltas
remove requirements without a `Removed` or breaking item gets a
`release_note_inconsistent` warning. Warnings go to stderr and never change
the exit status. Changes listed under
[`changelog.exclude`](configuration.md#changelog) are left out.

```bash
codev changelog --since v1.3.0 --format markdown
```

```text
### Added

- `--format json` on `codev export`.

### Fixed

- `codev export` no longer drops the last line.
```

`--write` updates `CHANGELOG.md` instead of printing the text: the
`[Unreleased]` block lives in a managed section right after the file header,
and the rest of the file is never rewritten. With `--release X.Y.Z`, the block
becomes `## [X.Y.Z] - <date>` below a new empty `[Unreleased]`. `--write` is
refused with `--format markdown`. See
[Release notes and CHANGELOG](../guides/release-notes.md).

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
  supersede  Create a proposed decision that will supersede an accepted one
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

Rewrites the frontmatter status of a local `proposed` decision and records the hash of its body in `seal.yaml`, in the same plan — all writes happen or none does. The body is left untouched. The decisions listed in its `supersedes` are marked `superseded` in the same plan; each must still be `accepted`. This is how a decision created by `new`, `supersede`, `deviate` or `promote` takes effect. Inherited decisions and decisions that are not `proposed` are refused.

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

This is the only command that makes a decision take effect, whichever command
created it. When the decision lists predecessors in `supersedes` — as one
created by `codev decision supersede` does — the same plan sets each of them to
`superseded`; their body, and therefore their seal, is not touched. A
predecessor that is no longer `accepted`, for example because another decision
superseded it in the meantime, is refused with `predecessor_not_accepted`, and
nothing is written.

```bash
codev decision accept 0002
```

```text
✓ Accepted 0002 — sealed (sha256:c8873b83c178dbf8434045f43a722911b9d9913ee9f13915ccda893539b788c1)
  File: /home/you/acme-app/_codev/decisions/0002-use-sqlite.md
  Superseded: project/0001 (status: superseded)
```

### codev decision supersede

```text
Create a proposed decision that will supersede an accepted one

The new decision is created `proposed` and unsealed, with the old one in its `supersedes`. The old decision is not modified and stays in effect until `codev decision accept` accepts the new one, which marks the old one `superseded` in the same step.

Usage: codev decision supersede [OPTIONS] <OLD_ID> <NEW_TITLE>

Arguments:
  <OLD_ID>
          Identifier of the decision to supersede (short or qualified)

  <NEW_TITLE>
          Title of the new decision

Options:
      --json


  -h, --help
          Print help (see a summary with '-h')
```

Creates a new `proposed` decision, unsealed, with `supersedes` pointing at
`OLD_ID`. The old decision is not modified: it stays `accepted`, and in effect,
until you accept the new one with `codev decision accept`, which sets it to
`superseded` in the same step. `OLD_ID` must be an `accepted` local decision;
otherwise the command refuses with `predecessor_not_accepted`. To depart from
an inherited decision, use `codev decision deviate`.

```bash
codev decision supersede 0001 "Use SQLite"
```

```text
✓ Decision `0002 Use SQLite` created (proposed), to supersede `project/0001`
  New ADR:     /home/you/acme-app/_codev/decisions/0002-use-sqlite.md
  Old ADR:     /home/you/acme-app/_codev/decisions/0001-use-postgresql.md (unchanged, still in effect)

Open the new file to write the Context, Decision and Consequences sections,
then run `codev decision accept 0002` to accept and seal it.
Accepting it marks `project/0001` superseded; until then it stays in effect.
```

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

Creates a local `proposed` ADR, unsealed, that explicitly references the inherited decision being departed from. The deviation takes effect once `codev decision accept` accepts it: the inherited decision then stays visible in `codev decision list`, but disappears from the instructions injected into the `design` artifact. To deviate from a local decision, use `codev decision supersede`.

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

The deviation is created `proposed`: the inherited decision stays in effect
until you accept the deviation with `codev decision accept`. See
[Inherited decisions](../guides/inherited-sources.md#inherited-decisions).

### codev decision promote

```text
Promote a `### Decision: <title>` block of a `design.md` to an ADR

Extracts the block's content into a `proposed` local ADR, unsealed, and replaces the block's body with a textual reference to the new ADR. Review and rework the ADR, then accept and seal it with `codev decision accept`. Refuses an archived change.

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

The ADR is created `proposed`, with the block's text verbatim under
`## Decision` and placeholders for the other sections. Rework it, then accept
and seal it with `codev decision accept`:

```bash
codev decision promote add-audit-log "Append-only audit table"
```

```text
✓ Decision `Append-only audit table` promoted to proposed ADR 0004
  File:    /home/you/acme-app/_codev/decisions/0004-append-only-audit-table.md
  Source:  /home/you/acme-app/_codev/changes/add-audit-log/design.md

Review the ADR promoted from change `add-audit-log`: split its body into Context,
Decision, Consequences and Alternatives considered,
then run `codev decision accept 0004` to accept and seal it.
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
