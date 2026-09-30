# Claude Code skills

codev ships eight workflows. `codev init` and `codev update` render each one as
a Claude Code skill in `.claude/skills/codev-<workflow>/SKILL.md`. A skill is
discovered by Claude Code automatically, and you can also invoke it directly
by typing its name as a slash command, such as `/codev-propose`.

| Skill | What it does | Modifies |
|---|---|---|
| `/codev-propose` | Creates a change and writes all its planning artifacts in one pass | `_codev/changes/<name>/` only |
| `/codev-explore` | Thinks an idea through, compares approaches, clarifies a need | Nothing |
| `/codev-onboard` | Introduces codev, reads the project's state, recommends the next action | Nothing |
| `/codev-configure` | Proposes a richer `context:` and per-artifact `rules:` for `_codev/config.yaml` | `_codev/config.yaml`, after confirmation |
| `/codev-apply` | Implements the tasks of a planned change, checking them off | Project code and `tasks.md` |
| `/codev-update` | Revises a written planning artifact and keeps the others consistent | `_codev/changes/<name>/` only |
| `/codev-sync` | Merges a change's deltas into the main specs without archiving | Through `codev sync` only |
| `/codev-archive` | Validates, merges and moves a change to the archive | Through `codev archive` only |

## Tool permissions

Each skill declares the tools it may use in the `allowed-tools` field of its
frontmatter. The permissions are part of each skill's promise: a skill that
must not write files is not given the tools to do so.

| Skill | `allowed-tools` |
|---|---|
| `codev-propose` | `Bash(codev:*), Read, Write, Edit, Glob, Grep`, plus the Jira MCP tool when configured |
| `codev-explore` | `Bash(codev:*), Read, Glob, Grep` |
| `codev-onboard` | `Bash(codev:*), Read, Glob` |
| `codev-configure` | `Bash(codev:*), Read, Write, Edit, Glob, Grep` |
| `codev-apply` | `Bash(codev:*), Read, Write, Edit, Glob, Grep, Bash` |
| `codev-update` | `Bash(codev:*), Read, Write, Edit, Glob, Grep` |
| `codev-sync` | `Bash(codev:*), Read` |
| `codev-archive` | `Bash(codev:*), Read` |

`Bash(codev:*)` lets a skill run `codev` commands and nothing else. Only
`codev-apply` gets general `Bash`, because it must run your tests and build
commands. `codev-sync` and `codev-archive` cannot write files at all: every
change they make goes through the CLI.

## The skills in detail

### Propose (`/codev-propose`)

Turns a request into a complete plan: proposal, spec deltas, design and
tasks. It reads the relevant code before writing, grounds the scope in what
it finds, and reports contradictions with existing specs instead of settling
them alone. It writes the prose in the project's
[artifact language](guides/artifact-language.md).

The request authorizes planning only, even if it says "build" or "fix": the
skill stops once the artifacts are written and waits for you.

If the request mentions a ticket identifier matching `[A-Z]{2,}-\d+` and a
Jira MCP tool is configured, it fetches the first ticket — read-only, a
single call — and cites it at the top of the proposal. See
[Jira and other MCP servers](guides/mcp.md).

### Explore (`/codev-explore`)

A thinking partner for vague requests, investigations and comparisons of
approaches. It reads code and specs but creates no change, no artifact and no
file. When the idea is clear, it suggests `/codev-propose`.

### Onboard (`/codev-onboard`)

A read-only tour for someone discovering codev: what the tool does, what the
current project contains (specs, active changes, decisions), and the one
action to take next — for example `/codev-configure` on a freshly initialized
project.

### Configure (`/codev-configure`)

Reads the README, contributing guide, documentation and a sample of the
source code, then drafts a `context:` block and per-artifact `rules:` for
`_codev/config.yaml`. It shows the diff and writes only after you confirm. It
never touches `schema`, `language`, `workflows`, `mcp` or `inherits`, and it
preserves existing comments.

While the configuration has no `rules:`, `codev init` and `codev status`
recommend running it.

### Apply (`/codev-apply`)

Implements a planned change. It checks that planning is complete, then works
through each unchecked task of `tasks.md` in order, verifies it, and checks
it off. It stops at the first blocker or ambiguity. It never modifies another
change, never syncs and never archives.

### Update (`/codev-update`)

Revises an artifact that already exists — for example a requirement in a spec
delta after review — then checks the other artifacts for ripple effects and
reports them. It does not create missing artifacts (that is
`/codev-propose`), does not modify code (that is `/codev-apply`), and does not
touch archived changes. It ends with `codev validate`.

### Sync (`/codev-sync`)

Runs `codev sync` for a change and reports which main specs were created,
updated or left unchanged.

### Archive (`/codev-archive`)

Runs `codev archive` for a change. If validation reports errors, it stops and
points you to `codev validate` rather than trying to work around them.

## Choosing which skills to install

By default all eight are installed. To install fewer, pick the **minimal**
preset at init time — `propose`, `explore`, `onboard` and `configure` — or
list the workflows you want in `_codev/config.yaml`:

```yaml
workflows:
  - propose
  - explore
  - apply
  - archive
```

Then run `codev update`. An unknown workflow name produces a warning and is
ignored. `codev update` does not delete skills that are no longer listed;
remove their folders from `.claude/skills/` yourself.

## Editing a generated skill

Each generated `SKILL.md` records the codev version that produced it. If you
edit a skill by hand, `codev update` detects the edit and leaves the file in
place:

```text
  Left in place because edited by hand — rerun with --force to overwrite:
    /home/you/acme-app/.claude/skills/codev-apply/SKILL.md
```

`codev update --force` overwrites it with the generated version. Skills from
an older codev version are always regenerated.
