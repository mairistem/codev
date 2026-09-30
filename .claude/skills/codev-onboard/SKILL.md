---
name: codev-onboard
description: "Introduce codev to a user discovering it: what the tool does, the current state of the project, and the recommended next action. Strictly read-only — modifies and creates nothing."
allowed-tools: "Bash(codev:*), Read, Glob"
license: MIT
metadata:
  generator: codev
  version: "0.3.2"
---

Introduce codev to a user discovering it: what the tool does, the current
state of the project, and the recommended next action.

**Strict read-only boundary.** This skill **modifies nothing**. It does not
create a change, does not run `codev init` on the user's behalf, does not
check off a task. It **guides** — the user acts.

---

## Input

Nothing. The skill is invoked without arguments. If the user asks a
question, it informs about codev; it does not implement anything.

## Steps

### 1. Describe codev (block 1 — "codev is")

Exactly three sentences, displayed as markdown:

> **codev** is a versioned planning tool for software projects: every
> change goes through a **propose → apply → archive** cycle documented
> in `_codev/`. Each step is driven by a Claude Code skill
> (`/codev-<step>`), and the `codev` CLI does the atomic work
> underneath. Your planning lives in the repository, alongside the
> code.

### 2. Read the project state (block 2 — "here you have")

**First check whether the repository is initialized**:

```bash
codev list
```

This command lists the active changes. Three cases:

- **Success** → the repository is initialized. Continue.
- **Failure with code `no_codev_root`** → no `_codev/` under the
  current folder. Go straight to step 3, "`codev init`" branch.
- **Other failure** → relay the message as is and stop.

If the repository is initialized, complete the picture:

```bash
codev list --specs           # capabilities already specified
codev decision list          # local and inherited decisions
ls _codev/changes/archive/   # number of archived changes (optional)
```

Count the entries of the form `<date>-<name>/` in
`_codev/changes/archive/` (ignore `.gitkeep` and hidden files).

Display a compact summary — **count, do not list exhaustively**:

> Here you have:
> - N main spec(s): `<list of ids>` (up to 5; otherwise
>   "and K more")
> - M local decision(s) in effect, P inherited
> - Q active change(s): `<list of names>`
> - R archived change(s) *(**this line only if R > 0**, so as not to
>   clutter a new project)*

A freshly initialized project (no spec, no change, no decision) is a
normal case — say so: "freshly initialized project, ready for your
first change".

### 3. Recommend the next action (block 3 — "what's next")

Ordered resolution, from the most constraining to the most general.
**Exactly one case applies**:

| State | Recommendation |
|---|---|
| `_codev/` missing | `codev init` |
| No active change AND thin config (no entry under `rules:`) | **`/codev-configure` first** — Claude will enrich the config from the project; then `/codev-propose <an-idea>` |
| No active change, non-thin config | **Read `README.md`** to get a feel for the project, then `/codev-propose <an-idea>`; `/codev-explore <topic>` as an alternative |
| A single active change, planning incomplete | `/codev-propose <that-name>` to continue |
| A single active change, planning complete | `/codev-apply <that-name>` |
| Several active changes | List them; let the user choose the skill |

To assess whether the config is "thin", read `_codev/config.yaml` and
look at the `rules:` key:

- **`rules:` missing or empty** → thin. `/codev-configure` really
  has something to contribute.
- **`rules:` has at least one entry** → non-thin. The user has
  already written their rules; the hint no longer applies.

The `context:` field does **not** factor into the decision. It is
usually filled by the `codev init` probe from the detected manifests,
which makes its length meaningless — a TypeScript project with several
dependencies has a long context without the user having written
anything.

How to determine whether a change's planning is complete:

```bash
codev status --change "<name>"
```

The last lines say "Planning: N/N artifacts" and "Planning is
complete." when everything is ready.

**Make the suggestion actionable**: quote the exact command to type,
not just "run `codev-propose`". A useful example on a new project whose
config is thin:

> **What's next**: your config is sparse. Start with
> `/codev-configure` — Claude will read the project (README, docs,
> a sample of the code) and enrich `_codev/config.yaml`. Then type
> `/codev-propose <an-idea>` for your first change.

On a project whose config is already well filled in:

> **What's next**: start by reading `README.md` to get a feel for the
> project. Then, when an idea comes up, type
> `/codev-propose <an-idea>`. Alternative if you have a question but
> no idea for an action yet: `/codev-explore <topic>`.

And with an active change:

> **What's next**: you can type `/codev-propose add-user-auth` to
> plan your first change.

A non-case: if the user has already typed `/codev-onboard` for the
Nth time, the state is stable; say the same thing again without
apologizing — the skill's role is to stay **predictable**.

## Output

The **three blocks** in order: description, state, what's next. Separated
by a blank line. No introduction ("Here is…"), no conclusion ("I hope
that…"). The reader wants the map of the terrain, not a guided tour.

## Guardrails

- **No writing** — no file, no skill, no change, no decision.
  `allowed-tools` contains only `Bash(codev:*), Read, Glob`.
- **Never run `codev init`** — if the repository is not initialized,
  display the command and let the user type it.
- **No excess detail** — numbers, not exhaustive lists. A user who
  wants the details runs `/codev-explore` or the commands
  `codev decision list` / `codev list --specs`.
- **Never suggest a missing skill** — only recommend
  `/codev-<something>` if the workflow exists in the installed
  catalog. In practice, this skill always ships with the workflows of
  the default catalog; the opt-in ones (`apply`, `sync`, `archive`,
  `update`) may not be installed. If you recommend `/codev-apply`
  when it is not installed, the user will not find the skill — check
  first by listing the skills in `.claude/skills/`.