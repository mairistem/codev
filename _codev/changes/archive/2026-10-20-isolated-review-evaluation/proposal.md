# Proposal: Evaluate the reviews in the isolated mode

## Why

The suite plays every review case in the `inline` mode: one session
applies the role. In the `isolated` mode, which this project uses and
`role-models` targets, the session starts the role as a sub-agent with
its own brief and, soon, its own model. Decision 0008 asks that a step
the suite does not cover be covered first, by its own change, and
`role-models` cannot be measured until it is. Its measure also compares
runs that use different models on purpose, which `compare` refuses
today.

## What Changes

- `codev-evals run --review-mode isolated` plays the review cases in the
  `isolated` mode: each case's configuration says so before the skills
  are rendered, and the sub-agent tool is granted.
- `codev-evals run --fixture-config <file>` appends a configuration to
  every case, for a setting a measure needs.
- `run.json` records both; a resumed run with other values stops.
- `codev-evals compare --across-models` compares two runs whose models
  differ, for a change that is a change of model, naming both; two runs
  in different review modes are never compared.

## Capabilities

### New Capabilities

### Modified Capabilities

- role-evaluation

### Removed Capabilities

## Impact

- `evals/src/runner.rs`, `evals/src/main.rs`, `evals/src/compare.rs`,
  `evals/src/observed.rs` (`run.json`).
- No change to codev itself: `evals/` is not shipped.
- `CONTRIBUTING.md` and `CONTRIBUTING.fr.md`: the two options.

## Domains

- [ ] security — the cases run in the same sandbox; the sub-agent tool is granted only inside it
- [ ] personal-data — no personal data is read or written
- [ ] user-interface — no interface beyond the runner's options
- [ ] performance — the options only run what the suite already runs, in another mode
- [ ] operations — no log, metric, timeout or deployment changes
- [ ] migration — `run.json` gains two optional fields, read as their defaults when absent

## Release note

None
