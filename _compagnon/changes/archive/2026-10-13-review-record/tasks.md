# Tasks

Scenarios of the `skills` capability describe what an agent does when it
runs a skill; this repository cannot run an agent in its test suite, so
they are covered by tests asserting that the rendered skill body carries
the instruction, as in `built-in-experts`. Scenarios a MODIFIED
requirement keeps unchanged stay covered by the tests already named for
them (`propose_body_carries_the_review_steps`,
`propose_challenge_applies_every_active_role_with_its_knowledge`,
`apply_review_applies_the_newcomer_and_the_experts`,
`apply_newcomer_review_keeps_behavior_scope_and_outside_code`,
`apply_summary_caps_review_notes_and_never_invents_them`,
`skills_skip_a_disabled_step_and_say_so`).

## 1. Core: scales, record, rules, evidence, gate

- [x] 1.1 `codev-core::review`: `Severity`, `Confidence`, `Status` with their definitions, the codes module, and `parse` (layout, finding blocks, fields, ids, locations), verified by unit tests `a_well_formed_record_parses_steps_roles_and_findings`, `an_unknown_severity_is_invalid` ("An unknown severity"), `a_role_with_nothing_in_it_is_invalid` ("A role section with nothing in it"), `a_block_that_is_not_yaml_is_invalid` ("A finding block that is not YAML"), `an_unknown_step_is_a_warning` ("A misspelled step"), `a_crlf_record_parses_like_lf` ("Windows line endings"), `a_finding_without_evidence_is_invalid` ("A missing evidence"), `an_id_must_match_its_role` ("An id that does not match its role"), `a_duplicate_id_is_invalid` ("Two findings with the same id"), `a_location_outside_the_project_is_invalid` ("A location outside the project"), `both_kinds_of_location_parse` ("Both kinds of location"), `backslashes_in_a_location_read_as_slashes`, `an_unknown_field_is_a_warning` ("A field from a newer codev"), `a_newer_format_is_not_checked` ("A record from a newer codev"), and `finding_codes_are_unique` extended to the review codes
- [x] 1.2 History rules and `awaiting` in `codev-core::review`, verified by unit tests `an_accepted_risk_needs_a_human` ("An accepted risk decided by the author"), `a_fix_needs_a_reference` ("A fix without a reference"), `a_high_finding_is_deferred_by_a_human` ("A high finding deferred by the author"), `only_the_reviewer_or_a_human_rejects` ("The author rejects a finding"), `a_complete_loop_is_valid` ("A complete loop"), `a_history_must_start_open_by_its_role`, `a_transition_outside_the_table_is_invalid`, `awaiting_follows_the_last_answer` ("A maintained dispute is for the human")
- [x] 1.3 Evidence matcher and plan sections in `codev-core::review`, verified by unit tests `evidence_matches_with_whitespace_normalized` ("Evidence at the cited lines"), `evidence_within_five_lines_matches` ("Evidence a few lines away"), `evidence_further_away_is_moved` ("Evidence that moved further"), `evidence_found_nowhere_is_not_found`, `evidence_ignores_crlf`, `a_plan_section_ends_at_the_next_heading_of_its_level`
- [x] 1.4 Completeness and gate in `codev-core::review`, verified by unit tests `an_active_role_without_a_section_is_missing`, `a_disabled_section_for_an_active_role_is_missing`, `gate_requires_a_record_and_a_step_only_where_roles_are_active`, `gate_blocks_an_open_or_disputed_high_finding`, `gate_lets_an_accepted_risk_through`, `gate_refuses_a_newer_format`

## 2. Engine: reading the record, validation, archive

- [x] 2.1 `codev-engine::review::check` (reads the record and each cited file once, size cap, roles resolved for the change), verified by tests in `codev-engine::review` `a_well_formed_record_has_no_finding` ("A well-formed record", "Windows line endings"), `invented_evidence_is_an_error` ("Invented evidence"), `a_line_beyond_the_end_is_missing` ("A line beyond the end of the file"), `a_plan_finding_is_found_in_its_section` ("A plan finding in its section"), `a_binary_file_is_unchecked` ("A binary file"), `a_file_over_the_size_cap_is_not_compared`, `a_fixed_finding_is_not_checked` ("A fixed finding whose code is gone"), `a_location_outside_the_project_is_not_read`, `an_active_expert_without_a_section_is_missing` ("An active expert without a section"), `a_disabled_role_may_say_so` ("A role disabled by configuration")
- [x] 2.2 `validate_change` adds the record's findings, verified by tests in `codev-engine::validate` `validate_reports_nothing_without_a_record` ("A change without a record"), `an_open_high_finding_is_not_a_validation_error` ("An open high finding is not a validation error"), `a_record_error_is_located_in_review_md` ("A record error fails validation"), `archived_records_are_not_checked` ("Archived records are not checked")
- [x] 2.3 The archive gate and `EngineError::ReviewBlocked` (`review_blocked`), after the validation pre-flight, verified by tests in `codev-engine::archive` `archive_refuses_an_open_high_finding` ("A high finding still open"), `archive_accepts_an_accepted_risk` ("An accepted risk lets the change go", "The archive keeps the record"), `archive_refuses_without_a_record` ("No record for a reviewed step"), `archive_without_active_review_roles_needs_no_record` ("Every review role turned off"), `archive_refuses_a_newer_format`; in `codev-engine::sync` `sync_ignores_the_review_record` ("Sync does not look at the record"); in `codev-engine::change` `status_ignores_the_review_record` ("Status ignores the record"); in `codev-engine::changelog` `changelog_ignores_the_review_record` ("The changelog ignores the record"); and the existing archive tests updated with a record
- [x] 2.4 Existing tests that archive a change (engine, CLI, decisions) get a settled record or review roles off, verified by `cargo test --workspace --locked`

## 3. CLI: `codev review`, contract, archive refusal

- [x] 3.1 `codev review [--change] [--json]`, `ReviewV1` in `contract`, human rendering, exit code 1 on a record error, verified by tests in `codev-cli::commands` `review_json_lists_findings_awaiting_and_reported` ("A maintained dispute is for the human", "A low-confidence finding is not reported"), `review_without_a_record_succeeds` ("No record yet"), `review_exits_1_on_a_record_error` ("A record with an error"), `archive_json_refusal_carries_review_blocked`; in `codev-cli::contract` `review_document_exposes_the_contract_fields`; in `codev-cli::render` `review_prints_steps_roles_findings_and_the_gate`

## 4. Skills

- [x] 4.1 `assets/workflows/_review-record.md` substituted for `{{REVIEW_RECORD}}` in `propose.md` step 7 and `apply.md` step 5, `{{REVIEW_SCALE}}` generated from the core; Points to challenge, Review notes and "To decide" from the record; `archive.md` handles `review_blocked`; verified by new tests in `codev-agents` `propose_and_apply_carry_the_review_scales` ("The scales reach the skills"), `propose_and_apply_describe_the_record_and_the_loop` ("The skills describe the record and the loop", "A maintained dispute goes to the human", "Points to challenge are reported", "At most five points", "An active expert challenges the design", "Nothing to challenge", "At most five review notes", "An expert reports a behavior-changing fix", "An expert fixes a clear defect within scope"), `skills_never_record_a_human_decision_alone` ("The agent never accepts a risk"), `skills_resume_from_the_record` ("A resumed session"), `archive_explains_review_blocked` ("Archive blocked by the review"), and the existing workflow tests updated
- [x] 4.2 Regenerate codev's own skills with `codev update --force`, verified by the diff of `.claude/skills/codev-propose/SKILL.md`, `codev-apply/SKILL.md` and `codev-archive/SKILL.md` showing the record procedure with the scales substituted

## 5. Documentation

- [x] 5.1 `roles.md` (severity, confidence, the record, the loop) and a new `guides/review-record.md` listed in `SUMMARY.md`, in `docs/en` and `docs/fr`, verified by new tests in `codev-cli::docs` `both_languages_document_the_review_record` ("The guide exists in both languages") and `roles_chapters_carry_the_review_scales` ("The scale in the docs is the scale in the skills")
- [x] 5.2 `reference/file-formats.md` (`review.md`), `reference/cli.md` (`codev review`, help copied from the binary; archive's refusal), `reference/json-output.md` (`ReviewV1`, the review codes, `review_blocked`), `skills.md` and `workflow.md` (the record in the challenge, the review and archiving), in both languages, verified by `both_languages_have_the_same_chapters` and an mdbook build of both books without warnings

## 6. Verification

- [x] 6.1 Whole workspace, verified by `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --locked -- -D warnings`, `cargo test --workspace --locked`, `codev validate --strict` and `codev doctor --strict` reporting nothing to fix
