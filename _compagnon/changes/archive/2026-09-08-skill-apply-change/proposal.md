# Proposal: ship the `/codev-apply` skill

## Why

The repository's cycle currently works **up to planning**
(`/codev-explore` + `/codev-propose`) and **beyond validation**
(`codev sync`, `codev archive` as direct CLI). The central link is missing:
the skill that guides the agent, in the Claude Code chat, through the tasks
of `tasks.md`. Without it, the user has to request implementation with a
free-form sentence every time — with no guardrail, no invariant, no tracking.

## What Changes

- **New `apply` workflow** in the `codev-agents::workflows` catalog — a
  third entry next to `propose` and `explore`, invocable as
  `/codev-apply` once installed by `codev init`/`codev update`.
- **New asset file `assets/workflows/apply.md`** — the skill body, loaded
  at compile time via `include_str!`, like the other two.
- **Expected behavior**:
  - reads the `tasks.md` of the named change (inferred if only one is
    active);
  - implements the unchecked tasks, in file order;
  - checks `- [ ]` → `- [x]` as it goes;
  - respects a change's boundary: does not modify other changes,
    and refuses to archive or sync — that is an explicit next step;
  - `allowed-tools` permits `Bash(codev:*), Read, Write, Edit, Glob, Grep,
    Bash` (the last one for the tasks' build/test commands).

## Capabilities

### New Capabilities

- `skills`

This change **creates** the capability, with a Purpose describing the
contract of the shipped skills, and puts only an `ADDED` for `apply` in it
— the `propose` and `explore` workflows already exist as files but do not
yet have their requirement documented. A future change may add them with
an additional `ADDED`, without this change dealing with it.

### Modified Capabilities

None.

## Impact

- **Code**: new `Workflow { id: "apply", … }` in the `CATALOG` of
  `codev-agents::workflows`, plus an entry in the catalog's invariant tests
  (valid YAML frontmatter, long enough description, `Bash(codev:*)`
  present).
- **Default config**: the `codev init` comment already mentioned `apply`
  in its examples block; the default catalog (`DEFAULT_WORKFLOWS`) stays
  `[propose, explore]` — adding `apply` by default is not required for the
  skill to exist; it appears as soon as a project declares it in
  `_codev/config.yaml`.
- **Out of scope**:
  - **`update`, `sync`, `archive`** as skills — these workflows already
    exist as CLI commands (`codev sync`, `codev archive`) directly
    invocable via `Bash`. A later change (named `skill-cycle-completion`
    or similar) will ship their skills.
  - **Auto-adding `apply` to `DEFAULT_WORKFLOWS`** — a decision to be made
    separately, after feedback on real usage of the skill.
  - **Parameterized skills** (`/codev-apply --dry-run`, etc.) — Claude Code
    does not natively support skill arguments; options go through the CLI
    behind it.
