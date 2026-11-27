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

What will you do with Compagnon? Each answer opens a short path:

- [I review and approve plans — I do not code](paths/review.md)
- [I do not code, but I want Claude to build something for me](paths/build.md)
- [I am a developer and I have Claude write code](paths/develop.md)
- [I set Compagnon up for my team](paths/team.md)

Every term is explained in the [Glossary](glossary.md).

Compagnon's story is in [Origins](origins.md).
