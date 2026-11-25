# Design: remove an entire capability

## Context

See `proposal.md`. The plumbing already half exists: the
`retire_capabilities` field is reserved in `ChangeMetadata`, and the
merge error documents its future arrival. F5 makes it functional.

## Goals / Non-Goals

This design covers: the plan's `deletions` field, the new
`FileSystem::remove_file` method, the extended signature of
`merge_into_existing`, propagation into `sync/archive`, the JSON
contract, the new template section. It does not cover: the CLI flag
`--retire-capabilities`, nor automatic deletion of linked decisions.

## Decisions

### Decision: `deletions` as a first-class operation of the `Plan`

Three options:

| Option | Pro | Con |
|---|---|---|
| **A. Signal via a `FileWrite` with `contents = ""`** | Zero structural change | Ambiguous: a legitimately empty file becomes indistinguishable from a deletion; hides the destructive step |
| **B. `deletions: Vec<PathBuf>` field on `Plan`** | Semantic clarity — the plan says what it does; the executor sees the destructive step explicitly | A bit more code, a bit more in the contract |
| **C. Special `MergeOutcome::DeleteFile` return from the merger** | Strong typed signal on the core side | The shell (sync) has to re-route — more plumbing than B for the same benefit |

**Chosen: B.** A `Plan` that exposes its three categories (writes,
deletions, moves) makes the transactional execution explicit: the
shell can, one day, refuse a deletion without confirmation
(`--dry-run` mode, interactive prompt), without having to re-parse
`contents=""`. Aligned with decision
[0001](../../decisions/0001-functional-core-imperative-shell.md):
the core describes all effects; the shell applies them.

### Decision: execution order — dirs → writes → **deletions** → moves

Rationale:

- **dirs before writes**: writes require their parent directory.
- **writes before deletions**: modifying a spec `A` and deleting `B`
  are independent, but putting deletions after shrinks the
  inconsistency window if a write were to crash after a deletion.
- **deletions before moves**: `archive` moves the change folder at
  the end — a file deleted during sync must not come back to life
  because it lived in the change folder (it does not — deletions
  target `_codev/specs/`, `move` targets `_codev/changes/`).

**Rejected alternative**: deletions before writes. No spec deletion
depends on a write, but we prefer "write what is new, remove what no
longer belongs" as the human reading of the plan.

### Decision: `merge_into_existing` takes `retire_capabilities: bool`

An extended signature rather than a new function. The merger already
knows how to detect total emptying
(`removed_count >= spec.requirements.len()`) — it already turns it
into an error. The flag turns that error into a
`should_delete_spec: true` signal.

**Rejected alternative**: a separate function `merge_and_maybe_delete`.
It duplicates the logic — two places to keep consistent.

### Decision: `MergePlan` exposes `should_delete_spec`, the shell routes

The core stays unaware of the main spec's path (it only knows the
delta's text). It is the shell (`sync::plan_sync`) that, seeing
`should_delete_spec: true`, adds the deletion to the `Plan` with the
absolute path it already computed for the initial read.

Consistent with the existing code: the pure function does not build
absolute `PathBuf`s; the `Layout` (shell) does.

### Decision: the template's `### Removed Capabilities` section is documentary

The parser does not use it — the source of truth remains
`retire_capabilities` in `change.yaml`. Two reasons:

1. **There is no proposal.md parser for a "capabilities" field** — the
   proposal is free text as far as the tool is concerned; its
   structure serves the human reviewer and the agent.
2. **Decoupling action from declaration** — a `REMOVED` delta on a
   capability says what changes; the `retire_capabilities` flag says
   "and I am ready to accept the irreversible consequence". The list
   in the proposal helps document, not enforce.

### Decision: `FileSystem::remove_file` mandatory on the port

Added to the trait, with no default implementation (each
implementation must commit to it explicitly). `RealFileSystem` calls
`std::fs::remove_file`. `MemoryFileSystem` removes the entry from its
`HashMap`. Without this method, `Plan.deletions` would be a field no
port could honor.

## Risks / Trade-offs

- **A user sets `retire_capabilities: true` by mistake, on a change
  that removes nothing.** → **Accepted trade-off**: without a REMOVED
  that empties a spec, the flag is a silent no-op. No dedicated
  warning — the flag documents intent, nothing more.
- **An `apply::execute` that fails mid-plan leaves an intermediate
  state.** → **Accepted trade-off**: strict atomicity would require a
  journaling system (rollback). Too expensive for current usage;
  `codev validate` + `git status` remain the safety net.
- **A capability deleted then re-added in the same change** — an odd
  case, but possible: a total REMOVED followed by an ADDED on the same
  capability in another delta. → **Accepted trade-off**: the order of
  delta files is stable (sorted `walk_files`), so the first delta
  decides the spec's fate. We will document this case at the first
  report, not now.

## Migration Plan

None. The `Plan`'s `deletions` field is initially empty for all
existing plans; the historical behavior is bit-identical as long as
`retire_capabilities` is not set to `true`.
