# Proposal: A review record with evidence and severity that gates archiving

## Why

The roles challenge the plan and review the code, but what they find lives
only in the conversation: a finding cites no evidence anyone can check,
"high" means what the agent felt that day, nobody records whether the
author fixed or disputed it, and nothing stops a change from being
archived with a known high-severity defect still open. A real team keeps
a record of its review; codev should too, and check what can be checked.

## What Changes

- A shared **severity** scale (`high`, `medium`, `low`, each defined in one
  sentence) and a **confidence** (`high`, `low`). A low-confidence finding
  is kept in the record but not presented to the human.
- A **review record** per change, `_codev/changes/<name>/review.md`:
  markdown a human reads, with a strict heading layout and one YAML block
  per finding. One section per step (`plan.challenge`, `code.review`) and
  per role: its findings, "No finding.", or "Disabled by configuration.".
- A **finding** carries a stable id (`R-<role>-<n>`), a severity, a
  confidence, a location (`path:line`, `path:start-end`, or
  `<artifact>#<heading>` for the plan), a verbatim evidence quote, a
  recommendation and a history of statuses — `open`, `fixed` (with a
  reference), `disputed` (with an argument), `accepted-risk` (with a
  justification, decided by a human), `deferred` (with where it goes),
  `rejected` (the reviewer agrees after the author's answer) — each entry
  saying who made it.
- A new command, `codev review [--change <name>] [--json]`: the record's
  findings with their status and who must act next, its defects, and
  whether it lets the change be archived.
- `codev validate` checks an active change's record: its format, the
  transitions of each history, that each cited file, line or section
  exists and that the evidence is found at (or near) the cited lines, and
  that every role active at a recorded step has its section. New codes,
  errors and warnings.
- **BREAKING** `codev archive` refuses, with the new stable code
  `review_blocked`, a change whose roles were active at `plan.challenge`
  or `code.review` when the record or that step's section is missing, or
  when a `high` finding is still `open` or `disputed`. `codev sync` is
  unchanged.
- `/codev-propose` (challenge) and `/codev-apply` (review) write their
  findings into the record, then run the author's answer loop: the author
  fixes or disputes each finding, the reviewer re-checks once, and what is
  still disputed — or an open high finding — is escalated to the human,
  whose decision is recorded. Their summaries present only the
  high-confidence findings, with the existing caps and the high-severity
  exception, and list what the human must decide. `/codev-archive`
  explains a `review_blocked` refusal.

## Capabilities

### New Capabilities

- `review-record` — the record's format, the severity and confidence
  scales, `codev review`, and the archive gate.

### Modified Capabilities

- `validation` — an active change's review record is checked.
- `skills` — the challenge and the review write the record and run the
  answer loop; `/codev-archive` handles `review_blocked`.
- `docs` — the Roles chapter carries the severity scale, and a "Review
  record" guide exists in both languages.

### Removed Capabilities

None.

## Impact

- `codev-core`: a new `review` module (the scales, the record parser, the
  history rules, the evidence matcher, the gate) and its codes in the
  uniqueness test.
- `codev-engine`: a new `review` module (reads the record and the cited
  files, resolves the active roles), `validate` (the record's findings),
  `archive` (the gate), `error` (`review_blocked`).
- `codev-cli`: the `codev review` command, `ReviewV1` in the JSON contract
  v1 (additive), its human rendering, the archive refusal.
- `codev-agents` and `assets/workflows`: a shared review procedure
  substituted into `propose.md` and `apply.md`; `archive.md`; the
  regenerated skills under `.claude/skills/`.
- Documentation, `docs/en` and `docs/fr`: `roles.md`, a new
  `guides/review-record.md`, `reference/file-formats.md`,
  `reference/cli.md`, `reference/json-output.md`, `skills.md`,
  `SUMMARY.md`.
- Active changes started with an earlier codev have no record: archiving
  them is refused until the record is written or their review roles are
  turned off for the change.

## Domains

- [x] security — the record names files the binary reads
- [ ] personal-data
- [ ] user-interface
- [ ] performance
- [ ] operations
- [x] migration — archive refuses changes that were fine before

## Release note

- Added: Each change keeps a review record, `review.md`, where the roles
  write their findings with a severity, a confidence, a location and the
  quoted evidence, and where every answer — fixed, disputed, accepted
  risk, deferred, rejected — is recorded with who decided it. `codev
  review` shows it and what is left to decide.
- Added: `codev validate` checks the review record: its format, that each
  cited file and line exists and that the quoted evidence is really
  there.
- Changed: `/codev-propose` and `/codev-apply` write their findings into
  the review record, let the author answer each one and the reviewer
  re-check it, and ask you to decide what is still disputed.
- Changed: **BREAKING** `codev archive` refuses (`review_blocked`) a change
  whose review record is missing for a step where roles were active, or
  that still has a high-severity finding open or disputed. For a change
  started with an earlier codev, run the review again or turn its review
  roles off in its `change.yaml`; a team that does not use the skills
  turns `skeptic` and `newcomer` off in `_codev/config.yaml`.
