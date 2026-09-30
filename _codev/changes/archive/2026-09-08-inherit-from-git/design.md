# Design: inherit from a remote git repository

## Context

The `inherits: path:` layer delivered earlier covers local sharing; it can
resolve a path, read the decisions and merge them with provenance. This
change plugs the same infrastructure into a remote repository by
inserting, between the declaration and the read, a "resolution by lock"
step that never requires the network.

## Goals / Non-Goals

This design covers:

- the new transport building block (`ProcessRunner` port, calling `git`);
- the cache layout and the fetch algorithm;
- the format of `codev.lock` and its integration into `config::resolve`;
- the three `codev sources` commands and their JSON contract.

It does **not** cover the degraded "no `git` on the PATH" mode, cache
cleanup, parallelization, or other authentication protocols. Cf. the
proposal's out-of-scope list.

## Decisions

### Decision: drive the `git` binary through a `ProcessRunner` port

This is the approach that decision
[0005](../../decisions/0005-read-only-inherited-sources.md) documents
explicitly: "driving the `git` binary reuses existing authentication". A
pure-Rust library such as `gix` would require replicating the `ssh`
configuration, the *credential helpers*, the proxies; `git` already knows
all that.

The port is the fourth one announced by decision
[0001](../../decisions/0001-functional-core-imperative-shell.md):
`FileSystem`, `Clock`, `Env`, and now `ProcessRunner`. Its interface stays
minimal:

```rust
pub trait ProcessRunner {
    fn run(&self, program: &str, args: &[&str], cwd: Option<&Path>)
        -> io::Result<ProcessOutput>;
}
pub struct ProcessOutput { pub stdout: Vec<u8>, pub stderr: Vec<u8>, pub exit_code: i32 }
```

The in-memory implementation records `(program, args, cwd)` and returns a
predefined `Vec<ProcessOutput>` — which makes the tests of the
`sources update` commands deterministic without ever touching the network.

**Rejected alternative**: `gix`. Technically attractive, but a
disproportionate implementation and compatibility cost for a need covered
100% by `git`. Worth reconsidering if `git` disappears from workstations.

### Decision: SHA-addressed cache, in an XDG-compliant folder

Cache root:

- `$XDG_CACHE_HOME/codev/` if set;
- otherwise `~/.cache/codev/`.

Subtree:

```
<cache-root>/
├── git/<url-hash>/               # bare repository, cloned once per URL
└── content/<sha>/                # content extracted for a given SHA
```

The URL hash is a `sha256` truncated to 16 characters — just enough for
readability and to avoid collisions. The `content/<sha>/` is a
`git worktree` checkout on the locked SHA; two distinct sources that share
the same SHA (unlikely in practice) would share the content.

**Rationale**: follows the XDG convention already honored by the Rust
ecosystem (`cargo`, `rustup`). Rebuilding the cache is cheap, so the user
can empty `~/.cache/codev/` when in doubt.

### Decision: TOML lock, an accepted new dependency

`_codev/codev.lock` in TOML:

```toml
version = 1

[[source]]
git = "git@github.com:acme/codev-shared.git"
ref = "main"
subpath = "shared/"                  # optional
commit = "9f2c1ab77bd3…"
resolved_at = "2026-09-08T15:22:44Z"
```

Format aligned with the `<tool>.lock` convention (Cargo.lock, poetry.lock,
uv.lock). New dependency `toml = "0.8"` — a cost accepted for the
convention and for the clarity of the format for humans.

**Rejected alternative**: YAML, to stay consistent with `config.yaml`,
`schema.yaml`, `change.yaml`. Rejected because a lock is a generated file,
not a hand-written one — the TOML convention dominates in that case, and
YAML would force `serde_norway` onto a format it has no business seeing.

### Decision: lazy resolution, never at `config::resolve` time

`config::resolve` touches neither the network nor the cache. For an
`inherits: git:`, resolution consults the lock:

- entry found with a cached SHA → path resolved to
  `<cache>/content/<sha>/[subpath]`;
- entry found but SHA missing from the cache → warning
  `git_source_needs_update`; the content is not exposed;
- entry missing from the lock → warning `git_source_unlocked`; the content
  is not exposed.

This is the only way to guarantee that everyday commands stay
deterministic and offline. `codev sources update` is the single point
where the lock is written.

### Decision: `codev sources update` does everything in a `Plan`

Like the rest of the tool: the pure function `plan_sources_update(index,
lock, remote_resolutions)` produces a `SourcesUpdatePlan` with the fetches
to perform, the lock entries to write, and a readable diff. The CLI shell
executes — order: `git fetch` first (into the cache), `worktree add` next,
writing the lock last. If the user interrupts between two fetches, the
lock stays unchanged — atomicity concerns the lock, not the cache, which
can be partially populated without consequence.

### Decision: filter by `.md` and `.yaml` extension on read

The inherited-source loader SHALL present only files with the `.md` and
`.yaml` extensions, even if the repository contains others. This is the
concrete implementation of the "no inherited executable content"
principle of decision
[0005](../../decisions/0005-read-only-inherited-sources.md). A
`.sh`, a `.py`, a binary, a hook: all ignored by the index, never
executed.

Filtering happens in `codev-engine::sources::load` — a single place to
audit.

**Rejected alternative**: an allowlist of exact names. Too rigid, and
validation by extension is already used elsewhere in the code.

## Risks / Trade-offs

- **`git ls-remote` on a slow server blocks `codev sources update`**.
  → **Accepted trade-off**: the command is explicitly the place that
  touches the network; the user expects some latency.
- **Two sources sharing the same URL with different `ref`s clone the bare
  repository only once but create two worktrees**.
  → **Correct**: the fetch populates the bare repository, and each
  worktree points to its own SHA. An invariant test checks this.
- **Corrupted cache** (SHA locked but `content/<sha>/` folder incomplete
  because a previous command was interrupted).
  → **Mitigation**: the folder's presence is not taken as sufficient
  proof; the loader checks that it contains at least a `_codev/` or a
  `.md` — otherwise a `cache_incomplete` warning invites the user to rerun
  `sources update`.
- **SSH URLs with `~` in the path** are not affected: nothing is expanded
  in a git URL; it is passed as is to `git`. The URL hash is computed on
  the exact string.

## Migration Plan

Not applicable — this is the first version. A project that had declared
`inherits: git:` previously received a warning; it will now see either the
content (if a `codev sources update` has been run) or the new
`git_source_unlocked` warning, which tells it exactly what to run.
