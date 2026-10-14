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
  The whole plan depends on "an e-mail address identifies exactly one person",
  which nothing verifies and which the Context section refutes: "Business
  customers often share a purchasing mailbox (`orders@acme.example`) between
  several buyers, each with their own customer record and billing address."
  Failing scenario: three buyers at Acme share `orders@acme.example`; the
  nightly job folds them into the oldest record, deletes the other two with
  their billing addresses, and attributes every buyer's orders to one person —
  the plan builds the wrong thing. It also merges beyond the Why, which is
  only about guest checkouts creating a new record each time (the release
  note says the same), while the requirement merges every customer sharing an
  e-mail. Choose a key that holds for shared mailboxes — for example restrict
  the merge to guest records, or require e-mail plus another matching
  attribute — state the assumption and its source in the design, and add a
  spec scenario "Shared business mailbox: records are not merged" with a test
  in task 1.1.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-2 Deleting the merged records is irreversible, with no way back

```yaml
severity: high
confidence: high
location: "design.md#Decision: Customers are merged by e-mail"
evidence: |
  moves the others' orders to it and deletes the others.
recommendation: >-
  The job deletes customer records nightly and unattended, and the plan gives
  no rollback, backup, audit trail, dry run or staged rollout. A wrong merge
  (see R-skeptic-1) cannot be undone: the deleted records' names, billing
  addresses and the original order-to-customer link are lost, and the spec
  says nothing about what happens to the fields of the records that are not
  kept. Add a way back — e.g. keep a merge log mapping each removed record and
  its moved orders to the survivor, soft-delete or archive instead of delete,
  start with a dry-run report — and a task that verifies it (an un-merge test,
  or a test that the log allows restoring the original records).
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
  The only scenarios are "Two records, one e-mail" and "No duplicate". Nothing
  says what "share an e-mail address" means when the case differs (the design
  lower-cases, the requirement does not say so), nor what happens to records
  with an empty or missing e-mail — if guest records can lack one, grouping on
  it could merge every such guest into one record. Add scenarios for an e-mail
  differing only in case, and for records without an e-mail (left untouched),
  each covered by a test named in task 1.1.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-4 The new nightly job leaves the operations domain unticked

```yaml
severity: medium
confidence: high
location: "proposal.md#Domains"
evidence: |
  - [ ] operations
recommendation: >-
  The change adds a scheduled job ("A nightly job merges customers that share
  an e-mail address.") that deletes data unattended, yet `operations` is
  unticked with no reason, so nobody reviews how the job is scheduled,
  observed, turned off or rolled back. Tick `operations`, or give the reason
  on its line.
history:
  - status: open
    by: skeptic
```

### privacy

Disabled by configuration.
