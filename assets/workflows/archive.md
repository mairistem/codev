Close a compagnon change: merge its deltas into the main specs and move the
folder to `_compagnon/changes/archive/<date>-<name>/`.

**When to use it.** After a successful `compagnon-apply`, when every task in
`tasks.md` is checked and the change is ready to be filed away.

**Strict refusal on validation errors.** The CLI runs an internal
`validate` pre-flight; if it reports an error, the skill does not insist
and points to `compagnon validate` for the details.

---

{{fragment:next-steps}}

## Input

A change name as argument, or nothing (implicit resolution if there is only
one active change).

## Steps

### 1. Resolve the change and check the planning

Without an explicit name:

```bash
compagnon list
```

- A single active change → use it.
- Several → ask the user, listing the names.
- None → say so and stop.

Then:

```bash
compagnon status --change "<name>" --json
```

{{fragment:version-check}}

If `isPlanningComplete` is `false`, stop: planning is not ready. Name what
is missing and suggest finishing the plan, say “finish the plan of <name>”
(shortcut: `/compagnon-propose`); do not run `archive`.

### 2. Run the archive in JSON mode

```bash
compagnon archive --change "<name>" --json
```

On **success** (exit 0), the JSON received is an `ArchiveReportV1` — a
public, versioned contract. Fields to use:

- `changeName` — to confirm what was acted on;
- `created[]` — paths of main specs that were just created;
- `updated[]` — paths of main specs that were just modified;
- `unchanged[]` — paths of specs already up to date at merge time;
- `movedTo` — dated archive path of the change folder;
- `status[]` — empty.

On a **non-zero exit**, read `status[0].code`:

- If `code == "validation_failed"` → reply with this sentence, said in
  the user's language, and nothing more:
  > The change has errors. Say “show me the errors” and I will list them (shortcut: `compagnon validate <name>`).
  Do not retry. Do not guess. Do not quote the human
  message (it may be reworded).
- If `code == "review_blocked"` → reply with this sentence, said in the
  user's language, and nothing more:
  > The review record blocks archiving. Say “show me what is left to decide” (shortcut: `compagnon review --change <name>`).
  Do not retry. Do not edit `review.md`: what blocks is a
  decision for the user, or a review step to run again.
- For **any other code** → relay `status[0].message` as is, and stop.
  The skill does not interpret.

### 3. Report to the user (success)

Expected summary, one line:

> ✓ Archived "`<changeName>`" — `N` spec(s) created, `M` updated,
> `K` unchanged.

Then the lists by category if non-empty (as for `sync`).

Finally, on its own line:

> Moved to: `<movedTo>`

## Output

The success report from step 3, or the short refusal on a validation
error or a blocked review, or the raw CLI message on any other error.

## Guardrails

- **Write nothing yourself** — everything goes through `compagnon archive`. The
  skill never modifies specs or folders directly.
- **Do not work around a review refusal** — a `review_blocked` stops the
  skill; it never writes an accepted risk or a deferral in the user's
  place.
- **Do not work around a validation refusal** — a `validation_failed`
  stops the skill. Fixing is the user's job, guided by `compagnon validate`,
  not the skill's.
- **Do not parse any JSON other than `archive`'s** — the skill knows the
  shape of no other contract.
- **Do not reinvent the CLI's messages** — for any error code other than
  `validation_failed` and `review_blocked`, the JSON `message` is relayed
  as is.
