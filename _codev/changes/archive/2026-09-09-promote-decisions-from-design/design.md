# Design: promoting a decision from a `design.md`

## Context

See `proposal.md`. K7 is the last gap in the decision lifecycle: K3
locks immutability, K6 allows deviation from inherited decisions, K7
gives birth to an ADR from reasoning carried out within a change.

## Goals / Non-Goals

This design frames: the selection of the block, the generation of the
ADR body, the rewriting of the design, the CLI command, the JSON
contract, the refusals. It does not frame: multiple promotion in one
command, automatic structuring into Context/Decision/Consequences
(impossible without ambiguity), nor an `unpromote`.

## Decisions

### Decision: lightweight internal parser, no markdown dependency

The format of the blocks to extract is highly constrained:

```
## Decisions
### Decision: <title>
<body>
### Decision: <other title>
<body>
```

A line-by-line scan is enough: find `## Decisions`, then loop over
`### Decision: ...`, and collect the bytes between two `###` (or up to
the next `## `). Aligned with decision
[0001](../../decisions/0001-functional-core-imperative-shell.md) —
the parser is pure and lives in a new module `codev-engine::design`
(the pure function takes the design source as an argument, the shell
does the disk read).

**Rejected alternative**: reuse an existing markdown parser
(pulldown-cmark, comrak). Dependency cost and rich parsing for an
ultra-constrained need. codev has a tradition of hand-written parsers in
`codev-core::parser` — we follow it.

### Decision: the body is reproduced verbatim

The body of a `### Decision: <title>` block goes **byte for byte** into
the `## Decision` section of the new ADR — no trim, no reformatting, no
dedenting. This is what lets the author find their text identically
and then split it into Context / Decision / Consequences / Rejected
Alternatives without having to guess what might have been modified.

**Aligned** with the K3 decision: an ADR's body is treated byte for
byte for the seal hash. Consistent from end to end.

### Decision: the reference in the design is text, not a link

`> Promoted to ADR **NNNN** — see `` `_codev/decisions/NNNN-<slug>.md` ``.

A link `[..](../../decisions/NNNN-slug.md)` would work while the change
is active (`_codev/changes/<name>/design.md` → `../../decisions/`
resolves to `_codev/decisions/`), but would break after archive
(`_codev/changes/archive/<date>-<name>/design.md` → `../../decisions/`
resolves to `_codev/changes/decisions/`, which does not exist). The
path as text stays **understandable** from any depth; the human finds
it.

**Rejected alternative**: absolute link `/_codev/decisions/...`. Works
on a website served from the repo root, but not in a plain
`less design.md`. Too many assumptions about the reading environment.

### Decision: the block heading stays, only the body is replaced

Two options:

| Option | Pro | Con |
|---|---|---|
| **A. Remove the whole block** | Shorter design after several promotions | The history of "we discussed X" disappears from the design; the reader no longer knows why the ADR exists |
| **B. Keep `### Decision: <title>` + quote** | Traceability maintained, the subject stays readable in the design | Slightly longer file |

**Chosen: B.** The design remains a trace of the discussion; a reader
going through `design.md` afterwards still sees which trade-offs were
made — and sees that they were *raised* to the rank of ADR. That is
precisely the value promotion adds.

### Decision: `plan_promote` reuses `plan_new_decision` from K3

No new pure ADR-creation function — we reuse
`decisions_actions::plan_new_decision`, passing it the extracted title
and a "prefilled" body (via a variant of the rendering that takes the
body as an argument). The seal is added to the plan, as for any new
`accepted` decision. The plan additionally gains **one write** for the
block substitution in `design.md` — hence an atomic plan with 3
writes: ADR + seal.yaml + design.md.

### Decision: refusal of archived changes is explicit via `codev list`

As in K3 (`decision seal`) and K6 (`decision deviate`), the command
refuses if the change name does not appear in `codev list` (which only
shows active ones). Dedicated stable code `cannot_promote_from_archived`
so that the agent can distinguish it from an `unknown_change` (for
example to propose that the user open the archive manually if they
insist).

## Risks / Trade-offs

- **The verbatim body may include inconsistent markdown** — a `###`
  nested in the body (unlikely but possible) would mislead the scan.
  → **Accepted trade-off**: the spec says "up to the next `###`", that
  is the contract; an author who writes a `###` in a decision block
  will get a truncated ADR. Documented. A validate warning could come
  later.
- **A design edited between computing the plan and applying it** — an
  author who edits `design.md` while the command runs would see their
  modification overwritten. → **Accepted trade-off**: atomicity at
  process scale only, as for all commands.
- **Two ambiguous titles** — the author may have duplicated a title
  while iterating on the wording. → **Handled** by the refusal
  `ambiguous_decision_heading`, which names both positions in the file
  (line numbers).

## Migration Plan

None. Existing designs do not change; the command is opt-in.
