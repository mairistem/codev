# Tasks

Every task's definition of done also includes `cargo fmt --all --check`,
`cargo clippy --workspace --all-targets -- -D warnings` and
`cargo test --workspace`, all passing. Tests use the in-memory
`FileSystem` and `FixedEnv` ports, never temporary directories, and make
no assumption about path separators or line endings.

Groups 1 to 4 are written under the current crate names (`codev-*`), with
the new names already in the values (`_compagnon`, `compagnon.lock`, …); group 5
renames the crates and the repository.

## 1. Former names and root discovery

- [x] 1.1 Add `codev-core::legacy` holding the former planning root,
  lock file, version key, skill prefix, managed markers, workflow path and
  binary stem, with a module comment stating it is the only place the
  former name may appear in code; verified by a unit test
  `legacy_names_are_the_codev_ones`.
- [x] 1.2 Rename the current constants: `PLANNING_DIR = "_compagnon"`,
  `LOCK_FILE = "compagnon.lock"`, the version key `compagnon_version` (config
  loader, doctor check id, `codev_version_unsatisfied` →
  `compagnon_version_unsatisfied`), `SKILL_PREFIX = "compagnon"`, the managed
  markers (`compagnon:begin`, `# compagnon:managed`, `compagnon-review`,
  `compagnon-corrections`), `.github/workflows/compagnon.yml`, the user cache
  folder `compagnon`, and every `CODEV_*` variable to `COMPAGNON_*`; update the
  existing tests' expectations; verified by `cargo test --workspace`.
- [x] 1.3 Rename the error and warning codes containing `codev` in the
  engine and in `contract::v1` (`no_codev_root` → `no_compagnon_root`, …) and
  list them in the contract's documentation comment; verified by the
  existing contract tests updated, and a test
  `no_json_code_contains_the_former_name` iterating the codes.
- [x] 1.4 In `root::discover`, return `legacy_root` naming the
  directory and `compagnon migrate` when an ancestor holds `_codev/` and no
  `_compagnon/`; `_compagnon/` wins when both exist; doctor goes through the
  same discovery; verified by engine tests
  `a_codev_root_is_refused_with_legacy_root`,
  `doctor_in_a_codev_project_fails_with_legacy_root`,
  `compagnon_root_wins_over_a_leftover_codev_root`,
  `no_root_at_all_fails_with_no_compagnon_root`; make `init` refuse a target
  holding only `_codev/`; and CLI tests
  `init_in_a_codev_project_fails_with_legacy_root_and_writes_nothing` and
  `status_json_in_a_codev_project_reports_legacy_root_and_writes_nothing`.
- [x] 1.5 In the sources loader, refuse a source holding only
  `_codev/config.yaml` with `legacy_source`, naming the source and
  `compagnon migrate`, without writing to it; verified by an engine test
  `a_source_laid_out_for_codev_is_refused_and_left_untouched` and
  `a_migrated_source_with_a_leftover_codev_folder_is_read_from_compagnon`,
  and `a_source_pinned_before_its_migration_names_sources_update`.

## 2. `compagnon migrate`

- [x] 2.1 Add `remove_dir` (empty directories only) to the `FileSystem`
  port, the real implementation and `MemoryFileSystem`; verified by unit
  tests `remove_dir_removes_an_empty_directory` and
  `remove_dir_refuses_a_non_empty_directory`.
- [x] 2.2 Add the pure planner `codev-core::migrate::plan(facts)`
  returning ordered rename, write, remove and keep effects: root, lock,
  `codev_version` key renamed by a line edit preserving comments and
  bytes, review/correction markers, generated skills replaced,
  hand-edited skills kept as `kept_modified`, managed blocks of
  `CLAUDE.md` and of the pull request template, computed from what is
  still present so a second run completes an interrupted one; a
  `codev_version` the running version does not satisfy refused up front
  with `migration_version_unsatisfied`;
  `codev.yml` replaced by `compagnon.yml` unless hand-edited; prose never
  touched; outcomes `already_migrated`, `migration_conflict`,
  `no_compagnon_root`; verified by unit tests in `migrate.rs`, one per
  outcome and one `the_prose_of_planning_files_is_never_rewritten`.
- [x] 2.3 Wire the engine executor and the CLI command
  `migrate [--dry-run] [--json]` with a v1 `migrate` document listing
  every path moved, written, removed or kept; skills and managed blocks
  rendered by the same code as `init`/`update`; verified by CLI tests
  `migrate_converts_a_codev_project`, `migrate_dry_run_writes_nothing`,
  `migrate_keeps_a_hand_edited_skill`,
  `migrate_in_a_migrated_project_does_nothing`,
  `migrate_with_both_roots_fails_with_migration_conflict`,
  `migrate_without_a_planning_root_fails_with_no_compagnon_root`,
  `migrate_leaves_spec_prose_alone`, and
  `a_migrated_project_matches_a_freshly_initialized_one`,
  `migrate_failure_names_the_path_and_the_applied_operations`,
  `migrate_resumes_after_a_failure_past_the_root_rename`,
  `migrate_refuses_an_unsatisfiable_version_requirement`,
  `migrate_rewrites_the_pr_template_managed_section`, and
  `migrate_with_git_sources_says_to_run_sources_update`.
- [x] 2.4 Add `migrate` to the shell completions and the CLI help;
  verified by the existing completion golden tests updated.

## 3. Changelog, bridge, update URLs

- [x] 3.1 In `changelog::ls_tree`, list `_codev/changes/archive/` at a ref
  whose tree has no `_compagnon/` but has `_codev/`; verified by an engine
  test with the `ProcessRunner` fake
  `a_range_spanning_the_migration_lists_only_changes_archived_after_it`,
  and the existing changelog scenarios' tests renamed.
- [x] 3.2 At start-up, when the executable's file stem is the former
  binary stem, print one stderr line saying codev is now Compagnon and how
  to rename or reinstall, then run normally; the decision is a pure function from the executable
  path to the optional notice, fed by the shell with `current_exe`;
  verified by unit tests `a_codev_stem_gets_the_rename_notice`
  (`codev`, `codev.exe`) and `a_compagnon_stem_gets_no_notice`, and CLI tests
  through an injected program name
  `running_as_codev_prints_the_rename_notice_on_stderr` and
  `running_as_codev_keeps_json_stdout_parseable`.
- [x] 3.3 Point `RELEASES_API`, `DOWNLOAD_BASE`, `RELEASE_PAGE_BASE` and
  `INSTALLER_URL` to `mairistem/compagnon`; rename `asset_name`,
  `archive_root`, `binary_name` and the staging names to `compagnon`; verified
  by the existing `update.rs` tests updated
  (`asset_names_follow_the_release_workflow`, …); below `0.7.0` the asset,
  archive root and binary names come from `legacy`, verified by
  `a_downgrade_below_0_7_asks_for_the_codev_archive`.
- [x] 3.4 Update `release.yml`: build `--bin compagnon`, stage
  `compagnon-<version>-<target>`; when the tag starts with `v0.7.`, also stage
  the same binary as `codev[.exe]` in `codev-<version>-<target>/`, archive
  it, and list all archives in `SHA256SUMS`; release notes from
  `compagnon changelog`; verified by tests in `crates/codev-cli/tests` reading the workflow file:
  `release_workflow_publishes_legacy_archives_only_for_0_7`,
  `release_workflow_stages_compagnon_exe_with_the_documents_on_windows`,
  `release_workflow_runs_only_on_version_tags`, and the existing
  `release_workflow_generates_the_body_with_codev_changelog` updated to
  the `compagnon-` stage; plus a manual check after tagging `v0.7.0` that
  `codev 0.6.1 self-update --yes` installs it and that `codev --version`
  then prints the Compagnon version and the rename line — manual because it
  needs a published release and a 0.6 binary.
  Done 2026-11-25: release v0.7.0 (run #14) holds the eight archives and
  SHA256SUMS; codev 0.6.1, downloaded from its release, ran
  `self-update --version 0.7.0 --yes` through `mairistem/codev` and then
  printed the rename notice and `compagnon 0.7.0`. The macOS builds needed
  `macos-latest` (#64): GitHub no longer serves `macos-14`.
- [x] 3.5 Update `install.sh` and `install.ps1`: repository
  `mairistem/compagnon`, `COMPAGNON_VERSION`, archives `compagnon-*`, installed as
  `~/.local/bin/compagnon` and `%LOCALAPPDATA%\Programs\compagnon\compagnon.exe`;
  read `COMPAGNON_VERSION`, else `CODEV_VERSION`; below `0.7.0` fetch
  `codev-*` and install `codev`; with only `CODEV_VERSION` set install as
  `codev`; verified by `sh -n install.sh`, the existing installer checks
  in `ci.yml`, and a CI job `installers_honor_a_codev_pin` running both
  installers with `CODEV_VERSION=0.6.1` (installs `codev` 0.6.1) and
  `COMPAGNON_VERSION=0.7.0` against the release (installs `compagnon`) — run
  after `v0.7.0` exists, before announcing it.
  Done 2026-11-25: Installers #1 green on Ubuntu, macOS and Windows.

## 4. Compagnon mark

- [x] 4.1 Draw the mark (a large open bleu C with round ends, its opening
  closed by three stacked dots — corail, jaune, vert — starting from the
  approved draft in this change's `logo-draft/`) on the Mairistem grid, as
  `docs/assets/compagnon-mark.svg`, `compagnon-mark-dark.svg`,
  `compagnon-logo.svg`, `compagnon-logo-dark.svg`, each with a `<title>`
  naming Compagnon and only palette colors; replace `docs/theme/favicon.svg`;
  verified by a test `compagnon_marks_use_only_the_mairistem_palette`
  parsing the fills, and by the user's visual approval of a rendered
  preview (manual: a visual judgement).
- [x] 4.2 Show the mark in `README.md` and `README.fr.md` (`<picture>`
  with light/dark sources), in `docs/theme/header.hbs` next to the
  Mairistem symbol, in `docs/site/index.html` (header, favicon), and
  inline in the offline manual; `alt="Compagnon"` on README images,
  `role="img" aria-label="Compagnon"` on a standalone inline mark,
  `aria-hidden="true"` where the word Compagnon or the link's own name sits
  beside it, brand links renamed in both editions; verified by CLI tests
  `both_readmes_show_the_light_and_dark_mark` and
  `offline_manual_inlines_the_mark_without_external_images`,
  `inline_marks_are_named_or_decorative`, and
  `mdbook build docs/en docs/fr` succeeding.

## 5. Renaming the repository's contents

- [x] 5.1 Rename the crates and the binary: `git mv crates/codev-*
  crates/compagnon-*`, `evals` package `compagnon-evals`, workspace
  `Cargo.toml` (members, dependencies, repository and homepage URLs,
  version `0.7.0`), `[[bin]] name = "compagnon"`, every `codev_core` /
  `codev_engine` / `codev_agents` path, and `Cargo.lock`; verified by
  `cargo build --release --bin compagnon` and `cargo test --workspace`.
- [x] 5.2 Run `target/release/compagnon migrate --dry-run` then
  `compagnon migrate` on this repository: `_codev/` becomes `_compagnon/` (this
  change included), `compagnon.lock`, `compagnon_version` keeping `">=0.6"`
  (then bumped by hand to `">=0.7"`), skills
  `.claude/skills/compagnon-*`, `CLAUDE.md`, workflows; verified by
  `compagnon validate --strict` and `compagnon doctor` passing.
- [x] 5.3 Sweep every remaining text file with the ordered mapping
  `CODEV`→`COMPAGNON`, `Codev`→`Compagnon`, `codev`→`compagnon` (product written
  **Compagnon** in prose), and `git mv` every path holding `codev` —
  decisions, the archived change `2026-10-23-eval-codev-binary-name`,
  evaluation fixtures' `_codev/` folders — excluding the deliberate
  exceptions of design.md (decision 0003 and this change's folder
  included); replace `changelog.exclude` entries accordingly and add the
  renamed archived change to it; rename by hand the changelog delta's
  `MODIFIED` heading like the main spec's; verified by `cargo test --workspace`,
  `compagnon validate --strict` reporting no missing `MODIFIED` target, and
  the audit `rg -i -l codev` listing only the exceptions.
- [x] 5.4 Re-seal the decisions the sweep changed with
  `compagnon decision seal <id> --force`, then create ADR 0014 "The product is named Compagnon" with
  `compagnon decision supersede 0003`, recording the rename and the re-seal,
  and accept it;
  verified by `compagnon validate --strict` reporting no
  `decision_seal_mismatch` and `git diff` of each re-sealed body showing
  only name changes.
- [x] 5.5 Update the CI, docs and evals workflows (`cargo run --bin
  compagnon`, job names, `COMPAGNON_EVAL_*` secrets' names) and the issue
  templates; verified by the CI run of the pull request being green.
  Green on mairistem/codev#63 (Ubuntu, Windows, lint, MSRV, audit, specs,
  docs); macOS gets no runner, as on `main`.

## 6. Evaluations (decision 0008)

- [x] 6.1 Make the runner run `compagnon migrate` on a case's directory built
  from a `ref` whose tree holds `_codev/`, before writing the case's files
  under `_compagnon/`; verified by a test in `evals/tests`
  `a_case_pinned_before_the_rename_is_migrated_before_it_runs`.
- [x] 6.2 Run `compagnon-evals compare` for every step the skills cover and
  re-record with `compagnon-evals baseline` only the steps whose quality
  leaves the baseline's range or whose tokens move beyond its spread;
  verified by the compare output pasted in the pull request.
  Waived by the user (2026-11-25): the skills change only by the product's
  name; scoring the baseline offline after the rename gives the same results
  as before it, so no paid run was made.

## 7. Documentation

- [x] 7.1 Write the migration guide in `docs/en/src/guides/` and
  `docs/fr/src/guides/` (vouvoiement), listed at the same place in both
  `SUMMARY.md`, and document `compagnon migrate` in both CLI references; the guide covers
  CI jobs and GitLab snippets to switch by hand, `compagnon sources update`
  after migrating, source maintainers keeping a copy of `_codev/` during
  the transition, a version requirement to widen, and going back with
  `compagnon self-update --version 0.6.1`;
  verified by CLI tests `both_editions_list_the_migration_guide` and
  `migration_guides_name_compagnon_migrate_dry_run`, plus the existing
  same-chapters test.
- [x] 7.2 Rewrite the product presentation in README(.fr),
  CONTRIBUTING(.fr), ROADMAP, SECURITY, CODE_OF_CONDUCT, the
  introduction chapters and `docs/site/index.html` (title, description,
  brand) so that they read naturally with the new name, beyond the
  mechanical sweep; verified by `mdbook build docs/en docs/fr`
  succeeding, a CLI test `site_and_introductions_name_compagnon` checking the
  site's `<title>` and description and both introduction chapters, and a
  manual reading of the prose listed in the pull request — manual because
  it is an editorial judgement.
- [x] 7.3 Add the 0.7.0 entry to `CHANGELOG.md` from
  `compagnon changelog`, with a "Moving from codev" note linking the guide;
  verified by `compagnon doctor` reporting `changelog` `ok`.

## 8. Release preparation (manual, outside the repository)

- [x] 8.1 Check that `compagnon` does not collide with a common command on
  macOS, Linux and Windows package indexes, and record the result in the
  pull request; manual: an external lookup.
  Checked 2026-11-25: no `compagnon` on crates.io (nor `compagnon-core`),
  PyPI, npm, Homebrew formulae or casks.
- [x] 8.2 Rename the GitHub repository `mairistem/codev` to
  `mairistem/compagnon` in its settings right before tagging, and never
  re-create `mairistem/codev`; verified by
  `curl -sI https://api.github.com/repos/mairistem/codev` answering a
  redirect; manual: a repository setting.
  Done 2026-11-25: the repository is `mairistem/compagnon`; the former
  name redirects.
- [x] 8.3 Tag `v0.7.0` and check the release lists eight archives and
  `SHA256SUMS`; verified on the release page.
  Done 2026-11-25: a first v0.7.0 published without archives (macOS
  builds on the retired `macos-14`) was deleted with its tag and
  re-created after #64.
