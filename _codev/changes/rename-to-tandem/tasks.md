# Tasks

Every task's definition of done also includes `cargo fmt --all --check`,
`cargo clippy --workspace --all-targets -- -D warnings` and
`cargo test --workspace`, all passing. Tests use the in-memory
`FileSystem` and `FixedEnv` ports, never temporary directories, and make
no assumption about path separators or line endings.

Groups 1 to 4 are written under the current crate names (`codev-*`), with
the new names already in the values (`_tandem`, `tandem.lock`, …); group 5
renames the crates and the repository.

## 1. Former names and root discovery

- [ ] 1.1 Add `codev-core::legacy` holding the former planning root,
  lock file, version key, skill prefix, managed markers, workflow path and
  binary stem, with a module comment stating it is the only place the
  former name may appear in code; verified by a unit test
  `legacy_names_are_the_codev_ones`.
- [ ] 1.2 Rename the current constants: `PLANNING_DIR = "_tandem"`,
  `LOCK_FILE = "tandem.lock"`, the version key `tandem_version` (config
  loader, doctor check id, `codev_version_unsatisfied` →
  `tandem_version_unsatisfied`), `SKILL_PREFIX = "tandem"`, the managed
  markers (`tandem:begin`, `# tandem:managed`, `tandem-review`,
  `tandem-corrections`), `.github/workflows/tandem.yml`, the user cache
  folder `tandem`, and every `CODEV_*` variable to `TANDEM_*`; update the
  existing tests' expectations; verified by `cargo test --workspace`.
- [ ] 1.3 Rename the error and warning codes containing `codev` in the
  engine and in `contract::v1` (`no_codev_root` → `no_tandem_root`, …) and
  list them in the contract's documentation comment; verified by the
  existing contract tests updated, and a test
  `no_json_code_contains_the_former_name` iterating the codes.
- [ ] 1.4 In `root::discover`, return `legacy_root` naming the
  directory and `tandem migrate` when an ancestor holds `_codev/` and no
  `_tandem/`; `_tandem/` wins when both exist; doctor goes through the
  same discovery; verified by engine tests
  `a_codev_root_is_refused_with_legacy_root`,
  `doctor_in_a_codev_project_fails_with_legacy_root`,
  `tandem_root_wins_over_a_leftover_codev_root`,
  `no_root_at_all_fails_with_no_tandem_root`, and a CLI test
  `status_json_in_a_codev_project_reports_legacy_root_and_writes_nothing`.
- [ ] 1.5 In the sources loader, refuse a source holding only
  `_codev/config.yaml` with `legacy_source`, naming the source and
  `tandem migrate`, without writing to it; verified by an engine test
  `a_source_laid_out_for_codev_is_refused_and_left_untouched` and
  `a_migrated_source_with_a_leftover_codev_folder_is_read_from_tandem`.

## 2. `tandem migrate`

- [ ] 2.1 Add `remove_dir` (empty directories only) to the `FileSystem`
  port, the real implementation and `MemoryFileSystem`; verified by unit
  tests `remove_dir_removes_an_empty_directory` and
  `remove_dir_refuses_a_non_empty_directory`.
- [ ] 2.2 Add the pure planner `codev-core::migrate::plan(facts)`
  returning ordered rename, write, remove and keep effects: root, lock,
  `codev_version` key renamed by a line edit preserving comments and
  bytes, review/correction markers, generated skills replaced,
  hand-edited skills kept as `kept_modified`, `CLAUDE.md` managed block,
  `codev.yml` replaced by `tandem.yml` unless hand-edited; prose never
  touched; outcomes `already_migrated`, `migration_conflict`,
  `no_tandem_root`; verified by unit tests in `migrate.rs`, one per
  outcome and one `the_prose_of_planning_files_is_never_rewritten`.
- [ ] 2.3 Wire the engine executor and the CLI command
  `migrate [--dry-run] [--json]` with a v1 `migrate` document listing
  every path moved, written, removed or kept; skills and managed blocks
  rendered by the same code as `init`/`update`; verified by CLI tests
  `migrate_converts_a_codev_project`, `migrate_dry_run_writes_nothing`,
  `migrate_keeps_a_hand_edited_skill`,
  `migrate_in_a_migrated_project_does_nothing`,
  `migrate_with_both_roots_fails_with_migration_conflict`,
  `migrate_without_a_planning_root_fails_with_no_tandem_root`,
  `migrate_leaves_spec_prose_alone`, and
  `a_migrated_project_matches_a_freshly_initialized_one`.
- [ ] 2.4 Add `migrate` to the shell completions and the CLI help;
  verified by the existing completion golden tests updated.

## 3. Changelog, bridge, update URLs

- [ ] 3.1 In `changelog::ls_tree`, list `_codev/changes/archive/` at a ref
  whose tree has no `_tandem/` but has `_codev/`; verified by an engine
  test with the `ProcessRunner` fake
  `a_range_spanning_the_migration_lists_only_changes_archived_after_it`,
  and the existing changelog scenarios' tests renamed.
- [ ] 3.2 At start-up, when the executable's file stem is the former
  binary stem, print one stderr line saying codev is now Tandem and how
  to rename or reinstall, then run normally; the decision is a pure function from the executable
  path to the optional notice, fed by the shell with `current_exe`;
  verified by unit tests `a_codev_stem_gets_the_rename_notice`
  (`codev`, `codev.exe`) and `a_tandem_stem_gets_no_notice`, and CLI tests
  through an injected program name
  `running_as_codev_prints_the_rename_notice_on_stderr` and
  `running_as_codev_keeps_json_stdout_parseable`.
- [ ] 3.3 Point `RELEASES_API`, `DOWNLOAD_BASE`, `RELEASE_PAGE_BASE` and
  `INSTALLER_URL` to `mairistem/tandem`; rename `asset_name`,
  `archive_root`, `binary_name` and the staging names to `tandem`; verified
  by the existing `update.rs` tests updated
  (`asset_names_follow_the_release_workflow`, …).
- [ ] 3.4 Update `release.yml`: build `--bin tandem`, stage
  `tandem-<version>-<target>`; when the tag starts with `v0.7.`, also stage
  the same binary as `codev[.exe]` in `codev-<version>-<target>/`, archive
  it, and list all archives in `SHA256SUMS`; release notes from
  `tandem changelog`; verified by tests in `crates/codev-cli/tests` reading the workflow file:
  `release_workflow_publishes_legacy_archives_only_for_0_7`,
  `release_workflow_stages_tandem_exe_with_the_documents_on_windows`,
  `release_workflow_runs_only_on_version_tags`, and the existing
  `release_workflow_generates_the_body_with_codev_changelog` updated to
  the `tandem-` stage; plus a manual check after tagging `v0.7.0` that
  `codev 0.6.1 self-update --yes` installs it and that `codev --version`
  then prints the Tandem version and the rename line — manual because it
  needs a published release and a 0.6 binary.
- [ ] 3.5 Update `install.sh` and `install.ps1`: repository
  `mairistem/tandem`, `TANDEM_VERSION`, archives `tandem-*`, installed as
  `~/.local/bin/tandem` and `%LOCALAPPDATA%\Programs\tandem\tandem.exe`;
  verified by `sh -n install.sh`, the existing installer checks in
  `ci.yml`, and a test `installers_name_only_tandem` grepping both files.

## 4. Tandem mark

- [ ] 4.1 Draw the mark (two wheels — corail rear, vert front — a bleu
  frame pointing forward, a jaune dot) on the Mairistem grid, as
  `docs/assets/tandem-mark.svg`, `tandem-mark-dark.svg`,
  `tandem-logo.svg`, `tandem-logo-dark.svg`, each with a `<title>`
  naming Tandem and only palette colors; replace `docs/theme/favicon.svg`;
  verified by a test `tandem_marks_use_only_the_mairistem_palette`
  parsing the fills, and by the user's visual approval of a rendered
  preview (manual: a visual judgement).
- [ ] 4.2 Show the mark in `README.md` and `README.fr.md` (`<picture>`
  with light/dark sources), in `docs/theme/header.hbs` next to the
  Mairistem symbol, in `docs/site/index.html` (header, favicon), and
  inline in the offline manual; verified by CLI tests
  `both_readmes_show_the_light_and_dark_mark` and
  `offline_manual_inlines_the_mark_without_external_images`, and
  `mdbook build docs/en docs/fr` succeeding.

## 5. Renaming the repository's contents

- [ ] 5.1 Rename the crates and the binary: `git mv crates/codev-*
  crates/tandem-*`, `evals` package `tandem-evals`, workspace
  `Cargo.toml` (members, dependencies, repository and homepage URLs,
  version `0.7.0`), `[[bin]] name = "tandem"`, every `codev_core` /
  `codev_engine` / `codev_agents` path, and `Cargo.lock`; verified by
  `cargo build --release --bin tandem` and `cargo test --workspace`.
- [ ] 5.2 Run `target/release/tandem migrate --dry-run` then
  `tandem migrate` on this repository: `_codev/` becomes `_tandem/` (this
  change included), `tandem.lock`, `tandem_version: ">=0.7"`, skills
  `.claude/skills/tandem-*`, `CLAUDE.md`, workflows; verified by
  `tandem validate --strict` and `tandem doctor` passing.
- [ ] 5.3 Sweep every remaining text file with the ordered mapping
  `CODEV`→`TANDEM`, `Codev`→`Tandem`, `codev`→`tandem` (product written
  **Tandem** in prose), and `git mv` every path holding `codev` —
  decisions, the archived change `2026-10-23-eval-codev-binary-name`,
  evaluation fixtures' `_codev/` folders — excluding the deliberate
  exceptions of design.md; replace `changelog.exclude` entries
  accordingly and add the renamed archived change to it; the delta specs of
  this change are swept with the main specs, so their `MODIFIED` headings
  keep matching; verified by `cargo test --workspace`,
  `tandem validate --strict` reporting no missing `MODIFIED` target, and
  the audit `rg -i -l codev` listing only the exceptions.
- [ ] 5.4 Re-seal the decisions the sweep changed with
  `tandem decision seal <id> --force`, then create and accept ADR 0014
  "The product is named Tandem" recording the rename and the re-seal;
  verified by `tandem validate --strict` reporting no
  `decision_seal_mismatch` and `git diff` of each re-sealed body showing
  only name changes.
- [ ] 5.5 Update the CI, docs and evals workflows (`cargo run --bin
  tandem`, job names, `TANDEM_EVAL_*` secrets' names) and the issue
  templates; verified by the CI run of the pull request being green.

## 6. Evaluations (decision 0008)

- [ ] 6.1 Make the runner run `tandem migrate` on a case's directory built
  from a `ref` whose tree holds `_codev/`, before writing the case's files
  under `_tandem/`; verified by a test in `evals/tests`
  `a_case_pinned_before_the_rename_is_migrated_before_it_runs`.
- [ ] 6.2 Run `tandem-evals compare` for every step the skills cover and
  re-record with `tandem-evals baseline` only the steps whose quality
  leaves the baseline's range or whose tokens move beyond its spread;
  verified by the compare output pasted in the pull request.

## 7. Documentation

- [ ] 7.1 Write the migration guide in `docs/en/src/guides/` and
  `docs/fr/src/guides/` (vouvoiement), listed at the same place in both
  `SUMMARY.md`, and document `tandem migrate` in both CLI references;
  verified by CLI tests `both_editions_list_the_migration_guide` and
  `migration_guides_name_tandem_migrate_dry_run`, plus the existing
  same-chapters test.
- [ ] 7.2 Rewrite the product presentation in README(.fr),
  CONTRIBUTING(.fr), ROADMAP, SECURITY, CODE_OF_CONDUCT, the
  introduction chapters and `docs/site/index.html` (title, description,
  brand) so that they read naturally with the new name, beyond the
  mechanical sweep; verified by `mdbook build docs/en docs/fr`
  succeeding, a CLI test `site_and_introductions_name_tandem` checking the
  site's `<title>` and description and both introduction chapters, and a
  manual reading of the prose listed in the pull request — manual because
  it is an editorial judgement.
- [ ] 7.3 Add the 0.7.0 entry to `CHANGELOG.md` from
  `tandem changelog`, with a "Moving from codev" note linking the guide;
  verified by `tandem doctor` reporting `changelog` `ok`.

## 8. Release preparation (manual, outside the repository)

- [ ] 8.1 Check that `tandem` does not collide with a common command on
  macOS, Linux and Windows package indexes, and record the result in the
  pull request; manual: an external lookup.
- [ ] 8.2 Rename the GitHub repository `mairistem/codev` to
  `mairistem/tandem` in its settings right before tagging, and never
  re-create `mairistem/codev`; verified by
  `curl -sI https://api.github.com/repos/mairistem/codev` answering a
  redirect; manual: a repository setting.
- [ ] 8.3 Tag `v0.7.0` and check the release lists eight archives and
  `SHA256SUMS`; verified on the release page.
