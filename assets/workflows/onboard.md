Introduce Compagnon to a user discovering it: what the tool does, the current
state of the project, and the recommended next action.

**Strict read-only boundary.** This skill **modifies nothing**. It does not
create a change, does not run `compagnon init` on the user's behalf, does not
check off a task. It **guides** — the user acts.

---

## Input

Nothing. The skill is invoked without arguments. If the user asks a
question, it informs about Compagnon; it does not implement anything.

## Steps

### 1. Describe Compagnon (block 1 — "Compagnon is")

Exactly three sentences, displayed as markdown:

> **Compagnon** is a versioned planning tool for software projects: every
> change goes through a **propose → apply → archive** cycle documented
> in `_compagnon/`. You move through the steps by asking Claude in your
> own words — the `/compagnon-<step>` commands are only shortcuts — and the
> `compagnon` CLI does the atomic work underneath. Your planning lives in
> the repository, alongside the code.

### 2. Read the project state (block 2 — "here you have")

**First check whether the repository is initialized**:

```bash
compagnon list
```

This command lists the active changes. Three cases:

- **Success** → the repository is initialized. Continue.
- **Failure with code `no_compagnon_root`** → no `_compagnon/` under the
  current folder. Go straight to step 3, "`compagnon init`" branch.
- **Other failure** → relay the message as is and stop.

If the repository is initialized, complete the picture:

```bash
compagnon list --specs           # capabilities already specified
compagnon decision list          # local and inherited decisions
ls _compagnon/changes/archive/   # number of archived changes (optional)
```

Count the entries of the form `<date>-<name>/` in
`_compagnon/changes/archive/` (ignore `.gitkeep` and hidden files).

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

Then check what the Compagnon setup still lacks:

```bash
compagnon doctor --json
```

Each entry of `checks` has an `id`, a `status` (`ok`, `missing`,
`warning`) and a `fix`. Add to the "here you have" block:

- the checks whose `status` is `missing` — **at most five**, one line
  each: a few plain words of your own saying what is missing, then the
  check's `id` and its `fix` verbatim:

  > - Setup to complete:
  >   - your project's rules are not written yet (`config.rules`) — fix: `/compagnon-configure`
  >   - no behavior is specified yet (`specs`) — fix: `/compagnon-propose <a change>`

- or, when none is `missing`, one line: "Compagnon setup complete".

Warnings may be summarized as a count ("1 warning, see `compagnon doctor`").
Never run a fix yourself — not even `compagnon init` or `compagnon update`: name
it, the user acts. If `compagnon doctor` fails or does not exist (an older
Compagnon), skip this list silently and go on. The recommended next action
of block 3 does not change.

### 3. Recommend the next action (block 3 — "what's next")

Ordered resolution, from the most constraining to the most general.
**Exactly one case applies**:

| State | Recommendation |
|---|---|
| `_compagnon/` missing | `compagnon init`, for the user to type in a terminal |
| No active change AND thin config (no entry under `rules:`) | **Configure first** — Claude will enrich the config from the project: say “configure Compagnon for this project” (shortcut: `/compagnon-configure`), on the last line; name before it the first change that comes next, say “plan <an idea>” (shortcut: `/compagnon-propose <an-idea>`) |
| No active change, non-thin config | **Read `README.md`** to get a feel for the project, then say “plan <an idea>” (shortcut: `/compagnon-propose <an-idea>`), on the last line; name before it the alternative, say “let's explore <a topic>” (shortcut: `/compagnon-explore <topic>`) |
| A single active change, planning incomplete | Say “finish the plan of <that-name>” (shortcut: `/compagnon-propose <that-name>`) |
| A single active change, planning complete | Say “implement <that-name>” (shortcut: `/compagnon-apply <that-name>`) |
| Several active changes | List them, each with its request and shortcut as in the two rows above; the last line asks which one to continue, in plain words, without a shortcut |

To assess whether the config is "thin", read `_compagnon/config.yaml` and
look at the `rules:` key:

- **`rules:` missing or empty** → thin. Configuring really has
  something to contribute.
- **`rules:` has at least one entry** → non-thin. The user has
  already written their rules; the hint no longer applies.

The `context:` field does **not** factor into the decision. It is
usually filled by the `compagnon init` probe from the detected manifests,
which makes its length meaningless — a TypeScript project with several
dependencies has a long context without the user having written
anything.

How to determine whether a change's planning is complete:

```bash
compagnon status --change "<name>"
```

The last lines say "Planning: N/N artifacts" and "Planning is
complete." when everything is ready.

**Make the suggestion actionable**: give the request to say, then the
exact command as the shortcut, not just "the propose step". The reply
ends on the one recommended request, alone on its last line; any other
step it names comes before. A useful example on a new project whose
config is thin:

> **What's next**: your config is sparse. Claude can read the project
> (README, docs, a sample of the code) and enrich `_compagnon/config.yaml`.
> Once that is done, you will plan your first change — say “plan <an idea>” (shortcut: `/compagnon-propose <an-idea>`).
> For now, say “configure Compagnon for this project” (shortcut: `/compagnon-configure`).

On a project whose config is already well filled in:

> **What's next**: start by reading `README.md` to get a feel for the
> project. If you have a question but no idea for an action yet, say
> “let's explore <a topic>” (shortcut: `/compagnon-explore <topic>`).
> When an idea comes up, say “plan <an idea>” (shortcut: `/compagnon-propose <an-idea>`).

With an active change whose plan is not finished:

> **What's next**: the plan of `add-user-auth` is not finished.
> Say “finish the plan of add-user-auth” (shortcut: `/compagnon-propose add-user-auth`).

With several active changes, each is listed with its request and
shortcut, and the last line asks which one, without a shortcut:
> **What's next**: two changes are in progress.
> - `add-user-auth`, plan not finished: say “finish the plan of add-user-auth” (shortcut: `/compagnon-propose add-user-auth`)
> - `export-csv`, plan complete: say “implement export-csv” (shortcut: `/compagnon-apply export-csv`)
>
> Which one do you want to continue?

A non-case: if the user has already asked for the tour for the Nth
time, the state is stable; say the same thing again without
apologizing — the skill's role is to stay **predictable**.

## Output

The **three blocks** in order: description, state, what's next. Separated
by a blank line. No introduction ("Here is…"), no conclusion ("I hope
that…"). The reader wants the map of the terrain, not a guided tour.
The last line is the one recommended request with its shortcut — or,
with several active changes, the question of which one.

{{fragment:next-steps}}

## Guardrails

- **No writing** — no file, no skill, no change, no decision.
  `allowed-tools` contains only `Bash(compagnon:*), Read, Glob`.
- **Never run a doctor fix** — `compagnon doctor` only reports; relay its
  `fix` lines, do not execute them.
- **Never run `compagnon init`** — if the repository is not initialized,
  display the command and let the user type it.
- **No excess detail** — numbers, not exhaustive lists. A user who
  wants the details can say “let's explore <a topic>” (shortcut: `/compagnon-explore`),
  or run the commands `compagnon decision list` / `compagnon list --specs`.
- **Never suggest a missing skill** — only recommend a step, and its
  shortcut, if its workflow exists in the installed catalog. In
  practice, this skill always ships with the workflows of the default
  catalog; the opt-in ones (`apply`, `sync`, `archive`, `update`) may
  not be installed. If you recommend implementing a change when the
  `apply` skill is not installed, the user will find neither the skill
  nor its shortcut — check first by listing the skills in
  `.claude/skills/`.
