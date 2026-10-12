# Tasks

No production code changes: every task adds or extends tests, or updates
documentation. Every task's definition of done also includes
`cargo fmt --all --check`,
`cargo clippy --workspace --all-targets --locked -- -D warnings` and
`cargo test --workspace --locked`, all passing. Tests follow the project's
conventions: unit tests in the `#[cfg(test)] mod tests` next to the code,
behavior-describing names, the in-memory `MemoryFileSystem` and `FixedEnv`
ports (through the CLI `Harness`) in engine and CLI tests, never a temporary
directory, no assumption about path separators or line endings.

No `design.md`: the change adds no code and no architectural choice; where
each command's behavior is specified is explained in the proposal.

## 1. Changes: `new change`, `list`, `status`, choosing the change

- [x] 1.1 Cover `codev new change` (changes: "A change with a goal", "A
  blank goal is not recorded", "JSON document of a created change"),
  verified by the new tests `new_change_writes_schema_date_and_goal_only`
  and `new_change_ignores_a_blank_goal` (codev-cli commands) and
  `new_change_json_carries_the_change_and_its_files` (codev-cli contract).
- [x] 1.2 Cover the naming rules (changes: "Names accepted", "An invalid
  name is refused with the rule it breaks", "An existing name is refused"),
  verified by the existing `accepts_kebab_case` and `rejects_and_explains`
  (codev-core id), `rejects_an_invalid_change_name` extended to check that
  nothing is written, and `rejects_an_already_existing_change` extended to
  check that `change.yaml` is unchanged (codev-cli commands).
- [x] 1.3 Cover `codev list` (changes: "Active changes, archive excluded",
  "No active change", "Capabilities, nested ones included", "No capability
  yet"), verified by the existing `lists_active_changes_without_the_archive`,
  `lists_capabilities_including_nested_ones`, `ignores_what_is_not_a_spec`
  (codev-engine) and `an_empty_list_says_what_to_do_next` (codev-cli
  render), and the new tests
  `list_ignores_folders_that_are_not_change_names` (codev-engine change),
  `an_empty_spec_list_says_when_specs_appear` (codev-cli render) and
  `list_documents_carry_their_items_and_root` (codev-cli contract).
- [x] 1.4 Cover the artifact states of `codev status` (changes: "A fresh
  change has only its first artifact ready", "A written artifact unlocks the
  next ones", "A nested spec delta satisfies the glob", "`skip_specs` skips
  the spec artifact", "Writing the tracked file first does not complete
  planning", "Every artifact written", "Resolved paths for the skills"),
  verified by the existing `a_nested_spec_satisfies_the_artifact_pattern`,
  `skip_specs_makes_tasks_writable_without_specs` (codev-engine change),
  `writing_tasks_first_does_not_complete_planning`,
  `planning_is_complete_when_the_closure_is_satisfied` (codev-core status),
  `status_flags_a_disabled_artifact`, `status_says_when_planning_is_complete`
  (codev-cli render), and the new tests
  `status_of_a_fresh_change_has_only_the_proposal_ready`,
  `status_after_the_proposal_shows_specs_and_design_ready` and
  `status_resolves_the_project_root_from_a_subfolder` (codev-cli commands).
- [x] 1.5 Cover the choice of the change (changes: "The single active
  change is inferred", "No active change", "Several active changes", "A
  named change that does not exist", "A named change with an invalid
  name"), verified by the existing `the_full_cycle_leads_to_instructions`,
  `status_without_active_change_points_to_creation`,
  `status_with_several_changes_refuses_to_guess` (codev-cli commands),
  `an_unknown_change_is_an_error_that_guides` (codev-engine change), and the
  new tests `instructions_refuse_an_invalid_change_name` (codev-cli
  commands) and `status_failure_keeps_the_shape` (codev-cli main).

## 2. Instructions

- [x] 2.1 Cover the fields of `codev instructions` (instructions: "The next
  artifact to write", "A named artifact carries the schema's guidance",
  "Dependencies not written yet", "The configured language", "A project
  schema without instruction or template"), verified by the existing
  `without_a_named_artifact_returns_the_next_to_write`,
  `tells_what_the_artifact_unlocks_and_what_to_reread`,
  `carries_the_schema_template_and_instruction` (codev-engine instructions),
  and the new tests `dependencies_not_written_yet_are_not_done`,
  `instructions_carry_the_configured_language`,
  `a_project_schema_without_instruction_nor_template_gives_null`
  (codev-engine instructions) and
  `instructions_for_the_next_artifact_keep_the_glob_and_write_nothing`
  (codev-cli commands).
- [x] 2.2 Cover the context and rules blocks (instructions: "Inherited
  blocks come before the project's", "Rules of other artifacts are left
  out"), verified by the existing `injects_only_the_requested_artifacts_rules`
  and the new test `inherited_blocks_come_before_the_project_ones`
  (codev-engine instructions).
- [x] 2.3 Cover a skipped artifact (instructions: "Instructions for a
  skipped spec artifact"), verified by the existing
  `a_disabled_artifact_is_reported_and_not_to_be_created` (codev-engine
  instructions) and the new test
  `skipped_instructions_show_neither_instruction_nor_template` (codev-cli
  render).
- [x] 2.4 Cover the human output (instructions: "Human instructions for the
  specs artifact", "No empty section"), verified by the new tests
  `human_instructions_list_the_sections_in_order` (codev-cli commands) and
  `instructions_without_context_rules_or_dependencies_have_no_empty_heading`
  (codev-cli render).
- [x] 2.5 Cover the errors (instructions: "Unknown artifact", "Nothing left
  to write", "A template missing from a project schema"), verified by the
  existing `rejects_an_artifact_missing_from_the_schema` and
  `when_everything_is_written_nothing_is_ready` (codev-engine instructions),
  and the new tests `instructions_of_an_unknown_artifact_name_it_and_the_schema`
  (codev-cli commands), `instructions_failure_keeps_the_shape` (codev-cli
  main) and `a_missing_project_template_fails_with_template_not_found`
  (codev-engine instructions).

## 3. Schemas

- [x] 3.1 Cover `codev schemas` (schemas: "Only the built-in schema", "A
  project schema is listed before the built-in one", "A broken project
  schema is reported, the others listed", "A project schema replaces the
  built-in one"), verified by the existing `lists_the_schemas_with_their_flow`
  (codev-cli commands), `a_broken_custom_schema_is_reported_without_hiding_the_others`
  extended to check that the warning names `broken`, and the new tests
  `schemas_list_a_project_schema_with_its_flow_before_the_built_in_one`
  (codev-cli commands) and `a_project_spec_driven_is_listed_once_as_project`
  (codev-engine schemas).
- [x] 3.2 Cover schema resolution (schemas: "The project's `spec-driven`
  wins", "Unknown schema"), verified by the existing
  `the_project_takes_precedence_over_the_embedded_schema` and
  `an_unknown_schema_is_an_error_that_guides` (codev-engine schemas), and
  the new test `status_uses_the_project_spec_driven_schema` (codev-cli
  commands).
- [x] 3.3 Cover schema validation (schemas: "A minimal valid schema", "An
  unknown key", "Structural defects"), verified by the existing
  `parses_a_minimal_schema`, `rejects_an_unknown_key`,
  `rejects_a_phantom_dependency`, `rejects_a_duplicate_artifact`,
  `rejects_a_cycle_and_names_the_culprits` (codev-core schema) and the new
  test `rejects_an_empty_artifact_list_and_an_empty_apply_requires`
  (codev-core schema).
- [x] 3.4 Cover the schema of a change (schemas: "The configured default",
  "`--schema` wins over the configuration", "An unknown schema creates
  nothing", "Changing the default leaves existing changes alone", "A
  hand-made change folder"), verified by the existing
  `rejects_an_unknown_schema_before_creating_the_folder` (codev-cli
  commands) and `a_change_without_metadata_inherits_the_project_schema`
  (codev-engine change), and the new tests
  `new_change_takes_the_schema_from_the_flag_then_the_configuration` and
  `changing_the_configured_schema_leaves_existing_changes_alone` (codev-cli
  commands).

## 4. Decisions and skills

- [x] 4.1 Cover `codev decision show` (decisions: "Show of an id without
  collision", "Header of a local decision", "Show of an ambiguous id",
  "Show with a qualified identifier", "An inherited decision", "Unknown
  identifier"), verified by the new tests
  `decision_show_prints_the_header_then_the_file`,
  `decision_show_refuses_an_ambiguous_id_and_lists_the_candidates`,
  `decision_show_accepts_a_qualified_id`,
  `decision_show_of_an_inherited_decision_gives_its_origin_and_absolute_path`
  and `decision_show_refuses_an_unknown_id` (codev-cli commands).
- [x] 4.2 Cover `codev update` and the skills (skills: "Missing and outdated
  skills are written", "A second run changes nothing", "`--force` rewrites a
  hand-edited skill", "An unknown workflow is ignored with a warning", "A
  workflow removed from the configuration keeps its skill", "A deleted
  `_codev/` folder is recreated", "Outside a project"), verified by the
  existing `update_requires_an_existing_project` (codev-cli commands) and
  the new tests `update_creates_missing_and_rewrites_outdated_skills`,
  `a_second_update_has_nothing_to_do`,
  `update_force_rewrites_a_hand_edited_skill`,
  `update_ignores_an_unknown_workflow_with_a_warning`,
  `update_keeps_the_skill_of_a_workflow_no_longer_listed`,
  `update_recreates_a_deleted_codev_folder` (codev-cli commands) and
  `setup_failure_keeps_the_shape` (codev-cli main).

## 5. Documentation and verification

- [x] 5.1 Update the documentation in docs/en and docs/fr (same chapters,
  French in vouvoiement) where it misses what the specs state:
  reference/cli.md (error codes of `new change`, `status` and
  `instructions`, the `schema_unusable` warning of `codev schemas`, a
  description of `codev decision show`), reference/json-output.md (common
  error codes `invalid_change_id` and `unknown_artifact`, the
  `schema_unusable` warning, the path of a decision, the instruction
  example), guides/custom-schemas.md (the folder and `name` of a schema);
  verified by building both books with mdbook without warnings.
- [x] 5.2 Run `codev validate --strict` and `codev doctor --strict` on the
  repository; verified by both exiting with code 0.
