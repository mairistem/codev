# Tasks

Every task's definition of done also includes `cargo fmt --all --check`,
`cargo clippy --workspace --all-targets --locked -- -D warnings` and
`cargo test --workspace --locked`, all passing. Tests follow the project's
conventions: unit tests in `#[cfg(test)] mod tests` next to the code,
behavior-describing names, the in-memory `MemoryFileSystem` and `FixedEnv`
ports (through the CLI `Harness`) in engine and CLI tests, never a
temporary directory, no assumption about path separators or line endings.

## 1. Core: downgrade target and doctor checks

- [x] 1.1 In `codev_core::update`, add `RequirementSide` and
  `requirement_side(requirement, current)` (`Above` for a failing upper
  bound, `Below` otherwise, `None` when satisfied or unparsable), and
  `downgrade_target(requirement, latest_known)` (cached latest when it
  satisfies, else the full inclusive upper bound, else the lowest admitted
  version of a lower bound, else `None`), verified by unit tests
  `a_version_above_an_upper_bound_is_above`,
  `a_version_below_a_lower_bound_is_below`,
  `the_downgrade_target_prefers_the_cached_release`,
  `the_downgrade_target_falls_back_to_an_exact_upper_bound`,
  `the_downgrade_target_falls_back_to_the_lowest_admitted_version`,
  `no_downgrade_target_without_any_bound`.
- [x] 1.2 In `codev_core::doctor`, add `latest_known` to `VersionFacts` and
  give the `codev_version` check the fix `codev self-update --version
  <target>` (or the placeholder naming the requirement) when the running
  codev is above, `codev self-update` when below, verified by unit tests
  `codev_version_above_the_bound_fixes_with_an_explicit_version`
  (doctor: "Running codev above the project's requirement", "Running codev
  above an exact upper bound, no usable cache", "Running codev above a
  requirement with no exact bound"),
  `codev_version_above_without_any_target_names_the_requirement` and the
  existing `codev_version_unsatisfied_is_missing` (doctor: "Running codev
  below the project's requirement").
- [x] 1.3 Add the `config.keys` check after `config.language` (fact
  `unknown_config_keys`): `ok`, or `warning` `unknown_config_key` naming
  the keys, with the fix `codev self-update` when the requirement is not
  satisfied, otherwise "correct or remove"; update `CHECK_IDS` and the
  summary count, verified by unit tests `unknown_config_keys_are_a_warning`,
  `unknown_config_keys_with_an_unsatisfied_requirement_fix_with_self_update`,
  `summary_counts_missing_and_warnings` (doctor: "An unknown configuration
  key", "An unknown key with an unsatisfied requirement").

## 2. Engine: tolerant configuration loading

- [x] 2.1 In `codev_engine::config`, drop `deny_unknown_fields` from
  `ProjectConfig` and `McpConfig`, keep it on `InheritSource` and
  `ChangelogConfig`; add `PROJECT_KEYS`, `MCP_KEYS` and the detection of
  unknown keys with their line into `ProjectConfig::unknown_keys`, verified
  by unit tests `unknown_top_level_keys_are_collected_with_their_line`,
  `unknown_mcp_keys_are_collected_with_a_dotted_name`,
  `every_known_key_is_recognized`,
  `an_unknown_changelog_key_still_fails` (replacing
  `changelog_exclude_is_read_and_unknown_keys_still_fail`),
  `an_unknown_inherits_key_still_fails`, `mcp_config_unknown_field_is_tolerated`
  (replacing `mcp_config_unknown_field_is_rejected`).
- [x] 2.2 Enrich the strict parse error with the lenient `codev_version`
  (quote it, name `codev self-update`), verified by unit tests
  `a_strict_section_error_quotes_codev_version` (configuration: "Unknown
  key of the `changelog` section") and
  `a_strict_section_error_without_codev_version_names_the_key`
  (configuration: "Unknown key of an `inherits` entry").
- [x] 2.3 In `config::resolve`, turn each project unknown key into a
  `unknown_config_key` warning (key, newer codev, `codev_version` when set,
  `codev self-update`), expose `unknown_keys` on `ResolvedConfig`, ignore
  inherited sources' unknown keys, verified by unit tests
  `an_unknown_top_level_key_warns_and_quotes_codev_version`,
  `an_unknown_top_level_key_without_requirement_warns` (replacing
  `reports_an_unknown_key_in_the_config`),
  `an_inherited_source_unknown_key_is_ignored_silently` (configuration:
  "Unknown key of an inherited source").
- [x] 2.4 Add `ItemKind::Config` and the `config` item of `validate_all`,
  only when unknown keys exist, one `warning` finding per key at its line,
  verified by unit tests `validate_all_reports_unknown_config_keys` and
  `validate_all_has_no_config_item_for_a_clean_config`.

## 3. CLI: surfacing

- [x] 3.1 Render the `config` kind in the human and JSON validation
  reports; feed `unknown_config_keys` and `latest_known` to doctor facts,
  verified by CLI tests
  `status_warns_on_an_unknown_config_key_and_quotes_the_requirement`
  (configuration: "Unknown top-level key with a version requirement"),
  `status_warns_on_a_misspelled_top_level_key` (configuration: "Unknown
  top-level key without a version requirement"),
  `update_installs_skills_despite_an_unknown_mcp_key` (configuration:
  "Unknown key of the `mcp` section"),
  `status_fails_on_an_unknown_changelog_key_quoting_the_requirement`,
  `validate_reports_an_unknown_config_key_as_a_warning` (configuration:
  "Strict validation with an unknown key", "Non-strict validation with an
  unknown key" — the report has warnings and no error, which is what the
  exit code reads), `validate_has_no_config_item_when_clean`
  (configuration: "Clean configuration"),
  `doctor_reports_an_unknown_config_key`,
  `doctor_fixes_a_codev_above_the_requirement_with_an_explicit_version`,
  and the existing doctor tests updated for the new check.
- [x] 3.2 Contract test: the `config` kind serializes as `"config"` in
  `ValidateReportV1`, verified by `validate_report_v1_carries_the_config_kind`.

## 4. Documentation (English and French)

- [x] 4.1 `reference/configuration.md`: unknown keys (warning at the top
  level and in `mcp`, error in `inherits` entries and `changelog`, why),
  forward compatibility, codev 0.5 and earlier still rejecting unknown keys;
  verified by both books building with no warning.
- [x] 4.2 `faq.md`: "A teammate's codev fails on the configuration" and
  "`unknown_config_key`"; `reference/cli.md` and `reference/json-output.md`
  doctor tables (`config.keys`, the `codev_version` fix) and the validate
  `config` kind; `guides/repository-integration.md` and `quickstart.md`
  doctor samples regenerated from a real `codev doctor` run; verified by
  both books building with no warning and `grep config.keys` in both
  editions.

## 5. Verification

- [x] 5.1 `cargo fmt --all --check`, `cargo clippy --workspace
  --all-targets --locked -- -D warnings`, `cargo test --workspace --locked`
  green; `codev validate --strict` and `codev doctor` on this repository
  report nothing missing.
