# Claude Code skills

A skill is a set of instructions that Claude Code follows for one kind of
task. Compagnon gives you eight, one for each step of its
[workflow](glossary.md#workflow): planning a change, building it, filing
it away, and so on. Claude Code finds them on its own, so you can simply
ask in your own words. You can also start one directly by typing its
name, such as `/compagnon-propose`.

You do not install them by hand: `compagnon init` and `compagnon update`
write each one in its own folder, `.claude/skills/compagnon-<workflow>/SKILL.md`.

| Skill | What it does | What it may change |
|---|---|---|
| `/compagnon-propose` | Creates a change and writes its whole plan in one go | Only the change's folder, `_compagnon/changes/<name>/` |
| `/compagnon-explore` | Helps you think an idea through, compare approaches, clarify a need | Nothing |
| `/compagnon-onboard` | Introduces Compagnon, tells you where the project stands, suggests what to do next | Nothing |
| `/compagnon-configure` | Reads your project and suggests how to describe it to Compagnon: a richer `context:` and `rules:` for each part of the plan | `_compagnon/config.yaml`, once you confirm |
| `/compagnon-apply` | Builds a planned change, task by task, ticking each one off | Your project's code and `tasks.md` |
| `/compagnon-update` | Revises part of a plan already written, and keeps the rest consistent | Only the change's folder, `_compagnon/changes/<name>/` |
| `/compagnon-sync` | Updates the project's specs with what the change brings, without closing it | Nothing directly: it goes through `compagnon sync` |
| `/compagnon-archive` | Checks a finished change, updates the project's specs with it, and files it away | Nothing directly: it goes through `compagnon archive` |

## How to ask

You do not need to remember the commands. Claude Code reads the
description of every installed skill and starts the one that matches your
request: say what you want, in your own words and in your own language.
The `/compagnon-…` command is only a shortcut that names the skill
directly. If Claude picks the wrong skill, or none, say which one you mean
("use Compagnon to plan this") or type the shortcut.

### To plan a change (`/compagnon-propose`)

The cue: you ask for a plan, a proposal, or for something new to build or
fix "with Compagnon".

- "Plan a dark mode that follows the system setting."
- "Write a proposal to let users export their invoices as PDF."
- "Prepare a change for ticket PROJ-123."

### To think an idea through (`/compagnon-explore`)

The cue: you are not sure yet what to build, or want options compared.
Nothing is written.

- "Let's explore how we could cache the search results."
- "I'm hesitating between two ways to store sessions: compare them."

### To get a tour (`/compagnon-onboard`)

The cue: you are new to Compagnon or to the project, and want to know where
things stand.

- "Show me around Compagnon."
- "Where does this project stand, and what should I do next?"

### To tailor the setup (`/compagnon-configure`)

The cue: you want Compagnon to know your project's conventions.

- "Configure Compagnon for this project."
- "Read our README and code, and suggest rules for the plans."

### To build a planned change (`/compagnon-apply`)

The cue: a plan exists and you approve it.

- "The plan is good, implement it."
- "Carry out the tasks of add-dark-mode."

### To revise a plan (`/compagnon-update`)

The cue: you want to change a plan already written, before it is built.

- "In the plan, keep the user's choice when they pick a theme by hand."
- "Update the design of add-dark-mode: we store the theme in a cookie."

### To publish the specs early (`/compagnon-sync`)

The cue: another change needs this change's specs before it is archived.

- "Add the specs of add-dark-mode to the project's specs, without archiving."
- "Sync the specs of this change now."

### To file a finished change (`/compagnon-archive`)

The cue: the work is done and checked.

- "The change is done, archive it."
- "Archive add-dark-mode."

## Tool permissions

Each skill may use only the tools its job needs. A skill that must not
write files is not given the tools to write them. This is part of what
each skill promises you.

- Every skill may run `compagnon` commands, and read files.
- Only `/compagnon-apply` may run any command, because it must run your
  [tests](glossary.md#test) and build your project.
- `/compagnon-sync` and `/compagnon-archive` cannot write files at all:
  every change they make goes through Compagnon itself.
- `/compagnon-explore` and `/compagnon-onboard` write nothing.

<details>
<summary>Under the hood: the allowed-tools of each skill</summary>

Each skill declares the tools it may use in the `allowed-tools` field of its
frontmatter.

| Skill | `allowed-tools` |
|---|---|
| `compagnon-propose` | `Bash(compagnon:*), Read, Write, Edit, Glob, Grep`, plus the Jira MCP tool when configured, plus `Agent` in isolated review mode |
| `compagnon-explore` | `Bash(compagnon:*), Read, Glob, Grep` |
| `compagnon-onboard` | `Bash(compagnon:*), Read, Glob` |
| `compagnon-configure` | `Bash(compagnon:*), Read, Write, Edit, Glob, Grep` |
| `compagnon-apply` | `Bash(compagnon:*), Read, Write, Edit, Glob, Grep, Bash`, plus `Agent` in isolated review mode or when the `developer` has a model |
| `compagnon-update` | `Bash(compagnon:*), Read, Write, Edit, Glob, Grep` |
| `compagnon-sync` | `Bash(compagnon:*), Read` |
| `compagnon-archive` | `Bash(compagnon:*), Read` |

`Bash(compagnon:*)` lets a skill run `compagnon` commands and nothing else. Only
`compagnon-apply` gets general `Bash`, because it must run your tests and build
commands. `compagnon-sync` and `compagnon-archive` cannot write files at all: every
change they make goes through the CLI.

`Agent` is Claude Code's sub-agent tool. `compagnon-propose` and `compagnon-apply`
request it when `_compagnon/config.yaml` sets `review: { mode: isolated }`
([Inline or isolated review](roles.md#inline-or-isolated-review)): each
reviewing role then runs as a sub-agent. `compagnon-apply` also requests it
when the `developer` has a model: it then hands the tasks to a sub-agent
on that model ([The developer's model](roles.md#the-developers-model)).
No other skill ever requests it.
After changing the mode, run `compagnon update`; `compagnon doctor` reports
`skills_review_mode` until you do. Both skills describe the isolated
review in either mode, because a change's `change.yaml` may override the
project's mode: without the tool pre-approved, Claude Code asks before
starting a sub-agent, and if it cannot start one, the roles review
inline.

</details>

## Update notice

Five skills check, before they start, that you are running the right
version of Compagnon: `/compagnon-propose`, `/compagnon-apply`,
`/compagnon-update`, `/compagnon-sync` and `/compagnon-archive`. They
speak up in two cases:

- your team requires a version of Compagnon (`compagnon_version` in the
  configuration) that you do not have;
- a newer Compagnon is available, and you have not put it off.

The skill then says so in one line, with a link to the release, and asks
you **Update now** or **Later** before doing anything else:

- **Update now** installs the new version (`compagnon self-update --yes`),
  then carries on with what you asked;
- **Later** silences the notice for that version for 7 days
  (`compagnon self-update --snooze`), then carries on.

When your team requires the version, **Later** silences nothing: the team
asked for it. `/compagnon-sync` and `/compagnon-archive` then stop, because
`compagnon sync` and `compagnon archive` refuse to run with the wrong
version (`compagnon_version_unsatisfied`). The message tells you the fix:
`compagnon self-update`, or the version to install.

The question comes once per request, never in the middle of the work:
`/compagnon-apply` asks it before the first task. `/compagnon-explore`,
`/compagnon-onboard` and `/compagnon-configure` never ask it.
`/compagnon-onboard` passes on the result of `compagnon doctor` instead,
whose `compagnon_version` check says the same thing.

<details>
<summary>Under the hood: how a skill knows about a new version</summary>

The five skills read the `update` and `versionRequirement` objects of their
first `compagnon status --json`. No skill needs more than
`Bash(compagnon:*)` for it.

</details>

## The author's corrections

Sometimes you correct Claude on how your project does things: a
convention, a technical choice, what you expect from a part of the plan.
When this happens during `/compagnon-propose`, `/compagnon-apply` or
`/compagnon-update`, Claude writes the correction down once, in the
change's [`corrections.md`](reference/file-formats.md#correctionsmd): where
it was, what it had done, what you asked, one line each. It then shows you
the entry, so that you can have it changed or removed before it is saved
in a [commit](glossary.md#commit). `compagnon review` lists the corrections, and `compagnon validate`
checks their file.

Claude only writes down an instruction that would apply to the next change
too. It does not note a one-off preference about this change, a typo, a
change of scope, or a misunderstanding of your request.

Where it goes depends on whom it concerns:

- an instruction about the project goes to `corrections.md`, which the team
  shares through [git](glossary.md#git). It does not go to Claude Code's
  personal memory: the notes Claude keeps for you alone, on your computer;
- a preference about how Claude works with you stays in that personal
  memory.

An entry names no person, and quotes no personal data and no secret.

## The skills in detail

### Propose (`/compagnon-propose`)

This skill turns your request into a complete plan in four parts: the
proposal, the changes to the specs (called
[deltas](glossary.md#delta)), the design and the tasks.

Before writing, it reads the code your request concerns, and it bases the
scope on what it finds there. If your request contradicts the existing
specs, it tells you rather than settling the question alone. It writes the
whole plan, headings included, in
[the language your project uses for plans](guides/artifact-language.md),
exactly as its instructions give them.

Once the proposal is written, Compagnon gives the skill a list of the
existing requirements the change probably touches. The skill reads them,
and changes those that the change alters, rather than adding new ones next
to them. That list is a starting point: it may miss some.

Your request only allows planning, even if it says "build" or "fix". The
skill stops once the plan is written, and waits for you.

**Who writes what.** Each part of the plan is written by a
[role](glossary.md#role), with
"Done when" criteria that Claude checks before moving on:

- the product owner writes the proposal, including its release note for
  end users, sorted by category, or `None` for a change users will not
  notice;
- the QA analyst writes the specs;
- the architect writes the design;
- the tech lead writes the tasks. Their criteria require a task that updates
  the documentation affected, and an automated test for every scenario,
  following your project's test conventions.

These roles come from the [schema](glossary.md#schema) your project uses.
A [custom schema](guides/custom-schemas.md) defines its own.

**The review before you see it.** Before showing you the plan, the skill
reviews it in two steps:

1. **Traceability**: it checks that nothing is missing between the parts.
   Every requirement has its scenarios, every scenario a task with an
   automated test, and the release note agrees with the plan. Gaps are
   fixed in the plan.
2. **Contrarian pass**: the skill re-reads the plan as a skeptical
   reviewer would: is it needed, is the scope right, do the decisions
   hold? Experts join this pass for the sensitive areas the plan touches,
   such as security or accessibility (each such area is called a
   [domain](glossary.md#domain)). Each finding is written down, with how
   serious it is and its evidence. The author fixes it or disputes it, and
   the reviewer checks each answer once. This pass never adds scope.

The skill's summary then ends with two lists:

- **Points to discuss** (headed "Points to challenge" in the summary): up
  to five short items, most serious first. Each says what the problem is,
  why it matters, which part of the plan it concerns, and which role raised
  it. Each also gives the finding's id and status. If there are none, the
  summary says "No point to challenge found".
- **To decide**: disagreements between Claude and its reviewer that only
  you can settle, and serious problems not yet fixed.

Start your own review there.

**Tickets.** If your request mentions a ticket number (two or more capital
letters, a hyphen and a number, such as PROJ-123) and Jira is connected
(through an [MCP](glossary.md#mcp) server), the skill
reads the first ticket: read-only, in a single call. It then quotes it at
the top of the proposal. See [Jira and other MCP servers](guides/mcp.md).

Roles: analyst, investigator, product owner, QA analyst, architect, tech
lead, tracer and skeptic — see [Built-in roles](roles.md#built-in-roles).

<details>
<summary>Under the hood: roles per step and the two review steps in full</summary>

Each artifact's role is declared by the schema and returned by
`compagnon instructions`. At each step, the skill asks
`compagnon roles --at <step>` for the roles of the step and applies their
criteria, your project's extra criteria included; a step whose role your
configuration disables is skipped, and the summary says so (see
[Configuring roles](roles.md#configuring-roles)).

1. **Traceability** — every capability has its spec file, every requirement
   a nominal and an error or edge-case scenario, every scenario a task whose
   verification covers it with an automated test, a task updates the
   documentation when the Impact touches documented behavior, and the
   proposal's [release note](reference/file-formats.md#release-note) agrees
   with the plan — every breaking item of What Changes, and every removed or
   breaking modified requirement, has a **BREAKING** item. Gaps are fixed in
   the artifacts.
2. **Contrarian pass** — the skill re-reads the plan as a skeptical reviewer:
   need, scope, specs, decisions, assumptions, tasks, and whether
   `## Domains` ticks every domain the plan touches, along with your
   project's [`rules`](reference/configuration.md#rules). The skeptic
   reviews no domain itself — security, privacy, migration, operations,
   performance and accessibility are their experts'. The
   [experts](roles.md#experts-and-domains) whose domain the proposal ticks
   in `## Domains` — security, privacy, accessibility, performance,
   operations, migration — join this pass with their own knowledge. Before
   writing `## Domains`, the skill reads the
   [domains the change's diff suggests](roles.md#triggers-and-suggested-domains)
   and ticks each one or keeps it unticked with a reason; the skeptic
   records a finding for any left `unticked`. Every
   finding is written to the change's
   [review record](guides/review-record.md) with its severity and quoted
   evidence; the author fixes it or disputes it, the reviewer checks each
   answer once, and the pass never adds scope. In
   [isolated mode](roles.md#inline-or-isolated-review), the tracer and
   each reviewer run as separate sub-agents that never see the
   conversation, and the skill merges their findings before recording
   them.

</details>

### Explore (`/compagnon-explore`)

A thinking partner for vague requests, investigations and comparisons of
approaches. It reads the code and the specs, but creates no change, no plan
and no file. Once the idea is clear, it suggests `/compagnon-propose`.

Stance: [thinking partner](roles.md#thinking-partner-compagnon-explore).

### Onboard (`/compagnon-onboard`)

A tour for someone discovering Compagnon. It changes nothing. It tells you
what the tool does, what the project contains (specs, changes in progress,
decisions), and the one thing to do next: for example `/compagnon-configure`
on a project just set up.

It also runs a check of your setup, `compagnon doctor --json`, and lists up
to five missing items with how to fix each. It runs none of these fixes
itself.

Stance: [guide](roles.md#guide-compagnon-onboard).

### Configure (`/compagnon-configure`)

This skill reads your README, contributing guide, documentation and a
sample of your code. From them, it drafts a description of your project
(the `context:` block) and rules for each part of the plan (`rules:`), for
`_compagnon/config.yaml`. It shows you the difference, and writes only
after you confirm. It never touches `schema`, `language`, `workflows`,
`mcp` or `inherits`, and it keeps your existing comments.

Its rules for the tasks (`tasks`) include testing rules for your
programming language: framework, where tests live, naming, commands. It
takes them from your existing tests. Only when the project has no tests
yet does it fall back on the usual conventions of Rust,
TypeScript/JavaScript, Python, Go or Java.

As long as the configuration has no `rules:`, `compagnon init` and
`compagnon status` suggest running this skill.

Stance: [project setup analyst](roles.md#project-setup-analyst-compagnon-configure).

### Apply (`/compagnon-apply`)

This skill builds a planned change. It works like this:

1. It checks that the plan is complete.
2. It takes each unticked task of `tasks.md` in order, does it, checks it,
   and ticks it only once the check passes.
3. It writes tests like your existing ones, then following your
   language's conventions and your `rules`. It finds how to run the
   [tests](glossary.md#test) in your project's settings (its
   configuration, its manifest or its [CI](glossary.md#ci)), and asks you
   when it finds none.
4. Once every task is ticked, it re-reads the code the change touched as a
   newcomer would, joined by the experts of the areas the proposal ticks.
   It simplifies what is clearly simpler, without widening the scope, and
   reports up to five review notes, each tagged with its role. A serious
   security or privacy finding is always reported.
5. Finally, it runs your full test suite once.

Every finding goes to the [review record](guides/review-record.md), and
follows the same answer loop as the plan's. In isolated mode, each reviewer
first works alone as a [sub-agent](glossary.md#sub-agent) (a separate copy
of Claude that does not see your conversation), and the skill merges their
findings.

The summary lists what you must decide. For example, the code may now
touch a sensitive area, such as security or personal data, that the plan
did not list. You choose: have that area's expert review it, or explain
why it is not needed.

An expert fixes a clear defect within the change's scope, with a test. Any
other change of behavior, it reports instead of making it. The criteria and
knowledge of both steps come from `compagnon roles`; when no role is active
for the review, the review is skipped and the summary says so.

The skill stops at the first blocker or doubt. It never touches another
change, never syncs and never archives.

Roles: [developer](roles.md#developer-compagnon-apply),
[newcomer and experts](roles.md#newcomer-and-experts-compagnon-apply).

### Update (`/compagnon-update`)

This skill revises a part of a plan that already exists: for example, a
requirement you want changed after reading the plan. It then looks for the
effects on the other parts and tells you about them.

It does not write missing parts (that is `/compagnon-propose`), does not
change code (that is `/compagnon-apply`), and does not touch archived
changes. It ends by checking the plan with `compagnon validate`.

Stance: [editor](roles.md#editor-compagnon-update).

### Sync (`/compagnon-sync`)

This skill brings a change's specs into the project's specs, without
archiving the change. It runs `compagnon sync`, then tells you which of the
project's specs were created, updated or left unchanged.

Stance: [maintainer of the living specs](roles.md#maintainer-of-the-living-specs-compagnon-sync-compagnon-archive).

### Archive (`/compagnon-archive`)

This skill closes a finished change: it runs `compagnon archive`. If the
check finds errors, it stops, and points you to `compagnon validate`
rather than trying to work around them. If the review record blocks the
archive (`review_blocked`), it points you to `compagnon review`, and never
records a decision in your place.

Stance: [maintainer of the living specs](roles.md#maintainer-of-the-living-specs-compagnon-sync-compagnon-archive).

## Choosing which skills to install

By default, all eight are installed. To install fewer, you have two ways:

- pick the **minimal** preset when you set up Compagnon: `propose`,
  `explore`, `onboard` and `configure`;
- or list the skills you want under `workflows:` in `_compagnon/config.yaml`:

```yaml
workflows:
  - propose
  - explore
  - apply
  - archive
```

Then run `compagnon update`. A name Compagnon does not know produces a
warning and is ignored. `compagnon update` does not delete the skills you
no longer list: remove their folders from `.claude/skills/` yourself.

## Editing a generated skill

You can edit a skill's `SKILL.md` by hand. `compagnon update` notices it,
and leaves your edited file in place rather than overwriting it.
`compagnon update --force` replaces it with the generated version: check
your changes first (`git diff`), since it overwrites them, and apply them
again after.

<details>
<summary>Under the hood: how Compagnon detects a hand edit</summary>

Each generated `SKILL.md` records the compagnon version that produced it and a
checksum of its own content. If you edit a skill by hand, its content no
longer matches its checksum: `compagnon update` detects the edit and leaves the
file in place:

```text
  Left in place because edited by hand — rerun with --force to overwrite:
    /home/you/acme-app/.claude/skills/compagnon-apply/SKILL.md
```

`compagnon update --force` overwrites it with the generated version. Skills from
another compagnon version are always regenerated, and so are skills of this
version that Compagnon rendered itself — an earlier build, or the other review
mode — since they still match their checksum. A skill generated before
checksums existed, of this version, is preserved like an edit: run `compagnon
update --force` once to give it its checksum — check `git diff` first, since
it also overwrites your real edits, and apply them again after.

</details>
