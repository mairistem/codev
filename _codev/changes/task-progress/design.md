# Design: Task progress in status, and unchecked tasks at archive

## Context

The schema already names the file to track: `apply.tracks` in
`crates/codev-core/src/schema.rs` (`Apply::tracks`), checked non-empty at
load and read by nothing else. `codev status` is computed in
`codev-engine::change::status` from one walk of the change folder, then
`codev_core::status::compute`; its JSON is `StatusV1` in
`crates/codev-cli/src/contract.rs`, rendered by `render::status`.
`codev archive` is `codev-engine::archive::plan_archive` (validation
pre-flight through `sync::plan_sync`, then the review gate) followed by
`execute_archive`; its JSON is `ArchiveReportV1`, whose `status` is always
empty on success today, and its failure shape is `archive_shape()` in
`main.rs`. `codev archive` has no flag besides `--change` and `--json`.

The archived changes show what the counter meets in practice: six of them
leave 20 boxes open, mostly post-merge steps (version bump, tag, changelog
date) from before `codev changelog` existed;
`2026-09-24-public-launch-polish/tasks.md:241-244` holds four boxes inside
a fence indented by six spaces, inside a list item. The `/codev-apply`
skill already treats every `- [ ]` as a task, at any depth, and points out
`- [-]` and `- [X]` as formats to fix.

## Goals / Non-Goals

**Goals**

- One pure counter, used by `status` and `archive`, reading only Markdown
  structure.
- Additive JSON: no existing key changes meaning or disappears.

**Non-Goals**

- Gating a phase on its group, a verification line per group, a review per
  phase: the archived change `2026-10-14-fewer-review-turns` settled on one
  review per step, and a per-group gate would bring the turns back.
- Localized group keywords: a heading is copied, never interpreted.
- Progress in `codev list` (see Open Questions).
- Any skill or template change.

## Decisions

### Decision: A pure counter in `codev-core::tasks`, fed by the engine

`codev_core::tasks::parse(text: &str) -> TaskProgress` is a pure function
of the file's text (decision 0001: the core never touches the disk; 0002:
the engine reads through the `FileSystem` port). `TaskProgress` holds the
groups in file order (`id: Option<String>`, `title: Option<String>`,
`checked`, `total`) and the unchecked boxes (1-based line number and the
trimmed line), from which the totals are derived. A line scanner with two
states (inside a fence, with its character and length; inside an HTML
comment) is enough; no Markdown crate is added.

The engine gets `change::task_progress(fs, layout, ctx) ->
Result<Option<TaskProgress>>`: it joins `apply.tracks` to the change folder,
returns `None` when the file is absent (the walk `status` already does
tells it), and a distinct `TasksUnreadable` outcome when it exists and
cannot be read as UTF-8 text. `codev status` turns that into `tasks: null`
plus a `tasks_unreadable` warning and exits 0: every skill calls `status`
first, and a hand-edited non-UTF-8 `tasks.md` that today breaks nothing
must not lock a change out of its workflows. `codev archive` refuses it
with `tasks_unreadable`, strict or not: archiving would claim a check it
could not make.
`ChangeStatus` stays the core's artifact view; the engine's status call
returns the progress beside it, so `codev_core::status::compute` keeps its
signature and its tests.

**Alternatives considered**: hard-coding `tasks.md` — a custom schema
tracking `plan.md` would read the wrong file, and `Apply::tracks` would stay
dead; `pulldown-cmark` — a dependency for two states, whose list-item
semantics would also decide that a box inside an indented paragraph is not a
list item, which the person reading the file would not expect.

### Decision: A box is a box at any depth; only `space`, `x`, `X` mark one

An indented sub-box counts like any other. It is what `/codev-apply`
already works through, and an unchecked sub-box is work left. The one
archived example of indented boxes sits inside a fence, which the fence
rule excludes. `[X]` is read as checked so that a hand-edited file is
counted as its author meant, even though `/codev-apply` keeps asking for
`[x]`.

**Alternatives considered**: only top-level boxes — a nested checklist
would read as complete while a sub-step is open; parent derived from its
children — a semantic codev has nowhere else and no template uses.

### Decision: No exclusion marker for post-merge steps

A step that happens after the merge (bump, tag, push) is written as a plain
bullet, not a box: the `tasks.md` template already says that a task not
written `- [ ]` is not tracked, and `codev changelog` replaced the release
tasks that left most of the archived boxes open. A `- [-]` line is simply
not a box, like any other marker.

**Alternatives considered**: `- [-]` as "excluded" — a third state in the
JSON, and a contradiction with `/codev-apply`, which asks to fix that
marker; changing the skill would be a workflow change to measure under
decision 0008 for a case the default (warning, not error) already absorbs.

### Decision: Group `id` from the heading's leading number, `null` otherwise

`## 2. Engine` gives `id: "2"`, `title: "Engine"`: the number is what task
`2.3` and the template already use to name the group, and it survives a
reworded title. A heading without that number gives `id: null` and the
whole text as `title`; boxes before the first `## ` give `id: null,
title: null`. Groups are an ordered array, so position identifies a group
when `id` is `null`. Headings with no box are not listed, to keep the JSON
the skills read small. The id is a string, not a number, so that a later
format (`2a`) stays additive.

**Alternatives considered**: a slug of the title — it changes with any
rewording and would need language-aware slugging; the ordinal position —
it changes when a group is inserted and disagrees with the task numbers as
soon as numbering skips.

### Decision: `tasks` is always present in `StatusV1` and `ArchiveReportV1`

`tasks: null` until the tracked file exists, an object afterwards
(`{checked, total, groups}`), `{0, 0, []}` for a file without boxes. A key
that is always present lets a consumer tell "no file" from "an older codev"
(no key at all). The JSON stays in `contract::v1` with its own
`TasksV1`/`TaskGroupV1` types, never serializing the core type directly.
The failure shapes `archive_shape()` and `status_shape()` in `main.rs`
gain `tasks: null`, so the key is present on failure too.

**Alternatives considered**: omitting the key when `null`, as `update`
does — `update`'s absence means "unknown", which is not the case here; a
per-box list in the JSON — tokens in every skill's `status` read for no
consumer.

### Decision: The archive check is a warning, an error only under `codev archive --strict`

`plan_archive` takes a `strict` flag. After the validation pre-flight and
the review gate, it computes the progress: under `--strict` with unchecked
boxes it returns `EngineError::TasksUnchecked { change, file, count }`
(code `tasks_unchecked`) before building the plan, so nothing is written;
otherwise the outcome carries the progress and the CLI adds the
`tasks_unchecked` warning to `status` and to the human report, after the
`Moved to:` line, listing every unchecked line. `--strict` follows the
meaning it has on `validate` and `doctor`: warnings become failures.

The check is not a validation rule: `codev validate --strict`, which runs
before every commit, would fail on every change being implemented, and
re-reading archives would flag the 20 boxes already archived. Archived
changes are never read.

**Alternatives considered**: refusing by default — it would block the
archives of the six past patterns, and a team that writes post-merge boxes,
with no override short of editing the file; a validation rule limited to
the archive pre-flight — the pre-flight's rule set is `validate`'s, and a
rule whose severity depends on the caller would be the first of its kind.

### Decision: No skill changes; the status JSON is measured

The `/codev-archive` skill runs `codev archive --json` without `--strict`:
the warning reaches it in `status`, which it does not relay today, and a
`tasks_unchecked` refusal would fall under its "any other code: relay the
message" branch. Changing the skill to relay the warning is a workflow
change on a step the evaluation suite does not cover, which decision 0008
asks to cover first, in its own change. The `tasks` key does enter what
`/codev-propose` and `/codev-apply` read through `codev status --json`, so
decision 0008 applies: `plan.write` and `code.implement` are run and
compared, quality only, with the `status` source's tokens reported.

**Alternatives considered**: relaying the warning in the archive skill now
— see above; skipping the measure because the field is small — decision
0008 names `contract.rs` explicitly.

## Risks / Trade-offs

- [`/codev-archive` users do not see the warning] → the human `codev archive`
  shows it, `codev status` shows the count before archiving, and the follow-up
  is named in the proposal's open questions and in a narrower ROADMAP item.
- [A schema's `tracks` with `..` or an absolute path reads outside the
  change] → the engine treats such a path as absent (`tasks: null`), so the
  read stays inside the change folder; the schema is the project's own file
  or a pinned inherited source (decision 0005).
- [A fence or comment never closed hides the boxes after it] → this is how
  Markdown renders it too, and the totals in `codev status` make the drop
  visible; covered by a unit test.
- [Older codev on the same project] → its JSON has no `tasks` key, and `codev archive --strict` fails there as
  an unknown argument (exit 2) before doing anything; the docs say the flag
  needs this release.
- [Group ids collide when two headings share a number] → ids are not keys:
  the array order stays the identity, documented in the JSON reference.

## Migration Plan

Nothing to migrate: no file format changes, existing archives are never
read, and the new JSON keys are additive. Rollback is reverting the
release; a consumer that reads `tasks` must accept its absence from an older
codev.

## Open Questions

- Should `/codev-archive` relay the `tasks_unchecked` warning? That needs
  the archive step covered by the evaluation suite first (decision 0008).
- Should `codev list` show progress per change? Its JSON is an array of
  names; progress would need a parallel key, to be designed with the
  dashboard item of the ROADMAP.
