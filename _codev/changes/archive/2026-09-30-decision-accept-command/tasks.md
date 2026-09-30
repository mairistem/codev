# Tasks

## 1. Engine

- [x] 1.1 `plan_accept` rewrites the status and seals the body in one plan, verified by `plan_accept_rewrites_the_status_and_seals_in_one_plan` and `plan_accept_keeps_the_body_byte_for_byte`
- [x] 1.2 Refusals `cannot_accept_inherited`, `decision_not_proposed` and `unknown_decision_id`, verified by `plan_accept_refuses_an_inherited_decision`, `plan_accept_refuses_an_accepted_decision_and_points_to_supersede`, `plan_accept_refuses_a_rejected_decision`, `plan_accept_refuses_an_unknown_id` and `plan_accept_refuses_a_leftover_seal_for_another_body`

## 2. CLI

- [x] 2.1 `decision new` defaults to `proposed`, verified by `decision_new_status_defaults_to_proposed`, `decision_new_proposed_neither_seals_nor_exposes_a_hash` and `decision_new_accepted_seals_the_entry_and_exposes_the_hash`
- [x] 2.2 `codev decision accept <id> [--json]` with human and JSON output, verified by `decision_accept_seals_a_proposed_decision`, `decision_accept_then_validate_is_clean`, `decision_accept_refuses_an_accepted_decision`, `decision_accept_refuses_an_inherited_decision`, `decision_accept_writes_nothing_when_the_seal_file_is_invalid` and `accepted_report_exposes_the_expected_fields_in_camel_case`
- [x] 2.3 Help of `decision new --status` and `codev decision --help` list the default and `accept`, verified by reading `codev decision --help`

## 3. Skills and documentation

- [x] 3.1 Check the skills in `assets/workflows/` for `decision new`, verified by a grep that finds none relying on the default
- [x] 3.2 CLI reference, JSON output, concepts and FAQ in `docs/en` and `docs/fr` with outputs from real runs, verified by `both_languages_have_the_same_chapters` and the docs anchor tests
- [x] 3.3 CHANGELOG entries under `[Unreleased]`, verified by reading `CHANGELOG.md`

## 4. Verification

- [x] 4.1 Whole workspace, verified by `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace` and `codev validate --strict`
