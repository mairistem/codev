# Tasks

## 1. Core

- [x] 1.1 `codev_core::managed`: markers and `splice` (create, append, refresh, unchanged, malformed; `Init` and `Refresh` modes), verified by `splice_creates_a_missing_file`, `splice_appends_after_user_content_with_a_blank_line`, `splice_refreshes_only_between_the_markers`, `splice_is_unchanged_when_the_section_is_current`, `splice_refuses_unbalanced_or_repeated_markers` and `refresh_mode_never_creates_nor_appends` (covers "A section is appended to an existing `CLAUDE.md`", "`codev update` refreshes only the section", "Unbalanced markers leave the file alone")
- [x] 1.2 `codev_core::config::inherit::parse` and the `inherits:` block of the generated config, verified by `parses_scp_https_ssh_file_and_dot_git_sources`, `splits_the_ref_after_the_path_not_the_user`, `defaults_the_ref_to_main`, `refuses_empty_values_and_empty_refs`, `render_with_inherits_matches_golden` and the engine round trip `generated_config_with_inherits_reads_back` (covers "A git source with a ref and a local path", "A git source without a ref gets `main`")
- [x] 1.3 `codev_core::detect::forge` from the text of `.git/config`, verified by `github_and_gitlab_remotes_are_recognized` and `no_remote_means_no_forge`
- [x] 1.4 `codev_core::doctor::evaluate` with the ten checks, their ids, codes, groups and fixes, verified by `fresh_project_checks`, `outdated_skills_are_missing_with_skills_outdated`, `unpinned_source_is_missing`, `stale_pin_is_a_warning`, `pr_template_check_only_on_github`, `validate_warnings_are_a_warning_and_errors_missing` and `summary_counts_missing_and_warnings`

## 2. Engine and agents

- [x] 2.1 `codev_engine::integration`: `RepoFacts` probe and the CI and pull request template plans (GitHub, GitLab snippet, none; existing unmanaged `codev.yml`; another workflow already validating; template locations; `Init` and `Refresh` modes), verified by `github_project_gets_a_managed_workflow_pinned_to_the_version`, `gitlab_project_gets_a_snippet_and_no_write`, `unmanaged_codev_yml_is_left_alone`, `existing_validate_workflow_skips_ci`, `pr_template_section_goes_into_the_first_existing_template`, `pr_template_is_created_when_none_exists` and `refresh_bumps_the_managed_workflow_and_creates_nothing` (covers the CI and pull request template scenarios of `init`)
- [x] 2.2 `ClaudeCode::plan_claude_md` and `ClaudeCode::skill_states`, verified by `claude_md_section_names_propose_apply_and_the_installed_skills`, `claude_md_falls_back_to_tasks_when_apply_is_not_installed` and `skill_states_report_missing_outdated_and_current`
- [x] 2.3 Update the `sources` module documentation: init may pin new sources, only `sources update` moves a pin, verified by reading `crates/codev-engine/src/sources/mod.rs`

## 3. CLI

- [x] 3.1 `Ctx` carries a `ProcessRunner`; `codev init` gains `--inherit`, `--no-claude-md`, `--no-ci`, `--no-pr-template`, `--no-integration`; it validates sources before writing, writes `inherits:`, warns `inherit_not_written` on an existing config, executes the integration plan, pins new git sources only when no lock exists and turns failures into `inherit_pin_failed`; `codev update` refreshes managed files only, verified by `init_writes_inherits_and_pins_git_sources`, `init_refuses_a_missing_inherit_path_before_writing`, `init_keeps_an_existing_config_and_warns_inherit_not_written`, `init_survives_a_pin_failure`, `init_writes_the_three_integration_files_on_github`, `init_no_integration_writes_none`, `init_no_ci_skips_only_the_workflow`, `init_outside_github_writes_only_claude_md_and_says_why`, `init_appends_to_an_existing_claude_md`, `rerunning_init_changes_nothing`, `rerunning_init_with_integration_is_idempotent`, `init_records_the_doctor_count_for_the_hint` and `update_refreshes_the_claude_md_section_and_adds_nothing_when_opted_out`
- [x] 3.2 `codev doctor [--json] [--strict]`: fact gathering, human rendering, exit code, `DoctorReportV1`, verified by `doctor_on_a_fresh_project_reports_the_expected_statuses`, `doctor_reports_outdated_skills_and_unpinned_sources`, `doctor_recognizes_cargo_run_validate_in_ci`, `doctor_writes_nothing`, `doctor_outside_a_project_fails_with_no_codev_root`, the render test `doctor_output_groups_checks_and_ends_with_the_count`, the contract tests `doctor_report_exposes_the_expected_fields` and `doctor_failure_keeps_the_shape`, and `codev doctor --strict; echo $?` in a scratch project with and without missing checks (covers every `doctor` scenario)
- [x] 3.3 Human summary of `init`/`update` with the integration steps, the GitLab snippet, `help: run codev sources update`, the `codev doctor` hint; `SetupV1.integration`; the `codev status` hint on stderr, verified by `setup_lists_integration_steps_and_reasons`, `setup_ends_with_the_doctor_hint_when_something_is_missing`, `setup_report_exposes_integration_in_camel_case` and a real `codev status` run in a scratch project (covers "The summary explains a skipped step", "The JSON report lists the integration steps", "Hint on a project with missing checks", "No hint when everything is in place")

## 4. Skills

- [x] 4.1 `assets/workflows/onboard.md` runs `codev doctor --json` and relays the missing checks, read-only, verified by `onboard_body_relays_codev_doctor` and the existing `onboard` catalog tests (covers the four scenarios of the `onboard` doctor requirement)
- [x] 4.2 Regenerate codev's own skills, verified by `cargo run -q --bin codev -- update --force` (the version stamp is unchanged, so a plain `update` keeps the skill as hand-edited) and a diff of `.claude/skills/codev-onboard/SKILL.md`

## 5. Real runs

- [x] 5.1 In scratch repositories: `--inherit` with a local bare git repository (`file://` and `.git` path, with and without `@ref`), an unreachable URL, an invalid path; a repository with an existing `CLAUDE.md` and PR template; a GitLab repository; `codev update` after editing user text around a stale managed section, verified by reading the written files, `codev sources list` and `git diff` of the user content

## 6. Documentation

- [x] 6.1 `installation.md`, `quickstart.md`, `reference/cli.md` (help blocks regenerated from the binary), `reference/configuration.md`, `guides/inherited-sources.md`, a `codev doctor` section, `reference/json-output.md` and `faq.md` in `docs/en` and `docs/fr`, with real outputs, verified by `both_languages_have_the_same_chapters`, the docs anchor tests and an mdbook build of both books without warnings
- [x] 6.2 CHANGELOG `[Unreleased]` Added entries, verified by reading `CHANGELOG.md`

## 7. codev's own repository

- [x] 7.1 Run the new `codev init` integration on this repository: `CLAUDE.md` with the managed section, the managed section in `.github/PULL_REQUEST_TEMPLATE.md` (replacing the hand-written duplicate lines), no `codev.yml`, verified by `codev doctor` ending with `0 to fix`

## 8. Verification

- [x] 8.1 Whole workspace, verified by `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --locked -- -D warnings`, `cargo test --workspace --locked`, `codev validate init-complete-project` and `codev validate --strict`
