# Proposal: A complete project from `codev init` — inherited conventions, repository integration, `codev doctor`

## Why

`codev init` sets up the mechanics, but a fresh project stays empty on what
matters: no team conventions, nothing in CI, nothing telling Claude Code that
the repository works through codev — so Claude may change behavior directly
and bypass the process — and no way to see what is still missing. Teams
adopting codev now hit these gaps on their first day, before any change is
planned.

## What Changes

- `codev init --inherit <SOURCE>`, repeatable: a git URL (`git@…`,
  `https://…`, `ssh://…`, `file://…`, or anything ending in `.git`, with an
  optional `@<ref>` suffix, `main` by default) or a local path. Each source
  is written under `inherits:` in the generated `config.yaml` with the
  provenance comment `# set with codev init --inherit`. Git sources are then
  pinned in `_codev/codev.lock` exactly as `codev sources update` does; if
  pinning fails (network, authentication, unknown ref), init still succeeds,
  warns, and points to `codev sources update`. An obviously invalid value
  (empty, a git URL with an empty ref, a local path that does not exist) is
  refused before anything is written. No source is added by default and no
  conventions repository is created.
- Repository integration, on by default: `codev init` writes
  - a codev section in the root `CLAUDE.md` (created if missing) telling
    Claude Code that specs are authoritative, decisions are settled, and
    behavior changes go through `/codev-propose` then `/codev-apply`;
  - for a GitHub repository, `.github/workflows/codev.yml` running
    `codev validate --strict` with the installer pinned to the running
    version — unless the file exists without the codev marker, or another
    workflow already runs `codev validate`; for GitLab CI, a ready-to-paste
    snippet is printed and `.gitlab-ci.yml` is never edited;
  - for a GitHub repository, a codev section in the pull request template
    (created at `.github/PULL_REQUEST_TEMPLATE.md` if no template exists): a
    "codev change" line and a `codev validate --strict` checkbox.
- Opt-out flags: `--no-claude-md`, `--no-ci`, `--no-pr-template`, and
  `--no-integration` for all three. No new interactive question.
- User content is never rewritten: codev's text in `CLAUDE.md` and the pull
  request template lives between `<!-- codev:begin -->` and
  `<!-- codev:end -->` markers, appended when absent. `codev update`
  refreshes the managed sections and the managed workflow file in place, and
  only them; it never adds a section the project does not have.
- The init summary lists each integration file as written, refreshed,
  unchanged or skipped, with the reason. The JSON report of `init` and
  `update` gains an additive `integration` array.
- New read-only command `codev doctor [--json] [--strict]`: ten checks —
  configuration rules and language, installed skills, main specs, accepted
  decisions, inherited source pins, `CLAUDE.md` section, CI running
  `codev validate`, pull request template section (GitHub only), and
  `codev validate` errors — each ok / missing / warning with a one-line fix.
  Exit code 0 unless `--strict` and something is missing. JSON contract v1
  gains `DoctorReportV1` with stable check ids and codes.
- `codev init` and `codev status` print a one-line hint to run
  `codev doctor` when a check is missing (human output only).
- `/codev-onboard` runs `codev doctor --json` and relays the missing checks
  with their fixes; it stays read-only.

## Capabilities

### New Capabilities

- `doctor` — the `codev doctor` command, its checks, fixes, exit code, JSON
  report, and the hints in `codev init` and `codev status`.

### Modified Capabilities

- `init` — the `--inherit` flag, repository integration (`CLAUDE.md`, CI
  workflow, pull request template), managed sections refreshed by
  `codev update`, the opt-out flags and the integration summary.
- `skills` — `/codev-onboard` relays what `codev doctor` reports missing.

### Removed Capabilities

None.

## Impact

- `codev-core`: managed-section splicing, `--inherit` parsing, forge
  detection from `.git/config`, the `inherits:` block of the generated
  config, and the pure doctor evaluation.
- `codev-engine`: repository integration planning (CI workflow, pull request
  template) and reuse of the sources update for pinning at init.
- `codev-agents`: the `CLAUDE.md` section and the skills' version state, both
  owned by the Claude Code target; `assets/workflows/onboard.md`.
- `codev-cli`: `init` flags, `doctor` command, `status` hint, human
  rendering, JSON contract v1 (`SetupV1.integration`, `DoctorReportV1`).
- Documentation in English and French: installation, quickstart, CLI
  reference, configuration, inherited sources guide, a `codev doctor`
  section, JSON output, FAQ; CHANGELOG.
- codev's own repository: a `CLAUDE.md` with the managed section and the
  managed section in its pull request template; no new workflow, since
  `ci.yml` already runs `codev validate`.
