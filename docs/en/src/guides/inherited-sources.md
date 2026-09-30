# Inherited sources

Teams that maintain several repositories usually share conventions: how
services log, which database to use, how APIs are versioned. With inherited
sources, those conventions live in one codev repository, and every project
that inherits from it receives them — without copying anything.

## What a source provides

An inherited source is another codev project, one that has its own
`_codev/config.yaml`. Your project reads three things from it:

- its **`context:`**, prepended to your own context in every artifact's
  instructions;
- its **`rules:`**, prepended to your own rules for the same artifact;
- its **decisions**, from its `_codev/decisions/` folder, which are handed to
  the agent with yours when it writes a `design.md`.

Sources are read-only. codev never writes to them, and it only ever reads
Markdown and YAML files from them: nothing executable is inherited.

When the agent receives the context and rules, every block carries its origin,
ordered from the most general (sources, in declaration order) to the most
specific (your project). When two blocks contradict each other, the last one
wins, and the skills are instructed to point out the contradiction rather than
settle it silently.

Inheritance is not transitive: if a source itself declares `inherits:`, those
sources are ignored and codev warns you. Declare every source you need
directly in your project.

## Local sources

A `path:` source is a folder on your machine — typically a clone of your
team's shared repository:

```yaml
# _codev/config.yaml
inherits:
  - path: ~/src/acme-standards
```

The path may be absolute or start with `~`. It is read directly, so edits in
that folder are visible immediately. `path:` sources are convenient while you
author shared conventions; for everyone else, prefer a pinned git source.

## Git sources

A `git:` source is a remote repository, pinned to a commit:

```yaml
inherits:
  - git: git@github.com:acme/codev-standards.git
    ref: main
```

`ref` is required: a branch or a tag. Use `subpath` when the codev project is
in a subfolder of the repository:

```yaml
inherits:
  - git: git@github.com:acme/platform.git
    ref: v2
    subpath: standards
```

A git source is used only once it is locked. Resolve and download it with:

```bash
codev sources update
```

```text
Changes:
  + git@github.com:acme/codev-standards.git @main  → 12958274cbed0eb66985f57d3272c6d3134b672d

codev.lock updated.
```

This resolves each `ref` to a commit with `git ls-remote`, fetches that
commit into a local cache, and records it in `_codev/codev.lock`:

```toml
version = 1

[[source]]
git = "git@github.com:acme/codev-standards.git"
ref = "main"
commit = "12958274cbed0eb66985f57d3272c6d3134b672d"
resolved_at = "2026-09-30"
```

**Commit `codev.lock`.** Everyone working on the project then reads exactly the
same commit, and a change of pin shows up in review like any other diff.

`codev sources update` is the only command that contacts the network. Every
other command reads the locked commit from the cache, offline. Run
`codev sources update` again to move the pins to the current commit of each
`ref`; it reports each source as added (`+`), moved (`~`, with the old and
new commits) or unchanged (`=`).

The cache lives in `$XDG_CACHE_HOME/codev/`, or `~/.cache/codev/` when
`XDG_CACHE_HOME` is not set. Git sources require `git` on your `PATH`, with
access to the repository — codev uses your existing git credentials.

## Inspecting sources

```bash
codev sources list
```

```text
Inherited sources:
  git git@github.com:acme/codev-standards.git  [locked]
      commit: 12958274cbed0eb66985f57d3272c6d3134b672d
      path:   /home/you/.cache/codev/content/12958274cbed0eb66985f57d3272c6d3134b672d
```

Each source is in one of these states:

| State | Meaning |
|---|---|
| `resolved` | `path:` source found, with a `_codev/config.yaml` |
| `unreadable` | `path:` source missing, or without a `_codev/config.yaml` |
| `unlocked` | `git:` source not in `codev.lock` yet — run `codev sources update` |
| `locked` | `git:` source pinned and present in the cache |
| `needs_update` | `git:` source pinned but missing from the cache — run `codev sources update` |

`codev sources show <TARGET>` shows one source, given by its path or URL, and
lists the files it exposes.

## Inherited decisions

Inherited decisions appear in `codev decision list` next to yours. Their
qualified id names the source they come from, as declared in your
configuration:

```text
path:~/src/acme-standards/0100
git:git@github.com:acme/codev-standards.git/0100
```

If a local decision has the same short id as an inherited one, the local one
wins and codev reports a `decision_id_collision` warning.

You cannot edit or supersede an inherited decision from your project. When
your project must depart from one, record a deviation:

```bash
codev decision deviate path:~/src/acme-standards/0100 "Services log in logfmt"
```

The new local decision references the inherited one in `deviates_from`. The
inherited decision stays listed, so the departure is visible, but it is no
longer in effect for your project. See [Deviation](../concepts.md#deviation).

## What is not inherited

- **Specs.** Each project owns its behavior specs.
- **`schema`, `language`, `workflows` and `mcp`.** These belong to the project
  and to the people working in it.
- **The source's own sources**, as explained above.
