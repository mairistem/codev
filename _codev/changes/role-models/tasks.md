# Tasks

## 1. The model of a role

- [x] 1.1 In `codev-core::roles` and `assets/roles/*.md`, the role's `model` (`opus`, `sonnet`, `haiku` or none), none for every built-in role; the project's `model:` under `roles:` (`inherit` for none) and its origin; `invalid_role_model` for any other value, `role_model_unused` for a model on a role serving no reviewing step; an inherited source's `roles:` skipped, not parsed; verified by unit tests `no_built_in_role_carries_a_model` (scenario "The default models"), `a_project_sets_a_role_s_model` ("A project sets a role's model"), `an_unknown_model_is_invalid` ("An unknown model", `codev validate` and `codev roles` failing with `invalid_role_model`), `a_model_off_the_review_steps_is_a_warning` ("A model on a role that is never a sub-agent"), the engine test `a_source_s_unknown_role_key_keeps_its_rules` on the in-memory FileSystem ("A source's role setting from a newer codev"), and the existing tests of the configured roles kept (scenarios of "The project configures roles under `roles:`": "A role disabled by the project", "Extra criteria are added after the role's own", "A change overrides the activation", "An unknown key inside a role", "Roles of an inherited source are ignored")
- [x] 1.2 In `codev-cli`, `model` and `modelOrigin` in the roles document of `contract::v1` (additions); the human output is unchanged; verified by the contract test `the_roles_document_carries_the_model` (scenarios "The default models" and "A project sets a role's model", JSON side)

## 2. The skills

- [x] 2.1 In `assets/workflows/fragments/review-isolated.md`, start each role's sub-agent on its role's `model`, or without one when it is `null`; run `codev update`; verified by the unit test `a_reviewer_runs_on_its_role_s_model` in `codev-agents` (scenario "A reviewer runs on its role's model") and the existing tests kept: `propose_and_apply_describe_the_isolated_review` (scenarios "The skills describe the isolated review", "Two roles report the same defect", "A sub-agent fails twice", "Material over the limit" and "A change reviewed inline in an isolated project": it checks the merge, the retry and fallback, the summaries and the review-mode reading) and `the_brief_never_carries_the_conversation`

## 3. Documentation

- [x] 3.1 In `docs/en` and `docs/fr` (French in vouvoiement): the roles chapter (each role's model, the measure behind the defaults, the isolated mode), the configuration reference (`model:`, and to raise `codev_version` first), the JSON reference (`model`, `modelOrigin`); verified by the unit test `both_languages_document_the_role_models` in `crates/codev-cli/src/docs.rs`

## 4. Measure (decision 0008)

- [x] 4.0 Prerequisite: the change `isolated-review-evaluation` is merged, so the suite plays the review cases in the `isolated` mode
- [x] 4.1 With this branch's binary, run `codev-evals run --step review --repeat 3 --jobs 4` in the `isolated` mode twice the same day: the reference with every reviewing role set to `model: inherit`, the candidate with this change's defaults; compare with `codev-evals compare <reference> <candidate>`: quality within the reference's range, cost under its floor; verified by the comparison and the sources table pasted in the pull request, the defaults decided from it (none, the saving being under 0008's floor and two roles slipping)

## 5. Checks

- [ ] 5.1 Run `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --locked -- -D warnings`, `cargo test --workspace --locked` and `codev validate --strict`; verified by all four passing
