# Proposal: A complete project from `compagnon init` — inherited conventions, repository integration, `compagnon doctor`

## Why

`compagnon init` sets up the mechanics, but a fresh project stays empty on what
matters: no team conventions, nothing in CI, nothing telling Claude Code that
the repository works through compagnon — so Claude may change behavior directly
and bypass the process — and no way to see what is still missing. Teams
adopting compagnon now hit these gaps on their first day, before any change is
planned.

## What Changes

- `compagnon init --inherit <SOURCE>`, repeatable: a git URL (`git@…`,
  `https://…`, `ssh://…`, `file://…`, or anything ending in `.git`, with an
  optional `@<ref>` suffix, `main` by default) or a local path. Each source
  is written under `inherits:` in the generated `config.yaml` with the
  provenance comment `# set with compagnon init --inherit`. Git sources are then
  pinned in `_compagnon/compagnon.lock` exactly as `compagnon sources update` does; if
  pinning fails (network, authentication, unknown ref), init still succeeds,
  warns, and points to `compagnon sources update`. An obviously invalid value
  (empty, a git URL with an empty ref, a local path that does not exist) is
  refused before anything is written. No source is added by default and no
  conventions repository is created.
- Repository integration, on by default: `compagnon init` writes
  - a compagnon section in the root `CLAUDE.md` (created if missing) telling
    Claude Code that specs are authoritative, decisions are settled, and
    behavior changes go through `/compagnon-propose` then `/compagnon-apply`;
  - for a GitHub repository, `.github/workflows/compagnon.yml` running
    `compagnon validate --strict` with the installer pinned to the running
    version — unless the file exists without the compagnon marker, or another
    workflow already runs `compagnon validate`; for GitLab CI, a ready-to-paste
    snippet is printed and `.gitlab-ci.yml` is never edited;
  - for a GitHub repository, a compagnon section in the pull request template
    (created at `.github/PULL_REQUEST_TEMPLATE.md` if no template exists): a
    "compagnon change" line and a `compagnon validate --strict` checkbox.
- Opt-out flags: `--no-claude-md`, `--no-ci`, `--no-pr-template`, and
  `--no-integration` for all three. No new interactive question.
- User content is never rewritten: compagnon's text in `CLAUDE.md` and the pull
  request template lives between `<!-- compagnon:begin -->` and
  `<!-- compagnon:end -->` markers, appended when absent. `compagnon update`
  refreshes the managed sections and the managed workflow file in place, and
  only them; it never adds a section the project does not have.
- The init summary lists each integration file as written, refreshed,
  unchanged or skipped, with the reason. The JSON report of `init` and
  `update` gains an additive `integration` array.
- New read-only command `compagnon doctor [--json] [--strict]`: ten checks —
  configuration rules and language, installed skills, main specs, accepted
  decisions, inherited source pins, `CLAUDE.md` section, CI running
  `compagnon validate`, pull request template section (GitHub only), and
  `compagnon validate` errors — each ok / missing / warning with a one-line fix.
  Exit code 0 unless `--strict` and something is missing. JSON contract v1
  gains `DoctorReportV1` with stable check ids and codes.
- `compagnon init` and `compagnon status` print a one-line hint to run
  `compagnon doctor` when a check is missing (human output only).
- `/compagnon-onboard` runs `compagnon doctor --json` and relays the missing checks
  with their fixes; it stays read-only.

## Capabilities

### New Capabilities

- `doctor` — the `compagnon doctor` command, its checks, fixes, exit code, JSON
  report, and the hints in `compagnon init` and `compagnon status`.

### Modified Capabilities

- `init` — the `--inherit` flag, repository integration (`CLAUDE.md`, CI
  workflow, pull request template), managed sections refreshed by
  `compagnon update`, the opt-out flags and the integration summary.
- `skills` — `/compagnon-onboard` relays what `compagnon doctor` reports missing.

### Removed Capabilities

None.

## Impact

- `compagnon-core`: managed-section splicing, `--inherit` parsing, forge
  detection from `.git/config`, the `inherits:` block of the generated
  config, and the pure doctor evaluation.
- `compagnon-engine`: repository integration planning (CI workflow, pull request
  template) and reuse of the sources update for pinning at init.
- `compagnon-agents`: the `CLAUDE.md` section and the skills' version state, both
  owned by the Claude Code target; `assets/workflows/onboard.md`.
- `compagnon-cli`: `init` flags, `doctor` command, `status` hint, human
  rendering, JSON contract v1 (`SetupV1.integration`, `DoctorReportV1`).
- Documentation in English and French: installation, quickstart, CLI
  reference, configuration, inherited sources guide, a `compagnon doctor`
  section, JSON output, FAQ; CHANGELOG.
- compagnon's own repository: a `CLAUDE.md` with the managed section and the
  managed section in its pull request template; no new workflow, since
  `ci.yml` already runs `compagnon validate`.
