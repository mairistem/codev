# Proposal: Every command that creates a decision creates it proposed

## Why

`codev decision supersede`, `codev decision deviate` and
`codev decision promote` still create `accepted` decisions and seal their
body at once. The placeholder or promoted text is then written, `codev
validate` fails with `decision_seal_mismatch`, and users are pushed to
`codev decision seal --force` — which defeats the seal. `promote` is the
worst case: its own output invites the user to rework the promoted text.
`decision new` already creates `proposed` decisions sealed by `codev
decision accept`; the other three should follow the same lifecycle.

## What Changes

- `codev decision supersede <old> <title>` creates a `proposed` ADR with
  `supersedes: ["<old>"]` and leaves the old decision untouched: it stays
  `accepted` and in effect until the new one is accepted.
- `codev decision deviate <qualified-id> <title>` creates a `proposed` ADR
  with `deviates_from`; the inherited decision stays in effect until the
  deviation is accepted.
- `codev decision promote <change> <title>` creates a `proposed` ADR whose
  `## Decision` section is the design block verbatim, and still replaces the
  block with a reference in `design.md`. Its output says to review the ADR,
  then run `codev decision accept NNNN`.
- None of the three writes to `seal.yaml` any more.
- `codev decision accept <id>` becomes the single place where a decision
  takes effect: for an ADR with `supersedes`, the same plan rewrites each
  predecessor to `status: superseded` (body byte for byte identical, seal
  untouched), sets the new ADR `accepted` and seals it.
- New stable code `predecessor_not_accepted`: `decision accept` refuses an
  ADR whose predecessor is no longer `accepted` (for example superseded by
  another decision in the meantime), and `decision supersede` refuses a
  target that is not `accepted`, instead of creating an ADR that could never
  be accepted.
- JSON contract v1, additive only: `decision deviate --json` and
  `decision promote --json` no longer carry `bodySha256`, since nothing is
  sealed; `decision accept --json` gains a `superseded` array listing the
  predecessors it marked superseded.
- No one-step flag: `decision new --status accepted` stays as it is, and no
  `--accept` is added (see design).
- **BREAKING** for scripts that relied on `supersede`, `deviate` or `promote`
  taking effect immediately: they must now run `codev decision accept` on the
  created ADR.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `decisions` — supersession, deviation and promotion create `proposed`
  decisions; `decision accept` marks the predecessors superseded; the seal
  requirement of `supersede`.

### Removed Capabilities

None.

## Impact

- `codev-engine` (`decisions_actions`: `plan_supersede`, `plan_deviate`,
  `plan_promote` without seal, `plan_accept` with predecessors, one new
  refusal code) and `codev-cli` (commands, clap help, human rendering, JSON
  contract).
- JSON contract v1: `DecisionAcceptedV1` gains `superseded`;
  `DecisionDeviatedV1` and `DecisionPromotedV1` lose `bodySha256`, which was
  only present when an ADR was sealed.
- Documentation in English and French: CLI reference, JSON output, decision
  lifecycle in the concepts chapter, FAQ, inherited sources guide; CHANGELOG
  and ROADMAP.
- Skills: none call `supersede`, `deviate` or `promote`; checked, unchanged.
