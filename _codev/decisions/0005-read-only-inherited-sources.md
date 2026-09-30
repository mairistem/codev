---
id: 0005
title: Read-only inherited sources, pinned by SHA — not stores
status: accepted
date: 2026-09-08
tags: [sharing, security]
---

## Context

Several projects need to share conventions, specs and architecture
decisions. OpenSpec addresses this with *stores*: a standalone planning
repository, registered by hand on the machine, readable **and** writable
through `--store <id>` on every command. Three drawbacks for our use: a
checkout has to be maintained, the tool never synchronizes (so one can read
stale content without knowing it), and the flag contaminates root resolution
for every command.

## Decision

A separate, **read-only** mechanism: inherited sources.

```yaml
inherits:
  - path: ~/codev/shared/back
  - git: git@github.com:acme/codev-decisions.git
    ref: main
    subpath: shared/
```

- Merge precedence: `git` → local path → project, the project always winning.
- The `ref` is resolved to a SHA and **pinned** in `_codev/codev.lock`,
  versioned with the project. Nothing is ever read from a floating branch at
  run time.
- Git transport: a *bare* repository in a cache, fed by the `git` binary
  through the `ProcessRunner` port (`ls-remote`, then `fetch --depth 1
  --filter=blob:none`, then `archive`). Never a working copy.
- The provenance of each inherited block is exposed in
  `codev instructions --json`.

Three safeguards, because this content **is injected into the agent's
prompt** — whoever controls the shared repository controls part of Claude
Code's instructions on every inheriting project:

1. mandatory pinning by SHA;
2. `codev sources update` is the only command that moves a pin, and it shows
   the diff of the inherited content before writing;
3. no executable inherited content — markdown and declarative YAML only,
   never a hook or a script.

## Consequences

- `--store` exists on no command, and root resolution stays "walk up until
  `_codev/`".
- Offline reading as soon as the SHA is cached.
- Driving the `git` binary reuses the user's existing authentication (ssh key,
  credential helper), so private GitHub, self-hosted GitLab and Azure DevOps
  work without per-forge token handling.
- Accepted cost: a dependency on the `git` binary for remote sources. It is an
  implementation behind a port, replaceable by `gix` later.

## Alternatives considered

- **OpenSpec's stores.** Writing into a shared repository from a project blurs
  the ownership of the shared content, and the checkout to maintain is
  precisely the chore we want to remove.
- **`git archive --remote` alone**, without a cache. GitHub disables
  `upload-archive` on the server side, so it does not work where we need it.
- **Forge APIs** (`/repos/.../contents`). Vendor-specific, and they would
  require per-forge token handling, while git already does it.
