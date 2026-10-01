# Changelog

All notable changes to codev are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
Detailed notes for each version are on the corresponding GitHub Release.

## [Unreleased]

### Changed

- **BREAKING:** `codev decision supersede`, `codev decision deviate` and
  `codev decision promote` now create a `proposed` decision, unsealed, like
  `codev decision new`: write or rework it, then run
  `codev decision accept <ID>` to make it take effect. Scripts that relied on
  these commands taking effect at once must add that step.
- `codev decision supersede` no longer modifies the old decision: it stays
  `accepted`, and in effect, until the new one is accepted. It now refuses a
  decision that is not `accepted` (`predecessor_not_accepted`).
- A deviation created by `codev decision deviate` only sets the inherited
  decision aside once it is accepted.
- `codev decision deviate --json` and `codev decision promote --json` no longer
  return `bodySha256`, since nothing is sealed.

### Added

- `codev decision accept` marks the decisions listed in `supersedes` as
  `superseded` in the same step, leaving their body and seal untouched, and
  lists them in a new `superseded` field of its JSON output. It refuses a
  predecessor that is no longer `accepted` — for example, one another decision
  superseded in the meantime — with `predecessor_not_accepted`.
- Each artifact instruction of the `spec-driven` schema opens with an explicit
  role and its "Done when" criteria: product owner (proposal), QA analyst
  (specs), architect (design), tech lead (tasks).
- `/codev-propose` checks traceability between capabilities, spec files,
  scenarios and task verifications before presenting the plan, and fixes the
  gaps in the artifacts.
- `/codev-propose` then challenges the plan as a skeptical reviewer — need,
  scope, specs, decisions, assumptions, tasks, plus security, compatibility,
  operability, performance or accessibility lenses when the proposal's Impact
  touches them, and the project's `rules:` — and ends its summary with at most
  five "Points to challenge", or says none was found.

## [0.4.0] - 2026-09-30

### Changed

- **BREAKING:** codev now speaks English throughout — CLI output and help,
  skills, and artifact templates. Template headings are now English
  (`# Proposal:`, `## Why`, `## What Changes`, `## Capabilities`, `## Impact`,
  `## Context`, `## Decisions`, `# Tasks`…). Scripts or tooling that relied on
  French output or headings must be updated; run `codev update` to regenerate
  the skills.
- **BREAKING:** `codev init --preset` takes `full`, `minimal` or `custom`.
- The minimal preset now installs `propose`, `explore`, `onboard` and
  `configure`, and the full preset includes `configure` (eight workflows).
- Decisions of the project itself are identified as `project/NNNN` in
  qualified ids and in the JSON output.
- The installers print their messages in English. On Windows, `install.ps1`
  now suggests a PATH command that updates only the user `PATH`, instead of
  `setx`, which can truncate it.
- `codev decision new` now creates a `proposed` decision, unsealed, so its
  body can be written before it is sealed; accept it with
  `codev decision accept`. Pass `--status accepted` to create and seal it at
  once, as before.

### Added

- `codev decision accept <ID>` sets a local `proposed` decision to `accepted`
  and seals its body in the same step. It refuses inherited decisions
  (`cannot_accept_inherited`) and decisions that are not `proposed`
  (`decision_not_proposed`).

- Artifact language: a `language:` key in `_codev/config.yaml` (an ISO 639
  code such as `en`, `fr` or `pt-BR`) sets the language skills write proposals,
  specs, designs and tasks in. Structural keywords stay in English.
- `codev init --language <CODE>` sets it; otherwise it is detected from the
  locale, falling back to `en`. `codev instructions --json` exposes it in a
  `language` field.
- Documentation in English and French, published as a website and embedded in
  the binary. `codev docs --lang en|fr` selects the language.
- The documentation website follows the Mairistem design system, with a
  landing page, light and dark themes, and a language switch between the
  English and French editions; `codev docs` renders in the same style.
- Continuous integration: formatting, lints, tests on Linux, macOS and
  Windows, a minimum Rust version check, validation of codev's own specs, and
  a dependency audit.
- Issue forms for bug reports and feature requests.

### Dependencies

- Updated `dialoguer` to 0.12, `toml` to 1.1, `pulldown-cmark` to 0.13 and
  `sha2` to 0.11, plus minor updates. Existing seals and `codev.lock` files
  are read and written exactly as before.

### Fixed

- `codev sync` validates the change first and refuses on errors, like
  `codev archive`; both now report the stable JSON code `validation_failed`.
- A `RENAMED` delta written with the template's `### Requirement: <name>` form
  is applied instead of being silently ignored, and renaming a requirement
  that does not exist is reported as `rename_source_missing`.
- Renaming and modifying the same requirement in one delta now works.
- A `MODIFIED` merge keeps the blank line before the next requirement.
- Validation reports unexpected headings inside delta sections
  (`delta_unexpected_heading`) and empty delta sections
  (`delta_section_empty`).
- The `subpath` of a git inherited source now also applies to its decisions.
- `codev init` counts the generated `config.yaml` among the files it created.
- `codev docs --print` no longer prints an error when its output is piped to a
  command that exits early, such as `head`.
- Decisions saved with Windows (CRLF) line endings are parsed, and changing
  their status keeps their sealed body intact.

## [0.3.2] - 2026-09-28

### Fixed

- `codev init` and `codev status` now recommend `/codev-configure` whenever the
  configuration has no `rules:`. A long auto-detected `context:` no longer
  hides the recommendation.

## [0.3.1] - 2026-09-28

### Added

- `/codev-configure` skill: analyzes the project (README, contributing guide,
  docs, a sample of the code) and proposes a richer `context:` and
  per-artifact `rules:` for `_codev/config.yaml`. It shows a diff and writes
  only after confirmation.
- `codev init`, and `codev status` when there is no active change, recommend
  `/codev-configure` while the configuration is sparse. JSON output is
  unchanged.
- `/codev-onboard` recommends `/codev-configure` first on a freshly
  initialized project.

### Changed

- `codev init` and `codev update` install eight workflows by default, adding
  `configure`. Projects that list `workflows:` explicitly are unaffected.

## [0.3.0] - 2026-09-28

### Added

- Interactive `codev init` with environment detection: it reads the project's
  manifests, license, CI setup and MCP servers, asks at most two questions
  (workflows and project context), and writes a `_codev/config.yaml` with a
  comment giving the source of each detected value.
- Detection of Jira and Atlassian MCP servers declared in Claude Code's
  configuration files, filled into `mcp.jira_tool`.
- `codev init` options `--yes` (`-y`), `--no-detect` and `--preset`. Without a
  terminal on stdin, `--yes` is implied.

### Changed

- `codev init` and `codev update` install all seven workflows by default —
  propose, explore, onboard, apply, sync, archive, update — instead of three.
  Projects can still restrict the list with `workflows:`.

## [0.2.2] - 2026-09-24

### Added

- A five-minute tutorial covering the whole cycle, from proposal to archive.
- Diagrams of a change's lifecycle, the crate graph and a delta's lifecycle in
  the documentation.

## [0.2.1] - 2026-09-24

### Added

- Standard open-source project files: contributing guide, changelog, code of
  conduct, security policy, and GitHub issue and pull request templates.
- Explicit credit to [OpenSpec](https://github.com/Fission-AI/OpenSpec) in the
  README and the documentation.
- README badges, table of contents and a diagram of the cycle.

### Fixed

- The distribution spec validates again with `codev validate --strict`.

## [0.2.0] - 2026-09-24

### Added

- Prebuilt Windows x86_64 binary, and an `install.ps1` script to install it
  without Rust.
- `codev docs`: opens the documentation, embedded in the binary, as a
  self-contained HTML page. `--write <PATH>` writes it to a file instead.
- `codev completions <SHELL>` for bash, zsh, fish, PowerShell and elvish.

## [0.1.1] - 2026-09-23

### Fixed

- Release builds for Intel macOS are cross-compiled from Apple Silicon
  runners.

## [0.1.0] - 2026-09-23

### Added

- Initial release: the propose → apply → sync → archive cycle, seven Claude
  Code skills, architecture decisions with sealing, deviations and promotion,
  `codev validate --strict`, project-configured MCP integration, and prebuilt
  binaries for macOS and Linux.

[Unreleased]: https://github.com/mairistem/codev/compare/v0.4.0...HEAD
[0.4.0]: https://github.com/mairistem/codev/releases/tag/v0.4.0
[0.3.2]: https://github.com/mairistem/codev/releases/tag/v0.3.2
[0.3.1]: https://github.com/mairistem/codev/releases/tag/v0.3.1
[0.3.0]: https://github.com/mairistem/codev/releases/tag/v0.3.0
[0.2.2]: https://github.com/mairistem/codev/releases/tag/v0.2.2
[0.2.1]: https://github.com/mairistem/codev/releases/tag/v0.2.1
[0.2.0]: https://github.com/mairistem/codev/releases/tag/v0.2.0
[0.1.1]: https://github.com/mairistem/codev/releases/tag/v0.1.1
[0.1.0]: https://github.com/mairistem/codev/releases/tag/v0.1.0
