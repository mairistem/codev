# Tasks

Scenarios of the `skills` capability describe what an agent does when it
runs a skill; this repository cannot run an agent in its test suite, so
they are covered by tests asserting that the rendered skill body carries
the instruction, as in `review-record` and `built-in-experts`. The
scenarios of the MODIFIED requirement "The `propose` and `apply` skills
take their roles from `codev roles`" keep their existing tests
(`propose_body_carries_the_review_steps` and
`skills_skip_a_disabled_step_and_say_so`); its changed sentence is covered by 4.1.

## 1. Core: mode, material budget, record lines

- [x] 1.1 `codev-core::review::mode`: `ReviewMode` (`inline`, `isolated`), `parse`, `as_str`, `Origin` and `resolve(project, change)`, verified by unit tests `the_default_mode_is_inline`, `the_project_mode_applies_without_an_override`, `a_change_overrides_the_project_mode`, `an_unknown_mode_is_refused_with_the_allowed_values`
- [x] 1.2 `codev-core::review::material`: `budget(files, limit)` and `LIMIT_BYTES` (163840), verified by unit tests `small_material_is_read_in_full` ("Small material is read in full"), `material_over_the_limit_summarizes_the_largest` ("Material over the limit is partly summarized"), `ties_are_broken_by_path`, `an_empty_material_is_not_summarized`
- [x] 1.3 Record: optional `raised_by` (`ReviewFinding::raised_by`), step `Mode:` and `Summarized:` lines, role `Fallback:` line, with their `review_invalid` errors, verified by unit tests in `codev-core::review::parse` `raised_by_credits_the_other_roles` ("A shared finding"), `a_finding_without_raised_by_is_raised_by_its_role` ("A finding without `raised_by`"), `raised_by_must_start_with_the_finding_role` ("A malformed `raised_by`"), `raised_by_names_each_role_once`, `mode_summarized_and_fallback_lines_are_read` ("An isolated step with a summary and a fallback"), `a_record_without_mode_lines_reads_as_before` ("A record written before isolated review"), `an_unknown_mode_line_is_invalid` ("An unknown mode in the record"), `a_second_mode_line_is_invalid`

## 2. Engine: configuration, change, material

- [x] 2.1 `config`: `review` in `PROJECT_KEYS`, `REVIEW_KEYS`, `ResolvedConfig.review_mode`, `EngineError::InvalidReviewMode` (`invalid_review_mode`), verified by tests in `codev-engine::config` `review_mode_is_read_from_the_project` ("The project sets isolated"), `an_unknown_review_mode_fails_with_its_code` ("An unknown mode in the configuration"), `an_unknown_review_key_warns` ("An unknown key under `review`", "No warning for `review`"), `inherited_review_mode_is_ignored` ("An inherited source's mode is not read"), and `every_known_key_is_recognized` updated
- [x] 2.2 `metadata`: optional `review: { mode }` in `change.yaml`, validated, verified by tests in `codev-engine::metadata` `change_review_mode_is_read` and `an_unknown_change_review_mode_fails` ("An unknown mode in a change")
- [x] 2.3 `codev-engine::review_material::measure(fs, layout, change, step, diff)` listing the plan files or the diff's code files with their sizes, verified by tests `plan_material_skips_change_yaml_and_review_md` ("The plan is the material of the challenge"), `code_material_leaves_noise_out` ("Noise is not material"), `code_material_needs_a_diff` ("No diff, no material"), `a_deleted_file_counts_zero_bytes`, `a_symlink_is_not_material` ("Noise is not material")

## 3. CLI: roles, review, doctor

- [x] 3.1 `codev roles`: the `review` object in `RolesOutcome` and `RolesV1` (`null` on failure), human output line for the mode, verified by tests in `codev-cli::commands` `roles_json_gives_the_default_review_mode` ("The default mode is inline"), `roles_json_gives_the_change_review_mode` ("A change overrides the project"), `roles_json_measures_the_code_material` ("Small material is read in full", "Material over the limit is partly summarized"), `roles_json_has_no_material_outside_reviewing_steps` ("No material without a reviewing step"), `roles_failure_has_no_review` ("A failure carries no review"), `status_fails_on_an_unknown_review_mode`; and in `codev-cli::contract` `roles_document_exposes_the_review_mode`
- [x] 3.2 `codev review`: `raisedBy`, `credited`, `mode`, `summarized`, `fallback` in `ReviewV1` and the human output, verified by tests `review_json_exposes_raised_by_and_credit`, `review_json_exposes_mode_summaries_and_fallbacks` in `codev-cli::commands`, `review_document_exposes_the_contract_fields` updated in `codev-cli::contract`, `review_prints_mode_summaries_fallbacks_and_shared_findings` in `codev-cli::render`
- [x] 3.3 `codev doctor`: `SkillState::OtherReviewMode` and `skills_review_mode`, verified by tests in `codev-core::doctor` `skills_rendered_for_another_mode_are_missing` ("Skills rendered before switching to isolated", "A skill of the other mode edited by hand"), `an_outdated_skill_takes_precedence_over_the_mode` ("An outdated skill takes precedence"), and in `codev-cli::commands` `doctor_flags_skills_of_another_review_mode`, `doctor_is_ok_after_update_in_isolated_mode` ("Skills in step with the mode")

## 4. Skills

- [x] 4.1 `RenderCtx.review_mode`, `{{AGENT_TOOL}}` in the `propose` and `apply` entries, the substitution; `plan_skills` regenerates a skill rendered for the other mode; `skill_states` per mode, verified by tests in `codev-agents::claude` `isolated_mode_adds_the_agent_tool_to_review_skills_only` ("Isolated mode adds the sub-agent tool"), `inline_mode_lists_no_agent_tool` ("Inline mode leaves the tools as they were"), `a_skill_of_the_other_mode_is_regenerated` ("Switching to isolated regenerates the skills"), `a_hand_edit_of_the_other_mode_is_preserved` ("A hand edit is still preserved"), `skill_states_report_the_other_review_mode`; and in `codev-cli::commands` `update_regenerates_skills_when_the_mode_changes`
- [x] 4.2 `assets/workflows/_isolated-review.md`, substituted for `{{ISOLATED_REVIEW}}` in `_review-record.md`, `propose.md` step 6 pointing the tracer to it, verified by tests in `codev-agents::workflows` `propose_and_apply_describe_the_isolated_review` ("The skills describe the isolated review", "Two roles report the same defect", "A sub-agent fails twice", "Material over the limit", "A change reviewed inline in an isolated project"), `the_brief_never_carries_the_conversation` (the material as data, the summarizing sub-agent), `the_tracer_runs_alone_at_its_step`
- [x] 4.3 Regenerate codev's own skills with `codev update --force` (the bodies changed under the same version), then set `review: { mode: isolated }` in codev's `_codev/config.yaml` (its `codev_version` is raised with the release that ships the key: the workspace version is not bumped by a change) and run plain `codev update --json`, verified by `updated` listing `codev-propose` and `codev-apply` with `preserved` empty, the diff of their `SKILL.md` (`Agent` in `allowed-tools`), and `codev doctor --strict` reporting `skills` ok

## 5. Documentation

- [x] 5.1 `roles.md` (the two modes, what a reviewer receives, the fallback, the meeting), `reference/configuration.md` (`review.mode`, default, override, switching, raising `codev_version`), `skills.md` (the `Agent` tool), in `docs/en` and `docs/fr`, verified by the new test `both_languages_document_the_review_modes` in `codev-cli::docs` ("Both languages document the review modes"), which also checks that the configuration reference's `review` section names `codev_version`
- [x] 5.2 `reference/file-formats.md` (`review:` in `change.yaml` and the `codev_version` it needs, `raised_by` and the record lines), `reference/json-output.md` (`RolesV1.review`, the `ReviewV1` additions, `invalid_review_mode`, `skills_review_mode`), `guides/review-record.md` (the shared finding, the lines), `faq.md` (switching modes, a sub-agent that fails), in both languages, verified by `both_languages_have_the_same_chapters` and an mdbook build of both books without warnings

## 6. Verification

- [x] 6.1 Whole workspace, verified by `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --locked -- -D warnings`, `cargo test --workspace --locked`, `codev validate --strict` and `codev doctor --strict` reporting nothing to fix
