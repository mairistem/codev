# Design: `/codev-update`

## Context

See `proposal.md` for the motivation. The pattern is the one already
proven by the five existing skills: an entry in the `CATALOG` and a
markdown file under `assets/workflows/`. The specific point is that
the skill reads **and** writes under `_codev/changes/`, like `apply`
— but without ever touching the project's code.

## Goals / Non-Goals

This design covers the skill's content, its `allowed-tools`, and an
invariant test that locks in its boundary. It covers neither batch
editing, nor a `--dry-run` mode, nor adding it to
`DEFAULT_WORKFLOWS`.

## Decisions

### Decision: `allowed-tools` = `Bash(codev:*), Read, Write, Edit, Glob, Grep`

Three writing tools: `Write` to rewrite an artifact entirely, `Edit`
for a targeted modification, and `Bash(codev:*)` to rerun
`codev validate` as the final safeguard. No general `Bash` — the skill
never runs a verification command on the project side.

**Comparison**:

| Skill | `Bash(codev:*)` | Editing | General `Bash` |
|---|---|---|---|
| `propose` | ✓ | ✓ | ✗ |
| `explore` | ✓ | ✗ | ✗ |
| `apply` | ✓ | ✓ | ✓ |
| `sync` | ✓ | ✗ | ✗ |
| `archive` | ✓ | ✗ | ✗ |
| **`update`** | ✓ | ✓ | ✗ |

The rule "only `apply` has general `Bash`" — locked in by
`workflows::apply_est_dans_le_catalogue_et_a_les_bons_outils` — stays
preserved. Directly aligned with decision
[0004](../../decisions/0004-single-identity-for-skill-and-command.md) on
the single skill/command identity: each skill does one distinct
thing, with a distinct `allowed-tools`.

### Decision: the skill reads and writes markdown by hand

It invokes **no editing CLI command** — none exists and there will be
none as long as the markdown format remains the source of truth, as
decision
[0001](../../decisions/0001-functional-core-imperative-shell.md)
requires (the core does not touch the disk). The skill uses `Edit`
for a targeted modification and `Write` for a full rewrite — a choice
left to its judgment depending on the scope of the revision.

**Rejected alternative**: a `codev update <artifact> <revision>`
command that would produce an editing plan. Useful someday, but it
requires a smarter markdown parser than the one delivered by
`parse-specs-and-deltas` (which locates blocks but does not replace
them semantically). Deferrable until the first real need.

### Decision: ripple reported, never applied as an automatic cascade

A `Write` on `proposal.md` that removes a capability leaves
`specs/<capa>/spec.md` orphaned. The skill **detects** it (a simple
walk of the `specs/` folders before and after), **reports** it to the
user, and **proposes** the next step (delete the file or run a
separate `/codev-update specs …`) — but **does not act** without
confirmation. Likewise when revising a `design.md` invalidates a task:
the skill names the task and proposes the action; the user decides.

**Rationale**: a skill that applies, in cascade, modifications the
user did not explicitly ask for ends up producing what the user did
not expect — a paternalistic pattern we have rejected since `sync`
(which invites to archive but does not archive).

### Decision: `codev validate` rerun at the end, systematically

After the last write, the skill runs `codev validate <change>` and
displays the report as-is. It is the only automatic safeguard — it
catches what the skill did not see.

**Rejected alternative**: rerun `validate` only on request. Too much
room left for "I forgot". The cost of the command is negligible.

### Decision: the skill refuses an archived change, materially

It lists `codev list` (which only shows active changes) and refuses
any name that does not appear there — even if it exists under
`changes/archive/`. An archived change is **history**; rewriting it
from a skill would compromise the trail that archiving is precisely
meant to preserve.

## Risks / Trade-offs

- **A poorly worded revision breaks consistency** between proposal
  and design without the skill noticing — especially if the change in
  meaning is subtle ("rephrase" vs "restrict"). → **Mitigation**: the
  skill reads the change's dependencies **first**, before writing, and
  `codev validate` at the end of processing surfaces structural
  defects. The real safety net remains the user reviewing the diff.
- **A user could make an architecture decision drift through a design
  revision.** → **Accepted trade-off**: the design cites the decisions
  in force (injection already in place), and the agent is expected to
  respect that constraint. The skill does nothing to prevent it — the
  social contract between the user and the agent does.
- **The ripple on tests is ignored** — a revision that replaces one
  requirement with another in `specs/` could make obsolete a line of
  `tasks.md` that cites the scenario's name. → **Accepted
  trade-off**: the skill reports that `tasks.md` cites a vanished name
  and proposes the separate call.

## Migration Plan

Not applicable — new skill. A project that already has
`workflows: [propose, explore, apply, sync, archive]` in its
`config.yaml` must add `update` to it and rerun `codev update` to
install it.
