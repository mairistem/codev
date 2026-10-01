# Tasks

## 1. Schema roles

- [x] 1.1 Prepend a `Role:` line and a `Done when:` list to the four artifact instructions of `assets/schemas/spec-driven/schema.yaml` (product owner, QA analyst, architect, tech lead), keeping the existing text, verified by `the_embedded_schema_is_valid`, the new test `spec_driven_instructions_start_with_a_role_block` (covers "The proposal instruction starts with the product owner role" and "The tasks criteria cover every spec scenario") and `carries_the_schema_template_and_instruction`
- [x] 1.2 Add the test `a_project_schema_instruction_is_returned_as_written` in `codev-engine::instructions`, verified by it passing (covers "A custom schema gets no role block")

## 2. Propose workflow

- [x] 2.1 Add the steps "Check traceability" and "Challenge the plan" to `assets/workflows/propose.md` before "Show the final status", the "Points to challenge" block to its output and the two guardrails, verified by the new test `propose_body_carries_the_review_steps` in `codev-agents::workflows` (covers the scenarios of both new `propose` requirements as written in the skill text)
- [x] 2.2 Regenerate the installed skills, verified by `cargo run -q --bin codev -- update --force` and a diff of `.claude/skills/codev-propose/SKILL.md` showing the new steps

## 3. Documentation

- [x] 3.1 `skills.md`, `workflow.md`, `reference/configuration.md` (`rules:` as project review lenses, with an RGAA example) and `concepts.md` (roles in the schema) in `docs/en` and `docs/fr`, verified by `both_languages_have_the_same_chapters`, `links_between_chapters_become_in_page_anchors` and an mdbook build of both books without warnings
- [x] 3.2 CHANGELOG entries under `[Unreleased]`, verified by reading `CHANGELOG.md`

## 4. Verification

- [x] 4.1 Whole workspace, verified by `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --locked -- -D warnings`, `cargo test --workspace --locked` and `codev validate --strict`
