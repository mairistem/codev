Close a codev change: merge its deltas into the main specs and move the
folder to `_codev/changes/archive/<date>-<name>/`.

**When to use it.** After a successful `codev-apply`, when every task in
`tasks.md` is checked and the change is ready to be filed away.

**Strict refusal on validation errors.** The CLI runs an internal
`validate` pre-flight; if it reports an error, the skill does not insist
and points to `codev validate` for the details.

---

## Input

A change name as argument, or nothing (implicit resolution if there is only
one active change).

## Steps

### 1. Resolve the change and check the planning

Without an explicit name:

```bash
codev list
```

- A single active change → use it.
- Several → ask the user, listing the names.
- None → say so and stop.

Then:

```bash
codev status --change "<name>" --json
```

If `isPlanningComplete` is `false`, stop: planning is not ready. Name what
is missing and suggest `/codev-propose`; do not run `archive`.

### 2. Run the archive in JSON mode

```bash
codev archive --change "<name>" --json
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

- If `code == "validation_failed"` → reply **exactly**:
  > The change has errors. Run `codev validate <name>` to see the details.
  Nothing more. Do not retry. Do not guess. Do not quote the human
  message (it may be reworded).
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
error, or the raw CLI message on any other error.

## Guardrails

- **Write nothing yourself** — everything goes through `codev archive`. The
  skill never modifies specs or folders directly.
- **Do not work around a validation refusal** — a `validation_failed`
  stops the skill. Fixing is the user's job, guided by `codev validate`,
  not the skill's.
- **Do not parse any JSON other than `archive`'s** — the skill knows the
  shape of no other contract.
- **Do not reinvent the CLI's messages** — for any error code other than
  `validation_failed`, the JSON `message` is relayed as is.
