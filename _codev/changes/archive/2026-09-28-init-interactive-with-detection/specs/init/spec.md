## Purpose

Describes the experience of initializing a project for the first time
with `codev init`: silent detection of the environment, targeted
interactive prompts, generation of a pre-filled `_codev/config.yaml`
with provenance comments. The capability contractualizes what no spec
pinned down until now — the command's behavior was left to the
implementation.

## ADDED Requirements

### Requirement: Interactive `codev init` detects the environment before asking questions

Before any prompt, `codev init` SHALL silently probe the current
folder and derive a `Detected` report covering:

- **Stack** — language and framework, read from `Cargo.toml`,
  `package.json`, `pyproject.toml`, `go.mod` or `pom.xml`. The first
  manifest found sets the primary stack; the others are ignored.
- **Project name** — taken from the manifest (`[package].name`, JSON
  `name`, etc.), or failing that `git remote get-url origin`
  (last path segment without `.git`).
- **License** — from a root `LICENSE` or `LICENSE.md` file,
  recognized by regex on the common names (MIT, Apache-2.0,
  BSD-3-Clause, GPL-3.0).
- **CI** — presence of a non-empty `.github/workflows/` folder.
- **Configured MCPs** — parses the following files if they exist, in
  this order, merging the entries (the first one wins in case of
  conflict): `<project>/.mcp.json`, `~/.claude.json`,
  `<project>/.claude/settings.json`, `<project>/.claude/settings.local.json`.
  Extracts the `mcpServers` key (map of `{name: {command? | url?, args?, env?}}`).

The probe MUST be **best-effort**: an unreadable manifest or a git
command in error produces a silent warning; detection continues with
the recoverable fields.

The probe MUST remain **read-only** — no file created, no process
launched, no network call.

A `--no-detect` flag MUST short-circuit the probe entirely.

#### Scenario: Detection of a Rust workspace

- **GIVEN** a folder with a workspace `Cargo.toml` containing 4
  members and `[workspace.package].edition = "2024"`
- **WHEN** the user runs `codev init --yes`
- **THEN** the `context:` of the generated `_codev/config.yaml`
  mentions the Rust workspace stack, the 2024 edition, the number of
  crates
- **AND** no prompt is displayed (`--yes` mode)

#### Scenario: Detection in the absence of a known manifest

- **GIVEN** a folder with no recognized manifest, no `.git/`, no
  `LICENSE`, no `.github/workflows/`
- **WHEN** the user runs `codev init --yes`
- **THEN** the command succeeds
- **AND** the generated `context:` is empty or minimal (a generic
  line)
- **AND** no error is raised because of an empty detection

### Requirement: `codev init` detects Jira/Atlassian MCPs and pre-fills the config

On detection of at least one MCP server whose name, command or URL
matches the case-insensitive regex `/jira|atlassian/`, `codev init`
SHALL compute a candidate `tool_id` via the Claude Code convention:

- The server name is normalized: spaces and dots replaced with `_`.
- The prefix is `mcp__`, the suffix is `__getJiraIssue`.
- Example: `"claude.ai Atlassian Rovo"` →
  `mcp__claude_ai_Atlassian_Rovo__getJiraIssue`.

The computation function MUST be pure (`codev-core::detect::mcp`),
with no network call nor introspection of the MCP server itself.

If a single unambiguous candidate emerges, interactive `codev init`
MUST display it for confirmation with its source (`.mcp.json`,
`~/.claude.json`, etc.), and the generated `_codev/config.yaml`
carries the tool_id under the `mcp.jira_tool` key with a **provenance
comment** above it.

If several candidates emerge, the user chooses from a list.

If zero candidates, no question is asked, no `mcp:` key is written.
The key will remain commented out in the YAML template for future
editing.

In `--yes` mode without an unambiguous candidate, no `mcp:` key is
written; in `--yes` mode with an unambiguous candidate, it is kept
without confirmation.

#### Scenario: Jira MCP detected from .mcp.json

- **GIVEN** a root `.mcp.json` containing a server named
  `"claude.ai Atlassian Rovo"`
- **WHEN** the user runs `codev init --yes`
- **THEN** the generated `_codev/config.yaml` contains
  `mcp.jira_tool: mcp__claude_ai_Atlassian_Rovo__getJiraIssue`
- **AND** a comment above it mentions `.mcp.json` as the source

#### Scenario: No MCP detected

- **GIVEN** a folder without any MCP config file
- **WHEN** the user runs `codev init --yes`
- **THEN** the generated `_codev/config.yaml` does not contain an
  active `mcp:` key (it remains commented out in the template)
- **AND** no warning is raised

### Requirement: Interactive `codev init` asks at most two questions

Without a non-interactive flag, `codev init` MUST ask exactly the
following two questions, in this order:

1. **Workflows to install** — choice among three presets, default
   "Full (7)":
   - `Full (7)` — `propose`, `explore`, `onboard`, `apply`, `sync`,
     `archive`, `update`.
   - `Minimal (3)` — `propose`, `explore`, `onboard`.
   - `Custom` — the user ticks each workflow one by one.
2. **Project context** — free text added to the generated `context:`.
   The detected stack is displayed above the prompt for context. The
   user can: type a sentence, press Enter to open `$EDITOR` on a
   pre-filled skeleton, or press Enter on an empty input to skip (the
   detected context remains alone).

No other question is asked in interactive mode — neither about the
schema (only one available), nor about the MCPs (confirmed by
detection, not asked).

#### Scenario: Two questions asked in interactive mode

- **GIVEN** an interactive TTY and a new project
- **WHEN** the user runs `codev init`
- **THEN** exactly two prompts are displayed — workflows then context
- **AND** the workflows prompt has "Full (7)" as the default option

### Requirement: Non-interactive flags compose cleanly

`codev init` MUST accept the following composable flags:

- `--yes` (`-y`) — applies the defaults for all questions, no prompt
  displayed.
- `--no-detect` — disables the probe.
- `--preset <complet|minimal|personnalise>` — preselects the answer
  to question 1; in interactive mode, `personnalise` still triggers
  the sub-prompt, `complet` and `minimal` skip it.
- `--force` — unchanged, rewrites skills even if modified by hand.

In the **absence of a TTY on stdin** (pipe, redirect, CI),
`codev init` MUST behave as if `--yes` had been passed — no prompt,
defaults applied. This guarantees scriptability and behavior in a
pipeline.

#### Scenario: --yes short-circuits the prompts

- **GIVEN** an interactive TTY
- **WHEN** the user runs `codev init --yes`
- **THEN** no prompt is displayed
- **AND** the 7 workflows are installed
- **AND** the generated `context:` carries only what detection
  produced

#### Scenario: Non-TTY stdin implies --yes

- **GIVEN** stdin redirected from `/dev/null`
- **WHEN** the user runs `codev init` (without an explicit `--yes`)
- **THEN** no prompt is displayed
- **AND** installation completes as under `--yes`

### Requirement: The generated `config.yaml` carries provenance comments

`codev init` MUST write a **pre-filled** `_codev/config.yaml` (and not
a commented template) in which each non-trivial key carries, on the
line above, a comment indicating its source:

- `# detected from Cargo.toml` (for `context:`).
- `# detected from .mcp.json → server "claude.ai Atlassian Rovo"`
  (for `mcp.jira_tool:`).
- No comment for trivial keys (`schema:`) or keys chosen by the user
  (`workflows:`).

The generated YAML format MUST remain **readable by the existing
codev reader** (`codev-engine::config::resolve`) — no regression on
reading back.

#### Scenario: Provenance present for context and mcp

- **GIVEN** a Rust workspace + an Atlassian `.mcp.json`
- **WHEN** the user runs `codev init --yes`
- **THEN** the generated `_codev/config.yaml` carries
  `# detected from Cargo.toml` above the `context:` key
- **AND** carries
  `# detected from .mcp.json → server "claude.ai Atlassian Rovo"`
  above `mcp.jira_tool:`
- **AND** `codev status` on this new project succeeds (read-back OK)
