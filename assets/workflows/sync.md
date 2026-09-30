Merge a codev change's deltas into the main specs, **without moving the
change**. The change stays active where it is.

**When to use it.** When a new capability must appear in the specs before
another change builds on it, or when you want to review the merge before
archiving. In the normal case, `codev-archive` already runs the sync as a
pre-flight step — there is no need to `sync` then `archive` separately.

---

## Input

A change name as argument, or nothing (in which case the change is resolved
implicitly if there is only one active change).

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

If `isPlanningComplete` is `false`, stop: planning is not ready. Name the
missing artifacts and suggest `/codev-propose` or `/codev-continue`
(depending on what already exists). Do not run `sync`.

### 2. Run the merge in JSON mode

```bash
codev sync --change "<name>" --json
```

The JSON received is a `SyncReportV1` — a public, versioned contract.
Fields to use:

- `changeName` — to confirm what is being acted on;
- `created[]` — paths of main specs that were just created;
- `updated[]` — paths of main specs that were just modified;
- `unchanged[]` — paths of specs that were already up to date;
- `status[]` — empty on success.

On a non-zero exit, read `status[0].code` and `status[0].message`, relay
the message as is and stop.

### 3. Report to the user

Expected summary, one line:

> ✓ Synced "`<changeName>`" — `N` spec(s) created, `M` updated,
> `K` unchanged.

Then, if at least one of the `created` or `updated` lists is non-empty,
list them by category, each on its own line:

```
Created:
  <path>
Updated:
  <path>
```

### 4. Finish — suggest archiving if something changed

If `created` or `updated` is non-empty, add **a single line**, without
pressing the point:

> The change is ready to be archived if you want to close the cycle.

If both lists are empty (`unchanged` only), suggest nothing — a no-op
sync does not call for an archive.

## Output

What you reported in step 3, plus the step 4 line if applicable. Nothing
more. The skill stops there; the user decides what comes next (review,
archive, or something else).

## Guardrails

- **Never move the change** — the power of `codev sync` ends at the
  merge. Moving is the job of `codev-archive`.
- **Do not archive** — if the user wanted to archive, they would have
  typed `/codev-archive`.
- **Do not parse any JSON other than `sync`'s** — the skill knows the
  shape of no other contract.
- **Do not hide an error** — if the CLI returns a non-zero exit, relay
  it; do not silently retry.
