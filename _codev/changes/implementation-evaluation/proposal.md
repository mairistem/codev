# Proposal: Evaluate the implementation of `/codev-apply`

## Why

codev-evals measures the review roles, the planning of `/codev-propose`
and the revisions of `/codev-update`, but not the step that writes the
code: `/codev-apply`. Without that measure, no one can say whether the
implementation can move from Opus to Sonnet without losing quality — the
next decision ADR 0009 and ADR 0010 leave open — nor whether a change to
the apply skill helps or hurts.

## What Changes

- A new kind of case, at the step `code.implement`: a past change of this
  repository, replayed from the commit before the pull request that
  brought it. The case lays the change's planning artifacts — its tasks
  unchecked, without its review record — on the tree at that commit, and
  holds a hidden acceptance script written from the change's scenarios,
  which drives the built `codev` binary as a user would.
- The runner prepares the fixture as for an update case, also fetches and
  builds the Rust dependencies before the agent starts, so that the agent
  works offline in its sandbox, then runs Claude Code on `/codev-apply` in
  evaluation mode: ask no question, do not archive.
- After the run, the runner itself formats, lints, tests and builds the
  fixture, and runs the acceptance script against the binary built from
  the agent's code.
- A run is scored without a model: every task checked; format, lints and
  tests passing; each acceptance assertion; a review record with a
  `code.review` section and no validation error added; nothing changed
  under `_codev/` outside the change's folder, no sync, no archive; the
  documentation of both languages touched when the case expects it; and,
  for a refusing case, no file changed and `/codev-propose` named.
- `codev-evals calibrate <case>` runs a case's acceptance script on the
  merged commit and on the commit before it: every assertion must pass on
  the first and at least one must fail on the second, so that a script
  that cannot tell the two apart is never used.
- The report gives a `code.implement` section — the share of runs that
  pass, the share of acceptance assertions that pass, with their spread —
  and `codev-evals compare` judges the implementation quality as it judges
  the planning quality.
- Four cases: three changes, chosen among those whose merged commit still passes today's checks, for instance of this repository — `decision-accept-command`,
  `forward-compatible-config`, `fix-merge-and-skill-refresh` — and a
  refusal on a change whose tasks are missing. The baseline holds five
  runs of each.
- An implementation run gets its own budget and timeout, above the
  planning ones.

## Capabilities

### New Capabilities

- implementation-evaluation

### Modified Capabilities

### Removed Capabilities

## Impact

- `evals/src/`: the case loader (`case.rs`), the runner (`runner.rs`), a
  new scoring module, the report, the comparison and the CLI (`main.rs`).
- `evals/cases/`: four new cases; `evals/baseline/`: their runs.
- `CONTRIBUTING.md` and `CONTRIBUTING.fr.md`: the evaluation section, which lists the steps and
  how to record a new step's baseline.
- `evals/` is not shipped: the codev binary, the skills and the user
  documentation do not change.

## Domains

- [x] security — the runner builds and runs, outside the sandbox, code an agent wrote, and fetches dependencies
- [ ] personal-data — the runs hold no personal data
- [ ] user-interface — no user interface
- [ ] performance — codev's own performance does not change
- [x] operations — long runs, with a timeout, a budget and a dependency fetch per run
- [ ] migration — no format, configuration or consumer of codev changes

## Release note

None
