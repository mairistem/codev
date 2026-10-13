# Tasks

Scenarios of the `skills` capability describe what an agent does when it
runs a skill; this repository cannot run an agent in its test suite, so
they are covered by tests asserting that the rendered skill body carries
the instruction, as in `review-record`. Engine and CLI tests drive git
through `MockProcessRunner` and files through `MemoryFileSystem`, never a
real repository: what git does with `--relative` or `merge-base` is
git's, and the tests check codev asks for it. Scenarios of MODIFIED
requirements left unchanged stay covered by the tests already named for
them (`roles_at_a_step_as_json`, `roles_at_plan_write_are_the_artifact_roles`,
`roles_follow_the_change_override_and_the_language`,
`roles_without_at_lists_every_role`).

## 1. Core: triggers, noise, matching, suggestions

- [x] 1.1 `codev-core::triggers`: trigger groups parsed from a YAML value (domain, `paths`, `manifests`, `patterns`, unknown keys collected, compile errors and size limit), `regex-automata` as a direct dependency with `globset`'s features (no new crate in `Cargo.lock`), codes `invalid_trigger` and `domain_suggested` in the uniqueness test, verified by unit tests `a_group_parses_its_three_kinds`, `an_unknown_domain_is_invalid`, `a_group_without_trigger_is_invalid`, `an_invalid_pattern_or_glob_is_invalid` ("An invalid project trigger"), `an_oversized_pattern_is_invalid`, `an_unknown_key_in_a_group_is_a_warning` ("An unknown key in a group"), and `git diff --stat Cargo.lock` showing no new package
- [x] 1.2 Built-in triggers in the frontmatter of the six experts (`RoleFrontmatter.triggers`, `Role.triggers`), verified by unit tests `every_expert_carries_triggers_for_its_own_domain` ("Every expert's triggers suggest its own domain") and `security_triggers_name_the_dependency_manifests`
- [x] 1.3 Noise classification and caps (`classify`, the categories, the file, line, per-file and byte caps, path order), verified by unit tests `a_lock_file_is_skipped` ("A lock file suggests nothing"), `vendored_and_planning_files_are_skipped` ("Vendored and planning files suggest nothing"), `prose_files_skip_the_patterns` ("A docs-only change"), `a_binary_file_meets_paths_only` ("A binary file is matched by its path only"), `a_large_file_meets_paths_only`, `a_diff_over_the_file_cap_is_truncated_in_path_order` ("A huge diff is truncated"), `the_byte_cap_stops_the_patterns`, `a_line_over_the_length_cap_is_not_matched`
- [x] 1.4 Matching and suggestions with their states, and the justifications of `codev-core::domains` (listed entries with ticked flag, comment and line), verified by unit tests `a_path_trigger_ignores_case_and_keeps_star_in_its_segment`, `a_manifest_trigger_matches_at_any_depth` ("A manifest trigger matches at any depth"), `a_pattern_trigger_names_the_added_line` ("A pattern trigger names the added line"), `a_ticked_domain_is_ticked` ("A suggested domain the proposal ticks"), `a_justified_domain_carries_its_reason` ("A justified domain"), `none_justifies_nothing` ("`None` does not justify a suggestion"), `matches_are_capped_at_five_in_path_then_line_order`, `crlf_lines_match_like_lf` ("Windows line endings"), and in `domains` `listed_entries_carry_their_comment_and_line`

## 2. Core: stacks and the `By stack` filter

- [x] 2.1 `codev-core::detect::stack::languages` (manifest names to languages, in the fixed order) and `codev-core::roles::filter_by_stack`, verified by unit tests `manifest_names_map_to_languages_in_order`, `filter_by_stack_keeps_detected_languages` ("A Rust project receives the Rust pitfalls only"), `an_item_naming_several_languages_is_kept_for_any` ("An item naming several languages"), `an_item_naming_no_known_language_is_kept`, `the_section_is_left_out_when_nothing_remains`, `no_language_keeps_every_item` ("No language detected")

## 3. Engine: diff, stacks, configuration, suggestions

- [x] 3.1 `FileSystem::is_regular_file` on every implementation (the real one through `symlink_metadata`, which does not follow a link; not tested on the real disk, per the project's rules), and `MemoryFileSystem::with_symlink` and `reads`, verified by unit test `is_regular_file_is_false_for_a_directory_and_a_missing_path` in `ports`
- [x] 3.2 `codev-engine::diff::read` (the git commands of the design, `--relative`, base detection, numstat, untracked files, bounded patch batches, hunk line numbers), verified by tests in `codev-engine::diff` `diff_reads_tracked_and_untracked_changes_against_the_merge_base` ("Every kind of change is in the diff"), `the_base_is_origin_head_when_set` ("The default branch from `origin/HEAD`"), `outside_git_there_is_no_diff` ("Not a git repository"), `without_a_default_branch_there_is_no_diff` ("No default branch"), `without_a_commit_there_is_no_diff`, `the_diff_is_relative_to_the_project_directory` ("A project in a subdirectory of the repository"), `added_lines_are_numbered_from_the_hunk_header` and `removed_lines_are_not_added_lines` ("A removed line suggests nothing"), `paths_are_passed_as_literal_pathspecs_in_batches`, `an_untracked_symbolic_link_is_not_read` ("An untracked symbolic link is not read"), `crlf_is_stripped_from_added_lines`
- [x] 3.3 `codev-engine::detect::stacks` (bounded breadth-first walk), verified by tests `stacks_of_a_monorepo` ("A monorepo receives each of its stacks"), `manifests_in_skipped_directories_do_not_count` ("Manifests in skipped directories do not count"), `the_walk_stops_at_three_levels`
- [x] 3.4 `triggers:` in `ProjectConfig` and `ResolvedConfig` (project only, known key, its line), verified by tests in `codev-engine::config` `triggers_is_a_known_key` ("`triggers` is a known key"), `triggers_of_an_inherited_source_are_ignored` ("Triggers of an inherited source are ignored")
- [x] 3.5 `codev-engine::suggest::for_change` (experts not set to `never` plus the project's valid groups, states from the proposal) and the `invalid_trigger` findings of `validate_config`, verified by tests `a_disabled_expert_suggests_nothing` ("A disabled expert suggests nothing"), `a_project_trigger_suggests_its_domain` ("A project trigger"), `an_invalid_project_group_is_ignored_with_a_warning`, `a_missing_proposal_leaves_every_suggestion_unticked`, and in `codev-engine::validate` `invalid_project_triggers_are_errors_at_their_key` ("An invalid project trigger")

## 4. CLI: `codev roles` and `codev validate`

- [x] 4.1 `codev roles --all-stacks`; `RolesOutcome` with stacks, diff and suggestions; `RolesV1` `stacks`, `diff`, `suggestions` and the failure shape; the knowledge filtered, verified by tests in `codev-cli::commands` `roles_json_suggests_security_for_an_auth_path` ("A path trigger suggests its domain"), `roles_outside_git_report_no_diff`, `roles_without_a_change_have_no_diff` ("Without a change"), `roles_suggestions_are_stable_and_write_nothing` ("Run twice, the same answer"), `roles_serve_by_stack_for_the_detected_languages`, `roles_all_stacks_serve_every_language` ("Every stack on request"), `roles_ignore_an_invalid_project_trigger_with_a_warning`, the updated `roles_rejects_an_unknown_step` ("An unknown step"); in `codev-cli::contract` `roles_document_exposes_stacks_diff_and_suggestions`
- [x] 4.2 Human rendering: the diff's summary or why there is none, one line per suggestion, the stacks before the knowledge, verified by tests in `codev-cli::render` `roles_print_the_diff_and_the_suggestions` ("Suggestions in the human output") and the updated knowledge test ("Knowledge in the human output")
- [x] 4.3 `codev validate` reads the diff once and adds `domain_suggested` to each active change's item, at the domain's line, verified by tests in `codev-cli::commands` `validate_warns_on_an_unticked_suggested_domain` ("An unticked suggestion"), `validate_is_silent_on_a_justified_or_ticked_suggestion` ("A justified or ticked suggestion"), `validate_outside_git_reports_no_suggestion` ("No diff"), `archive_does_not_ask_git_for_a_diff` ("Archive does not read the diff"), and `doctor_does_not_ask_git_for_a_diff`

## 5. Skills

- [x] 5.1 `propose.md` (the product owner ticks or justifies each suggestion; the skeptic's check; no diff), `apply.md` (an `unticked` suggestion under "To decide"), the "Roles" paragraph of both no longer asking the agent to filter `By stack`, verified by tests in `codev-agents` `propose_and_apply_act_on_suggested_domains` ("The skills name the suggestions", "No diff") and `skills_do_not_filter_by_stack_themselves` ("The knowledge is not filtered twice")
- [x] 5.2 Regenerate codev's own skills with `codev update --force`, verified by the diff of `.claude/skills/codev-propose/SKILL.md` and `codev-apply/SKILL.md` showing the suggestions

## 6. Documentation and configuration

- [x] 6.1 `roles.md` (triggers and the built-in outline, the diff and its exclusions and caps, the states and the justification, `domain_suggested`, the project's `triggers:`, the stack filter and `--all-stacks`) and `reference/configuration.md` (`triggers`), in `docs/en` and `docs/fr`, verified by the new test in `codev-cli::docs` `roles_chapters_document_triggers_and_the_stack_filter` ("Both languages document them", "A chapter that forgets the flag")
- [x] 6.2 `reference/cli.md` (`codev roles --all-stacks`, help copied from the binary), `reference/json-output.md` (`stacks`, `diff`, `suggestions`, `domain_suggested`, `invalid_trigger`), `reference/file-formats.md` (the justification on a `## Domains` line and the two codes; the triggers of a role file, a built-in asset, are documented in `roles.md`), `skills.md` (the suggestions in the propose and apply steps), in both languages, verified by `both_languages_have_the_same_chapters` and an mdbook build of both books without warnings
- [x] 6.3 This repository's `_codev/config.yaml` adds a project trigger for the JSON contract (`crates/codev-cli/src/contract.rs` suggests `migration`), verified by `codev roles --change expert-triggers-and-stack-filter --json` listing a `project` match on that file

## 7. Verification

- [x] 7.1 Whole workspace, verified by `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --locked -- -D warnings`, `cargo test --workspace --locked`, `codev validate --strict` and `codev doctor --strict` reporting nothing to fix, and `time codev roles --change expert-triggers-and-stack-filter --json` on this repository under one second
