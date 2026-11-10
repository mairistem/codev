# Tasks

## 1. Where a role's model applies

- [x] 1.1 In `crates/codev-core/src/roles.rs`, `unused_models` spares the `developer` only: a role that serves no reviewing step and is not the developer is warned, a project role at `code.implement` included; verified by the unit tests `a_model_on_the_developer_is_used`, `a_model_on_another_role_at_code_implement_is_unused` and the existing test, changed from `developer` to `product` (roles: "A model on a role that is never a sub-agent", "A model on another role at code.implement", "A model on the developer is used")
- [x] 1.2 A CLI test: `codev validate --json` with `roles: { developer: { model: sonnet } }` reports no `role_model_unused`, and `codev roles --at code.implement --json` gives the `developer` `model: "sonnet"`; verified by `validate_does_not_warn_on_a_developer_model`

## 2. The sub-agent tool

- [x] 2.1 In `crates/codev-agents/src/claude.rs`, the render context records whether the `developer` has a model, and `{{AGENT_TOOL}}` becomes `Agent` in `apply` when the review mode is `isolated` or the developer has a model, in `propose` only in the `isolated` mode; the CLI fills the context from the resolved roles; verified by the unit tests `a_developer_model_adds_the_sub_agent_tool_to_apply` and `a_developer_model_adds_the_sub_agent_tool_to_no_skill_but_apply`, and by the existing isolated- and inline-mode tests still passing (skills: "A developer model adds the sub-agent tool to apply", "Isolated mode adds the sub-agent tool", "Inline mode leaves the tools as they were")

## 3. The apply skill

- [x] 3.1 In `assets/workflows/apply.md`, step 3: when `codev roles --at code.implement` gives the `developer` a model, start one sub-agent on that model with the change's name, the skill's path (steps 2 to 4), the developer's criteria and the project's `rules` and `context`; it returns the tasks checked and any stop; the session re-reads `tasks.md`, relays a stop as step 4 says, then goes on to steps 5 and 6; retry once, then implement in the session and say so; without a model, unchanged; a stop is relayed, then a new sub-agent on the developer's model takes the remaining tasks with the answer (or, without a user, the session's assumption, the brief carrying the session's instructions); a failure, defined apart from a stop, is retried once after inspecting the working tree, then the session implements and says so; verified by the unit tests `the_apply_skill_hands_the_tasks_to_the_developer_model` (the rendered skill names `code.implement`, the developer's `model`, a path without a model that implements in the session, and keeps steps 5 and 6 in the session), `the_apply_skill_resumes_a_stop_on_the_developer_model` and `the_apply_skill_falls_back_after_two_failures` (skills: "The developer's model implements the tasks", "The sub-agent stops on an ambiguity", "The sub-agent cannot be started", "Without a model, nothing changes"). A skill's runtime behavior cannot be unit-tested: what the agent does is measured in 5.1
- [x] 3.2 Refresh codev's own skills with `codev update` (built from this change); verified by a second `codev update` writing nothing and `codev doctor --strict`

## 4. The evaluation

- [x] 4.1 In `evals/src/runner.rs`, `--fixture-config` is appended to an implementation fixture's `_codev/config.yaml` before the skills are installed, and `--review-mode` still reaches only review cases; the comment of `RunInfo::fixture_config` in `evals/src/observed.rs` says so; verified by the unit test `the_fixture_config_reaches_an_implementation_case` (implementation-evaluation: "An implementation run with the developer on Sonnet"; role-evaluation: the modified option requirement)
- [x] 4.2 A suite test: the tree at the `ref` of every implementation case holds no `roles:` key in `_codev/config.yaml`, so that an appended `roles:` is never a duplicate; verified by `no_implementation_case_holds_roles_at_its_ref`

## 5. Measure and record

- [x] 5.1 Measure: `codev-evals run --step code.implement --repeat 5 --jobs 4 --fixture-config <roles: { developer: { model: sonnet } }>` on Opus with a frozen binary named `codev`, then `codev-evals compare evals/baseline <out>`; the bar for recommending `developer: sonnet` is the comparison's full verdict passing — every quality measure within the baseline's range and the total cost under the baseline's floor (decision 0008); verified by that verdict, by every run's `consumption.models` naming a Sonnet model, and by no run's session implementing a task (the delegation happened)
- [x] 5.2 The decision: `codev decision new` then `accept`, superseding 0011 — the model is the role's, chosen by the project, with no built-in default, and the `developer`'s model applies at `code.implement` in either review mode — with the measure of 5.1 (cost against the baseline, quality) and, if 5.1's bar is met, the recommendation `developer: sonnet`; verified by `codev validate --strict` and by `codev decision list` showing 0011 superseded

## 6. Documentation and checks

- [x] 6.1 `docs/en/src/roles.md` and `docs/fr/src/roles.md`: the developer's model implements the tasks in a sub-agent, the session keeps the review, the measure and the recommendation; `role_model_unused` no longer names the developer; the configuration reference in both languages likewise; `docs/en/src/skills.md` and `docs/fr/src/skills.md`: `codev-apply` requests `Agent` in the isolated review mode or when the developer has a model, and hands the tasks to the developer's model; `CONTRIBUTING.md` and `CONTRIBUTING.fr.md`: `--fixture-config` reaches implementation cases; verified by `cargo test -p codev-cli` (the docs checks) and the suite's contributing tests
- [x] 6.2 Run `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --locked -- -D warnings`, `cargo test --workspace --locked` and `codev validate --strict`; verified by all four passing
