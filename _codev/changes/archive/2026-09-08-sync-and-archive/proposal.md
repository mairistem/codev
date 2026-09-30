# Proposal: close the cycle with sync and archive

## Why

A codev change can currently be created, planned, implemented and
validated — but it stays **active indefinitely**. Nothing brings it into
the main specs, nothing removes it from the changes in progress. This is
the missing link for the repository to run the complete cycle, and
therefore for codev to be useful beyond planning.

## What Changes

- **New `codev sync [change]` command**: merges a change's deltas into
  `_codev/specs/`, without moving it. The change stays active — it is used
  when a new capability must appear in the specs before another change
  relies on it.
- **New `codev archive [change]` command**: merges (like sync) then moves
  the change's folder to `_codev/changes/archive/YYYY-MM-DD-<name>/`.
  Refuses to act if `codev validate <change>` reports the slightest error —
  the validation pre-flight is the only safeguard between a consistent
  delta and a main spec.
- **Semantic merge per operation**: `ADDED` appends at the end of
  `## Requirements`, `MODIFIED` replaces the block of the same-named
  requirement character for character via its span, `REMOVED` deletes the
  whole block, `RENAMED` retitles the heading and nothing else. The file
  content the delta does not mention — comments, order, spacing, free
  section after `## Requirements` — stays **strictly unchanged**.
- **Creation of a main spec for a new capability**: when the delta targets
  a capability that does not exist yet under `_codev/specs/`, `sync`
  creates the file from the delta's `## Purpose` and `ADDED` requirements.
  A new-capability delta without a `## Purpose` would be refused — the
  pre-flight already checks it; this change merely adds the consequence.
- **Atomicity through an effect plan**: the merge first produces a
  complete plan — main specs to rewrite, main specs to create, change
  folder to move — checked in its entirety before a single write touches
  the disk. If a `MODIFIED` does not find its target requirement, no write
  happens anywhere else. This is the concrete implementation of decision
  [0001](../../decisions/0001-coeur-fonctionnel-coquille-imperative.md) for
  the tool's most delicate operation.
- **Extension of the `Plan` type**: new variant `Move { from, to }`, so
  that moving the change is part of the same plan as the writes. A
  `codev archive --dry-run` (upcoming) will therefore show both the specs
  that would change and the folder that would move, in a single view.

## Capabilities

### New Capabilities

- `spec-merge`

### Modified Capabilities

None. `validation` and `spec-parsing` are used but not modified: rules
E1–E3 serve as pre-flight, the block spans (B6) serve for targeted
rewrites. No existing public contract is touched.

## Impact

- **Code**: new module `codev-core::merge` (pure computation of edits from
  a `Delta` and a `Spec`), new modules `codev-engine::sync` and
  `codev-engine::archive`, new variant `Plan::Move`, two CLI subcommands,
  two `contract::v1` shapes (`SyncReport`, `ArchiveReport`).
- **Dependencies**: none new.
- **Out of scope**:
  - **`retire_capabilities: true`** (F5, batch 2) — a `REMOVED` that would
    empty the main spec is **refused** here, for lack of a marker allowing
    the author to take responsibility for deleting the file.
  - **Batch archive** (F7, batch 4) — `codev archive` takes one change at a
    time. Two changes that touch the same spec must be archived one after
    the other; the user chooses the order.
  - **Cross pre-flight of `MODIFIED` against the main spec** (E6, batch 2) —
    the parser does not yet compare the `MODIFIED` name with what the main
    spec contains. This change does it **at merge time** (required to
    build the plan), and reports an error before writing; it does not add
    it to the standalone validator.
  - **Cross-change conflict detection** — two active changes touching the
    same requirement do not conflict here since only one is archived at a
    time. The first archived wins; the second fails if it becomes
    inconsistent, with a message pointing to its orphan `MODIFIED`.
