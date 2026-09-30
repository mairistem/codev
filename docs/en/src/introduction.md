# codev

**Spec-driven development for Claude Code.**

codev adds a thin layer of specifications to a repository, so that you and
your coding agent agree on what must be built before a line of code is
written — and so that architecture decisions, once made, stop being
re-debated on every change.

It has two halves:

| Where | What | Examples |
|---|---|---|
| Your terminal | The `codev` binary: the engine | `codev init`, `codev status`, `codev archive` |
| The Claude Code chat | Generated skills: the steering wheel | `/codev-propose`, `/codev-apply` |

`codev init` installs the skills into `.claude/skills/`. From then on you work
mostly in the chat, and the skills drive the CLI for you.

codev never calls a language model. It manages Markdown files, the
dependency graph between planning artifacts, validation, and the merging of
spec changes. Your agent does the writing; codev tells it what to write,
where, and under which constraints.

## Why codev

In a repository without specs, intent lives in people's heads. Code is
written on one side, reviewed on the other, and six months later someone digs
through commits to rediscover why a behavior exists. codev reverses that: for
every significant change, you first write down what you want, review it, then
implement it.

What codev gives you:

- **An explicit cycle.** Every change goes through propose, apply, sync and
  archive. Each step has an artifact and a dedicated Claude Code skill. See
  [The workflow](workflow.md).
- **Spec deltas, merged precisely.** A change never rewrites a spec wholesale.
  It declares `ADDED`, `MODIFIED`, `REMOVED` or `RENAMED` requirements, and
  codev merges them into the main specs when the change is archived, leaving
  everything else in the file untouched. This is what makes codev usable on
  existing code, not only on greenfield projects.
- **Immutable architecture decisions.** Accepted decisions (ADRs) are sealed
  with a hash of their body. `codev validate` reports any silent rewrite. You
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

codev is inspired by [OpenSpec](https://github.com/Fission-AI/OpenSpec) (MIT),
the TypeScript tool that popularized a spec-driven development cycle for
coding agents. codev keeps its skeleton, rebuilds it in Rust, and makes its
own choices on top.

**What codev owes to OpenSpec:**

- The propose → apply → archive cycle, and the idea that a *change* is the
  unit of work.
- Spec deltas with four readable operations — `ADDED`, `MODIFIED`,
  `REMOVED`, `RENAMED`.
- The notion of a *capability* as a grouping of observable behavior, rather
  than of files or modules.
- Workflows and templates kept as data, not code.

**What codev does differently:**

- **A Rust binary instead of a Node.js package.** One prebuilt executable, no
  runtime.
- **A visible `_codev/` folder.** Planning artifacts are first-class source
  files: you read them, diff them and review them. The leading underscore keeps
  the folder visible to tools such as ripgrep and fd, which skip hidden
  directories by default.
- **A pure core and an imperative shell.** The core library performs no I/O;
  every decision it makes is testable without a disk. See
  [Architecture](architecture.md).
- **One target, done well.** codev targets Claude Code only, where OpenSpec
  supports many tools.

**What is specific to codev:**

- Sealed architecture decisions, with supersession, local deviations from
  inherited decisions, and promotion of a design decision into an ADR.
- Read-only inherited sources pinned by git commit in `_codev/codev.lock`.
- Skill-side MCP integration: `/codev-propose` detects a ticket identifier
  such as `PROJ-123` and fetches the ticket through the Jira MCP server
  configured for the project.
- A versioned JSON contract on every command, so skills can rely on a stable
  output shape.
- Documentation embedded in the binary (`codev docs`), readable offline.

## Where to go next

- New to codev? Start with [Installation](installation.md), then the
  [Quickstart](quickstart.md).
- Want the mental model? Read [The workflow](workflow.md) and
  [Concepts](concepts.md).
- Looking for a flag or a key? See the [CLI reference](reference/cli.md) and
  the [configuration reference](reference/configuration.md).
