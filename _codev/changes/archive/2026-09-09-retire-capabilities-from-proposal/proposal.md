# Proposal: remove an entire capability from a proposal

## Why

Today, a proposal can **add** (`### New Capabilities`) or **modify**
(`### Modified Capabilities`) a capability, but it has **no complete
mechanism to remove one**. A delta's `## REMOVED Requirements` already
exists — but if all the requirements of a capability are removed, the
tool **refuses the merge** with
`would_leave_spec_without_requirement`: "the
`retire_capabilities: true` marker (coming soon) will be required to
remove the capability".

The marker is already reserved in `ChangeMetadata` but nobody reads
it. F5 delivers the feature behind it: when a change removes all the
requirements of a spec **and** declares `retire_capabilities: true`,
`sync`/`archive` **deletes** the file `_codev/specs/<capa>/spec.md`
instead of leaving it empty or refusing.

This closes the CRUD loop on specs — without this step, an obsolete
capability either lingers in the main specs, or forces the user to
delete the file by hand (which bypasses atomicity and leaves the
change's history incomplete).

## What Changes

- **New `### Removed Capabilities` subsection** in the proposal
  template, next to the `New` and `Modified` sections. Its content
  documents the capabilities the change removes — useful for human
  review, not a source of truth for the tool.
- **The source of truth remains `retire_capabilities: true`** in
  `change.yaml`. Without this marker, a `## REMOVED Requirements` that
  would empty a main spec is **refused** as today — an irreversible
  step, hence an explicit opt-in.
- **New `deletions: Vec<PathBuf>` field on `Plan`** (core), so that a
  file deletion is a first-class operation of the atomic plan — same
  granularity as `writes` and `moves`.
- **New `FileSystem::remove_file(&Path)` method** on the port, with a
  real implementation (`std::fs::remove_file`) and an in-memory one.
- **`merge_into_existing` extended** — returns a `MergePlan` that now
  exposes a `should_delete_spec: bool`. True when all requirements
  have been removed **and** `retire_capabilities: true` is passed as
  an argument. The old `WouldLeaveSpecWithoutRequirement` error is
  kept for the case without the flag.
- **`sync` propagates the deletion** — `SyncPlan` gains `deleted:
  Vec<PathBuf>`; so does `SyncOutcome`; so does the JSON contract.
- **`archive` inherits the same behavior** (it goes through `sync`).

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `spec-merge` — three new ADDED requirements: atomic deletion of an
  emptied spec, propagation into `sync`/`archive`, the plan's
  `deletions` mechanism.

### Removed Capabilities

None. (Documentation-only — useful to showcase the new section in the
template as soon as it ships.)

## Impact

- **Code**:
  - `codev-core::plan::Plan` gains `deletions: Vec<PathBuf>` and
    `Plan::delete()`.
  - `codev-core::merge::MergePlan` gains `should_delete_spec: bool` and
    `merge_into_existing` takes a `retire_capabilities: bool` argument.
  - `codev-engine::ports::FileSystem` gains `remove_file(&Path)` (with
    `RealFileSystem` and `MemoryFileSystem`).
  - `codev-engine::apply::execute` applies deletions **after** the
    writes and **before** the moves.
  - `codev-engine::sync::SyncPlan/SyncOutcome` gain `deleted:
    Vec<PathBuf>`.
- **JSON contract** — `SyncReportV1` and `ArchiveReportV1` gain
  `deleted: Vec<String>` (additive, always present, empty in the usual
  case). No field removed or renamed.
- **Proposal template** — `assets/schemas/spec-driven/templates/proposal.md`
  gains the `### Removed Capabilities` section.
- **Documentation** — the `retire_capabilities: true` line of
  `ChangeMetadata` gains a usage example in its comment.
- **Migration** — none. Without the flag, behavior is unchanged (the
  historical refusal remains).
- **Out of scope**:
  - **A `codev new change --retire-capabilities` flag** — the user
    edits `change.yaml` by hand in this rare case; no extra ergonomics
    until a need shows up.
  - **Restoring a capability removed by a previous change** — no
    undo. An ADR or an ADDED in the next proposal does the job.
  - **Also deleting the decisions linked to the removed capability**
    — out of scope; the decisions remain (they are history).
