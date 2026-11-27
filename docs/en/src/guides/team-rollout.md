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
plan in the pull request, on GitHub or GitLab — see
[Approving a plan on GitHub or GitLab](approve-a-plan.md).

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
spec or change is invalid, or which edits an accepted decision; on
GitLab, you paste the job `compagnon init` prints. See
[Repository integration and compagnon doctor](repository-integration.md).

A failing check only blocks merging once you make it required, in your
site's settings:

- **GitHub:** a branch protection rule (or a ruleset) on your main branch,
  with "Require status checks to pass before merging" and the check
  `compagnon validate`.
- **GitLab:** in the project's merge request settings, "Pipelines must
  succeed".

You can also run `compagnon doctor --strict` in CI: it fails when a check of
the setup is missing — a skill out of date, the `CLAUDE.md` section
removed — and never on a warning. A new project starts with no spec, which
`doctor` reports as missing: add this step once `compagnon doctor` is clean.

## Who approves what

Compagnon stops at the points where a person decides; it does not know who
in your team that person should be. A policy that works:

| What | Who approves | Where |
|---|---|---|
| The plan: proposal, specs, design, tasks | the product owner or the domain expert, with the developer | in the pull request, before Claude builds — see [Approving a plan on GitHub or GitLab](approve-a-plan.md) |
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

## Go further

- [Repository integration and compagnon doctor](repository-integration.md)
- [Roles](../roles.md)
- [Configuration](../reference/configuration.md)
- [Inherited sources](inherited-sources.md)
- [Approving a plan on GitHub or GitLab](approve-a-plan.md)
