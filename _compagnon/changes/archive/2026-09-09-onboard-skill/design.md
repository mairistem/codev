# Design: `/compagnon-onboard`

## Context

See `proposal.md`. One more skill added to the catalog, on the same
pattern as the six existing ones — but with two specifics: a strictly
informative role (read-only) and a place in the default catalog.

## Goals / Non-Goals

This design frames the skill's content, its `allowed-tools`, its place
in `DEFAULT_WORKFLOWS`, and the two invariant tests. It does not frame
an interactive tutorial or MCP auto-detection.

## Decisions

### Decision: `onboard` enters `DEFAULT_WORKFLOWS`

It is the only skill whose role is to **explain itself** and to guide
a user who has asked for nothing. Hiding it behind an opt-in would be
absurd: whoever would need to discover it would not know how to enable
it.

**Rejected alternative**: keep `DEFAULT_WORKFLOWS = ["propose",
"explore"]` and leave `onboard` as opt-in. Rejected — it breaks the
very principle of the skill. A new user does not think of editing
`_compagnon/config.yaml` before invoking a skill.

### Decision: `allowed-tools = Bash(compagnon:*), Read, Glob`

Comparison with the six others:

| Skill | `Bash(compagnon:*)` | `Read` | `Glob` | `Grep` | `Write` | `Edit` | General `Bash` |
|---|---|---|---|---|---|---|---|
| `propose` | ✓ | ✓ | ✓ | ✓ | ✓ | ✗ | ✗ |
| `explore` | ✓ | ✓ | ✓ | ✓ | ✗ | ✗ | ✗ |
| `apply` | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `sync` | ✓ | ✓ | ✗ | ✗ | ✗ | ✗ | ✗ |
| `archive` | ✓ | ✓ | ✗ | ✗ | ✗ | ✗ | ✗ |
| `update` | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✗ |
| **`onboard`** | ✓ | ✓ | ✓ | ✗ | ✗ | ✗ | ✗ |

`Read` + `Glob` without `Grep`: the skill does not search for patterns
in code — it looks at well-known paths (`_compagnon/config.yaml`,
`_compagnon/decisions/`, listing of active changes). No need for grep.
**The rule "only `apply` has general `Bash`"** is preserved.

### Decision: the skill reads the CLI's **human** output

`compagnon list`, `compagnon list --specs`, `compagnon status <change>` — without
`--json`. Consistent with `apply` and `update`: these skills are
conversational, they do not consume a structured contract. The human
output is shorter, more readable for the agent, and introduces no
dependency on a versioned format.

**Rejected alternative**: read JSON to be robust. Useful if the human
output ever changed without warning — but it is stable in practice,
and the other "guide" skills (`apply`, `update`) do not consume JSON
either.

### Decision: the skill **never runs** `compagnon init`

A user discovering compagnon on an uninitialized repository might expect
the skill to do it for them. It **refuses**: `init` is a disk write,
non-trivial (full scaffolding), and requires an explicit intention from
the user. The skill therefore displays `compagnon init` as a
**suggestion**, not an action.

**Aligned** with the decision to separate gesture and action — the
same philosophy as `sync`, which invites archiving but does not
archive.

### Decision: the skill copes with an uninitialized repository

A `compagnon list` on a folder without `_compagnon/` surfaces an error with a
known stable code (`no_compagnon_root`). The skill intercepts this case as
**information** — not an error — and switches to the "suggest
`compagnon init`" branch. The final rendering stays useful.

### Decision: the action recommendation depends on a simple case tree

Five branches, mutually exclusive, resolved in order:

```
no _compagnon/             → compagnon init
no active change       → /compagnon-propose <idea>
1 change, planning OK  → /compagnon-apply <name>
1 change, planning KO  → /compagnon-propose <name> (continue)
≥ 2 active changes     → list them, let the user choose
```

This tree lives in the skill's markdown body, not in Rust code. The
agent executes it — the skill says **what to read** and **what to
recommend accordingly**, the agent looks, chooses, answers.

## Risks / Trade-offs

- **The skill says "run `compagnon init`" but the user has not installed
  the binary.** → **Accepted trade-off**: without `compagnon` in the PATH,
  the skill could not have been installed by a `compagnon update`.
  Improbable edge case, not handled.
- **The skill becomes dense when the project has many specs and
  changes.** → **Mitigation**: the rendering does **not** list each
  spec or each decision individually — it gives **counts** ("5 specs",
  "6 decisions"). A user who wants the detail runs `/compagnon-explore` or
  the commands `compagnon list --specs` / `compagnon decision list`.
- **A future evolution of the catalog breaks the invariant contract
  "the body cites the three blocks".** → **Handled** by a dedicated
  test `onboard_cite_ses_trois_blocs` that checks the presence of the
  expected keywords ("description", "state", "action" — to be adjusted
  according to the final wording).

## Migration Plan

None. Projects that explicitly declare `workflows:` in their
`config.yaml` keep their list; they add `onboard` when they want. New
projects (without a `workflows:` key) get the skill on the first
`compagnon init` / `compagnon update`.
