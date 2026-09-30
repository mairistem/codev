# Design: CLI commands for decisions

## Context

See `proposal.md` for the motivation. The `codev-engine::decisions`
module delivered by the previous change already provides the index with
`in_effect` and qualified resolution. This change makes it the central
resolution building block: each command (`new`, `list`, `show`,
`supersede`) starts from an up-to-date index and computes a `Plan` to
execute.

## Goals / Non-Goals

This design covers:

- where the plan-computing functions live (pure core vs shell);
- the shape of the new public contract types;
- the atomicity of `supersede`, which rewrites two files.

It does **not** cover immutability (K3), deviation (K6), promotion from
design (K7), or interactive editing in `$EDITOR`.

## Decisions

### Decision: the `plan_new` and `plan_supersede` functions live in `codev-engine`

They do not write — they produce a `Plan`, like `plan_init` or
`plan_new_change`. Follows directly from decision
[0001](../../decisions/0001-functional-core-imperative-shell.md):
deciding is not executing. The CLI shell applies the plan via
`apply::execute`, which puts the idempotence and `--dry-run` mechanism to
use (the latter will arrive when a consumer asks for it).

**Rejected alternative**: put the computation in `codev-core::decisions`.
Blocked: generating the next `id` requires knowing the decisions already
present, hence the result of the index — which is on the engine side
(disk reads). Impossible to stay pure without going through the port.

### Decision: the ADR skeleton is embedded, not configurable

The file `assets/templates/decision.md` is included in the binary via
`include_str!` and interpolated with the `id`, title, status and date at
creation time. This is aligned with how artifact templates already work
— cf. decision
[0002](../../decisions/0002-crate-graph-as-dependency-rule.md),
which prefers fixed data over premature extension points.

**Accepted cost**: a project that wants a different decision skeleton will
have to write it by hand. Templating the file via
`_codev/templates/decision.md` will be a future change at the first real
need.

### Decision: `supersede` rewrites the predecessor's frontmatter, never its body

The old ADR is rewritten with a new frontmatter — `status:
superseded` — but everything after the second `---` stays identical down
to the character. The parser already carries `frontmatter_span` on
`Decision`, so the operation is a one-off replacement of the range
`[0..frontmatter_span.end)` with the new frontmatter. Same mechanism as
`merge::apply_edits` — same invariant, testable by golden test.

**Rationale**: the body is what the author wrote. The tool rewrites a
status, not a decision.

### Decision: `supersede` refuses to touch an inherited decision

An inherited decision is **read-only** — decision
[0005](../../decisions/0005-read-only-inherited-sources.md). The
command refuses with code `cannot_supersede_inherited` and suggests
deviation (K6, upcoming). This is the first time reads and writes are
distinguished by origin; the future `deviate` and `promote` commands will
rely on the same rule.

### Decision: the ADR skeleton contains only the useful sections

Four sections: **Context**, **Decision**, **Consequences**,
**Rejected alternatives**. These are the ones used by the repository's
six existing ADRs. An ADR is short by nature — imposing more is noise.

### Decision: the contract types are three distinct structs

- `DecisionV1` — the full shape, used by `list` (each entry of the array)
  and by `show` (at the root level).
- `DecisionCreatedV1` — returned by `new`, contains `decision:
  DecisionV1` plus `path: String`.
- `DecisionSupersededV1` — returned by `supersede`, contains `newDecision`
  and `oldId` with `oldPath`.

Three structs rather than a single flexible one: a consumer calling `new`
has no use for the `oldId` field, and vice versa. The compiler catches the
wrong command on the agent side before the wrong key gets into
circulation.

## Risks / Trade-offs

- **Concurrent id generation**. Two `codev decision new` runs launched at
  the same time would compute the same `id` and one would overwrite the
  other. → **Mitigation**: `plan_new` produces a `WriteMode::CreateOnly`
  on the output file. The second exec fails with `already_exists`. Rare
  in practice (human use, not batch) and reported rather than masked.
- **Title with unusual Unicode characters** (emoji, foreign punctuation).
  The slug drops these characters, and the file name may become short or
  empty. → **Accepted trade-off**: if the slug is empty after cleanup,
  use `decision` as a fallback (`0007-decision.md`). Documented in the
  rendering.
- **Predecessor body with `\r\n`**. Reading then rewriting may normalize
  to `\n` inadvertently. → **Mitigation**: the content after
  `frontmatter_span.end` is carried over as is, byte for byte — same
  logic as `merge::apply_edits`.

## Migration Plan

Not applicable — new commands. The repository's six existing ADRs will
serve as the first real test when a seventh decision is created with the
new command — a direct check that the "+ 1" numbering lands on `0007`.
