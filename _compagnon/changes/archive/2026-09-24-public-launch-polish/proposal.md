# Proposal: polish codev for the public launch

## Why

The `mairistem/codev` repo has just gone public. A visitor who lands
on it must **understand within ten seconds**:

- what codev does;
- how to install it;
- how to contribute;
- that the project is alive, maintained, and follows basic open
  source conventions.

Today the README is technical and minimal, there is no
`CONTRIBUTING`, no `CHANGELOG`, no `CODE_OF_CONDUCT`, no
`SECURITY`, no issue/PR templates, and **nothing credits
OpenSpec** — which openly inspired the project at its start
(exploration, feature listing, divergence decisions).

This batch adds **no functional capability** — it is a **meta and
documentation** change, explicitly provided for by the spec-driven
schema through the `skip_specs: true` marker.

## What Changes

**Files added at the root**:

- `CONTRIBUTING.md` — how to contribute through the codev cycle
  itself (fork, `codev init` if needed, `/codev-propose`,
  `/codev-apply`, PR). Meta and consistent with the tool.
- `CHANGELOG.md` — published versions with their archived changes,
  Keep-a-Changelog format. Points to the GitHub Releases for the
  full notes.
- `CODE_OF_CONDUCT.md` — standard Contributor Covenant 2.1,
  `mairistem` contact by default.
- `SECURITY.md` — vulnerability reporting procedure: private email
  (`security@…` or GitHub Security Advisories), target response
  time 72h.

**Files added under `.github/`**:

- `.github/ISSUE_TEMPLATE/bug_report.md` — bug template.
- `.github/ISSUE_TEMPLATE/feature_request.md` — feature template.
- `.github/ISSUE_TEMPLATE/config.yml` — disables blank issues,
  points to CONTRIBUTING and Discussions.
- `.github/PULL_REQUEST_TEMPLATE.md` — PR checklist: associated
  codev change, green tests, up-to-date docs.

**Files modified**:

- `README.md` — overhaul:
  - Header with badges (build status, latest release, MIT license,
    supported platforms).
  - "Inspired by [OpenSpec]" block in the first lines.
  - Explicit table of contents for a reader who scrolls.
  - A visual example of the cycle (`ASCII art` of the propose →
    apply → archive cycle).
  - Link to `docs/codev.md` for the full manual.
- `docs/codev.md` — new **"Origins"** subsection in section 1
  (Why codev), which explicitly credits OpenSpec, and lists what
  was taken from the idea and what was set aside.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

None.

### Removed Capabilities

None.

*(Change marked `skip_specs: true` — pure meta / documentation, no
new observable requirement on codev's behavior.)*

## Impact

- **Code**: nothing in the Rust crates or in the install scripts.
  Only markdown files at the root and under `.github/`.
- **JSON contract**: nothing.
- **Files written**: 8 new files, 2 modified files (README,
  docs/codev.md). No tests to write — the docs have no automated
  tests.
- **Migration**: none.
- **Out of scope**:
  - **`docs.rs`** — requires `crates.io`, potentially later.
  - **Dedicated website** — `codev docs` is enough for now.
  - **English translation of the README** — can wait for the first
    non-French-speaking user who shows up.
  - **Download / star count badges** — too early; they will be
    added when there are numbers to show.
  - **Automating the CHANGELOG on each tag** — the V1 version is
    edited by hand at bump time; it will be automated from
    `_codev/changes/archive/` in a dedicated cycle if the need arises.
