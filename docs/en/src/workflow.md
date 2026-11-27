# How it works

Every change in a Compagnon project goes through the same five steps, in
the same order: the Compagnon [workflow](glossary.md#workflow). You decide
at two of them; Claude works at the others and stops whenever it needs you.

<figure class="diagram">
<div class="scroll">
<svg viewBox="0 0 760 170" role="img" aria-label="Five steps: you ask in plain words; Claude writes a plan; you approve it or ask for changes; Claude builds it task by task; the change is filed away and the plan stays with the code.">
<defs><marker id="dg-workflow-arrow" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" style="fill:var(--dg-line)"/></marker></defs>
<g class="st"><text x="70" y="22" text-anchor="middle" class="mono" font-size="12" style="fill:var(--dg-you-text)">YOU</text><rect x="4" y="32" width="132" height="118" rx="14" style="fill:var(--dg-box);stroke:var(--dg-you)" stroke-width="2"/><circle cx="30" cy="58" r="13" style="fill:var(--dg-you)"/><text x="30" y="63" text-anchor="middle" class="mono" font-size="13" font-weight="700" style="fill:var(--dg-on-you)">1</text><text x="20" y="98" font-size="16" font-weight="600" style="fill:var(--dg-ink)">Ask</text><text x="20" y="120" font-size="13" style="fill:var(--dg-muted)">in plain words</text></g>
<line x1="138" y1="91" x2="154" y2="91" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-workflow-arrow)"/>
<g class="st"><text x="223" y="22" text-anchor="middle" class="mono" font-size="12" style="fill:var(--dg-claude)">CLAUDE</text><rect x="157" y="32" width="132" height="118" rx="14" style="fill:var(--dg-box);stroke:var(--dg-claude)" stroke-width="2"/><circle cx="183" cy="58" r="13" style="fill:var(--dg-claude)"/><text x="183" y="63" text-anchor="middle" class="mono" font-size="13" font-weight="700" style="fill:var(--dg-box)">2</text><text x="173" y="98" font-size="16" font-weight="600" style="fill:var(--dg-ink)">Plan</text><text x="173" y="120" font-size="13" style="fill:var(--dg-muted)">why, what, how,</text><text x="173" y="137" font-size="13" style="fill:var(--dg-muted)">tasks</text></g>
<line x1="291" y1="91" x2="307" y2="91" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-workflow-arrow)"/>
<g class="st"><text x="376" y="22" text-anchor="middle" class="mono" font-size="12" style="fill:var(--dg-you-text)">YOU</text><rect x="310" y="32" width="132" height="118" rx="14" style="fill:var(--dg-box);stroke:var(--dg-you)" stroke-width="2"/><circle cx="336" cy="58" r="13" style="fill:var(--dg-you)"/><text x="336" y="63" text-anchor="middle" class="mono" font-size="13" font-weight="700" style="fill:var(--dg-on-you)">3</text><text x="326" y="98" font-size="16" font-weight="600" style="fill:var(--dg-ink)">Approve</text><text x="326" y="120" font-size="13" style="fill:var(--dg-muted)">or ask for</text><text x="326" y="137" font-size="13" style="fill:var(--dg-muted)">changes</text></g>
<line x1="444" y1="91" x2="460" y2="91" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-workflow-arrow)"/>
<g class="st"><text x="529" y="22" text-anchor="middle" class="mono" font-size="12" style="fill:var(--dg-claude)">CLAUDE</text><rect x="463" y="32" width="132" height="118" rx="14" style="fill:var(--dg-box);stroke:var(--dg-claude)" stroke-width="2"/><circle cx="489" cy="58" r="13" style="fill:var(--dg-claude)"/><text x="489" y="63" text-anchor="middle" class="mono" font-size="13" font-weight="700" style="fill:var(--dg-box)">4</text><text x="479" y="98" font-size="16" font-weight="600" style="fill:var(--dg-ink)">Build</text><text x="479" y="120" font-size="13" style="fill:var(--dg-muted)">task by task,</text><text x="479" y="137" font-size="13" style="fill:var(--dg-muted)">tested</text></g>
<line x1="597" y1="91" x2="613" y2="91" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-workflow-arrow)"/>
<g class="st"><text x="682" y="22" text-anchor="middle" class="mono" font-size="12" style="fill:var(--dg-muted)">KEPT</text><rect x="616" y="32" width="140" height="118" rx="14" style="fill:var(--dg-box);stroke:var(--dg-line)" stroke-width="2" stroke-dasharray="5 4"/><circle cx="642" cy="58" r="13" style="fill:var(--dg-line)"/><text x="642" y="63" text-anchor="middle" class="mono" font-size="13" font-weight="700" style="fill:var(--dg-box)">5</text><text x="632" y="98" font-size="16" font-weight="600" style="fill:var(--dg-ink)">File away</text><text x="632" y="120" font-size="13" style="fill:var(--dg-muted)">the plan stays</text><text x="632" y="137" font-size="13" style="fill:var(--dg-muted)">with the code</text></g>
</svg>
</div>
<figcaption>The five steps: coral steps are yours, blue steps are Claude's, and the dashed one is kept for later. Nothing is built before step 3.</figcaption>
</figure>

<div class="diagram-steps">
<details><summary>1. You ask</summary>

Say what you want in your own words: "add a dark mode", or "Prepare a plan
for PROJ-123". Claude picks the matching skill.
Go further: [Asking Claude](skills.md).

</details>
<details><summary>2. Claude plans</summary>

Claude studies the code, then writes four short files: why, what, how and
the tasks. It challenges its own plan before showing it to you.
Go further: [An annotated example plan](guides/example-plan.md).

</details>
<details><summary>3. You approve</summary>

You read the plan, ask for corrections, then approve it. You can do this
in the conversation or on GitHub and GitLab.
Go further: [Reviewing a plan](guides/review-a-plan.md).

</details>
<details><summary>4. Claude builds</summary>

Claude does the tasks in order and runs each one's test. It stops at the
first doubt instead of guessing.
Go further: [Roles](roles.md).

</details>
<details><summary>5. Filed away</summary>

The plan's changes join the project's specs, and the plan is archived. The
next person knows why the code is the way it is.
Go further: [Concepts](concepts.md).

</details>
</div>

At each step, Claude follows a skill (see [Asking Claude](skills.md)).
This page says what the main four do: propose, apply, sync and archive.

## The plan's files

A change lives in its own folder, `_compagnon/changes/<name>/`. Each file
in it is an [artifact](glossary.md#artifact), and each has one author:

| File | What it says | Who writes it |
|---|---|---|
| `proposal.md` | Why the change is needed, what changes, which [capabilities](glossary.md#capability) (areas of the product) it touches, and a release note for users | Claude as product owner |
| `specs/<capability>/spec.md` | The [deltas](glossary.md#delta): the behavior added, modified, removed or renamed, each with its scenarios | Claude as QA analyst |
| `design.md` | How it will be built: decisions, alternatives, risks | Claude as architect |
| `tasks.md` | The checklist of work, each task with its check | Claude as tech lead; `/compagnon-apply` ticks the boxes |
| `review.md` | The [review record](guides/review-record.md): each finding, the author's answer, the reviewer's check | Claude's reviewing roles, while planning and building |
| `corrections.md` | What you corrected Claude on, for the next change | Claude, when you correct it ([The author's corrections](skills.md#the-authors-corrections)) |
| `change.yaml` | The change's settings, such as `skip_specs` | Compagnon creates it; you edit it |

The [schema](glossary.md#schema) sets which files a plan holds; the
default one, `spec-driven`, asks for the first four. The specs and the
design need the proposal first; the tasks need both. You approve a plan
by reading the proposal; the other files are there when you want detail.

<details>
<summary>Under the hood: how `compagnon status` shows each file</summary>

`compagnon status` reports each artifact as done (`[x]`), ready to write
(`[ ]`), blocked on a dependency (`[-]`) or skipped (`[~]`). State comes only
from which files exist — Compagnon keeps no state file.

</details>

## Propose

`/compagnon-propose` turns your request into a complete plan, and nothing
else: even if you say "build" or "fix", it stops once the plan is written
and waits for you.

<figure class="diagram">
<div class="scroll">
<svg viewBox="0 0 760 170" role="img" aria-label="How a plan is built, in six steps: Claude understands the request and reads the ticket; investigates what the code already does; writes the four files, one role each; traces that the files agree; challenges the plan with a skeptic and experts; then you approve it.">
<defs><marker id="dg-workflow-plan-arrow" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" style="fill:var(--dg-line)"/></marker></defs>
<g class="st"><text x="59" y="22" text-anchor="middle" class="mono" font-size="12" style="fill:var(--dg-claude)">CLAUDE</text><rect x="4" y="32" width="110" height="118" rx="14" style="fill:var(--dg-box);stroke:var(--dg-claude)" stroke-width="2"/><circle cx="28" cy="56" r="12" style="fill:var(--dg-claude)"/><text x="28" y="61" text-anchor="middle" class="mono" font-size="13" font-weight="700" style="fill:var(--dg-box)">1</text><text x="16" y="96" font-size="15" font-weight="600" style="fill:var(--dg-ink)">Understand</text><text x="16" y="117" font-size="12" style="fill:var(--dg-muted)">the request,</text><text x="16" y="133" font-size="12" style="fill:var(--dg-muted)">the ticket</text></g>
<line x1="116" y1="91" x2="128" y2="91" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-workflow-plan-arrow)"/>
<g class="st"><text x="185" y="22" text-anchor="middle" class="mono" font-size="12" style="fill:var(--dg-claude)">CLAUDE</text><rect x="130" y="32" width="110" height="118" rx="14" style="fill:var(--dg-box);stroke:var(--dg-claude)" stroke-width="2"/><circle cx="154" cy="56" r="12" style="fill:var(--dg-claude)"/><text x="154" y="61" text-anchor="middle" class="mono" font-size="13" font-weight="700" style="fill:var(--dg-box)">2</text><text x="142" y="96" font-size="15" font-weight="600" style="fill:var(--dg-ink)">Investigate</text><text x="142" y="117" font-size="12" style="fill:var(--dg-muted)">what the code</text><text x="142" y="133" font-size="12" style="fill:var(--dg-muted)">already does</text></g>
<line x1="242" y1="91" x2="254" y2="91" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-workflow-plan-arrow)"/>
<g class="st"><text x="311" y="22" text-anchor="middle" class="mono" font-size="12" style="fill:var(--dg-claude)">CLAUDE</text><rect x="256" y="32" width="110" height="118" rx="14" style="fill:var(--dg-box);stroke:var(--dg-claude)" stroke-width="2"/><circle cx="280" cy="56" r="12" style="fill:var(--dg-claude)"/><text x="280" y="61" text-anchor="middle" class="mono" font-size="13" font-weight="700" style="fill:var(--dg-box)">3</text><text x="268" y="96" font-size="15" font-weight="600" style="fill:var(--dg-ink)">Write</text><text x="268" y="117" font-size="12" style="fill:var(--dg-muted)">four files,</text><text x="268" y="133" font-size="12" style="fill:var(--dg-muted)">one role each</text></g>
<line x1="368" y1="91" x2="380" y2="91" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-workflow-plan-arrow)"/>
<g class="st"><text x="437" y="22" text-anchor="middle" class="mono" font-size="12" style="fill:var(--dg-claude)">CLAUDE</text><rect x="382" y="32" width="110" height="118" rx="14" style="fill:var(--dg-box);stroke:var(--dg-claude)" stroke-width="2"/><circle cx="406" cy="56" r="12" style="fill:var(--dg-claude)"/><text x="406" y="61" text-anchor="middle" class="mono" font-size="13" font-weight="700" style="fill:var(--dg-box)">4</text><text x="394" y="96" font-size="15" font-weight="600" style="fill:var(--dg-ink)">Trace</text><text x="394" y="117" font-size="12" style="fill:var(--dg-muted)">do the files</text><text x="394" y="133" font-size="12" style="fill:var(--dg-muted)">agree?</text></g>
<line x1="494" y1="91" x2="506" y2="91" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-workflow-plan-arrow)"/>
<g class="st"><text x="563" y="22" text-anchor="middle" class="mono" font-size="12" style="fill:var(--dg-claude)">CLAUDE</text><rect x="508" y="32" width="110" height="118" rx="14" style="fill:var(--dg-box);stroke:var(--dg-claude)" stroke-width="2"/><circle cx="532" cy="56" r="12" style="fill:var(--dg-claude)"/><text x="532" y="61" text-anchor="middle" class="mono" font-size="13" font-weight="700" style="fill:var(--dg-box)">5</text><text x="520" y="96" font-size="15" font-weight="600" style="fill:var(--dg-ink)">Challenge</text><text x="520" y="117" font-size="12" style="fill:var(--dg-muted)">a skeptic and</text><text x="520" y="133" font-size="12" style="fill:var(--dg-muted)">experts re-read</text></g>
<line x1="620" y1="91" x2="632" y2="91" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-workflow-plan-arrow)"/>
<g class="st"><text x="695" y="22" text-anchor="middle" class="mono" font-size="12" style="fill:var(--dg-you-text)">YOU</text><rect x="634" y="32" width="122" height="118" rx="14" style="fill:var(--dg-box);stroke:var(--dg-you)" stroke-width="2"/><circle cx="658" cy="56" r="12" style="fill:var(--dg-you)"/><text x="658" y="61" text-anchor="middle" class="mono" font-size="13" font-weight="700" style="fill:var(--dg-on-you)">6</text><text x="646" y="96" font-size="15" font-weight="600" style="fill:var(--dg-ink)">You approve</text><text x="646" y="117" font-size="12" style="fill:var(--dg-muted)">or ask for</text><text x="646" y="133" font-size="12" style="fill:var(--dg-muted)">changes</text></g>
</svg>
</div>
<figcaption>How a plan is built: Claude goes through five steps on its own, and checks its plan twice, before you see it.</figcaption>
</figure>

1. **Understand.** Claude makes sure what must be built. If your request
   names a ticket, such as `PROJ-123`, and Jira is connected, Claude reads
   that ticket once, without changing it, and cites it at the top of the
   proposal. See [Start from a Jira ticket](guides/mcp.md).
2. **Investigate.** Before writing, Claude reads the code, tests and
   documentation your request concerns, and bases the scope on them. If
   your request contradicts the project's specs, it tells you rather than
   settling it alone.
3. **Write.** Claude writes the four files, each in the role the table
   above gives, in [the language your project uses for plans](guides/artifact-language.md).
   It changes the existing requirements the change touches rather than
   adding new ones next to them. The design follows the
   [decisions](concepts.md#decision) in force, or openly proposes to
   replace one.
4. **Trace.** Claude checks that nothing is missing between the files:
   every requirement has scenarios, every scenario a task with an
   automated test, and the release note agrees with the plan. Gaps are
   fixed in the plan.
5. **Challenge.** Claude re-reads the plan as a skeptical reviewer would:
   is it needed, is the scope right, do the decisions hold? Experts join
   for the sensitive areas the plan touches, such as security or
   accessibility (each one a [domain](glossary.md#domain)). Each finding
   goes to the review record; the author fixes or disputes it, and the
   reviewer checks each answer once. This pass never adds scope.
6. **You approve.** The summary ends with up to five **points to
   discuss**, most serious first, then what **only you can decide**.
   Start your own review there.

Which role works at each step, and what it checks, is in [Roles](roles.md).

Some changes alter no behavior: a cleanup, an upgrade, documentation. Such
a change declares `skip_specs: true` in its `change.yaml`, and its plan
has no specs.

<details>
<summary>Under the hood: the commands the propose skill runs</summary>

Creating the folder runs `compagnon new change <name>`. Then, for each
artifact that is ready:

```bash
compagnon instructions <artifact> --change <name> --json
```

Each artifact's role is declared by the schema and returned by
`compagnon instructions`. At each step, the skill asks
`compagnon roles --at <step>` for the roles of the step and applies their
criteria, your project's extra criteria included; a step whose role your
configuration disables is skipped, and the summary says so (see
[Configuring roles](roles.md#configuring-roles)). Your request only allows
planning: the skill writes nothing outside the change's folder.

Once the proposal is written, Compagnon gives the skill the existing
requirements the change probably touches. That list is a starting point:
it may miss some.

</details>

<details>
<summary>Under the hood: the two review passes in full</summary>

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
   in `## Domains` join this pass with their own knowledge. Before
   writing `## Domains`, the skill reads the
   [domains the change's diff suggests](roles.md#triggers-and-suggested-domains)
   and ticks each one or keeps it unticked with a reason; the skeptic
   records a finding for any left `unticked`. Every finding is written to
   the [review record](guides/review-record.md) with its severity and
   quoted evidence. In
   [isolated mode](roles.md#inline-or-isolated-review), the tracer and
   each reviewer run as separate sub-agents that never see the
   conversation, and the skill merges their findings before recording
   them.

Each point to discuss (headed "Points to challenge" in the summary) says
what the problem is, why it matters, which file it concerns, which role
raised it, and the finding's id and status. With none, the summary says
"No point to challenge found".

</details>

## Apply

`/compagnon-apply <name>` builds what the plan describes:

1. It checks that the plan is complete.
2. It takes each unticked task of `tasks.md` (`- [ ]`) in order, does it,
   runs the check the task names, and ticks the box only once that check
   passes.
3. It writes tests like your existing ones, then by your language's
   habits and your [`rules`](reference/configuration.md#rules), and asks
   you how to run them when your project does not say.
4. Once every task is ticked, Claude re-reads the code it touched, the way
   a newcomer to the project would, joined by the
   [experts](roles.md#experts-and-domains) of the domains the proposal
   ticks. It simplifies what is clear-cut, without widening the scope, and
   lists up to five review notes. A serious security or privacy finding is
   always reported.
5. Finally, it runs all the project's tests once.

Every finding goes to the review record, with the author's answer and the
reviewer's check. The summary ends with what only you can decide: for
example, the code now touches personal data that the plan did not list.

Claude stops at the first blocker or doubt and asks you, rather than
guessing. It works only on the change you named, and never syncs or
archives: you ask for those steps yourself. Apply is the only skill allowed
to run any terminal command, because it must run your tests.

<details>
<summary>Under the hood: experts' fixes and isolated mode</summary>

Claude finds how to run your tests in the project's configuration, its
manifest or its [CI](glossary.md#ci). An expert fixes a clear defect
within the change's scope, with a test. Any
other change of behavior, it reports instead of making it. The criteria and
knowledge of both steps come from `compagnon roles`; when no role is active
for the review, the review is skipped and the summary says so. In isolated
mode, each reviewer first works alone as a
[sub-agent](glossary.md#sub-agent) (a separate copy of Claude that does not
see your conversation), and the skill merges their findings.

</details>

## Sync

```bash
compagnon sync --change <name>
```

`/compagnon-sync` copies the change's deltas into the project's specs, in
`_compagnon/specs/`, **without** moving the change. Use it when another
change in progress needs to see a new capability, or to check the result
before archiving. It tells you which specs were created, updated or left
unchanged. Usually you skip it: archiving syncs for you.

## Archive

```bash
compagnon archive --change <name>
```

`/compagnon-archive` closes a finished change, in four steps:

1. It validates the change, and **refuses** on an error.
2. It checks the [review record](guides/review-record.md), and **refuses**
   if a review was expected but not recorded, or if a serious finding is
   not settled yet (`review_blocked`). `compagnon review --change <name>`
   shows what is left. Claude never records a decision in your place.
3. It copies the deltas into the project's specs, as sync does.
4. It moves the change's folder to
   `_compagnon/changes/archive/<YYYY-MM-DD>-<name>/`.

Nothing on disk changes until the whole update has been checked.
Afterwards, the project's specs describe what you shipped, and the
archived folder keeps why and how. Save both in a [commit](glossary.md#commit). If the change
has a release note, `compagnon changelog --write` adds it to
`CHANGELOG.md` (see [Release notes and CHANGELOG](guides/release-notes.md)).

<details>
<summary>Under the hood: when archiving refuses on the review record</summary>

Archiving reports `review_blocked` when the review record is missing for a
step where review roles were active, or when a finding of severity `high`
is still `open` or `disputed`. Sync, like archive, validates first: on an
error it writes nothing and points to `compagnon validate <name>`.

</details>

What `compagnon validate` checks, and how `--change` is chosen when
several changes are in progress, are in the
[Command-line interface](reference/cli.md#compagnon-validate) reference
and its [conventions](reference/cli.md#conventions).

## The other skills

- `/compagnon-explore` thinks an idea through with you and writes nothing.
- `/compagnon-update` revises one file of a plan, tells you the effects on
  the others, and ends with `compagnon validate`. It touches no code.
- `/compagnon-onboard` gives a tour, lists up to five missing setup items
  from `compagnon doctor`, and suggests one next step. It changes nothing.
- `/compagnon-configure` reads your project and drafts the `context:` and
  `rules:` of `_compagnon/config.yaml`, testing rules included, and writes
  only after you confirm. `compagnon init` and `compagnon status` suggest
  it while there are no `rules:`.

Their stances are in [Roles](roles.md).

## Go further

- [Asking Claude](skills.md): how to ask for each skill.
- [Reviewing a plan](guides/review-a-plan.md): what to check in each file.
- [Roles](roles.md): the criteria of each role, at each step.
- [Review record](guides/review-record.md): the life of a finding.
- [Command-line interface](reference/cli.md): every command and option.
