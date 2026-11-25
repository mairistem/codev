# Proposal: `/compagnon-configure` skill + first-use hint

## Why

The previous batch (`init-interactive-with-detection`) made good progress
on the first invocation: `compagnon init` probes the environment, asks two
questions, writes a prefilled `_compagnon/config.yaml`. But in practice,
the file often stays **thin**: on a fresh project with just a
`Cargo.toml` without comments, we get a `context: | Rust project,
2024.` — useful but very poor for steering the skills.

Two limits the CLI cannot cross on its own:

- **It does not talk to an LLM** — that is a principle (`_compagnon/decisions/0001`).
  The CLI therefore cannot read the README, guess the project's code
  style, or synthesize rules.
- **It ignores unstructured context** — conventions live in
  `CONTRIBUTING.md`, in the docs, in code comments. The
  CLI is blind to them.

This is exactly where **skills** come in: they have access to
Claude, they can read an entire project and derive a rich context
from it. Today no skill has this role — `onboard` presents
compagnon, `propose` plans a change, but nothing tells Claude
"look at this project and enrich its config".

## What Changes

### New `/compagnon-configure` skill

An **8th skill**, opt-out like the other 7, which:

1. Reads the existing `_compagnon/config.yaml`.
2. Explores the project read-only — `README.md`, `CONTRIBUTING.md`,
   `docs/`, a sample of the most edited source files according to
   `git log`, the folder structure.
3. Drafts a proposed enrichment of `context:` (2-5 lines
   focused on what matters for steering the skills — stack beyond
   the language, API conventions, error style, comment tone,
   structuring library choices) and of per-artifact `rules:`
   (`specs:`, `design:`, `tasks:` — 1-2 rules each, grounded in
   what the project does).
4. Shows the diff to the user.
5. Writes the new `_compagnon/config.yaml` on confirmation — while
   **preserving** the schema, the workflows, the `mcp:` key and the
   provenance comments already present.

The skill is explicit about what it does not do: it modifies
neither the workflows, nor the detected MCPs, nor the schema. It only
touches the free-form fields (`context`, `rules`).

### Pure function `is_config_thin` in `compagnon-core`

```
pub fn is_config_thin(cfg: &ProjectConfig) -> bool {
    cfg.context.as_deref().map(str::len).unwrap_or(0) < 200
        && cfg.rules.is_empty()
}
```

Reused by the three prompting points below. Testable on its own,
a trivial change in the future if the threshold needs to move.

### Hint in `compagnon init`

Today the output of `compagnon init` ends with:

```
Restart Claude Code so it discovers the skills, then type /compagnon-propose.
```

It becomes a real **conditional** call to action — only
when `is_config_thin(&resolved_config)` is true:

```
✓ compagnon initialized — 7 skills ready.

→ Recommended next step: in Claude Code, type /compagnon-configure.
  Claude will analyze the project and enrich _compagnon/config.yaml
  (context, per-artifact rules) — ~30 seconds.

Or skip this step and type /compagnon-propose <an-idea> directly.
```

If the config is not thin (the user filled it in by hand, or
a `compagnon-configure` has already run), we keep the current short
output.

### Hint in `compagnon-onboard`

The `onboard` skill (spec `skills`, requirement "presents compagnon and
recommends the next action") gains a new branch in
its decision table: **if `_compagnon/config.yaml` has a thin context,
the recommendation becomes `/compagnon-configure` before `/compagnon-propose`**.

On a project whose user has filled in the context, `onboard` behaves
as it does today — it recommends `/compagnon-propose` first.

### Hint in `compagnon status` (new project)

On a project that is initialized but has no active change (the typical
case after a `compagnon init`), `compagnon status` currently outputs:

```
error: no active change in this project
help: create one with `compagnon new change <name>`
```

When the config is thin, add a **third line**:

```
hint: config barely filled in — /compagnon-configure can enrich it.
```

This hint appears only in human output, never in the JSON.

### `DEFAULT_WORKFLOWS` update — 7 → 8

`configure` joins the default workflows: a new user
must be able to invoke it without reconfiguring. The **Minimal** preset
(currently 3) becomes 4 with `configure` — it is a rarely used skill
but its absence would force a reinstall later.

## Capabilities

### New Capabilities

- **`configure`** — new capability describing the contract of the
  `/compagnon-configure` skill: what it reads, what it writes, what it
  preserves, how it asks for confirmation.

### Modified Capabilities

- **`skills`** — three Requirements affected:
  - The Requirement "`onboard` is part of the default catalog"
    (already rewritten in the previous batch) explicitly becomes "the
    **8 workflows** — `configure` included — are in the default".
  - An ADDED one: "`configure` skill enriches `_compagnon/config.yaml` by
    analyzing the project".
  - A MODIFIED one on the `onboard` Requirement: adds the branch
    "if the config is thin, recommend `/compagnon-configure` before
    `/compagnon-propose`".
- **`init`** — Requirement added: "`compagnon init` prompts for
  `/compagnon-configure` in human output when the generated config is
  thin". The JSON output stays unchanged to preserve the contract.

### Removed Capabilities

None.

## Impact

- **Code**:
  - Pure function `is_config_thin` in `compagnon-core::config`. Unit
    test.
  - New entry in `CATALOG` of `compagnon-agents::workflows` for
    `configure`. Invariant test.
  - Skill body in `assets/workflows/configure.md`.
  - Human output of `compagnon init` in `compagnon-cli::render`: branch
    conditional on `is_config_thin`.
  - Human output of `compagnon status` ("no active change" case):
    same branch.
  - Body of `assets/workflows/onboard.md`: new branch in the
    decision table.
  - Update of `DEFAULT_WORKFLOWS` and the associated tests.
- **JSON contract**: nothing changes. The hints live in human
  output only.
- **Backward compat**: on an existing project with a `_compagnon/config.yaml`
  already well filled in, no hint is shown — the existing user's
  experience is identical. On a project whose user
  has filled in neither `context:` nor `rules:`, a hint appears the next
  time they run `compagnon status` or `compagnon init` (idempotent, non
  intrusive — just one line).
- **Files written**: ~3 new files (configure skill body,
  configure spec section, is_config_thin function) + ~5 modified
  (workflows, onboard body, init render, status render, tests). Spec
  delta on `skills` + `init` + new `configure` capability.
- **Out of scope**:
  - **Skill that enriches `mcp:`** — MCP detection already works, no
    need for LLM enrichment.
  - **Skill that proposes `inherits:`** — too speculative, a
    user who inherits from a repository knows it.
  - **Auto-configure on first launch** — the skill stays in the
    user's hands, it is never invoked without their
    consent.
  - **Hint in `compagnon list`** — too marginal, `list` is called by
    scripts, avoid noise.
