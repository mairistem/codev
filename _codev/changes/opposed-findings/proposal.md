# Proposal: Opposed review findings reach the human together

## Why

Two reviewing roles can ask for opposed fixes at the same place — the
performance expert "add a cache here", the skeptic "no cache, the data
would go stale". The meeting keeps them as two findings, since their
fixes differ, but nothing links them. The author can then fix the first
and dispute the second by citing the first. If the second role accepts,
the conflict is settled with no human. If it maintains, the human sees the
second finding without the first. A `medium` one can even be deferred by
the author alone, and in inline mode one agent plays every role and can
settle it silently. The choice between two reviewers is a judgement that
belongs to a person, and today the record cannot even say that two
findings oppose each other.

## What Changes

- The review procedure shared by `/codev-propose` and `/codev-apply`
  (the review-record and isolated-review fragments) gains a rule. When
  two findings ask for fixes that cannot both be applied, they are both
  kept and linked to each other. That holds at the isolated meeting and
  in inline mode alike. The author fixes, disputes and defers neither,
  and does not cite one to answer the other. The "To decide" block
  presents them side by side, with the choice between the two fixes.
- A finding of the review record may carry `conflicts_with`, a list of
  the ids of the findings it opposes. `codev validate` reports an
  unreadable list, an id that is not in the record, a finding listed
  twice or listing itself, and a link that is not mutual.
- Each link stays unsettled until a human has answered one of its two
  findings, in its history or with a new optional `conflict_decision`
  (`by` a human, and a `note`), which can record the choice whatever the
  statuses. While a link is unsettled, both findings await the human in
  `codev review`, whatever their own statuses.
- `codev archive` refuses a change with an unsettled conflict that holds
  a confident `high` finding, under the new reason
  `review_conflict_unsettled`. An unsettled `medium` conflict waits for
  the human without blocking, as a maintained `medium` dispute does
  today.
- `codev review --json` gives each finding `conflictsWith`, the ids it
  is linked to, empty without the field. The human output names the
  linked findings.
- Compatibility: the record's format marker stays `1`, and a record
  without `conflicts_with` reads exactly as before. An older codev
  reports `review_unknown_field` as a warning for the field and keeps
  working.

## Capabilities

### New Capabilities

### Modified Capabilities

- review-record
- skills

### Removed Capabilities

## Impact

- `crates/codev-core/src/review/` — the field, the record-level checks,
  who a linked finding awaits, and the archive gate.
- `crates/codev-cli/src/contract.rs` — `conflictsWith` in `ReviewV1`
  (additive in v1), and the new gate reason in `archive.blockers`;
  `crates/codev-cli/src/render.rs` for the human output.
- `assets/workflows/fragments/review-record.md` and `review-isolated.md`
  — the rule, rendered into `/codev-propose` and `/codev-apply`;
  `crates/codev-agents` tests assert its text.
- Documentation, in `docs/en` and `docs/fr`: the review-record guide,
  the file-formats reference, the JSON-output reference, the CLI
  reference, the workflow chapter, the skills chapter and the roles
  chapter.
- Evaluation (decision 0008): the review steps are measured in both
  review modes against the baseline. The suite plays one role per case,
  so it can show no regression but cannot yet measure the meeting
  itself.

## Domains

- [ ] security
- [ ] personal-data
- [ ] user-interface
- [ ] performance
- [ ] operations
- [x] migration

## Release note

- Added: Two review findings asking for opposed fixes can be linked with
  `conflicts_with`; the skills keep and link them, and present them side
  by side to the human, who alone chooses between them.
- Changed: `codev review` shows a finding linked to an unsettled
  conflict as waiting for a human. `codev archive` refuses a change whose
  unsettled conflict holds a confident `high` finding, with the new
  reason `review_conflict_unsettled` in
  `archive.blockers`. A `medium` conflict waits for the human without
  blocking.
