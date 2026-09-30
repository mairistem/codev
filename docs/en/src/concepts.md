# Concepts

codev keeps three kinds of records in the repository: **specs** say what the
system does, **decisions** say why it is built the way it is, and **changes**
are the work in progress that modifies both.

```text
_codev/
├── config.yaml     # project configuration
├── specs/          # behavior contracts — WHAT the system does
├── decisions/      # architecture decisions — WHY it is built this way
├── changes/        # work in progress, which modifies the two above
│   ├── <name>/     #   proposal.md, design.md, tasks.md, specs/** (deltas)
│   └── archive/    #   finished changes, dated
├── schemas/        # custom workflow schemas (optional)
└── codev.lock      # pinned inherited git sources (when used)
```

codev finds this folder by walking up from the current directory, so every
command works from any subdirectory of the project.

## Capability

A capability is a unit of observable behavior the system offers — user
authentication, the theme setting, the `validate` command. Capabilities are
identified by a path of kebab-case segments, such as `ui/theme` or
`identity/user-auth`, which is also where their spec lives. Group them the way
your domain is organized, not the way your code is.

## Spec

The main spec of a capability lives at `_codev/specs/<capability>/spec.md`. It
has a `## Purpose` — one or two sentences on what the capability is for — and
a `## Requirements` section:

```markdown
# Theme Specification

## Purpose

Let users choose the color theme of the interface.

## Requirements

### Requirement: Theme follows the system preference by default

The application SHALL render with the operating system's color scheme until the user picks a theme.

#### Scenario: No theme chosen yet

- **GIVEN** a user who has never changed the theme setting
- **WHEN** their operating system is set to dark mode
- **THEN** the application renders with the dark theme
```

Each requirement is normative (it uses `SHALL` or `MUST`) and has at least one
scenario describing observable behavior. A spec describes behavior, never
implementation: if the code can change without the behavior changing, the
spec should not change either.

You never edit a main spec directly. It changes only when a change is synced
or archived, which is what makes its history readable.

## Change

A change is a folder `_codev/changes/<name>/` that carries one evolution of
the system: a proposal, spec deltas, a design and a task list, plus a
`change.yaml` holding its metadata. Which artifacts a change has is defined by
its [schema](guides/custom-schemas.md); the default `spec-driven` schema is
described in [The workflow](workflow.md).

A change is **active** while it lives in `_codev/changes/<name>/`. Archiving
moves it to `_codev/changes/archive/<YYYY-MM-DD>-<name>/`, where it becomes a
record of why and how the specs changed.

## Delta

A delta is the part of a change that modifies a spec. It lives at
`_codev/changes/<name>/specs/<capability>/spec.md` — the same capability path
as the main spec it targets — and is organized in up to four operations:

| Section | Effect when merged |
|---|---|
| `## ADDED Requirements` | Appends the requirements to the main spec |
| `## MODIFIED Requirements` | Replaces the requirement of the same name, in place |
| `## REMOVED Requirements` | Deletes the requirement, with a **Reason** and a **Migration** |
| `## RENAMED Requirements` | Changes a requirement's title, leaving its body untouched |

Here is a delta that modifies one requirement and renames another:

```markdown
## MODIFIED Requirements

### Requirement: Theme follows the system preference by default

The application SHALL render with the operating system's color scheme until the user picks a theme, and SHALL switch live when that preference changes.

#### Scenario: No theme chosen yet

- **GIVEN** a user who has never changed the theme setting
- **WHEN** their operating system is set to dark mode
- **THEN** the application renders with the dark theme

## RENAMED Requirements

- FROM: The chosen theme is remembered
- TO: The chosen theme follows the user
```

A `MODIFIED` block replaces the whole requirement, so copy the entire block
from the main spec — scenarios included — before editing it. Content you leave
out is lost at merge time. If you are adding behavior without changing
existing behavior, use `ADDED`.

The merge touches only the requirements named in the delta. Everything else in
the main spec — other requirements, free-form sections, comments — stays as
it was.

A delta for a **new** capability starts with its own `## Purpose`; merging it
creates the main spec. Removing a capability entirely — a `REMOVED` delta that
empties a spec — requires `retire_capabilities: true` in `change.yaml`,
because the deletion can only be undone from git.

The exact syntax is in [File formats](reference/file-formats.md).

## Decision

A decision, or ADR (architecture decision record), is a file
`_codev/decisions/NNNN-<slug>.md` that records an architectural choice: its
context, the decision, its consequences and the alternatives considered.

```bash
codev decision new "Use PostgreSQL for persistence"
```

A decision has one of five statuses: `accepted`, `superseded`, `proposed`,
`deprecated` or `rejected`. A decision is **in effect** when it is `accepted`
and nothing supersedes it. The decisions in effect are handed to the agent
whenever it writes a `design.md`, so designs follow them instead of
re-debating them.

Each decision has a short id (`0007`) and a qualified id that names its
origin: `project/0007` for a decision of the project itself,
`path:~/shared/0100` or `git:git@github.com:acme/shared.git/0100` for a
decision inherited from another repository.

## Seal

An accepted decision is immutable. When a `codev decision` command creates an
accepted decision, it records a SHA-256 hash of the decision's body in
`_codev/decisions/seal.yaml`; `codev decision seal` does the same for a
decision you wrote or accepted by hand. `codev validate` compares the body of
every local `accepted` or `superseded` decision with its hash, and reports an
edit as a `decision_seal_mismatch` error and a missing seal as a
`decision_unsealed` warning.

The frontmatter is not covered by the seal, so a status can move from
`accepted` to `superseded` without breaking it.

> **Note**
> `codev decision new` creates an `accepted` decision by default and seals it
> immediately, with the template's placeholder text. To draft the decision
> first, create it with `--status proposed`, write it, set its status to
> `accepted`, then run `codev decision seal <ID>`. If you have already edited
> a sealed decision on purpose, `codev decision seal <ID> --force` records the
> new body.

## Supersession

To change a decision, supersede it rather than editing it:

```bash
codev decision supersede 0001 "Use SQLite for persistence"
```

codev creates a new accepted decision whose frontmatter lists
`supersedes: ["0001"]`, and marks `0001` as `superseded`. The old body stays
exactly as it was, so the history of the reasoning is preserved.

## Deviation

An [inherited](#inherited-source) decision cannot be superseded from your
project, because you cannot write to its source. Instead, record a local
deviation:

```bash
codev decision deviate path:~/shared/0100 "Services log in logfmt"
```

This creates a local accepted decision with `deviates_from` pointing at the
inherited one. The inherited decision stays visible in `codev decision list`,
but it is no longer in effect for your project and is no longer handed to the
agent when writing designs.

## Promotion

A design often contains a decision that deserves to outlive the change. Write
it in `design.md` as a `### Decision: <title>` block, then promote it:

```bash
codev decision promote add-audit-log "Append-only audit table"
```

codev creates a sealed ADR from the block's content and replaces the block's
body with a reference to the new ADR. Split the promoted text into the ADR's
sections before archiving the change.

## Inherited source

A project can inherit read-only material from other codev repositories — a
shared folder on your machine (`path:`) or a git repository pinned to a commit
(`git:`). Each source contributes its `context:`, its `rules:` and its
decisions. Nothing executable is ever inherited, and nothing is read from a
floating branch: git sources are pinned in `_codev/codev.lock`. See
[Inherited sources](guides/inherited-sources.md).

## Schema

A schema defines which artifacts a change has, what each one requires, and the
instructions and templates the agent follows to write them. codev embeds one
schema, `spec-driven`. A project can define its own in `_codev/schemas/`. See
[Custom schemas](guides/custom-schemas.md).
