# Init Specification

## Purpose

Describes the first-time initialization experience of a project through
`codev init`: silent detection of the environment, targeted
interactive prompts, and generation of a prefilled `_codev/config.yaml`
with provenance comments. The capability puts under contract what no
spec previously pinned down — the command's behavior used to be left
to the implementation.

## Requirements

### Requirement: Interactive `codev init` detects the environment before asking questions

Before any prompt, `codev init` SHALL silently probe the current
directory and derive a `Detected` report covering:

- **Stack** — language and framework, read from `Cargo.toml`,
  `package.json`, `pyproject.toml`, `go.mod` or `pom.xml`. The first
  manifest found sets the primary stack; the others are ignored.
- **Project name** — taken from the manifest (`[package].name`, JSON
  `name`, etc.), or failing that from `git remote get-url origin`
  (last path segment without `.git`).
- **License** — from a root `LICENSE` or `LICENSE.md` file,
  recognized by regex against common names (MIT, Apache-2.0,
  BSD-3-Clause, GPL-3.0).
- **CI** — presence of a non-empty `.github/workflows/` directory.
- **Configured MCPs** — parses the following files if they exist, in
  this order, merging entries (the first one wins on conflict):
  `<project>/.mcp.json`, `~/.claude.json`,
  `<project>/.claude/settings.json`, `<project>/.claude/settings.local.json`.
  Extracts the `mcpServers` key (a map of `{name: {command? | url?, args?, env?}}`).

The probe MUST be **best-effort**: an unreadable manifest or a failing
git invocation produces a silent warning; detection continues with the
recoverable fields.

The probe MUST remain **read-only** — no file created, no process
spawned, no network call.

A `--no-detect` flag MUST bypass the probe entirely.

#### Scenario: Detection of a Rust workspace

- **GIVEN** a directory with a workspace `Cargo.toml` containing 4
  members and `[workspace.package].edition = "2024"`
- **WHEN** the user runs `codev init --yes`
- **THEN** the `context:` of the generated `_codev/config.yaml` mentions
  the Rust workspace stack, the 2024 edition, and the number of crates
- **AND** no prompt is displayed (`--yes` mode)

#### Scenario: Detection without any known manifest

- **GIVEN** a directory with no recognized manifest, no `.git/`, no
  `LICENSE`, no `.github/workflows/`
- **WHEN** the user runs `codev init --yes`
- **THEN** the command succeeds
- **AND** the generated `context:` is empty or minimal (one generic
  line)
- **AND** no error is reported because detection came up empty

### Requirement: `codev init` detects Jira/Atlassian MCPs and prefills the config

When at least one MCP server is detected whose name, command or URL
matches the case-insensitive regex `/jira|atlassian/`, `codev init`
SHALL compute a candidate `tool_id` using the Claude Code
convention:

- The server name is normalized: spaces and dots are replaced by
  `_`.
- The prefix is `mcp__`, the suffix is `__getJiraIssue`.
- Example: `"claude.ai Atlassian Rovo"` →
  `mcp__claude_ai_Atlassian_Rovo__getJiraIssue`.

The computing function MUST be pure (`codev-core::detect::mcp`), with
no network call and no introspection of the MCP server itself.

If a single unambiguous candidate emerges, interactive `codev init`
MUST display it for confirmation along with its source (`.mcp.json`,
`~/.claude.json`, etc.), and the generated `_codev/config.yaml` carries
the tool_id under the `mcp.jira_tool` key with a **provenance
comment** above it.

If several candidates emerge, the user picks one from a list.

If there are zero candidates, no question is asked and no `mcp:` key
is written. The key remains commented out in the YAML template for
future editing.

In `--yes` mode without an unambiguous candidate, no `mcp:` key is
written; in `--yes` mode with an unambiguous candidate, it is kept
without confirmation.

#### Scenario: Jira MCP detected from .mcp.json

- **GIVEN** a root `.mcp.json` containing a server named
  `"claude.ai Atlassian Rovo"`
- **WHEN** the user runs `codev init --yes`
- **THEN** the generated `_codev/config.yaml` contains
  `mcp.jira_tool: mcp__claude_ai_Atlassian_Rovo__getJiraIssue`
- **AND** a comment above it cites `.mcp.json` as the source

#### Scenario: No MCP detected

- **GIVEN** a directory without any MCP config file
- **WHEN** the user runs `codev init --yes`
- **THEN** the generated `_codev/config.yaml` contains no active
  `mcp:` key (it remains commented out in the template)
- **AND** no warning is reported

### Requirement: Interactive `codev init` asks at most two questions

Without a non-interactive flag, `codev init` MUST ask exactly the
following two questions, in this order:

1. **Workflows to install** — a choice among three presets, defaulting
   to "Full (8)":
   - `Full (8)` — `propose`, `explore`, `onboard`, `apply`, `sync`,
     `archive`, `update`, `configure`.
   - `Minimal (3)` — `propose`, `explore`, `onboard`.
   - `Custom` — the user ticks each workflow one by one.
2. **Project context** — free text added to the generated `context:`.
   The detected stack is displayed above the prompt as context. The
   user can: type a sentence, press Enter to open `$EDITOR` on a
   prefilled skeleton, or press Enter on an empty input to skip (the
   detected context then stands alone).

No other question is asked in interactive mode — neither about the
schema (only one is available), nor about MCPs (confirmed by
detection, not asked for).

#### Scenario: Two questions asked in interactive mode

- **GIVEN** an interactive TTY and a new project
- **WHEN** the user runs `codev init`
- **THEN** exactly two prompts are displayed — workflows, then
  context
- **AND** the workflows prompt has "Full (8)" as its default option

### Requirement: Non-interactive flags compose cleanly

`codev init` MUST accept the following composable flags:

- `--yes` (`-y`) — applies the defaults for every question, no
  prompt displayed.
- `--no-detect` — disables the probe.
- `--preset <full|minimal|custom>` — preselects the answer to
  question 1; in interactive mode, `custom` still triggers the
  sub-prompt, while `full` and `minimal` skip it.
- `--force` — unchanged, rewrites skills even when they were modified
  by hand.

When **stdin is not a TTY** (pipe, redirect, CI), `codev init`
MUST behave as if `--yes` had been passed — no prompt, defaults
applied. This guarantees scriptability and pipeline behavior.

#### Scenario: --yes bypasses the prompts

- **GIVEN** an interactive TTY
- **WHEN** the user runs `codev init --yes`
- **THEN** no prompt is displayed
- **AND** the 8 workflows are installed
- **AND** the generated `context:` carries only what detection
  produced

#### Scenario: Non-TTY stdin implies --yes

- **GIVEN** stdin redirected from `/dev/null`
- **WHEN** the user runs `codev init` (without an explicit `--yes`)
- **THEN** no prompt is displayed
- **AND** the installation completes as under `--yes`

### Requirement: The generated `config.yaml` carries provenance comments

`codev init` MUST write a **prefilled** `_codev/config.yaml` (rather
than a commented-out template) in which each non-trivial key carries,
on the line above it, a comment stating its source:

- `# detected from Cargo.toml` (for `context:`).
- `# detected from .mcp.json → server "claude.ai Atlassian Rovo"`
  (for `mcp.jira_tool:`).
- No comment for trivial keys (`schema:`) or keys chosen by the user
  (`workflows:`).

The generated YAML format MUST remain **readable by the existing codev
reader** (`codev-engine::config::resolve`) — no regression when
reading it back.

#### Scenario: Provenance present for context and mcp

- **GIVEN** a Rust workspace + an Atlassian `.mcp.json`
- **WHEN** the user runs `codev init --yes`
- **THEN** the generated `_codev/config.yaml` carries
  `# detected from Cargo.toml` above the `context:` key
- **AND** carries
  `# detected from .mcp.json → server "claude.ai Atlassian Rovo"`
  above `mcp.jira_tool:`
- **AND** `codev status` on this new project succeeds (read-back OK)

### Requirement: `codev init` hints at `/codev-configure` when the generated config is thin

At the end of `codev init`, the **human** output SHALL evaluate whether
the freshly written or already present `_codev/config.yaml` is
**thin** — that is, whether its `rules:` key is absent or empty.

The `context:` field is no longer part of the definition of "thin" —
the `codev init` probe systematically fills it from the detected
manifests, which makes its length useless for guessing whether the
user has actually filled in their config. `rules:`, on the other hand,
are always an explicit user choice; their presence is the only
reliable indicator.

If the config is thin, the last lines of the human output MUST
invite the user to run `/codev-configure`:

```
→ Recommended next step: in Claude Code, run /codev-configure.
  Claude will analyze the project and enrich _codev/config.yaml
  (context, per-artifact rules) — ~30 seconds.

Or skip this step and run /codev-propose <an-idea> directly.
```

If the config is not thin (the user had already written rules, or a
`codev-configure` has already run), the output keeps its current short
form: "Restart Claude Code, then run /codev-propose."

The **JSON** output MUST remain unchanged — no field added, no promise
broken. The hint is reserved for the human output, where it does not
affect scripts that consume the machine report.

#### Scenario: Config without rules triggers the hint

- **GIVEN** a new project with a minimal `Cargo.toml` (hence a
  detected `context:`, but no `rules:` written)
- **WHEN** the user runs `codev init --yes`
- **THEN** the human output contains the keyword `/codev-configure`
- **AND** the JSON output (`--json`) does not contain it

#### Scenario: Config with rules does not trigger the hint

- **GIVEN** a project whose `_codev/config.yaml` already exists and
  carries at least one entry in `rules:` (for example
  `rules: { specs: [...] }`)
- **WHEN** the user runs `codev init --yes` (idempotence)
- **THEN** the human output does not contain `/codev-configure`

#### Scenario: A long auto-detected context does not suppress the hint

- **GIVEN** a TypeScript project with many dependencies
  (auto-detected context longer than 200 characters), without `rules:`
- **WHEN** the user runs `codev init --yes`
- **THEN** the human output does contain `/codev-configure` — the
  length of the auto-detected context no longer suppresses the hint

### Requirement: `codev init` records the artifact language

`codev init` SHALL write a `language:` key in the generated
`_codev/config.yaml`: the language in which skills write the artifacts —
their prose, and their structural keywords when the keyword catalog ships
the language — as an ISO 639 code optionally followed by one subtag (`en`,
`fr`, `pt-BR`). The comment above the key MUST say that the structural
keywords follow the language when codev ships it, and stay in English
otherwise.
The value MUST come, in order, from `--language <CODE>`, from the locale
(`LC_ALL`, then `LC_MESSAGES`, then `LANG`; `C` and `POSIX` carry no
language), then from the default `en`. A provenance comment MUST name its
source. An invalid `--language` MUST be refused before anything is written,
and an invalid `language:` in an existing config MUST be reported as an
invalid configuration.

#### Scenario: Language detected from the locale

- **GIVEN** an environment where `LANG=fr_FR.UTF-8` and `LC_ALL` is unset
- **WHEN** the user runs `codev init --yes`
- **THEN** `_codev/config.yaml` contains `language: fr`
- **AND** the line above it reads `# detected from LANG=fr_FR.UTF-8`
- **AND** the comment above the key says that the structural keywords
  follow the language

#### Scenario: The flag wins over the locale

- **GIVEN** an environment where `LANG=fr_FR.UTF-8`
- **WHEN** the user runs `codev init --yes --language de`
- **THEN** `_codev/config.yaml` contains `language: de`

#### Scenario: An invalid language is refused

- **WHEN** the user runs `codev init --yes --language 'fr_FR!'`
- **THEN** the command fails and no file is written

#### Scenario: No locale, English by default

- **GIVEN** an environment with no locale variable, or only `LC_ALL=C`
- **WHEN** the user runs `codev init --yes`
- **THEN** `_codev/config.yaml` contains `language: en`

### Requirement: `codev init --inherit` declares inherited sources

`codev init` SHALL accept `--inherit <SOURCE>`, any number of times. A
SOURCE is a git source when it starts with `git@`, `https://`, `http://`,
`ssh://` or `file://`, or ends with `.git` once an optional `@<ref>`
suffix is removed; anything else is a local path. A git source MAY carry an
`@<ref>` suffix (`https://example.com/acme/conventions.git@v2`); the `@`
of a user name before the host (`git@github.com:…`) is not a ref
separator. Without a suffix the ref is `main`.

When `codev init` writes `_codev/config.yaml`, it MUST declare every
source under `inherits:`, in the order given, as `git:` + `ref:` or as
`path:`, with the comment `# set with codev init --inherit` above the
key. A relative local path MUST be written as an absolute path, so that it
resolves the same from any folder of the project, and with forward slashes
on every platform (Windows accepts them), so that the file stays readable
and identical whatever machine ran `codev init`. The written file MUST
stay readable by every other codev command.

The command MUST refuse, before writing anything, with the stable code
`invalid_inherit`:

- an empty value;
- a git source with an empty ref (`…/conventions.git@`) or an empty URL;
- a local path that does not exist or is not a folder.

When `_codev/config.yaml` already exists, `codev init` MUST NOT modify it:
the sources are not written, and a warning with the code
`inherit_not_written` names them and tells the user to add them under
`inherits:` by hand.

No source is declared when `--inherit` is absent.

#### Scenario: A git source with a ref and a local path

- **GIVEN** a new project and an existing folder `/home/you/conventions`
- **WHEN** the user runs `codev init --yes --inherit
  git@github.com:acme/conventions.git@v2 --inherit /home/you/conventions`
- **THEN** `_codev/config.yaml` contains, under `inherits:`, an entry
  `git: git@github.com:acme/conventions.git` with `ref: v2`, then an entry
  `path: /home/you/conventions`
- **AND** the line above `inherits:` reads
  `# set with codev init --inherit`
- **AND** `codev status` and `codev sources list` read the file without
  error

#### Scenario: A git source without a ref gets `main`

- **WHEN** the user runs `codev init --yes --inherit
  https://example.com/acme/conventions.git`
- **THEN** the declared source carries `ref: main`

#### Scenario: An invalid value is refused before anything is written

- **GIVEN** a folder without `_codev/`
- **WHEN** the user runs `codev init --yes --inherit ./does-not-exist`
- **THEN** the command fails with the code `invalid_inherit`
- **AND** no file or folder has been created

#### Scenario: An existing configuration is left untouched

- **GIVEN** a project whose `_codev/config.yaml` already exists
- **WHEN** the user runs `codev init --yes --inherit /home/you/conventions`
- **THEN** `_codev/config.yaml` is identical byte for byte
- **AND** a warning `inherit_not_written` names the source

#### Scenario: A relative local path is written with forward slashes

- **GIVEN** a project folder and an existing folder `../conventions` next to it
- **WHEN** the user runs `codev init --yes --inherit ../conventions`, on any
  platform
- **THEN** the declared `path:` is the absolute path of that folder, written
  with `/` separators and without quotes

### Requirement: `codev init` pins inherited git sources without failing on the network

After writing the configuration, `codev init` SHALL pin every declared git
source in `_codev/codev.lock` and fetch its content into the cache, with
the same result as `codev sources update`. Local paths are not pinned.

If pinning fails — `git` missing, network or authentication failure,
unknown ref — `codev init` MUST still succeed: the configuration and every
other file are written, the lock is left without the source, a warning
with the code `inherit_pin_failed` gives the reason, and the human output
says `help: run codev sources update`.

#### Scenario: A reachable git source is pinned

- **GIVEN** a reachable git repository whose `main` branch contains a
  `_codev/config.yaml`
- **WHEN** the user runs `codev init --yes --inherit <its URL>`
- **THEN** `_codev/codev.lock` contains an entry for the URL and ref
  `main` with the commit of `main`
- **AND** `codev sources list` reports the source as `locked`
- **AND** `codev instructions` exposes the inherited context with the
  origin `git:<its URL>`

#### Scenario: An unreachable git source leaves the source unpinned

- **GIVEN** a git URL that cannot be reached
- **WHEN** the user runs `codev init --yes --inherit <that URL>`
- **THEN** the command exits with code 0 and the skills are installed
- **AND** the source is declared in `_codev/config.yaml`
- **AND** a warning `inherit_pin_failed` is reported, followed by
  `help: run codev sources update`
- **AND** `codev sources list` reports the source as `unlocked`

### Requirement: `codev init` integrates codev into the repository

Unless opted out, `codev init` SHALL write three integration files, after
the skills:

- **`CLAUDE.md`** at the project root, created if missing, holding a codev
  section that tells Claude Code, in English: the repository follows
  spec-driven development with codev; `_codev/specs/` describes the current
  behavior and is authoritative; `_codev/decisions/` holds accepted
  decisions that are not re-debated; any change of behavior starts with
  `/codev-propose` and is implemented with `/codev-apply` (or by following
  the change's `tasks.md` when `/codev-apply` is not installed), never
  without a codev change; trivial fixes (typos, documentation wording) are
  exempt; `codev validate` runs before committing. The section lists the
  installed skills.
- **A CI workflow**, for a GitHub repository only — one that has a
  `.github/` folder, or a git remote on `github.com`:
  `.github/workflows/codev.yml`, which checks out the repository, installs
  codev with the official installer pinned to the version of the running
  binary (`CODEV_VERSION=<version>`), and runs `codev validate --strict`.
  It is not written when that file exists without the codev marker, nor
  when another workflow of `.github/workflows/` already runs
  `codev validate`; no other workflow file is ever modified. For a GitLab
  repository — a `.gitlab-ci.yml` file or a remote on `gitlab.com` —
  `.gitlab-ci.yml` is never modified and the human output prints a
  ready-to-paste job running `codev validate --strict`. Otherwise the CI
  step is skipped with a hint.
- **A pull request template section**, for a GitHub repository only: in the
  first existing template among `.github/`, the project root and `docs/`
  (file name `pull_request_template.md`, in any case), or in a new
  `.github/PULL_REQUEST_TEMPLATE.md` holding only the section. The section
  has a "codev change" line, to fill with the folder under
  `_codev/changes/` or "trivial fix, no change", and a checkbox
  "`codev validate --strict` passes".

The flags `--no-claude-md`, `--no-ci` and `--no-pr-template` MUST skip the
corresponding file, and `--no-integration` MUST skip all three. No
interactive question is added. `--no-detect` does not affect integration.

#### Scenario: A new GitHub project gets the three files

- **GIVEN** a new project with a `.github/` folder, no `CLAUDE.md` and no
  pull request template
- **WHEN** the user runs `codev init --yes`
- **THEN** `CLAUDE.md` holds the codev section and names
  `/codev-propose` and `/codev-apply`
- **AND** `.github/workflows/codev.yml` runs `codev validate --strict`
  after installing codev with `CODEV_VERSION=<running version>`
- **AND** `.github/PULL_REQUEST_TEMPLATE.md` holds the "codev change" line
  and the `codev validate --strict` checkbox

#### Scenario: A project outside GitHub gets only `CLAUDE.md`

- **GIVEN** a new project with no `.github/` folder, no `.gitlab-ci.yml`
  and no git remote
- **WHEN** the user runs `codev init --yes`
- **THEN** `CLAUDE.md` is written
- **AND** no `.github/` folder is created
- **AND** the summary reports the CI and pull request template steps as
  skipped, with the reason

#### Scenario: GitLab CI gets a snippet, not an edit

- **GIVEN** a project with a `.gitlab-ci.yml` file and no `.github/`
- **WHEN** the user runs `codev init --yes`
- **THEN** `.gitlab-ci.yml` is identical byte for byte
- **AND** the human output prints a job that installs codev and runs
  `codev validate --strict`

#### Scenario: An existing workflow already runs `codev validate`

- **GIVEN** a GitHub project whose `.github/workflows/ci.yml` runs
  `codev validate --strict`
- **WHEN** the user runs `codev init --yes`
- **THEN** `.github/workflows/codev.yml` is not created
- **AND** the summary names `ci.yml` as the reason

#### Scenario: A hand-written `codev.yml` is left alone

- **GIVEN** a GitHub project whose `.github/workflows/codev.yml` exists
  without the codev marker
- **WHEN** the user runs `codev init --yes`
- **THEN** the file is identical byte for byte
- **AND** the summary says it is not managed by codev

#### Scenario: Opting out

- **GIVEN** a new GitHub project
- **WHEN** the user runs `codev init --yes --no-integration`
- **THEN** neither `CLAUDE.md`, nor `.github/workflows/codev.yml`, nor a
  pull request template is created
- **AND WHEN** the user runs `codev init --yes --no-ci` on another new
  GitHub project
- **THEN** `CLAUDE.md` and the pull request template are written, and no
  workflow is

### Requirement: codev's text lives in managed sections that never touch user content

In `CLAUDE.md` and the pull request template, codev's text SHALL sit
between a line starting with `<!-- codev:begin` and a line
`<!-- codev:end -->` — HTML comments, invisible once the markdown is
rendered. The managed workflow file carries a `# codev:managed` comment on
its first line.

- When the file has no managed section, `codev init` appends one, separated
  from the existing content by a blank line; every byte before it is kept.
- When the file has one, `codev init` and `codev update` replace only what
  lies between the markers; every byte outside is kept. Identical content
  is not rewritten.
- When the markers are unbalanced or repeated, the file is not modified
  and a warning with the code `managed_section_malformed` names it.
- `codev update` refreshes the managed sections and the managed workflow
  file that exist, and never creates a file or appends a section: a
  project that opted out stays opted out.
- `--force` is not needed to refresh a managed section.

#### Scenario: A section is appended to an existing `CLAUDE.md`

- **GIVEN** a project whose `CLAUDE.md` holds the user's own instructions
  and no codev markers
- **WHEN** the user runs `codev init --yes`
- **THEN** `CLAUDE.md` starts with the user's instructions, unchanged
- **AND** ends with the codev managed section

#### Scenario: `codev update` refreshes only the section

- **GIVEN** a project whose `CLAUDE.md` has user text before and after a
  codev section whose content is outdated
- **WHEN** the user runs `codev update`
- **THEN** the text between the markers is the current codev section
- **AND** the user text before and after it is unchanged byte for byte
- **AND** running `codev update` a second time changes nothing

#### Scenario: `codev update` adds nothing to an opted-out project

- **GIVEN** a project initialized with `--no-integration`
- **WHEN** the user runs `codev update`
- **THEN** no `CLAUDE.md`, workflow or pull request template is created

#### Scenario: `codev update` bumps the managed workflow's version

- **GIVEN** a `.github/workflows/codev.yml` carrying `# codev:managed`,
  written by an earlier codev version
- **WHEN** the user runs `codev update` with a newer codev
- **THEN** the workflow installs the newer version

#### Scenario: Unbalanced markers leave the file alone

- **GIVEN** a `CLAUDE.md` with a `<!-- codev:begin` line and no
  `<!-- codev:end -->` line
- **WHEN** the user runs `codev init --yes`
- **THEN** `CLAUDE.md` is identical byte for byte
- **AND** a warning `managed_section_malformed` names `CLAUDE.md`

### Requirement: The init summary reports each integration step

The human output of `codev init` and `codev update` SHALL list each
integration file with what happened to it — written, refreshed, unchanged
or skipped — and, when skipped, why. `codev init` MUST end with a one-line
hint to run `codev doctor` when `codev doctor` would report a missing
check.

The JSON report of `codev init` and `codev update` MUST carry an
`integration` array, one entry per step, with the stable fields `step`
(`claude_md`, `ci`, `pr_template`), `action` (`created`, `appended`,
`refreshed`, `unchanged`, `skipped`), `path` (relative to the project, or
`null`) and `reason` (or `null`). The doctor hint is human output only.

#### Scenario: The summary explains a skipped step

- **GIVEN** a project outside GitHub
- **WHEN** the user runs `codev init --yes`
- **THEN** the summary reports `CLAUDE.md` as written and the CI step as
  skipped, with the reason
- **AND** the last line suggests `codev doctor`

#### Scenario: The JSON report lists the integration steps

- **WHEN** the user runs `codev init --yes --json` on a new GitHub project
- **THEN** `integration` holds three entries with `action: "created"`
- **AND** no entry mentions `codev doctor`
