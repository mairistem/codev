# Proposal: Newcomer readability review in /codev-apply

## Why

No role checks that the code a change produces can be understood and
maintained by a human. The contrarian of `/codev-propose` challenges the
plan, not the code; `/codev-apply` checks that tests pass, not that a
newcomer can follow the result — correct, tested, approved code can still
be obscure.

## What Changes

- `/codev-apply` gains a "Review as a newcomer" step, after every task is
  implemented and before the full test suite runs: it re-reads only the code
  the change touched, as a developer who has seen neither the plan nor the
  conversation, along a fixed grid (understandable without context, names,
  size and nesting, cleverness, comments, tests as a description of the
  behavior) plus the project's `rules`.
- The step simplifies what is unambiguous, without changing behavior,
  widening the scope or touching code outside the change, and re-runs the
  affected tests. What it does not simplify alone goes into a "Readability
  notes" block of the final summary — at most three items — or "No
  readability concern found".
- The architect's "Done when" criteria of the `spec-driven` `design`
  instruction gain: the approach is the simplest that satisfies the specs,
  and a newcomer can follow the design without knowing its history.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `skills` — the `apply` readability review, and the architect criteria of
  the default schema.

### Removed Capabilities

None.

## Impact

- `assets/workflows/apply.md`, `assets/schemas/spec-driven/schema.yaml`
  (`design` instruction); the regenerated skills under `.claude/skills/`.
- Tests asserting on skill and instruction text in `codev-agents` and
  `codev-engine`.
- Documentation, in `docs/en` and `docs/fr`: `roles.md` (newcomer reviewer
  role, architect criterion), `skills.md` (apply), `workflow.md` (apply
  step).
- No change to the CLI, the JSON contract or the file formats. Plans written
  by custom schemas are unaffected.

## Release note

- Added: `/codev-apply` reviews the code a change touched as a newcomer
  would, simplifies what is unambiguous without changing behavior, and
  reports up to three readability notes in its summary.
- Changed: The architect's criteria of the default schema require the
  simplest approach that satisfies the specs, and a design a newcomer can
  follow without knowing its history.
