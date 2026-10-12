# Design: Release notes generated from archived changes — `codev changelog`

## Context

An archived change lives in `_codev/changes/archive/<date>-<name>/` with its
`proposal.md` (`## Why`, `## What Changes`, …) and its spec deltas under
`specs/**/spec.md`, parsed by `codev_core::parser::parse_delta`. Nothing
parses a proposal today. `codev validate` checks active changes only
(`change::list`), through `codev-engine::validate`, which reads files and
lets pure rules in `codev-core` decide.

`codev_core::managed` splices a section between `<!-- codev:begin` and
`<!-- codev:end -->` markers, appending it at the end of a file when absent;
`CLAUDE.md` and the pull request template use it. `codev doctor` gathers
facts in `codev-cli::commands::doctor_at` and lets
`codev_core::doctor::evaluate` judge them; it never spawns a process today,
and its spec says so. The only code that spawns `git` is
`codev-engine::sources`, through the `ProcessRunner` port carried by `Ctx`.

codev's own `CHANGELOG.md` is hand-written, Keep a Changelog style, with a
curated `## [Unreleased]` covering the four changes archived since
`v0.4.0`, none of which has a release note. The release workflow publishes
with `generate_release_notes: true` only.

## Goals / Non-Goals

**Goals:**

- Every decision about content, range, ordering, rendering and file
  splicing is a pure function of `codev-core`, tested without I/O.
- Git is queried through `ProcessRunner`; engine and CLI tests use the
  in-memory `FileSystem`, `FixedEnv` and `MockProcessRunner`.
- `codev changelog --write` is idempotent and never rewrites hand-written
  text, apart from adopting an existing `## [Unreleased]` heading once.

**Non-Goals:**

- Choosing the next version, tagging, publishing, compare links in the
  link-reference footer.
- Reading archived content from git objects: content is read from the
  working tree (an archived change is not edited after archive); git only
  decides which changes are in range.
- Rewriting archived proposals to add release notes.

## Decisions

### Decision: A pure `codev_core::changelog` module decides everything

Following `_codev/decisions/0001-functional-core-imperative-shell.md`,
`codev_core::changelog` holds:

- `release_note::parse(proposal) -> Parsed<ReleaseNote>` (`Absent`,
  `Internal` for `None`, `Items(Vec<NoteItem>)`), tolerant of wrapping,
  HTML comments (the template's) and fenced blocks, with
  `release_note_invalid` findings carrying the line;
  `release_note::what_changes(proposal) -> Vec<String>` for the fallback.
- `default_since(tags, until) -> Option<String>`: `v*` tags parsed as
  versions (numeric `X.Y.Z…`, a pre-release sorting before its release),
  unparseable ones ignored, the highest one strictly lower than `until`
  when `until` is a version tag, else the highest.
- `collect(changes, exclude) -> Collected { entries, notices }` over
  `ChangeInput { name, archive_path, proposal, removes_requirements }`:
  `None` omitted, fallback on What Changes with `release_note_missing`,
  `release_note_inconsistent`, then a stable sort by (category order,
  not breaking, archive order).
- `render(entries, Heading) -> String` for keep-a-changelog and markdown.
- `file::write(existing, unreleased_block, release) -> Result<String, String>`
  for `CHANGELOG.md`, and `file::managed_block(text)` for doctor.

Codes (`release_note_missing`, `release_note_invalid`,
`release_note_inconsistent`) are constants of the module and join the
code-uniqueness test of `validate`.

**Alternatives considered**: parsing in the engine next to the I/O (logic
not testable without a file system, against 0001); reusing the spec parser
(a proposal is not a spec, and its findings would mix with spec codes).

### Decision: The range comes from `git ls-tree` through `ProcessRunner`

`codev_engine::changelog::gather` runs, with the project root as working
directory: `git rev-parse --is-inside-work-tree`; then, for each explicit
ref, `git ls-tree -d --name-only <ref> -- _codev/changes/archive/` (`-d`
keeps folders only, so the archive's `.gitkeep` is not a change; paths are
relative to the working directory, so a project nested in a repository
works); and, when `--since` is absent, `git tag --list v*`, handing the
names to `default_since`. `--until` absent means the working tree
(`FileSystem::list_dir` of the archive). A failing `ls-tree` is
`unknown_ref`. Outside a git repository, every archived change is listed
with the `not_a_git_repository` warning, and explicit refs fail. The
`ProcessRunner` port is the one of
`_codev/decisions/0002-crate-graph-as-dependency-rule.md`; the crate graph is
unchanged (`codev-engine` reads, `codev-core` decides, `codev-cli` renders
and writes).

**Alternatives considered**: archive dates against tag dates (a change
archived on the release day is ambiguous, and the brief rules dates out);
`git log --diff-filter=A` on the archive (more process output to parse for
the same answer); `git show <ref>:<path>` for content (one process per file,
for content that does not change after archive).

### Decision: The managed section of `CHANGELOG.md` sits after the header and adopts a hand-written `[Unreleased]` heading

`managed` gains `render_section_with(begin_line, body)`,
`section_range(text)` and `section_body(text)`; its
`splice` stays as it is for `CLAUDE.md`. The changelog begin line says the
block is generated by `codev changelog --write`. The header is everything
before the first line starting with `## `. On first write, when that line is
`## [Unreleased]`, it is replaced by the managed section (whose own heading
is `## [Unreleased]`), so the hand-written entries below it continue the
same section instead of sitting under a second `[Unreleased]` heading. With
`--release`, the managed section becomes `managed(## [Unreleased])`, a blank
line, then the version block outside the markers — a single managed section
remains, so the next `--write` refreshes it. Link references at the end are
outside the markers and never touched. Writes go through a `Plan` executed
by `apply::execute`.

**Alternatives considered**: appending the section at the end, as
`CLAUDE.md` does (it would land after released versions); owning the whole
file (it would destroy curated history); keeping the dated block inside
the markers (two managed blocks after the next release, which `splice`
refuses as malformed).

### Decision: codev's own repository excludes the hand-described changes

The four changes archived since `v0.4.0` have no release note; the fallback
would turn their What Changes into `Changed` items duplicating the curated
`[Unreleased]` entries. `_codev/config.yaml` lists them under
`changelog.exclude` (project-only, like `mcp:`, not inherited). Running
`codev changelog --write` then adopts the hand-written `## [Unreleased]`
heading: the managed section holds this change's items, the curated
entries follow untouched, and `codev doctor` is clean because the managed
section matches the generated one. Once `v0.5.0` is tagged, the excluded
changes fall out of range and the list can be emptied.

**Alternatives considered**: editing the archived proposals (forbidden:
history is not rewritten); letting the fallback duplicate the curated text
(duplicates, and worse wording); a `--since` stored in the file (a second
source of truth for the range); a doctor exception for the fallback (the
check would no longer mean "matches what `codev changelog` generates").

### Decision: Doctor runs local, read-only git for the `changelog` check

The `changelog` fact is computed by the same `gather` + `collect` +
`render` as `codev changelog --write` with default arguments, then compared
with `managed::section_body` of `CHANGELOG.md` (whitespace-trimmed). It needs
`git tag` and `git ls-tree`: local, read-only, offline. The doctor spec is
modified accordingly; the `codev status` hint, which runs doctor, inherits
those calls. Any failure gathering the facts makes the check a warning
rather than failing doctor.

**Alternatives considered**: comparing without git (the expected content
depends on the latest tag); a separate `codev changelog --check` (two places
to look; doctor is where adoption gaps are reported).

### Decision: Release workflow runs the Linux build's binary

The `release` job already downloads every archive; it checks out with
`fetch-depth: 0` (tags and history for `ls-tree`), extracts `codev` from
the `x86_64-unknown-linux-musl` tarball (static, runs on the Ubuntu runner),
runs `codev changelog --until "$GITHUB_REF_NAME" --format markdown >
release-notes.md` — the default `--since` is the previous version tag — and
passes `body_path: release-notes.md` while keeping
`generate_release_notes: true`, which `softprops/action-gh-release` appends
below the body.

**Alternatives considered**: `cargo run` in the release job (a second full
compile); the latest published codev (would not know a format introduced by
the tag being released); dropping GitHub's notes (they credit contributors
and pull requests, which release notes do not).

## Risks / Trade-offs

- [`--release --write` is run but the tag is not created: the next
  `--write` puts the same items back under `[Unreleased]`] → CONTRIBUTING
  says to tag right after committing the release; doctor would show it.
- [A fresh project now has a doctor warning until it writes a
  `CHANGELOG.md`] → Warning only, never fails `--strict`; the fix is one
  command.
- [`codev status` now spawns `git` through its doctor hint] → Local,
  read-only, a few milliseconds; the hint is skipped when doctor fails.
- [The working tree may lack a change present at `--until` (deleted since)]
  → It is skipped with an `archived_change_unreadable` warning.
- [Adopting the `[Unreleased]` heading touches one hand-written line] →
  Only on the first write, specified, documented; nothing else outside the
  markers changes.
- [The `v0.5.0` release body omits the four excluded changes] → GitHub's
  generated notes follow it; the curated text is in `CHANGELOG.md`, shipped
  in every archive.

## Migration Plan

No migration for users: the `## Release note` section is new in proposals,
missing sections only warn on active changes, and `codev update`
regenerates the skills. On codev's repository: `changelog.exclude` and one
`codev changelog --write`.

## Open Questions

- Should `changelog.exclude` warn on a name that is not an archived change?
  Deferred: a typo today shows up as the change's fallback items and its
  `release_note_missing` warning in `codev changelog`.
