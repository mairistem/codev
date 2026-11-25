# Tasks

Every task's definition of done also includes `cargo fmt --all --check`,
`cargo clippy --workspace --all-targets -- -D warnings` and
`cargo test --workspace`, all passing. Tests follow the project's
conventions: unit tests in `#[cfg(test)] mod tests` next to the code,
behavior-describing names, in-memory `FileSystem`, `FixedEnv` and
`MockProcessRunner` in engine and CLI tests, no assumption about path
separators or line endings.

## 1. Release note in the schema and the propose workflow

- [x] 1.1 Add the `## Release note` section to
  `assets/schemas/spec-driven/templates/proposal.md`, describe it in the
  proposal instruction of `schema.yaml` and add the product owner's "Done
  when" criterion, verified by `codev-engine` tests
  `spec_driven_instructions_start_with_a_role_block` (criterion mentions the
  release note) and `proposal_template_and_instruction_carry_the_release_note`
  (skills: "The proposal instruction starts with the product owner role").
- [x] 1.2 Add the release note bullet to step 6 of
  `assets/workflows/propose.md`, verified by the `codev-agents` test
  `propose_traceability_checks_the_release_note` (skills: "A breaking change
  missing from the release note is added", "A complete plan is left
  unchanged").

## 2. Pure core: release note parser and changelog planning

- [x] 2.1 `codev_core::changelog::release_note`: `parse` (sections,
  wrapping, `None`, BREAKING, HTML comments, fences, findings
  `release_note_invalid` with lines) and `what_changes`, verified by unit
  tests `parses_wrapped_and_breaking_items`, `none_is_an_internal_change`,
  `absent_section_is_absent`, `unknown_category_is_invalid_with_its_line`,
  `empty_bullet_is_invalid`, `empty_section_is_invalid`,
  `template_comment_is_ignored`, `what_changes_bullets_are_joined`.
- [x] 2.2 Range helpers: version parsing and `default_since`, verified by
  `default_since_is_the_highest_version_tag`,
  `default_since_is_below_a_version_until`,
  `default_since_ignores_non_version_tags` and
  `no_tag_means_no_since` (changelog: "Default range since the latest tag",
  "Range between two tags", "No tag yet").
- [x] 2.3 `collect` and `render`: omission of `None`, exclusion, fallback,
  inconsistency, ordering, keep-a-changelog / markdown headings, verified by
  `none_and_excluded_changes_contribute_nothing`,
  `fallback_uses_what_changes_and_warns`,
  `removed_requirements_without_removed_item_warn`,
  `breaking_first_then_archive_order_by_category`,
  `release_heading_carries_version_and_date`,
  `markdown_has_no_version_heading`.
- [x] 2.4 `CHANGELOG.md` splicing in `codev_core::changelog` on new
  `managed` helpers, verified by `write_creates_a_missing_file_with_a_header`,
  `write_adopts_a_hand_written_unreleased_heading`,
  `write_keeps_released_sections_and_link_references`,
  `write_is_idempotent`, `release_rotates_the_managed_section`,
  `write_refuses_malformed_markers` (changelog: "Created when missing",
  "Hand-written content is kept", "Idempotent", "Release", "Malformed
  markers").

## 3. Validation of active changes

- [x] 3.1 Report `release_note_missing` and `release_note_invalid` for active
  changes in `codev-engine::validate`, register the codes in the uniqueness
  test, and give existing test proposals a release note, verified by
  `validate_reports_a_missing_release_note_as_a_warning`,
  `validate_accepts_a_valid_or_internal_release_note`,
  `validate_reports_an_unknown_category_and_an_empty_bullet` and
  `validate_ignores_archived_changes_without_release_note` (validation: all
  six scenarios), and the CLI test that `--strict` fails on the warning.

## 4. Engine: gathering the changelog

- [x] 4.1 `changelog.exclude` in `ProjectConfig`, verified by the engine
  config test `changelog_exclude_is_read_and_unknown_keys_still_fail`.
- [x] 4.2 `codev_engine::changelog::gather` (git detection, `ls-tree`,
  `git tag`, working tree, deltas with REMOVED requirements, unreadable
  changes), verified by `gather_uses_the_latest_tag_by_default`,
  `gather_between_two_tags`, `gather_without_tag_lists_everything`,
  `gather_outside_git_lists_everything_with_a_warning`,
  `gather_outside_git_refuses_explicit_refs`,
  `gather_unknown_ref_fails`, `gather_skips_a_change_missing_from_the_tree`,
  `gather_flags_removed_requirements`.

## 5. CLI: `codev changelog` and the JSON contract

- [x] 5.1 `commands::changelog` (range, formats, `--release`, `--date`,
  `--write` through a plan) and the `Changelog` subcommand, verified by
  CLI tests `changelog_lists_the_changes_since_the_latest_tag`,
  `changelog_reports_fallback_and_inconsistency_warnings`,
  `changelog_release_heading`, `changelog_rejects_an_invalid_version_or_date`,
  `changelog_write_creates_then_is_idempotent`,
  `changelog_write_release_rotates`, `changelog_write_refuses_markdown`,
  `changelog_write_refuses_malformed_markers_without_writing`.
- [x] 5.2 `ChangelogV1` in `contract.rs` and the human rendering, verified by
  `changelog_json_carries_entries_and_status` and
  `changelog_json_failure_keeps_the_shape` (changelog: "JSON").

## 6. Doctor

- [x] 6.1 `changelog` check in `codev_core::doctor` (facts, codes, order,
  warning only) and its gathering in `commands::doctor_at`, verified by core
  tests `changelog_check_is_a_warning_when_missing_or_outdated` and the
  updated `summary_counts_missing_and_warnings`, and CLI tests
  `doctor_reports_a_missing_changelog`,
  `doctor_changelog_is_ok_after_write`,
  `doctor_changelog_is_outdated_after_a_new_archive` and the updated
  `doctor_writes_nothing` (doctor: "A freshly initialized project",
  "CHANGELOG.md up to date", "CHANGELOG.md behind the archive", "Doctor
  writes nothing").

## 7. codev's own repository

- [x] 7.1 `.github/workflows/release.yml`: full-depth checkout, run the
  Linux build's binary, `body_path` plus `generate_release_notes`, verified
  by the CLI test `release_workflow_generates_the_body_with_codev_changelog`
  (distribution: both scenarios; publishing itself cannot run in CI
  without a tag, hence a static check of the workflow).
- [x] 7.2 `changelog.exclude` in `_codev/config.yaml` for the five changes
  described by hand since `v0.4.0`, and `codev changelog --write`, which
  adopts the hand-written `## [Unreleased]` heading; run it again right
  after archiving this change so the managed section holds its release note
  (the `[Unreleased]` Added lines), verified by `codev changelog` printing
  only this change's items and `codev doctor` reporting `changelog` ok.
- [x] 7.3 CONTRIBUTING.md and CONTRIBUTING.fr.md release process:
  `codev changelog --release X.Y.Z --write` before tagging, verified by
  reading both files.

## 8. Documentation (English and French)

- [x] 8.1 `reference/cli.md` (help blocks regenerated from the binary),
  `reference/configuration.md` (`changelog.exclude`),
  `reference/file-formats.md` (Release note), `reference/json-output.md`
  (`ChangelogV1`, new codes, doctor check), `skills.md` (propose),
  `guides/repository-integration.md` (doctor check), and a new
  `guides/release-notes.md` in both `SUMMARY.md`, verified by building both
  books with mdbook without warnings and by the `codev-cli` docs tests.

## 9. Finish

- [x] 9.1 Regenerate the skills with `cargo run -q --bin codev -- update
  --force`; `codev validate --strict` and `codev doctor` clean; fmt, clippy
  `-D warnings --locked`, `cargo test --workspace --locked` green.
- [x] 9.2 Manual end-to-end check in a scratch git repository with two tags
  and archived changes with and without release notes (default range,
  `--until`, `--write`, `--release`), complementing the in-memory tests
  with the real `git`.
