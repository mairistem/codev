Implement the tasks of a codev change — work through each `- [ ]` box in
`tasks.md` in order, check them off as you go, stop at the first blocker.

**Implementation boundary.** This workflow **writes project code**: it is
the only codev workflow that touches anything outside the `_codev/` folder.
In return, it is strictly confined to the named change: it modifies no other
change, it does not archive, it does not sync. Those are explicit next
steps, for the user to request.

---

## Input

A change name as argument, or nothing (in which case the change is resolved
implicitly if there is only one active change).

## Roles

Steps 3 and 5 belong to a role. At each one, run:

```bash
codev roles --at <step> --change "<name>" --json
```

Apply every criterion of every role returned (`roles[].doneWhen`), the
project's included, using the role's `knowledge` when it has one. Its
`By stack` part comes already filtered to the project's languages
(`stacks`): use it as served. If `roles` is empty, the role is disabled by
configuration: skip the step, and say so in the summary.

## Steps

### 1. Resolve the change and check that planning is complete

If the user named a change, use that one. Otherwise, run:

```bash
codev list
```

- A single active change → use it.
- Several active changes → ask the user which one, listing the names. Do
  not guess.
- No active change → say so, and suggest `/codev-propose` to create one.

Then check the planning:

```bash
codev status --change "<name>" --json
```

{{fragment:version-check}}

If `isPlanningComplete` is `false`, stop: planning is not ready. Say which
artifacts are missing and suggest `/codev-propose` or editing by hand.
Implement nothing.

### 2. Read the tasks

```
Read _codev/changes/<name>/tasks.md
```

The expected format is strict:

- a task: `- [ ] X.Y Description, verified by <test or command>`
- a checked task: `- [x] X.Y …`
- groups under `## N. …` headings

If you find a different format (`-[ ]` without a space, uppercase `- [X]`,
another marker such as `- [-]`), point it out to the user and offer to fix
it before continuing.

### 3. Work through each unchecked task, in file order — `code.implement`

Ask for the roles of this step once, before the first task: the
developer's criteria apply to every task. The project's `rules` and
`context` come from `codev instructions tasks --change "<name>" --json`.

For each `- [ ]` encountered, in the order it appears:

**a. Announce.** "Task X.Y: <description summarized in a fragment>."

**b. Study.** Read the files the task concerns.

**c. Implement.** Write what the task asks for.

**d. Verify.** Run the command, the test, the observation the task cites.

**e. Check off.** Edit `tasks.md` with `Edit`: the line `- [ ] X.Y …`
becomes `- [x] X.Y …`. Be precise — a `replace_all` would also replace
tasks from other changes read earlier; avoid it. Use the full old line as
the anchor.

**f. Brief feedback to the user.** "✓ X.Y — <what it produced, in one
line>". Move on to the next one.

### 4. Stop on ambiguity or blocker

A task cannot be carried out in two cases:

- **Material ambiguity**: its wording allows several interpretations that
  would materially change the result (API choice, output format, behavior
  on an edge case). In that case:
  - **do not check it off**,
  - describe the interpretations to the user,
  - suggest splitting the task into `X.Y.a` / `X.Y.b` in `tasks.md`
    — but let the user decide or rephrase.
- **Technical blocker**: missing dependency, a test that cannot run, a
  command that requires manual intervention. In that case:
  - **do not check it off**,
  - describe the blocker,
  - suggest the resolution you think is best, without applying it.

In both cases, stop after the current task. The following tasks are not
processed until the obstacle is cleared.

### 5. Review the change — `code.review`

When every box is checked, re-read only the code this change added or
modified — its diff — with every role returned, each along its criteria
and knowledge: the newcomer, as a developer joining the project who has
seen neither the plan nor this conversation; and the experts the
proposal's `## Domains` activates, asking whether the implementation
respects their domain.

Simplify directly what is unambiguous, then re-run the affected tests.
Never widen the change's scope, never touch code the change did not add or
modify. The newcomer never changes behavior. An expert never changes
behavior silently: a behavior-changing fix becomes a review note, unless it
is a clear defect within the change's scope — then fix it, add a test that
fails without the fix, and name the fix in the summary. Keep the rest for
the review notes. Here `<step>` is `code.review`.

Now that the code exists, the roles of this step come with the domains
its diff suggests (`suggestions`). Every suggestion whose `state` is
`unticked` goes under **To decide**, with its first match and the two
choices: tick the domain — its expert then reviews this code: ask for the
roles of the step again and run that expert's review — or keep it
unticked with a reason on its line. Do not edit the proposal yourself:
the user decides.

{{fragment:review-record}}

### 6. Finish — suggest archiving as an explicit next step

Then run the project's full test suite once. If it fails, report the
failure instead of the summary below: the change is not done.

Otherwise, summarize:

- number of tasks done,
- main files touched (one line),
- the defects the experts fixed in step 5, one line each,
- **Review notes** — from the record's `code.review` findings reported to
  the human and not rejected, at most five items in total across the
  roles, ranked by severity, one line each, tagged with the role id:
  `[<role-id>] <finding id> (<status>) what — where — why`. A
  high-severity `security` or `privacy` finding is always listed, even
  beyond five. If nothing is worth raising, write
  "No review concern found"; never invent notes. When no role is active at
  step 5, write "The review is disabled by configuration" instead,
- **To decide** — from step 5: what only the user can decide, the
  suggested domains still `unticked` included, or "Nothing to decide".
  `codev archive` refuses while a `high` finding is `open` or `disputed`,
- "The change is ready to be archived. Run `/codev-archive` or
  `codev archive` as the next step."

**Do not archive yourself.** The user must review first.

{{fragment:memory-capture}}

## Output

A final summary, as described in step 6.

## Guardrails

- **Change boundary**: do not modify any file of another active or
  archived change. If a task forces you to touch another change, that is a
  sign the current change is poorly scoped — stop and ask.
- **No automatic sync or archive**: these operations are separate
  decisions, made by the user.
- **No working around errors**: a failing test is not checked off.
  `--no-verify`, `#[ignore]`, a convenient `expect_err`: each is a red flag
  that replaces silence with a lie.
- **Verify before checking off**: checking a box without having run the
  verification the task cites = a regression waiting to happen. Always
  verify.
- **The review record is the truth**: every finding is in `review.md`
  before it is fixed or reported; an accepted risk or the deferral of a
  high finding is recorded only once the user decided it.
- **Targeted editing of tasks.md**: the changed line is identified by its
  full text, never by a bare `- [ ]`. Otherwise, another box would be
  checked by accident.
