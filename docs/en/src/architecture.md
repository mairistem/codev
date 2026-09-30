# Architecture

This chapter is for contributors: how the code is organized, and why. The
decisions behind it, with their context and the alternatives considered, are
recorded as ADRs in
[`_codev/decisions/`](https://github.com/mairistem/codev/tree/main/_codev/decisions)
— codev is developed with codev. This chapter summarizes and connects them.

## A pure core, an imperative shell

codev follows one principle: **deciding is not executing**.

- Everything that is pure lives in `codev-core`: the domain model, schemas,
  the artifact graph, the parsers, validation rules, merging, and the
  computation of plans. No `std::fs`, no `std::env`, no clock, no network.
- Everything that needs the outside world lives above it, behind a port.

The dependency rule is not a folder convention: it is the crate graph, and
Cargo refuses to compile a cycle. The compiler enforces it, without facade
interfaces or a dependency-injection container.

## The crate graph

```text
codev-cli ──▶ codev-agents ──▶ codev-engine ──▶ codev-core
    │                               ▲                ▲
    ├───────────────────────────────┘                │
    └────────────────────────────────────────────────┘
```

| Crate | Role | I/O |
|---|---|---|
| `codev-core` | Model, schemas, artifact graph, parsers, validation rules, merge, plans | None |
| `codev-engine` | Disk layout, configuration, schema resolution, decisions index, inherited sources, plan execution | Through ports |
| `codev-agents` | The `AgentTarget` trait and its Claude Code implementation; the workflow catalog | Through ports |
| `codev-cli` | Argument parsing, human rendering, the versioned JSON contract, prompts, embedded docs | Yes — it is the shell |

`codev-agents` also depends on `codev-core`, and `codev-cli` on every crate.

A module becomes a crate only when it has earned it — a clean test boundary,
or an external consumer. Splitting earlier only produces refactoring churn.

## Plans, not side effects

The core never touches the disk. It returns a **plan** of effects — files to
create or overwrite, files to delete, folders to move — and the shell
executes it. `init`, `new change`, `sync`, `archive`, the skill installation
and the decision commands all work this way.

This buys several things at once: the whole plan is validated before the first
write, merges are tested without temporary files, and dry runs or previews
come for free.

## Ports

There are four ports, only for effects:

| Port | Why |
|---|---|
| `FileSystem` | Makes `init`, `sync` and `archive` testable in memory |
| `Clock` | Archive and decision dates must be deterministic in tests |
| `Env` | Home directory, `XDG_CACHE_HOME`, locale variables |
| `ProcessRunner` | Driving `git` for inherited git sources |

What codev does not do: no repository trait hiding `std::fs`, no one-method
`*UseCase` structs, no `Arc<dyn Trait>` by default. Traits serve boundaries
that are genuinely open (`AgentTarget`, `Rule`), generics serve the rest, and
enums model closed sets.

## The two risks the architecture guards against

1. **A destructive Markdown rewrite.** A `MODIFIED` delta that reformats the
   file or loses content it does not mention. The parsers therefore keep the
   original byte spans of every block, so the merge rewrites one block without
   touching the rest.
2. **Drift of the JSON contract.** The JSON output is a public API, consumed
   by skills already installed on users' machines. Domain types therefore do
   not derive `Serialize`: `codev-cli` has a dedicated `contract` module whose
   types translate the domain, with tests that pin the field names. It is the
   project's only DTO boundary, and it has a reason to exist.

## Extension points

1. **`AgentTarget`** — one implementation per agent tool. Claude Code today;
   another tool can be added without touching the core.
2. **`Rule`** — one validation rule per implementation, registered in a list.
3. **Schemas, templates and workflows as data** — `schema.yaml`, the templates
   and the skill bodies live in `assets/` and are embedded at build time.
   Improving an instruction never requires touching Rust code.

## Where things live

| Path | Content |
|---|---|
| `assets/schemas/spec-driven/` | The built-in schema and its templates |
| `assets/templates/decision.md` | The ADR template |
| `assets/workflows/*.md` | The body of each skill |
| `crates/codev-agents/src/workflows.rs` | The workflow catalog: descriptions, `allowed-tools`, defaults |
| `crates/codev-cli/src/contract.rs` | The JSON contract, version 1 |
| `crates/codev-cli/build.rs` | Embeds the documentation chapters listed in `docs/<lang>/src/SUMMARY.md` |
| `_codev/` | codev's own specs, decisions and changes |

## Architecture decisions

| ADR | Decision |
|---|---|
| 0001 | Functional core, imperative shell: the core returns plans, the shell executes them |
| 0002 | The crate graph enforces the dependency rule; four ports for effects |
| 0003 | The planning root is `_codev/`, visible |
| 0004 | One identity for a skill and its slash command; no separate command files |
| 0005 | Inherited sources are read-only and pinned by commit, not shared writable stores |
| 0006 | `serde_norway` for YAML, the maintained fork of `serde_yaml` |

## Conventions

- **Errors.** `thiserror` and typed errors in library crates; `anyhow` only in
  `codev-cli`, where errors are translated into the `status` array of the JSON
  contract.
- **Comments** are in English and explain *why*. A comment that paraphrases
  the code is noise.
- **Tests.** The core is tested with pure unit tests; the engine with an
  in-memory `FileSystem` and fixed `Clock` and `Env`; the JSON contract with
  tests on its field names and shapes.
