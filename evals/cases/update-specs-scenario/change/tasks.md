# Tasks

The skill scenarios describe what an agent does when it runs a skill; this
repository cannot run an agent in its test suite, so they are covered by
tests asserting that the skill body carries the instruction, as in
`propose-review-roles`.

## 1. Schema

- [x] 1.1 Append the documentation, automated-test and test-convention criteria to the `tasks` "Done when" list, and ask Impact to name the affected documentation in the `proposal` instruction, in `assets/schemas/spec-driven/schema.yaml`, verified by the new tests `tasks_criteria_require_documentation_and_automated_tests` (covers "The tasks criteria require documentation and automated tests") and `proposal_impact_names_the_affected_documentation` (covers "The proposal instruction starts with the product owner role") in `codev-engine::instructions`, plus the existing `spec_driven_instructions_start_with_a_role_block` ("The tasks criteria cover every spec scenario"), `a_project_schema_instruction_is_returned_as_written` ("A custom schema gets no role block") and `the_embedded_schema_is_valid`

## 2. Skills

- [x] 2.1 `assets/workflows/propose.md`: extend the traceability bullets (automated test per scenario, documentation task), verified by the new test `propose_traceability_requires_automated_tests_and_documentation` in `codev-agents::workflows` (covers "A plan without a documentation task gets one", "A scenario verified only by hand gets an automated test", "A complete plan is left unchanged") and the existing `propose_body_carries_the_review_steps` ("A requirement without an error scenario gets one", "A spec file missing from the proposal is listed")
- [x] 2.2 `assets/workflows/apply.md`: tests after the existing suite, then the language and `rules`; the test command from `context`/`rules`, manifest or CI, asked when not found; the full suite once before the final summary, verified by the new test `apply_body_tests_by_the_project_conventions_and_runs_the_full_suite` (covers the four scenarios of the new `apply` requirement) and the existing `apply_is_in_the_catalog_with_the_right_tools`
- [x] 2.3 `assets/workflows/configure.md`: testing rules in the drafted `tasks` rules, reading existing tests within the budget, the conventions reference after "existing practice wins", verified by the new test `configure_body_carries_testing_rules_and_the_conventions_reference` (covers the three `configure` scenarios) and the existing `configure_is_in_the_catalog_with_the_right_tools`
- [x] 2.4 Regenerate codev's own skills, verified by `cargo run -q --bin codev -- update --force` and a diff of `.claude/skills/codev-{propose,apply}/SKILL.md` showing the new text (`configure` is not in this project's `workflows:`)

## 3. Documentation

- [x] 3.1 `skills.md` (propose, apply, configure), `workflow.md` (apply step) and `reference/configuration.md` (testing rules example per language) in `docs/en` and `docs/fr`, verified by `both_languages_have_the_same_chapters`, `links_between_chapters_become_in_page_anchors` and an mdbook build of both books without warnings
- [x] 3.2 CHANGELOG entries under `[Unreleased]`, verified by reading `CHANGELOG.md`

## 4. Verification

- [x] 4.1 Whole workspace, verified by `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --locked -- -D warnings`, `cargo test --workspace --locked`, `codev validate --strict` and `codev doctor` reporting nothing to fix
