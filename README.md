# codev

**Spec-driven development for Claude Code.**

[![CI](https://github.com/mairistem/codev/actions/workflows/ci.yml/badge.svg)](https://github.com/mairistem/codev/actions/workflows/ci.yml)
[![Latest release](https://img.shields.io/github/v/release/mairistem/codev)](https://github.com/mairistem/codev/releases/latest)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Platforms](https://img.shields.io/badge/platforms-macOS%20%7C%20Linux%20%7C%20Windows-lightgrey)](https://github.com/mairistem/codev/releases/latest)

English · [Français](README.fr.md)

codev adds a thin layer of specifications to your repository, so that you and
Claude Code agree on what must be built before a line of code is written — and
so that architecture decisions, once made, stop being re-debated on every
change.

<!-- demo: docs/assets/demo.gif (to record with vhs, see docs/assets/demo.tape) -->

## Why

Without specs, intent lives in chat history and in people's heads. With codev,
every change starts as a small plan in `_codev/changes/`: why, what, how, and
a **spec delta** that states the behavior it adds or modifies.

A change adds one requirement:

```markdown
## ADDED Requirements

### Requirement: The chosen theme is remembered

The application SHALL restore the theme the user last picked, on every device they sign in from.

#### Scenario: Returning user

- **GIVEN** a user who picked the dark theme on their laptop
- **WHEN** they sign in on their phone
- **THEN** the application renders with the dark theme
```

When the change is archived, codev merges the delta into the living spec,
`_codev/specs/ui/theme/spec.md`, and leaves everything else in it untouched:

```markdown
# Theme Specification

## Purpose

Let users choose the color theme of the interface.

## Requirements

### Requirement: Theme follows the system preference by default

The application SHALL render with the operating system's color scheme until the user picks a theme.

#### Scenario: No theme chosen yet
…

### Requirement: The chosen theme is remembered

The application SHALL restore the theme the user last picked, on every device they sign in from.

#### Scenario: Returning user
…
```

The spec always describes what the system does today, and each archived
change records why it changed. codev never calls a model itself: Claude Code
writes, and codev tells it what to write, where, and under which constraints.

## Getting started

Install codev — no Rust toolchain needed.

macOS and Linux:

```bash
curl -sSL https://raw.githubusercontent.com/mairistem/codev/main/install.sh | sh
```

Windows (PowerShell):

```powershell
iwr -useb https://raw.githubusercontent.com/mairistem/codev/main/install.ps1 | iex
```

Both scripts verify the SHA-256 of the binary they download. You can also
download a binary from the
[releases page](https://github.com/mairistem/codev/releases/latest), or build
from source with `cargo install --path crates/codev-cli`; see the
[installation guide](https://mairistem.github.io/codev/en/installation.html).

**Requirements:** macOS (Apple Silicon or Intel), Linux x86_64 or Windows
x86_64, and [Claude Code](https://claude.com/claude-code) to use the skills.

### Quick start

```bash
cd your-project
codev init
```

Then, in Claude Code:

```text
/codev-propose add a dark mode that follows the system preference
/codev-apply add-dark-mode
/codev-archive add-dark-mode
```

The [quickstart](https://mairistem.github.io/codev/en/quickstart.html) walks
through it step by step.

## How it works

Every change follows the same cycle, with one Claude Code skill per step:

```text
┌───────────┐    ┌───────────┐    ┌───────────┐    ┌───────────┐
│  propose  │───▶│   apply   │───▶│   sync    │───▶│  archive  │
│ plan it   │    │ build it  │    │ merge the │    │ file it   │
│           │    │           │    │ deltas    │    │ away      │
└───────────┘    └───────────┘    └───────────┘    └───────────┘
```

- **propose** — writes the plan: proposal, spec deltas, design, tasks. No code.
- **apply** — implements the tasks, running their verification and checking
  them off.
- **sync** — merges the deltas into the main specs (optional; archive does it).
- **archive** — validates, merges, and moves the change to a dated archive.

`/codev-explore`, `/codev-update`, `/codev-onboard` and `/codev-configure`
complete the set. Behind the skills, the `codev` CLI does the atomic work —
status, instructions, validation, merging — with a stable JSON output.

Everything lives in a visible folder at the root of your repository:

```text
_codev/
├── specs/        # behavior contracts — what the system does
├── decisions/    # architecture decisions — why it is built this way
├── changes/      # work in progress, and the dated archive
└── config.yaml   # context, rules, language, inherited sources
```

- **Changes are deltas.** A change declares `ADDED`, `MODIFIED`, `REMOVED` or
  `RENAMED` requirements instead of rewriting specs, which makes codev work on
  existing code, not only on new projects.
- **Decisions are immutable.** Accepted ADRs are sealed with a hash; changing
  one means superseding it, and `codev validate` catches silent edits.
- **Conventions can be shared.** A project can inherit context, rules and
  decisions from another repository, read-only and pinned to a git commit.
- **Your language.** Artifacts are written in the language your team sets;
  their structure stays machine-readable.

## Comparison

codev is one of several spec-driven tools for coding agents. A summary of
documented features, as of September 2026:

| | codev | [OpenSpec](https://github.com/Fission-AI/OpenSpec) | [Spec Kit](https://github.com/github/spec-kit) |
|---|---|---|---|
| Distribution | Single native binary | npm package (Node.js) | Python CLI (installed with uv) |
| Supported agents | Claude Code | 30+ tools | Many, including GitHub Copilot |
| Spec deltas merged into living specs | Yes | Yes | No — artifacts per feature |
| Architecture decisions (ADRs) | Yes, sealed against edits | No | No — a project constitution instead |
| Sharing across repositories | Read-only sources pinned by git commit | Stores (beta) | — |
| License | MIT | MIT | MIT |

codev builds directly on OpenSpec's ideas: the propose → apply → archive
cycle, changes as the unit of work, and the `ADDED` / `MODIFIED` / `REMOVED` /
`RENAMED` delta operations all come from OpenSpec. If you need an agent other
than Claude Code, OpenSpec and Spec Kit are the better fit. If something in
this table is out of date, please open an issue.

## Documentation

The full documentation is at
**[mairistem.github.io/codev](https://mairistem.github.io/codev/en/)** —
workflow, concepts, guides, and the CLI, configuration and file format
references. It is also embedded in the binary, readable offline:

```bash
codev docs
```

## Contributing

Contributions are welcome. codev is developed with codev: small fixes go
through a regular pull request, and everything else starts as a codev change.
See [CONTRIBUTING.md](CONTRIBUTING.md), and the [roadmap](ROADMAP.md) for what
is planned.

Please report security issues privately, as described in
[SECURITY.md](SECURITY.md). This project follows a
[code of conduct](CODE_OF_CONDUCT.md).

## License

codev is released under the [MIT License](LICENSE).

codev is inspired by [OpenSpec](https://github.com/Fission-AI/OpenSpec), by
Fission AI, also released under the MIT License.
