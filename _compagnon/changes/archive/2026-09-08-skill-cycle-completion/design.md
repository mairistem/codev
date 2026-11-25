# Design: `/compagnon-sync` and `/compagnon-archive`

## Context

See `proposal.md` for the motivation. The pattern is the one already proven
by `propose`, `explore` and `apply`: two pairs {entry in `CATALOG`,
markdown file under `assets/workflows/`}. The difference in nature from
`apply` is that these skills write nothing themselves — they delegate the
entire effect to the compagnon binary, whose "plan then execute" pattern,
framed by decision
[0001](../../decisions/0001-functional-core-imperative-shell.md),
guarantees atomicity on the disk side.

## Goals / Non-Goals

This design covers:

- the content of the two skills (structure, guardrails, handling of errors
  reported by the CLI);
- the restricted `allowed-tools` contract and why it is restricted;
- the invariant test that locks in the restriction.

It does **not** cover adding to `DEFAULT_WORKFLOWS`, nor the `update`
skill, nor parsing the JSON output of commands.

## Decisions

### Decision: `allowed-tools` = `Bash(compagnon:*), Read`

The only effect of these skills is a `compagnon sync` or `compagnon archive` call —
the rest is just text displayed to the user. No `Write`, no `Edit`, no
general `Bash`. `Read` remains useful for answering a context question
from the user ("what does `tasks.md` say?"), without ever writing.

**Rationale**: each extra tool in `allowed-tools` widens what the skill can
do by mistake. Restricting is safer than opening "just in case". The
testable invariant — that general `Bash` only appears in `apply` — becomes
the structural guarantee.

**Rejected alternative**: add `Bash(git:*)` so that the skill can suggest
a `git status` after archive. Can be deferred — the user knows how to run
`git status` on their own. If the need grows, we will open it then, not
before.

### Decision: the skill reads the JSON, not the human output

`compagnon sync` and `compagnon archive` both produce a structured `--json`, a
versioned, snapshot-tested contract (`SyncReportV1`, `ArchiveReportV1`).
The skill invokes it and reads that shape rather than the human text.

**Rationale**:

1. The JSON contract is **precisely made to be consumed** — that is its
   reason for being. Snapshot-testing it in `contract::tests` without a
   first consumer would be a waste of an invariant.
2. Structured rendering lets the skill compose cleanly — "✓ 2 specs
   created, 1 updated, 0 unchanged, moved to …" rather than relaying a
   patchy block of text.
3. The stable code in `status[0].code` is directly testable on the agent
   side — `validation_failed` triggers exactly the "point to
   `compagnon validate`" branch, without having to grep the human message.

**Rejected alternative**: read the human text. Appealing for its loose
coupling, but fragile in practice: any well-meant rewording would make the
interpretation drift. The JSON is stable *by contract*, and that stability
is what is worth exploiting.

**Accepted cost**: the skill becomes dependent on the JSON's shape. Since
the contract is versioned (v1), a change of shape will require a new
version — which is exactly what versioning guarantees.

### Decision: `archive` explicitly points to `validate` on refusal

When `compagnon archive` refuses because of the validate pre-flight, the skill
does not retry, does not guess, does not "fix". It says exactly:
"The change has validation errors; run `compagnon validate <name>` to see the
details."

This is consistent with the design of `compagnon archive` itself, which
already refuses to duplicate rule messages. The skill carries on this
division of labor: `compagnon archive` handles the action, `compagnon validate`
explains.

### Decision: `sync` ends with a non-directive invitation to archive

When the merge produced a change (at least one `created` or `updated`),
the skill adds **a single line** at the end of the rendering:

> The change is ready to be archived if you want to close the cycle.

It **suggests nothing** when the merge is a no-op (all `unchanged`) —
there is then nothing new to archive beyond what already was.

**Rationale**: archiving is the natural step after a sync that modified
the main specs, and recalling that `/compagnon-archive` exists saves the user
from having to remember the next verb on their own. The phrasing
"if you want" removes the paternalism — it is a reminder, not an order.

**Accepted cost**: the skill becomes sensitive to the `updated`/`created`
field of the JSON report. This is aligned with the previous decision:
since we read the JSON, we may as well use it to make this rendering
decision.

## Risks / Trade-offs

- **A user could invoke `/compagnon-archive` without having read the result of
  their `/compagnon-apply`**. → **Accepted trade-off**: the `validate`
  pre-flight built into `compagnon archive` reports the errors, and the refusal
  to archive interrupts the process. A blind archiving that fails is safer
  than an archiving that would go through silently.
- **Two active changes declaring `skills` as a "new capability"** —
  cf. proposal. On archiving, the first creates the spec, the second
  enriches it via ADDED. Nothing to do on the design side. A unit test of
  the merger already covers the "ADDED on existing main spec" case.

## Migration Plan

Not applicable — two new skills. An existing project that already has
`workflows: [propose, explore, apply]` must add `- sync` and `- archive`,
then rerun `compagnon update`.
