# Tasks

The `apply` scenarios describe what an agent does when it runs the skill;
this repository cannot run an agent in its test suite, so they are covered
by tests asserting that the skill body carries the instruction, as in
`docs-and-tests-in-done`.

## 1. Schema

- [x] 1.1 Append the simplicity criterion to the architect's "Done when" list in `assets/schemas/spec-driven/schema.yaml`, verified by the new test `design_criteria_require_the_simplest_approach_a_newcomer_can_follow` in `codev-engine::instructions` (covers "The design criteria require the simplest approach"), plus the existing `spec_driven_instructions_start_with_a_role_block`, `tasks_criteria_require_documentation_and_automated_tests` and `proposal_template_and_instruction_carry_the_release_note` (the unchanged scenarios of the modified requirement), `a_project_schema_instruction_is_returned_as_written` ("A custom schema gets no role block") and `the_embedded_schema_is_valid`

## 2. Skill

- [x] 2.1 `assets/workflows/apply.md`: new step "5. Review as a newcomer" (scope, grid, project `rules`, simplify and re-run the affected tests, guardrails), "Finish" renumbered 6 with the "Readability notes" block in its summary, verified by the new tests in `codev-agents::workflows` `apply_reviews_the_change_as_a_newcomer_before_the_full_suite` (covers "A needlessly clever construction is simplified"), `apply_newcomer_review_keeps_behavior_scope_and_outside_code` (covers "The review keeps behavior and scope", "Code outside the change is untouched") and `apply_summary_caps_readability_notes_and_never_invents_them` (covers "At most three readability notes", "Nothing to note"), plus the existing `apply_body_tests_by_the_project_conventions_and_runs_the_full_suite` and `action_skills_carry_the_update_notice_once`
- [x] 2.2 Regenerate codev's own skills, verified by `codev update --force` and a diff of `.claude/skills/codev-apply/SKILL.md` showing the new step

## 3. Documentation

- [x] 3.1 `roles.md` (newcomer reviewer role in the `/codev-apply` part, architect criterion), `skills.md` (apply) and `workflow.md` (apply step) in `docs/en` and `docs/fr`, verified by `both_languages_have_the_same_chapters`, `links_between_chapters_become_in_page_anchors`, an mdbook build of both books without warnings, and no duplicate heading anchor in either book

## 4. Verification

- [x] 4.1 Whole workspace, verified by `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --locked -- -D warnings`, `cargo test --workspace --locked`, `codev validate --strict` and `codev doctor --strict` reporting nothing to fix
