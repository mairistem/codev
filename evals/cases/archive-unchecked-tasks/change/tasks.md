# Tasks

## 1. The roles disabled at a step

- [x] 1.1 In `crates/codev-core/src/roles.rs`, add `disabled_at(roles, step)`: the roles that serve the step and whose activation is `never`, in workflow order; verified by unit tests `the_roles_set_to_never_are_disabled_at_their_step` (project `newcomer: never` → `newcomer` at `code.review`, not at `plan.challenge`) `a_role_turned_back_on_by_the_change_is_not_disabled` (project `never`, change `always`) and `an_inactive_expert_is_not_disabled`
- [x] 1.2 Pass it through `ResolvedRoles` (`crates/codev-engine/src/roles.rs`) and `commands::roles` (`RolesOutcome.disabled`, empty without `--at`); add `disabled` (`id`, `name`, `whenOrigin`) to `RolesV1` and a line naming them to the human output (`render.rs`); verified by the new test `roles_list_the_roles_disabled_at_the_step` (scenario "The roles disabled at a step", JSON and human output, and nothing at a step the role does not serve), `roles_document_exposes_the_contract_fields` with `disabled`, the new `roles_without_at_have_no_disabled` (the rule "empty without `--at`"), and `roles_failure_keeps_the_shape` extended to assert `disabled` is empty (scenario "An unknown step")

## 2. The skills

- [x] 2.1 In `assets/workflows/_review-record.md`: resume with `codev review --change "<name>"` as printed, and word steps a, g and h in the terms of that output — the findings it lists per role, its "Waiting for an answer" list (author, reviewer, human), its "Issues in the record", its low-confidence mark — instead of `findings[]`, `awaiting`, `issues[]` and `reported`; write `Disabled by configuration.` for each role under `disabled` of the step's `codev roles --at` call; check the record with `codev review --change "<name>"`; add the rule to read codev's output as printed, never through another program. In `_isolated-review.md`, take the ids already in the record from the same output; verified by the new test `the_review_record_takes_disabled_roles_from_the_step_and_reads_codev_as_printed` in `crates/codev-agents/src/workflows.rs` (scenario "The disabled roles come with the active ones") and by the existing test `propose_and_apply_describe_the_record_and_the_loop`, updated to expect `codev review --change "<name>"` without `--json` and the human output's terms instead of the JSON field names (scenario "The skills describe the record and the loop")
- [x] 2.1.b After the first measurement, a rule asking for each `codev` command as a call of its own was tried, measured (turns up 15 %, same cost) and removed (review finding R-performance-2); verified by the test `the_review_record_takes_disabled_roles_from_the_step_and_reads_codev_as_printed` no longer asking for it
- [x] 2.2 Refresh codev's own installed skills (`cargo run --bin codev -- update`) and check them; verified by `codev doctor --strict` reporting the skills up to date

## 3. The evaluation prompt

- [x] 3.1 In `evals/src/runner.rs`, have the review prompt say to check the record as the skill says instead of naming `codev review --json`; verified by the unit test `the_review_prompt_names_the_skill_step_and_the_evaluation_mode` extended to assert the prompt no longer contains `--json` after `codev review`

## 4. Documentation

- [x] 4.1 Document `disabled` in `docs/en/src/reference/json-output.md` and `docs/fr/src/reference/json-output.md` (the `codev roles` document and its example), the line naming the disabled roles in the `codev roles` section of both `reference/cli.md`, and, in both `guides/review-record.md`, that the skills resume from `codev review`'s printed output rather than its JSON; verified by `cargo test -p codev-cli` (the docs checks) and a read of the six pages

## 5. Measure

- [x] 5.1 Record a candidate run of the whole suite with this change, three repetitions at the baseline's jobs (`cargo build --bin codev`, `cargo run -p codev-evals -- run --repeat 3 --jobs 4 --out target/evals/fewer-turns`), then `cargo run -p codev-evals -- compare evals/baseline target/evals/fewer-turns`; verified by the comparison's verdict — quality within the baseline's range for every role (required), cost under the baseline's floor (the goal) — and by an explicit check of the claim: the candidate's total turns and permission denials lower than the baseline's, and no `codev review --json | …` or `codev roles … | …` filter left among the candidate's denials (counted from its `claude.json` files). If quality fails, the change stops there and is not shipped. If quality holds and cost does not beat the floor, the numbers go to the human, who decides whether to keep it; the release note is worded from what the comparison showed. Outcome, on the run of the shipped text (`target/evals/fewer-turns`): every role's quality within the baseline's range; cost −7.3 %, above the floor (12.69 against 12.49 USD); denials −46 %; cache reads −16 %; turns −5.6 %; time −12 %. The run with 2.1.b's rule (turns +14.9 %) is not the shipped text. Kept on the user's standing instruction to apply the recommendation without losing quality; the release note gives these numbers
- [x] 5.2 If the change is kept, complete the run of the shipped text to five repetitions (`run --repeat 5 --out target/evals/fewer-turns`, the run without 2.1.b's rule), make it the new `evals/baseline/`, and regenerate the reliability chapter (`codev-evals docs`); verified by `cargo test -p codev-evals` (five repetitions, none in error, pinned Claude Code, pages follow the baseline)

## 6. Checks

- [ ] 6.1 Run `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --locked -- -D warnings`, `cargo test --workspace --locked`, `codev validate --strict`; verified by all four passing
