# FAQ and troubleshooting

## General

### Does codev send my code to a model?

No. The `codev` binary never calls a language model and, apart from
`codev sources update` fetching inherited git sources, never uses the network.
Your agent — Claude Code — does the writing, with the same access it has in
any other session.

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

To write a decision before sealing it, keep it `proposed` — the default of
`codev decision new` — and run `codev decision accept <ID>` once its text is
final. A decision created with `--status accepted` is sealed straight away,
with the template's placeholder text.

To change what a decision says, supersede it with
`codev decision supersede <ID> "<new title>"` instead.

To keep formatters away from decisions, exclude `_codev/decisions/` in their
configuration.

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

A `###` heading in a delta section is not `### Requirement:` — usually a
translated keyword such as `### Exigence :`. The keywords stay in English
whatever the artifact language; see
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

### A git source is `unlocked` or `needs_update`

Run `codev sources update`. `unlocked` means the source has never been
resolved; `needs_update` means it is pinned in `codev.lock` but missing from
your local cache, typically on a fresh machine.

### `codev: command not found`

The install folder is not on your `PATH`. See
[Check the installation](installation.md#check-the-installation).
