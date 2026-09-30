# Tasks

## 1. Engine

- [x] 1.1 `plan_supersede` creates a `proposed`, unsealed ADR, leaves the predecessor untouched and refuses a target that is not `accepted` with `predecessor_not_accepted`, verified by `plan_supersede_creates_a_proposed_adr_and_leaves_the_predecessor_untouched`, `plan_supersede_writes_no_seal`, `plan_supersede_refuses_a_predecessor_that_is_not_accepted`, `supersede_unknown_id_is_refused` and `supersede_inherited_source_is_refused`
- [x] 1.2 `plan_deviate` and `plan_promote` create `proposed`, unsealed ADRs, verified by `plan_deviate_creates_a_proposed_adr_without_seal`, `plan_deviate_renders_frontmatter_with_deviates_from`, `plan_promote_produces_adr_and_references_design` and `plan_promote_creates_a_proposed_unsealed_adr`
- [x] 1.3 `plan_accept` marks the predecessors `superseded` in the same plan and refuses a predecessor that cannot be superseded, verified by `plan_accept_marks_the_predecessor_superseded_in_the_same_plan`, `plan_accept_keeps_the_predecessor_body_byte_for_byte`, `plan_accept_refuses_a_predecessor_that_is_no_longer_accepted`, `plan_accept_refuses_an_inherited_predecessor` and `plan_accept_refuses_an_unknown_predecessor`
- [x] 1.4 The index ignores the relations of a `proposed` ADR, verified by `proposed_supersession_has_no_effect_on_the_index`, `deviation_by_proposed_adr_has_no_effect` and `proposed_deviations_on_the_same_target_do_not_conflict`

## 2. CLI

- [x] 2.1 `decision supersede`, `deviate` and `promote` report a `proposed` decision and name `codev decision accept NNNN` as the next step, verified by `decision_supersede_creates_a_proposed_decision_and_keeps_the_old_one_in_effect`, `decision_supersede_refuses_a_decision_that_is_not_accepted`, `decision_deviate_creates_a_proposed_adr_without_seal`, `decision_promote_creates_a_proposed_adr_and_references_the_design` and the render tests `superseded_output_names_the_accept_step`, `deviated_output_names_the_accept_step` and `promoted_output_names_the_accept_step`
- [x] 2.2 `decision accept` supersedes the predecessors and reports them, verified by `decision_supersede_then_accept_supersedes_the_old_one_and_validates_clean`, `decision_accept_refuses_a_second_supersession_of_the_same_decision`, `decision_deviate_has_no_effect_until_accepted`, `decision_list_exposes_deviated_by_on_the_inherited_one` and `decision_promote_rework_then_accept_validates_clean`
- [x] 2.3 JSON contract v1: `superseded` on `DecisionAcceptedV1`, no `bodySha256` on `DecisionDeviatedV1` and `DecisionPromotedV1`, verified by `accepted_report_exposes_the_expected_fields_in_camel_case`, `accepted_report_lists_the_superseded_predecessors` and `deviated_and_promoted_reports_carry_no_hash`
- [x] 2.4 Help of `decision supersede`, `deviate`, `promote` and `accept` describes the lifecycle, verified by reading `codev decision --help` and each subcommand's `--help`

## 3. Skills and documentation

- [x] 3.1 Check the skills in `assets/workflows/` for `supersede`, `deviate`, `promote` and the decision lifecycle, verified by a grep and `codev update`
- [x] 3.2 CLI reference, JSON output, concepts, FAQ and inherited sources guide in `docs/en` and `docs/fr`, with outputs from real runs, verified by `both_languages_have_the_same_chapters`, the docs anchor tests and an mdbook build of both books without warnings
- [x] 3.3 CHANGELOG entry under `[Unreleased]` and ROADMAP item removed, verified by reading `CHANGELOG.md` and `ROADMAP.md`

## 4. Verification

- [x] 4.1 Whole workspace, verified by `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --locked -- -D warnings`, `cargo test --workspace --locked` and `codev validate --strict`
