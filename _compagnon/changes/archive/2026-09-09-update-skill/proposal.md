# Proposal: deliver `/codev-update`

## Why

On this very repository, in recent changes, I edited a planning
artifact by hand **at least four times**: a proposal revision after
discussion, a design adjustment, a tasks correction once `validate`
was real. Each time: open the file, edit, rerun `codev validate` from
memory, and cross my fingers that I had not broken consistency
between proposal, design and tasks. It is the pattern that recurs
most after the five delivered skills, and the only one missing from
the *fluid* cycle the README announced.

## What Changes

- **New `update` workflow** in the catalog, invocable as
  `/codev-update` once installed. It revises an already-written
  planning artifact (proposal, specs, design, tasks) — one at a time,
  guided by the user's description, while preserving consistency with
  the others.
- **New file `assets/workflows/update.md`** — the skill's body,
  loaded at compile time via `include_str!` like the five others.
- **Strict boundary**: the skill modifies **only** the files under
  `_codev/changes/<name>/` and **never** project code. Nor does it
  create a missing artifact — that is the role of `/codev-propose`.
- **Ripple announced, not hidden**: when revising one artifact makes
  another inconsistent (for example, removing a capability from the
  proposal while a `specs/<capa>/` file has already been written), the
  skill reports it to the user and proposes the fix before acting.
- **`codev validate` as the final safeguard**: after applying the
  revisions, the skill reruns `codev validate <change>` and displays
  the result.
- **Human output** — the skill reads the text output of the commands
  it invokes, without parsing JSON. A choice consistent with `apply`,
  which is of the same nature (it guides the agent, does not consume
  a structured contract).

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `skills` — three new ADDED requirements describing the contract of
  `/codev-update`: revising an artifact, announced ripple,
  planning-only boundary.

## Impact

- **Code**: new `Workflow { id: "update", … }` entry in the `CATALOG`
  of `codev-agents::workflows`, and a dedicated test that checks its
  presence and its `allowed-tools`.
- **Config**: add `- update` to the `workflows` list of this project's
  `_codev/config.yaml`.
- **Default catalog**: do NOT add `update` to `DEFAULT_WORKFLOWS` —
  consistent with the current line that limits the default catalog to
  what prepares the work (`propose`, `explore`).
- **Out of scope**:
  - **Batch editing** of several artifacts in a single call — each
    `/codev-update` call targets one artifact and its ripple; a
    broader revision is done in several sequential invocations.
  - **Code modification** — the boundary is strict. A code adjustment
    is done via `/codev-apply` after revision.
  - **Creating missing artifacts** — it is `/codev-propose` (or
    `/codev-continue` from the extended profile) that creates them.
  - **Editing an already-archived change** — technically possible by
    modifying the files under `changes/archive/`, but the skill
    refuses. An archived change is history; correcting it requires
    un-archiving it by hand.
