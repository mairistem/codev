# Tasks

## 1. Category vocabulary and case format

- [x] 1.1 `evals/categories.yaml` with every category the existing and new cases use (description and terms), and its strict parser in `case` (unknown key, category without term refused), verified by `a_vocabulary_category_without_term_is_refused`
- [x] 1.2 `case::load` takes the vocabulary: an unknown category is refused, the category's terms are copied into `Expected`; `alternatives` read and checked like `locations`; `suite::load_cases` reads `evals/categories.yaml`; verified by `a_category_outside_the_vocabulary_is_refused` and `an_alternative_beyond_the_end_is_refused`, with the scenarios kept from the first suite still passing (`a_case_with_an_unknown_key_is_refused`, `a_location_beyond_the_end_is_refused`, `a_binary_fixture_file_is_refused`)

## 2. Fixture lint

- [x] 2.1 New pure module `evals/src/lint.rs`: backquoted path and test references of the change's markdown files, resolved against the merged fixture, the change's directory and `planned`, verified by `a_plan_citing_a_missing_file_is_refused`, `a_file_the_plan_creates_is_accepted_when_planned`, `a_task_verified_by_a_missing_test_is_refused` and `the_lint_reads_windows_paths_and_crlf`
- [x] 2.2 `planned` in `case.yaml`: refused in a `code.review` case, when already in the fixture, or cited by no artifact; the lint's problems refuse the case in `case::load`, all named at once; verified by `a_code_case_that_plans_is_refused` and `a_stale_or_unused_planned_entry_is_refused`

## 3. Matching by category

- [x] 3.1 `score::case` pairs by role, location (or alternative) and category with a maximum matching (expected in case order, findings in record order), verified by `overlapping_lines_match`, `lines_too_far_away_are_a_miss_and_a_false_positive`, `the_right_place_with_another_issue_is_a_miss`, `two_close_defects_cited_in_reverse_order_both_pair`, `an_alternative_location_matches`, `plan_headings_match_ignoring_case_and_spacing` and `a_windows_path_matches`
- [x] 3.2 Unpaired findings: duplicate and cross-role require the category; `near` names the expected finding at the same place for a false positive or unreported finding of another category, in `report.json` and `report.md`; verified by `the_same_defect_twice_is_one_true_positive_and_one_duplicate`, `a_control_counts_high_confidence_findings_as_false_positives` and `another_issue_at_the_defects_place_names_it_in_the_report`

## 4. Coverage

- [x] 4.1 `COVERAGE` per expert (3 seeded, 2 controls), a seeded case of two severities per expert, a Python case for security and privacy, verified by `coverage_names_a_role_without_control`, `coverage_names_an_expert_without_two_severities` and `coverage_names_security_without_python`

## 5. Cases

- [x] 5.1 Complete the existing cases: categories from the vocabulary, secondary locations moved to `alternatives`, every cited file or test present in the fixture or `planned` (`skeptic-control-small-flag`, `skeptic-plan-scope-creep`, `newcomer-code-clever-one-liner` and every case the lint names), keeping unchanged the lines existing expectations point to, so that the first baseline's observations still describe the code they cite, verified by `every_case_of_the_suite_loads`
- [x] 5.2 Re-score the first baseline's observations (copied outside the repository, with its `run.json`) against the completed cases under the new matching, and tune `evals/categories.yaml` until every finding that named its seeded defect stays a true positive and the icon button's findings pair the right way round; then freeze the vocabulary for the new run, verified by `cargo run -p codev-evals -- score <copy> --out <dir>` and the per-case lines of its report
- [x] 5.3 Add the fourteen cases of the design's table (security, privacy and their Python cases, accessibility, performance, ops, migration; seeded and controls), verified by `every_case_of_the_suite_loads`, `the_suite_covers_every_reviewing_role` and `categories_of_one_case_share_no_term`

## 6. Baseline

- [x] 6.1 Record the baseline again: remove `evals/baseline/`, `cargo build --bin codev`, `cargo run -p codev-evals -- run --out evals/baseline` on Claude Code's default model, verified by `every_case_has_a_baseline_observation` and `the_baseline_agrees_with_the_scoring`
- [x] 6.2 Fill the design's "Baseline before and after" with each role's scores (first baseline, first baseline re-scored, new baseline) and the cases that explain the moves, verified by reading `design.md` against `evals/baseline/report.md`

## 7. Documentation

- [x] 7.1 `docs/en/src/roles.md` and `docs/fr/src/roles.md` "Measuring the roles": matching by role, location and category, harder cases and controls, verified by `contributing_and_roles_chapters_name_the_baseline` and `mdbook build` of both books without warning
- [x] 7.2 `CONTRIBUTING.md` and `CONTRIBUTING.fr.md`: writing a case (the vocabulary, `alternatives`, `planned`, the lint), the size of a full run, verified by `contributing_and_roles_chapters_name_the_baseline` and reading both guides side by side

## 8. Done

- [x] 8.1 `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --locked -- -D warnings`, `cargo test --workspace --locked`, `codev validate --strict` and `codev doctor --strict` all pass
