# Review record: merge-duplicate-customers

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### skeptic

#### R-skeptic-1 The merge key rests on an assumption the design's own context contradicts

```yaml
severity: high
confidence: high
location: "design.md#Decision: Customers are merged by e-mail"
evidence: |
  Since an e-mail address identifies exactly one person, the job groups
  customers by lower-cased e-mail, keeps the oldest record of each group,
  moves the others' orders to it and deletes the others.
recommendation: >-
  The design's Context states the opposite: "Business customers often share a
  purchasing mailbox (`orders@acme.example`) between several buyers, each with
  their own customer record and billing address." With that data, the job
  merges several distinct buyers into the oldest record: their orders end up
  under another person and their names and billing addresses are deleted. Drop
  or prove the assumption: choose a key that separates people sharing a mailbox
  (for example e-mail plus name and billing address, or guest records only),
  and add to specs/customer-merge/spec.md a scenario where two customers share
  an e-mail but are different people and SHALL NOT be merged, with a test in
  task 1.1.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-2 Records are deleted with no way back

```yaml
severity: high
confidence: high
location: "design.md#Decision: Customers are merged by e-mail"
evidence: |
  moves the others' orders to it and deletes the others.
recommendation: >-
  The deletion is irreversible and the plan gives no rollback, no record of
  which customer was merged into which, no dry run and no staged rollout; a
  wrong merge (see R-skeptic-1) cannot be undone. Keep the merged records
  recoverable (a soft delete or a merge log holding the original rows and the
  moved order ids), run the job in report-only mode first, and add a task to
  check the undo path.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-3 The spec covers only the happy path and differs from the design on case

```yaml
severity: medium
confidence: high
location: "specs/customer-merge/spec.md#Requirement: Duplicate customers are merged"
evidence: |
  A nightly job SHALL merge customer records that share an e-mail address into the oldest one, moving their orders to it.
recommendation: >-
  The design groups by "lower-cased e-mail", the requirement says only "share an
  e-mail address", so `Ada@example.org` and `ada@example.org` are merged by one
  and not by the other. The two scenarios cover only two exact duplicates and no
  duplicate; nothing says which name and billing address survive when duplicates
  differ, nor what happens with three or more records or a tie on creation time.
  State the comparison rule in the requirement and add scenarios for these
  cases, each with a test in task 1.1.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-4 The nightly job's operations domain is unticked with no reason

```yaml
severity: medium
confidence: high
location: "proposal.md#Domains"
evidence: |
  - [ ] operations
recommendation: >-
  The proposal adds "A nightly job merges customers that share an e-mail
  address." — a scheduled job that someone must deploy, schedule and monitor —
  yet operations is unticked with no reason, so no expert reviews how the job is
  scheduled, monitored or stopped. Tick operations, or keep it unticked with the
  reason on its line.
history:
  - status: open
    by: skeptic
```
