# Your first change

This page takes you from an empty
[repository](glossary.md#repository) to your first finished change,
in about ten minutes. You talk to Claude in plain words: each step shows
the sentence to say, then the shortcut command, which does the same thing.

You need [Claude Code](glossary.md#claude-code) and a repository to work
in. The example adds a dark mode to an application; use your own idea.

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

It may ask which [skills](glossary.md#skill) to install, and for a few words
about your project. Then it adds a `_compagnon/` folder and gives Claude Code
the skills. Restart Claude Code so it sees them.

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
**design** (how) and the **tasks** (the steps). Read them as you would a
colleague's proposal. To change something, just say so:

> In the plan, keep the user's choice when they pick a theme by hand.

[Reviewing a plan](guides/review-a-plan.md) tells you what to look for in
each file, without the terminal.

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
stops to ask you if something is unclear. Shortcut:
`/compagnon-apply add-dark-mode`.

## 6. File it away (`/compagnon-archive`)

Check the result, then:

> The change is done, archive it.

Claude merges the plan's specs into the project's specs and moves the
change to the archive: the project now remembers why dark mode exists.
Shortcut: `/compagnon-archive add-dark-mode`. Then save the result in the
project's history ([commit](glossary.md#commit) it), or ask Claude to do it.

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

## What's next

- [The workflow](workflow.md) details each step.
- [Claude Code skills](skills.md#how-to-ask) lists, for every skill, ways
  to ask for it.
- Say "show me around Compagnon" for a guided tour of your project.
