# Tasks

## 1. The runner

- [ ] 1.1 In `evals/src/main.rs` and `evals/src/runner.rs`, `--review-mode` (`inline`, `isolated`) and `--fixture-config <file>`: for a review case, append to `_codev/config.yaml` before rendering the skills again and committing; `Agent` in the allowed tools in the isolated mode; both recorded in `run.json` and checked on resume; verified by unit tests `an_isolated_review_case_says_so_in_its_configuration` (scenario "A review case in the isolated mode"), `a_fixture_config_is_appended_to_every_case` ("A configuration appended to every case"), `resuming_in_another_mode_stops` ("Resuming in another mode"), `resuming_with_another_fixture_config_stops` ("Resuming with another appended configuration"), `an_earlier_run_json_reads_as_inline` ("A run recorded before these options"), and `an_isolated_skill_without_the_agent_tool_is_an_error` ("An isolated skill that cannot start sub-agents": the check the runner applies to the rendered skills, on canned skill text)
- [ ] 1.2 Run one isolated review case with Claude Code and check that the session's reported cost covers its sub-agent: only a real session shows how Claude Code reports a sub-agent's cost, so this check stays manual; it passes when the session's reported cost is at least the sum of the per-call costs of the session and its sub-agent in the transcript, within 1 %; if it fails, no isolated measure is used until the runner adds the sub-agent's figures; verified by the figures pasted in the pull request

## 2. The comparison

- [ ] 2.1 In `evals/src/compare.rs`, `--across-models` and the review-mode guard; verified by unit tests `across_models_compares_and_names_both` (scenario "Comparing across models on purpose") and `two_review_modes_are_not_compared` ("Two review modes"), and the existing model-guard tests kept (scenarios "Two models", "The same default, another model used")

## 3. Documentation

- [ ] 3.1 In `CONTRIBUTING.md` and `CONTRIBUTING.fr.md` (French in vouvoiement), the two run options and `--across-models`; verified by the test `contributing_chapters_document_the_isolated_mode` in `evals/tests/suite.rs`

## 4. Checks

- [ ] 4.1 Run `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --locked -- -D warnings`, `cargo test --workspace --locked` and `codev validate --strict`; verified by all four passing
