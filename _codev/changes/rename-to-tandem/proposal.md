# Proposal: Rename codev to Compagnon

## Why

"codev" is ambiguous. Codev by Cluesmith is an open-source,
spec-driven methodology for Claude Code with its own `codev` command and
`codev/` folder, the same field and nearly the same vocabulary; and, for
French-speaking teams, "codev" is the usual short name of
*codéveloppement professionnel*, a peer-coaching method for managers. The
product is renamed **Mairistem Compagnon**, from the craft tradition of
*compagnonnage*: the developer is the master who leads the work, the Claude
agents are the *compagnons* who carry it out under the developer's
direction, following plans agreed beforehand. It is a method and its
tooling, not an AI. Its sibling, **Mairistem Atelier** (project Talos), will
run Compagnon autonomously on Jira tickets; the two names form one family
— the compagnons work in the atelier. The name "codev" must disappear
everywhere — binary, crates, planning root, files, keys, environment
variables, skills, error codes, URLs, documentation and history — and
the product gets its own mark instead of borrowing the Mairistem logo
alone. It is done now, in 0.x, while a breaking rename is still
acceptable and before more projects adopt the current names.

## What Changes

- **BREAKING** The binary is `compagnon` (`compagnon.exe` on Windows); every
  command is `compagnon <command>`.
- **BREAKING** The planning root is `_compagnon/` instead of `_codev/`; the
  lock file is `_compagnon/compagnon.lock`; the configuration key
  `codev_version` is `compagnon_version`.
- **BREAKING** The skills are `/compagnon-propose`, `/compagnon-apply`, … and
  are installed under `.claude/skills/compagnon-*/`; the managed markers
  (`<!-- compagnon:begin -->`, `# compagnon:managed`, `<!-- compagnon-review: N -->`,
  `<!-- compagnon-corrections: N -->`) replace the `codev` ones.
- **BREAKING** The JSON error and warning codes containing `codev` are
  renamed (`no_codev_root` → `no_compagnon_root`,
  `codev_version_unsatisfied` → `compagnon_version_unsatisfied`, …); the JSON
  contract stays v1 otherwise.
- **BREAKING** Environment variables `CODEV_*` become `COMPAGNON_*`
  (`COMPAGNON_NO_UPDATE_CHECK`, `COMPAGNON_EVAL_*`, …); the user cache moves to
  a `compagnon` folder; the generated CI workflow is
  `.github/workflows/compagnon.yml`.
- **BREAKING** The release archives are `compagnon-<version>-<target>.<ext>`,
  the update check and the installers use `mairistem/compagnon`.
- New: a project still laid out for codev (a `_codev/` root and no
  `_compagnon/`) is detected. Compagnon refuses to work on it silently and
  points to `compagnon migrate`.
- New: `compagnon migrate` converts such a project in place: planning root,
  lock file, configuration key, skills, managed blocks of `CLAUDE.md` and
  of the pull request template, generated CI workflow and review/correction
  markers; an interrupted run is completed by running it again.
- The installers and `self-update` still install codev releases below
  0.7.0 under their former names, so CI jobs pinned to codev keep working
  and going back to codev 0.6 stays possible.
- New: during the 0.7 series, each release also publishes legacy-named
  archives so that an installed codev 0.6 can still `self-update`; the
  binary they carry, run under the name `codev`, says it is now Compagnon
  and how to finish the switch.
- `compagnon changelog` keeps finding the changes archived before a project
  was migrated, when a git ref predates the rename.
- A dedicated Compagnon mark — a large open C (the master) completed by
  three dots (the compagnons) — (SVG, Mairistem palette, light and dark
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

- All four crates (renamed `compagnon-core`, `compagnon-engine`,
  `compagnon-agents`, `compagnon-cli`) and the evaluation crate
  (`compagnon-evals`); the workspace manifest and the lock file.
- The JSON contract v1: renamed codes, consumed by the installed skills —
  which are reinstalled by `compagnon migrate`.
- Existing CI jobs that install codev with `CODEV_VERSION` keep working;
  GitLab jobs and hand-edited workflows are updated by hand (migration
  guide).
- Release pipeline (`.github/workflows/release.yml`), CI and evaluation
  workflows, `install.sh`, `install.ps1`, the update check's URLs.
- The GitHub repository itself: renamed `mairistem/compagnon` by hand
  (GitHub redirects the old URLs).
- Every planning file of this repository, moved from `_codev/` to
  `_compagnon/`, including this change.
- Documentation: README(.fr), CONTRIBUTING(.fr), CHANGELOG, ROADMAP,
  SECURITY, CODE_OF_CONDUCT, every chapter of `docs/en` and `docs/fr`
  (with a migration guide in both), the site, the theme and the favicon.
- Mairistem Atelier (project Talos), not built yet, will drive Compagnon
  through its CLI and its JSON contract: it is born on the new names and
  needs no migration.
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

- Changed: **BREAKING** codev is now **Compagnon**. The command is `compagnon`, the planning folder is `_compagnon/`, the skills are `/compagnon-*`, the configuration key is `compagnon_version` and environment variables start with `COMPAGNON_`. Run `compagnon migrate` once in each existing project.
- Added: `compagnon migrate` converts a project set up with codev — planning folder, lock file, configuration, skills, `CLAUDE.md` and CI workflow — and Compagnon tells you to run it when it finds a codev project.
- Changed: **BREAKING** Release archives are named `compagnon-<version>-<target>`, the repository is `mairistem/compagnon` (installers and update check), the lock file is `_compagnon/compagnon.lock`, the generated CI workflow is `.github/workflows/compagnon.yml`, the update cache lives in a `compagnon` folder, and managed markers read `compagnon`. Scripts that download `codev-*` archives or read these paths must be updated; `compagnon migrate` handles the project files.
- Changed: **BREAKING** JSON error codes that contained `codev` are renamed (`no_codev_root` becomes `no_compagnon_root`, and so on).
- Changed: Existing CI jobs that install codev with `CODEV_VERSION` keep installing codev; switch them to `COMPAGNON_VERSION` and `compagnon` when you migrate. GitLab jobs and hand-edited workflows are not changed by `compagnon migrate`.
- Changed: A codev 0.6 binary can still `self-update` during the 0.7 series; it then tells you to rename it to `compagnon`.
- Added: Compagnon has its own logo, in the documentation, the README and the site.
