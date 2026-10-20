# Tasks

## 1. The model of a role

- [ ] 1.1 In `codev-core::roles` and `assets/roles/*.md`, the role's `model` (`opus`, `sonnet`, `haiku` or none), `sonnet` in the frontmatter of `tracer`, `skeptic`, `newcomer`, `security`, `performance` and `migration`; the project's `model:` under `roles:` (`inherit` for none) and its origin; `invalid_role_model` for any other value; verified by unit tests `the_default_models_follow_the_measure` (scenario "The default models"), `a_project_sets_a_role_s_model` ("A project sets a role's model"), `an_unknown_model_is_invalid` ("An unknown model", `codev validate` and `codev roles` failing with `invalid_role_model`), and the existing tests of the configured roles kept (scenarios of "The project configures roles under `roles:`": "A role disabled by the project", "Extra criteria are added after the role's own", "A change overrides the activation", "An unknown key inside a role", "Roles of an inherited source are ignored")
- [ ] 1.2 In `codev-cli`, `model` and `modelOrigin` in the roles document of `contract::v1` (additions); the human output is unchanged; verified by the contract test `the_roles_document_carries_the_model` (scenarios "The default models" and "A project sets a role's model", JSON side)

## 2. The skills

- [ ] 2.1 In `assets/workflows/fragments/review-isolated.md`, start each role's sub-agent on its role's `model`, or without one when it is `null`; run `codev update`; verified by the unit test `a_reviewer_runs_on_its_role_s_model` in `codev-agents` (scenario "A reviewer runs on its role's model") and the existing tests kept: `propose_and_apply_describe_the_isolated_review` (scenarios "The skills describe the isolated review", "Two roles report the same defect", "A sub-agent fails twice", "Material over the limit" and "A change reviewed inline in an isolated project": it checks the merge, the retry and fallback, the summaries and the review-mode reading) and `the_brief_never_carries_the_conversation`

## 3. Documentation

- [ ] 3.1 In `docs/en` and `docs/fr` (French in vouvoiement): the roles chapter (each role's model, the measure behind the defaults, the isolated mode), the configuration reference (`model:`, and to raise `codev_version` first), the JSON reference (`model`, `modelOrigin`); verified by the unit test `both_languages_document_the_role_models` in `crates/codev-cli/src/docs.rs`

## 4. Measure (decision 0008)

- [ ] 4.1 Paste in the pull request the per-role comparison on Sonnet of the design, then run `codev-evals run --step review --repeat 3 --jobs 4` with this branch's binary and compare with `codev-evals compare evals/baseline <out> --quality-only`; verified by both pasted in the pull request

## 5. Checks

- [ ] 5.1 Run `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --locked -- -D warnings`, `cargo test --workspace --locked` and `codev validate --strict`; verified by all four passing
