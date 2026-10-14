# FAQ and troubleshooting

## General

### Does codev send my code to a model?

No. The `codev` binary never calls a language model. It uses the network
only to fetch inherited git sources (`codev sources update`) and to look for a
newer codev release (`codev status`, `codev self-update`).
Your agent — Claude Code — does the writing, with the same access it has in
any other session.

### Does codev check for updates?

Yes, and only that. At most once every 24 hours, `codev status` sends one
anonymous HTTPS request to GitHub's public API
(`https://api.github.com/repos/mairistem/codev/releases/latest`) to read the
latest release number, and caches it in `~/.cache/codev/update-check.json`.
Nothing about you or your project is sent; there is no telemetry. A failed
request is silent and is not retried before the next day.

The check never runs when `CI` is set, so pipelines stay reproducible. To
turn it off on your machine, set `CODEV_NO_UPDATE_CHECK=1`. Nothing is ever
installed without your consent: updating takes `codev self-update`, or your
answer "Update now" in a skill.

### Can I use codev without Claude Code?

The CLI works on its own: you can create changes, write the artifacts
yourself following `codev instructions`, validate, sync and archive. The
skills, which do the writing for you, target Claude Code only.

### Should I commit `_codev/` and `.claude/skills/`?

Yes. `_codev/` is the record of your specs, decisions and changes; commit it
entirely, including `_codev/decisions/seal.yaml` and `_codev/codev.lock`.
Committing `.claude/skills/` gives everyone on the team the same skills
without running `codev init`.

### Does a small fix need a full change?

Every change goes through the same cycle, but the plan can be small: a
one-line proposal, a one-requirement delta, a two-task list. For a change that
has no behavioral impact, set `skip_specs: true` in its `change.yaml`.

### Can I keep codev out of `CLAUDE.md`, CI or the pull request template?

Yes. Pass `--no-claude-md`, `--no-ci` or `--no-pr-template` to `codev init`,
or `--no-integration` for all three. `codev update` never adds them later.
To keep the workflow but own it, delete its first line, `# codev:managed`:
codev then leaves the file alone. To remove codev's text from `CLAUDE.md` or
the template, delete the block between the `codev:begin` and `codev:end`
markers. See [Repository integration and codev doctor](guides/repository-integration.md).

### Should my reviewers work in isolation?

By default (`inline`), the skill that wrote the plan or the code also
reviews it, one role after the other, in the same conversation. With
`review: { mode: isolated }` in `_codev/config.yaml`, each reviewing role
works alone as a Claude Code sub-agent that never sees the conversation,
and the skill then merges what they found. It costs more — each reviewer
reads the material — and its benefit is not measured yet, so start with
the changes where a fresh look matters most: set `review: { mode:
isolated }` in those changes' `change.yaml`, or project-wide once you
have seen the difference. Run `codev update` after changing the project's
mode. See [Inline or isolated review](roles.md#inline-or-isolated-review).

### How do I share this documentation?

```bash
codev docs --write ~/Downloads/codev-manual.html
```

The file is self-contained — inline CSS, no external resources — and can be
sent by email or dropped into a wiki.

## Troubleshooting

### `no codev project found`

```text
error: no codev project found from /home/you — run `codev init` at the project root
```

codev looks for `_codev/` in the current directory and its parents. Run the
command inside your project, or initialize it with `codev init`.

### The skills do not appear in Claude Code

Claude Code discovers skills when a session starts. After `codev init` or
`codev update`, restart Claude Code or open a new session. Check that
`.claude/skills/codev-*/SKILL.md` exist at the root of the project you opened.

### `codev update` leaves a skill in place

```text
  Left in place because edited by hand — rerun with --force to overwrite:
```

That skill was edited by hand, and codev does not discard your work. Keep the
edit and ignore the message, or run `codev update --force` to regenerate it
(check `git diff` first).

### A reviewer's sub-agent fails

In isolated review, a sub-agent that fails or is stopped is run once
more; if it fails again, that role reviews inline and the record says
why on a `Fallback:` line under the role. Nothing is lost: the review is
done, only not in isolation for that role. If every role falls back,
check that Claude Code can start sub-agents (the `Agent` tool) and that
`codev doctor` reports the skills current for the review mode —
`skills_review_mode` means `codev update` has not run since the mode
changed.

### `codev validate` reports `zero_delta_without_marker`

The change has no spec delta under `specs/`. Either write the delta — the
behavior the change adds or modifies — or, if the change truly has no
behavioral impact, add `skip_specs: true` to its `change.yaml`.

### `codev validate` reports `decision_seal_mismatch`

The body of a sealed decision was modified — by hand, or by a formatter.
Accepted decisions are immutable:

- **Restore** the original body (`git diff` shows what changed); the seal is
  valid again.
- Or, if the edit was deliberate, **reseal** it:
  `codev decision seal <ID> --force`.

To write a decision before sealing it, keep it `proposed` — as
`codev decision new`, `supersede`, `deviate` and `promote` create it — and run
`codev decision accept <ID>` once its text is final. A decision created with
`codev decision new --status accepted` is sealed straight away, with the
template's placeholder text.

To change what a decision says, supersede it with
`codev decision supersede <ID> "<new title>"` instead, write the new decision,
then accept it.

To keep formatters away from decisions, exclude `_codev/decisions/` in their
configuration.

### `codev decision accept` reports `predecessor_not_accepted`

The decision you are accepting lists, in `supersedes`, a decision that is no
longer `accepted` — usually because another decision superseded it after yours
was created. Nothing was written. Run `codev decision list` to see which
decision is now in effect, then either point `supersedes` at that decision in
the frontmatter of yours, or drop yours if it has become redundant.
`codev decision supersede` refuses a decision that is not `accepted` with the
same code.

### `codev validate` reports `decision_missing_frontmatter`

Every `.md` file in `_codev/decisions/` is parsed as a decision. Move other
Markdown files, such as a README, out of that folder.

### Sync or archive refuses a change

```text
error: change `add-dark-mode` has errors; run `codev validate add-dark-mode` for details
```

With `--json`, the error code is `validation_failed`. Run the suggested
`codev validate` command, fix the findings, and sync or archive again. An archive refused because a `MODIFIED` requirement does not exist in
the main spec means the name differs from the one in the main spec: fix the
name, or use `ADDED` if the requirement is new.

### `codev validate` reports `rename_source_missing`

The name after `FROM:` in `## RENAMED Requirements` matches no
`### Requirement:` heading of the main spec. It must match exactly, whether
you write the bare name or the full heading in backticks:

```markdown
- FROM: `### Requirement: The chosen theme is remembered`
- TO: `### Requirement: The chosen theme follows the user`
```

### `codev validate` reports `delta_unexpected_heading`

A `###` heading in a delta section is not a requirement heading — for
example `### Notes`, or a keyword in a language codev does not know. codev
reads `### Requirement:` and its French equivalent `### Exigence :`; see
[Artifact language](guides/artifact-language.md).

### `/codev-propose` ignores my ticket

1. `mcp.jira_tool` was not set in `_codev/config.yaml` when the skills were
   generated: `.claude/skills/codev-propose/SKILL.md` then reads
   "(Jira MCP not configured)" where the tool name should be. Add the key,
   then run `codev update --force`.
2. The tool is configured but not available in the current Claude Code session
   — wrong name, server not connected, expired login. The skill tells you, and
   writes the proposal from your request alone.

See [Jira and other MCP servers](guides/mcp.md).

### `codev init` warns `inherit_pin_failed`

A git source given with `--inherit` was declared but could not be pinned:
`git` is missing, the network or your credentials failed, or the `ref` does
not exist (the default is `main`; a repository whose default branch is
`master` needs `…/repo.git@master`). Fix the cause — or the `ref:` in
`_codev/config.yaml` — then run `codev sources update`.

### `codev init` warns `managed_section_malformed`

`CLAUDE.md` or the pull request template has a `<!-- codev:begin` line
without a matching `<!-- codev:end -->`, or several of them. codev left the
file untouched. Keep exactly one pair of markers, or remove them all to let
`codev init` append a fresh section.

### A git source is `unlocked` or `needs_update`

Run `codev sources update`. `unlocked` means the source has never been
resolved; `needs_update` means it is pinned in `codev.lock` but missing from
your local cache, typically on a fresh machine.

### A teammate's codev fails on the configuration

Their codev is older than your `_codev/config.yaml`: the file holds a key
their version does not know.

- **codev 0.5 and earlier** reject any unknown key: every command fails with
  `invalid`, naming the key. They need to update — `codev self-update` from
  a recent version, otherwise the installer of
  [Installation](installation.md).
- **More recent versions** warn with `unknown_config_key` instead and keep
  working; the warning quotes your `codev_version` and points to
  `codev self-update`. An unknown key inside an `inherits` entry or
  `changelog` still fails with `invalid`, quoting `codev_version`.

To make the message explicit for everyone, raise `codev_version` whenever you
add a key that only recent versions know. See
[Unknown keys](reference/configuration.md#unknown-keys).

### `unknown_config_key`

`_codev/config.yaml` holds a key this codev does not know: it is ignored and
the rest of the configuration applies. Either the key comes from a newer codev
(run `codev self-update`, or the version `codev doctor` names in its
`codev_version` check), or it is a typo, such as `shema` for `schema`: correct
it. `codev validate --strict` fails until then. When `codev doctor` suggests
`codev self-update --version <X.Y.Z>`, your codev is newer than the project's
`codev_version` allows, and that command installs a version that satisfies it.

### `codev self-update` reports `update_checksum_mismatch`

The downloaded archive does not match its line in the release's
`SHA256SUMS`, or that line is missing. Nothing was installed. Retry later — a
proxy may have altered the download — and, if it persists, report it: it is
the check that protects you from a tampered archive.

### `codev self-update` reports `update_not_writable`

codev cannot write in the folder of its binary, for example `/usr/local/bin`
owned by root. Reinstall codev in a folder you own (`install.sh` uses
`~/.local/bin`), or run the command with the rights that folder needs.

### `codev self-update` says codev is managed elsewhere

Your binary was installed by `cargo install`, built from a clone, or
installed by a package manager: codev leaves it to that tool and prints the
command to run, such as `cargo install --path crates/codev-cli` from an
up-to-date clone.

### `codev: command not found`

The install folder is not on your `PATH`. See
[Check the installation](installation.md#check-the-installation).
