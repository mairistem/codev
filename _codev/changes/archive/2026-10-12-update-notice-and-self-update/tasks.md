# Tasks

Every task's definition of done also includes `cargo fmt --all --check`,
`cargo clippy --workspace --all-targets --locked -- -D warnings` and
`cargo test --workspace --locked`, all passing. Tests follow the project's
conventions: unit tests in `#[cfg(test)] mod tests` next to the code,
behavior-describing names, in-memory `FileSystem`, `FixedEnv`,
`FixedClock`, `MockProcessRunner` and the new `MemoryHttp` and
`MemoryExecutable` doubles in engine and CLI tests, never a temporary
directory or the network, no assumption about path separators or line
endings.

## 1. Pure core: versions, check policy, cache, release, install decisions

- [x] 1.1 Add `semver` to the workspace and `codev-core`; create
  `codev_core::update` with version parsing (leading `v` dropped),
  requirement parsing and matching, verified by unit tests
  `versions_parse_with_or_without_a_leading_v`,
  `requirements_use_cargo_syntax`, `an_invalid_requirement_is_refused`
  (self-update: "Requirement satisfied", "Requirement not satisfied",
  "Invalid requirement").
- [x] 1.2 Check policy and cache model: `CheckEnv`, `should_check`
  (`Disabled` under `CI` or `CODEV_NO_UPDATE_CHECK=1`, `UseCache` under 24
  hours, `Refresh` otherwise), `UpdateCache` JSON parse/render (unreadable
  → none), `record_check` keeping the previous release on failure, verified
  by `ci_or_opt_out_disables_the_check`, `a_recent_check_is_reused`,
  `a_stale_or_missing_check_is_refreshed`,
  `a_failed_check_keeps_the_known_release_and_its_time`,
  `cache_round_trips_and_garbage_reads_as_none` (self-update: "A recent
  check is reused", "A stale check is refreshed once", "Offline, the check
  fails silently and is not retried", "CI disables the check", "The
  opt-out variable disables the check").
- [x] 1.3 Notice and snooze: `notice` (`available`, `snoozed`), `snooze`
  for 7 days of the latest known version, `None` when nothing to snooze,
  verified by `notice_reports_an_available_update`,
  `notice_is_not_available_when_up_to_date`,
  `a_snooze_hides_its_version_for_seven_days`,
  `a_newer_release_ends_the_snooze`, `nothing_to_snooze_when_up_to_date`
  (self-update: "Update available", "Already up to date", "Snoozed
  update", "Snooze expires after 7 days", "A newer release ends the
  snooze", "Nothing to snooze").
- [x] 1.4 Release and integrity: `parse_latest_release` (`tag_name`,
  `html_url`), `asset_name` per target (`.zip` on Windows, gnu Linux mapped
  to musl, unknown → none), `verify_archive` against `SHA256SUMS` (missing
  entry or mismatch → error), verified by
  `latest_release_reads_tag_and_page`, `asset_names_follow_the_release_workflow`,
  `unsupported_targets_have_no_asset`, `a_matching_checksum_passes`,
  `a_mismatching_or_missing_checksum_fails` (self-update: "Checksum
  mismatch").
- [x] 1.5 Installation method and decision: `install_method` (cargo home,
  `~/.cargo/bin`, `target/debug|release`, Homebrew, Nix, Scoop,
  `/usr/bin`, otherwise standalone, Windows separators normalized),
  `upgrade_hint`, `decide` (up to date, refuse implicit downgrade, explicit
  downgrade allowed), staging and aside paths, verified by
  `cargo_installations_are_detected`, `build_folders_are_detected`,
  `package_manager_trees_are_detected`,
  `installer_locations_are_standalone`, `windows_paths_are_classified`,
  `same_version_is_up_to_date`, `implicit_downgrade_is_refused`,
  `explicit_downgrade_is_installed`, `staged_binary_sits_next_to_the_target`
  (self-update: "Installed with cargo", "Installed by the installer
  script", "Latest release older than the running binary", "Explicit
  downgrade").

## 2. Engine: ports, cache, check and self-update orchestration

- [x] 2.1 Ports: `Clock::now_unix` (default from `today()`, system time
  for `SystemClock`); `Http` with `CurlHttp` over `ProcessRunner` and
  `MemoryHttp`; `Executable` with `RealExecutable` (atomic same-folder
  rename, Unix mode 0755, Windows move-aside) and `MemoryExecutable`;
  export them, verified by `fixed_clock_now_is_midnight_utc_of_its_date`,
  `curl_http_passes_the_timeout_and_returns_stdout_bytes`,
  `curl_http_reports_a_failed_request`,
  `memory_executable_replaces_and_keeps_the_previous_binary_aside`.
- [x] 2.2 `codev_engine::update`: cache path under the codev cache root,
  `status_notice` (refresh when due, one request, silent failure recorded),
  `cached_notice` (never requests), `snooze`, `fetch_latest`, verified by
  `status_notice_reuses_a_recent_cache_without_a_request`,
  `status_notice_refreshes_a_stale_cache_once`,
  `status_notice_offline_records_the_attempt_and_stays_silent`,
  `status_notice_is_disabled_in_ci_and_on_opt_out`,
  `cached_notice_never_requests`.
- [x] 2.3 `codev_engine::update::self_update`: method gate, target
  resolution (always fresh), decision, `--check`, download, checksum,
  extraction with `tar`, staged write (`update_not_writable`), swap,
  temporary folder cleanup, cache refresh, project refresh by running the
  new binary's `update` (`update_project_refresh_failed` warning), typed
  `UpdateError` with stable codes, verified by
  `self_update_installs_a_verified_release_and_refreshes_the_project`,
  `self_update_does_nothing_when_up_to_date`,
  `self_update_refuses_a_checksum_mismatch_without_touching_the_binary`,
  `self_update_reports_a_download_failure`,
  `self_update_reports_an_unwritable_folder`,
  `self_update_warns_when_the_project_refresh_fails`,
  `self_update_refuses_an_implicit_downgrade`,
  `self_update_installs_an_explicit_older_version`,
  `self_update_leaves_a_cargo_install_alone`,
  `self_update_check_reports_without_downloading`,
  `self_update_check_offline_fails_with_update_check_failed`,
  `self_update_skips_the_project_refresh_outside_a_project`,
  `self_update_rejects_a_malformed_version` (self-update: "Update to the
  latest release", "Already up to date", "Checksum mismatch", "Download
  failure", "Read-only installation folder", "The project refresh fails
  after the update", "Latest release older than the running binary",
  "Explicit downgrade", "Installed with cargo", "Check with an update
  available", "Check offline").
- [x] 2.4 `codev_version` in `ProjectConfig`, validated at load with
  `EngineError::InvalidCodevVersion` (`invalid_codev_version`), project
  only, verified by `codev_version_is_read_from_the_project_config`,
  `an_invalid_codev_version_fails_with_its_code` (self-update: "Invalid
  requirement").

## 3. Doctor check

- [x] 3.1 `codev_core::doctor`: `codev_version` check after `sources`
  (`codev_version_unsatisfied` missing, `update_available` warning, fix
  `codev self-update`), facts gathered by the CLI from the config and the
  cache only, verified by core tests
  `codev_version_unsatisfied_is_missing`,
  `a_known_newer_release_is_a_warning`, the updated
  `a_complete_project_is_all_ok_in_the_stable_order` and
  `summary_counts_missing_and_warnings`, and CLI tests
  `doctor_reports_an_unsatisfied_codev_version`,
  `doctor_warns_about_a_cached_newer_release_without_a_request`, the
  updated `doctor_on_a_fresh_project_reports_the_expected_statuses` (doctor:
  "A freshly initialized project", "Running codev below the project's
  requirement", "A newer release is known").

## 4. CLI: status, self-update, contract

- [x] 4.1 `Ctx` gains `http` and `exe`; `codev status` computes the notice
  and the requirement; `StatusV1` gains optional `update` and
  `versionRequirement`; human status prints the update line on stderr;
  the doctor hint stays out of the JSON, verified by
  `status_json_carries_update_and_version_requirement`,
  `status_json_omits_update_when_unknown`,
  `status_human_prints_the_update_line_unless_snoozed`,
  `status_json_never_carries_the_doctor_hint`,
  `other_commands_never_request_the_release_api` (self-update: "Update
  available", "Snoozed update", "Other commands never check"; doctor:
  "The hint stays out of the JSON").
- [x] 4.2 `codev self-update [--version] [--check] [--snooze] [--yes]
  [--json]`: target triple embedded by `build.rs`, confirmation on a TTY
  without `--yes`, human rendering, `SelfUpdateV1` and its failure shape,
  Windows startup cleanup of `codev.old.exe`, verified by
  `self_update_json_reports_the_action`,
  `self_update_json_failure_keeps_the_shape`,
  `self_update_snooze_records_and_silences_the_notice`,
  `windows_cleanup_removes_the_previous_binary`, and the parsing test
  `self_update_flags_parse` (self-update: "JSON after an update", "JSON
  failure keeps the shape", "Previous Windows binary cleaned up").
- [x] 4.3 Real end-to-end run against the published v0.5.0 release: a
  scratch copy of codev built as 0.4.9 runs `codev self-update --yes` in a
  scratch codev project and becomes 0.5.0 (download, `SHA256SUMS`
  verification, swap, project refresh), and `codev self-update --check`
  reports the release, verified by the commands' output and
  `<scratch>/codev --version`. Manual: it needs the real network and a
  published release, which the test suite must never depend on; the
  developer's `~/.local/bin/codev` is never touched.

## 5. Skills

- [x] 5.1 `assets/workflows/_update-notice.md` and the `{{UPDATE_NOTICE}}`
  substitution in `codev-agents::claude`; the placeholder after the first
  `codev status --json` of `propose`, `apply`, `update`, `sync` and
  `archive`; `configure` lists `codev_version` among preserved fields,
  verified by `codev-agents` tests
  `action_skills_carry_the_update_notice_once`,
  `read_only_skills_never_offer_an_update`,
  `update_notice_offers_now_and_later_with_self_update`,
  `allowed_tools_are_unchanged_by_the_notice`,
  `configure_preserves_codev_version` (skills: all six scenarios of "Skills
  offer to update codev at the start of an action"; configure: "Role
  documented in the catalog").
- [x] 5.2 Regenerate this repository's skills, verified by
  `cargo run --bin codev -- update --force` and the diff of
  `.claude/skills/`.

## 6. Documentation, configuration, release note

- [x] 6.1 Docs EN and FR (vouvoiement, non-breaking spaces in French
  prose only): installation (upgrading with `codev self-update`), CLI
  reference (`self-update`, help from the real binary), configuration
  (`codev_version`), JSON output (`update`, `versionRequirement`,
  `SelfUpdateV1`), skills (the notice), FAQ (privacy, CI, disabling the
  check), repository integration (doctor `codev_version`), verified by
  both mdBook builds without warnings and `codev docs --print` containing
  `self-update` in both languages.
- [x] 6.2 This repository: `codev_version: ">=0.5"` in
  `_codev/config.yaml`, and `codev doctor` with nothing missing, verified by
  `cargo run --bin codev -- doctor` (`12 check(s), 0 to fix`). `codev
  changelog --write` reads archived changes only, so `[Unreleased]` is
  regenerated right after `codev archive`, and checked in the commit's
  `CHANGELOG.md` diff.
- [x] 6.3 Final verification: `cargo fmt --all --check`, `cargo clippy
  --workspace --all-targets --locked -- -D warnings`, `cargo test
  --workspace --locked`, `codev validate --strict`, all green.
