# Proposal: Lessons from review findings and corrections

## Why

Each archived change keeps what its reviewers found and what its author
corrected (`review.md`, `corrections.md`), but nothing reads them again:
the same defects come back change after change (compatibility with older
codevs, symbolic links leaving the project, workflow input reaching the
shell, long functions), and the team repeats itself. The first half of
the lessons loop (`capture-corrections`) now records the corrections;
this change closes the loop by turning what recurs into durable project
knowledge, with the author's agreement.

## What Changes

- `codev review --archived [--last <n>] [--json]` serves the history of
  the last `n` archived changes (default 10): their findings whose last
  status is `fixed` or `accepted-risk`, and their corrections, without
  checking evidence, each marked when a lesson already considered it, and
  per change the counts the downstream metric needs.
- A new, optional, versioned file `_codev/lessons.md`, the lessons
  ledger: each lesson considered (written, handed off, covered, declined
  or set aside), the archived findings and corrections it comes from,
  and each pass of the mode. `codev validate` checks it; an older codev
  ignores it.
- `/codev-configure lessons [<n>]`, an on-demand mode of the existing
  skill: it reads the served history, the decisions in force or
  proposed, the configuration and the active changes; groups what
  recurs (a finding in at least two changes, or a single correction);
  and proposes each lesson for one destination, in this order: an
  automatable check, a decision, a rule (`specs`, `design`, `tasks`) or a
  `context:` line, or nothing when it is general expertise of codev's
  built-in roles. It shows a diff, lets the author change a destination,
  and writes nothing without explicit agreement. Past ten rules on one
  artifact it proposes a merge instead of an addition.
- `codev archive --json` tells whether lessons may be waiting, and
  `/codev-archive` adds one invitation line to its report when they are.
  It gets no new step and no new tool. The skill is measured against the
  `change.archive` baseline (decision 0008).
- A new evaluation step, `configure.lessons`, measures the mode per call
  for quality and tokens (decision 0008), with cases drawn from this
  repository's archive and a baseline, built on the evaluation
  infrastructure of `evaluate-archive-sync-configure` (its shared
  scoring of check-scored steps and its `overlay/`). The default mode of
  `/codev-configure`, whose skill text grows, must not regress against
  the `configure.enrich` baseline.
- This change depends on `evaluate-archive-sync-configure`, which must
  be archived first: it covers `/codev-archive` (`change.archive`) and
  the default mode of `/codev-configure` (`configure.enrich`) with a
  recorded baseline before this change touches them.
- A one-time catch-up on this repository, as a task of this change: the
  mode run once over every archived change, each lesson recorded in the
  ledger, and, with the author's agreement, the project instructions of
  this project's Claude Code memory proposed for codev.
- A second change, `lessons-effect`, is described in the design: once ten
  changes are archived after the catch-up, it reports the downstream
  metric (findings and corrections per change) in the reliability
  chapter.

## Capabilities

### New Capabilities

- lessons
- lessons-evaluation

### Modified Capabilities

- configure
- review-record
- validation
- skills

### Removed Capabilities

## Impact

- `codev-core`: a parser and checks for `_codev/lessons.md`; the
  selection of an archived change's served findings; the waiting signal.
- `codev-engine`: reading the archived records and corrections, the
  ledger's validation, the signal computed after an archive.
- `codev-cli`: `--archived` and `--last` on `codev review`, its human
  output, the `ArchivedReviewV1` document, and an additive `lessons`
  object in `ArchiveReportV1` (`contract.rs`).
- `assets/workflows/configure.md` (the lessons mode) and
  `assets/workflows/archive.md` (one conditional line).
- `evals/`: the `configure.lessons` step, its cases and its baseline, on
  the shared scoring and the `overlay/` that `evaluate-archive-sync-configure`
  adds; candidate runs of `change.archive` and `configure.enrich`.
- Documentation in `docs/en` and `docs/fr`: the skills chapter, the CLI
  and JSON references, the configuration reference, the validation codes,
  and the review-record guide.
- This repository: `_codev/lessons.md`, and whatever the catch-up's
  confirmed lessons write (`_codev/config.yaml`, proposed decisions).

## Domains

- [ ] security — the served history is the repository's own planning files, already read by the skills; no input reaches the shell, nothing executable is added, and the folders the ledger names are single path segments looked up only under `_codev/changes/archive/`, refused otherwise
- [ ] personal-data — the ledger quotes findings and corrections, which already name no person; the one-time reading of this project's Claude Code memory happens only with the author's agreement, proposes only project instructions (never a personal preference, a person or a secret), and each write is confirmed
- [ ] user-interface — no interface beyond the CLI's text output and one line in a skill's report
- [x] performance
- [ ] operations — no log, timeout or deployment changes; the evaluation runs only when its step is selected
- [x] migration

## Release note

- Added: `/codev-configure lessons` reads the findings and corrections of your last archived changes, groups what recurs, and proposes to turn each lesson into a check, a decision, a rule or a context line, writing only what you confirm and remembering what you declined in `_codev/lessons.md`.
- Added: `codev review --archived` lists the settled findings and the corrections of the last archived changes, and `/codev-archive` says when lessons may be waiting.
