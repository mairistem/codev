# Proposal: Validate detects what merging parallel changes breaks

## Why

A team, or one developer in several clones or worktrees, carries several
changes on different branches, and codev only ever sees the current tree:
when the branches meet, git merges `_codev/` as text, not as specs. Today
`codev validate` passes on a merged tree where a main spec defines a
requirement twice, where two local decisions share the id `0013`, where
`seal.yaml` seals one id twice, or where a conflict marker was left in a
spec; and an active change whose `MODIFIED` target was removed or renamed
by another branch, or whose `ADDED` requirement another branch already
added with other text, is only refused — or silently skipped — at archive
time. This change makes `codev validate`, which every project already runs
in CI on the pull request's merge result, the safety net for parallel work;
it is the first of a series (see design.md), and needs nothing but the tree
it reads.

## What Changes

- **BREAKING** `codev validate` reports a main spec that defines the same
  requirement twice as the error `duplicate_requirement`, naming both
  lines. A project whose main specs already hold such a duplicate now fails
  `codev validate` until it is resolved.
- **BREAKING** `codev validate` reports two local decisions sharing an id as
  the error `decision_id_duplicate`, naming both files and the next free id,
  and two `seal.yaml` entries for one id as the error
  `decision_seal_duplicate`.
- **BREAKING** `codev validate` reports a git conflict marker left in a planning file it
  reads — main specs, change artifacts, decisions — as the error
  `merge_conflict_marker`, at its line; a marker in `_codev/config.yaml`,
  `seal.yaml` or a `change.yaml` that no longer parses makes every command
  reading it fail with that code instead of `invalid`, `unreadable` or
  `seal_invalid`.
- **BREAKING** The check of a delta against its main spec, which today covers only
  `RENAMED`, also reports a `MODIFIED` whose requirement the main spec no
  longer holds (`modified_target_missing`, the code archive already uses),
  and an `ADDED` whose name the main spec already holds with a different
  text (`added_target_exists`), instead of letting archive refuse the first
  and silently skip the second; an archive that used to drop such an
  addition is now refused. The change's validation also reports a
  duplicate requirement or a conflict marker in a main spec its deltas
  target, so the pre-flight of sync and archive refuses it.
- A `REMOVED` whose requirement the main spec no longer holds is skipped by
  sync and archive, as an `ADDED` already present is, instead of failing
  with `modified_target_missing`.
- `codev decision seal <id> --force` leaves a single seal entry for an id
  sealed several times.
- A new guide, "Parallel changes", in English and French: the rule to
  archive inside the pull request after updating the branch from the
  default branch, CI validating the merge result, and how to resolve each
  collision, including the generated files (`seal.yaml`, the `CHANGELOG.md`
  managed section, the skills).

## Capabilities

### New Capabilities

### Modified Capabilities

- spec-parsing
- validation
- spec-merge
- decisions
- docs

### Removed Capabilities

## Impact

- `codev-core`: the merge (the `REMOVED` skip), the seal plan (`--force`
  collapsing duplicates), the main-spec parser (duplicate detection, today limited to
  delta sections), the cross-file check of a delta against its main spec,
  a conflict-marker scan, the decision index (local duplicate ids) and the
  seal file reading.
- `codev-engine`: `validate` runs the new checks on the files it already
  reads; no new port, no git call.
- `codev-cli`: no new command or flag; the JSON contract v1 gains finding
  codes only, an addition its consumers already tolerate.
- Documentation: the new guide `guides/parallel-changes.md` in `docs/en` and
  `docs/fr`, the finding-code table of `reference/file-formats.md`, and the
  workflow chapter's archive step, in both languages.

## Domains

- [ ] security — reads only the files `validate` already reads; no new input or process
- [ ] personal-data
- [ ] user-interface — no new command, flag or output layout; only new finding codes
- [ ] performance — one more pass over files already read, linear in their size
- [ ] operations
- [x] migration

## Release note

- Fixed: **BREAKING** `codev validate` now reports a requirement defined twice in a main spec, two local decisions sharing an id, and a decision sealed twice in `seal.yaml` as errors — the traces of two branches merged as text. A project whose specs already hold such a duplicate fails validation until it is resolved.
- Changed: **BREAKING** `codev validate` reports a git conflict marker left in a planning file, a `MODIFIED` requirement the main spec no longer holds, and an `ADDED` requirement the main spec already holds with another text, so a collision between parallel changes shows up in CI rather than at archive time; `codev archive` now refuses such an addition instead of dropping it silently. Every command that reads a `_codev/config.yaml`, `seal.yaml` or `change.yaml` holding a conflict marker now fails with `merge_conflict_marker` instead of `invalid`, `unreadable` or `seal_invalid`.
- Fixed: archiving a change that removes a requirement already gone — removed by another branch, or by the change's own earlier sync — now succeeds instead of failing with `modified_target_missing`; `codev decision seal <id> --force` repairs a seal file that seals one id twice.
- Added: a "Parallel changes" guide explains when to archive and how to resolve each collision between branches.
