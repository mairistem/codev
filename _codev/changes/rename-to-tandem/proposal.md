# Proposal: Rename codev to Tandem

## Why

The product is renamed **Tandem**: a person and Claude Code moving
forward together on a shared plan. The name "codev" must disappear
everywhere — binary, crates, planning root, files, keys, environment
variables, skills, error codes, URLs, documentation and history — and
the product gets its own mark instead of borrowing the Mairistem logo
alone. It is done now, in 0.x, while a breaking rename is still
acceptable and before more projects adopt the current names.

## What Changes

- **BREAKING** The binary is `tandem` (`tandem.exe` on Windows); every
  command is `tandem <command>`.
- **BREAKING** The planning root is `_tandem/` instead of `_codev/`; the
  lock file is `_tandem/tandem.lock`; the configuration key
  `codev_version` is `tandem_version`.
- **BREAKING** The skills are `/tandem-propose`, `/tandem-apply`, … and
  are installed under `.claude/skills/tandem-*/`; the managed markers
  (`<!-- tandem:begin -->`, `# tandem:managed`, `<!-- tandem-review: N -->`,
  `<!-- tandem-corrections: N -->`) replace the `codev` ones.
- **BREAKING** The JSON error and warning codes containing `codev` are
  renamed (`no_codev_root` → `no_tandem_root`,
  `codev_version_unsatisfied` → `tandem_version_unsatisfied`, …); the JSON
  contract stays v1 otherwise.
- **BREAKING** Environment variables `CODEV_*` become `TANDEM_*`
  (`TANDEM_NO_UPDATE_CHECK`, `TANDEM_EVAL_*`, …); the user cache moves to
  a `tandem` folder; the generated CI workflow is
  `.github/workflows/tandem.yml`.
- **BREAKING** The release archives are `tandem-<version>-<target>.<ext>`,
  the update check and the installers use `mairistem/tandem`.
- New: a project still laid out for codev (a `_codev/` root and no
  `_tandem/`) is detected. Tandem refuses to work on it silently and
  points to `tandem migrate`.
- New: `tandem migrate` converts such a project in place: planning root,
  lock file, configuration key, skills, managed blocks of `CLAUDE.md` and
  of the pull request template, generated CI workflow and review/correction
  markers; an interrupted run is completed by running it again.
- The installers and `self-update` still install codev releases below
  0.7.0 under their former names, so CI jobs pinned to codev keep working
  and going back to codev 0.6 stays possible.
- New: during the 0.7 series, each release also publishes legacy-named
  archives so that an installed codev 0.6 can still `self-update`; the
  binary they carry, run under the name `codev`, says it is now Tandem
  and how to finish the switch.
- `tandem changelog` keeps finding the changes archived before a project
  was migrated, when a git ref predates the rename.
- A dedicated Tandem mark (SVG, Mairistem palette, light and dark
  variants) is shown in the README, the documentation's header and the
  landing page, and is the favicon.
- Every mention of codev in this repository is replaced: crates, code,
  tests, assets, the planning tree (specs, decisions, archived changes),
  evaluation cases and baselines, `CLAUDE.md`, README, CONTRIBUTING,
  CHANGELOG (past entries included), ROADMAP, SECURITY, `docs/en`,
  `docs/fr` and the site.

## Capabilities

### New Capabilities

- `migration`

### Modified Capabilities

- `changelog`
- `distribution`
- `docs`

### Removed Capabilities

## Impact

- All four crates (renamed `tandem-core`, `tandem-engine`,
  `tandem-agents`, `tandem-cli`) and the evaluation crate
  (`tandem-evals`); the workspace manifest and the lock file.
- The JSON contract v1: renamed codes, consumed by the installed skills —
  which are reinstalled by `tandem migrate`.
- Existing CI jobs that install codev with `CODEV_VERSION` keep working;
  GitLab jobs and hand-edited workflows are updated by hand (migration
  guide).
- Release pipeline (`.github/workflows/release.yml`), CI and evaluation
  workflows, `install.sh`, `install.ps1`, the update check's URLs.
- The GitHub repository itself: renamed `mairistem/tandem` by hand
  (GitHub redirects the old URLs).
- Every planning file of this repository, moved from `_codev/` to
  `_tandem/`, including this change.
- Documentation: README(.fr), CONTRIBUTING(.fr), CHANGELOG, ROADMAP,
  SECURITY, CODE_OF_CONDUCT, every chapter of `docs/en` and `docs/fr`
  (with a migration guide in both), the site, the theme and the favicon.
- Evaluation baselines: their recorded prompts and outputs change, so
  the token measures they hold must be checked or recorded again.

## Domains

- [x] security
- [ ] personal-data — no personal data is read, stored or sent
- [x] user-interface
- [ ] performance — a rename changes no hot path
- [x] operations
- [x] migration

## Release note

- Changed: **BREAKING** codev is now **Tandem**. The command is `tandem`, the planning folder is `_tandem/`, the skills are `/tandem-*`, the configuration key is `tandem_version` and environment variables start with `TANDEM_`. Run `tandem migrate` once in each existing project.
- Added: `tandem migrate` converts a project set up with codev — planning folder, lock file, configuration, skills, `CLAUDE.md` and CI workflow — and Tandem tells you to run it when it finds a codev project.
- Changed: **BREAKING** Release archives are named `tandem-<version>-<target>`, the repository is `mairistem/tandem` (installers and update check), the lock file is `_tandem/tandem.lock`, the generated CI workflow is `.github/workflows/tandem.yml`, the update cache lives in a `tandem` folder, and managed markers read `tandem`. Scripts that download `codev-*` archives or read these paths must be updated; `tandem migrate` handles the project files.
- Changed: **BREAKING** JSON error codes that contained `codev` are renamed (`no_codev_root` becomes `no_tandem_root`, and so on).
- Changed: Existing CI jobs that install codev with `CODEV_VERSION` keep installing codev; switch them to `TANDEM_VERSION` and `tandem` when you migrate. GitLab jobs and hand-edited workflows are not changed by `tandem migrate`.
- Changed: A codev 0.6 binary can still `self-update` during the 0.7 series; it then tells you to rename it to `tandem`.
- Added: Tandem has its own logo, in the documentation, the README and the site.
