# Tasks

## 1. OpenSpec credit

- [x] 1.1 Edit `README.md` — add a credit line after the short
      description, before the two tables ("The two
      halves"):
      ```
      > Inspired by [OpenSpec](https://github.com/tobyhs/openspec) —
      > rebuilt in Rust with its own choices. See the
      > "Origins" section of the documentation for details.
      ```
- [x] 1.2 Edit `docs/codev.md`, section 1 ("Why codev"),
      add a new **"### Origins"** subsection at the end
      of section 1:
      - One sentence on the OpenSpec inspiration.
      - What was **taken from the idea**: propose/apply/archive cycle,
        spec deltas (ADDED/MODIFIED/REMOVED/RENAMED), capabilities,
        first-class ADRs.
      - What was **set aside or done differently**: Rust rather
        than TypeScript, visible `_codev/` rather than hidden, pure
        core + imperative shell rather than a monolithic binary.
      - What is **specific to codev**: K3 seal, K6 drifts,
        K7 promotion, project-side configurable MCP, `codev docs`,
        multi-OS prebuilt distribution.

## 2. Richer README

- [x] 2.1 Add the 4 badges at the top of the README, just under the
      `# codev` title:
      ```markdown
      [![Release](https://github.com/mairistem/codev/actions/workflows/release.yml/badge.svg)](https://github.com/mairistem/codev/actions/workflows/release.yml)
      [![Latest release](https://img.shields.io/github/v/release/mairistem/codev)](https://github.com/mairistem/codev/releases/latest)
      [![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
      [![Platforms](https://img.shields.io/badge/platforms-macOS%20%7C%20Linux%20%7C%20Windows-lightgrey)](docs/codev.md)
      ```
- [x] 2.2 Add a **table of contents** under the description:
      Getting Started, The model, The cycle, Concepts, Documentation,
      Contributing, License.
- [x] 2.3 Add a visual **ASCII art example of the cycle** in the
      "The cycle" section (create it if absent):
      ```
      ┌──────────┐   ┌──────────┐   ┌──────────┐   ┌──────────┐
      │ propose  │→→ │  apply   │→→ │   sync   │→→ │ archive  │
      │ plans    │   │ code     │   │ merge    │   │ files    │
      └──────────┘   └──────────┘   └──────────┘   └──────────┘
      ```
- [x] 2.4 Add at the end of the README a clear link to
      `docs/codev.md` ("Full manual") and the one-liner
      `codev docs`.
- [x] 2.5 Add the **Contributing** ("see
      [CONTRIBUTING.md]") and **License** ("MIT — see
      [LICENSE]") sections in the footer.

## 3. CONTRIBUTING.md

- [x] 3.1 Create `CONTRIBUTING.md` at the root, in French, with
      the sections:
      - **Thanks** — one sentence.
      - **Prerequisites** — stable Rust + cargo, git, optional `gh`.
      - **The cycle** — explain that **contributing to codev means
        using codev**. Steps:
        1. Fork + clone.
        2. `cargo install --path crates/codev-cli` to build.
        3. `codev init` on the fork if not already done.
        4. `/codev-propose <my-idea>` (or `codev new change`).
        5. Implement, `cargo test --workspace`,
           `cargo clippy --workspace --all-targets`,
           `codev validate --strict`.
        6. `codev archive <my-change>`.
        7. Push, PR.
      - **Style** — commits in French or English, neutral tone,
        Co-Authored-By for AIs if applicable.
      - **Release** — reminder: bump `Cargo.toml`, tag `vX.Y.Z`,
        `git push --tags`, edit `CHANGELOG.md` with the V entry.
      - **Reporting a vulnerability** — pointer to `SECURITY.md`.
      - **Code of conduct** — pointer to `CODE_OF_CONDUCT.md`.

## 4. CHANGELOG.md

- [x] 4.1 Create `CHANGELOG.md` at the root, in
      **Keep-a-Changelog** format:
      ```markdown
      # Changelog

      All notable changes to codev are listed here.
      Format: [Keep a Changelog](https://keepachangelog.com/fr/1.1.0/),
      SemVer versioning.

      ## [0.2.0] — 2026-09-24

      ### Added
      - Windows x86_64-pc-windows-msvc target in the prebuilt
        binaries + `install.ps1` script.
      - Embedded documentation: `codev docs` opens a standalone HTML
        manual in the browser.
      - Shell completions: `codev completions <shell>` (bash, zsh,
        fish, powershell, elvish).
      - `codev docs --write PATH` for targeted distribution.

      Full notes:
      https://github.com/mairistem/codev/releases/tag/v0.2.0

      ## [0.1.1] — 2026-09-23

      ### Fixed
      - Release CI: cross-compile `x86_64-apple-darwin` from
        `macos-14` (the free `macos-13` runners are no longer
        reliable).

      Full notes:
      https://github.com/mairistem/codev/releases/tag/v0.1.1

      ## [0.1.0] — 2026-09-23

      ### Added
      - Initial import: propose → apply → sync → archive cycle,
        7 Claude Code skills, decision governance (K3/K6/K7),
        `codev validate --strict`, configurable MCP integration.

      Full notes:
      https://github.com/mairistem/codev/releases/tag/v0.1.0
      ```

## 5. CODE_OF_CONDUCT.md

- [x] 5.1 Create `CODE_OF_CONDUCT.md` at the root — reuse the
      **Contributor Covenant 2.1** official text, in **French**
      (official translation available on the site:
      contributor-covenant.org/version/2/1/code_of_conduct/).
- [x] 5.2 Add the contact — either an email if Ludovic has a
      dedicated one, or a note "open a private GitHub Security
      Advisory on the repo". To be decided at apply time.

## 6. SECURITY.md

- [x] 6.1 Create `SECURITY.md` at the root:
      ```markdown
      # Security policy

      ## Supported versions

      | Version | Support |
      |---------|---------|
      | 0.x     | All recent versions while codev is in 0.x |

      ## Reporting a vulnerability

      Please **do not open a public issue** to report a
      security vulnerability.

      Use GitHub's private mechanism:

      1. Go to https://github.com/mairistem/codev/security/advisories
      2. Click "Report a vulnerability".
      3. Describe the vulnerability, a reproducible scenario, and the
         expected impact.

      Target time for a first response: **72 hours**. If you do not
      receive a response, follow up via a public issue
      asking "Re: vulnerability report" without giving the
      details.
      ```

## 7. GitHub templates

- [x] 7.1 Create `.github/ISSUE_TEMPLATE/bug_report.md`:
      ```markdown
      ---
      name: Bug report
      about: Report incorrect behavior in codev
      title: 'bug: '
      labels: bug
      ---

      **Description**
      <!-- One clear sentence describing the observed problem. -->

      **Reproduction**
      1. …
      2. …
      3. …

      **Expected behavior**
      <!-- What you expected. -->

      **Observed behavior**
      <!-- What happens. Paste the exact error messages. -->

      **Environment**
      - `codev --version`:
      - OS + version:
      - Shell:
      ```
- [x] 7.2 Create `.github/ISSUE_TEMPLATE/feature_request.md`:
      ```markdown
      ---
      name: Feature request
      about: Propose an improvement
      title: 'feat: '
      labels: enhancement
      ---

      **Problem it solves**
      <!-- Describe what is currently missing, with a concrete case. -->

      **Proposed solution**
      <!-- Your idea, with no commitment to implement it. -->

      **Alternatives considered**
      <!-- What was set aside and why. -->

      **Scope**
      <!-- What IS in the request, what IS NOT. -->
      ```
- [x] 7.3 Create `.github/ISSUE_TEMPLATE/config.yml`:
      ```yaml
      blank_issues_enabled: false
      contact_links:
        - name: Questions and discussions
          url: https://github.com/mairistem/codev/discussions
          about: For anything that is not a bug or a feature request.
        - name: Contribution guide
          url: https://github.com/mairistem/codev/blob/main/CONTRIBUTING.md
          about: How to propose a codev change.
      ```
- [x] 7.4 Create `.github/PULL_REQUEST_TEMPLATE.md`:
      ```markdown
      ## Associated codev change

      <!-- Name of the folder under `_codev/changes/` or link to its
           archive. A PR without a codev change is acceptable for a
           trivial fix (typo, doc correction). -->

      ## Summary

      <!-- What the PR brings, in one or two sentences. -->

      ## Checks

      - [ ] `cargo test --workspace` green.
      - [ ] `cargo clippy --workspace --all-targets` with no warnings.
      - [ ] `codev validate --strict` green.
      - [ ] Documentation up to date (`docs/codev.md`, README if needed).

      ## Points of attention for the reviewer

      <!-- Sensitive areas, debatable choices, known missing
           tests. Better to name them here than to leave them as
           surprises. -->
      ```

## 8. Final checks

- [x] 8.0 Fix the pre-existing `distribution` debt: create the
      `MODIFIED` delta on the requirement "The documentation lists
      the three installation paths" (inherited from the Windows
      change) — copy the whole block from the main spec, finish
      the truncated sentence, add two `#### Scenario:` scenarios
      that check the order of the paths in `docs/codev.md` and the
      presence of the two one-liners in `README.md`. The delta lives
      at `_codev/changes/public-launch-polish/specs/distribution/spec.md`.
- [x] 8.1 `codev validate --strict` — green on the change
      (`codev validate public-launch-polish --strict`); the main
      requirement also becomes strict-green after `codev archive`,
      once the delta is merged.
- [x] 8.2 `cargo test --workspace` stays green (nothing in Rust has
      changed, but we check).
- [x] 8.3 `codev docs` opens and correctly renders the new
      "Origins" subsection.
- [ ] 8.4 Open `README.md` on GitHub and check:
      - The 4 badges appear and link correctly.
      - The TOC is clickable.
      - The link to `docs/codev.md` works.
      - The ASCII art renders correctly (code block).

## 9. Delivery

- [ ] 9.1 After merge, bump `Cargo.toml` to `0.2.1` (patch — no
      API changed), `git tag v0.2.1`, `git push origin main
      v0.2.1`. The workflow runs, the release appears.
- [ ] 9.2 Add the `[0.2.1]` entry in `CHANGELOG.md` (in a
      commit after the tag or at the top just before).
