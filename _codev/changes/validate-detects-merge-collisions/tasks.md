# Tasks

Every task is done when `cargo fmt --all --check`,
`cargo clippy --workspace --all-targets -- -D warnings` and
`cargo test --workspace` pass. Engine and CLI tests use the in-memory
`FileSystem` and `FixedEnv` ports, never temporary directories, and build
their fixtures with explicit `\n` or `\r\n`, never the platform's.

## 1. Duplicate requirements in a main spec

- [ ] 1.1 In `codev-core/src/parser/spec.rs`, report `duplicate_requirement`
  for a requirement name defined twice in the requirements section of a
  main spec, reusing the detection of `parse_requirement_blocks` in
  `parser/delta.rs` (extracted into `parser/shared.rs` if needed), both
  requirements kept in the result; verified by the unit tests
  `a_main_spec_defining_a_requirement_twice_reports_both_lines` and
  `distinct_names_in_a_main_spec_are_not_duplicates`.
- [ ] 1.2 Check that `codev validate` reports it as an error of the spec
  item and exits non-zero; verified by the engine test
  `validate_reports_a_duplicate_requirement_in_a_main_spec` in
  `codev-engine/src/validate/mod.rs`.

## 2. A delta checked against its main spec

- [ ] 2.1 In `codev-core/src/validate/codes.rs`, add
  `MODIFIED_TARGET_MISSING` (`modified_target_missing`, the code the
  merge already uses) and `ADDED_TARGET_EXISTS` (`added_target_exists`);
  add both to `CROSS_FILE_CODES`; verified by the existing
  code-uniqueness test of `validate/mod.rs`.
- [ ] 2.2 In `check_delta_against_spec`, report a `MODIFIED` requirement the
  main spec does not hold, unless a `RENAMED` of the same delta has it as
  `TO`; verified by the unit tests
  `a_modified_requirement_the_spec_no_longer_holds_is_an_error` and
  `modifying_a_requirement_the_same_delta_renames_is_accepted`.
- [ ] 2.3 Report an `ADDED` requirement whose name the main spec holds with
  a different content, compared on the parsed requirement — description,
  scenario names and bodies — with CRLF read as LF, trailing whitespace of
  each line and trailing blank lines ignored; leave a `REMOVED` of an
  absent requirement unreported; verified by the unit tests
  `an_addition_the_spec_holds_with_another_text_is_an_error`,
  `an_addition_already_synced_from_a_french_crlf_delta_is_accepted` and
  `a_removal_already_applied_is_not_reported`; in the same task, update
  the comment of the `ADDED` skip in `codev-core/src/merge/mod.rs` to name
  `added_target_exists` as the upstream check, verified by
  `grep added_target_exists crates/codev-core/src/merge/mod.rs`.
- [ ] 2.4 Report, as errors of the change, the `duplicate_requirement` and
  `merge_conflict_marker` findings of each main spec its deltas target
  (today `read_main_spec` in `codev-engine/src/validate/mod.rs` drops the
  parser's findings), naming the spec and the lines; verified by the
  engine tests `validate_change_reports_a_duplicate_in_a_targeted_spec`
  and `archive_refuses_a_change_targeting_a_spec_with_a_duplicate`.
- [ ] 2.5 In `codev-core/src/merge/mod.rs`, skip a `REMOVED` whose
  requirement the main spec no longer holds instead of returning
  `ModifiedTargetMissing`; verified by the unit test
  `a_removal_of_a_requirement_already_gone_is_skipped` and the engine test
  `archive_succeeds_for_a_removal_another_branch_already_merged`.
- [ ] 2.6 Check that `codev archive` and `codev sync` refuse an `ADDED`
  collision with `validation_failed` and still archive a change whose
  addition was already synced; verified by the engine tests
  `archive_refuses_an_addition_another_change_made` and
  `archive_after_a_sync_keeps_a_single_copy_of_an_addition` in
  `codev-engine/src/archive.rs`, and the existing rename tests unchanged.

## 3. Conflict markers

- [ ] 3.1 Add a pure scan in `codev-core` (next to the validation rules)
  that returns the lines starting with seven `<`, `|` or `>` followed by a
  space or the end of the line, skipping fenced code blocks for markdown
  with the fence tracking of `parser/fence.rs`, never matching `=======`,
  under a new code `merge_conflict_marker`; verified by the unit tests
  (the scan alone decides the heading-underline and fence scenarios, which
  3.2 also checks through `validate`)
  `opener_base_and_closer_are_markers`, `markers_in_a_fence_are_ignored`,
  `a_heading_underline_is_not_a_marker` and
  `markers_are_found_with_crlf_line_endings`.
- [ ] 3.2 Run the scan in `codev-engine/src/validate/` on each main spec,
  on every markdown file of each active change (artifacts, delta specs,
  `review.md`, `corrections.md`) and on each local decision, as errors
  of the item that file belongs to, at the marker's line; verified by the
  engine tests `validate_reports_a_conflict_left_in_a_main_spec`,
  `validate_reports_a_conflict_left_in_a_design`,
  `validate_reports_a_conflict_left_in_a_decision`,
  `validate_ignores_a_marker_shown_in_a_decision_code_block` and
  `validate_ignores_a_heading_underline_in_a_design`.
- [ ] 3.3 When `_codev/config.yaml`, `_codev/decisions/seal.yaml` or an
  active change's `change.yaml` fails to parse and holds a marker, fail with a new engine error whose code is
  `merge_conflict_marker` and whose message names the file and the line,
  instead of the YAML error; verified by the engine test
  `a_conflict_left_in_the_seal_file_names_its_line` and
  `a_conflict_left_in_the_config_names_its_line` and
  `a_conflict_left_in_a_change_yaml_names_its_line`, and the CLI test
  `validate_json_fails_with_merge_conflict_marker` in `codev-cli`.

## 4. Decisions

- [ ] 4.1 In `codev-engine/src/decisions.rs`, report `decision_id_duplicate`
  (error, new code in `parser/codes.rs` and its `ALL` list) for an id
  carried by two or more local decisions, naming every file and the next
  free id; keep `decision_id_collision` for project against source;
  verified by the unit tests
  `two_local_decisions_with_one_id_are_a_duplicate_naming_the_next_free_id`
  and `a_local_id_shared_with_a_source_stays_a_collision_warning`.
- [ ] 4.2 In `validate_decisions`, stop keying the checked paths by id alone
  so both duplicates are sealed-checked, and report
  `decision_seal_duplicate` (error) for an id `seal.yaml` seals twice;
  verified by the engine tests
  `validate_reports_a_seal_file_sealing_one_id_twice`,
  `validate_checks_the_seal_of_both_decisions_sharing_an_id` and
  `validate_reports_nothing_for_distinct_ids_sealed_once`; while an id is
  sealed several times, a decision matching any of its entries raises no
  `decision_seal_mismatch`, verified by
  `two_decisions_each_matching_their_own_seal_entry_raise_no_mismatch`.
- [ ] 4.3 Make `plan_seal_force` in `codev-core/src/decisions/seal.rs`
  leave a single entry for an id sealed several times; verified by the
  unit test `seal_force_collapses_a_duplicated_entry` and the engine test
  `seal_force_on_a_duplicated_seal_clears_decision_seal_duplicate`.

## 5. Documentation

- [ ] 5.1 Write `docs/en/src/guides/parallel-changes.md` and
  `docs/fr/src/guides/parallel-changes.md` (French in vouvoiement), listed
  in both `SUMMARY.md`: the rule (archive inside the pull request, last
  commit, after updating from the default branch; CI validates the merge
  result, which needs "require branches to be up to date" or a merge
  queue on GitHub and merged results pipelines on GitLab, otherwise a
  collision between two pull requests merged back to back is caught after
  the merge; conflicts inside fenced code blocks are not detected), each
  error and its
  resolution (duplicate requirement, `decision_id_duplicate` with the
  manual renumbering and `codev decision seal --force`, `decision_seal_duplicate`,
  `merge_conflict_marker`, `modified_target_missing`,
  `added_target_exists`), the generated files regenerated
  (`codev decision seal`, `codev changelog --write`, `codev update`), and
  stale skills in another clone reported by `codev doctor`; verified by
  the `codev-cli` test `both_languages_document_parallel_changes`
  (table of contents) and `mdbook build docs/en` and `docs/fr`.
- [ ] 5.2 Add the five codes to the finding-code table of
  `reference/file-formats.md` and `merge_conflict_marker` to the error
  table of `reference/json-output.md`, in both languages, change the
  `duplicate_requirement` row to cover main specs, and link the guide from
  the archive step of `workflow.md` and from `faq.md`, in both languages;
  verified by the `codev-cli` test
  `every_merge_collision_code_is_documented_in_both_languages`, which
  checks the guide, the `json-output.md` error row, the table (the `duplicate_requirement` row naming main
  specs) and that `workflow.md` and `faq.md` link
  `guides/parallel-changes.md`, in both languages.

## 6. Verification

- [ ] 6.1 Run `codev validate --strict` with the built binary on this
  repository: no new error on its own specs and decisions; verified by the
  command's output, pasted in the pull request.
- [ ] 6.2 Run `codev-evals run --step plan.write` and
  `codev-evals compare` against the committed baseline, as decision 0008
  requires for an output the agents read (`codev validate` decides a
  plan's validity); verified by the compare verdict, pasted in the pull
  request; a validity drop is explained case by case before merging.
