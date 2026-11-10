# Proposal: No implementation over an unsettled plan review

## Why

The plan review (`plan.challenge`, during `/codev-propose`) records its
findings in `review.md`, but `/codev-apply` only checks that the planning
artifacts exist before writing code. A `high` finding still `open` or
`disputed` on the plan is only caught by `codev archive`, once the code is
written — when fixing the plan costs the most. Now that the review record
and its archive gate are in place, the same rule can stop the
implementation before it starts.

## What Changes

- `codev status --json` gains an additive `planReview` object, present
  when the change has a `review.md`: `settled` (boolean) and `blockers`
  (each with a stable `code` and a `message`). It
  is unsettled when a `high` finding of the record's `plan.challenge`
  section is `open` or `disputed` (`review_unresolved`), when such a
  finding's `accepted-risk` or `deferred` breaks the record's rules — not
  by a human, no justification, no destination
  (`review_decision_incomplete`) —, or when the record
  is in a format newer than this codev reads (`review_format_unsupported`)
  — the same judgement the archive gate applies, restricted to the plan
  step.
- A missing record, or a record without a `plan.challenge` section, does
  not unsettle the plan review: a plan written by hand or before the
  review existed can still be implemented; `codev archive` keeps refusing
  it while roles were active.
- The human output of `codev status` adds one line when the plan review is
  unsettled, naming the findings and pointing to `codev review`.
- **BREAKING** `/codev-apply` refuses to implement, before its first task, while
  `planReview.settled` is `false`: it names each blocker, gives the stable
  refusal code `plan_review_unsettled`, points to `codev review`, says how
  to settle it (fix the plan, or a human accepts the risk or defers the
  finding in `review.md`), and changes no file. A document without
  `planReview` (no record, or an older codev) is not a refusal.
- No setting: the gate is always on. The conscious override is the one the
  review record already has — a human writes `accepted-risk` (with a
  justification) or `deferred` on the finding.
- The implementation evaluation gains a refusal case: a complete plan
  whose record holds a `high` finding `open` at `plan.challenge`. A
  refusing case may now lay a `tasks.md` with its `review.md`, and names
  what the refusal must point to (`points_to`, `/codev-propose` by
  default).

## Capabilities

### New Capabilities

### Modified Capabilities

- changes
- review-record
- skills
- implementation-evaluation

### Removed Capabilities

## Impact

- `codev-core::review` (the gate, restricted to one step), `codev-engine`
  (status reads the record), `codev-cli` (`StatusV1` in
  `contract.rs`, human rendering of `codev status`).
- `assets/workflows/apply.md` and the rendered
  `.claude/skills/codev-apply/SKILL.md`.
- `evals/cases/` (a new `code.implement` refusal case) and its baseline
  runs in `evals/baseline/`.
- Documentation, in `docs/en` and `docs/fr`: the review record guide, the
  skills chapter, the JSON output reference, the CLI reference
  (`codev status`), and the reliability chapter, regenerated from the
  baseline.
- Changes in progress whose record holds an unsettled `high` plan finding
  can no longer be implemented with `/codev-apply` until it is settled.

## Domains

- [ ] security
- [ ] personal-data
- [ ] user-interface
- [ ] performance
- [ ] operations
- [x] migration

## Release note

- Changed: **BREAKING** `/codev-apply` refuses to start while the plan review holds a `high` finding `open` or `disputed`; fix the plan, or have a human accept the risk or defer the finding in `review.md`, then run it again.
- Added: `codev status --json` reports `planReview` — whether the plan review is settled, and what keeps it from being so.
