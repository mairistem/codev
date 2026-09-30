# Design: sync and archive

## Context

See `proposal.md` for the motivation. The parser provides byte-accurate
spans (B6), the validator provides the set of rules E1–E3 as pre-flight.
This design assembles these two building blocks to produce, without
writing, a merge plan checked in its entirety.

## Goals / Non-Goals

This design covers:

- the semantic merge as a **pure function** producing a list of point
  edits on the main spec;
- the extension of the [`Plan`](../../../crates/codev-core/src/plan.rs)
  type to also carry a move;
- the orchestration on the engine side (validate → merge → plan) and the
  CLI shell (sync/archive with a binary exit code).

It does not cover capability retirement (F5, batch 2) nor batch archive
(F7, batch 4). A `REMOVED` that would empty the spec is refused here; F5
will later allow that action.

## Decisions

### Decision: the merge produces `Edit { byte_range, replacement }`

Rather than rebuilding the target AST and then re-serializing it to
markdown, the merge produces a list of point edits on the main spec's
source:

- `MODIFIED` → `Edit { byte_range: req.span, replacement: <rendered block> }`
- `REMOVED`  → `Edit { byte_range: <req.span extended to the spacing>, replacement: "" }`
- `RENAMED`  → `Edit { byte_range: <heading line only>, replacement: <new heading> }`
- `ADDED`    → `Edit { byte_range: <end_of_requirements..end_of_requirements>, replacement: <rendered block> }`

The edits are then sorted by `byte_range.end` **descending**, then
applied: each application preserves the offsets of the remaining edits.

Rationale: this approach mechanically satisfies the preservation of
unmentioned content. A full rebuild would lose everything the AST does not
capture — free comments between requirements, free sections after
`## Requirements`, variable spacing. The parser's round-trip is already
tested (`spans_reproduisent_le_source_au_caractere_pres`); we rely on that
invariant rather than redoing it.

**Alternatives considered**:

- **Full rebuild** from the target AST. Simple to write, catastrophic in
  use: everything that is not modeled gets lost.
- **Successive writes to the file**, one per delta section. Offsets move
  between two writes, forcing a re-parse at each step. Costs performance
  and introduces intermediate states that atomicity forbids.

### Decision: `PlanOp::Move { from, to }` extends the `Plan` type

Moving the change to the archive joins the existing `Plan`, as a third
category next to `dirs` and `writes`:

```rust
pub struct Plan {
    pub dirs: Vec<PathBuf>,
    pub writes: Vec<FileWrite>,
    pub moves: Vec<Move>,
}

pub struct Move { pub from: PathBuf, pub to: PathBuf }
```

This keeps the central property of the architecture: a single plan goes
through the shell, a single call to `apply::execute` can decide
`--dry-run` or preview as JSON. The execution order in `apply::execute`
stays: `dirs` → `writes` → `moves`, because a move assumes that what it
moves has already been written and that its destination exists.

Rationale: cited by decision
[0001](../../decisions/0001-coeur-fonctionnel-coquille-imperative.md) — the
principle "deciding is not executing" requires that **every** disk
modification go through a plan. Handling the move on the side would reopen
the door to an inconsistent state (main spec written, folder not moved)
that this whole project was designed to eliminate.

### Decision: `archive` refuses to act if `validate` has the slightest error

`codev archive` internally calls `codev_engine::validate::validate_change`
and refuses if `has_errors()`. No merge, no write, no move. The only
dialogue with the user is a pointer to `codev validate <change>` to see
the details.

**Rationale**: the validator (previous change) already delivers all the
useful codes. Duplicating the messages in `archive` would create two
wordings of the same error, to maintain in parallel. A pointer is shorter
to write, easier to read, and impossible to contradict.

`codev sync`, on the other hand, settles for the invariants **strictly
necessary for the merge**: `MODIFIED` target present, `REMOVED` target
present, `Purpose` required if new capability, last requirement not
removed. A delta that has a `requirement_no_shall` can be synced — the
defect will then also be in the main spec, where the next
`validate --specs` will find it. This is consistent with the semantics of
a sync: "make it visible right away", as opposed to archive, which closes.

### Decision: `sync` separates `changed` from `already_up_to_date` in its report

An already merged delta (for example: `sync` then rerunning `sync` without
modifying the change) must not appear as a change — same reasoning as
`apply::execute` on identical files. The report therefore distinguishes
`updated` (file actually modified) from `unchanged` (file identical after
merge). Useful in pre-commit, and makes `codev sync` idempotent in the
strict sense.

### Decision: format of new main spec files

A new main spec is rendered according to a strict canon:

```text
# <Capability in Title Case> Specification

## Purpose

<Purpose text from the delta>

## Requirements

<ADDED blocks, one per blank line>
```

The `# <...>` title is derived from the capability path
(`identity/user-auth` → `User Auth`). It is neither parser-critical nor
user-critical — a human can fix it by hand later — but it is better than
"# spec".

**Rejected alternative**: impose an external `spec.md` template (as for
change artifacts). Adding a configuration point for a case where the
author writes the spec themselves **once created** would be ceremony; the
canon is enough to get started.

### Decision: separate `SyncReport` and `ArchiveReport` reports

Two distinct types in `contract::v1`, rather than a single `MergeReport`
with a `moved_to: Option<...>`:

```
SyncReport    { root, changeName, updated: [...], created: [...], unchanged: [...], status }
ArchiveReport { root, changeName, updated: [...], created: [...], movedTo: "...", status }
```

**Rationale**: a consumer calling `sync` never expects to see a
`movedTo`; a consumer calling `archive` always expects to see one. Two
separate types remove the conditional branch on the reader's side, at the
cost of two similar structures — a minor cost, a gain worth the duplicate.

## Risks / Trade-offs

- **End of the `## Requirements` section misidentified**. `ADDED` entries
  must be inserted at the end of this section, before any free section
  that follows. → **Mitigation**: the parser already exposes, via the AST,
  the order of the top-level `##` sections; we add an accessor that
  returns the position of the next `##` after `## Requirements`, or the end
  of the file. A dedicated test `sync::added_precede_une_section_libre_qui_suit`
  covers the case.
- **REMOVED leaves a double blank**. Deleting a block can leave two
  consecutive empty lines. → **Accepted trade-off**: we extend the
  REMOVED's `byte_range` to include the following spacing, so that a
  deletion leaves the same density as before. An extra space around a
  section is cosmetic and can be fixed by hand.
- **RENAMED of a requirement referenced by a MODIFIED in the same delta**.
  The validator already reports `modified_uses_old_name`; here we refuse in
  case of residual inconsistency. → **Rationale**: the double check is
  cheap and avoids silent disaster.
- **Cross-device `Move`**. On macOS/Linux, `rename(2)` fails across
  different volumes. → **Mitigation**: copy + remove fallback in the
  `FileSystem` port, with a test that simulates the `rename` failure in
  memory. Unlikely in practice (planning and code live in the same repo),
  but a test documents the fallback behavior.

## Migration Plan

Not applicable — new capability, no existing consumer. The
`parse-specs-and-deltas` and `validate-changes-and-specs` changes already
present in this repository will become the first **archive candidates**
once this change itself is applied: they already pass
`codev validate --all`.
