# Changelog Specification

## Purpose

`codev changelog` turns the release notes of archived changes into the
content of a changelog — for `CHANGELOG.md` and for release bodies — so that
release notes come from the planned work rather than from commit messages.
Choosing versions, tagging and publishing stay with release tooling.

## Requirements

### Requirement: A proposal carries a categorized release note

The `## Release note` section of a proposal SHALL hold either the single
line `None`, for a change users will not notice, or one or more bullets of
the form `- <Category>: <text>`, where Category is one of `Added`,
`Changed`, `Deprecated`, `Removed`, `Fixed` or `Security`. An item is
breaking when its text starts with `**BREAKING**`. A bullet MAY wrap over
several lines: the continuation lines belong to the same item, joined with
single spaces. A `None` note MUST produce no changelog item.

#### Scenario: Wrapped and breaking items

- **GIVEN** an archived change whose release note is
  `- Removed: **BREAKING** the legacy\n  flag is gone.` and
  `- Fixed: a crash.`
- **WHEN** the user runs `codev changelog --format json`
- **THEN** the entries are a `Removed` item with the text
  `**BREAKING** the legacy flag is gone.` and `breaking: true`, and a
  `Fixed` item with `breaking: false`

#### Scenario: An internal change is omitted

- **GIVEN** an archived change whose release note is `None`
- **WHEN** the user runs `codev changelog`
- **THEN** no item of that change appears and no warning names it

### Requirement: `codev changelog` selects the changes archived between two git refs

`codev changelog` SHALL list the changes present in `_codev/changes/archive/`
at `--until` — by default, in the working tree — and absent from it at
`--since`. Presence at a ref MUST be read from git, never from dates. The
default `--since` is the highest `v*` tag by version order, excluding
`--until` and any tag of a higher version when `--until` names a version
tag; with no such tag, every archived change is in range. Changes are
listed in archive order. Changes named under `changelog.exclude` in
`_codev/config.yaml` are left out without a warning.

Outside a git repository, every archived change of the working tree SHALL
be listed with the warning `not_a_git_repository`; passing `--since` or
`--until` there fails with `not_a_git_repository`. A ref git does not know
fails with `unknown_ref`. A change present at `--until` but missing from
the working tree is skipped with the warning `archived_change_unreadable`.

#### Scenario: Default range since the latest tag

- **GIVEN** a git repository with tags `v0.9.0` and `v0.10.0`, change `a`
  archived before `v0.9.0`, `b` between the two tags, and `c` archived
  after `v0.10.0`, uncommitted
- **WHEN** the user runs `codev changelog`
- **THEN** only the items of `c` are listed

#### Scenario: Range between two tags

- **GIVEN** the same repository
- **WHEN** the user runs `codev changelog --until v0.10.0`
- **THEN** only the items of `b` are listed

#### Scenario: No tag yet

- **GIVEN** a git repository with no `v*` tag and two archived changes
- **WHEN** the user runs `codev changelog`
- **THEN** the items of both changes are listed

#### Scenario: Excluded change

- **GIVEN** a change in range named under `changelog.exclude`
- **WHEN** the user runs `codev changelog`
- **THEN** none of its items is listed and no warning names it

#### Scenario: Change missing from the working tree

- **GIVEN** a change present at `--until` whose folder was since deleted
- **WHEN** the user runs `codev changelog --until <ref>`
- **THEN** the warning `archived_change_unreadable` names it

#### Scenario: Outside a git repository

- **GIVEN** a codev project that is not a git repository, with two
  archived changes
- **WHEN** the user runs `codev changelog`
- **THEN** the items of both changes are listed
- **AND** the warning `not_a_git_repository` is reported

#### Scenario: Unknown ref

- **GIVEN** a git repository without a `v9.9.9` ref
- **WHEN** the user runs `codev changelog --since v9.9.9`
- **THEN** the command fails with the code `unknown_ref`

### Requirement: Items are grouped by category, breaking first, with a fallback for changes without a note

The items SHALL be grouped by category in the order `Added`, `Changed`,
`Deprecated`, `Removed`, `Fixed`, `Security`; within a category, breaking
items come first, then archive order. A change in range whose proposal has
no `## Release note` section SHALL contribute its `## What Changes` bullets
as `Changed` items — breaking when they start with `**BREAKING**` — and be
named in a `release_note_missing` warning. A change whose spec deltas
remove requirements while its release note has neither a `Removed` item
nor a breaking item SHALL be named in a `release_note_inconsistent`
warning. Warnings never change the exit code.

#### Scenario: Ordering

- **GIVEN** changes `a` then `b` in range, `a` with `- Fixed: x` and
  `- Added: y`, `b` with `- Added: **BREAKING** z`
- **WHEN** the user runs `codev changelog`
- **THEN** the sections are `Added` then `Fixed`, and `Added` lists `z`
  before `y`

#### Scenario: Fallback on What Changes

- **GIVEN** a change in range archived without a `## Release note` section,
  whose What Changes has two bullets
- **WHEN** the user runs `codev changelog`
- **THEN** both bullets are listed under `Changed`
- **AND** a `release_note_missing` warning names the change

#### Scenario: Removed requirements without a matching item

- **GIVEN** a change in range whose delta has a `## REMOVED Requirements`
  section and whose release note is `- Added: a new flag.`
- **WHEN** the user runs `codev changelog`
- **THEN** a `release_note_inconsistent` warning names the change
- **AND** the exit code is 0

### Requirement: `codev changelog` prints Keep a Changelog, markdown or JSON

`--format keep-a-changelog`, the default, SHALL print a `## [Unreleased]`
heading — `## [<version>] - <date>` with `--release <version>`, the date
being `--date` or today — followed by one `### <Category>` heading per
non-empty category and its items as `- ` bullets. `--format markdown`
SHALL print the same without the version heading. `--format json` SHALL
print one JSON document following the contract v1, with every item's
`category`, `text`, `breaking`, `change` and `archivePath`, and the
warnings in `status`. A version that is not `X.Y.Z` (an optional `v`
prefix and pre-release suffix allowed) fails with
`invalid_release_version`, a date that is not `YYYY-MM-DD` with
`invalid_date`.

#### Scenario: Version heading

- **WHEN** the user runs `codev changelog --release 1.2.0 --date 2026-10-20`
- **THEN** the output starts with `## [1.2.0] - 2026-10-20`

#### Scenario: Markdown for a release body

- **WHEN** the user runs `codev changelog --format markdown`
- **THEN** the output has no `## [` heading and starts with a `### `
  category heading, or is empty when no item is in range

#### Scenario: JSON

- **WHEN** the user runs `codev changelog --format json`
- **THEN** stdout is one JSON document whose entries carry `category`,
  `text`, `breaking`, `change` and `archivePath`

#### Scenario: Invalid version

- **WHEN** the user runs `codev changelog --release next`
- **THEN** the command fails with the code `invalid_release_version`

### Requirement: `codev changelog --write` keeps a managed section of `CHANGELOG.md`

`codev changelog --write` SHALL write the generated `[Unreleased]` block
into a managed section of `CHANGELOG.md`, between codev's markers, placed
right after the file header — before the first `## ` heading. When the file
is missing, it MUST be created with a standard Keep a Changelog header. When
the file has no managed section yet and its first `## ` heading is
`## [Unreleased]`, that heading is taken over by the managed section and the
entries below it are kept as they are. Apart from that heading, text
outside the markers MUST NOT change, including the link references at the
end of the file. Running it again with nothing new MUST leave the file
unchanged.

With `--release <version>`, the managed section SHALL be replaced by an
empty managed `## [Unreleased]` followed, outside the markers, by the
`## [<version>] - <date>` block. Markers that are unbalanced or repeated
make the command fail with `changelog_malformed` without writing.
`--write` MUST be combined with the `keep-a-changelog` or `json` format;
with `markdown`, it fails with `invalid_argument`.

#### Scenario: Created when missing

- **GIVEN** a project without `CHANGELOG.md` and one change in range
- **WHEN** the user runs `codev changelog --write`
- **THEN** `CHANGELOG.md` starts with `# Changelog` and holds a managed
  section with `## [Unreleased]` and the change's items

#### Scenario: Hand-written content is kept

- **GIVEN** a `CHANGELOG.md` with a header, a hand-written
  `## [Unreleased]` section and a `## [1.0.0]` section followed by link
  references
- **WHEN** the user runs `codev changelog --write`
- **THEN** the managed section sits where the hand-written
  `## [Unreleased]` heading was, the hand-written entries follow it, and
  the `## [1.0.0]` section and the link references are unchanged

#### Scenario: Idempotent

- **GIVEN** a `CHANGELOG.md` just written by `codev changelog --write`
- **WHEN** the user runs `codev changelog --write` again
- **THEN** the file is unchanged

#### Scenario: Release

- **GIVEN** a `CHANGELOG.md` whose managed section lists one item
- **WHEN** the user runs
  `codev changelog --release 1.1.0 --date 2026-10-20 --write`
- **THEN** the managed section holds only `## [Unreleased]`
- **AND** it is followed by `## [1.1.0] - 2026-10-20` and the item, outside
  the markers

#### Scenario: Malformed markers

- **GIVEN** a `CHANGELOG.md` with two begin markers
- **WHEN** the user runs `codev changelog --write`
- **THEN** the command fails with `changelog_malformed` and the file is
  unchanged

### Requirement: A release note in any catalog language feeds the same categories

`codev changelog` SHALL read a release note written under any catalog
language's heading (`## Note de version`), with categories and the single
line `None` in any catalog language, and SHALL report each item under the
category's English name — `Added`, `Changed`, `Deprecated`, `Removed`,
`Fixed` or `Security` — in every output format, so that one changelog
gathers changes written in different languages. A change archived without
a release note falls back to the bullets of its `## What Changes` section,
in any catalog language (`## Changements`).

#### Scenario: French items grouped with English ones

- **GIVEN** two archived changes, one whose note is `- Added: a flag.` and
  one whose `## Note de version` is `- Ajouté : un export.`
- **WHEN** the user runs `codev changelog --format json`
- **THEN** both items have the category `Added`

#### Scenario: A French internal change is omitted

- **GIVEN** an archived change whose `## Note de version` is `Aucune`
- **WHEN** the user runs `codev changelog`
- **THEN** no item of that change appears and no warning names it

#### Scenario: A French change without a release note

- **GIVEN** an archived change whose proposal has no release note and
  whose `## Changements` holds `- Un export mensuel.`
- **WHEN** the user runs `codev changelog --format json`
- **THEN** the item `Un export mensuel.` is listed under `Changed`
