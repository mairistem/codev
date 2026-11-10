# Review record: task-progress

<!-- codev-review: 1 -->

## plan.challenge

Mode: isolated.

### skeptic

#### R-skeptic-1 The archive warning never reaches users who archive through `/codev-archive`

```yaml
severity: medium
confidence: high
location: "design.md#Decision: No skill changes; the status JSON is measured"
evidence: |
  The `/codev-archive` skill runs `codev archive --json` without `--strict`:
  the warning reaches it in `status`, which it does not relay today, and a
recommendation: >-
  The Why says a change can be archived with open boxes without anyone being
  told, and the plan removes the ROADMAP item "A check that archived changes
  have no unchecked task", yet `/codev-archive` (assets/workflows/archive.md)
  ignores warnings in `status[]` and says it is "empty" on success. Either
  scope the claim (Why and release note cover the CLI and `--strict` only,
  keep a ROADMAP item until the skill relays the warning, name the stale
  "`status[]` — empty" line in Open Questions), or make covering the archive
  step in the evaluation suite a prerequisite and have the skill relay
  `tasks_unchecked`.
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    ref: "proposal.md (Why, What Changes, Open Questions, Release note) and tasks.md 4.2 scope the claim to the CLI and `--strict`, name the stale skill line, and replace the two ROADMAP items with a narrower one: the `/codev-archive` skill relays the warning"
  - status: fixed
    by: skeptic
```

### migration

#### R-migration-1 A tracked file existing changes can already hold would make `codev status` fail

```yaml
severity: medium
confidence: low
location: "design.md#Decision: A pure counter in `codev-core::tasks`, fed by the engine"
evidence: |
  returns `None` when the file is absent (the walk `status` already does
recommendation: >-
  Today nothing reads the tracked file's contents. An active change with a
  hand-edited non-UTF-8 `tasks.md`, or a custom schema whose `tracks` names
  a folder, would make `codev status --json` fail with `unreadable` after the
  upgrade, and every skill calls it first. Keep the hard failure for
  `codev archive`; in `codev status`, report `tasks: null` with a warning and
  exit 0; update the spec scenario and test 2.1. Otherwise add a release-note
  line.
history:
  - status: open
    by: migration
  - status: fixed
    by: architect
    ref: "design.md (the counter decision), specs/task-progress/spec.md (requirement and scenario `An unreadable tracked file warns in status and blocks the archive`), specs/changes/spec.md (status JSON and human line), tasks.md 2.1, proposal.md (What Changes, Release note): status warns `tasks_unreadable` and exits 0, archive refuses with `tasks_unreadable`"
  - status: fixed
    by: migration
```

## code.review

Mode: isolated.
Summarized: crates/codev-cli/src/commands.rs — over the material limit (261651 bytes).
Summarized: crates/codev-cli/src/contract.rs — over the material limit (103452 bytes).
Summarized: crates/codev-cli/src/main.rs — over the material limit (67462 bytes).
Summarized: crates/codev-cli/src/render.rs — over the material limit (79394 bytes).
Summarized: crates/codev-engine/src/ports.rs — over the material limit (39104 bytes).
Summarized: docs/en/src/reference/cli.md — over the material limit (48529 bytes).
Summarized: docs/en/src/reference/json-output.md — over the material limit (36534 bytes).
Summarized: docs/fr/src/reference/cli.md — over the material limit (52203 bytes).
Summarized: docs/fr/src/reference/json-output.md — over the material limit (39033 bytes).

### migration

#### R-migration-2 The new archive refusal of a non-UTF-8 tracked file is not marked BREAKING and does not say what to do

```yaml
severity: low
confidence: high
location: "proposal.md#Release note"
evidence: |
  `codev status` only warns about it.
recommendation: >-
  `codev archive` never read the tracked file before this change, so an
  active change whose `tasks.md` is not UTF-8 could be archived until now and
  is refused after the upgrade, strict or not. Mark the release-note item
  **BREAKING**, say how to unblock it (re-save the file as UTF-8), and add a
  matching **BREAKING** bullet to What Changes. No shim is needed.
history:
  - status: open
    by: migration
  - status: fixed
    by: developer
    ref: "proposal.md: a **BREAKING** bullet in What Changes, and the release-note item marked **BREAKING** with the remedy (re-save as UTF-8)"
  - status: fixed
    by: migration
```

### newcomer

#### R-newcomer-1 The path guard in `task_progress` has unexplained, platform-specific conditions

```yaml
severity: low
confidence: high
location: "crates/codev-engine/src/change.rs:125"
evidence: |
  fn leaves_the_change_folder(tracks: &str) -> bool {
recommendation: >-
  Move the check into a small named helper with one short comment per
  condition: why `starts_with(['/', '\\'])` is needed beside `is_absolute()`
  (a path rooted for another platform), and that `get(1..2) == Some(":")`
  catches a Windows drive letter. A newcomer otherwise reads the first two
  conditions as duplicates they could delete.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-engine/src/change.rs: `leaves_the_change_folder`, one comment per condition"
  - status: fixed
    by: newcomer
```

#### R-newcomer-2 `parse` keeps the current heading as an anonymous tuple read with `.0` and `.1`

```yaml
severity: low
confidence: high
location: "crates/codev-core/src/tasks.rs:74"
evidence: |
  let mut heading = TaskGroup::anonymous();
recommendation: >-
  Make `group_heading` return a named value — a `TaskGroup` with zero counts —
  so that `parse` pushes `heading.clone()` instead of building the group from
  `heading.0` and `heading.1`, two `Option<String>` easy to swap.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-core/src/tasks.rs: `group_heading` returns `TaskGroup::empty(..)`, the state starts at `TaskGroup::anonymous()`"
  - status: fixed
    by: newcomer
```
