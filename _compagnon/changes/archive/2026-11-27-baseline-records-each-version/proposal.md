# Proposal: The baseline records the Compagnon version of each step

## Why

The reliability chapter says the whole baseline ran on "compagnon 0.6.1",
from `evals/baseline/run.json`. The nine `idea.explore` and
`project.onboard` cases were added to it later (change
`explore-onboard-evaluation`) by a resumed run built from the branch,
Compagnon 0.7.0: their figures are right, the version the page gives
them is not. A resumed run checks the model and the Claude Code version
against `run.json`, never the Compagnon version, so nothing keeps the
difference.

## What Changes

- `run.json` gains an optional list `added`: for each resumed run whose
  Compagnon version differs from the one `run.json` records, the steps
  it ran, its Compagnon version and its date. The runner appends to it;
  the original fields stay as they were recorded.
- The reliability chapter and the report (`report.md`) give, beside the
  run's Compagnon version, the version and date of each step added later.
- The committed baseline's `run.json` gets the entry for `idea.explore`
  and `project.onboard` (Compagnon 0.7.0, 2026-11-26), from the change
  that recorded them; the baseline is scored again (its report embeds
  `run.json`) and the chapter regenerated. No paid run.

## Capabilities

### New Capabilities

### Modified Capabilities

- role-evaluation

### Removed Capabilities

## Impact

- `evals/src/observed.rs` (`RunInfo`), `evals/src/main.rs` and
  `evals/src/runner.rs` (resume), `evals/src/reliability.rs` and
  `evals/src/report.rs` (the run block), the test fixtures that build a
  `RunInfo` (`evals/src/score.rs`, `runner.rs`, `reliability.rs`).
- `evals/baseline/run.json`, `report.json` and `report.md` (scored
  again), `docs/en/src/reliability.md`, `docs/fr/src/reliability.md`
  (generated).

## Domains

- [ ] security — the matches are the manifests of the `explore-*` and `onboard-*` evaluation fixtures, from another change on the branch
- [ ] personal-data — the match is the word "phone" in the home page's example (`docs/site/index.html`), from another change on the branch
- [ ] user-interface — the matches are the documentation and its stylesheet, from other changes on the branch; here only the generated reliability page gains a version line
- [ ] performance — the matches are SQL of evaluation fixtures and « cache » in `evals/src/reliability.rs`, from other changes on the branch
- [ ] operations — the matches are words of an evaluation fixture, from another change on the branch
- [ ] migration — `run.json` gains an optional field; a file without it reads as before, and older readers are the suite's own code, changed here

## Release note

None
