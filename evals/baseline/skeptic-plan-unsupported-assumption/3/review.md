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
  The Context of the same design states "Business customers often share a
  purchasing mailbox (`orders@acme.example`) between several buyers, each
  with their own customer record and billing address." With e-mail as the
  sole key, the job would fold distinct buyers into one record, attach
  their orders to another buyer and delete their names and billing
  addresses. Either restrict the merge to records that are verifiably the
  same person (e-mail plus name and/or billing address, or guest records
  only), or state and verify why shared mailboxes cannot occur. Add a spec
  scenario "Two buyers share a mailbox" with the expected outcome, and a
  task and test for it.
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
  The job hard-deletes customer rows nightly and the plan gives no backup,
  merge log, soft delete, dry run or staged rollout; a wrong merge (see
  R-skeptic-1) cannot be undone, since the original customer_id of each
  moved order is lost. Record each merge (kept id, deleted id, its fields,
  moved order ids) or soft-delete, and plan a dry-run first run, with a
  task that verifies an unmerge is possible.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-3 The spec covers only the happy path and leaves conflicting fields undefined

```yaml
severity: medium
confidence: high
location: "specs/customer-merge/spec.md#Requirement: Duplicate customers are merged"
evidence: |
  A nightly job SHALL merge customer records that share an e-mail address into the oldest one, moving their orders to it.
recommendation: >-
  The scenarios are only "Two records, one e-mail" and "No duplicate". The
  requirement does not say whether e-mails differing in case are the same
  (the design says "lower-cased e-mail", the spec says "share an e-mail
  address"), nor what happens to the name and billing address of the
  deleted records when they differ from the kept one, nor what happens with
  three or more records or two with the same created_at. Add scenarios for
  case differences, conflicting fields and ties, so the requirement can be
  checked.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-4 The plan schedules and runs a data-deleting job with operations and migration unticked

```yaml
severity: medium
confidence: high
location: "proposal.md#Domains"
evidence: |
  - [ ] operations
  - [ ] migration
recommendation: >-
  The proposal adds "A nightly job merges customers that share an e-mail
  address." — a scheduled job that someone must deploy, run and monitor —
  and the job rewrites orders.customer_id and deletes customer rows in
  existing data. Tick operations and migration, or give on each line the
  reason they do not apply, so their experts review the scheduling, the
  failure handling and the transformation of existing data.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-5 The plan cleans up duplicates without stopping their source

```yaml
severity: low
confidence: low
location: "proposal.md#Why"
evidence: |
  Guest checkouts create a new customer each time; support sees five records for one person.
recommendation: >-
  The cause named in the Why is app/checkout.py `guest_checkout`, which
  always runs "INSERT INTO customers" per order. The plan leaves it as is,
  so duplicates reappear every day until the next run. Consider whether
  reusing the existing guest record at checkout (with the same identity
  rule as R-skeptic-1) would solve the Why with less risk, the nightly job
  then only cleaning historical data.
history:
  - status: open
    by: skeptic
```

### privacy

Disabled by configuration.
