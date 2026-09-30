# Proposal: allow local deviation from an inherited decision

## Why

Today, a project that inherits from a source (`path:` or `git:`)
consumes its ADRs **as is**. If the consumer team understands an
inherited decision and deliberately chooses to do otherwise — because a
local context justifies it — it has **no clean way** to record it:

- `codev decision supersede path:~/shared/0100 "…"` is already refused
  (code `cannot_supersede_inherited`) — an inherited decision stays
  read-only on the consumer side, settled since
  [0005](../../decisions/0005-sources-heritees-en-lecture-seule.md).
- Writing a local ADR with no reference to the inherited one leaves the
  agent facing both decisions at once in the `design` instructions: it
  does not know which one prevails.

Result today: the only clean path is a note in the `design.md` of the
current change, invisible to every subsequent change. K6 fills that gap
by providing an **explicit gesture**: "we understand the inherited
decision, we choose a local alternative, the trace is there".

## What Changes

- **New frontmatter field `deviates_from`** — a list of qualified
  identifiers (`path:~/shared/0100`, `git:git@github.com:acme/shared.git/0100`).
  Additive: an ADR without this field keeps its current semantics.
- **New command `codev decision deviate <qualified-id> <title>`** —
  creates a local `accepted` ADR with `deviates_from: ["<qualified>"]`,
  sealed like any other local ADR (K3).
- **The index takes deviations into account** — an inherited decision
  referenced by a local `deviates_from` is marked `deviated_by:
  <qualified-local>` in the index. It stays visible in `codev
  decision list` (transparency), but **disappears** from the
  instructions injected into `design` — the agent sees the deviation,
  not the decision it replaces.
- **New stable finding `decision_dangling_deviation`** (warning) —
  when a `deviates_from` targets a `qualified-id` that no longer exists
  (source moved, SHA changed, folder removed).
- **New stable finding `decision_conflicting_deviations`** (error) —
  when two local ADRs deviate from the same inherited target. The rule
  is: "one target, one deviation".
- **JSON contract** — `DecisionV1` gains a `deviatesFrom: Vec<String>`
  (additive, never filled for an ADR without the field). Inherited
  index entries gain a computed `deviatedBy: Option<String>`.
- **Explicit refusals**:
  - Deviating from a **local** decision is refused (code
    `cannot_deviate_from_local`) — the clean path for that is
    `codev decision supersede`.
  - Deviating from an unknown id is refused (code `unknown_decision_id`,
    an existing code).

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `decisions` — four new ADDED requirements: format and semantics of
  `deviates_from`, behavior of the `decision deviate` command, effect on
  the index and on injection into design, validate findings for
  degenerate cases.

## Impact

- **Code**: extension of the ADR parser (`codev-core::decisions::parser`)
  to read the `deviates_from` field; extension of the index
  (`codev-engine::decisions`) to compute `deviated_by` and remove
  deviated inherited decisions from the `in_effect` computation on the
  consumer side; extension of `plan_new_decision` / creation of a
  `plan_deviate` in `codev-engine::decisions_actions`; extension of
  `validate_decisions` for the two new findings; new CLI subcommand.
- **JSON contract**: two additive fields on `DecisionV1`
  (`deviatesFrom`, `deviatedBy`) and a `DecisionDeviatedV1` close to
  `DecisionCreatedV1`. No field removed or renamed.
- **File written**: nothing new — the local ADR is written in
  `_codev/decisions/NNNN-<slug>.md`, like any other. The seal is added
  to `seal.yaml` by the same plan (consistent with K3).
- **Injection into design instructions**: a deviated inherited decision
  disappears from the `decisions[]` array **and** from the human
  "Decisions in effect" section. A contract consumer only sees the
  deviation.
- **Migration**: none. The 6 existing ADRs have no `deviates_from`,
  this field is optional, the index computes an empty `deviatedBy`.
- **Out of scope**:
  - **Deviating from a local decision** — refused by this change. A
    `codev decision revise <id>` (controlled edit with re-seal) is a
    different gesture, to be discussed separately if needed.
  - **Deviating from an inherited decision already deviated by the
    source itself** — treated as a normal deviation on the new
    decision. No cascading resolution logic in this batch.
  - **Interactive interface to choose what to deviate from** — the
    command takes a `qualified-id` as argument, the user knows it via
    `codev decision list`. No wizard.
