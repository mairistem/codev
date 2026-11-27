# Tasks

Every task's definition of done also includes `cargo fmt --all --check`,
`cargo clippy --workspace --all-targets -- -D warnings` and
`cargo test --workspace`, all passing.

## 1. The record

- [x] 1.1 Add `added` to `RunInfo` (`evals/src/observed.rs`) as the
  design says, read as empty when absent, and update the test fixtures
  that build a `RunInfo` (`score.rs`, `runner.rs`, `reliability.rs`);
  verified by unit tests in `observed.rs` named after "A run.json without
  added" (it parses as before) and a round trip with an entry.
- [x] 1.2 Write `runner::record_added` and call it from `evals/src/main.rs`
  after `pending`, rewriting `run.json` when it changed; verified by unit
  tests in `evals/src/runner.rs` named after "Steps added with another
  Compagnon", "Resuming with the same Compagnon", "A resume with nothing
  left to run" and "A resume of a few cases", and one for a last entry
  with the same steps and version kept as it is.

## 2. The page, the report and the baseline

- [x] 2.1 In `evals/src/reliability.rs` and `evals/src/report.rs`, give the
  added steps' version and date after the run's; verified by a test named
  after "The chapter names the added steps' version", in both languages,
  and one on the markdown report.
- [x] 2.2 Add the `added` entry to `evals/baseline/run.json`, score the
  baseline again (`cargo run -p compagnon-evals -- score evals/baseline`),
  regenerate the chapter (`cargo run -p compagnon-evals -- docs`);
  verified by `the_baseline_agrees_with_the_scoring`,
  `the_reliability_pages_follow_the_baseline` and both pages showing
  0.7.0 for `idea.explore` and `project.onboard`.
