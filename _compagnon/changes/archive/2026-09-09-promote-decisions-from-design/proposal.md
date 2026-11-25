# Proposal: promote a decision from `design.md` to an ADR

## Why

Every change's `design.md` contains a `## Decisions` section with
`### Decision: <title>` blocks describing the technical trade-offs
chosen for the implementation. Today, these decisions **stay local** to
the change: they live in the design while the change is active, and
migrate to `_codev/changes/archive/<date>-<name>/` at archive time.
**The decision index never sees them** — `codev decision list` does not
list them, `codev instructions design` does not inject them into
subsequent changes, and the equivalent ADR does not exist in
`_codev/decisions/`.

Result today: either the change author creates an ADR by hand with
`codev decision new`, duplicating the design's prose by hand (risk of
divergence); or the decision stays buried in the archived design and
can no longer be consulted like the others.

K7 closes that gap with an explicit gesture: `codev decision promote
<change> <title>` extracts a `### Decision: <title>` block from the
design, turns it into a real ADR in `_codev/decisions/`, sealed by K3,
and replaces the original block with a traceable reference.

Completes the trio K3 (immutability) / K6 (inherited deviations) / K7
(promotion) — after this, the lifecycle of a decision is complete in
codev.

## What Changes

- **New command `codev decision promote <change> <title>`** —
  extracts a `### Decision: <title>` block from the change's
  `design.md`, creates a sealed local ADR (`plan_new_decision` from
  K3), and updates the design.
- **The body of the new ADR** reproduces the block's content verbatim,
  under a single `## Decision` section. The `## Context`,
  `## Consequences`, `## Rejected Alternatives` sections of the
  standard ADR are generated with a `<!-- placeholder -->` inviting the
  author to split the content.
- **The design block is replaced** by a short textual note:
  `> Promoted to ADR **NNNN** — see `_codev/decisions/NNNN-<slug>.md`.`.
  Deliberately text, not a markdown link: preserves traceability even
  when the change goes to the archive (where a `../../decisions/`
  breaks).
- **The block's `### Decision: <title>` heading stays** — allows a
  later `promote` if the author writes several revisions; also allows
  quickly finding where the decision came from.
- **Explicit refusals**:
  - Change absent or archived (not found in `codev list`) → stable
    code `unknown_change` or `cannot_promote_from_archived`.
  - Design absent → code `design_missing`.
  - Title not found in `## Decisions` → `decision_heading_not_found`.
  - Ambiguous title (two blocks with the same title) →
    `ambiguous_decision_heading`.
- **JSON contract** — new `DecisionPromotedV1` (close to
  `DecisionCreatedV1`), with the additive field `sourceChange:
  Option<String>` to trace where the promotion comes from.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `decisions` — two new ADDED requirements: the `decision promote`
  command and the handling of the design after promotion.

### Removed Capabilities

None.

## Impact

- **Code**:
  - New lightweight parser for `### Decision: ...` blocks in a
    `design.md` — lives in `codev-engine::design` (new module — stays
    on the engine side since parsing markdown at this level makes no
    sense without I/O).
  - New `plan_promote(change_id, heading, …) -> DeviatePlan-like` in
    `codev-engine::decisions_actions`: reuses `plan_new_decision` for
    the ADR + a `write` on `design.md` for the block substitution.
  - New subcommand `codev decision promote` in the CLI.
- **JSON contract** — `DecisionPromotedV1` added, no field removed or
  renamed.
- **Files written**: new ADR under `_codev/decisions/`, entry in
  `seal.yaml`, design.md of the change in question rewritten.
- **Migration** — none. The command is opt-in; existing designs do not
  change.
- **Out of scope**:
  - **Promoting several decisions in a single command** — one call per
    decision, like `decision new` or `deviate`. Can be deferred.
  - **Automatically splitting into Context/Decision/Consequences** —
    the free prose of a design has no exploitable structure; we deliver
    the body verbatim and the author splits it by hand.
  - **Re-importing a promoted decision in case of cancellation** — a
    `codev decision supersede` or a manual edit is enough; no dedicated
    "unpromote".
