# Compagnon in two minutes

**Plan with Claude before it codes.**

Compagnon helps you work with Claude Code, Anthropic's coding assistant,
without losing control. Before writing code, Claude writes a short plan
that you can read and correct. Once you agree, Claude builds exactly that,
and the plan stays in your [repository](glossary.md#repository) (the folder
that holds your project's files) as a record of why.

You do not need to know how AI works. You ask in plain words — "add a dark
mode that follows the system setting" — and Compagnon guides Claude through
the rest. Your work is in Jira? Say "Prepare a plan for PROJ-123": Claude
reads the ticket, without changing it, and cites it at the top of the plan
([Start from a Jira ticket](guides/mcp.md)).

<div class="example">
<p class="example-title">Example</p>

```text
> Add a dark mode that follows the system setting
⏺ I will write a plan first: why, what changes, how, and the tasks.
⏺ The plan is ready in _compagnon/changes/dark-mode/. Read proposal.md
  first; nothing is built before you approve it.
> The plan is good, implement it
⏺ 7 tasks out of 7 done, each one tested.
```

You asked in your own words; Claude planned, waited, then built.

</div>

## Who decides

You do. Claude proposes, you decide, and nothing is built before you agree:

<figure class="diagram">
<div class="scroll">
<svg viewBox="0 0 760 150" role="img" aria-label="Five steps: you ask, Claude plans, you approve, Claude builds, the change is filed away.">
<g stroke-width="2" style="fill:var(--dg-box)">
<g style="stroke:var(--dg-you)"><rect x="4" y="16" width="132" height="118" rx="14"/><rect x="310" y="16" width="132" height="118" rx="14"/></g>
<g style="stroke:var(--dg-claude)"><rect x="157" y="16" width="132" height="118" rx="14"/><rect x="463" y="16" width="132" height="118" rx="14"/></g>
<rect x="616" y="16" width="140" height="118" rx="14" style="stroke:var(--dg-line)" stroke-dasharray="5 4"/>
</g>
<path d="M139,75h14m-6,-5l6,5l-6,5M292,75h14m-6,-5l6,5l-6,5M445,75h14m-6,-5l6,5l-6,5M598,75h14m-6,-5l6,5l-6,5" fill="none" stroke-width="2" style="stroke:var(--dg-line)"/>
<g class="mono" font-size="12" style="fill:var(--dg-muted)">
<text x="20" y="44">1 · YOU</text><text x="173" y="44">2 · CLAUDE</text><text x="326" y="44">3 · YOU</text><text x="479" y="44">4 · CLAUDE</text><text x="632" y="44">5 · KEPT</text>
</g>
<g font-size="16" font-weight="600" style="fill:var(--dg-ink)">
<text x="20" y="78">Ask</text><text x="173" y="78">Plan</text><text x="326" y="78">Approve</text><text x="479" y="78">Build</text><text x="632" y="78">File away</text>
</g>
<g font-size="13" style="fill:var(--dg-muted)">
<text x="20" y="104">in plain words</text><text x="173" y="104">why, how, tasks</text><text x="326" y="104">or correct it</text><text x="479" y="104">task by task</text><text x="632" y="104">with the code</text>
</g>
</svg>
</div>
<figcaption>Coral steps are yours, blue ones Claude's; the plan is kept with the code.</figcaption>
</figure>

## The name

Compagnon comes from *compagnonnage*, the French tradition of craft guilds:
a master leads the work, the *compagnons* carry it out, and everyone follows
plans agreed beforehand. Here, you are the master, and Claude's [agents](glossary.md#agent)
are your compagnons.

## What you get

- **Fewer surprises.** You see what Claude will do before it does it.
- **A shared memory.** Plans and decisions stay in the repository: anyone
  can see later why something was built.
- **Decisions that stick.** Claude follows the choices your team settled
  instead of reopening them.

## Where to start

The documentation reads as a path in five steps. Steps 1 to 3 are enough
to work every day; steps 4 and 5 are for those who want to go further.
Each page ends with "Next step"; a page some readers do not need opens
with a button that says who can skip it.

<figure class="diagram">
<div class="scroll">
<svg viewBox="0 0 760 190" role="img" aria-label="The reading tunnel in five steps: 1, Discover, one page; 2, Get started, two pages; 3, Every day, five pages; 4, Understand, five pages; 5, For your team, six pages. Steps 1 to 3 are for everyone, steps 4 and 5 for experts when they want. The appendices are read when needed.">
<g stroke-width="2" style="fill:var(--dg-box)">
<rect x="4" y="40" width="136" height="96" rx="14" style="stroke:var(--dg-you)"/>
<rect x="158" y="40" width="136" height="96" rx="14" style="stroke:var(--dg-you)"/>
<rect x="312" y="40" width="136" height="96" rx="14" style="stroke:var(--dg-you)"/>
<rect x="466" y="40" width="136" height="96" rx="14" style="stroke:var(--dg-claude)"/>
<rect x="620" y="40" width="136" height="96" rx="14" style="stroke:var(--dg-claude)"/>
</g>
<path d="M142,88h12m-6,-5l6,5l-6,5M296,88h12m-6,-5l6,5l-6,5M450,88h12m-6,-5l6,5l-6,5M604,88h12m-6,-5l6,5l-6,5" fill="none" stroke-width="2" style="stroke:var(--dg-line)"/>
<g class="mono" font-size="12" style="fill:var(--dg-muted)">
<text x="4" y="24">EVERYONE</text><text x="466" y="24">EXPERTS, WHEN THEY WANT</text>
</g>
<path d="M4,30H446M466,30H756" fill="none" stroke-width="2" style="stroke:var(--dg-line)"/>
<circle cx="30" cy="66" r="12" style="fill:var(--dg-you)"/><text x="30" y="71" text-anchor="middle" font-size="13" font-weight="600" style="fill:var(--dg-on-you)">1</text>
<text x="18" y="104" font-size="15" font-weight="600" style="fill:var(--dg-ink)">Discover</text>
<text x="18" y="124" font-size="13" style="fill:var(--dg-muted)">1 page</text>
<circle cx="184" cy="66" r="12" style="fill:var(--dg-you)"/><text x="184" y="71" text-anchor="middle" font-size="13" font-weight="600" style="fill:var(--dg-on-you)">2</text>
<text x="172" y="104" font-size="15" font-weight="600" style="fill:var(--dg-ink)">Get started</text>
<text x="172" y="124" font-size="13" style="fill:var(--dg-muted)">2 pages</text>
<circle cx="338" cy="66" r="12" style="fill:var(--dg-you)"/><text x="338" y="71" text-anchor="middle" font-size="13" font-weight="600" style="fill:var(--dg-on-you)">3</text>
<text x="326" y="104" font-size="15" font-weight="600" style="fill:var(--dg-ink)">Every day</text>
<text x="326" y="124" font-size="13" style="fill:var(--dg-muted)">5 pages</text>
<circle cx="492" cy="66" r="12" style="fill:var(--dg-claude)"/><text x="492" y="71" text-anchor="middle" font-size="13" font-weight="600" style="fill:var(--dg-box)">4</text>
<text x="480" y="104" font-size="15" font-weight="600" style="fill:var(--dg-ink)">Understand</text>
<text x="480" y="124" font-size="13" style="fill:var(--dg-muted)">5 pages</text>
<circle cx="646" cy="66" r="12" style="fill:var(--dg-claude)"/><text x="646" y="71" text-anchor="middle" font-size="13" font-weight="600" style="fill:var(--dg-box)">5</text>
<text x="634" y="104" font-size="15" font-weight="600" style="fill:var(--dg-ink)">For your team</text>
<text x="634" y="124" font-size="13" style="fill:var(--dg-muted)">6 pages</text>
<rect x="4" y="150" width="752" height="34" rx="10" fill="none" stroke-width="1.5" stroke-dasharray="5 4" style="stroke:var(--dg-line)"/>
<text x="20" y="172" font-size="13" style="fill:var(--dg-muted)">Appendices: reference, glossary, FAQ, an example plan — open them when you need them.</text>
</svg>
</div>
<figcaption>Read the steps in order; steps 1 to 3 are enough to work every day, steps 4 and 5 go further.</figcaption>
</figure>

Every term is explained in the [Glossary](glossary.md).

Compagnon's story is in [Origins](origins.md).

## Next step

[Installation](installation.md)
