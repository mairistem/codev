# Proposal: Task progress in status, and unchecked tasks at archive

## Why

The `tasks.md` template, the `spec-driven` schema ("parsed to track
progress") and the custom-schemas guide all promise that the checkboxes of
the tracked file measure implementation progress, but no command reads them:
`codev status` says nothing about tasks, and a change can be archived with
boxes left open without anyone being told. Both items sit in the ROADMAP;
they are small, and the promise is already published. This change tells
whoever runs `codev status` and `codev archive`, and lets a team refuse with
`--strict`; relaying the warning through the `/codev-archive` skill is left
to a later change (see Open Questions).

## What Changes

- The checkboxes of the file named by the schema's `apply.tracks` (not
  `tasks.md` by name) are counted: `- [ ]` open, `- [x]` and `- [X]`
  checked, at any indentation, outside fenced code blocks and HTML comments,
  grouped under the `## ` heading that precedes them, with the boxes before
  the first such heading in a group of their own. Only the Markdown structure
  is read, never a word of any language.
- `codev status` prints a `Tasks:` line once the tracked file exists, and
  its JSON gains an additive `tasks` field: `null` while the tracked file is
  absent, otherwise `{ checked, total, groups: [{ id, title, checked, total
  }] }`, with `total: 0` when the file has no box.
- `codev archive` warns when the tracked file still holds unchecked boxes
  (a `tasks_unchecked` warning in the JSON `status` and a human line naming
  each one) and archives anyway; with the new `--strict` flag it refuses
  instead, with the code `tasks_unchecked`, and writes nothing. Its JSON
  gains the same additive `tasks` field.
- **BREAKING** for a change whose tracked file is not UTF-8 text: it could
  be archived until now, and is refused (`tasks_unreadable`) until the file
  is re-saved as UTF-8.
- A tracked file that exists but is not UTF-8 text no longer goes unseen:
  `codev status` keeps working and warns (`tasks_unreadable`), and
  `codev archive` refuses it until it is fixed.
- Only `codev archive` evaluates the boxes, on the change it archives:
  `codev validate` does not, and changes already archived are never
  re-read.
- No marker excludes a box: a step that only happens after the merge (a
  version bump, a tag) is written as a plain bullet, which is not a box.
- The two items leave `ROADMAP.md`; a narrower one replaces them: the
  `/codev-archive` skill relays the unchecked-task warning.

## Capabilities

### New Capabilities

- `task-progress`

### Modified Capabilities

- `changes`
- `spec-merge`

### Removed Capabilities

## Impact

- `codev-core`: a pure parser of the tracked file's text into a progress
  value; `Apply.tracks` gets its first reader.
- `codev-engine`: `status` and `archive` read the tracked file through the
  `FileSystem` port; the archive outcome carries the progress, and a new
  refusal exists under `--strict`.
- `codev-cli`: `codev status` (human line, `tasks` in `StatusV1`),
  `codev archive --strict` (flag, warning, error code `tasks_unchecked`,
  `tasks` in `ArchiveReportV1`). Additive JSON only; an older codev omits
  `tasks` and rejects `--strict` on `archive` as an unknown argument.
- No skill and no template changes. The JSON every skill reads through
  `codev status` grows by one field, so the planning and implementation
  steps are measured under decision 0008.
- Documentation, in `docs/en` and `docs/fr`: the CLI reference (`status`,
  `archive --strict`), the JSON output reference, the `tasks.md` file format,
  and the custom-schemas guide (`apply.tracks`). `ROADMAP.md` loses two
  lines.

## Open Questions

- `/codev-archive` does not relay the new warning: changing the skill is a
  workflow change on a step the evaluation suite does not cover yet
  (decision 0008), left to its own change. Until then, its line "`status[]`
  — empty" is stale once a warning can appear there; the agent sees the
  warning in the JSON but is not told to relay it.
- `codev list` shows no progress; its JSON is a list of names, and a
  parallel key is better designed with the ROADMAP's dashboard.

## Domains

- [ ] security — the tracked path comes from the project's own schema and is only read
- [ ] personal-data
- [ ] user-interface — terminal lines only, no visual interface
- [ ] performance — one more small file read per `status` and `archive`
- [ ] operations
- [x] migration

## Release note

- Added: `codev status` shows how many task boxes of `tasks.md` are checked,
  in total and per `##` group, and its JSON gains a `tasks` field.
- Added: `codev archive` warns when `tasks.md` still has unchecked boxes;
  `codev archive --strict` refuses to archive until they are checked. The
  `/codev-archive` skill does not relay the warning yet.
- Changed: **BREAKING** `codev archive` refuses a change whose `tasks.md` is
  not UTF-8 text (`tasks_unreadable`); re-save the file as UTF-8 to archive
  it. `codev status` only warns about it.
