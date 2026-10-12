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

{{UPDATE_NOTICE}}

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

### 3. Work through each unchecked task, in file order

For each `- [ ]` encountered, in the order it appears:

**a. Announce.** "Task X.Y: <description summarized in a fragment>."

**b. Study.** Read the files the task concerns. Stay read-only while you
build understanding, then write. Each task in `tasks.md` states how to
verify it is done — that verification is the acceptance criterion, not a
suggestion.

**c. Implement.** Write the code, the tests, the configuration, the
documentation. Target what the task asks for, nothing more. Write tests
like the project's existing test suite (framework, location, naming, unit
or integration level), then by the conventions of its language and the
project's `rules` — `codev instructions tasks --change "<name>" --json`
returns them with the `context`.

**d. Verify.** Run the command, the test, the observation the task cites.
Take the test command from that `context` or `rules`, the manifest or the CI
configuration — never guess one; if none is found, ask the user.
A failing build, a test turning red, a missing behavior: the task is not
done. Fix, run again.

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

### 5. Review as a newcomer

When every box is checked, re-read only the code this change added or
modified — its diff — as a developer joining the project who has seen
neither the plan nor this conversation. Check:

- **Without context**: each changed function makes sense without the plan
  or the conversation.
- **Names** say what things do; no obscure abbreviation.
- **Size and nesting**: a function too long or too nested is split, when it
  splits simply.
- **Cleverness**: dense one-liners, premature genericity, single-use
  abstractions — the simple version wins unless the clever one is justified.
- **Comments** explain the why where it is not obvious, never paraphrase
  the code.
- **Tests** read as a description of the behavior.
- The project's `rules` from step 3c, such as size thresholds.

Simplify directly what is unambiguous, then re-run the affected tests.
Never change behavior, never widen the change's scope, never touch code the
change did not add or modify. Keep the rest for the readability notes.

### 6. Finish — suggest archiving as an explicit next step

Then run the project's full test suite once. If it fails, report the
failure instead of the summary below: the change is not done.

Otherwise, summarize:

- number of tasks done,
- main files touched (one line),
- **Readability notes** — from step 5, at most three items, one line each:
  what, where, why. If nothing is worth raising, write "No readability
  concern found"; never invent notes,
- "The change is ready to be archived. Run `/codev-archive` or
  `codev archive` as the next step."

**Do not archive yourself.** The user must review first.

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
- **Targeted editing of tasks.md**: the changed line is identified by its
  full text, never by a bare `- [ ]`. Otherwise, another box would be
  checked by accident.
