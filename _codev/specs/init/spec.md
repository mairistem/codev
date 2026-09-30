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
   to "Full (7)":
   - `Full (7)` — `propose`, `explore`, `onboard`, `apply`, `sync`,
     `archive`, `update`.
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
- **AND** the workflows prompt has "Full (7)" as its default option

### Requirement: Non-interactive flags compose cleanly

`codev init` MUST accept the following composable flags:

- `--yes` (`-y`) — applies the defaults for every question, no
  prompt displayed.
- `--no-detect` — disables the probe.
- `--preset <complet|minimal|personnalise>` — preselects the answer
  to question 1 (`complet` is the full preset, `personnalise` the
  custom one); in interactive mode, `personnalise` still triggers the
  sub-prompt, while `complet` and `minimal` skip it.
- `--force` — unchanged, rewrites skills even when they were modified
  by hand.

When **stdin is not a TTY** (pipe, redirect, CI), `codev init`
MUST behave as if `--yes` had been passed — no prompt, defaults
applied. This guarantees scriptability and pipeline behavior.

#### Scenario: --yes bypasses the prompts

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
