# Design: Every command that creates a decision creates it proposed

## Context

The previous change (`decision-accept-command`) made `decision new` create
`proposed` decisions and added `plan_accept`, which rewrites the status and
seals the body in one plan. `plan_supersede`, `plan_deviate` and
`plan_promote` in `codev-engine::decisions_actions` still render an
`accepted` frontmatter and add a seal entry; `plan_supersede` also rewrites
the predecessor to `superseded` at creation time, with
`rewrite_frontmatter_status`, which keeps the body byte for byte so the
predecessor's seal stays valid.

The index already ignores relations carried by a non-`accepted` ADR:
`resolve_in_effect` only counts the `supersedes` of `accepted` entries, and
`resolve_deviations` only counts the `deviates_from` of local `accepted`
entries — for `deviated_by`, `decision_dangling_deviation` and
`decision_conflicting_deviations` alike. `build_summaries` in the CLI
computes `supersededBy` from `accepted` entries only. So a `proposed`
supersession or deviation already has no effect on the index; what is
missing is the transition that gives it one.

## Goals / Non-Goals

**Goals:**

- `decision accept` is the only command that makes a decision take effect,
  whatever command created it.
- The predecessor of a supersession changes status in the same plan as the
  acceptance of its successor.

**Non-Goals:**

- A one-step flag on `supersede`, `deviate` or `promote` (see below).
- Refusing, at accept time, a deviation whose inherited target is already
  deviated from by another accepted ADR: `validate` reports it as
  `decision_conflicting_deviations`, as before.
- Migrating ADRs created by earlier versions: they are already `accepted`
  and sealed, and stay valid.

## Decisions

### Decision: Creation plans write no seal and never touch the predecessor

Following `_codev/decisions/0001-functional-core-imperative-shell.md`,
each creation stays a pure plan. `plan_supersede`, `plan_deviate` and
`plan_promote` render a `proposed` frontmatter and drop the `seal.yaml`
write; `plan_supersede` also drops the rewrite of the predecessor, so its
plan is a single `CreateOnly` write. With no seal involved, these plans no
longer need the current seal file, and the `body_sha256` fields of their
results disappear: nothing is sealed, so nothing is hashed. `plan_promote`
keeps writing the reference into `design.md` in the same plan as the ADR.

`plan_supersede` gains one refusal: a local target whose status is not
`accepted` is refused with `predecessor_not_accepted`. Without it, the
command would create an ADR that `decision accept` would refuse later —
failing early is cheaper for the user. The existing refusals
(`unknown_decision_id`, `cannot_supersede_inherited` pointing to
`deviate`) are unchanged.

**Alternatives considered**: keeping the predecessor rewrite in
`supersede` and only dropping the seal (the old decision would leave the
set of decisions in effect before its replacement is written — exactly the
gap the proposal closes); creating the ADR with a status other than
`proposed`, such as a new `draft` (a sixth status for the same meaning).

### Decision: `plan_accept` supersedes the predecessors in the same plan

`plan_accept` reads the `supersedes` of the decision being accepted. For
each listed id it resolves the local entry with that id, checks it, and
adds an `Overwrite` of its file produced by `rewrite_frontmatter_status`
with `superseded` — body byte for byte identical, seal entry untouched.
These writes join the ADR rewrite and the seal write in one plan, so the
shell writes all of them or none. Every check happens while computing the
plan:

- no entry with that id → `unknown_decision_id`, naming the predecessor;
- the id only exists in an inherited source →
  `cannot_supersede_inherited` — inherited sources are read-only
  (`_codev/decisions/0005-read-only-inherited-sources.md`), a hand-written
  `supersedes` cannot reach into them;
- a local entry whose status is not `accepted` →
  `predecessor_not_accepted`, whose message names the predecessor, its
  status and, when there is one, the accepted decision that superseded it
  in the meantime.

The source reader passed to `plan_accept` becomes `FnMut`, since it now
reads the ADR and each predecessor. The result gains the list of
superseded predecessors (`id`, qualified id, path), which the CLI exposes
as `superseded` in `DecisionAcceptedV1` — an additive field, always
present, empty when nothing is superseded.

**Alternatives considered**: a separate `decision supersede --apply`
step (two commands to make one decision take effect); marking the
predecessors in a second plan after the acceptance (a failure between the
two leaves two decisions in effect for the same subject); silently
skipping a predecessor that is already superseded (the new decision would
claim to replace a decision that is no longer in effect, and the chain
would fork without anyone noticing).

### Decision: `predecessor_not_accepted` is one code for both commands

The same code is used by `supersede` (target not `accepted`) and by
`accept` (predecessor no longer `accepted`): in both cases the decision
the user wants to replace is not in effect. An agent handles it the same
way — look at the chain with `codev decision list` and supersede the
decision that is in effect. `decision_not_proposed` is not reused: it
describes the decision being accepted, not its predecessor.

**Alternatives considered**: `predecessor_already_superseded` (too narrow:
a `proposed` or `rejected` predecessor is refused for the same reason);
two distinct codes per command (the distinction carries no different
remedy).

### Decision: No one-step `--accept` flag

A flag creating and accepting in one plan would be cheap for `supersede`
and `deviate`, but it would bring back the problem this change removes:
sealing a placeholder body that is meant to be written. For `promote` it
is worse — the promoted body is explicitly meant to be reworked. Two
commands (`supersede`, then `accept`) keep one code path through which a
decision takes effect, and a script that wants one step chains them
through the JSON output (`newDecision.id`, `decision.id`). The existing
`decision new --status accepted` stays as it is, for backward
compatibility and for decisions whose text is final at creation; it is not
extended to the other commands.

**Alternatives considered**: `--accept` on all four commands (a second way
to seal, and on `new` a duplicate of `--status accepted`); `--status` on
`supersede`, `deviate` and `promote` (the same duplication, with statuses
that make no sense there, such as `rejected`).

### Decision: The JSON contract drops `bodySha256` where nothing is sealed

`DecisionDeviatedV1` and `DecisionPromotedV1` declared `bodySha256` as
optional, present only when an ADR is sealed. Since those commands no
longer seal, the field is no longer emitted. Consumers of contract v1
already had to tolerate its absence, as with `decision new` for a
`proposed` decision. `DecisionSupersededV1` never carried it; its
`oldId`, `oldQualifiedId` and `oldPath` fields stay and now name the
decision that will be superseded on acceptance.

**Alternatives considered**: emitting the hash of the unsealed body
(a consumer would take it for a seal).

## Risks / Trade-offs

- [A script relying on `supersede`, `deviate` or `promote` taking effect
  immediately] → Marked **BREAKING** in the CHANGELOG, with the
  `decision accept` step to add; the human output of each command names it.
- [A forgotten acceptance leaves the old decision in effect] → That is the
  meaning of `proposed`; `codev decision list` shows the proposed ADR next
  to the decision it will replace.
- [Two proposed supersessions of the same decision] → The first accepted
  wins; the second is refused with `predecessor_not_accepted`, whose
  message points to the decision now in effect.
- [An ADR created by an earlier version, `accepted` with its predecessor
  already `superseded`] → Untouched: `accept` refuses it with
  `decision_not_proposed`, as before.
