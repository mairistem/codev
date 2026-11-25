# Proposal: end-to-end tutorial + Mermaid diagrams

## Why

`docs/codev.md` is 693 lines across 10 sections — it is already solid, but
a visitor discovering codev finds a **reference manual** there, not
a **guided path** through their first change. They read *what* each
command does, without ever following *a concrete case* from start to finish.

In addition, several structuring relationships in codev are
currently described in prose where a diagram would make them
immediately readable:

- the **state machine of a change** (propose → apply → sync/archive)
  is described by ASCII art in section 3, but the conditional
  transitions (skip_specs, sync without archive) are not visible;
- the **crate graph** — the concrete application of ADR 0002 (dependency
  rule) — has no diagram even though it materializes the
  pure core / imperative shell boundary;
- the **lifecycle of a delta** (proposal → apply → sync → merge into
  the main spec) spans several sections without ever being
  visualized.

GitHub renders Mermaid natively in `.md` files. `pulldown-cmark`
(the library used by `codev docs`) does not render Mermaid — but a
Mermaid block degrades cleanly to a code block, which remains readable in
the embedded HTML.

This change touches **no Rust code** and no observable behavior of
the CLI: `skip_specs: true`.

## What Changes

**Modified file**: `docs/codev.md`.

**New section 3.5 — "Your first change, in five minutes"** —
a step-by-step tutorial, inserted between section 3 (The cycle) and
section 4 (The 7 skills). It simulates a concrete case: "add a
`--json` option to `codev list`". Each step carries:

- the exact command (copyable);
- the expected output (`text` block);
- one sentence of explanation.

Tutorial steps:

1. `codev init` on a blank project (optional if already done).
2. `/codev-propose add-list-json` in Claude Code — or the equivalent
   `codev new change add-list-json`.
3. Manual editing of the artifacts without Claude Code, or driven by
   the skill.
4. `codev status --change add-list-json` to confirm that planning
   is complete.
5. `/codev-apply add-list-json` — guided implementation.
6. `codev validate add-list-json` — final check.
7. `/codev-archive add-list-json` — closing, with the
   `ArchiveReportV1` output.

The tutorial MUST explicitly point to the reference sections
for details ("for the full artifact format,
see §3.1"), to avoid duplication.

**Three Mermaid diagrams** — inserted into existing sections,
with no new section:

- In **§3 (The cycle)**, just before the ASCII art: a `stateDiagram-v2`
  showing the state machine of a change (states: *proposed*,
  *applied*, *synced*, *archived*), the named transitions, and the
  guards (`skip_specs`, `validate --strict`).
- In **§5 (Concepts)**, "Architecture" subsection (to be created if
  missing): a `graph LR` of the crate graph (`codev-core` →
  `codev-engine` → `codev-cli`, with `codev-agents` on the side),
  captioned "the ADR 0002 dependency rule made visible".
- In **§5 (Concepts)**, under the "Deltas" subsection: a
  `sequenceDiagram` of the lifecycle of a delta from its birth
  (editing `specs/<capa>/spec.md` in the change) to the merge
  (`codev archive` → content written to `_codev/specs/<capa>/spec.md`).

## Capabilities

### New Capabilities

None.

### Modified Capabilities

None.

### Removed Capabilities

None.

*(Change marked `skip_specs: true` — a pure evolution of the content of
`docs/codev.md`; no observable behavior of the binary changes.
The `docs` spec covers what the `codev docs` command does, not
what the markdown contains; the two are independent.)*

## Impact

- **Code**: nothing in the Rust crates or in the scripts. Only
  markdown in `docs/codev.md`.
- **`codev docs` rendering**: the embedded HTML grows by roughly
  200-300 lines; the Mermaid blocks will appear as code
  blocks (clean degradation), with no rendering error.
- **README**: unchanged — the README's ASCII cycle stays; it plays the
  role of a "teaser" and the full tutorial lives in the docs.
- **Tests**: no automated test — the docs have no suite. A
  manual check of the Mermaid rendering on GitHub + a read-through of the
  tutorial are enough.
- **Out of scope**:
  - **Recipes/cookbook** ("how to split a capability", etc.) —
    deferred to the next cycle (`docs-cookbook`).
  - **v1 JSON contract reference** — deferred to a dedicated cycle.
  - **EN translation** — deferred.
  - **Static SVG diagrams** — Mermaid is enough as long as GitHub
    renders it.
