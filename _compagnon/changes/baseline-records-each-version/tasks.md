# Tasks

Every task's definition of done also includes `cargo fmt --all --check`,
`cargo clippy --workspace --all-targets -- -D warnings` and
`cargo test --workspace`, all passing.

## 1. The record

- [ ] 1.1 Add `added` to `RunInfo` (`evals/src/observed.rs`) as the
  design says, read as empty when absent; verified by a unit test that a
  `run.json` without `added` parses as before and one with it
  round-trips.
- [ ] 1.2 In `evals/src/main.rs` and `evals/src/runner.rs`, append the
  entry on a resume with another Compagnon and rewrite `run.json`, nothing
  on a resume with the same one; verified by tests named after the
  scenarios "Steps added with another Compagnon" and "Resuming with the
  same Compagnon".

## 2. The page and the baseline

- [ ] 2.1 In `evals/src/reliability.rs`, give the added steps' version
  and date after the run's; verified by a test named after "The chapter
  names the added steps' version", in both languages.
- [ ] 2.2 Add the `added` entry to `evals/baseline/run.json` and
  regenerate the chapter (`cargo run -p compagnon-evals -- docs`);
  verified by `the_baseline_agrees_with_the_scoring`,
  `the_reliability_pages_follow_the_baseline` and both pages showing
  0.7.0 for `idea.explore` and `project.onboard`.
