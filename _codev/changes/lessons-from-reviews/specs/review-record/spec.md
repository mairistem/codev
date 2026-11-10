## ADDED Requirements

### Requirement: `codev review --archived` serves the settled findings and the corrections of archived changes

`codev review --archived [--last <n>] [--json]` SHALL read the last `n`
archived changes — `n` a positive integer, 10 by default, the changes of
`_codev/changes/archive/` ordered by folder name, the newest last — and
print, per change, oldest first:

- its folder name, and its counts: every finding of its `review.md`
  (`findings`), those with confidence `high` whose status is not
  `rejected` (`reported`), those served (`served`), and the entries of
  its `corrections.md` (`corrections`);
- each finding whose last status is `fixed` or `accepted-risk`, with its
  id, title, role, step, severity, confidence, location, recommendation,
  status, the note of its last entry when it has one, and its
  `raised_by`; never a finding `open`, `disputed`, `rejected` or
  `deferred`;
- each entry of its corrections file, with its id, title, step, `where`,
  `did` and `asked`;
- for each finding and correction served, the identifier of the lesson
  of `_codev/lessons.md` whose `origin` names it, or none.

The evidence of an archived finding SHALL NOT be checked against the
tree, nor served. A record or a corrections file this codev cannot read —
a newer format, or a block it cannot parse — serves nothing from that
file and is named with its warning under the change's `issues`; the
ledger's own warnings are listed apart. An archive with fewer than `n`
changes serves them all. The command SHALL exit with 0, archived records
never gating anything. `--archived` SHALL NOT be combined with
`--change`, and `--last` SHALL NOT be given without `--archived`: either
is refused with a usage error. With `--json`, it prints an
`ArchivedReviewV1` document holding `last`, `changes` (each with `name`,
`counts`, `findings`, `corrections` and `issues`, the served entries
carrying `considered`, a lesson id or `null`) and `lessons` (the ledger's
`path`, `exists`, and its `issues`).

#### Scenario: Only settled findings are served

- **GIVEN** an archived change whose record holds four findings, whose
  last statuses are `fixed`, `accepted-risk`, `rejected` and `deferred`
- **WHEN** the user runs `codev review --archived --json`
- **THEN** that change's `findings` holds the first two only, and its
  `counts` give `findings: 4` and `served: 2`

#### Scenario: The last ten changes by default

- **GIVEN** a project with twelve archived changes
- **WHEN** the user runs `codev review --archived --json`
- **THEN** `changes` holds the ten newest, oldest first, and `last` is 10

#### Scenario: A chosen window

- **GIVEN** a project with twelve archived changes
- **WHEN** the user runs `codev review --archived --last 30 --json`
- **THEN** `changes` holds the twelve

#### Scenario: Evidence that left the tree

- **GIVEN** an archived finding `fixed` whose evidence no longer appears
  at its location
- **WHEN** the user runs `codev review --archived --json`
- **THEN** the finding is served without evidence and no
  `review_evidence_*` finding is reported

#### Scenario: Corrections of an archived change

- **GIVEN** an archived change without `review.md` whose `corrections.md`
  holds `C-1`
- **WHEN** the user runs `codev review --archived --json`
- **THEN** that change's `corrections` holds `C-1` with `step`, `where`,
  `did` and `asked`, and its `counts.findings` is 0

#### Scenario: An entry a lesson already considered

- **GIVEN** a ledger lesson `L-2` whose `origin` names
  `2026-10-13-review-record/R-security-3`, and that change among the
  last ten
- **WHEN** the user runs `codev review --archived --json`
- **THEN** that finding's `considered` is `L-2`, and the other entries'
  is `null`

#### Scenario: A record from a newer codev in the archive

- **GIVEN** an archived change whose `review.md` starts with
  `<!-- codev-review: 2 -->`
- **WHEN** the user runs `codev review --archived --json`
- **THEN** that change serves no finding, its `issues` holds the
  `review_format_unsupported` warning, and the exit code is 0

#### Scenario: An empty archive

- **GIVEN** a project with no archived change
- **WHEN** the user runs `codev review --archived`
- **THEN** it says there is no archived change, and the exit code is 0

#### Scenario: Two selections at once

- **GIVEN** a project with an active change `add-auth`
- **WHEN** the user runs `codev review --archived --change add-auth`
- **THEN** the command fails with a usage error, and prints no document
