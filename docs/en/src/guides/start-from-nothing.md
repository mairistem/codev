# Starting from nothing

This page is for you if you do not code and want Claude to build
something for you — a sign-up form, a small tool, a page — starting with
nothing but the idea. You will type two commands, once. After that, you
only talk to Claude.

## What to get first

Two things, before any command:

- **Claude Code.** It is the [Claude Code](../glossary.md#claude-code)
  assistant that writes the plan and builds what you ask for. Get it from
  [claude.com/claude-code](https://claude.com/claude-code), which explains
  how to install it. It needs a Claude account with a subscription that
  includes Claude Code; that page says which.
- **A folder for your project** on your computer, empty to begin with.
  Create it as you would any folder, for example `event-signup` in your
  Documents. Everything Claude builds will go there.

## Two commands, typed once

These two commands are the only ones you type. If someone in your team is
used to a [terminal](../glossary.md#terminal) (the window where you type
commands), they can do this part for you.

**1. Install Compagnon.** Open a terminal: on macOS, the Terminal app (press Cmd + Space, type
"Terminal", then Enter); on
Windows, PowerShell. Copy the line for your computer, paste it, then press
Enter.

On macOS or Linux:

```bash
curl -sSL https://raw.githubusercontent.com/mairistem/compagnon/main/install.sh | sh
```

On Windows, in PowerShell:

```powershell
iwr -useb https://raw.githubusercontent.com/mairistem/compagnon/main/install.ps1 | iex
```

If the installer prints a line to add, or if the next command is "not
found", close the terminal and open a new one. If it still fails,
[Installation](../installation.md) explains why, for the person helping
you.

**2. Prepare your folder.** Open a terminal in your project folder: on
macOS, right-click the folder in the Finder, then Services and "New
Terminal at Folder"; on Windows,
right-click inside the folder in File Explorer, then "Open in Terminal". Then type:

```bash
compagnon init
```

It may ask which [skills](../glossary.md#skill) to give Claude, and for a
few words about your project: the proposed choice is fine; for the
words, always type one sentence about your idea: an empty answer opens
an awkward text editor (if that happens, call the person helping you). It adds a `_compagnon/` folder, where your plans will be
kept. You can close the terminal now.

## Your first request

Open Claude Code on your project folder, for example from the Claude
desktop app, choosing that folder. Then say, in your own words, what you
want, and ask for a plan first:

> Prepare a plan for a sign-up form for an event.

Claude writes a plan and stops: it builds nothing yet.

<details>
<summary>Under the hood: opening Claude Code from the terminal</summary>

In a terminal opened in the project folder, Claude Code starts with:

```bash
claude
```

Start it after `compagnon init`, or restart it, so that it sees the skills.

</details>

## Read the plan, then let Claude build

The plan says why, what will be true afterwards, how, and in which steps.
[Reviewing a plan](review-a-plan.md) tells you what to check in each part;
[An annotated example plan](example-plan.md) shows a complete one. To
change something, say it in plain words:

> In the plan, also ask for the number of guests.

When the plan says what you want:

> The plan is good, implement it.

Claude builds what the plan says, step by step, and asks you when
something is unclear. To see the result, ask:

> How can I try the form on my computer?

## Putting it online

Compagnon helps Claude plan and build; putting the result online, so that
others can use your form, is outside Compagnon. Ask a developer, or ask
Claude how to do it.
