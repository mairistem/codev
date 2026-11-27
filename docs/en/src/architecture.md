# Architecture

This chapter is for contributors: how the code is organized, and why. The
decisions behind it, with their context and the alternatives considered, are
recorded as ADRs in
[`_compagnon/decisions/`](https://github.com/mairistem/compagnon/tree/main/_compagnon/decisions)
— Compagnon is developed with Compagnon. This chapter summarizes and connects them.

## A pure core, an imperative shell

Compagnon follows one principle: **deciding is not executing**.

- Everything that is pure lives in `compagnon-core`: the domain model, schemas,
  the artifact graph, the parsers, validation rules, merging, and the
  computation of plans. No `std::fs`, no `std::env`, no clock, no network.
- Everything that needs the outside world lives above it, behind a port.

The dependency rule is not a folder convention: it is the crate graph, and
Cargo refuses to compile a cycle. The compiler enforces it, without facade
interfaces or a dependency-injection container.

## The crate graph

<figure class="diagram">
<div class="scroll">
<svg viewBox="0 0 760 330" role="img" aria-label="The crate graph, an arrow meaning depends on: compagnon-cli, the shell, depends on compagnon-agents, compagnon-engine and compagnon-core; compagnon-agents, the Claude Code target, depends on compagnon-engine and compagnon-core; compagnon-engine, the disk through ports, depends on compagnon-core; compagnon-core, pure, depends on no other crate; compagnon-evals, the role evaluations, a workspace member outside the binary, depends on compagnon-core only.">
<defs><marker id="dg-architecture-arrow" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" style="fill:var(--dg-line)"/></marker></defs>
<text x="8" y="22" font-size="13" style="fill:var(--dg-muted)">An arrow means “depends on”</text>
<rect x="8" y="120" width="150" height="58" rx="10" style="fill:var(--dg-box);stroke:var(--dg-line)" stroke-width="2"/>
<text x="83" y="144" text-anchor="middle" class="mono" font-size="14" font-weight="600" style="fill:var(--dg-ink)">compagnon-cli</text>
<text x="83" y="166" text-anchor="middle" font-size="12" style="fill:var(--dg-muted)">the shell</text>
<rect x="206" y="120" width="150" height="58" rx="10" style="fill:var(--dg-box);stroke:var(--dg-line)" stroke-width="2"/>
<text x="281" y="144" text-anchor="middle" class="mono" font-size="14" font-weight="600" style="fill:var(--dg-ink)">compagnon-agents</text>
<text x="281" y="166" text-anchor="middle" font-size="12" style="fill:var(--dg-muted)">Claude Code target</text>
<rect x="404" y="120" width="150" height="58" rx="10" style="fill:var(--dg-box);stroke:var(--dg-line)" stroke-width="2"/>
<text x="479" y="144" text-anchor="middle" class="mono" font-size="14" font-weight="600" style="fill:var(--dg-ink)">compagnon-engine</text>
<text x="479" y="166" text-anchor="middle" font-size="12" style="fill:var(--dg-muted)">disk, through ports</text>
<rect x="602" y="120" width="150" height="58" rx="10" style="fill:var(--dg-box);stroke:var(--dg-line)" stroke-width="2"/>
<text x="677" y="144" text-anchor="middle" class="mono" font-size="14" font-weight="600" style="fill:var(--dg-ink)">compagnon-core</text>
<text x="677" y="166" text-anchor="middle" font-size="12" style="fill:var(--dg-muted)">pure, no I/O</text>
<line x1="160" y1="149" x2="204" y2="149" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-architecture-arrow)"/>
<line x1="358" y1="149" x2="402" y2="149" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-architecture-arrow)"/>
<line x1="556" y1="149" x2="600" y2="149" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-architecture-arrow)"/>
<path d="M50,118 V44 H710 V116" fill="none" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-architecture-arrow)"/>
<path d="M110,118 V80 H479 V116" fill="none" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-architecture-arrow)"/>
<path d="M281,180 V216 H660 V182" fill="none" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-architecture-arrow)"/>
<rect x="602" y="256" width="150" height="58" rx="10" style="fill:var(--dg-file);stroke:var(--dg-line)" stroke-width="2" stroke-dasharray="6 4"/>
<text x="677" y="280" text-anchor="middle" class="mono" font-size="14" font-weight="600" style="fill:var(--dg-ink)">compagnon-evals</text>
<text x="677" y="302" text-anchor="middle" font-size="12" style="fill:var(--dg-muted)">role evaluations</text>
<line x1="720" y1="254" x2="720" y2="182" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-architecture-arrow)"/>
<text x="588" y="280" text-anchor="end" font-size="12" style="fill:var(--dg-muted)">Workspace member, never built</text>
<text x="588" y="298" text-anchor="end" font-size="12" style="fill:var(--dg-muted)">into the compagnon binary</text>
</svg>
</div>
<figcaption>The crate graph: every arrow points towards compagnon-core, which depends on no other crate, and Cargo refuses to compile a cycle.</figcaption>
</figure>

| Crate | Role | I/O |
|---|---|---|
| `compagnon-core` | Model, schemas, artifact graph, parsers, validation rules, merge, plans | None |
| `compagnon-engine` | Disk layout, configuration, schema resolution, decisions index, inherited sources, plan execution | Through ports |
| `compagnon-agents` | The `AgentTarget` trait and its Claude Code implementation; the workflow catalog | Through ports |
| `compagnon-cli` | Argument parsing, human rendering, the versioned JSON contract, prompts, embedded docs | Yes — it is the shell |

The fifth workspace member, `compagnon-evals`, scores the reviewing roles
(see [Reliability of the roles](reliability.md)). It depends on
`compagnon-core` only and is never built into the released binary.

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

What Compagnon does not do: no repository trait hiding `std::fs`, no one-method
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
   not derive `Serialize`: `compagnon-cli` has a dedicated `contract` module whose
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
| `crates/compagnon-agents/src/workflows.rs` | The workflow catalog: descriptions, `allowed-tools`, defaults |
| `crates/compagnon-cli/src/contract.rs` | The JSON contract, version 1 |
| `crates/compagnon-cli/build.rs` | Embeds the documentation chapters listed in `docs/<lang>/src/SUMMARY.md` |
| `_compagnon/` | Compagnon's own specs, decisions and changes |

## Architecture decisions

| ADR | Decision |
|---|---|
| 0001 | Functional core, imperative shell: the core returns plans, the shell executes them |
| 0002 | The crate graph enforces the dependency rule; four ports for effects |
| 0003 | The planning root is `_compagnon/`, visible |
| 0004 | One identity for a skill and its slash command; no separate command files |
| 0005 | Inherited sources are read-only and pinned by commit, not shared writable stores |
| 0006 | `serde_norway` for YAML, the maintained fork of `serde_yaml` |

## Conventions

- **Errors.** `thiserror` and typed errors in library crates; `anyhow` only in
  `compagnon-cli`, where errors are translated into the `status` array of the JSON
  contract.
- **Comments** are in English and explain *why*. A comment that paraphrases
  the code is noise.
- **Tests.** The core is tested with pure unit tests; the engine with an
  in-memory `FileSystem` and fixed `Clock` and `Env`; the JSON contract with
  tests on its field names and shapes.
