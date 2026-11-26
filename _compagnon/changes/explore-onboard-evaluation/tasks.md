# Tasks

Every task's definition of done also includes `cargo fmt --all --check`,
`cargo clippy --workspace --all-targets -- -D warnings` and
`cargo test --workspace`, all passing.

## 1. The cases and their scoring

- [ ] 1.1 Add `Step::IdeaExplore` (`idea.explore`) and
  `Step::ProjectOnboard` (`project.onboard`) to `evals/src/case.rs`, with
  the case format of the spec and its loading errors; verified by unit
  tests named after the scenarios "An explore case that crystallizes",
  "An onboard case with a request" and "An explore case with an unknown
  outcome".
- [ ] 1.2 Write `evals/src/read_only.rs`: the scoring of the spec (files,
  `_compagnon/changes/`, `contains`, `absent`) and the measures; verified
  by unit tests named after the scenarios "An explore run that writes a
  file", "An onboard run that recommends the wrong action" and "An
  explore run that offers to implement".
- [ ] 1.3 In `evals/src/runner.rs`, the setup (as `setup_configure`), the
  prompt and the observation of the two steps; verified by tests named
  after "The prompt of an explore case" and "An onboard case without
  Compagnon" in `evals/src/runner.rs` and `evals/tests/fixture.rs`.
- [ ] 1.4 Carry the measures into `report.rs`, `compare.rs` and
  `reliability.rs`; verified by a test named after "Comparing two runs of
  explore" and the existing report and reliability tests.

## 2. The cases

- [ ] 2.1 Write the explore cases — `explore-crystallizes-*`,
  `explore-missing-*`, `explore-not-worth-*` — and the five onboard
  cases, one per state of the table, each with a small project in
  `base/`; add the coverage test of the spec to `evals/tests/suite.rs`;
  verified by that test and `every_case_of_the_suite_loads`.
- [ ] 2.2 Document the two steps in `CONTRIBUTING.md` and
  `CONTRIBUTING.fr.md` (the steps the suite covers, their cost); verified
  by the existing contributing tests of `evals/tests/suite.rs`.

## 3. The baseline (decision 0008)

- [ ] 3.1 Ask the user's agreement on the estimated cost of the whole
  group — about 3 USD for this trial run, 15 to 25 USD for the baseline —
  then run each new case once (`run --step idea.explore --step
  project.onboard --repeat 1`) and adjust the `contains` and
  `absent` texts a good reply fails; verified by the runs' output.
- [ ] 3.2 Record the five repetitions into `evals/baseline/`, regenerate the
  reliability chapter (`cargo run -p compagnon-evals -- docs`), and commit;
  verified by `every_baseline_case_has_five_repetitions`,
  `no_baseline_repetition_is_in_error`,
  `the_baseline_agrees_with_the_scoring` and
  `the_reliability_pages_follow_the_baseline`.
