# Tasks

Every task's definition of done includes `cargo fmt --all --check`,
`cargo clippy --workspace --all-targets -- -D warnings` and
`cargo test --workspace`, all passing. Tests use the in-memory
`FileSystem` and `FixedEnv` ports, in `#[cfg(test)] mod tests` next to the
code, named after the behavior.

## 1. A shared message for unknown keys

- [x] 1.1 Generalize `unknown_key_message` in `crates/codev-engine/src/config.rs` to take the file and the key, keeping the `config.yaml` wording byte for byte, so that every unknown-key warning names the key, the file, quotes `codev_version` when set and points to `codev self-update`; verified by the existing `unknown_config_key` tests passing unchanged and a new `unknown_key_message_names_the_file_and_the_requirement`.

## 2. Role settings warn

- [x] 2.1 Drop `deny_unknown_fields` from `RoleSettingMapping` in `crates/codev-core/src/roles.rs`; collect the unknown keys of each project role's mapping in `config.rs` as `UnknownKey { key: "roles.<id>.<key>", line }` so they flow into `unknown_config_key` warnings, the validate `config` item and doctor's `config.keys`; replace the tests `an_unknown_key_inside_a_role_is_refused` (`roles.rs`) and `an_unknown_key_inside_a_role_fails_the_configuration` (`config.rs`); verified by `unknown_key_inside_a_role_warns_and_keeps_the_model` (roles spec, scenario "An unknown key inside a role"), `misspelled_role_key_fails_strict_validation_at_its_line` (scenario "A misspelled key inside a role fails strict validation") and `doctor_reports_an_unknown_role_key_under_config_keys` (doctor spec, scenario "An unknown key in a role's mapping").
- [x] 2.2 Check that an inherited source's `roles:` is still not parsed nor reported; verified by the existing test `a_source_s_unknown_role_key_keeps_its_rules` (`crates/codev-engine/src/config.rs`) passing.

## 3. `change.yaml` warns

- [x] 3.1 Drop `deny_unknown_fields` from `ChangeMetadata` in `crates/codev-engine/src/metadata.rs` and the strict check of `review:` keys in `load`; return the unknown keys (top level and `review.<key>`, with lines) beside the metadata; keep `review.mode` and role activation values strict; replace the test `rejects_an_unknown_key` of `metadata.rs`; verified by `unknown_change_key_is_ignored_and_reported` and `unknown_review_key_in_change_yaml_keeps_the_mode`.
- [x] 3.2 Report them as `change_unknown_key` warnings in the `status` array of the commands that load a change, and as `warning` findings on the change's item in `codev validate`; verified by the CLI tests `status_warns_on_an_unknown_change_key` (changes spec, "An unknown top-level key"), `roles_warns_on_an_unknown_review_key_of_a_change` ("An unknown key under `review`"), `validate_strict_fails_on_a_misspelled_change_key` ("Strict validation catches a misspelled key") and `validate_passes_with_an_unknown_change_key` ("Non-strict validation with an unknown key"), and by the review-mode scenarios still passing.

## 4. Project schemas warn

- [x] 4.1 Make `Schema::parse` in `crates/codev-core/src/schema.rs` tolerate unknown keys at the top level, in an artifact, in `apply` and in an artifact's `role:` (`ArtifactRole` in `roles.rs`), returning them with their lines; keep `schema_unreadable` for invalid YAML and missing mandatory fields; add `built_in_schema_has_no_unknown_key`; replace the tests `rejects_an_unknown_key` and `rejects_an_unknown_role_key` of `schema.rs`; verified by `unknown_schema_key_is_ignored_with_its_line`, `unknown_artifact_role_key_is_ignored` and `schema_that_is_not_yaml_is_unreadable` (schemas spec, "A schema that is not YAML").
- [x] 4.2 Report them as `schema_unknown_key` warnings in the `status` array of the commands that resolve a project schema, list the schema as usable in `codev schemas`, and add them to the validate `config` item located in the schema's file; verified by `status_warns_on_an_unknown_schema_key` ("An unknown key in a project schema"), `roles_warns_on_an_unknown_artifact_role_key` ("An unknown key in an artifact role"), `validate_strict_fails_on_a_misspelled_schema_key` ("A misspelled key in a project schema fails strict validation") and `no_config_item_when_config_and_schemas_are_clean` (configuration spec, "Clean configuration").

## 5. ADR frontmatter and seal file

- [x] 5.1 Drop `deny_unknown_fields` from `RawFrontmatter` in `crates/codev-core/src/decisions/parser.rs`; keep the unknown fields, as written and in order, on the parsed decision, and emit a `decision_unknown_field` warning at each one's line; replace `unknown_key_in_frontmatter_is_reported`; report nothing for an inherited decision; verified by `unknown_frontmatter_field_warns_and_the_decision_is_read` the CLI test `validate_reports_an_unknown_adr_field_and_strict_fails`, and `inherited_adr_unknown_field_is_not_reported` ("An inherited decision's unknown field is not reported") (decisions spec, "A frontmatter field this codev does not know", "Strict validation catches a misspelled field").
- [x] 5.2 Make `rewrite_frontmatter_status` in `crates/codev-engine/src/decisions_actions.rs` append the unknown fields after the known ones, so `codev decision accept` keeps them on the accepted decision and on each predecessor; verified by `accept_keeps_unknown_frontmatter_fields` ("Fields codev does not know survive the acceptance") and the existing seal tests passing (body untouched).
- [x] 5.3 Give `seal_version_unsupported` a message naming the version found, the version read, `codev_version` when set and `codev self-update`, and make `codev validate` report that code instead of `unreadable`; verified by `validate_refuses_a_newer_seal_file` ("A seal file from a newer codev"), `accept_writes_nothing_against_a_newer_seal_file` ("Nothing is sealed against a newer seal file"), `seal_and_new_refuse_a_newer_seal_file` ("The other sealing commands refuse a newer seal file") and `accept_refuses_an_unknown_seal_field` ("An unknown field in a seal file of a known version").

## 6. Verdict commands refuse a codev outside `codev_version`

- [x] 6.1 Extract the fix computation of doctor's `codev_version` check (`crates/codev-core/src/doctor.rs`) into a function both doctor and the CLI call; verified by the existing doctor `codev_version` scenarios passing unchanged.
- [x] 6.2 In `crates/codev-cli/src/commands.rs`, make `validate`, `sync` and `archive` fail with `codev_version_unsatisfied` right after the configuration is read when the running codev does not satisfy `codev_version`, before any check or write, keeping their JSON document's shape; verified by the CLI tests `validate_refuses_below_the_requirement`, `archive_refuses_and_moves_nothing_below_the_requirement`, `sync_refuses_and_leaves_specs_below_the_requirement`, `validate_refusal_names_the_version_to_install_above_the_requirement`, `status_still_works_below_the_requirement` and `validate_runs_without_a_requirement` (self-update spec, the six scenarios of "The commands that give a verdict refuse a codev outside `codev_version`").

## 7. Documentation

- [x] 7.1 Update `docs/en/src/reference/configuration.md` and `docs/fr/src/reference/configuration.md` (vouvoiement): the "Unknown keys" table (a `roles` entry becomes a warning `roles.<id>.<key>`), the other hand-written files (`change.yaml`, project `schema.yaml`, ADR frontmatter) and their codes, and in `codev_version` the refusal of `validate`, `sync` and `archive`, with the advice to raise the CI pin together with it and to widen an upper bound before upgrading past it; verified by both editions having the same sections and `mdbook build docs/en` and `mdbook build docs/fr` succeeding.
- [x] 7.2 Update `docs/en/src/faq.md` and `docs/fr/src/faq.md` (which versions refuse which files; entries for `change_unknown_key`, `schema_unknown_key`, `decision_unknown_field`, `codev_version_unsatisfied` from `validate`/`sync`/`archive`), `reference/file-formats.md` (unknown keys of `change.yaml`, `schema.yaml`, ADR frontmatter; `seal.yaml` versioning) and `reference/cli.md` (the refusal) and `skills.md` ("Update notice": after **Later** on an unsatisfied requirement, `/codev-sync` and `/codev-archive` stop at the refusal, which gives the fix) in both editions; verified by the same `mdbook` builds.
- [x] 7.3 Update `reference/json-output.md` (`schema_unreadable` no longer covers an unknown key; the `config` item also carries `schema_unknown_key`; the codes `change_unknown_key`, `schema_unknown_key`, `decision_unknown_field`; `codev_version_unsatisfied` failing `validate`, `sync` and `archive`) and `guides/custom-schemas.md` (an unknown key warns and fails `--strict`, no longer rejected) in both editions; verified by the same `mdbook` builds and `grep -n "unknown key" docs/*/src/guides/custom-schemas.md` showing the new wording.
- [x] 7.4 Add to `CONTRIBUTING.md` and `CONTRIBUTING.fr.md`, in "Release process", a pointer to decision 0013: a change adding a key, a value or a format states in its design what an older codev does and gives the minimum `codev_version` in its release note; verified by both files carrying the paragraph.

## 8. Verification

- [x] 8.1 Run `codev validate --strict` on this repository with the built binary: it passes, and this repository's own files (no unknown key) produce no new warning.
