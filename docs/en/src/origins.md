# Origins

Why Compagnon exists, what it owes to the tools before it, and where to see it at work.

## Why Compagnon

In a repository without specs, intent lives in people's heads. Code is
written on one side, reviewed on the other, and six months later someone digs
through commits to rediscover why a behavior exists. Compagnon reverses that: for
every significant change, you first write down what you want, review it, then
implement it.

What Compagnon gives you:

- **An explicit cycle.** Every change goes through propose, apply, sync and
  archive. Each step has an artifact and a dedicated Claude Code skill. See
  [The workflow](workflow.md).
- **Spec deltas, merged precisely.** A change never rewrites a spec wholesale.
  It declares `ADDED`, `MODIFIED`, `REMOVED` or `RENAMED` requirements, and
  Compagnon merges them into the main specs when the change is archived, leaving
  everything else in the file untouched. This is what makes Compagnon usable on
  existing code, not only on greenfield projects.
- **Immutable architecture decisions.** Accepted decisions (ADRs) are sealed
  with a hash of their body. `compagnon validate` reports any silent rewrite. You
  change a decision by superseding it, never by editing it.
- **Shared conventions across repositories.** A project can inherit context,
  rules and decisions from another repository, read-only, pinned to a git
  commit. See [Inherited sources](guides/inherited-sources.md).
- **Your team's language.** Artifacts are written in the language set in the
  configuration, while their structure stays machine-readable. See
  [Artifact language](guides/artifact-language.md).
- **A single native binary.** No runtime to install. Prebuilt binaries for
  macOS, Linux and Windows, verified by SHA-256 at install time.

## Origins

Compagnon is inspired by [OpenSpec](https://github.com/Fission-AI/OpenSpec) (MIT),
the TypeScript tool that popularized a spec-driven development cycle for
coding agents. Compagnon keeps its skeleton, rebuilds it in Rust, and makes its
own choices on top.

**What Compagnon owes to OpenSpec:**

- The propose → apply → archive cycle, and the idea that a *change* is the
  unit of work.
- Spec deltas with four readable operations — `ADDED`, `MODIFIED`,
  `REMOVED`, `RENAMED`.
- The notion of a *capability* as a grouping of observable behavior, rather
  than of files or modules.
- Workflows and templates kept as data, not code.

**What Compagnon does differently:**

- **A Rust binary instead of a Node.js package.** One prebuilt executable, no
  runtime.
- **A visible `_compagnon/` folder.** Planning artifacts are first-class source
  files: you read them, diff them and review them. The leading underscore keeps
  the folder visible to tools such as ripgrep and fd, which skip hidden
  directories by default.
- **A pure core and an imperative shell.** The core library performs no I/O;
  every decision it makes is testable without a disk. See
  [Architecture](architecture.md).
- **One target, done well.** Compagnon targets Claude Code only, where OpenSpec
  supports many tools.

**What is specific to Compagnon:**

- Sealed architecture decisions, with supersession, local deviations from
  inherited decisions, and promotion of a design decision into an ADR.
- Read-only inherited sources pinned by git commit in `_compagnon/compagnon.lock`.
- Skill-side MCP integration: `/compagnon-propose` detects a ticket identifier
  such as `PROJ-123` and fetches the ticket through the Jira MCP server
  configured for the project.
- A versioned JSON contract on every command, so skills can rely on a stable
  output shape.
- Documentation embedded in the binary (`compagnon docs`), readable offline.

**The name.** Compagnon comes from *compagnonnage*, the French tradition of
craft guilds: a master leads the work, the *compagnons* carry it out, and
everyone follows plans agreed beforehand. Here you are the master and the
Claude agents are your compagnons. Until 0.6 the product was named codev;
[Moving from codev](guides/moving-from-codev.md) explains how to convert a
project set up then.

## Built with Compagnon

Compagnon is developed with Compagnon: every change to its repository is planned,
reviewed and archived with the tool itself. Its `_compagnon/` folder is a
real-world example you can browse:

- [the living specs](https://github.com/mairistem/compagnon/tree/main/_compagnon/specs) describing what Compagnon does today;
- [the archived changes](https://github.com/mairistem/compagnon/tree/main/_compagnon/changes/archive), each with its proposal,
  spec deltas, design and tasks;
- [a review record](https://github.com/mairistem/compagnon/blob/main/_compagnon/changes/archive/2026-10-13-review-record/review.md), with findings, evidence, the author's answers and
  how each was settled;
- [the architecture decisions](https://github.com/mairistem/compagnon/tree/main/_compagnon/decisions), sealed.
