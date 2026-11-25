# Proposal: ship `/codev-sync` and `/codev-archive`

## Why

The cycle is closed on the CLI side (`codev sync`, `codev archive`) and on
the planning side (`/codev-propose`, `/codev-apply`). The last missing link:
making the closing step invocable **in the chat**, without the user having
to leave Claude Code to type a bash command. Two skills — that is not much —
but they transform the rhythm of use: "plan, implement, archive" becomes
three consecutive slash commands.

## What Changes

- **New `sync` workflow** in the catalog, invocable as `/codev-sync`:
  resolves an active change, runs `codev sync <name>`, summarizes the main
  specs created/modified/unchanged. Never moves the change.
- **New `archive` workflow** in the catalog, invocable as `/codev-archive`:
  checks that planning is complete, runs `codev archive <name>`,
  summarizes the archive destination and the main specs touched. If
  `codev archive` refuses (validate pre-flight failed), the skill
  explicitly points to `/codev-validate` (upcoming) or `codev validate <name>`
  to see the details — without trying to decide on its own.
- **Two asset files**: `assets/workflows/sync.md` and
  `assets/workflows/archive.md`, loaded at compile time via `include_str!`
  like the other three.
- **These two workflows do not need general `Bash`** — their only effective
  action goes through `codev` itself. `allowed-tools` therefore stays
  `Bash(codev:*), Read` for both (the `Read` serves to reread `tasks.md` if
  the user asks a context question).
- **Structured rendering from the CLI's JSON**: both skills invoke
  `codev sync <name> --json` and `codev archive <name> --json`, read the
  `SyncReportV1` / `ArchiveReportV1` (a public contract already versioned
  and snapshot-tested), and give the user a clean summary — counts per
  category, archive path, stable error code on refusal.
- **The `sync` skill ends with an invitation to archive when the merge
  produced a change**: "The change is ready to be archived if you want to
  close the cycle." A single line, non-directive.

## Capabilities

### New Capabilities

- `skills`

**Ordering note.** The `skills` capability is also declared by the
currently active `skill-apply-change` change. Two changes declaring the
same "new" capability is the normal case: the one archived first creates
`_codev/specs/skills/spec.md` with its own `Purpose` and `ADDED`; the one
archived second sees `merge_into_existing` add its own `ADDED` to the
existing spec — the second delta's `## Purpose` is silently ignored, as
the contract intends. No conflict to expect, whatever the order.

### Modified Capabilities

None — the existing workflows (`propose`, `explore`, `apply`) do not
change.

## Impact

- **Code**: two more `Workflow { … }` entries in the `CATALOG` of
  `codev-agents::workflows`, a dedicated test
  (`workflows::cycle_completion_skills_present_et_restreintes`) that checks
  that `sync` and `archive` are there, with the right `allowed-tools` (no
  general `Bash`) and a clear description.
- **Config**: add `- sync` and `- archive` to the `workflows` list of this
  repository's `_codev/config.yaml`, so that the skill exists after a
  `codev update`.
- **Default catalog**: do NOT add `sync` or `archive` to
  `DEFAULT_WORKFLOWS` — same decision as for `apply`, to stay consistent:
  the default catalog is limited to what prepares the work (propose,
  explore); the rest is opt-in project by project.
- **Out of scope**:
  - **`update`** as a skill — different semantics (revises artifacts
    already written, closes nothing), to be handled separately if the
    need arises.
  - **Default auto-installation** — cf. previous paragraph.
  - **Parsing JSON from commands other than sync/archive** — the skill
    only reads the JSON of these two specific commands. A change to the
    shape of `codev status --json` or of another contract does not affect
    it.
