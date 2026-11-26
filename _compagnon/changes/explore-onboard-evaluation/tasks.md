# Tasks

Every task's definition of done also includes `cargo fmt --all --check`,
`cargo clippy --workspace --all-targets -- -D warnings` and
`cargo test --workspace`, all passing.

## 1. The steps, their runs and their scoring

- [x] 1.1 In `evals/src/case.rs`, add `Step::IdeaExplore` (`idea.explore`)
  and `Step::ProjectOnboard` (`project.onboard`), their parsing for
  `--step` (`main.rs`), and the case format of the spec (`request`,
  `outcome`, `state`, `contains`, `absent`) with its loading errors;
  verified by unit tests named after the scenarios "An explore case that
  crystallizes", "An onboard case with a request", "An explore case with
  an unknown outcome" and "An onboard case without its state".
- [x] 1.2 In `evals/src/runner.rs` and `evals/src/observed.rs`, the setup
  (as `setup_configure`), the prompt of the evaluation mode, the step
  selection and the observation (final message, messages, files
  touched); verified by tests named after "The prompt of an explore
  case", "Selecting the explore cases" and "A run that creates a file" in
  `runner.rs`, and "An onboard case without Compagnon" in
  `evals/tests/fixture.rs`.
- [x] 1.3 Write `evals/src/read_only.rs`: the checks of the spec on the
  final message and the files, and the measures, wired into `score.rs`;
  verified by unit tests named after "An explore run that writes a
  file", "An onboard run that recommends the wrong action", "A reply
  concluding on another outcome" and "Narration is not the conclusion".
- [x] 1.4 Carry the measures into `report.rs`, `compare.rs` and
  `reliability.rs`; verified by new tests named after "The report of an
  explore run", "Comparing two runs of explore" and "The reliability
  chapter", and the existing report tests still passing for a run
  without these steps.

## 2. The cases

- [x] 2.1 Write the three explore cases (`explore-crystallizes-*`,
  `explore-missing-*`, `explore-not-worth-*`) and the six onboard cases,
  one per state, each with a small project in `base/` and both
  `contains` and `absent`; add the coverage test of the spec to
  `evals/tests/suite.rs`; verified by that test and
  `every_case_of_the_suite_loads`.
- [x] 2.2 Document the two steps in `CONTRIBUTING.md` and
  `CONTRIBUTING.fr.md` (the steps the suite covers, `--step idea.explore`,
  `--step project.onboard`, their cost); verified by extending
  `contributing_chapters_document_the_sync_archive_and_configure_cases`
  in `evals/tests/suite.rs` (or a sibling test) with those texts.

## 3. The baseline (decision 0008)

- [x] 3.1 Ask the user's agreement on the estimated cost of the whole
  group — about 3 USD for this trial run, 15 to 25 USD for the baseline —
  then run each new case once (`run --step idea.explore --step
  project.onboard --repeat 1`); adjust the `contains` and `absent` texts
  a good reply fails, and check, by scoring each trial observation
  against another case of the same step, that a reply concluding on
  another outcome or state fails; verified by the runs' output and those
  scores, recorded in the pull request.
- [ ] 3.2 Record the five repetitions into `evals/baseline/`, regenerate
  the reliability chapter (`cargo run -p compagnon-evals -- docs`), and
  commit; verified by `every_baseline_case_has_five_repetitions`,
  `no_baseline_repetition_is_in_error`,
  `the_baseline_agrees_with_the_scoring` and
  `the_reliability_pages_follow_the_baseline`.
