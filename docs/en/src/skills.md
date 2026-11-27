# Asking Claude

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

Each skill gets only the tools its job needs: only `/compagnon-apply` may
run any command, because it runs your [tests](glossary.md#test). In
isolated review mode, `/compagnon-propose` and `/compagnon-apply` also use
`Agent`, Claude Code's sub-agent tool. See
[Tool permissions](reference/configuration.md#tool-permissions).

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
- "Prepare a plan for PROJ-123."

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

## Update notice

Five skills check, before they start, that you run the right version of
Compagnon: `/compagnon-propose`, `/compagnon-apply`, `/compagnon-update`,
`/compagnon-sync` and `/compagnon-archive`. If your team requires a
version you do not have, or a newer one is out, the skill says so in one
line and asks you **Update now** or **Later**, once per request, before
doing anything else:

- **Update now** installs the new version, then carries on;
- **Later** silences the notice for that version for 7 days, then carries
  on.

When your team requires the version, **Later** silences nothing, and
`/compagnon-sync` and `/compagnon-archive` stop: the message gives the
version to install.

<details>
<summary>Under the hood: how a skill knows about a new version</summary>

The five skills read the `update` and `versionRequirement` objects of their
first `compagnon status --json`; no skill needs more than
`Bash(compagnon:*)` for it. **Update now** runs `compagnon self-update --yes`;
**Later** runs `compagnon self-update --snooze`. The team's requirement is
`compagnon_version` in the [configuration](reference/configuration.md#compagnon_version):
`compagnon sync` and `compagnon archive` refuse to run with the wrong version
(`compagnon_version_unsatisfied`). `/compagnon-apply` asks before the first
task. `/compagnon-explore`, `/compagnon-onboard` and `/compagnon-configure`
never ask; `/compagnon-onboard` passes on the result of `compagnon doctor`
instead, whose `compagnon_version` check says the same thing.

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

## Go further

- [How it works](workflow.md): what each skill does, step by step, and
  the files it writes.
- [Start from a Jira ticket](guides/mcp.md): "Prepare a plan for
  PROJ-123".
- [Roles](roles.md): the criteria Claude checks at each step.
- [Configuration](reference/configuration.md#the-installed-skills):
  choosing which skills to install, their tool permissions, and editing a
  generated skill.
