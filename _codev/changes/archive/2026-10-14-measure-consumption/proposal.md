# Proposal: Measure what a run consumes, and compare two runs with a verdict

## Why

The next changes aim to make codev's skills consume fewer tokens (targeted
spec excerpts in `codev instructions`, failures-only verifications in apply,
a compact `codev validate`), and each must prove it loses no quality. The
evaluation suite cannot prove it today: it records one run per case, so a
score is a single noisy point; it records the cost but not the tokens nor
the turns that make it; it denies the compound shell commands the agent
naturally writes (51 denials over 373 turns in the committed baseline),
which wastes turns that real use does not; and nothing compares two runs.

## What Changes

- The runner observes, for each case, the tokens Claude Code reports
  (uncached input, cache creation, cache read, output), the number of
  turns, the API duration and the permission denials, next to the cost and
  duration it already records.
- The runner runs the agent as a user working with Claude Code's sandbox
  would: every shell command is approved without a prompt — compound
  commands included — inside a sandbox that confines writes to the
  case's directory and the network to the Anthropic API; anything else
  is denied and counted.
- `codev-evals run --repeat N` runs each case N times. The report gives,
  per role, the median of each quality measure over the repetitions with
  its minimum and maximum, and per case the median cost, tokens, turns and
  duration. Thresholds apply to the medians. With one repetition, the
  report says the same as today.
- New command `codev-evals compare <baseline> <candidate>`: on the cases
  both runs hold, it fails when a role's quality falls outside the range
  the baseline's repetitions spanned, and when the candidate's total cost
  (the sum of each case's median cost) is not lower than the baseline's; `--quality-only` drops the cost
  criterion for changes that do not aim at consumption, such as a role
  change. The duration is shown, never decides.
- The committed baseline is recorded again with five repetitions, with
  open permissions, and the reliability chapter is regenerated from it.
- The contributing guides tell a role change to attach the output of
  `codev-evals compare --quality-only` against the baseline, and an
  optimization change to attach `codev-evals compare`.
- An output directory of the evaluation suite now holds one
  sub-directory per repetition of each case, and `observed.json`,
  `run.json` and `report.json` move to format 2. A directory in the
  previous layout is still read, as one repetition. This concerns only
  the suite's own output, not codev's users.
- The Roles chapter, in both languages, describes the baseline's five
  repetitions and how a role change is judged with `codev-evals compare`.

## Capabilities

### New Capabilities

### Modified Capabilities

- role-evaluation

### Removed Capabilities

## Impact

- `evals/src/`: `observed.rs` (format 2, consumption fields), `runner.rs`
  (permissions, consumption parsing, repetitions), `suite.rs` (layout per
  repetition), `score.rs` and `report.rs` (medians, ranges, consumption),
  `reliability.rs` (the "one run per case" limit becomes the number of
  repetitions), new `compare.rs`, `main.rs` (`--repeat`, `compare`).
- `evals/baseline/`: recorded again, five repetitions of the 40 cases
  (about 70 USD at the last baseline's cost).
- `.github/workflows/evals.yml`: a `repeat` input and the hosted maximum
  timeout.
- `evals/.gitignore`: raw Claude Code logs stay ignored in the
  per-repetition layout.
- `CONTRIBUTING.md`, `CONTRIBUTING.fr.md`, `docs/en/src/roles.md`,
  `docs/fr/src/roles.md`, and the generated `docs/en/src/reliability.md`
  and `docs/fr/src/reliability.md`.
- No change to codev itself: no crate under `crates/`, no skill, no JSON
  contract.

## Domains

- [x] security
- [ ] personal-data — the matches are in the recorded review records of the evaluation fixtures under `evals/baseline/`, data the suite stores about seeded defects; the change handles no personal data
- [ ] user-interface — the matches are in the fixtures' recorded review records under `evals/baseline/`; the change adds no user interface
- [ ] performance — the change measures the consumption of codev's skills; it does not change it
- [x] operations
- [ ] migration — the format change concerns only the evaluation suite's own output directories, which are regenerated, not a user's data or configuration

## Release note

None
