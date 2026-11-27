# Your first change

This page takes you from an empty
[repository](glossary.md#repository) to your first finished change, in
about ten minutes. Each step shows what to say to Claude, then the shortcut
command.

You need [Claude Code](glossary.md#claude-code). The example adds a dark
mode to an application; use your own idea.

## 1. Install Compagnon

On macOS or Linux:

```bash
curl -sSL https://raw.githubusercontent.com/mairistem/compagnon/main/install.sh | sh
```

On Windows, in PowerShell:

```powershell
iwr -useb https://raw.githubusercontent.com/mairistem/compagnon/main/install.ps1 | iex
```

Other ways to install are in [Installation](installation.md).

## 2. Set up your repository

In a [terminal](glossary.md#terminal) (the window where you type commands), in
your project's main folder, type once:

```bash
compagnon init
```

It may ask a few questions, then adds a `_compagnon/` folder and gives
Claude Code the [skills](glossary.md#skill). Restart Claude Code so it sees
them.

<details>
<summary>Under the hood: what init writes</summary>

```bash
compagnon init --yes --language en
```

```text
compagnon initialized in /home/you/acme-app

  Structure   _compagnon/ — specs, decisions, changes, schemas
  Skills      compagnon-propose, compagnon-explore, compagnon-onboard, compagnon-apply, compagnon-sync, compagnon-archive, compagnon-update, compagnon-configure → .claude/skills/

  Repository integration
    + CLAUDE.md                          created
    + .github/workflows/compagnon.yml        created
    + .github/PULL_REQUEST_TEMPLATE.md   created

  17 file(s) created, 0 updated

→ Recommended next step: in Claude Code, ask
  "configure Compagnon for this project" (shortcut: /compagnon-configure).
```

The probe, the questions and every file written are described in
[What `compagnon init` does](reference/cli.md#what-compagnon-init-does).
`compagnon doctor` then lists what the setup still lacks, with the fix for
each item.

</details>

## 3. Ask for a change (`/compagnon-propose`)

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

## 4. Read the plan

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

## 5. Let Claude build it (`/compagnon-apply`)

Once the plan suits you:

> The plan is good, implement it.

Claude works through the tasks one by one, checks each with its
[test](glossary.md#test) (a small program that proves the task works), and
asks you if something is unclear. Shortcut: `/compagnon-apply add-dark-mode`.

## 6. File it away (`/compagnon-archive`)

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
<path d="M312,110h40M352,110V60h62m-6,-5l6,5l-6,5M352,110V160h62m-6,-5l6,5l-6,5" fill="none" stroke-width="2" style="stroke:var(--dg-line)"/>
<g class="mono" font-size="13" style="fill:var(--dg-ink)">
<text x="24" y="48" font-weight="600">_compagnon/changes/&lt;name&gt;/</text><text x="40" y="84">proposal.md</text><text x="40" y="112">specs/</text><text x="40" y="140">design.md</text><text x="40" y="168">tasks.md</text>
<text x="436" y="48" font-weight="600">_compagnon/changes/archive/</text><text x="436" y="70">&lt;date&gt;-&lt;name&gt;/</text><text x="436" y="148" font-weight="600">_compagnon/specs/</text>
</g>
<g font-size="13" style="fill:var(--dg-muted)">
<text x="320" y="100">archive</text><text x="436" y="90">kept as written</text><text x="436" y="176">updated by the plan</text>
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

## Go further

- [How it works](workflow.md): each step in detail.
- [Asking Claude](skills.md#how-to-ask): other ways to ask.
- [Start from a Jira ticket](guides/mcp.md)
- [Reviewing a plan](guides/review-a-plan.md): what to check in a plan.
