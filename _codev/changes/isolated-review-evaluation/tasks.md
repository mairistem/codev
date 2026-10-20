# Tasks

## 1. The runner

- [ ] 1.1 In `evals/src/main.rs` and `evals/src/runner.rs`, `--review-mode` (`inline`, `isolated`) and `--fixture-config <file>` for review cases: appended to `_codev/config.yaml` before rendering the skills again and committing, the sub-agent tool granted in the isolated mode; the options checked before any case runs; both recorded in `run.json`, checked on resume, an earlier `run.json` read as `inline`; verified by unit tests `an_isolated_review_case_says_so_in_its_configuration` (scenario "A review case in the isolated mode"), `a_fixture_config_is_appended_to_every_case` ("A configuration appended to every case"), `options_that_cannot_be_applied_stop_the_run` ("Options that cannot be applied"), `resuming_in_another_mode_stops` ("Resuming in another mode"), `resuming_with_another_fixture_config_stops` ("Resuming with another appended configuration") and `an_earlier_run_json_reads_as_inline` ("A run recorded before these options")
- [ ] 1.2 After an isolated case's run, the check that the role ran alone (`Mode: isolated.`, no `Fallback:`, a sub-agent started), and the source `sub-agents` for the tokens the totals hold beyond the session's own calls, a run whose totals fall below its calls failing; verified by unit tests `a_case_that_fell_back_inline_is_an_error` (scenario "A case that fell back to the inline mode") and `the_sub_agents_tokens_are_a_source` ("The sub-agents' tokens"), on canned records and streams
- [ ] 1.3 Run one isolated review case with Claude Code and check what only a real session shows: the session started a sub-agent and the source `sub-agents` is above zero; verified by the run's figures pasted in the pull request

## 2. The comparison

- [ ] 2.1 In `evals/src/compare.rs`, `--across-models` and the review-mode guard; verified by unit tests `across_models_compares_and_names_both` (scenario "Comparing across models on purpose") and `two_review_modes_are_not_compared` ("Two review modes"), `different_appended_configurations_are_named` ("Different appended configurations"), and the existing model-guard tests kept (scenarios "Two models", "The same default, another model used")

## 3. Documentation

- [ ] 3.1 In `CONTRIBUTING.md` and `CONTRIBUTING.fr.md` (French in vouvoiement), the two run options and `--across-models`; verified by the test `contributing_chapters_document_the_isolated_mode` in `evals/tests/suite.rs`

## 4. Checks

- [ ] 4.1 Run `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --locked -- -D warnings`, `cargo test --workspace --locked` and `codev validate --strict`; verified by all four passing
