Revise an already written planning artifact of an active compagnon change —
proposal, specs, design or tasks — while keeping it consistent with the
other artifacts.

**Strict boundary.** This skill reads and writes **only** files under
`_compagnon/changes/<name>/`. It does **not modify** project code — that is
the apply step's job after the revision. It does **not create** a missing
artifact — that is the propose step's job. It does **not touch** an
already archived change — an archived change is history.

---

{{fragment:next-steps}}

## Input

Two pieces, in this order:

1. The identifier of the artifact to revise — `proposal`, `specs`, `design`
   or `tasks`. If the user does not name it, ask explicitly which one — do
   not guess.
2. A free-form description of the requested revision.

Optional: `--change <name>` if several changes are active.

## Steps

### 1. Resolve the change and check its state

```bash
compagnon list
```

- A single active change → use it.
- Several → if the user did not name one, ask.
- None → say so and stop.

If the given name does not appear in `compagnon list` — because it is archived,
or does not exist — **refuse**. Remind the user that an archived change is
frozen; correcting it requires un-archiving it by hand.

Then:

```bash
compagnon status --change "<name>" --json
```

{{fragment:version-check}}

Find the requested artifact in `artifacts[]`:

- status `done` → OK, it can be revised.
- status `ready` or `blocked` → the artifact does not exist yet.
  **Refuse** and explicitly suggest creating it with the plan, say
  “finish the plan of <name>” (shortcut: `/compagnon-propose`).
- status `skipped` → **refuse** and explain that this artifact is
  disabled by `skip_specs` in `change.yaml`.

### 2. Read what exists

Read, from disk (never from the conversation):

- the artifact to revise itself;
- the other artifacts of the change that might be affected.

This reading is what lets you spot the ripple in step 4 — that is why it
happens **before** writing, not after.

### 3. Apply the revision

Two forms, depending on the scope:

- **Targeted edit** — a paragraph to adjust, a decision to replace, a task
  to reword → `Edit` with a precise `old_string`.
- **Full rewrite** — an artifact that must be largely redone → `Write`,
  but keep in mind that the other artifacts will rely on its new shape.

Stay within the artifact's contract — expected sections, scenario format,
checkboxes. The change is about **content**, not structure.

Write in the language set by the `language:` key of `_compagnon/config.yaml`
(`en` when absent), whatever language the conversation is in. Structural
keywords — headings, delta sections, requirement and scenario headings,
the scenario lines, the normative words — keep the language the artifact
you revise already writes them in, so that a change stays in one
language; a file you create writes them in `keywordLanguage`, exactly as
`compagnon instructions <artifact> --change "<name>" --json` gives them in
`template` and `keywords`. Never translate a keyword yourself.

### 4. Detect and report the ripple

After writing, compare the state of the change with what changed:

- **`proposal` revised**:
  - a capability listed under `### New Capabilities` or
    `### Modified Capabilities` that disappears → name the file
    `specs/<capability>/spec.md` that becomes orphaned;
  - a new capability that appears → name the `specs/<capability>/` file
    that is now missing.
- **`specs` revised**:
  - a removed requirement that was cited by a task → name the task
    concerned in `tasks.md`;
  - a renamed scenario — a `tasks.md` that cited the old name is
    flagged.
- **`design` revised**:
  - a cited decision that disappears on one side and a constraint on the
    other → flag it, but let the user decide (`compagnon decision list` can
    help).
- **`tasks` revised**:
  - a task that contradicts a requirement in `specs/` → name the
    requirement at stake.

**Report**, **suggest** the next action (often: say “update the specs: …”
(shortcut: `/compagnon-update specs …`), or say “delete the orphaned spec”
for `_compagnon/changes/<name>/specs/<capability>/spec.md`),
but **do not act** without explicit confirmation.

### 5. Final guardrail — `compagnon validate`

Whatever the ripple, at the end, run:

```bash
compagnon validate "<name>"
```

Relay the report as is. If `validate` reports errors, cite the stable
codes; do not rewrite the human messages.

### 6. Final summary

One or two sentences:

- the file or files touched;
- the `compagnon validate` verdict;
- the recommended next action — often say “implement the change”
  (shortcut: `/compagnon-apply`) if the revision affects the
  implementation, or say “archive the change”
  (shortcut: `/compagnon-archive`) if the acknowledged gap is closed.

{{fragment:memory-capture}}

## Output

The step 6 summary, preceded by the ripple report if there is one and its
status (accepted by the user, declined, deferred to a separate revision).

## Guardrails

- **No code** — every path written by this skill lives under
  `_compagnon/changes/<name>/`. Refuse if the user asks to adjust code: that is
  the apply step's job after the revision.
- **No creation** — if the requested artifact does not exist, refuse and
  suggest finishing the plan (shortcut: `/compagnon-propose`). Never create a `proposal.md`, `design.md` or
  `tasks.md` from scratch in this skill.
- **Nothing archived** — a change living under `changes/archive/` is
  history. Refuse and remind the user that correcting it requires
  un-archiving it by hand.
- **No cascade** — a detected ripple is reported, never applied without
  confirmation.
- **Always `validate` at the end** — it is the only automatic guardrail.
