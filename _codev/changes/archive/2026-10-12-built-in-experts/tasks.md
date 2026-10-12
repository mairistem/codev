# Tasks

Scenarios of the `skills` capability describe what an agent does when it
runs a skill; this repository cannot run an agent in its test suite, so
they are covered by tests asserting that the skill body carries the
instruction, as in `declarative-roles` and `newcomer-readability-review`. The scenarios that a MODIFIED requirement
keeps unchanged stay covered by the tests already named for them in
`declarative-roles` (for example `a_change_overrides_the_project_activation`,
`roles_at_plan_write_are_the_artifact_roles`, `roles_rejects_an_unknown_step`,
`propose_body_carries_the_review_steps`); a task below updates those tests
when the new catalog changes their expectations.

## 1. Core: role files, steps, knowledge

- [x] 1.1 Replace `assets/roles.yaml` by `assets/roles/<id>.md` for the six step roles (criteria verbatim, `analyst` `required: true`, knowledge bodies for `newcomer` and `skeptic`), `parse_role_file` and `Role.steps` / `Role.knowledge` in `codev-core::roles`, verified by new unit tests `a_role_file_splits_frontmatter_and_knowledge`, `a_role_file_without_frontmatter_is_refused`, `at_accepts_one_step_or_a_list`, the updated `the_builtin_catalog_has_the_step_roles_and_the_experts_at_their_steps` and `step_role_criteria_carry_the_review_grids`, and `default_catalog_lists_sixteen_roles_in_workflow_order` ("The default catalog"), `a_schema_without_roles_adds_only_built_in_roles` ("A custom schema without roles")
- [x] 1.2 The six experts `assets/roles/{security,privacy,accessibility,performance,ops,migration}.md` (names, both steps, domains, question, criteria, knowledge in the five sections), verified by `knowledge_uses_only_the_five_sections_in_order` and `every_expert_has_the_five_sections_and_its_references` ("An expert's knowledge"), and a review of every cited identifier against its standard
- [x] 1.3 `codev-core::domains` (`DOMAINS`, `parse`, codes) with `parser::section` shared with `release_note`, verified by unit tests `ticked_and_plain_bullets_list_their_domain` ("Ticked domains"), `none_and_unticked_lists_mean_no_domain` ("No domain"), `an_unknown_domain_is_an_error_with_its_line` ("Unknown domain"), `none_with_a_ticked_domain_is_invalid` ("`None` with a ticked domain"), `a_stray_line_is_invalid`, `a_missing_section_is_absent`, the existing `release_note` tests, and `finding_codes_are_unique` extended to the domain codes
- [x] 1.4 `on_domain` activation, `ActivatedBy`, `resolve` taking the change's domains, analyst required, in `codev-core::roles`, verified by new tests `experts_are_active_only_for_their_ticked_domain` ("A ticked domain activates its expert"), `on_domain_roles_are_inactive_without_domains` ("Without a change, experts stay inactive", "A proposal that says `None`"), `a_change_forces_an_expert` ("A change forces an expert"), `a_project_turns_an_expert_off` ("A project turns an expert off"), `on_domain_on_a_role_without_domains_is_reported` ("`on_domain` on a role without domains"), the updated `an_activation_other_than_always_never_or_on_domain_is_reported` ("An invalid activation") and `artifact_roles_the_analyst_and_the_developer_cannot_be_disabled` ("Disabling the analyst"), `activated_by_names_the_origin_of_an_always`

## 2. Engine: domains of a change, validation

- [x] 2.1 `codev-engine::roles::resolve` reads the change's proposal domains, verified by new tests in `codev-engine::roles` `a_ticked_domain_activates_its_expert_for_the_change`, `experts_stay_inactive_without_a_change_or_a_proposal`
- [x] 2.2 Domain findings in `codev validate` (active changes only, proposal read once with the release note), verified by new tests in `codev-engine::validate` `validate_accepts_ticked_domains_and_none`, `validate_reports_an_unknown_domain_as_an_error`, `validate_reports_none_with_a_ticked_domain`, `validate_reports_a_missing_domains_section_as_a_warning` ("Missing section"), `validate_ignores_archived_changes_without_domains` ("Archived changes are not checked")

## 3. CLI: `codev roles`, doctor

- [x] 3.1 `RoleV1` gains `steps`, `domains`, `activatedBy`, `knowledge`; `at` is the asked step or the first; `--knowledge` in the human output, verified by new or updated tests in `codev-cli::commands` `roles_at_a_step_as_json` ("The roles of a step, as JSON"), `roles_without_at_lists_every_role` ("The default catalog", "Knowledge is not served to an inactive role"), `roles_serve_the_experts_of_the_ticked_domains` ("A ticked domain activates its expert", "An expert's knowledge"), `roles_without_a_change_serve_no_expert`, `roles_refuse_to_disable_the_analyst` ("Disabling the analyst"), in `codev-cli::contract` `roles_document_exposes_the_contract_fields`, and in `codev-cli::render` `roles_lists_every_role_with_its_activation` ("Human output of every role") and `roles_print_knowledge_only_on_request` ("Knowledge in the human output")
- [x] 3.2 Doctor `config.roles` names the `on_domain` roles with their domains, verified by tests in `codev-core::doctor` `roles_check_names_the_roles_active_by_domain` ("The experts in the doctor report") and in `codev-cli::commands` `doctor_lists_an_expert_forced_by_the_project` ("An expert forced by the project"), plus the updated `doctor_lists_the_roles_in_effect`

## 4. Schema and skills

- [x] 4.1 `## Domains` in the proposal template (six domains unticked, between Impact and Release note), the `proposal` instruction and a `product` criterion, verified by the new test `proposal_template_and_instruction_carry_the_domains` in `codev-engine::instructions` ("The proposal lists the domains it touches") and the existing `proposal_template_and_instruction_carry_the_release_note`
- [x] 4.2 `propose.md` step 7 and `apply.md` step 5: every active role with its knowledge, caps of five across roles, role-id tags, the severity exception, the expert's behavior rule, "Review notes"; verified by new tests in `codev-agents::workflows` `propose_challenge_applies_every_active_role_with_its_knowledge` ("An active expert challenges the design", "At most five points"), `apply_review_applies_the_newcomer_and_the_experts` ("At most five review notes", "A high-severity security finding is never dropped", "An expert reports a behavior-changing fix", "An expert fixes a clear defect within scope"), the updated `apply_summary_caps_review_notes_and_never_invents_them` ("Nothing to note", "Code outside the change is untouched"), `propose_body_carries_the_review_steps` ("Points to challenge are reported", "Nothing to challenge", "A conditional lens outside the Impact is not applied", "The pass does not add scope", "A project criterion joins the grid"), `apply_newcomer_review_keeps_behavior_scope_and_outside_code` ("A needlessly clever construction is simplified", "The review keeps behavior and scope"), `skills_skip_a_disabled_step_and_say_so` ("The review disabled by configuration", "The skeptic disabled by configuration"), and the unchanged `allowed_tools` tests

## 5. Documentation

- [x] 5.1 `roles.md` in `docs/en` and `docs/fr`: experts in the built-in roles table with their domain, steps column, "Experts and domains" section (domains, `## Domains`, `on_domain`, knowledge sections), analyst not disableable, Review notes; verified by the extended test `roles_chapters_list_exactly_the_catalog` in `codev-cli::docs` ("Every expert is documented with its domain")
- [x] 5.2 `reference/configuration.md` (`on_domain`, analyst), `reference/file-formats.md` (proposal `## Domains`), `reference/cli.md` (`codev roles` help copied from the real binary), `reference/json-output.md` (new role fields, domain codes), `skills.md` and `workflow.md` (Review notes, experts), `quickstart.md` if it shows a proposal, and the landing page's challenged-plan card, in both languages, verified by `both_languages_have_the_same_chapters` and an mdbook build of both books without warnings

## 6. Verification

- [x] 6.1 Regenerate codev's own skills with `codev update --force`, verified by a diff of `.claude/skills/codev-propose/SKILL.md` and `.claude/skills/codev-apply/SKILL.md` showing the new step 7 and step 5
- [x] 6.2 Whole workspace, verified by `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --locked -- -D warnings`, `cargo test --workspace --locked`, `codev validate --strict` and `codev doctor --strict` reporting nothing to fix
