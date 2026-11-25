# FAQ and troubleshooting

## General

### Does Compagnon send my code to a model?

No. The `compagnon` binary never calls a language model. It uses the network
only to fetch inherited git sources (`compagnon sources update`) and to look for a
newer Compagnon release (`compagnon status`, `compagnon self-update`).
Your agent — Claude Code — does the writing, with the same access it has in
any other session.

### Does Compagnon check for updates?

Yes, and only that. At most once every 24 hours, `compagnon status` sends one
anonymous HTTPS request to GitHub's public API
(`https://api.github.com/repos/mairistem/compagnon/releases/latest`) to read the
latest release number, and caches it in `~/.cache/compagnon/update-check.json`.
Nothing about you or your project is sent; there is no telemetry. A failed
request is silent and is not retried before the next day.

The check never runs when `CI` is set, so pipelines stay reproducible. To
turn it off on your machine, set `COMPAGNON_NO_UPDATE_CHECK=1`. Nothing is ever
installed without your consent: updating takes `compagnon self-update`, or your
answer "Update now" in a skill.

### Can I use Compagnon without Claude Code?

The CLI works on its own: you can create changes, write the artifacts
yourself following `compagnon instructions`, validate, sync and archive. The
skills, which do the writing for you, target Claude Code only.

### Should I commit `_compagnon/` and `.claude/skills/`?

Yes. `_compagnon/` is the record of your specs, decisions and changes; commit it
entirely, including `_compagnon/decisions/seal.yaml` and `_compagnon/compagnon.lock`.
Committing `.claude/skills/` gives everyone on the team the same skills
without running `compagnon init`.

### Does a small fix need a full change?

Every change goes through the same cycle, but the plan can be small: a
one-line proposal, a one-requirement delta, a two-task list. For a change that
has no behavioral impact, set `skip_specs: true` in its `change.yaml`.

### Can I keep Compagnon out of `CLAUDE.md`, CI or the pull request template?

Yes. Pass `--no-claude-md`, `--no-ci` or `--no-pr-template` to `compagnon init`,
or `--no-integration` for all three. `compagnon update` never adds them later.
To keep the workflow but own it, delete its first line, `# compagnon:managed`:
Compagnon then leaves the file alone. To remove Compagnon's text from `CLAUDE.md` or
the template, delete the block between the `compagnon:begin` and `compagnon:end`
markers. See [Repository integration and compagnon doctor](guides/repository-integration.md).

### Should my reviewers work in isolation?

By default (`inline`), the skill that wrote the plan or the code also
reviews it, one role after the other, in the same conversation. With
`review: { mode: isolated }` in `_compagnon/config.yaml`, each reviewing role
works alone as a Claude Code sub-agent that never sees the conversation,
and the skill then merges what they found. It costs more — each reviewer
reads the material — and its benefit is not measured yet, so start with
the changes where a fresh look matters most: set `review: { mode:
isolated }` in those changes' `change.yaml`, or project-wide once you
have seen the difference. Run `compagnon update` after changing the project's
mode. See [Inline or isolated review](roles.md#inline-or-isolated-review).

### How do I share this documentation?

```bash
compagnon docs --write ~/Downloads/compagnon-manual.html
```

The file is self-contained — inline CSS, no external resources — and can be
sent by email or dropped into a wiki.

## Troubleshooting

### `no compagnon project found`

```text
error: no compagnon project found from /home/you — run `compagnon init` at the project root
```

Compagnon looks for `_compagnon/` in the current directory and its parents. Run the
command inside your project, or initialize it with `compagnon init`.

### The skills do not appear in Claude Code

Claude Code discovers skills when a session starts. After `compagnon init` or
`compagnon update`, restart Claude Code or open a new session. Check that
`.claude/skills/compagnon-*/SKILL.md` exist at the root of the project you opened.

### `compagnon update` leaves a skill in place

```text
  Left in place because edited by hand — rerun with --force to overwrite:
```

That skill was edited by hand — its content no longer matches its checksum —
and Compagnon does not discard your work. Keep the edit and ignore the message,
or run `compagnon update --force` to regenerate it (check `git diff` first). A
skill generated before checksums existed is preserved the same way until
one `compagnon update --force` gives it its checksum.

### A reviewer's sub-agent fails

In isolated review, a sub-agent that fails or is stopped is run once
more; if it fails again, that role reviews inline and the record says
why on a `Fallback:` line under the role. Nothing is lost: the review is
done, only not in isolation for that role. If every role falls back,
check that Claude Code can start sub-agents (the `Agent` tool) and that
`compagnon doctor` reports the skills current for the review mode —
`skills_review_mode` means `compagnon update` has not run since the mode
changed.

### `compagnon validate` reports `zero_delta_without_marker`

The change has no spec delta under `specs/`. Either write the delta — the
behavior the change adds or modifies — or, if the change truly has no
behavioral impact, add `skip_specs: true` to its `change.yaml`.

### `compagnon validate` reports `decision_seal_mismatch`

The body of a sealed decision was modified — by hand, or by a formatter.
Accepted decisions are immutable:

- **Restore** the original body (`git diff` shows what changed); the seal is
  valid again.
- Or, if the edit was deliberate, **reseal** it:
  `compagnon decision seal <ID> --force`.

To write a decision before sealing it, keep it `proposed` — as
`compagnon decision new`, `supersede`, `deviate` and `promote` create it — and run
`compagnon decision accept <ID>` once its text is final. A decision created with
`compagnon decision new --status accepted` is sealed straight away, with the
template's placeholder text.

To change what a decision says, supersede it with
`compagnon decision supersede <ID> "<new title>"` instead, write the new decision,
then accept it.

To keep formatters away from decisions, exclude `_compagnon/decisions/` in their
configuration.

### `compagnon decision accept` reports `predecessor_not_accepted`

The decision you are accepting lists, in `supersedes`, a decision that is no
longer `accepted` — usually because another decision superseded it after yours
was created. Nothing was written. Run `compagnon decision list` to see which
decision is now in effect, then either point `supersedes` at that decision in
the frontmatter of yours, or drop yours if it has become redundant.
`compagnon decision supersede` refuses a decision that is not `accepted` with the
same code.

### `compagnon validate` reports `decision_missing_frontmatter`

Every `.md` file in `_compagnon/decisions/` is parsed as a decision. Move other
Markdown files, such as a README, out of that folder.

### A spec archived with an earlier Compagnon holds a cut requirement

In Compagnon 0.6.1 and earlier — builds made before this fix — archiving a change
that both modified the last requirement of a spec and added requirements could
write the added ones in the middle of a line of the modified one: the spec
then holds the start of that requirement, the added ones, and its end glued
after them. `compagnon validate` does not detect it, and syncing or archiving the
change again does not repair it. To check, look at the diff of each archive of
a change whose delta modified a spec's last requirement and added
requirements; to repair, put the requirement back whole by hand, or with a new
change whose `MODIFIED` delta gives its full text.

### Sync or archive refuses a change

```text
error: change `add-dark-mode` has errors; run `compagnon validate add-dark-mode` for details
```

With `--json`, the error code is `validation_failed`. Run the suggested
`compagnon validate` command, fix the findings, and sync or archive again. An archive refused because a `MODIFIED` requirement does not exist in
the main spec means the name differs from the one in the main spec: fix the
name, or use `ADDED` if the requirement is new.

### `compagnon validate` reports `rename_source_missing`

The name after `FROM:` in `## RENAMED Requirements` matches no
`### Requirement:` heading of the main spec. It must match exactly, whether
you write the bare name or the full heading in backticks:

```markdown
- FROM: `### Requirement: The chosen theme is remembered`
- TO: `### Requirement: The chosen theme follows the user`
```

### `compagnon validate` reports `delta_unexpected_heading`

A `###` heading in a delta section is not a requirement heading — for
example `### Notes`, or a keyword in a language Compagnon does not know. Compagnon
reads `### Requirement:` and its French equivalent `### Exigence :`; see
[Artifact language](guides/artifact-language.md).

### `/compagnon-propose` ignores my ticket

1. `mcp.jira_tool` was not set in `_compagnon/config.yaml` when the skills were
   generated: `.claude/skills/compagnon-propose/SKILL.md` then reads
   "(Jira MCP not configured)" where the tool name should be. Add the key,
   then run `compagnon update --force`.
2. The tool is configured but not available in the current Claude Code session
   — wrong name, server not connected, expired login. The skill tells you, and
   writes the proposal from your request alone.

See [Jira and other MCP servers](guides/mcp.md).

### `compagnon init` warns `inherit_pin_failed`

A git source given with `--inherit` was declared but could not be pinned:
`git` is missing, the network or your credentials failed, or the `ref` does
not exist (the default is `main`; a repository whose default branch is
`master` needs `…/repo.git@master`). Fix the cause — or the `ref:` in
`_compagnon/config.yaml` — then run `compagnon sources update`.

### `compagnon init` warns `managed_section_malformed`

`CLAUDE.md` or the pull request template has a `<!-- compagnon:begin` line
without a matching `<!-- compagnon:end -->`, or several of them. Compagnon left the
file untouched. Keep exactly one pair of markers, or remove them all to let
`compagnon init` append a fresh section.

### A git source is `unlocked` or `needs_update`

Run `compagnon sources update`. `unlocked` means the source has never been
resolved; `needs_update` means it is pinned in `compagnon.lock` but missing from
your local cache, typically on a fresh machine.

### A teammate's Compagnon fails on the configuration

Their Compagnon is older than your `_compagnon/config.yaml`: the file holds a key
their version does not know.

- **Compagnon 0.5 and earlier** reject any unknown key: every command fails with
  `invalid`, naming the key. They need to update — `compagnon self-update` from
  a recent version, otherwise the installer of
  [Installation](installation.md).
- **Compagnon 0.6.1 and earlier** also reject an unknown key inside a role's
  mapping under `roles:`, in a change's `change.yaml`, in a project
  `schema.yaml` and in an ADR frontmatter.
- **More recent versions** warn instead and keep working —
  `unknown_config_key`, `change_unknown_key`, `schema_unknown_key` or
  `decision_unknown_field`, depending on the file; the warning quotes your
  `compagnon_version` and points to `compagnon self-update`. An unknown key inside an
  `inherits` entry or `changelog` still fails with `invalid`, quoting
  `compagnon_version`.

To make the message explicit for everyone, raise `compagnon_version` whenever you
add a key that only recent versions know. See
[Unknown keys](reference/configuration.md#unknown-keys).

### `unknown_config_key`

`_compagnon/config.yaml` holds a key this Compagnon does not know: it is ignored and
the rest of the configuration applies. Either the key comes from a newer Compagnon
(run `compagnon self-update`, or the version `compagnon doctor` names in its
`compagnon_version` check), or it is a typo, such as `shema` for `schema`: correct
it. `compagnon validate --strict` fails until then. When `compagnon doctor` suggests
`compagnon self-update --version <X.Y.Z>`, your Compagnon is newer than the project's
`compagnon_version` allows, and that command installs a version that satisfies it.

### `change_unknown_key`

A change's `change.yaml` holds a key this Compagnon does not know, at the top level
or under `review:` (`review.<key>`): it is ignored and the rest of the file
applies. The change was written with a newer Compagnon — run `compagnon self-update` —
or the key is misspelled, such as `skip_spec` for `skip_specs`: correct it, or
the marker it names is not applied. `compagnon validate --strict` fails until then.

### `schema_unknown_key`

A project schema, `_compagnon/schemas/<name>/schema.yaml`, holds a key this Compagnon
does not know: the warning names the key and its line, the key is ignored and
the schema stays usable. Run `compagnon self-update` if the schema was written for
a newer Compagnon, otherwise correct the key — a misspelled `require` leaves the
artifact without its dependency. `compagnon validate --strict` reports it on the
`config` item.

### `decision_unknown_field`

An ADR frontmatter holds a field this Compagnon does not know: the decision is
read from the fields it knows, and `compagnon decision accept` keeps the field
when it rewrites the frontmatter. Run `compagnon self-update` if a newer Compagnon
added it, otherwise correct or remove it. An inherited decision's fields are
not reported: you cannot edit its source.

### `compagnon validate`, `sync` or `archive` reports `compagnon_version_unsatisfied`

The running Compagnon does not satisfy the project's `compagnon_version`, and the
commands whose success is a verdict on the project refuse to give one with it:
an older Compagnon could pass what a teammate's fails. Run the fix the message
names — `compagnon self-update`, or `compagnon self-update --version <X.Y.Z>` when your
Compagnon is above an upper bound such as `^0.6` (or widen `compagnon_version` if the
team is moving to the newer release). In CI, raise the `COMPAGNON_VERSION=` pin
whenever `compagnon_version` is raised. Every other command, `compagnon status` and
`compagnon doctor` included, keeps working.

### `compagnon self-update` reports `update_checksum_mismatch`

The downloaded archive does not match its line in the release's
`SHA256SUMS`, or that line is missing. Nothing was installed. Retry later — a
proxy may have altered the download — and, if it persists, report it: it is
the check that protects you from a tampered archive.

### `compagnon self-update` reports `update_not_writable`

Compagnon cannot write in the folder of its binary, for example `/usr/local/bin`
owned by root. Reinstall Compagnon in a folder you own (`install.sh` uses
`~/.local/bin`), or run the command with the rights that folder needs.

### `compagnon self-update` says Compagnon is managed elsewhere

Your binary was installed by `cargo install`, built from a clone, or
installed by a package manager: Compagnon leaves it to that tool and prints the
command to run, such as `cargo install --path crates/compagnon-cli` from an
up-to-date clone.

### `compagnon: command not found`

The install folder is not on your `PATH`. See
[Check the installation](installation.md#check-the-installation).
