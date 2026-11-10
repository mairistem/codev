# Design: Opposed review findings reach the human together

## Context

The review procedure lives in two shared fragments, rendered into
`/codev-propose` and `/codev-apply`:
`assets/workflows/fragments/review-record.md` (the record, the finding
format, the answer loop, "To decide") and `review-isolated.md` (the
briefs, the sub-agents and the meeting). Today the meeting says
"Findings at the same place that need different fixes stay separate", and
nothing more. `crates/codev-agents/src/workflows.rs` asserts the
fragments' key sentences in the rendered skills.

In the binary, `codev-core::review` parses the record
(`parse.rs`, `FINDING_FIELDS`), checks each history alone
(`history.rs`), computes who acts next per finding
(`ReviewFinding::awaiting`, `mod.rs`) and the archive gate (`gate.rs`).
`codev-cli` renders `codev review` (`render.rs`, "Waiting for an
answer") and the `ReviewV1` JSON (`contract.rs`, `review_finding`). An
unknown finding field is the warning `review_unknown_field`. Duplicate
ids are already caught across the whole record (`Parser::ids`).

The constraints come from decisions in force. The judgement — whether two
fixes are opposed — stays in the skill, and the binary only checks what
is deterministic (0001, 0007). The skill change is measured on the steps
it touches (0008), and judged on quality, not on its length (0010).

## Goals / Non-Goals

**Goals**

- A conflict between two findings is visible in the record, survives
  the answer loop, and reaches a human whole.
- The binary refuses to call a conflict settled without a human, so a
  skill that breaks the rule is seen in `codev review` and, for a `high`
  finding, at the archive.

**Non-Goals**

- Detecting opposed fixes in the binary: that is judgement.
- A new history status or transition: an older codev would reject the
  record (`review_invalid`), which the compatibility constraint forbids.
  The human's choice that the history cannot hold goes in a new optional
  field instead, which an older codev only warns about.
- Measuring the meeting's linking quality with a new multi-role
  evaluation case (see the evaluation decision).

## Decisions

### Decision: A conflict is a mutual `conflicts_with` link, checked over the whole record

`conflicts_with` is an optional list of finding ids. It is parsed with
the other fields, like `raised_by`: it must be a non-empty list of ids,
without duplicates, and must not name the finding itself. These errors
are reported at the block. Existence and mutuality need every id. They
are checked once the whole record is parsed, in a new
`codev-core::review::conflict` module: a pure function over `Record`
that returns located `review_invalid` findings. The error goes on the
finding that lists an id which is missing, or which does not list it
back. Links may cross steps and roles. Ids are unique across the record,
and a `code.review` finding may oppose a fix chosen at `plan.challenge`.

Each link is settled on its own, never through a chain. Suppose A–B was
settled by the human at `plan.challenge` (B accepted as a risk, A fixed),
and a `code.review` finding C later opposes A's fix. The link A–C must
reach the human too. A connected-component rule would have counted B's
old human entry and let A–C pass silently.

**Alternatives considered**:

- A one-way link, read both ways by codev: half the writing, but a
  reader of one finding would not see the link, and a typo would go
  unnoticed. The request asks for mutual links.
- A dedicated section listing conflicts (`## Conflicts`): a new
  structure an older codev would report as an unknown step, and a second
  place to keep in sync with the findings.

### Decision: A link is settled by a human's answer on either of its findings: a history entry, or a `conflict_decision`

A link is settled once either finding holds a human's answer. That is an
entry by `human` or `human:<name>` in its history, or a
`conflict_decision: { by: human:<name>, note: … }`. The intended path
stays within today's transition table. The author leaves both findings
`open`, the human picks one, and the choice is written on the other as
`accepted-risk` with the justification, or `deferred` with `to:`. Both
are allowed from `open`. The loop then resumes on the chosen finding.

`conflict_decision` exists for the states the history cannot answer.
After a broken rule, the finding not chosen may be `rejected` or
`deferred` (no entry may follow), or the chosen finding may be `fixed`
and verified (only its role may write next). There, the human writes
`conflict_decision` on either finding, whatever the statuses. Without it,
"keep the fix already applied" would have no legal record, and a
blocked archive would have no exit. The history stays the audit trail of
the findings. `conflict_decision` records only the choice between them,
and validation requires it to be a human's, with a note, on a linked
finding.

While a link is unsettled and at least one of its two findings has
confidence `high`, `awaiting` is `human` for both, whatever their own
statuses. A fixed and verified finding counts, and so does one its
reviewer `rejected`. This closes the gap: the "fix A, dispute B citing A"
path no longer ends with nothing awaiting a human. `awaiting` therefore
needs the record. `Record::awaiting(&ReviewFinding)` wraps the
per-finding rule, and `render.rs` and `contract.rs` call it instead of
`ReviewFinding::awaiting`. That method stays as the history-only rule.

**Alternatives considered**:

- Settlement through the history alone: a reviewer of this plan showed
  that "keep the applied fix" then has no legal entry once the other
  finding is `rejected`. Its only route was a reopening that claims a
  holding fix "does not hold".
- New transitions letting a human answer a `fixed` or `rejected` finding:
  an older codev would report `review_invalid` on the record, breaking
  the compatibility the request sets.
- Settling per connected component: one old human entry would settle any
  finding linked to the conflict later, across steps.
- Counting only the last entry: a human entry followed by the author's
  fix of the chosen side would reopen the link.

### Decision: The archive blocks an unsettled link only through a `high` finding

The gate gains the reason `review_conflict_unsettled`. It applies to an
unsettled link joining a `high` finding with confidence `high`, whatever
that finding's status. The rule already says a `high` finding "must not
be archived until it is fixed or a human accepts the risk", and a fix
chosen against an opposed reviewer without a human is precisely what it
forbids. Every blocking state has an exit: `conflict_decision`, always
writable.

A link between `medium` findings waits for the human without blocking.
That keeps the severity scale's meaning, which `codev-core` serves to the
skills and the docs. `medium` is "to fix in this change or to defer
deliberately", and a maintained `medium` dispute does not block today
either.

**Alternatives considered**:

- Blocking at `medium` too: stricter, but `medium` would become blocking
  only when linked, contradicting the scale. Any medium/medium
  disagreement would stop an archive that a maintained medium dispute
  does not stop.
- Not blocking at all, relying on "To decide": a skill that settled a
  `high` conflict alone would archive it silently — the original gap.
- Exempting a `rejected` finding from the gate: it avoided a dead end
  only while settlement was history-only, and let a high conflict pass
  without a human.

### Decision: A low-confidence finding is presented when it is linked to a confident one

`reported` keeps its meaning (confidence `high`). A link waits for the
human, and is presented whole, when at least one of its two findings is
confident. The low-confidence side comes with it, marked as such: it is
the cost of the confident fix, and hiding it would recreate the
one-sided view. A link between two low-confidence findings neither waits
for the human nor is presented, as each finding alone would not be. The
author then answers both as usual, and the skill rule restricts the
author only while a link waits for the human, so the two rules agree.

**Alternatives considered**:

- Presenting only confident findings: the human would see "add a cache"
  without the doubt about stale data — the gap, in a smaller form.
- Raising a linked finding's confidence: it would change what the
  reviewer said.

### Decision: The rule is written once, in the shared fragments

The rule goes in `review-isolated.md` (the meeting's **Link** bullet,
next to "stay separate") and `review-record.md`. That second fragment
gets the `conflicts_with` field in the finding format, the inline-mode
rule, the author's restraint in step d, the human's choice in step f (on the finding not chosen, or as a `conflict_decision`) and
the side-by-side presentation in step h. Both fragments are rendered in
both modes (0007), so a change that overrides the mode at run time still
carries the rule. `workflows.rs` asserts the new sentences in both skills,
as for the existing rule texts. A sub-agent never links: it cannot see
the other roles' findings, and the brief must not carry them (0007). The
meeting does.

**Alternatives considered**:

- Linking in the sub-agent's brief: it would require another role's
  findings in the brief, which 0007 forbids.

### Decision: The evaluation measures non-regression on the review steps; the binary carries the new behavior's tests

The role-evaluation runner plays each role alone, so no case can show
the meeting linking two roles. Extending it means multi-role cases, a
new expected-link scoring and a new baseline. That is a capability of its
own, and 0008 has a step the suite does not cover covered first, by its
own change. Here the steps touched, `plan.challenge` and `code.review`,
are covered. This change is measured on them for non-regression, in the
`inline` and `isolated` modes: quality within the baseline's range and
cost not rising (0010: a longer text is judged on quality, its cost only
checked not to rise). The new deterministic behavior is covered by unit
and CLI tests. The skill rule is covered by rendered-text assertions, as
every skill rule is today. `plan.write` stops before the challenge and
does not read the changed procedure's effect, so it is not run.

**Alternatives considered**:

- Extending the runner in this change: it doubles the scope and couples a
  format change with an evaluation design, which 0008 wants separate.
- Testing only the binary, without a measure: 0008 requires one for any
  change to what the agents read.

## Risks / Trade-offs

- [The meeting links findings that are merely different, not opposed] →
  The rule defines opposed: one fix undoes or forbids the other. The
  non-regression measure checks that precision does not drop. A spurious
  link only costs the human a choice.
- [A skill breaks the rule and leaves a `high` conflict where no history
  entry may follow] → `codev review` shows both findings awaiting a
  human, and `conflict_decision` records the human's choice whatever the
  statuses.
- [An earlier human answer on a finding settles a later link to it] →
  It happens only when that finding was itself the human's subject. A
  link to a finding the human chose against (`accepted-risk`) opposes a
  fix that was not applied.
- [An older codev reads a record with links] → It warns
  `review_unknown_field` and computes `awaiting` and the gate as before.
  An unsettled conflict is then not enforced, which is acceptable for a
  team mixing versions. The project raises `codev_version` to enforce it.
- [`awaiting` moves from the finding to the record] → Every caller goes
  through `Record::awaiting`. A unit test pins a record without links to
  today's values.

## Migration Plan

None for existing records: without `conflicts_with`, nothing changes,
and the format marker stays `1`. An older codev warns
`review_unknown_field` on `conflicts_with` and `conflict_decision`. The JSON gains `conflictsWith` (additive
in v1), and `archive.blockers[].code` may now hold
`review_conflict_unsettled` — a new value of an existing field. A skill
installed before this release ignores both. Rollback: an older codev
reads the record with one warning per linked finding.

## Open Questions

- Should a later format (2) fold `conflict_decision` into the history,
  as a human transition from any status, once older codevs are gone?
- A multi-role evaluation case for the meeting — two roles seeded with
  opposed fixes, scored on the link — belongs to its own change of the
  `role-evaluation` capability.
