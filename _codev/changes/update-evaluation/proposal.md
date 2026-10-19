# Proposal: Evaluate the `/codev-update` skill

## Why

Decision 0008 asks that every change to what codev's agents read be
measured on the steps it touches, and that a step the suite does not
cover be covered first. The suite covers `/codev-propose` (planning and
the plan's reviews) and `/codev-apply` at `code.review`, but no case runs
`/codev-update`. Since then, `capture-corrections` (#48) waits on this:
it adds text to `/codev-update` that nothing can measure.

## What Changes

- A new kind of case, `plan.update`: a past state of the repository, an
  active change laid on it, an artifact to revise and the author's
  revision request, with what must result — the files the revision
  changes, the files it must leave alone, the files whose ripple the
  agent must report rather than edit, the text the revised artifact must
  hold, or a refusal.
- `codev-evals run --step plan.update` replays the request with the
  `/codev-update` skill of the codev under test, headlessly, and records
  what changed.
- Each run is scored by deterministic checks, without a model: the
  revised and the kept files, the ripple named in the agent's reply, the
  expected text, the change's validity, and nothing written outside the
  change's folder. The report and `codev-evals compare` give the
  update measures next to the planning and review ones.
- Six cases drawn from archived changes, and their runs recorded in the
  baseline.

## Capabilities

### New Capabilities

- update-evaluation

### Modified Capabilities

### Removed Capabilities

## Impact

- `evals/`: the case format, the runner's fixture and prompt for
  `plan.update`, the scoring, the report, the comparison, six cases under
  `evals/cases/`, and `evals/baseline/`.
- No change to codev itself: `evals/` is not shipped.
- Documentation: the generated reliability page (`evals/src/reliability.rs`,
  `docs/{en,fr}/src/reliability.md`) and `CONTRIBUTING.md` /
  `CONTRIBUTING.fr.md`.

## Domains

- [ ] security — the cases run in the sandbox the suite already uses; no new input reaches a command
- [ ] personal-data — the cases hold only this repository's own artifacts
- [ ] user-interface — no interface beyond the report's tables
- [ ] performance — the runs cost what the suite already pays per case, and only when the step is selected
- [ ] operations — no log, metric, timeout or deployment changes beyond the runner's existing ones
- [ ] migration — `evals/` is not shipped, and the report gains a section without changing the others

## Release note

None
