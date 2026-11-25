# Concepts

Compagnon keeps three kinds of records in your project's
[repository](glossary.md#repository), next to
the code:

- **specs** say what the system does;
- **decisions** say why it is built the way it is;
- **changes** are the work in progress that modifies the specs and the
  decisions.

Here is where each one lives:

```text
_compagnon/
├── config.yaml     # project configuration
├── specs/          # behavior contracts — WHAT the system does
├── decisions/      # architecture decisions — WHY it is built this way
├── changes/        # work in progress, which modifies the two above
│   ├── <name>/     #   proposal.md, design.md, tasks.md, specs/** (deltas)
│   └── archive/    #   finished changes, dated
├── schemas/        # custom workflow schemas (optional)
└── compagnon.lock      # pinned inherited git sources (when used)
```

You can run every Compagnon command from any folder inside the project.
Compagnon looks upward, folder by folder, until it finds `_compagnon/`.

## Capability

A [capability](glossary.md#capability) is one thing the system does that
someone can see from the outside. For example: signing users in, the theme
setting, or the `validate` command.

Each capability has a name made of parts separated by slashes, such as
`ui/theme` or `identity/user-auth`. Each part is written in
[kebab-case](glossary.md#kebab-case): lowercase words joined by hyphens. The
name is also the folder where the capability's spec is stored.

Group your capabilities the way your business is organized, not the way your
code is.

## Spec

Each capability has one spec, stored in
`_compagnon/specs/<capability>/spec.md`. It is the project's reference for
what that capability does today. It has two sections:

- `## Purpose`: one or two sentences on what the capability is for;
- `## Requirements`: the rules the capability follows.

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

Each requirement is a firm rule: it uses `SHALL` or `MUST` (`DOIT` or
`DOIVENT` in a spec written in French). Each requirement has at least one
scenario: a short example of what someone can see happen. In the example
above, when the computer is set to dark mode, the application shows the dark
theme.

A spec describes what the system does, never how the code does it. A simple
test: if the code can change without the behavior changing, the spec should
not change either.

You never edit the project's specs directly. They change only when a change
is synced or archived. That way, every edit to a spec comes with the change
that explains it, and its history stays readable.

## Change

A change is one piece of work on the system, in its own folder
`_compagnon/changes/<name>/`. It holds:

- a proposal: why the work is needed and what it changes;
- the planned edits to the specs, called [deltas](glossary.md#delta);
- a design: how the work will be done;
- a task list;
- a `change.yaml` file, which holds the change's details (its metadata).

These documents are the change's [artifacts](glossary.md#artifact): the
parts of its plan. The [schema](glossary.md#schema) sets which parts the
plan holds: see [Custom schemas](guides/custom-schemas.md). The default
schema is called `spec-driven`. It also sets the steps a change goes
through, the [workflow](glossary.md#workflow), described in
[Step by step](workflow.md).

A change is **active** while its folder is in `_compagnon/changes/<name>/`.
Archiving moves the folder to
`_compagnon/changes/archive/<YYYY-MM-DD>-<name>/`. There, it stays as a
record of why and how the specs changed.

## Delta

A delta is the part of a change that edits one spec. It lives at
`_compagnon/changes/<name>/specs/<capability>/spec.md`, under the same
capability name as the project's spec it edits. It holds up to four kinds of
edit, each in its own section:

| Section | What it does when merged into the project's spec |
|---|---|
| `## ADDED Requirements` | Adds the requirements at the end of the spec |
| `## MODIFIED Requirements` | Replaces the requirement with the same name, where it stands |
| `## REMOVED Requirements` | Deletes the requirement, giving a **Reason** and a **Migration** (what to do instead) |
| `## RENAMED Requirements` | Changes a requirement's title, without touching its text |

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

Be careful with `MODIFIED`: it replaces the whole requirement. First copy the
entire requirement from the project's spec, scenarios included, then edit the
copy. Anything you leave out is lost when the delta is merged. If you only
add new behavior, without changing what exists, use `ADDED`.

The merge touches only the requirements the delta names. Everything else in
the project's spec stays as it was: other requirements, free-form sections,
comments.

A delta for a **new** capability starts with its own `## Purpose`. Merging it
creates the capability's spec.

You can also remove a capability entirely, with a `REMOVED` delta that
empties its spec. This needs `retire_capabilities: true` in `change.yaml`.
This safety switch exists because only [git](glossary.md#git), the
project's version history, can undo the deletion.

The exact syntax is in [File formats](reference/file-formats.md).

## Decision

A decision, or ADR (architecture decision record), is a file in
`_compagnon/decisions/`, named with a number and a short title
(`NNNN-<slug>.md`). It records a choice about how the
system is built: what led to it (the context), the choice itself, its
consequences, and the other options considered.

```bash
compagnon decision new "Use PostgreSQL for persistence"
compagnon decision accept 0001
```

A decision has one of five statuses: `accepted`, `superseded`, `proposed`,
`deprecated` or `rejected`.

Every command that creates a decision creates it `proposed`:
`compagnon decision new`, [`supersede`](#supersession),
[`deviate`](#deviation) and [`promote`](#promotion). While it is `proposed`,
you can write and rewrite it freely. Once its text is final,
`compagnon decision accept` makes it `accepted`.

Accepting is the only step that puts a decision into effect. A `proposed`
decision does not replace or set aside any other decision yet. A decision is
**in effect** when it is `accepted` and no other decision replaces it.

The [agent](glossary.md#agent) receives the decisions in effect every time it writes a
`design.md`. So designs follow them instead of debating them again.

Each decision has a short id, such as `0007`, and a full id that also says
where the decision comes from: your project, or another repository it
inherits from.

<details>
<summary>Under the hood: the full id of a decision</summary>

The qualified id names the decision's origin: `project/0007` for a decision
of the project itself, `path:~/shared/0100` or
`git:git@github.com:acme/shared.git/0100` for a decision inherited from
another repository.

</details>

## Seal

An accepted decision must not be edited afterwards. Compagnon checks this
with a [seal](glossary.md#seal): a fingerprint of the decision's text, recorded when the decision
is accepted. If someone edits the text later, `compagnon validate` sees that
the fingerprint no longer matches and reports it. It also warns about an
accepted decision that has no seal. A `proposed` decision is not sealed.

The seal does not cover the decision's [frontmatter](glossary.md#frontmatter),
the short header at the top of the file that holds its status. So the status
can move from `accepted` to `superseded` without breaking the seal.

> **Note**
> Seal a decision when its text is final: write the `proposed` decision
> first, then run `compagnon decision accept <ID>`, which sets the status and
> records the seal in one step. To change what an accepted decision says,
> [supersede](#supersession) it instead of editing it. If you have edited a
> sealed decision on purpose, `compagnon decision seal <ID> --force` records
> the new text.

<details>
<summary>Under the hood: how the seal is recorded and checked</summary>

When a decision becomes accepted through a `compagnon decision` command —
`accept`, or `new --status accepted` — Compagnon records a SHA-256 hash of the
decision's body in `_compagnon/decisions/seal.yaml`. `compagnon decision seal`
does the same for a decision you accepted by hand. `compagnon validate`
compares the body of every local `accepted` or `superseded` decision with its
hash, and reports an edit as a `decision_seal_mismatch` error and a missing
seal as a `decision_unsealed` warning.

</details>

## Supersession

To change a decision, do not edit it: replace it with a new one. Compagnon
calls this superseding it.

```bash
compagnon decision supersede 0001 "Use SQLite for persistence"
```

Compagnon creates a new `proposed` decision whose frontmatter names the one
it replaces: `supersedes: ["0001"]`. It does not touch `0001`, which stays
`accepted`, and in effect, while you write the new one. When you accept the
new decision, `0001` is marked `superseded` in the same step:

```bash
compagnon decision accept 0002
```

The old decision's text stays exactly as it was. So its seal stays valid, and
the history of the reasoning is kept.

Only an `accepted` decision can be replaced. If another decision replaced
`0001` in the meantime, `compagnon decision accept` refuses, with the error
`predecessor_not_accepted`, and writes nothing.

## Deviation

Some decisions come from another repository: they are
[inherited](#inherited-source). You cannot replace one of those from your
project, because you cannot write to its source. Instead, record a local
exception, called a deviation:

```bash
compagnon decision deviate path:~/shared/0100 "Services log in logfmt"
```

This creates a `proposed` decision in your project. Its `deviates_from` field
points at the inherited decision.

The deviation takes effect when you accept it with
`compagnon decision accept`. From then on, the inherited decision still
appears in `compagnon decision list`, but it no longer applies to your
project, and the agent no longer receives it when writing designs. Until you
accept the deviation, the inherited decision still applies.

## Promotion

While a change is being planned, its design often makes a choice that
deserves to outlive the change. Write that choice in `design.md` as a
`### Decision: <title>` block, then promote it to a decision of its own:

```bash
compagnon decision promote add-audit-log "Append-only audit table"
```

Compagnon creates a `proposed` ADR. Its `## Decision` section holds the
block's text, word for word. In `design.md`, the block's text is replaced by
a reference to the new ADR.

Then reread the ADR and spread the promoted text across its sections
(context, decision, consequences). Finally, accept it with
`compagnon decision accept`, which seals it. Ideally, do this before
archiving the change.

## Inherited source

A project can reuse material from other Compagnon repositories, without
being able to change it. A source can be:

- a shared folder on your computer (`path:`);
- a git repository, pinned to one precise [commit](glossary.md#commit)
  (`git:`).

Each source brings its `context:`, its `rules:` and its decisions. Nothing
that runs as a program is ever inherited. Sources taken from a git repository
are frozen at one exact version, so they never change without you knowing
(that version is recorded in `_compagnon/compagnon.lock`).
See [Inherited sources](guides/inherited-sources.md).

## Schema

The schema defines which artifacts a change has, what each one must contain,
and the instructions and templates the agent follows to write them.
Compagnon comes with one schema, `spec-driven`. A project can define its own
in `_compagnon/schemas/`. See [Custom schemas](guides/custom-schemas.md).

In `spec-driven`, each part of the plan has a responsible
[role](glossary.md#role), and a list of criteria ("Done when") that say when
it is finished:

- the product owner, for the proposal (why and what);
- the quality (QA) analyst, for the specs (behavior someone can observe);
- the architect, for the design (how);
- the tech lead, for the tasks (their order, and how to check each one).
