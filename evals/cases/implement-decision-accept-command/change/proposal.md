# Proposal: `decision new` creates a proposed decision, `decision accept` seals it

## Why

`codev decision new <title>` creates an `accepted` decision and seals the
placeholder template body at once. The first real edit of the decision then
makes `codev validate` fail with `decision_seal_mismatch`, and the only way
out is `codev decision seal --force` — which teaches users to force the seal
routinely and defeats the immutability it is meant to guarantee. A decision
should be sealed when its text is final, not when its file is created.

## What Changes

- `codev decision new` defaults to `--status proposed`: the decision is
  created unsealed and can be written freely. `--status accepted` stays
  available and still writes the decision and its seal in the same plan.
- New command `codev decision accept <id> [--json]`: turns a local
  `proposed` decision into an `accepted` one and seals its body, in a single
  plan of effects — the status and the seal are written together or not at
  all. The body is left byte for byte identical.
- `decision accept` refuses an inherited decision (`cannot_accept_inherited`)
  and a decision whose status is not `proposed` (`decision_not_proposed`,
  which points to `supersede` for an already accepted one). An unknown
  identifier keeps the existing `unknown_decision_id` code.
- JSON contract v1, additive only: `decision accept --json` returns
  `decision`, `path` and `bodySha256`, like `decision new`.
- No **BREAKING** change: existing ADRs and seals are untouched, and scripts
  that relied on the default can pass `--status accepted`.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `decisions` — default status of `decision new`, the new `decision accept`
  command, and the seal requirements that assumed `new` seals by default.

### Removed Capabilities

None.

## Impact

- `codev-engine` (`decisions_actions`: a new pure plan and two refusal
  codes) and `codev-cli` (command, clap help, human rendering, JSON
  contract).
- JSON contract v1: new `DecisionAcceptedV1` document; `decision new` now
  omits `bodySha256` by default, as it already did for `proposed`.
- Documentation in English and French: CLI reference, JSON output,
  decision lifecycle in the concepts chapter, FAQ; CHANGELOG.
- Skills: none create decisions with `decision new`; `decision promote`
  keeps sealing the promoted ADR.
