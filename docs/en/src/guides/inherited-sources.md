# Inherited sources

Teams that maintain several repositories usually share conventions: how
services log, which database to use, how APIs are versioned. With inherited
sources, those conventions live in one Compagnon repository, and every project
that inherits from it receives them — without copying anything.

## What a source provides

An inherited source is another Compagnon project, one that has its own
`_compagnon/config.yaml`. Your project reads three things from it:

- its **`context:`**, prepended to your own context in every artifact's
  instructions;
- its **`rules:`**, prepended to your own rules for the same artifact;
- its **decisions**, from its `_compagnon/decisions/` folder, which are handed to
  the agent with yours when it writes a `design.md`.

Sources are read-only. Compagnon never writes to them, and it only ever reads
Markdown and YAML files from them: nothing executable is inherited.

When the agent receives the context and rules, every block carries its origin,
ordered from the most general (sources, in declaration order) to the most
specific (your project). When two blocks contradict each other, the last one
wins, and the skills are instructed to point out the contradiction rather than
settle it silently.

Inheritance is not transitive: if a source itself declares `inherits:`, those
sources are ignored and Compagnon warns you. Declare every source you need
directly in your project.

## Declaring sources at init

The quickest way to start a project on shared conventions is to name the
sources when you initialize it, once per `--inherit`:

```bash
compagnon init --yes --inherit /home/you/acme-standards.git@main
```

```text
compagnon initialized in /home/you/acme-api

  Structure   _compagnon/ — specs, decisions, changes, schemas
  Skills      compagnon-propose, compagnon-explore, compagnon-onboard, compagnon-apply, compagnon-sync, compagnon-archive, compagnon-update, compagnon-configure → .claude/skills/
  Inherits    /home/you/acme-standards.git @main → pinned 8dc995327621
…
```

`compagnon init` writes them under `inherits:` in the new `_compagnon/config.yaml`:

```yaml
# set with compagnon init --inherit
inherits:
  - git: /home/you/acme-standards.git
    ref: main
```

A value starting with `git@`, `https://`, `http://`, `ssh://` or `file://`, or
ending in `.git`, is a git source; `@<ref>` picks the branch or tag, and `main`
is used without it. Anything else is a local folder, which must exist; a
relative folder is written as an absolute path, with forward slashes on every
platform, so the file reads the same whatever machine ran `compagnon init`. A
value that is obviously wrong — empty, a missing folder, a git URL ending in
`@` — is refused with `invalid_inherit` before anything is written.

Git sources are pinned right away, exactly as `compagnon sources update` would.
When that fails — no network, missing credentials, a `ref` that does not
exist — the project is still initialized and the source still declared, but
left unpinned:

```text
warning: git source `https://git.example.com/acme/compagnon-standards.git` has no locked SHA — run `compagnon sources update` to resolve it
warning: the inherited git source(s) could not be pinned: https://git.example.com/acme/compagnon-standards.git is invalid: ls-remote for `https://git.example.com/acme/compagnon-standards.git` failed: fatal: unable to access 'https://git.example.com/acme/compagnon-standards.git/': Could not resolve host: git.example.com
help: run compagnon sources update
```

Fix the cause, then run `compagnon sources update`. `compagnon init` never moves an
existing pin and never edits an existing `config.yaml`: on an initialized
project, add sources under `inherits:` by hand.

## Local sources

A `path:` source is a folder on your machine — typically a clone of your
team's shared repository:

```yaml
# _compagnon/config.yaml
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
  - git: git@github.com:acme/compagnon-standards.git
    ref: main
```

`ref` is required: a branch or a tag. Use `subpath` when the Compagnon project is
in a subfolder of the repository:

```yaml
inherits:
  - git: git@github.com:acme/platform.git
    ref: v2
    subpath: standards
```

A git source is used only once it is locked. Resolve and download it with:

```bash
compagnon sources update
```

```text
Changes:
  + git@github.com:acme/compagnon-standards.git @main  → 12958274cbed0eb66985f57d3272c6d3134b672d

compagnon.lock updated.
```

This resolves each `ref` to a commit with `git ls-remote`, fetches that
commit into a local cache, and records it in `_compagnon/compagnon.lock`:

```toml
version = 1

[[source]]
git = "git@github.com:acme/compagnon-standards.git"
ref = "main"
commit = "12958274cbed0eb66985f57d3272c6d3134b672d"
resolved_at = "2026-09-30"
```

**Commit `compagnon.lock`.** Everyone working on the project then reads exactly the
same commit, and a change of pin shows up in review like any other diff.

`compagnon sources update` and `compagnon init --inherit` are the only commands that
contact the network, and only `compagnon sources update` moves a pin. Every other
command reads the locked commit from the cache, offline. Run
`compagnon sources update` again to move the pins to the current commit of each
`ref`; it reports each source as added (`+`), moved (`~`, with the old and
new commits) or unchanged (`=`).

The cache lives in `$XDG_CACHE_HOME/compagnon/`, or `~/.cache/compagnon/` when
`XDG_CACHE_HOME` is not set. Git sources require `git` on your `PATH`, with
access to the repository — Compagnon uses your existing git credentials.

## Inspecting sources

```bash
compagnon sources list
```

```text
Inherited sources:
  git git@github.com:acme/compagnon-standards.git  [locked]
      commit: 12958274cbed0eb66985f57d3272c6d3134b672d
      path:   /home/you/.cache/compagnon/content/12958274cbed0eb66985f57d3272c6d3134b672d
```

Each source is in one of these states:

| State | Meaning |
|---|---|
| `resolved` | `path:` source found, with a `_compagnon/config.yaml` |
| `unreadable` | `path:` source missing, or without a `_compagnon/config.yaml` |
| `unlocked` | `git:` source not in `compagnon.lock` yet — run `compagnon sources update` |
| `locked` | `git:` source pinned and present in the cache |
| `needs_update` | `git:` source pinned but missing from the cache — run `compagnon sources update` |

`compagnon sources show <TARGET>` shows one source, given by its path or URL, and
lists the files it exposes.

## Inherited decisions

Inherited decisions appear in `compagnon decision list` next to yours. Their
qualified id names the source they come from, as declared in your
configuration:

```text
path:~/src/acme-standards/0100
git:git@github.com:acme/compagnon-standards.git/0100
```

If a local decision has the same short id as an inherited one, the local one
wins and Compagnon reports a `decision_id_collision` warning.

You cannot edit or supersede an inherited decision from your project. When
your project must depart from one, record a deviation:

```bash
compagnon decision deviate path:~/src/acme-standards/0100 "Services log in logfmt"
```

The new local decision references the inherited one in `deviates_from`. It is
created `proposed` and unsealed: write why your project departs, then accept it,
which seals it:

```bash
compagnon decision accept 0003
```

The deviation takes effect only once accepted. Until then, the inherited
decision stays in effect and is still injected into the `design` instructions.
Once the deviation is accepted, the inherited decision stays listed, so the
departure is visible, but it is no longer in effect for your project. See
[Deviated from](decisions.md#deviated-from).

## What is not inherited

- **Specs.** Each project owns its behavior specs.
- **`schema`, `language`, `workflows` and `mcp`.** These belong to the project
  and to the people working in it.
- **The source's own sources**, as explained above.
