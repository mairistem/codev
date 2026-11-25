# Claude Code skills

Compagnon ships eight workflows. `compagnon init` and `compagnon update` render each one as
a Claude Code skill in `.claude/skills/compagnon-<workflow>/SKILL.md`. A skill is
discovered by Claude Code automatically, and you can also invoke it directly
by typing its name as a slash command, such as `/compagnon-propose`.

| Skill | What it does | Modifies |
|---|---|---|
| `/compagnon-propose` | Creates a change and writes all its planning artifacts in one pass | `_compagnon/changes/<name>/` only |
| `/compagnon-explore` | Thinks an idea through, compares approaches, clarifies a need | Nothing |
| `/compagnon-onboard` | Introduces Compagnon, reads the project's state, recommends the next action | Nothing |
| `/compagnon-configure` | Proposes a richer `context:` and per-artifact `rules:` for `_compagnon/config.yaml` | `_compagnon/config.yaml`, after confirmation |
| `/compagnon-apply` | Implements the tasks of a planned change, checking them off | Project code and `tasks.md` |
| `/compagnon-update` | Revises a written planning artifact and keeps the others consistent | `_compagnon/changes/<name>/` only |
| `/compagnon-sync` | Merges a change's deltas into the main specs without archiving | Through `compagnon sync` only |
| `/compagnon-archive` | Validates, merges and moves a change to the archive | Through `compagnon archive` only |

## Tool permissions

Each skill declares the tools it may use in the `allowed-tools` field of its
frontmatter. The permissions are part of each skill's promise: a skill that
must not write files is not given the tools to do so.

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

## Update notice

`/compagnon-propose`, `/compagnon-apply`, `/compagnon-update`, `/compagnon-sync` and
`/compagnon-archive` read the `update` and `versionRequirement` objects of their
first `compagnon status --json`. When the project's `compagnon_version` is not
satisfied, or a newer Compagnon is available and not snoozed, the skill says so
in one line, with the release link, and asks **Update now** or **Later**
before doing anything else:

- **Update now** runs `compagnon self-update --yes`, then resumes the action;
- **Later** runs `compagnon self-update --snooze` — 7 days of silence for that
  version — then resumes. For an unsatisfied `compagnon_version`, nothing is
  snoozed: the team asked for it. `/compagnon-sync` and `/compagnon-archive` then
  stop at the refusal of `compagnon sync` or `compagnon archive`, which do not run
  outside `compagnon_version` (`compagnon_version_unsatisfied`); the message gives
  the fix — `compagnon self-update`, or the version to install.

The question is asked once per action, never in the middle of one:
`/compagnon-apply` asks it before the first task. `/compagnon-explore`,
`/compagnon-onboard` and `/compagnon-configure` never ask; `/compagnon-onboard` relays
`compagnon doctor`, whose `compagnon_version` check says the same thing. No skill
needs more than `Bash(compagnon:*)` for it.

## The author's corrections

When you correct the agent during `/compagnon-propose`, `/compagnon-apply` or
`/compagnon-update` on how your project does things — a convention, a
technical choice, an expectation on an artifact — it notes the correction
once in the change's [`corrections.md`](reference/file-formats.md#correctionsmd):
where, what it had done, what you asked, one line each. It then quotes the
entry to you, so that you can have it changed or removed before it is
committed. `compagnon review` lists the corrections, and `compagnon validate`
checks their file.

Only an instruction that would apply to the next change too is noted. A
one-off preference about this change, a typo, a change of scope or a
misunderstanding of the request is not. A project instruction goes to
`corrections.md`, which the team shares through git, and not to Claude
Code's personal memory. A preference about how the agent works with you
stays in that personal memory. An entry names no person and quotes no
personal data or secret.

## The skills in detail

### Propose (`/compagnon-propose`)

Turns a request into a complete plan: proposal, spec deltas, design and
tasks. It reads the relevant code before writing, grounds the scope in what
it finds, and reports contradictions with existing specs instead of settling
them alone. It writes the prose and the structural keywords in the
project's [artifact language](guides/artifact-language.md), exactly as the
instructions give them.

Once the proposal is written, the instructions it reads list the existing
requirements the change likely touches. The skill reads them and modifies
those the change alters rather than adding new ones beside them; the list
is a starting point, not a complete one.

The request authorizes planning only, even if it says "build" or "fix": the
skill stops once the artifacts are written and waits for you.

Each artifact has an explicit role, declared by the schema and returned by
`compagnon instructions` — product owner for the proposal, QA analyst for the
specs, architect for the design, tech lead for the tasks — with "Done when"
criteria the agent checks before moving on. The product owner writes the proposal's release note for
end users, categorized, or `None` for an internal change. The tech lead's criteria require the affected documentation
to be updated in a task, and every scenario to be covered by an automated
test that follows your project's test conventions. A
[custom schema](guides/custom-schemas.md) defines its own.

At each step it asks `compagnon roles --at <step>` for the roles of the step and
applies their criteria, your project's extra criteria included; a step whose
role your configuration disables is skipped, and the summary says so (see
[Configuring roles](roles.md#configuring-roles)).

Before presenting the plan, the skill reviews it in two steps:

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

The summary ends with **Points to challenge**: at most five one-line items
across all roles, ranked by severity and tagged with the role that raised
them — finding id, status, what, why it matters, which artifact — or
"No point to challenge found", then **To decide**: the disputes the
reviewer maintained and the high-severity findings still open. Start your
review there.

If the request mentions a ticket identifier matching `[A-Z]{2,}-\d+` and a
Jira MCP tool is configured, it fetches the first ticket — read-only, a
single call — and cites it at the top of the proposal. See
[Jira and other MCP servers](guides/mcp.md).

Roles: analyst, investigator, product owner, QA analyst, architect, tech
lead, tracer and skeptic — see [Built-in roles](roles.md#built-in-roles).

### Explore (`/compagnon-explore`)

A thinking partner for vague requests, investigations and comparisons of
approaches. It reads code and specs but creates no change, no artifact and no
file. When the idea is clear, it suggests `/compagnon-propose`.

Stance: [thinking partner](roles.md#thinking-partner-compagnon-explore).

### Onboard (`/compagnon-onboard`)

A read-only tour for someone discovering Compagnon: what the tool does, what the
current project contains (specs, active changes, decisions), and the one
action to take next — for example `/compagnon-configure` on a freshly initialized
project. It also runs `compagnon doctor --json` and lists up to five missing
setup items with their fix, without running any of them.

Stance: [guide](roles.md#guide-compagnon-onboard).

### Configure (`/compagnon-configure`)

Reads the README, contributing guide, documentation and a sample of the
source code, then drafts a `context:` block and per-artifact `rules:` for
`_compagnon/config.yaml`. It shows the diff and writes only after you confirm. It
never touches `schema`, `language`, `workflows`, `mcp` or `inherits`, and it
preserves existing comments.

Its `tasks` rules include testing rules for your language — framework, where
tests live, naming, commands — taken from your existing tests. Only when the
project has no tests yet does it fall back on the mainstream conventions of
Rust, TypeScript/JavaScript, Python, Go or Java.

While the configuration has no `rules:`, `compagnon init` and `compagnon status`
recommend running it.

Stance: [project setup analyst](roles.md#project-setup-analyst-compagnon-configure).

### Apply (`/compagnon-apply`)

Implements a planned change. It checks that planning is complete, then works
through each unchecked task of `tasks.md` in order, verifies it, and checks
it off only once the verification passes. It writes tests like your existing
test suite, then by your language's conventions and your `rules`. It takes
the test command from your configuration, manifest or CI, and asks you when
it finds none. Once every task is checked, it reviews the code the change
touched as a newcomer would, joined by the experts the proposal's domains
activate: it simplifies what is unambiguous, without widening the scope,
and reports up to five review notes, each tagged with its role — a
high-severity security or privacy finding is always reported. Every
finding goes to the [review record](guides/review-record.md) and through
the same answer loop as the plan's — in isolated mode, after each
reviewer has worked alone as a sub-agent and the skill has merged their
findings — and the summary lists what you must
decide — including any domain the code now suggests that the proposal
neither ticks nor justifies: tick it and its expert reviews the code, or
justify it. An expert
fixes a clear defect within the change's scope, with a test, and reports
any other behavior change instead of making it. It takes the criteria and
knowledge of both steps from `compagnon roles`; when no role is active at the
review, it is skipped and the summary says so. Then it runs the full test suite once. It stops at the first blocker or ambiguity. It never modifies
another change, never syncs and never archives.

Roles: [developer](roles.md#developer-compagnon-apply),
[newcomer and experts](roles.md#newcomer-and-experts-compagnon-apply).

### Update (`/compagnon-update`)

Revises an artifact that already exists — for example a requirement in a spec
delta after review — then checks the other artifacts for ripple effects and
reports them. It does not create missing artifacts (that is
`/compagnon-propose`), does not modify code (that is `/compagnon-apply`), and does not
touch archived changes. It ends with `compagnon validate`.

Stance: [editor](roles.md#editor-compagnon-update).

### Sync (`/compagnon-sync`)

Runs `compagnon sync` for a change and reports which main specs were created,
updated or left unchanged.

Stance: [maintainer of the living specs](roles.md#maintainer-of-the-living-specs-compagnon-sync-compagnon-archive).

### Archive (`/compagnon-archive`)

Runs `compagnon archive` for a change. If validation reports errors, it stops and
points you to `compagnon validate` rather than trying to work around them. If
the review record blocks archiving (`review_blocked`), it points you to
`compagnon review` and never records a decision in your place.

Stance: [maintainer of the living specs](roles.md#maintainer-of-the-living-specs-compagnon-sync-compagnon-archive).

## Choosing which skills to install

By default all eight are installed. To install fewer, pick the **minimal**
preset at init time — `propose`, `explore`, `onboard` and `configure` — or
list the workflows you want in `_compagnon/config.yaml`:

```yaml
workflows:
  - propose
  - explore
  - apply
  - archive
```

Then run `compagnon update`. An unknown workflow name produces a warning and is
ignored. `compagnon update` does not delete skills that are no longer listed;
remove their folders from `.claude/skills/` yourself.

## Editing a generated skill

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
