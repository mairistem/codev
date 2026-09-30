# Proposal: architecture decisions as first-class objects

## Why

The repository already contains 6 ADRs under `_codev/decisions/` — written
by hand, read by humans, cited by hand in the changes' `design.md`. This is
the foundation the README announces and the reason codev exists rather than
OpenSpec. Yet today no codev code *sees* them: they are not indexed, they
are not injected into the `design` instructions, and an agent drafting a
design may very well silently re-litigate a choice already settled. This
change delivers the building block that makes decisions **usable by the
tool**.

## What Changes

- **Recognized ADR format** — YAML frontmatter (`id`, `title`, `status`,
  `date`, `tags`, optional `supersedes`) followed by free-form sections.
  The format already used by the repository's 6 ADRs.
- **Pure parser** in `codev-core::decisions` — takes the text, returns a
  typed `Decision`, without I/O.
- **Decision index** in `codev-engine::decisions` — reads the
  `_codev/decisions/` of the project and of each `path:` inherited source,
  resolves the `supersedes` links → computes for each decision its effect
  state (`in_effect` / `superseded_by(<id>)` / `superseded_by(<source>/<id>)`).
- **Recognized statuses**: `accepted`, `superseded`, `proposed`,
  `deprecated`, `rejected` — only `accepted` and `superseded` take part in
  the effect computation. The others are exposed as is in the index.
- **Injection into the `design` instructions** — the call `codev
  instructions design --change <name>` gains a new `decisions[]` field
  carrying the decisions **in effect** (those not superseded by any other).
  Each entry carries `id`, `title`, `status`, `tags`, a relative `path`, and
  `origin` (`project` or `path:<path>`). The full content stays in the
  file — the agent reads it through `path`, like dependencies.
- **Human rendering of `codev instructions design`** enriched with a
  "Decisions in effect" section that lists the entries, one line each.

## Capabilities

### New Capabilities

- `decisions`

### Modified Capabilities

- `skills` — the `propose` workflow (and possibly `apply` later) keeps
  reading `codev instructions`. The shape of the contract gains a field,
  without breaking what exists. No requirement changes, hence no MODIFIED
  delta on this capability.

## Impact

- **Code**: new module `codev-core::decisions` (parser + AST), new module
  `codev-engine::decisions` (index + supersession), extension of
  `codev-engine::instructions::Instructions` with a `decisions:
  Vec<DecisionRef>` field, new `DecisionRefV1` in `contract::v1`, extended
  human rendering in `codev-cli::render`.
- **Dependencies**: none new — the YAML frontmatter is already handled by
  `serde_norway`, and section parsing uses the machinery of the `parser`
  module.
- **Inherited sources**: when an `inherits: path: X` is declared, the ADRs
  in `X/_codev/decisions/` are merged into the index, with visible
  provenance. Precedence order: project last (so it "wins" for the same
  `id` — a rare case, to be reported as a conflict).
- **Out of scope**:
  - **K3 — immutability** of an `accepted` decision: detecting a
    modification requires a reference hash, whose regeneration in turn
    requires a CLI command (`codev decision seal`) that does not exist
    yet. Deferred along with K5 to stay consistent.
  - **K5 — CLI commands** `codev decision new/list/show/supersede`:
    the user keeps creating and editing ADRs by hand for this change.
    Ergonomically sufficient for dogfooding, to be improved later.
  - **K6 — deviation** `deviates-from` from an inherited decision: requires
    K5 upstream to cleanly create a deviation decision.
  - **K7 — promotion** from `design.md`: requires a guided write path that
    overlaps with K5.
  - **Injection into the instructions of artifacts** other than `design`:
    the index exists on the engine side; a future change can expose it
    elsewhere (for instance in `proposal`, to recall the decisions that
    bound the scope). Nothing blocking, just outside strict K4.
