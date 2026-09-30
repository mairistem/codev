# The workflow

Every change in a codev project goes through the same four steps. Each step
has a Claude Code skill, and — except for implementation — a CLI command that
does the underlying work.

```text
┌───────────┐    ┌───────────┐    ┌───────────┐    ┌───────────┐
│  propose  │───▶│   apply   │───▶│   sync    │───▶│  archive  │
│ plan it   │    │ build it  │    │ merge the │    │ file it   │
│           │    │           │    │ deltas    │    │ away      │
└───────────┘    └───────────┘    └───────────┘    └───────────┘
```

| Step | Skill | CLI | Writes |
|---|---|---|---|
| Propose | `/codev-propose` | `codev new change`, `codev instructions` | `_codev/changes/<name>/` |
| Apply | `/codev-apply` | — | Project code, `tasks.md` checkboxes |
| Sync (optional) | `/codev-sync` | `codev sync` | `_codev/specs/` |
| Archive | `/codev-archive` | `codev archive` | `_codev/specs/`, `_codev/changes/archive/` |

Two more skills support the cycle: `/codev-explore` to think an idea through
before proposing anything, and `/codev-update` to revise a plan that is
already written. See [Claude Code skills](skills.md).

## Propose

Proposing creates a change folder and writes its planning artifacts. With the
default `spec-driven` schema, a change has four artifacts:

| Artifact | File | Requires | Purpose |
|---|---|---|---|
| `proposal` | `proposal.md` | — | Why the change is needed, what changes, which capabilities are affected |
| `specs` | `specs/**/*.md` | `proposal` | Spec deltas: the behavior added, modified, removed or renamed |
| `design` | `design.md` | `proposal` | How to implement it: decisions, alternatives, risks |
| `tasks` | `tasks.md` | `specs`, `design` | The implementation checklist |

The dependencies form a graph: `specs` and `design` can be written once the
proposal exists, and `tasks` once both are done. `codev status` reports each
artifact as done (`[x]`), ready to write (`[ ]`), blocked on a dependency
(`[-]`) or skipped (`[~]`). State comes only from which files exist — codev
keeps no state file.

In Claude Code, `/codev-propose` does everything in one pass:

1. If your request mentions a ticket identifier such as `PROJ-123` and a Jira
   MCP server is configured, it fetches the ticket first. See
   [Jira and other MCP servers](guides/mcp.md).
2. It creates the change with `codev new change <name>`.
3. For each artifact that is ready, it runs
   `codev instructions <artifact> --change <name> --json`, studies the
   relevant code, and writes the file following the template.
4. It stops when planning is complete and presents the plan. It never starts
   implementing in the same turn.

When writing `design.md`, the skill receives the architecture decisions
currently in effect, so the design follows them — or proposes explicitly to
supersede one. See [Decisions](concepts.md#decision).

A change that has no behavioral impact — a refactor, a dependency bump,
documentation — declares `skip_specs: true` in its `change.yaml`. The `specs`
artifact is then skipped, and validation accepts the absence of deltas.

## Apply

`/codev-apply <name>` implements the plan. It reads `tasks.md`, works through
each unchecked `- [ ]` task in file order, runs the verification the task
names (a test, a command, an observable behavior), and checks the box.

It stops at the first blocker or ambiguity and asks you rather than
guessing. It modifies only the named change, never archives and never syncs:
those remain explicit steps for you to request.

`apply` is the only skill allowed to run arbitrary shell commands, because it
has to run your tests. There is no CLI equivalent: implementation is the
agent's job.

## Sync

```bash
codev sync --change <name>
```

Syncing merges the change's deltas into the main specs under `_codev/specs/`
**without** moving the change. Use it when another change in progress needs
to see a new capability in the main specs, or to review the merge before
archiving.

Syncing is idempotent: running it again reports the specs as unchanged. In
the usual case you skip this step — archiving syncs for you.

> **Warning**
> `codev sync` does not run validation first. Run `codev validate <name>`
> before syncing; `codev archive` does it for you.

## Archive

```bash
codev archive --change <name>
```

Archiving closes a change:

1. It validates the change and **refuses** if validation reports an error,
   pointing you to `codev validate <name>` for the details.
2. It merges the deltas into the main specs, exactly as `sync` does.
3. It moves the change folder to `_codev/changes/archive/<YYYY-MM-DD>-<name>/`.

The whole merge is computed and checked before the first file is written, so
an invalid change or a delta that does not apply — for example a `MODIFIED`
requirement missing from the main spec — is reported before anything on disk
changes.

After archiving, the main specs describe the behavior you shipped, and the
archived folder records why and how it changed. Commit both.

## Choosing the change

`codev status`, `codev instructions`, `codev sync` and `codev archive` accept
`--change <name>`. When the project has exactly one active change, you can
omit it. With several, codev asks you to choose:

```text
error: several active changes: add-audit-log, rework — specify which one
help: `--change add-audit-log`
```

Change names are kebab-case: lowercase letters, digits and hyphens, such as
`add-user-auth`.

## Validating

```bash
codev validate
```

Without arguments, `codev validate` checks every active change, every main
spec and the decision seals. Pass a change or capability name to check one
item, or `--changes` / `--specs` to check one family. Errors make the command
exit with status 1; with `--strict`, warnings do too — use it in CI:

```bash
codev validate --strict
```

The checks cover the structure of specs and deltas (every requirement uses
`SHALL` or `MUST` and has at least one `#### Scenario:`), consistency inside a
delta (no requirement both added and modified, no rename colliding with an
addition), the zero-delta rule, and the integrity of sealed decisions. See
[File formats](reference/file-formats.md) for the rules each file must follow.
