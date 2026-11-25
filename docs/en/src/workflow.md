# The workflow

In a Compagnon project, every change goes through the same four steps, in the
same order. This fixed sequence is the Compagnon
[workflow](glossary.md#workflow). At each step, you ask Claude in your own
words, and Claude follows a skill: a set of instructions written for that
step. Three of the steps also have a command you can type in a terminal (the
CLI), which does the underlying work. Building the change has no command:
that is the agent's job.

```text
┌───────────┐    ┌───────────┐    ┌───────────┐    ┌───────────┐
│  propose  │───▶│   apply   │───▶│   sync    │───▶│  archive  │
│ plan it   │    │ build it  │    │ merge the │    │ file it   │
│           │    │           │    │ deltas    │    │ away      │
└───────────┘    └───────────┘    └───────────┘    └───────────┘
```

| Step | Skill | Command | What it writes |
|---|---|---|---|
| Propose | `/compagnon-propose` | `compagnon new change`, `compagnon instructions` | `_compagnon/changes/<name>/` |
| Apply | `/compagnon-apply` | — | Project code, `tasks.md` checkboxes |
| Sync (optional) | `/compagnon-sync` | `compagnon sync` | `_compagnon/specs/` |
| Archive | `/compagnon-archive` | `compagnon archive` | `_compagnon/specs/`, `_compagnon/changes/archive/` |

Two more skills help along the way. `/compagnon-explore` lets you think an
idea through before you propose anything. `/compagnon-update` revises a plan
that is already written. See [Claude Code skills](skills.md).

At each step, Claude works in a set role: product owner, QA analyst,
architect, tech lead, skeptical reviewer or developer. Each role has
criteria it checks before moving on. Claude stops at the points where you
decide. See [Roles](roles.md).

## Propose

Proposing means planning the change before any code is written. Claude
creates a folder for the change and writes the plan in it. The plan is made
of several files, which Compagnon calls [artifacts](glossary.md#artifact).
A [schema](glossary.md#schema) sets which files a plan holds. With the
default schema, `spec-driven`, a plan has four:

| Artifact | File | Needs first | What it says |
|---|---|---|---|
| `proposal` | `proposal.md` | — | Why the change is needed, what changes, and which [capabilities](glossary.md#capability) (areas of the product) it touches |
| `specs` | `specs/**/*.md` | `proposal` | The changes to the specs, called [deltas](glossary.md#delta): the behavior added, modified, removed or renamed |
| `design` | `design.md` | `proposal` | How to build it: decisions, alternatives, risks |
| `tasks` | `tasks.md` | `specs`, `design` | The checklist of work to do |

The files depend on each other. The specs and the design can be written as
soon as the proposal exists. The tasks come once both are done. Compagnon
works out where a plan stands only from the files that exist: it keeps no
separate record of progress.

<details>
<summary>Under the hood: how `compagnon status` shows each artifact</summary>

`compagnon status` reports each artifact as done (`[x]`), ready to write
(`[ ]`), blocked on a dependency (`[-]`) or skipped (`[~]`). State comes only
from which files exist — Compagnon keeps no state file.

</details>

In Claude Code, `/compagnon-propose` does the whole plan in one go:

1. If your request mentions a ticket number such as `PROJ-123`, and a Jira
   connection (an MCP server) is set up, Claude reads the ticket first. See
   [Jira and other MCP servers](guides/mcp.md).
2. It creates the folder for the change.
3. For each file that is ready to write, it gets the instructions for that
   file, studies the code concerned, and writes the file from its template.
4. It checks that the proposal, the specs and the tasks match each other.
   Then it challenges the plan as a skeptical reviewer. It writes each
   finding in the change's [review record](guides/review-record.md), fixes
   what is clear-cut, and lists up to five points for you to look at. See
   [Propose](skills.md#propose-compagnon-propose).
5. It stops once the plan is complete and shows it to you. It never starts
   building in the same turn.

<details>
<summary>Under the hood: the commands the propose skill runs</summary>

Step 2 runs `compagnon new change <name>`. Step 3 runs, for each artifact
that is ready:

```bash
compagnon instructions <artifact> --change <name> --json
```

</details>

When it writes `design.md`, Claude receives the architecture decisions in
force. The design then follows them, or openly proposes to replace one. See
[Decisions](concepts.md#decision).

Some changes do not change what the product does: a cleanup of the code, a
newer version of a library, documentation. Such a change declares
`skip_specs: true` in its `change.yaml`. Its plan then has no specs, and
validation accepts that it has no deltas.

## Apply

`/compagnon-apply <name>` builds what the plan describes. Claude reads
`tasks.md` and takes each unchecked task (`- [ ]`) in the order of the file.
For each one, it runs the check the task names: a test, a command, or a
behavior you can observe. It ticks the box only once that check passes.

New tests follow your existing tests first, then the usual habits of your
programming language, then your [`rules`](reference/configuration.md#rules).

Once every task is ticked, Claude re-reads the code it touched, the way a
newcomer to the project would. The [experts](roles.md#experts-and-domains)
matching the domains named in the proposal join this reading. Claude
simplifies what is clear-cut, without widening the scope, and lists up to
five review notes in its summary. Every finding goes into the change's
[review record](guides/review-record.md), with the author's answer and the
reviewer's check. What only you can decide is listed at the end. Then
Claude runs the full test suite once.

Claude stops at the first blocker or unclear point and asks you, rather
than guessing. It works only on the change you named. It never archives and
never syncs: you ask for those steps yourself.

Apply is the only skill allowed to run any terminal command it needs,
because it has to run your tests. It has no command of its own: building is
the agent's job.

## Sync

```bash
compagnon sync --change <name>
```

Syncing copies the change's deltas into the project's specs, in
`_compagnon/specs/`, **without** moving the change. Use it in two cases:
when another change in progress needs to see a new capability in the
project's specs, or when you want to check the result before archiving.

Running it twice does no harm: the second time, it reports that the specs
are unchanged. Usually you skip this step, because archiving syncs for you.

Like archiving, syncing first validates the change. If validation reports
an error, it refuses: nothing is written, and `compagnon validate <name>`
shows what is wrong.

## Archive

```bash
compagnon archive --change <name>
```

Archiving closes a change, in four steps:

1. It validates the change. If validation reports an error, it **refuses**
   and points you to `compagnon validate <name>` for the details.
2. It checks the [review record](guides/review-record.md). It **refuses**
   (`review_blocked`) if the record is missing for a step where review roles
   were active, or if a high-severity finding is still open or disputed.
   `compagnon review --change <name>` shows what is left.
3. It copies the deltas into the project's specs, exactly as sync does.
4. It moves the change's folder, review record included, to
   `_compagnon/changes/archive/<YYYY-MM-DD>-<name>/`.

Compagnon works out and checks the whole merge before it writes the first
file. So a problem is reported before anything on disk changes: an invalid
change, or a delta that does not fit. For example, a delta may modify
(`MODIFIED`) a requirement that the project's spec does not have.

After archiving, the project's specs describe the behavior you shipped. The
archived folder keeps why and how it changed. Commit both. If the change has
a release note, run `compagnon changelog --write` to add it to
`CHANGELOG.md` (see [Release notes and CHANGELOG](guides/release-notes.md)).

## Choosing the change

`compagnon status`, `compagnon instructions`, `compagnon sync` and
`compagnon archive` accept `--change <name>`. When the project has exactly
one change in progress, you can leave it out. When there are several,
Compagnon asks you to choose:

```text
error: several active changes: add-audit-log, rework — specify which one
help: `--change add-audit-log`
```

Change names are written in [kebab-case](glossary.md#kebab-case): lowercase
letters, digits and hyphens, such as `add-user-auth`.

## Validating

```bash
compagnon validate
```

On its own, `compagnon validate` checks every change in progress, every
spec of the project, and the seals that protect the decisions. To check a
single item, give the name of a change or a capability. To check one
family, add `--changes` or `--specs`.

An error makes the command fail (exit status 1). With `--strict`, warnings
do too. Use `--strict` in continuous integration (CI), the checks that run
automatically on every push:

```bash
compagnon validate --strict
```

Validation checks:

- the shape of specs and deltas: every requirement uses `SHALL` or `MUST`
  (or `DOIT` or `DOIVENT`) and has at least one `#### Scenario:`;
- that a delta does not contradict itself: no requirement both added and
  modified, no renaming that clashes with an addition;
- the zero-delta rule: a change with no deltas at all must declare
  `skip_specs: true`;
- that sealed decisions have not been altered.

See [File formats](reference/file-formats.md) for the rules each file must
follow.
