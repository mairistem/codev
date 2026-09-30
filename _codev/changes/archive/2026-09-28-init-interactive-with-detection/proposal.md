# Proposal: interactive codev init with auto-detection

## Why

Today, `codev init` does the bare minimum: it creates the `_codev/`
tree, writes a template `config.yaml` with the workflows **commented
out**, and installs **three** skills — `propose`, `explore`,
`onboard`. The four others (`apply`, `sync`, `archive`, `update`) are
opt-in: one has to edit `_codev/config.yaml` by hand, uncomment the
list, and rerun `codev update`.

This default breaks discovery: a user who installs codev naturally
types `/codev-apply` after `/codev-propose`, does not find the skill,
and concludes that codev has no implementer — or that they installed
it wrong.

Three other frictions that no `codev init` handles:

- **MCP detection** — the project probably has one or more MCPs
  declared (Jira, Confluence, Figma). Today, the user has to
  copy-paste the tool ID `mcp__…__getJiraIssue` into `mcp.jira_tool`.
- **Stack detection** — `context:` stays empty, even though the
  language, the edition, the workspace crates, the license and the
  presence of a CI can be read in 5 lines.
- **No onboarding** — the first invocation ends in a YAML file full of
  comments that the user has to decode.

This change turns `codev init` into **real onboarding**: maximal
auto-detection, two targeted questions, config generated with
provenance comments.

## What Changes

### Detection (no question asked)

On startup, `codev init` silently probes the current folder and
derives:

| Field | Source | Use |
|---|---|---|
| **Stack + language** | `Cargo.toml` / `package.json` / `pyproject.toml` / `go.mod` / `pom.xml` | Base of the generated `context:` |
| **Edition, MSRV** | `Cargo.toml` `[package]` / `[workspace.package]` | Added to `context:` |
| **Project name** | manifest + `git remote get-url origin` (fallback) | Welcome log |
| **Test framework** | declared dependencies | `context:` |
| **License** | root `LICENSE` file (regex on the 3-4 common ones) | `context:` |
| **CI** | presence of `.github/workflows/` | `context:` |
| **Jira / Atlassian MCPs** | `<project>/.mcp.json`, `~/.claude.json`, `.claude/settings.json`, `.claude/settings.local.json` — `mcpServers` key, matcher `/jira\|atlassian/i` on name/command/URL | Pre-fills `mcp.jira_tool:` |
| **Git repo** | `.git/` present | Welcome log |

The server-name → tool-id resolution rule is the Claude Code
convention: spaces and dots → `_`, prefix `mcp__`, suffix `__<tool>`.
For `"claude.ai Atlassian Rovo"` + `getJiraIssue` →
`mcp__claude_ai_Atlassian_Rovo__getJiraIssue`. Pure function in
`codev-core::detect::mcp`.

### Prompts (two questions, not three)

**Question 1 — Workflows**:

```
Which workflows to install?
    > Full (7) — propose, explore, onboard, apply, sync, archive, update  [default]
      Minimal (3) — propose, explore, onboard
      Custom — choose them one by one
```

**Question 2 — Project context**:

```
Context for the skills (what cannot be inferred from the code):
    detected stack: Rust workspace (4 crates), edition 2024

    Add your conventions (Enter to open $EDITOR, or type your sentence):
    ▓
```

One sentence is enough. What detection has already filled in stays
and is preserved.

The detected MCP is **confirmed, not asked**:

```
✓ Jira MCP detected: mcp__claude_ai_Atlassian_Rovo__getJiraIssue
   (source: .mcp.json → server "claude.ai Atlassian Rovo")
```

If zero candidates: nothing to confirm, silence. If several: a short
list to choose from.

### `config.yaml` generated with provenance

Instead of the current commented template, the user sees a
**pre-filled** file in which each non-trivial field carries its
origin:

```yaml
schema: spec-driven

workflows:
  - propose
  - explore
  - onboard
  - apply
  - sync
  - archive
  - update

# detected from .mcp.json → server "claude.ai Atlassian Rovo"
mcp:
  jira_tool: mcp__claude_ai_Atlassian_Rovo__getJiraIssue

# detected from Cargo.toml — edit if needed
context: |
  Rust workspace project (4 crates: codev-core, codev-engine,
  codev-agents, codev-cli), edition 2024, MSRV 1.89. MIT license.
  GitHub Actions CI.

  <what the user typed at question 2>

# rules: to be defined over the cycles, per artifact — see docs §7.
```

### CLI flags

- `--yes` (`-y`) — applies all defaults, no prompt (useful in CI,
  scripts, or for the user in a hurry).
- `--no-detect` — disables the probe (useful for deterministic tests).
- `--preset <complet|minimal|personnalise>` — preselects the answer to
  question 1.
- `--force` — unchanged, rewrites skills even if modified by hand.
- **Without a TTY** (non-interactive stdin — pipe, redirect): implicit
  `--yes`, no prompt.

All these options compose: `codev init --preset complet --yes` uses
the preset and asks no question.

### Reversed default

`DEFAULT_WORKFLOWS` goes from `["propose", "explore", "onboard"]` to
the **full list of 7 workflows**. This stays consistent with the new
flow:

- **Interactive**: question 1 offers "Full (7)" as the default; the
  user can choose minimal.
- **`--yes`**: applies the new default → 7 workflows.
- **Missing from `_codev/config.yaml`**: `codev update` on an existing
  project applies the default → 7 workflows as well.

The `skills` spec must be updated accordingly.

## Capabilities

### New Capabilities

- **`init`** — new capability that describes the interactive contract
  of `codev init`: detection, prompts, generation with provenance,
  non-interactive flags. Today `codev init` has a behavior, but no
  spec pins it down — this change fills the gap.

### Modified Capabilities

- **`skills`** — the Requirement "`onboard` is part of the default
  catalog" is rewritten to reflect the new `DEFAULT_WORKFLOWS` (the 7
  workflows). The note "the other opt-in ones stay opt-in" is removed.

### Removed Capabilities

None.

## Impact

- **Code**:
  - New module `codev-core::detect` (pure — receives `&[u8]` of
    manifests, returns a typed `Detected`).
  - New module `codev-core::config::generate` (pure — receives
    `Detected` + user choices, returns YAML with comments).
  - Rework of `codev-cli::commands::init`: orchestration
    (sniff → prompt → generate → scaffold → install).
  - New workspace dependency: **`dialoguer`** (prompt library,
    mature, used by cargo/rustup). Rejected alternative: `inquire` —
    heavier, features not needed here.
- **JSON contract**: the `SetupOutcome`/`InitReportV1` contract gains
  an optional `detected` field that lists what was detected. Not
  breaking — additive.
- **Backward compat**: on an already initialized project, `codev init`
  detects the existing `config.yaml` and **does not re-prompt** — same
  idempotent behavior as today. Only difference: `codev update`
  honors the new `DEFAULT_WORKFLOWS`, so a project that never set an
  explicit `workflows:` suddenly receives the 4 missing skills.
  Documented in the CHANGELOG.
- **Tests**:
  - `codev-core::detect`: unit tests per manifest (fixtures of
    `Cargo.toml`, `package.json`, etc.) + MCP tests per variant of
    `.mcp.json`.
  - `codev-core::config::generate`: golden test — a fixed `Detected`
    + known choices always produces the same YAML.
  - `codev-cli::commands::init`: end-to-end `--yes` integration test
    with a `MemoryFileSystem`, plus a `--no-detect` test, plus a
    "without TTY implicit --yes" test.
- **Files written**: ~4 new code files, ~2 modified, the `init` spec
  (~150 lines), `skills` delta (MODIFIED), test fixtures, a
  `docs/codev.md` §2 entry.
- **Out of scope**:
  - **Figma / design MCP detection** — the same mechanism will apply
    when we add the `mcp.design_tool:` key, but not in this batch. The
    matching function remains extensible (list of patterns).
  - **Rule detection** (per-artifact `rules:`) — too niche; users
    discover them by editing the YAML.
  - **`codev init --update`** — a flag that would replay the prompts
    on an existing project to complete the config. Deferred to a
    future cycle if requested.
  - **Detection beyond the project root** — we do not walk up the
    tree.
