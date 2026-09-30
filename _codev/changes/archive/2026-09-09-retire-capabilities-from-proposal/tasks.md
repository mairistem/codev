# Tasks

## 1. Core — `Plan.deletions`

- [x] 1.1 Add `deletions: Vec<PathBuf>` to `codev_core::plan::Plan`,
      initialized empty via the existing `#[derive(Default)]`.
- [x] 1.2 Add `Plan::delete(&mut self, path: impl Into<PathBuf>)` —
      deduplication on exact path, like `dir()`.
- [x] 1.3 Extend `Plan::merge` to absorb `other.deletions`.
- [x] 1.4 `Plan::is_empty` now includes `deletions.is_empty()`.
- [x] 1.5 Tests: `delete` deduplicates; `merge` absorbs; `is_empty` covers.

## 2. Core — `merge_into_existing` extended

- [x] 2.1 New signature:
      ```rust
      pub fn merge_into_existing(
          spec_source: &str,
          spec: &Spec,
          delta: &Delta,
          retire_capabilities: bool,
      ) -> Result<MergePlan, MergeError>
      ```
- [x] 2.2 Add `should_delete_spec: bool` to `MergePlan` — initially
      `false`. It is `true` when `retire_capabilities == true` **and**,
      after applying the REMOVED entries, the spec would be empty
      (`removed_count >= spec.requirements.len()`).
- [x] 2.3 The historical refusal `WouldLeaveSpecWithoutRequirement`
      stays active when `retire_capabilities == false`. Error message
      updated: "... use `retire_capabilities: true` in `change.yaml`
      to remove the capability" — drops the "coming soon".
- [x] 2.4 Tests: total emptying without flag → error; total emptying
      with flag → plan with `should_delete_spec: true`, edits applied;
      partial emptying with flag → normal plan,
      `should_delete_spec: false`.
- [x] 2.5 Adapt the existing calls in `codev-engine::sync.rs` to pass
      the new parameter (see task 4.1).

## 3. Shell — `FileSystem::remove_file`

- [x] 3.1 New method on the `FileSystem` trait:
      ```rust
      fn remove_file(&self, path: &Path) -> io::Result<()>;
      ```
      Documented: "deletes a single file; a missing file returns
      `NotFound` — the executor decides whether to ignore or surface
      it".
- [x] 3.2 `RealFileSystem::remove_file` → `std::fs::remove_file`.
- [x] 3.3 `MemoryFileSystem::remove_file` → removes the entry from its
      internal structure, returns `NotFound` if absent.
- [x] 3.4 Tests: write then remove, remove on missing, remove then
      exists (result `false`).

## 4. Shell — `apply::execute` handles deletions

- [x] 4.1 In `apply::execute`, after the writes loop, loop over the
      deletions and call `fs.remove_file`. Order: `dirs` → `writes`
      → `deletions` → `moves`.
- [x] 4.2 `AppliedOutcome` gains a `deleted: Vec<PathBuf>` field
      carried by the execution — each successful deletion is
      recorded.
- [x] 4.3 A file missing during a deletion is **not** an error (the
      change may already have been applied). Silent; does not mark
      the deletion as done.
- [x] 4.4 Tests: deletion of an existing file → deleted and listed;
      deletion of a missing one → no error, not listed.

## 5. Shell — `sync` propagates the deletion

- [x] 5.1 `sync::plan_sync` reads `metadata.retire_capabilities` from
      the change context and passes it to `merge_into_existing`.
- [x] 5.2 If `merge_plan.should_delete_spec`, add `main_spec_path`
      to `plan.deletions` **and** to a new `deleted:
      Vec<PathBuf>` field of the `SyncPlan`. Do not write into
      `updates` — the capability is removed, not updated.
- [x] 5.3 `SyncOutcome` gains `deleted: Vec<PathBuf>`. Same for
      `ArchiveOutcome` (which gets it via sync).
- [x] 5.4 Integration tests: sync on a change that removes a
      capability + flag → file gone from disk, `outcome.deleted` non
      empty, `outcome.updated/unchanged` do not contain it.

## 6. JSON contract

- [x] 6.1 `SyncReportV1` gains `deleted: Vec<String>` (camelCase),
      always present, empty in the usual case.
- [x] 6.2 `ArchiveReportV1` gains `deleted: Vec<String>` (same
      rule).
- [x] 6.3 The sync and archive failure shape in `main.rs` gains
      `"deleted": []` — consistency with the shape's other fields.
- [x] 6.4 Test: `codev sync <c> --json` on a change that removes a
      capability does expose `deleted[0]`.

## 7. Proposal template

- [x] 7.1 `assets/schemas/spec-driven/templates/proposal.md` gains a
      new `### Removed Capabilities` subsection under
      `## Capabilities`. Comment: "One line per removed capability,
      exact path. Requires `retire_capabilities: true` in
      `change.yaml`."
- [x] 7.2 `codev instructions proposal` returns the updated template
      (via `include_str!`).

## 8. Docs + dogfooding

- [x] 8.1 `ChangeMetadata::retire_capabilities` — update the comment
      to drop "coming soon" and add an example: "See F5 for the
      behavior."
- [x] 8.2 `cargo test --workspace` stays green, +15 tests minimum.
- [x] 8.3 `cargo clippy --workspace --all-targets` free of warnings.
- [x] 8.4 `codev validate --strict` on this repository stays green.
- [x] 8.5 Manual test: create a test change that removes a dummy
      capability, apply it via `sync`, check that the file is
      deleted, restore.
