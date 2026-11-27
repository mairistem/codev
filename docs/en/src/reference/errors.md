# Error codes and fixes

When `compagnon validate`, `compagnon doctor` or another command reports a
problem, it names it with a stable code. Find the code below: each entry
says what it means and how to fix it.

The full list of codes `--json` can return is in
[JSON output](json-output.md); the checks of `compagnon validate` are in
[File formats](file-formats.md).

## Installing and finding the project

### `compagnon: command not found`

Your [terminal](../glossary.md#terminal) does not know where Compagnon is
installed: its folder is not on your `PATH`. See
[Check the installation](../installation.md#check-the-installation).

### `no compagnon project found`

```text
error: no compagnon project found from /home/you — run `compagnon init` at the project root
```

Compagnon looks for a `_compagnon/` folder in the current folder, then in
each folder above it. It found none. Run the command from inside your
project, or set the project up with `compagnon init`. With `--json`, the
code is `no_compagnon_root`.

## Changes and deltas

### `zero_delta_without_marker`

`compagnon validate` reports that the change does not say how the
software's behavior changes: its `specs/` folder holds no
[delta](../glossary.md#delta). Do one of these:

- write the delta: the behavior the change adds or modifies;
- or, if the change truly does not affect behavior, add `skip_specs: true`
  to its `change.yaml`.

### `validation_failed`

```text
error: change `add-dark-mode` has errors; run `compagnon validate add-dark-mode` for details
```

Sync or archive refused the change: it has errors, so Compagnon will not
merge it into the project's specs. Run the `compagnon validate` command the
message suggests, fix what it reports, then sync or archive again.

A common case: the archive is refused because a `MODIFIED` requirement does
not exist in the project's spec. Its name differs from the one in the
project's spec. Fix the name, or use `ADDED` if the requirement is new.

### `rename_source_missing`

`compagnon validate` reports that you are renaming a requirement, but the
old name (after `FROM:` in `## RENAMED Requirements`) matches no
`### Requirement:` heading of the project's spec. It must match exactly.
You may write the bare name, or the full heading in backticks:

```markdown
- FROM: `### Requirement: The chosen theme is remembered`
- TO: `### Requirement: The chosen theme follows the user`
```

### `delta_unexpected_heading`

A `###` heading in a delta is not a requirement heading. For example,
`### Notes`, or a keyword in a language Compagnon does not know. Compagnon
reads `### Requirement:` and its French version `### Exigence :`; see
[Artifact language](../guides/artifact-language.md).

### A spec archived with an earlier Compagnon holds a cut requirement

This is a bug of Compagnon 0.6.1 and earlier (copies installed before the
fix). It happened when archiving a change that did two things at once: it
modified the last requirement of a spec, and it added new requirements.
The modified requirement could then be cut in two, in the middle of a line,
with the added requirements stuck in the middle.

`compagnon validate` does not detect it, and syncing or archiving the change
again does not repair it.

- **To check:** look at what each such archive changed in the spec
  (`git diff`).
- **To repair:** put the requirement back whole, by hand, or with a new
  change whose `MODIFIED` delta gives its full text.

## Decisions

### `decision_seal_mismatch`

`compagnon validate` reports that someone changed the text of an accepted
decision, by hand or with a formatting tool. Accepted decisions are
[sealed](../glossary.md#seal): they must not change. You can:

- **restore** the original text (`git diff` shows what changed); the seal
  is valid again;
- or, if the edit was on purpose, **seal it again**:
  `compagnon decision seal <ID> --force`.

To change what a decision says, the right way is to replace it with a new
decision: `compagnon decision supersede <ID> "<new title>"`, write the new
decision, then accept it. See [Decisions](../guides/decisions.md#superseded).

To keep formatting tools away from decisions, exclude
`_compagnon/decisions/` in their settings.

### `decision_unsealed`

A warning of `compagnon validate`: an accepted or superseded local decision
has no seal, usually because it was accepted by hand, by editing its
status. Once its text is final, run `compagnon decision seal <ID>` to
record it. See [Accepted and sealed](../guides/decisions.md#accepted-and-sealed).

### `predecessor_not_accepted`

`compagnon decision accept` reports that the decision you are accepting
says it replaces another one (in `supersedes`), but that other decision is
no longer `accepted`: usually, a third decision replaced it after yours was
created. Nothing was written.

Run `compagnon decision list` to see which decision is in effect now. Then
either point `supersedes` at that decision, in the header
([frontmatter](../glossary.md#frontmatter)) of yours, or drop yours if it is
no longer needed. `compagnon decision supersede` refuses a decision that is
not `accepted`, with the same code.

### `decision_missing_frontmatter`

Compagnon reads every `.md` file in `_compagnon/decisions/` as a decision.
Move other Markdown files, such as a README, out of that folder.

## Skills and review

### The skills do not appear in Claude Code

Claude Code looks for skills only when a session starts. After
`compagnon init` or `compagnon update`, restart Claude Code or open a new
session. Also check that the files `.claude/skills/compagnon-*/SKILL.md`
exist at the root of the project you opened.

### A skill is left in place by an update

```text
  Left in place because edited by hand — rerun with --force to overwrite:
```

`compagnon update` found a skill someone edited by hand, and does not throw
that work away. You have two choices:

- keep your edit, and ignore the message;
- or replace it with a fresh copy: run `compagnon update --force`. Check
  what you would lose with `git diff` first.

<details>
<summary>Under the hood: how Compagnon spots a hand edit</summary>

Compagnon stores a checksum of each skill it writes. A skill whose content
no longer matches its checksum counts as edited by hand. A skill generated
before checksums existed is kept the same way, until one
`compagnon update --force` gives it its checksum.

</details>

### `skills_review_mode`

`compagnon doctor` reports that the review mode was changed (`review.mode`,
`inline` or `isolated`) but `compagnon update` has not run since: the
skills still follow the old mode. Run `compagnon update`.

The same check helps when a reviewer's sub-agent fails. This happens only
in isolated review: when a reviewer's helper
([sub-agent](../glossary.md#sub-agent)) fails or is stopped, Compagnon runs
it once more. If it fails again, that role reviews in the main conversation
instead, and the review record says why, on a `Fallback:` line under the
role. Nothing is lost: the review is done, just not in isolation for that
role. If every role falls back, check that Claude Code can start
sub-agents (the `Agent` tool), and that `compagnon doctor` reports no
`skills_review_mode`.

### `/compagnon-propose` ignores my ticket

There are two likely causes.

1. The Jira tool was not set when the skills were generated. In
   `.claude/skills/compagnon-propose/SKILL.md`, you then read
   "(Jira MCP not configured)" where the tool name should be. Add the
   `mcp.jira_tool` key to `_compagnon/config.yaml`, then run
   `compagnon update --force`.
2. The tool is set, but it is not available in the current Claude Code
   session: wrong name, server not connected, or expired login. The skill
   tells you, and writes the proposal from your request alone.

See [Start from a Jira ticket](../guides/mcp.md).

## Setting up and inherited sources

### `inherit_pin_failed`

A warning of `compagnon init`: you asked to inherit rules from another git
repository (with `--inherit`). Compagnon wrote it down, but could not lock
it to a precise version. The cause is one of these:

- `git` is not installed;
- the network or your credentials failed;
- the branch or tag (`ref`) does not exist. The default is `main`: a
  repository whose default branch is `master` needs `…/repo.git@master`.

Fix the cause, or the `ref:` in `_compagnon/config.yaml`, then run
`compagnon sources update`.

### `managed_section_malformed`

A warning of `compagnon init`: `CLAUDE.md` or the pull request template has
a `<!-- compagnon:begin` line without a matching `<!-- compagnon:end -->`,
or has several of them. Compagnon did not touch the file. Keep exactly one
pair of markers, or remove them all so that `compagnon init` adds a fresh
section.

### `unlocked`

A git source has never been fetched. Run `compagnon sources update`. See
[Inherited sources](../guides/inherited-sources.md).

### `needs_update`

A git source is recorded in `compagnon.lock`, but missing from your
computer. This is typical on a new machine. Run `compagnon sources update`.

## Unknown keys and versions

### `invalid`

A file Compagnon refuses to read. The most common case: a teammate's
Compagnon fails on your configuration, because it is older than your
`_compagnon/config.yaml` and the file holds a setting their version does
not know.

- **Compagnon 0.5 and earlier** reject any unknown key: every command fails
  with `invalid`, naming the key. Your teammate needs to update: with
  `compagnon self-update` from a recent version, otherwise with the
  installer of [Installation](../installation.md).
- **Compagnon 0.6.1 and earlier** also reject an unknown key in a few more
  places: inside a role under `roles:`, in a change's `change.yaml`, in a
  project's `schema.yaml` and in the header of a decision file.
- **More recent versions** show a warning instead, and keep working: one of
  the four codes below. The warning quotes your `compagnon_version` and
  points to `compagnon self-update`.

To make the message clear for everyone, raise `compagnon_version` whenever
you add a key that only recent versions know. See
[Unknown keys](configuration.md#unknown-keys). An unknown key inside an
`inherits` entry or `changelog` still fails with `invalid`, quoting
`compagnon_version`.

### `unknown_config_key`

`_compagnon/config.yaml` holds a key this Compagnon does not know. That key
is ignored; the rest of the configuration applies. There are two possible
reasons:

- the key comes from a newer Compagnon: run `compagnon self-update`, or
  install the version `compagnon doctor` names in its `compagnon_version`
  check;
- it is a typo, such as `shema` for `schema`: correct it.

Until then, `compagnon validate --strict` fails.

When `compagnon doctor` suggests `compagnon self-update --version <X.Y.Z>`,
your Compagnon is newer than the project's `compagnon_version` allows. That
command installs a version that fits.

### `change_unknown_key`

A change's `change.yaml` holds a key this Compagnon does not know, at the
top level or under `review:` (`review.<key>`). That key is ignored; the rest
of the file applies. Either:

- the change was written with a newer Compagnon: run
  `compagnon self-update`;
- or the key is misspelled, such as `skip_spec` for `skip_specs`: correct
  it, or the setting it names is not applied.

Until then, `compagnon validate --strict` fails.

### `schema_unknown_key`

A [schema](../glossary.md#schema) of your project,
`_compagnon/schemas/<name>/schema.yaml`, holds a key this Compagnon does not
know. The warning names the key and its line. The key is ignored, and the
schema still works.

- If the schema was written for a newer Compagnon, run
  `compagnon self-update`.
- Otherwise, correct the key. For example, a misspelled `require` leaves an
  artifact without the document it depends on.

`compagnon validate --strict` reports it on the `config` item.

### `decision_unknown_field`

The header of a decision file (an ADR) holds a field this Compagnon does
not know. The decision is read from the fields it knows, and
`compagnon decision accept` keeps the unknown field when it rewrites the
header. Run `compagnon self-update` if a newer Compagnon added it; otherwise
correct or remove it. Fields of a decision inherited from another repository
are not reported: you cannot edit its source.

### `compagnon_version_unsatisfied`

`compagnon validate`, `sync` or `archive` refused to run: your Compagnon is
not a version the project allows (its `compagnon_version` setting). Those
commands give a verdict on the project, so they refuse to run with a
version the project does not allow. Otherwise, an older Compagnon could
pass what a teammate's would fail.

Run the fix the message names:

- `compagnon self-update`;
- or `compagnon self-update --version <X.Y.Z>`, when your Compagnon is newer
  than the highest version the project allows. You can also widen
  `compagnon_version`, if the team is moving to the newer version.

In the project's automatic checks ([CI](../glossary.md#ci)), raise the
Compagnon version they install whenever `compagnon_version` is raised.
Every other command, including `compagnon status` and `compagnon doctor`,
keeps working.

<details>
<summary>Under the hood: upper limits and the CI pin</summary>

An upper limit is a `compagnon_version` such as `^0.6`, which allows 0.6.x
but not 0.7. In CI, the installed version is pinned by `COMPAGNON_VERSION=`:
raise that pin whenever `compagnon_version` is raised.

</details>

## Updating Compagnon

### `update_checksum_mismatch`

`compagnon self-update` reports that the downloaded file does not match the
fingerprint published with the release (its line in `SHA256SUMS`), or that
line is missing. Nothing was installed. Try again later: your network may
have altered the download. If it keeps happening, report it. This check is
what protects you from a tampered file.

### `update_not_writable`

`compagnon self-update` cannot write in the folder where Compagnon is
installed, for example `/usr/local/bin`, owned by the administrator (root).
Either reinstall Compagnon in a folder you own (`install.sh` uses
`~/.local/bin`), or run the command with the rights that folder needs.

### Compagnon is managed elsewhere

`compagnon self-update` says your copy of Compagnon was installed another
way: with `cargo install`, built from a clone of its code, or by a package
manager. Compagnon leaves updates to that tool, and prints the command to
run, such as `cargo install --path crates/compagnon-cli` from an up-to-date
clone.
