# FAQ and troubleshooting

## General

### Does Compagnon send my code to a model?

No. The `compagnon` program never talks to an AI model. It goes online for
two things only:

- to download rules shared from another repository, when you ask for it
  (`compagnon sources update`);
- to see whether a newer version of Compagnon exists (`compagnon status`,
  `compagnon self-update`).

The writing is done by your agent, Claude Code. It has the same access as in
any other session.

### Does Compagnon check for updates?

Yes, and that is all it checks. At most once a day, `compagnon status` asks
GitHub for the number of the latest Compagnon version and remembers the
answer on your computer.

- Nothing about you or your project is sent. There is no tracking of any
  kind.
- If the request fails, nothing is shown, and Compagnon does not try again
  before the next day.
- The check never runs in automated pipelines (when the `CI` variable is
  set), so their results stay the same from one run to the next.
- To turn it off on your computer, set `COMPAGNON_NO_UPDATE_CHECK=1`.
- Nothing is ever installed without your consent. To update, you run
  `compagnon self-update`, or you answer "Update now" when a skill asks.

<details>
<summary>Under the hood: the update request</summary>

The request is a single anonymous HTTPS call to GitHub's public API:
`https://api.github.com/repos/mairistem/compagnon/releases/latest`. It reads
the latest release number and caches it in
`~/.cache/compagnon/update-check.json`, for 24 hours.

</details>

### Can I use Compagnon without Claude Code?

Yes, partly. The `compagnon` command works on its own. You can create a
change and write its plan documents (the
[artifacts](glossary.md#artifact)) yourself, following what
`compagnon instructions` tells you. You can then check, sync and archive the
change. What you lose are the skills: they do the writing for you, and they
work only in Claude Code.

### Should I commit `_compagnon/` and `.claude/skills/`?

Yes, both.

- `_compagnon/` is the record of your specs, your decisions and your
  changes. Commit all of it, including `_compagnon/decisions/seal.yaml` and
  `_compagnon/compagnon.lock`.
- When `.claude/skills/` is committed, everyone on the team gets the same
  skills, without running `compagnon init`.

### Does a small fix need a full change?

Every change goes through the same steps, but its plan can be very short.
For example: a one-line proposal, one requirement in its
[delta](glossary.md#delta) (the part of the spec it adds or changes), and a
list of two tasks. If the change does not affect how the software behaves,
set `skip_specs: true` in its `change.yaml`: it then needs no spec at all.

### Can I keep Compagnon out of `CLAUDE.md`, CI or the pull request template?

Yes. When you set up the project, tell `compagnon init` what to leave alone:

- `--no-claude-md` leaves `CLAUDE.md` alone;
- `--no-ci` adds no automatic check (CI);
- `--no-pr-template` leaves the pull request template alone;
- `--no-integration` does all three.

`compagnon update` never adds them later.

You can also take them back after setup:

- To keep the automatic check but manage its file yourself, delete its first
  line, `# compagnon:managed`. Compagnon then leaves the file alone.
- To remove Compagnon's text from `CLAUDE.md` or from the template, delete
  everything between the `compagnon:begin` and `compagnon:end` markers.

See [Repository integration and compagnon doctor](guides/repository-integration.md).

### Should my reviewers work in isolation?

By default, the skill that wrote the plan or the code also reviews it. It
plays each reviewing role in turn, in the same conversation. This is the
`inline` mode.

In the `isolated` mode, each reviewing role works alone, as a separate
helper (a Claude Code sub-agent) that never sees your conversation. The skill
then gathers what they found.

Isolated review costs more, because each reviewer reads everything again.
Its benefit has not been measured yet. So start small: use it on the changes
where a fresh look matters most. Turn it on for the whole project once you
have seen the difference. See
[Inline or isolated review](roles.md#inline-or-isolated-review).

<details>
<summary>Under the hood: turning on isolated review</summary>

For one change, add `review: { mode: isolated }` to that change's
`change.yaml`. For the whole project, add it to `_compagnon/config.yaml`,
then run `compagnon update`: the skills are regenerated for the new mode.

</details>

### How do I share this documentation?

Save it as a single web page:

```bash
compagnon docs --write ~/Downloads/compagnon-manual.html
```

The file holds everything it needs: it loads nothing from the internet. You
can send it by email or drop it into a wiki.

## Troubleshooting

### `no compagnon project found`

```text
error: no compagnon project found from /home/you — run `compagnon init` at the project root
```

Compagnon looks for a `_compagnon/` folder in the current folder, then in
each folder above it. It found none. Run the command from inside your
project, or set the project up with `compagnon init`.

### The skills do not appear in Claude Code

Claude Code looks for skills only when a session starts. After
`compagnon init` or `compagnon update`, restart Claude Code or open a new
session. Also check that the files `.claude/skills/compagnon-*/SKILL.md`
exist at the root of the project you opened.

### `compagnon update` leaves a skill in place

```text
  Left in place because edited by hand — rerun with --force to overwrite:
```

Someone edited that skill by hand, and Compagnon does not throw your work
away. You have two choices:

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

### A reviewer's sub-agent fails

This happens only in isolated review. When a reviewer's helper (sub-agent)
fails or is stopped, Compagnon runs it once more. If it fails again, that
role reviews in the main conversation instead, and the review record says
why, on a `Fallback:` line under the role. Nothing is lost: the review is
done, just not in isolation for that role.

If every role falls back, check two things:

- that Claude Code can start sub-agents (the `Agent` tool);
- that `compagnon doctor` says the skills match the review mode. If it
  reports `skills_review_mode`, the mode was changed but
  `compagnon update` has not run since.

### `compagnon validate` reports `zero_delta_without_marker`

The change does not say how the software's behavior changes: its `specs/`
folder holds no delta. Do one of these:

- write the delta: the behavior the change adds or modifies;
- or, if the change truly does not affect behavior, add `skip_specs: true`
  to its `change.yaml`.

### `compagnon validate` reports `decision_seal_mismatch`

Someone changed the text of an accepted decision, by hand or with a
formatting tool. Accepted decisions are sealed: they must not change. You
can:

- **restore** the original text (`git diff` shows what changed); the seal
  is valid again;
- or, if the edit was on purpose, **seal it again**:
  `compagnon decision seal <ID> --force`.

To change what a decision says, the right way is to replace it with a new
decision: `compagnon decision supersede <ID> "<new title>"`, write the new
decision, then accept it.

To keep formatting tools away from decisions, exclude
`_compagnon/decisions/` in their settings.

<details>
<summary>Under the hood: when a decision gets sealed</summary>

A decision is sealed when it is accepted. To write a decision before sealing
it, keep it `proposed` — as `compagnon decision new`, `supersede`, `deviate`
and `promote` create it — and run `compagnon decision accept <ID>` once its
text is final. A decision created with
`compagnon decision new --status accepted` is sealed straight away, with the
template's placeholder text.

</details>

### `compagnon decision accept` reports `predecessor_not_accepted`

The decision you are accepting says it replaces another one (in
`supersedes`). But that other decision is no longer `accepted`: usually,
a third decision replaced it after yours was created. Nothing was written.

Run `compagnon decision list` to see which decision is in effect now. Then
either point `supersedes` at that decision, in the header
([frontmatter](glossary.md#frontmatter)) of yours, or drop yours if it is no
longer needed. `compagnon decision supersede` refuses a decision that is not
`accepted`, with the same code.

### `compagnon validate` reports `decision_missing_frontmatter`

Compagnon reads every `.md` file in `_compagnon/decisions/` as a decision.
Move other Markdown files, such as a README, out of that folder.

### A spec archived with an earlier Compagnon holds a cut requirement

This is a bug of Compagnon 0.6.1 and earlier (builds made before the fix).
It happened when archiving a change that did two things at once: it modified
the last requirement of a spec, and it added new requirements.

The added requirements could then land in the middle of a line of the
modified one. The spec holds the start of that requirement, then the added
ones, then the end of the cut requirement, glued after them.

`compagnon validate` does not detect it, and syncing or archiving the change
again does not repair it.

- **To check:** look at what each such archive changed in the spec
  (`git diff`).
- **To repair:** put the requirement back whole, by hand, or with a new
  change whose `MODIFIED` delta gives its full text.

### Sync or archive refuses a change

```text
error: change `add-dark-mode` has errors; run `compagnon validate add-dark-mode` for details
```

The change has errors, so Compagnon will not merge it into the project's
specs. Run the `compagnon validate` command the message suggests, fix what
it reports, then sync or archive again. With `--json`, the error code is
`validation_failed`.

A common case: the archive is refused because a `MODIFIED` requirement does
not exist in the project's spec. Its name differs from the one in the
project's spec. Fix the name, or use `ADDED` if the requirement is new.

### `compagnon validate` reports `rename_source_missing`

You are renaming a requirement, but the old name (after `FROM:` in
`## RENAMED Requirements`) matches no `### Requirement:` heading of the
project's spec. It must match exactly. You may write the bare name, or the
full heading in backticks:

```markdown
- FROM: `### Requirement: The chosen theme is remembered`
- TO: `### Requirement: The chosen theme follows the user`
```

### `compagnon validate` reports `delta_unexpected_heading`

A `###` heading in a delta is not a requirement heading. For example,
`### Notes`, or a keyword in a language Compagnon does not know. Compagnon
reads `### Requirement:` and its French version `### Exigence :`; see
[Artifact language](guides/artifact-language.md).

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

See [Jira and other MCP servers](guides/mcp.md).

### `compagnon init` warns `inherit_pin_failed`

You asked to inherit rules from another git repository (with `--inherit`).
Compagnon wrote it down, but could not lock it to a precise version. The
cause is one of these:

- `git` is not installed;
- the network or your credentials failed;
- the branch or tag (`ref`) does not exist. The default is `main`: a
  repository whose default branch is `master` needs `…/repo.git@master`.

Fix the cause, or the `ref:` in `_compagnon/config.yaml`, then run
`compagnon sources update`.

### `compagnon init` warns `managed_section_malformed`

`CLAUDE.md` or the pull request template has a `<!-- compagnon:begin` line
without a matching `<!-- compagnon:end -->`, or has several of them.
Compagnon did not touch the file. Keep exactly one pair of markers, or remove
them all so that `compagnon init` adds a fresh section.

### A git source is `unlocked` or `needs_update`

Run `compagnon sources update`.

- `unlocked` means the source has never been fetched.
- `needs_update` means it is recorded in `compagnon.lock`, but missing from
  your computer. This is typical on a new machine.

### A teammate's Compagnon fails on the configuration

Your teammate's Compagnon is older than your `_compagnon/config.yaml`: the
file holds a setting their version does not know.

- **Compagnon 0.5 and earlier** reject any unknown key: every command fails
  with `invalid`, naming the key. Your teammate needs to update: with
  `compagnon self-update` from a recent version, otherwise with the
  installer of [Installation](installation.md).
- **Compagnon 0.6.1 and earlier** also reject an unknown key in a few more
  places: inside a role under `roles:`, in a change's `change.yaml`, in a
  project's `schema.yaml` and in the header of a decision file.
- **More recent versions** show a warning instead, and keep working. The
  warning quotes your `compagnon_version` and points to
  `compagnon self-update`.

To make the message clear for everyone, raise `compagnon_version` whenever
you add a key that only recent versions know. See
[Unknown keys](reference/configuration.md#unknown-keys).

<details>
<summary>Under the hood: the warning codes for unknown keys</summary>

Depending on the file, the warning is `unknown_config_key`,
`change_unknown_key`, `schema_unknown_key` or `decision_unknown_field`. An
unknown key inside an `inherits` entry or `changelog` still fails with
`invalid`, quoting `compagnon_version`.

</details>

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

A [schema](glossary.md#schema) of your project,
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

### `compagnon validate`, `sync` or `archive` reports `compagnon_version_unsatisfied`

Your Compagnon is not a version the project allows (its `compagnon_version`
setting). Some commands give a verdict on the project: they pass it or fail
it. Those commands refuse to run with a version the project does not allow.
Otherwise, an older Compagnon could pass what a teammate's would fail.

Run the fix the message names:

- `compagnon self-update`;
- or `compagnon self-update --version <X.Y.Z>`, when your Compagnon is above
  an upper limit such as `^0.6`. You can also widen `compagnon_version`, if
  the team is moving to the newer version.

In CI, raise the `COMPAGNON_VERSION=` pin whenever `compagnon_version` is
raised. Every other command, including `compagnon status` and
`compagnon doctor`, keeps working.

### `compagnon self-update` reports `update_checksum_mismatch`

The downloaded file does not match the fingerprint published with the
release (its line in `SHA256SUMS`), or that line is missing. Nothing was
installed. Try again later: a proxy may have altered the download. If it
keeps happening, report it. This check is what protects you from a tampered
file.

### `compagnon self-update` reports `update_not_writable`

Compagnon cannot write in the folder where it is installed, for example
`/usr/local/bin`, owned by the administrator (root). Either reinstall
Compagnon in a folder you own (`install.sh` uses `~/.local/bin`), or run the
command with the rights that folder needs.

### `compagnon self-update` says Compagnon is managed elsewhere

Your copy of Compagnon was installed another way: with `cargo install`,
built from a clone of its code, or by a package manager. Compagnon leaves
updates to that tool, and prints the command to run, such as
`cargo install --path crates/compagnon-cli` from an up-to-date clone.

### `compagnon: command not found`

Your terminal does not know where Compagnon is installed: its folder is not
on your `PATH`. See
[Check the installation](installation.md#check-the-installation).
