# Deploying Compagnon for a team

This page is for the person who sets Compagnon up for a team: everyone
installs the same version, the [repository](../glossary.md#repository)
holds what the team shares, a check blocks a
[pull request](../glossary.md#pull-request) whose plan is invalid, and
everyone knows who approves what.

## Install it on every workstation

Every developer who has Claude write code needs the `compagnon` command and
Claude Code. [Installation](../installation.md) gives the one-line
installers for macOS, Linux and Windows, and how to install a given
version (`COMPAGNON_VERSION=0.6.1`).

People who only review plans install nothing: they read and approve the
plan in the pull request, on GitHub — see
[Approving a plan on GitHub](approve-a-plan.md).

## One version for everyone

Set the versions the team accepts in `_compagnon/config.yaml`:

```yaml
compagnon_version: ">=0.6, <0.8"
```

To require one exact version, write `"=0.6.1"`. Quote it: YAML reads `>`
specially. With a version outside that range, the skills offer to update
before they start, and `compagnon validate`, `compagnon sync` and
`compagnon archive` refuse to run until the developer updates — see
[`compagnon_version`](../reference/configuration.md#compagnon_version).

The [CI](../glossary.md#ci) job that `compagnon init` writes installs a
pinned version (`COMPAGNON_VERSION=` in `.github/workflows/compagnon.yml`).
When you raise `compagnon_version` above it, raise the pin in the same
[commit](../glossary.md#commit). Compagnon 0.5 and earlier do not know the
key and reject the file: add it once everyone runs a newer version. See
[Update notice](../skills.md#update-notice).

## What to commit

Run `compagnon init` once, on one machine, and commit what it wrote:

| Path | Why |
|---|---|
| `_compagnon/` | the specs, the decisions, the changes and the configuration, including `_compagnon/decisions/seal.yaml` and `_compagnon/compagnon.lock` |
| `.claude/skills/` | everyone gets the same skills without running `compagnon init` |
| `CLAUDE.md` | tells Claude Code, in every session, to plan before changing behavior |
| `.github/workflows/compagnon.yml` | the check on every push and pull request (GitHub) |
| the pull request template (`.github/PULL_REQUEST_TEMPLATE.md` or yours) | the line where the developer names the change (GitHub) |

After upgrading Compagnon, `compagnon update` refreshes the skills, the
managed sections and the workflow's pinned version: commit the result too.
See [Should I save `_compagnon/` and `.claude/skills/` in the project's
history?](../faq.md#should-i-save-_compagnon-and-claudeskills-in-the-projects-history-commit-them) in the FAQ.

## The check that blocks a pull request

The check `compagnon init` sets up fails a push or a pull request whose
spec or change is invalid, or which edits an accepted decision; what
it writes is described in [What compagnon init writes](#what-compagnon-init-writes)
below.

A failing check only blocks merging once you make it required, in your
repository's settings on GitHub:

- **GitHub:** a branch protection rule (or a ruleset) on your main branch,
  with "Require status checks to pass before merging" and the check
  `compagnon validate`.

You can also run `compagnon doctor --strict` in CI: it fails when a check of
the setup is missing — a skill out of date, the `CLAUDE.md` section
removed — and never on a warning. A new project starts with no spec, which
`doctor` reports as missing: add this step once `compagnon doctor` is clean.

## Who approves what

Compagnon stops at the points where a person decides; it does not know who
in your team that person should be. A policy that works:

| What | Who approves | Where |
|---|---|---|
| The plan: proposal, specs, design, tasks | the product owner or the domain expert, with the developer | in the pull request, before Claude builds — see [Approving a plan on GitHub](approve-a-plan.md) |
| The code | the developers, as for any pull request | in the pull request, once the tasks are done |
| A [decision](../concepts.md#decision) | the team | accepted with `compagnon decision accept` in a pull request that those it binds review |
| A risk the review left open | the person accountable for it | the skill asks, and records the answer with that person's name |

This is a suggestion, not something Compagnon enforces: it is your site's
rules — required approvals, a `CODEOWNERS` file naming who must approve
`_compagnon/changes/` or `_compagnon/decisions/` — that make it binding.
The [roles page](../roles.md#where-you-decide) lists every point where
Compagnon hands over.

## The review roles

Before a person reads the plan or the code, Compagnon's reviewing roles
have challenged it, and their findings are in the change's `review.md`.
Which roles run, with which criteria and in which mode, is set for the
whole team in `_compagnon/config.yaml` — see [Roles](../roles.md).

## What compagnon init writes

### CLAUDE.md

Claude Code reads `CLAUDE.md` at the start of every session. `compagnon init`
creates it, or adds a section to yours, so that Claude does not change
behavior without a compagnon change:

```markdown
<!-- compagnon:begin — managed by compagnon; `compagnon update` rewrites this block, edit outside it -->
## Spec-driven development with compagnon

This repository follows spec-driven development with compagnon.

- `_compagnon/specs/` describes the current behavior and is authoritative.
- `_compagnon/decisions/` holds accepted architecture decisions: follow them, do not re-debate them.
- For any change of behavior, start with `/compagnon-propose`, then implement it with `/compagnon-apply`. Do not modify behavior without a compagnon change.
- Trivial fixes (typos, documentation wording) are exempt.
- Run `compagnon validate` before committing.

Installed skills: `/compagnon-propose`, `/compagnon-explore`, `/compagnon-onboard`, `/compagnon-apply`, `/compagnon-sync`, `/compagnon-archive`, `/compagnon-update`, `/compagnon-configure`.
<!-- compagnon:end -->
```

The section is in English whatever the `language:` of the project: it is
written for the agent. When `/compagnon-apply` is not installed, it asks to
implement by following the change's `tasks.md` instead.

### CI workflow

On a GitHub repository — one with a `.github/` folder, or a git remote on
`github.com` — `compagnon init` creates `.github/workflows/compagnon.yml`:

```yaml
# compagnon:managed — generated by `compagnon init`, refreshed by `compagnon update`.
# Delete this first line to take the file over: compagnon will then leave it alone.
name: compagnon

on:
  push:
  pull_request:

jobs:
  validate:
    name: compagnon validate
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v7
      - name: Install compagnon 0.6.1
        run: |
          curl -sSL https://raw.githubusercontent.com/mairistem/compagnon/main/install.sh | COMPAGNON_VERSION=0.6.1 sh
          echo "$HOME/.local/bin" >> "$GITHUB_PATH"
      - name: Validate specs and changes
        run: compagnon validate --strict
```

The installer is pinned to the version of Compagnon that wrote the file, so CI
validates with the same rules as your machine. Compagnon does not write this file
when:

- another workflow of `.github/workflows/` already runs `compagnon validate` —
  any line naming `compagnon` then `validate` counts, such as
  `cargo run --bin compagnon -- validate`;
- `.github/workflows/compagnon.yml` exists and was not written by Compagnon (its first
  line is not `# compagnon:managed`).

Compagnon never edits any other workflow file.

### Pull request template

On GitHub, `compagnon init` adds a section to your pull request template — the
first `pull_request_template.md` found in `.github/`, the project root or
`docs/`, in any case — or creates `.github/PULL_REQUEST_TEMPLATE.md`:

```markdown
<!-- compagnon:begin — managed by compagnon; `compagnon update` rewrites this block, edit outside it -->
## compagnon

- compagnon change: <!-- folder under `_compagnon/changes/`, or "trivial fix, no change" -->
- [ ] `compagnon validate --strict` passes.
<!-- compagnon:end -->
```

## Managed sections

Compagnon's text in `CLAUDE.md` and the pull request template sits between two
HTML comments, invisible once the markdown is rendered:

```markdown
<!-- compagnon:begin — managed by compagnon; `compagnon update` rewrites this block, edit outside it -->
…
<!-- compagnon:end -->
```

- Everything outside the markers is yours and is never modified. A section
  added to an existing file goes at its end, after a blank line; you may move
  the whole block anywhere.
- `compagnon update` rewrites what lies between the markers, and nothing else:
  edits you make inside the block are lost on the next update.
- When the markers are unbalanced or repeated, the file is left alone and the
  `managed_section_malformed` warning names it.

The workflow file is Compagnon's as a whole. `compagnon update` rewrites it, which
moves its pinned version after an upgrade, as long as its first line is
`# compagnon:managed`. Delete that line to take the file over: Compagnon then leaves
it alone.

`compagnon update` only refreshes what exists. It never creates `CLAUDE.md`, the
workflow or the template, and never adds a section to a file that has none.

## Opting out

| Flag | Skips |
|---|---|
| `--no-claude-md` | the section in `CLAUDE.md` |
| `--no-ci` | `.github/workflows/compagnon.yml` |
| `--no-pr-template` | the section in the pull request template |
| `--no-integration` | all three |

The flags apply to one run of `compagnon init`. A later `compagnon init` without
them writes the files; `compagnon update` never does.

## compagnon doctor

`compagnon doctor` checks the whole setup and gives, for each gap, the command or
skill that closes it. It is read-only and works offline; for the `changelog`
check, it asks the local git repository which changes were archived since the
latest tag:

```bash
compagnon doctor
```

```text
compagnon doctor — /home/you/acme-app

Configuration
  ✗ config.rules     no rules: in _compagnon/config.yaml
      fix: /compagnon-configure
  ✓ config.language  language: is set
  ✓ config.keys      no unknown key in _compagnon/config.yaml
  ✓ config.roles     active roles: analyst, investigator, product, qa, architect, lead, tracer, skeptic, developer, newcomer; on domain: security (security), privacy (personal-data), accessibility (user-interface), performance (performance), ops (operations), migration (migration); disabled: none
  ✓ skills           8 skill(s) installed and current
  ✓ sources          no inherited git source
  ✓ compagnon_version    no newer compagnon release known

Planning
  ✗ specs            no main spec in _compagnon/specs/
      fix: /compagnon-propose <a change> — documenting existing behavior is planned work too
  ! decisions        no accepted decision (not every project needs one)
      fix: compagnon decision new "<title>"
  ✓ validate         compagnon validate reports no issue

Repository
  ✓ claude_md        CLAUDE.md has the compagnon section
  ✓ ci               .github/workflows/compagnon.yml runs compagnon validate
  ✓ pr_template      the pull request template has the compagnon section
  ✓ changelog        CHANGELOG.md matches the archived changes

14 check(s), 2 to fix, 1 warning(s)
```

`✗` marks a missing item, `!` a warning — a project may legitimately have no
accepted decision. The `pr_template` check only appears for GitHub
repositories. The `changelog` check, a warning only, compares the
`[Unreleased]` section of `CHANGELOG.md` with what `compagnon changelog` generates;
the fix is `compagnon changelog --write` — see
[Release notes and CHANGELOG](release-notes.md). The `compagnon_version` check is missing when
the running Compagnon does not satisfy the project's
[`compagnon_version`](../reference/configuration.md#compagnon_version), and a
warning when the update cache knows a newer release; the fix is
`compagnon self-update`, or `compagnon self-update --version <X.Y.Z>` when the running
Compagnon is newer than the requirement allows. The `config.keys` check warns
about keys of `_compagnon/config.yaml` this Compagnon does not know — see
[Unknown keys](../reference/configuration.md#unknown-keys). The full list of checks is in the
[CLI reference](../reference/cli.md#compagnon-doctor).

`compagnon init` and `compagnon status` end with a one-line hint when something is
missing, and `/compagnon-onboard` lists the missing items with their fix.

In CI, `compagnon doctor --strict` exits with status `1` when a check is missing;
warnings never fail. `compagnon doctor --json` gives the same report to scripts,
with stable check ids and codes — see [JSON output](../reference/json-output.md#doctor).

## Go further

- [Roles](../roles.md)
- [Configuration](../reference/configuration.md)
- [Inherited sources](inherited-sources.md)
- [Approving a plan on GitHub](approve-a-plan.md)
