# Design: local deviations from an inherited decision

## Context

See `proposal.md`. K3 has just established the immutability of local
ADRs; this change (K6) completes the framework on the inherited sources
side: the consumer cannot modify what it inherits, but must be able to
**record its divergence** in a way the tool can read.

## Goals / Non-Goals

This design frames the new field, the CLI command, the effect on the
index, the injection into `design`, and the two new validate findings.
It does not frame promotion from `design.md` (K7), transitive
deviations (deviating from a deviation), or an "undo my deviation" mode
(removing the local ADR is enough — no dedicated gesture).

## Decisions

### Decision: new `deviates_from` field, distinct from `supersedes`

Two options:

| Option | Pro | Con |
|---|---|---|
| **A. Reuse `supersedes`** with qualified identifiers | A single mechanism to know | Semantics already loaded — "supersede" means "replaces". In the source project, the decision has not been replaced at all: saying so is false |
| **B. New `deviates_from` field** | Semantics specific to this case — "we understand it, we choose otherwise" | One more frontmatter field |

**Chosen: B.** An ADR is a historical document; writing "supersedes
path:~/shared/0100" in it would make the reader believe that the
source itself withdrew that decision. Deviation is a **local** gesture;
the field must reflect its scope. Aligned with decision
[0005](../../decisions/0005-sources-heritees-en-lecture-seule.md):
inherited sources are **read-only**, even the semantics of their status
is not driven from the consumer.

### Decision: deviation only applies to inherited targets

`codev decision deviate project/0003 …` is refused (code
`cannot_deviate_from_local`), with a pointer to `codev decision
supersede`. Two distinct gestures, each with its own semantics:

- `supersede` — "we replace our own decision with a new one"; the file
  of the old one switches to `status: superseded`.
- `deviate` — "we understand the inherited decision, we set it aside
  locally"; **nothing** is written on the source side, the local ADR
  carries the trace.

Conflating the two would lose the nuance that distinguishes them.

### Decision: hidden in `design` instructions, but visible in `decision list`

The index computes a `deviated_by: <qualified-local>` attribute on each
inherited entry referenced by a local `accepted` ADR. Two effects:

- **`design` instructions** — the deviated entry **no longer** appears
  in the `decisions[]` array nor in the human "Decisions in effect"
  section. The agent drafting a `design.md` sees the deviation, not the
  replaced decision — otherwise it would propose to respect it,
  wrongly.
- **`codev decision list`** — the deviated entry stays listed, with its
  `deviatedBy` visible. Transparency prevails: the user must be able to
  see everything that exists in the index, including inherited
  decisions set aside.

**Rejected alternative**: hide the deviated entry everywhere. Makes
tracking impossible — the user would no longer know a decision exists
on the source side unless someone told them.

### Decision: one target, one deviation — enforced by validate

Two local ADRs deviating from the same target → an unresolvable
conflict: which of the two prevails? We refuse to decide silently and
emit `decision_conflicting_deviations` as an **error**. The user
chooses — either they remove one of the two ADRs, or they supersede one
with the other.

**Rejected alternative**: keep the most recent one. A frontmatter
`date:` is a free field, a user could lie in it. We refuse to resolve
by heuristic.

### Decision: a target that disappears → warning, not error

A moved source (`git:` removed from the config, a changed SHA hiding
the file, a renamed `path:`) breaks the target of a `deviates_from`.
Two reasonable scenarios:

1. The project has evolved and so has the source; the deviation no
   longer makes sense → remove it.
2. The source is temporarily inaccessible; the deviation is still
   relevant → wait.

`decision_dangling_deviation` as a **warning** covers both without
blocking the flows (`sync`, `archive`). The user sees and decides.

**Aligned** with the K3 decision: `decision_unsealed` is also a warning
(migration does not block); `decision_seal_mismatch` is an error (the
index is no longer reliable). Here, an orphan deviation does not
compromise the index — the target is simply absent.

### Decision: the local deviation ADR is a normal ADR, sealed by K3

Nothing special on the seal side: `plan_deviate` produces a plan that
writes the ADR **and** the seal entry, exactly like `plan_new`. The
rule "accepted → sealed" still holds.

**Corollary**: `codev decision deviate <target> <title>` refuses if the
target is ambiguous (two sources expose the same `<qualified-id>`); the
stable code is `ambiguous_decision_id` (an existing code).

## Risks / Trade-offs

- **A source that, in turn, deviates from the same target** as the
  consumer project — the index on the source side does not know the
  consumer's notion of deviation, so no conflict is detected at the
  source.
  → **Accepted trade-off**: this batch does no cascading resolution
  (out of the proposal's scope). The user will see both deviations side
  by side in `codev decision list`, and will decide.
- **A user might deviate "to test"** and then forget to remove the
  local ADR, leaving a `deviatesFrom` entry that no longer makes sense.
  → **Mitigation**: the propose/apply/archive workflow leaves a clean
  trace — a deviation ADR is a complete ADR, with context and decision,
  not a throwaway tag.
- **`deviatedBy` computed, not persisted** — the index recomputes it on
  every call. Negligible cost (one more pass over entries already
  loaded), and it avoids the problem of "how to keep a derived
  attribute up to date when the file changes".

## Migration Plan

None. The `deviates_from` field is optional; existing ADRs do not have
it; the index computes an empty `deviatedBy` for all entries.
