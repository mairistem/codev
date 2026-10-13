# Proposal: Role precision, measured against the baseline

## Why

The committed role evaluation baseline (40 cases, recorded 2026-10-13)
finds every seeded defect, but eight of the nine reviewing roles report
more false positives than the threshold allows (two per role): 52 in
all, from 4 for security to 11 for ops. A reviewer that buries one real
defect under four plausible remarks costs the human the time the review
was meant to save. The measure now discriminates, so the roles can be
tuned against it.

## What Changes

- Reviewers grade with evidence: a `medium` or `high` finding reported
  with confidence `high` quotes the line and states the scenario that
  fails; a remark that depends on something the reviewer could not see,
  or a preference with no failure, is recorded with confidence `low`, so
  it is kept in the record but not presented to the human. The review
  record procedure of the `propose` and `apply` skills says so, with
  what is not a finding: a concern another role owns, code or data the
  change does not add or modify, what the plan explicitly excludes, a
  missing test when the role's criteria do not ask for one, and a matter
  of taste.
- Reviewers report each defect once: the consequences of one flaw go in
  its recommendation; two defects that need two different fixes stay two
  findings.
- Each expert's knowledge says what it does not report, from what the
  baseline showed — for example, the ops expert no longer asks for a
  feature flag when deploying the previous version undoes the change, the
  performance expert no longer reports constant-factor savings, the
  accessibility expert no longer reports a working custom widget for not
  being native.
- **The skeptic no longer applies the domain lenses** (security &
  privacy, compatibility & migration, operability, performance,
  accessibility & UX): it challenges the plan's logic and evidence, and
  checks that the proposal's `## Domains` brings the right experts, who
  review their domain themselves.
- The newcomer groups the readability problems of one function in one
  finding, and leaves missing tests and correctness to the other roles.
- Evaluation cases whose fixture held a real defect the case did not
  state are fixed: three controls that were not clean — one whose checks
  passed without the flag it adds, two whose components missed criteria
  their role or pattern requires — and two seeded cases where a reviewer
  may rightly cite the same defect elsewhere. The baseline is recorded again and the reliability chapter
  regenerated.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `skills`
- `review-record`

### Removed Capabilities

None.

## Impact

- `assets/roles/` (every reviewing role but the tracer),
  `assets/workflows/_review-record.md`, and the skills rendered from it
  (`.claude/skills/`, regenerated with `codev update --force`);
  `crates/codev-core` and `crates/codev-agents` tests that check the
  role criteria and the rendered skills.
- `evals/cases/` (fixture fixes), `evals/baseline/` (recorded again).
- Documentation: `docs/en/src/roles.md`, `docs/fr/src/roles.md`,
  `docs/en/src/skills.md`, `docs/fr/src/skills.md`, and the generated
  `reliability.md` of both books.
- Projects pick up the new roles and skills with the next codev release
  (`codev update`); no file of theirs changes format.

## Domains

- [ ] security — the security role's prompt changes, but no input, secret or permission of codev does
- [ ] personal-data — the matches are evaluation fixtures (a worker's `health` port, never run) and the privacy role's prompt; codev handles no personal data
- [ ] user-interface — the match is an evaluation fixture's component, never served; no interface of codev changes
- [ ] performance — no runtime path of codev changes
- [ ] operations — the matches are evaluation fixtures (timeouts and retries of a worker that is never run) and the ops role's prompt; no log, timeout or rollback of codev changes, and the evaluation run stays manual
- [ ] migration — the review record's format and the JSON contract are unchanged; installed skills keep working until `codev update` regenerates them

## Release note

- Changed: The reviewing roles report fewer false positives. A finding
  presented to you with high confidence now quotes the line and states
  the scenario that fails; hypotheses and preferences stay in the review
  record with low confidence. Each defect is reported once, and each
  expert says what lies outside its domain.
- Changed: The skeptic no longer reviews security, privacy, migration,
  operations, performance or accessibility itself: it challenges the
  plan's logic and evidence and checks that the proposal's Domains bring
  the experts who do. Run `codev update` to refresh your skills.
