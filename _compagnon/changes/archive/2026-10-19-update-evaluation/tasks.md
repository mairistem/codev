# Tasks

## 1. The update cases in the suite

- [x] 1.1 In `evals/src/case.rs`, the `plan.update` step and its case: `change`, `artifact`, `request`, `summary`, `expected` (`revised`, `kept`, `ripple`, `contains`, `refuse`) and the `change/` folder, with the loading failures; verified by unit tests `an_update_case_loads` (scenario "A case revising a design"), `an_update_path_the_change_does_not_hold_fails` ("A path the change does not hold") and `a_refusing_case_that_expects_a_revision_fails` ("A refusing case that expects a revision")
- [x] 1.2 In `evals/src/runner.rs`, the fixture (the tree at `ref`, `change/` laid at `_codev/changes/<change>/`, the skills under test, committed), the validation of the fixture before the run, the prompt, the observation (the change's files after the run, the text of every message the agent wrote, `codev validate <change> --json` after the run), and the modified-outside-the-step rule (outside the change's folder); `--step plan.update` selects them; verified by unit tests `the_update_prompt_names_the_skill_and_the_request` (scenario "The prompt of an update case"), `an_update_run_that_edits_the_code_strays` ("A run that edits the code") and `review_does_not_select_update_cases`
- [x] 1.3 In a new `evals/src/update.rs`, the checks and the per-run score; verified by unit tests `editing_the_ripple_fails_the_run` (scenario "A revision that also edits the ripple"), `a_refusal_well_made_passes` ("A refusal well made"), `an_error_already_in_the_fixture_passes_validity` ("An error already in the fixture") and `expected_text_ignores_case_and_spaces` ("Expected text written differently")
- [x] 1.4 In `evals/src/report.rs` and `evals/src/compare.rs`, the `plan.update` section and its per-run checks, and the comparison of the pass rate and of the runs modifying a file outside the step; verified by unit tests `the_report_has_an_update_section` (scenario "The report of a suite with update cases"), `a_report_without_update_cases_has_no_section` and `an_update_regression_fails_the_comparison` ("An update regression")

## 2. The cases and the baseline

- [x] 2.1 Write the six cases of the design under `evals/cases/`, each `change/` copied from its archived change and `ref` the parent of the commit that brought it to `main`; verified by the new test `the_suite_covers_the_update_behaviors` in `evals/tests/suite.rs` (scenario "The update cases of the suite": six `plan.update` cases, one per artifact kind, a refusing one, one whose `ripple` names a delta spec and one whose `ripple` names `tasks.md`) and the existing `every_case_of_the_suite_loads`
- [x] 2.2 Record the update cases alone in `evals/baseline/`, five repetitions each, with the pinned Claude Code (2.1.289) and the binary built from `main` (codev 0.6.1), then `codev-evals score evals/baseline` and commit its `report.json` and `report.md`; in `CONTRIBUTING.md` and `CONTRIBUTING.fr.md`, say that a new step's cases may be recorded alone with the pinned version, changed cases or scoring still requiring a full recording; verified by the existing tests `every_case_has_a_baseline_observation` (scenario "An update case without its baseline runs"), `every_baseline_case_has_five_repetitions` and `the_baseline_agrees_with_the_scoring`

## 3. Documentation

- [x] 3.1 In `evals/src/reliability.rs`, a `plan.update` section of the generated reliability page, in both languages, regenerated with `cargo run -p codev-evals -- docs` once 2.2 has recorded the baseline; in `CONTRIBUTING.md` and `CONTRIBUTING.fr.md` (French in vouvoiement), the update cases, their checks and `--step plan.update`; verified by the existing test `the_reliability_pages_follow_the_baseline` and the new test `contributing_chapters_document_the_update_cases` in `evals/tests/suite.rs`

## 4. Checks

- [x] 4.1 Run `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --locked -- -D warnings`, `cargo test --workspace --locked` and `codev validate --strict`; verified by all four passing
