# Your first change

<div class="skip-step">

[You do not code? Skip to Asking Claude](skills.md)

</div>

This page takes you from a prepared project to your first finished
change, in about ten minutes. Each step shows what to say to Claude, then
the shortcut command, which does the same.

You need Compagnon installed and `compagnon init` run in your project
([Installation](installation.md)), and Claude Code open on it. The example
adds a dark mode to an application; use your own idea.

<div class="example">
<p class="example-title">Example</p>

```text
> Add a dark mode that follows the system setting
⏺ Bash(compagnon new change add-dark-mode)
  ⎿ Change `add-dark-mode` created
⏺ The plan is ready in _compagnon/changes/add-dark-mode/. Read proposal.md first.
> The plan is good, implement it
⏺ Update(src/theme.ts)
  ⎿ Updated src/theme.ts with 18 additions and 2 removals
⏺ Bash(npm test)
  ⎿ ✓ 42 tests passed
⏺ 7 tasks out of 7 done · 2 review notes, nothing to decide.
> The change is done, archive it
⏺ Bash(compagnon archive --change add-dark-mode)
  ⎿ ✓ Archived · moved to _compagnon/changes/archive/2026-11-27-add-dark-mode
```

Three sentences of yours; the steps below take them one at a time.

</div>

## 1. Ask for a change (`/compagnon-propose`)

In Claude Code, say what you want:

> Plan a dark mode that follows the system setting.

Claude writes a plan and stops: it changes no code yet. The shortcut does
the same: `/compagnon-propose add a dark mode that follows the system
setting`.

**You have a Jira ticket?** Name it:

> Prepare a plan for PROJ-123.

Claude reads it, without changing it, and cites it at the top of the plan
([Start from a Jira ticket](guides/mcp.md)).

<details>
<summary>Under the hood: how the plan is written</summary>

The skill names the change (`add-dark-mode`), creates its folder, then asks
Compagnon, for each part of the plan, what to write and in which language:

```bash
compagnon new change add-dark-mode
compagnon instructions proposal --change add-dark-mode
```

```text
Artifact: proposal (change `add-dark-mode`, schema spec-driven)
Write to: /home/you/acme-app/_compagnon/changes/add-dark-mode/proposal.md
Language: en — prose and structural keywords in en
```

</details>

## 2. Read the plan

The plan sits in `_compagnon/changes/add-dark-mode/`, in four short files:
the **proposal** (why), the **specs** (what will be true afterwards), the
**design** (how) and the **tasks** (the steps). To change something, just
say so:

> In the plan, keep the user's choice when they pick a theme by hand.

<details>
<summary>Under the hood: checking the plan</summary>

```bash
compagnon status --change add-dark-mode
compagnon validate add-dark-mode
```

`status` shows the four parts as written; `validate` checks their format.
A change with no spec fails on purpose with `zero_delta_without_marker`.

</details>

## 3. Let Claude build it (`/compagnon-apply`)

Once the plan suits you:

> The plan is good, implement it.

Claude works through the tasks one by one, checks each with its
[test](glossary.md#test) (a small program that proves the task works), and
asks you if something is unclear. Shortcut: `/compagnon-apply add-dark-mode`.

## 4. File it away (`/compagnon-archive`)

Check the result, then:

> The change is done, archive it.

Claude adds the plan's specs to the project's specs and moves the change
to the archive: the project now remembers why dark mode exists. Shortcut:
`/compagnon-archive add-dark-mode`. Then save the result in the project's
history ([commit](glossary.md#commit) it), or ask Claude to.

<figure class="diagram">
<div class="scroll">
<svg viewBox="0 0 760 220" role="img" aria-label="A change's folder: _compagnon/changes/&lt;name&gt;/ holds proposal.md, specs/, design.md and tasks.md; archiving moves it to _compagnon/changes/archive/&lt;date&gt;-&lt;name&gt;/ and updates the project's specs.">
<g stroke-width="2" style="fill:var(--dg-file)">
<rect x="8" y="20" width="300" height="180" rx="12" style="stroke:var(--dg-claude)"/><rect x="420" y="20" width="332" height="80" rx="12" style="stroke:var(--dg-line)" stroke-dasharray="5 4"/><rect x="420" y="120" width="332" height="80" rx="12" style="stroke:var(--dg-line)"/>
</g>
<path d="M312,110h60M372,110V60h42m-6,-5l6,5l-6,5M372,110V160h42m-6,-5l6,5l-6,5" fill="none" stroke-width="2" style="stroke:var(--dg-line)"/>
<g class="mono" font-size="13" style="fill:var(--dg-ink)">
<text x="24" y="48" font-weight="600">_compagnon/changes/&lt;name&gt;/</text><text x="40" y="84">proposal.md</text><text x="40" y="112">specs/</text><text x="40" y="140">design.md</text><text x="40" y="168">tasks.md</text>
<text x="436" y="48" font-weight="600">_compagnon/changes/archive/</text><text x="436" y="70">&lt;date&gt;-&lt;name&gt;/</text><text x="436" y="148" font-weight="600">_compagnon/specs/</text>
</g>
<g font-size="13" style="fill:var(--dg-muted)">
<text x="318" y="102" font-size="12">archive</text><text x="436" y="90">kept as written</text><text x="436" y="176">updated by the plan</text>
</g>
</svg>
</div>
<figcaption>From start to archive: the plan is kept, and the project's specs say what it changed.</figcaption>
</figure>

<details>
<summary>Under the hood: what archiving does</summary>

```bash
compagnon archive --change add-dark-mode
```

```text
Archived change `add-dark-mode`

Created:
  /home/you/acme-app/_compagnon/specs/ui/theme/spec.md

Moved to: /home/you/acme-app/_compagnon/changes/archive/2026-09-30-add-dark-mode
```

</details>

## Next step

[Asking Claude](skills.md)
