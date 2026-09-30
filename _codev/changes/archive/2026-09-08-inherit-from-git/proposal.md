# Proposal: inherit from a remote git repository

## Why

Since the start of the project, the promise has been: "decisions and
conventions are shared across projects of the same organization, without
any project having to clone a repository next to it". The local half is
already there — a project can inherit from another via `inherits: path:`.
The remote half is not: `inherits: git:` emits an `inherit_git_unsupported`
warning and stops there. This change delivers the real transport, pinned
by SHA, read-only, aligned with decision
[0005](../../decisions/0005-read-only-inherited-sources.md).

## What Changes

- **`git:` subkey of `inherits`** — currently accepted by the
  configuration but blocked by a warning; becomes functional. A typical
  entry:

  ```yaml
  inherits:
    - git: git@github.com:my-org/codev-decisions.git
      ref: main
      subpath: shared/           # optional
  ```

- **New `ProcessRunner` port** in `codev-engine`, with a real
  implementation that runs `git` via `std::process::Command` and an
  in-memory implementation for tests. This is the fourth port announced by
  the architecture — this change is its first consumer.

- **SHA-addressed cache** under `~/.cache/codev/`
  (honoring `XDG_CACHE_HOME`): one *bare* repository per URL in
  `git/<url-hash>/`, the content extracted per SHA in `content/<sha>/`.
  Reads are fully offline as soon as the SHA is cached.

- **Lock file `_codev/codev.lock`** versioned with the project, in TOML
  format — like the `Cargo.lock` convention. Each `git:` entry carries the
  URL, the requested `ref`, the `subpath`, the resolved SHA, and the
  resolution time. `codev sources update` is **the only command that
  moves a pin**.

- **Lazy resolution in `config::resolve`**: when an `inherits: git:` is
  declared, `resolve` looks up the matching entry in the lock. Found → the
  resolved path points to the cache content for the locked SHA. Missing →
  a `git_source_unlocked` warning inviting the user to run
  `codev sources update`. Nothing is fetched at run time; `sources update`
  is what touches the network, nothing else.

- **Three new `codev sources` commands**:
  - `codev sources list [--json]`: lists all declared sources with their
    state (local resolved, git locked, git unlocked);
  - `codev sources update [--json]`: resolves each `git:` via
    `git ls-remote`, downloads if the SHA is new, shows the diff of SHA
    changes before writing the lock, writes;
  - `codev sources show <ref> [--json]`: details of a source (URL, ref,
    locked SHA, path in the cache, inherited content).

- **Safeguards** — decision 0005 states them; they become concrete:
  - the SHA is mandatory, never read from a floating branch at run time;
  - only `.md` and `.yaml` files are exposed to consumers (decisions, main
    specs, inherited config) — no binary, no hook, no script is ever
    loaded from an inherited source.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `decisions` — inherited decisions can now come from a git repository in
  addition to a local folder. `origin` gains a new `git:<url>` form,
  exposed in the JSON contract. The index logic and the injection into
  `design` remain identical.

## Impact

- **Code**: new `ProcessRunner` port (`codev-engine::ports`), new module
  `codev-engine::sources` with the resolution and cache functions, new
  module `codev-engine::lockfile` for reading/writing the TOML, extension
  of `codev-engine::config` to use the lock, new group
  `codev-cli::commands::sources` with its human renderings and its
  versioned JSON shapes.
- **Dependencies**: new dependency `toml = "0.8"` for the lock —
  consistent with the `<tool>.lock` convention (`Cargo.lock`,
  `poetry.lock`, `uv.lock`). Rejected alternative (keep YAML for
  everything) in the design.
- **External binary**: `git` must be on the PATH for
  `codev sources update`. Missing → `codev sources update` fails with a
  stable code `git_not_found` and an explicit message. The other commands
  need neither `git` nor the network.
- **Out of scope**:
  - **codeload HTTP "tarball" support** (`https://codeload.github.com/…`) —
    useful for a degraded case without `git` installed, to be raised if
    needed.
  - **Lazy extraction via `git archive` without a full clone** — the MVP
    implementation does a `git fetch --depth 1 --filter=blob:none` followed
    by a `git worktree add` in the cache; switching to `git archive` may
    come after real-world feedback.
  - **Automatic cache cleanup** — the cache grows as SHAs change; a
    `codev sources gc` command (or equivalent) will be useful later but
    not here.
  - **Writable git sources** — decision 0005: read-only, period.
  - **Authentication other than `git`'s** — no PAT token handling, no
    separate HTTP proxy. Whatever `git` can do on the user's machine, we
    do; the rest, we don't.
  - **Parallel fetching of several sources** — sequential in this change,
    parallelizable later if the need arises.
