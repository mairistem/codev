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
recommendation: >-
  The load-bearing fact "an e-mail address identifies exactly one person" is
  refuted by design.md#Context: "Business customers often share a purchasing
  mailbox (`orders@acme.example`) between several buyers, each with their own
  customer record and billing address." Scenario: buyers A (oldest) and B both
  use orders@acme.example; the nightly job moves B's orders to A and deletes B,
  so B's orders are billed to A's address and B no longer exists. Choose a key
  that holds for that case (e.g. e-mail plus name/billing address, or merge only
  records created by guest checkout as the Why describes — the schema in
  app/schema.sql has no column telling guests apart, so the plan must say how),
  or exclude shared mailboxes explicitly. Add a spec scenario "Two buyers share
  a mailbox → both records remain", and align the requirement with the design's
  case-insensitive matching ("share an e-mail address" vs "lower-cased e-mail").
history:
  - status: open
    by: skeptic
```

#### R-skeptic-2 Duplicate records are deleted with no way back

```yaml
severity: high
confidence: high
location: "design.md#Decision: Customers are merged by e-mail"
evidence: |
  moves the others' orders to it and deletes the others.
recommendation: >-
  The deletion is irreversible: the deleted records' name and billing_address
  (app/schema.sql) are lost and the moved orders can no longer be traced back to
  their original customer, so a wrong merge (see R-skeptic-1) cannot be undone.
  Keep a trace before deleting (soft delete, or a merge log mapping old id → kept
  id with the original fields), or stage the rollout (dry-run report first), and
  state the rollback in the design and a task.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-3 The spec covers only the happy path

```yaml
severity: medium
confidence: high
location: "specs/customer-merge/spec.md#Requirement: Duplicate customers are merged"
evidence: |
  A nightly job SHALL merge customer records that share an e-mail address into the oldest one, moving their orders to it.
recommendation: >-
  The only scenarios are "Two records, one e-mail" and "No duplicate". Missing:
  e-mails differing only by case; records with the same e-mail but different
  names or billing addresses (which one's data survives); two records with the
  same created_at (which is "the oldest"); a guest checkout inserting a customer
  or order while the job runs. Add a scenario for each, and extend task 1.1's
  tests to them.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-4 A scheduled job is added with operations unticked and no reason

```yaml
severity: medium
confidence: high
location: "proposal.md#Domains"
evidence: |
  - [ ] operations
recommendation: >-
  The change adds "A nightly job" (proposal.md#What Changes) that must be
  scheduled, run and monitored, and that deletes production rows, yet the
  operations domain is unticked with no reason, so no expert reviews how it is
  deployed, scheduled or watched. Tick operations, or give the reason on its
  line.
history:
  - status: open
    by: skeptic
```

### privacy

Disabled by configuration.
