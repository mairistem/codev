# Tasks

## 1. The runner

- [ ] 1.1 In `evals/src/main.rs` and `evals/src/runner.rs`, `--review-mode` (`inline`, `isolated`) and `--fixture-config <file>`: for a review case, append to `_codev/config.yaml` before rendering the skills again and committing; `Agent` in the allowed tools in the isolated mode; both recorded in `run.json` and checked on resume; verified by unit tests `an_isolated_review_case_says_so_in_its_configuration` (scenario "A review case in the isolated mode"), `a_fixture_config_is_appended_to_every_case` ("A configuration appended to every case") and `resuming_in_another_mode_stops` ("Resuming in another mode"), and the fixture test in `evals/tests/fixture.rs` extended to the isolated mode
- [ ] 1.2 Run one isolated review case with Claude Code and check that the session's reported cost covers its sub-agent (the per-call sum of the transcript); verified by the two figures pasted in the pull request

## 2. The comparison

- [ ] 2.1 In `evals/src/compare.rs`, `--across-models` and the review-mode guard; verified by unit tests `across_models_compares_and_names_both` (scenario "Comparing across models on purpose") and `two_review_modes_are_not_compared` ("Two review modes"), and the existing model-guard tests kept (scenarios "Two models", "The same default, another model used")

## 3. Documentation

- [ ] 3.1 In `CONTRIBUTING.md` and `CONTRIBUTING.fr.md` (French in vouvoiement), the two run options and `--across-models`; verified by the test `contributing_chapters_document_the_isolated_mode` in `evals/tests/suite.rs`

## 4. Checks

- [ ] 4.1 Run `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --locked -- -D warnings`, `cargo test --workspace --locked` and `codev validate --strict`; verified by all four passing
