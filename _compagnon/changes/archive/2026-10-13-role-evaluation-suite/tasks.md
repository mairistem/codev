# Tasks

The scoring core, the case loader and the pure parts of the runner are
tested without any model, in `#[cfg(test)] mod tests` next to the code;
tests reading the repository's own files (cases, baseline, workflow,
guides) are under `evals/tests/`. Writing a report directory in a test
uses Cargo's `CARGO_TARGET_TMPDIR`, never a system temporary directory.
Running Claude Code itself cannot be automated in the test suite (it
needs a model and a key): task 8.1 runs it for real once, and its output
is the committed baseline that task 8.2 checks.

## 1. Crate

- [x] 1.1 `evals/Cargo.toml` (`codev-evals`, `publish = false`, workspace lints, workspace dependencies only), `members = ["crates/*", "evals"]`, `lib` with modules `case`, `observed`, `score`, `report`, `runner` and a `bin` with `list`, `run`, `score` subcommands, verified by `cargo build -p codev-evals --locked` and `git diff Cargo.lock` adding only the `codev-evals` package

## 2. Cases

- [x] 2.1 `case`: `case.yaml` parsed strictly, the role-to-step table from the `at:` of `assets/roles/*.md`, the validation rules of the spec over the fixture's files (unknown key, role not serving the step, seeded without expectation or control with one, location absent from the fixture, line beyond the end or heading missing, file not UTF-8 or over 64 KiB), and the coverage check, verified by unit tests `a_case_with_an_unknown_key_is_refused` ("An unknown key"), `a_location_beyond_the_end_is_refused` ("A location beyond the end of the file"), `a_binary_fixture_file_is_refused` ("A binary fixture file"), `a_role_that_does_not_serve_the_step_is_refused`, `a_control_with_an_expectation_is_refused`, `a_missing_heading_is_refused`, `coverage_names_a_role_without_control` ("A role without a control")

## 3. Observations and scoring

- [x] 3.1 `observed`: `observed.json` with its format marker and the parts of `ReviewV1` the scoring reads (lenient), verified by unit tests `a_newer_observation_is_refused_with_its_case` ("A newer observation") and `unknown_review_fields_are_ignored`
- [x] 3.2 `score::case`: matching by role and location, true positives, misses, severity agreement, false positives, unreported, duplicates, cross-role, evidence states, outcomes, tracer checks, verified by unit tests `overlapping_lines_match` ("Overlapping lines"), `lines_too_far_away_are_a_miss_and_a_false_positive` ("Lines too far away"), `plan_headings_match_ignoring_case_and_spacing` ("A plan heading written in another case and spacing"), `a_windows_path_matches` ("A Windows path"), `a_location_without_heading_matches_the_whole_file`, `a_control_counts_high_confidence_findings_as_false_positives` ("A clean control"), `the_same_defect_twice_is_one_true_positive_and_one_duplicate` ("The same defect raised twice"), `a_finding_matching_another_roles_defect_is_cross_role`, `evidence_not_found_is_invalid` ("A quote that is not in the code"), `evidence_moved_counts_as_valid` ("A quote a few lines away"), `a_disabled_evaluated_role_is_role_not_run` ("The evaluated role was disabled"), `a_missing_record_misses_every_expectation`, `a_fixed_check_is_a_true_positive_when_the_text_is_there` ("The missing task was added"), `an_unchanged_file_that_changed_is_a_false_positive` ("A complete plan rewritten")

## 4. Report and thresholds

- [x] 4.1 `score::suite` and `report`: per-role totals, recall, evidence validity, the thresholds (flags over `CODEV_EVAL_*`), the failures, the exit code, `report.json` and `report.md` in a fixed order with the run's metadata from `run.json`, the strayed files and the answered findings per case, verified by unit tests `every_role_passing_exits_zero` ("Every role passes"), `a_role_under_the_recall_threshold_fails_with_its_name` ("A role under the recall threshold"), `a_timed_out_run_is_an_error_and_fails_the_suite` ("The run timed out"), `an_invalid_threshold_in_the_environment_is_a_usage_error` ("An invalid threshold"), `a_role_without_checked_evidence_fails_no_evidence_threshold`, `the_report_is_sorted_and_identical_twice` ("Scoring twice gives the same report")

## 5. Runner

- [x] 5.1 Pure parts of `runner`: the `change.yaml` from the roles a step serves, the prompts per step, the Claude Code arguments from the environment, the cases still to run in an output directory, the files modified outside the step from `git status --porcelain`, verified by unit tests `change_yaml_disables_every_other_reviewing_role` ("Only the evaluated role reviews"), `the_review_prompt_names_the_skill_step_and_the_evaluation_mode` ("The prompt names the step and the evaluation mode"), `the_trace_prompt_asks_to_fix_the_plan`, `the_claude_command_takes_model_and_budget_from_the_environment` ("The model and the budget come from the environment"), `a_resumed_run_runs_only_unrecorded_cases` ("A resumed run"), `files_modified_outside_the_step_are_strayed` ("A run that strays outside the step"), `a_resumed_run_with_another_model_is_refused` ("Resuming with another model")
- [x] 5.2 Shell parts of `runner`: preflight, `run.json` written once, fixture setup (git, `codev init`, branch, commits), the Claude Code process in its own process group with its timeout, `codev review --json`, the checked files, `observed.json`, parallel jobs, verified by unit test `preflight_names_a_missing_tool` ("Claude Code is not installed") and by task 8.1's real run
- [x] 5.3 The `codev-evals` binary: `list`, `run` (`--case`, `--role`, `--out`, `--work`, `--keep`, `--rerun`, `--jobs`, `--codev`, the thresholds), `score <dir>` (`--out`), exit codes 0/1/2, verified by integration test `score_writes_identical_reports_twice` in `evals/tests/cli.rs` ("Every role passes", "Scoring twice gives the same report") and `cargo run -p codev-evals -- list` listing every case

## 6. Fixtures

- [x] 6.1 Cases for `security` (TypeScript SQL injection in code, tokens stored and logged in clear in a Rust plan, a docs-only control), `privacy` (Python e-mail in logs, a plan collecting birth dates with no retention, a TypeScript control) and `accessibility` (a TSX icon-only button and image without text, a plan signalling errors by colour only, an HTML form control), verified by `every_case_of_the_suite_loads` in `evals/tests/suite.rs` ("The repository's suite is well formed")
- [x] 6.2 Cases for `performance` (an N+1 query in a Rust and TypeScript monorepo, an unbounded listing in a plan, a control), `ops` (a Python HTTP call without timeout and with unbounded retries, a plan with no rollback or metrics, a control) and `migration` (a SQL migration dropping a column and a renamed JSON field, a plan renaming a public field without **BREAKING**, a control), verified by `every_case_of_the_suite_loads`
- [x] 6.3 Cases for the `newcomer` (a clever one-liner, deep nesting, a control), the `skeptic` (scope creep, an unsupported assumption, a control) and the `tracer` (a scenario without a task, a control), verified by `every_case_of_the_suite_loads` and `the_suite_covers_every_reviewing_role` ("The repository's suite meets the minimum")

## 7. Workflow

- [x] 7.1 `.github/workflows/evals.yml`: `workflow_dispatch` only, the secret checked first with a skip message, Claude Code installed, the suite run with a job timeout, the output uploaded; no secret added, verified by tests `the_workflow_runs_only_on_dispatch` ("Never on a pull request") and `the_workflow_checks_the_secret_before_running` ("No secret") in `evals/tests/suite.rs`

## 8. Baseline

- [x] 8.1 Run the whole suite once for real (`cargo build --bin codev`, `cargo run -p codev-evals -- run --out evals/baseline`), with Claude Code's default model; roles that score badly are recorded, not tuned, verified by `evals/baseline/report.md` and `report.json` listing every case with its scores, the model, the date and the cost
- [x] 8.2 The baseline checks, verified by tests `the_baseline_agrees_with_the_scoring` ("The baseline agrees with the scoring") and `every_case_has_a_baseline_observation` in `evals/tests/suite.rs`, and unit test `missing_from_baseline_names_the_case` ("A case missing from the baseline")

## 9. Documentation

- [x] 9.1 `CONTRIBUTING.md` and `CONTRIBUTING.fr.md` (running the suite, its cost, the before and after scores for any change to `assets/roles/`), and a "Measuring the roles" section in `roles.md` of `docs/en` and `docs/fr` (what a case is, the scores, the baseline `evals/baseline/report.md`), verified by tests `contributing_and_roles_chapters_name_the_baseline` ("Both languages say it") and `the_documentation_check_names_the_file_that_forgets` ("A guide that forgets the rule") and both mdBooks building without warnings

## 10. Verification

- [x] 10.1 Whole workspace, verified by `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --locked -- -D warnings`, `cargo test --workspace --locked`, `codev validate --strict` and `codev doctor --strict` reporting nothing to fix
