# Review record: merge-duplicate-customers

<!-- codev-review: 1 -->

## plan.challenge

### skeptic

#### R-skeptic-1 The merge rests on "one e-mail = one person", which the design's own context contradicts

```yaml
severity: high
confidence: high
location: "design.md#Decision: Customers are merged by e-mail"
evidence: |
  Since an e-mail address identifies exactly one person, the job groups
  customers by lower-cased e-mail, keeps the oldest record of each group,
  moves the others' orders to it and deletes the others.
recommendation: >-
  The Context of the same design states the opposite: "Business customers often
  share a purchasing mailbox (`orders@acme.example`) between several buyers, each
  with their own customer record and billing address." Run as written, the job
  folds every buyer of such a company into the oldest one: their orders move to
  another person's record and their own name and billing address are deleted.
  Drop or qualify the assumption — e.g. merge only records whose e-mail AND
  name/billing address match, or exclude shared mailboxes — and add a spec
  scenario for "same e-mail, different buyers" stating that they are NOT merged.
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
  Deleting the duplicates is irreversible, and the plan gives no rollback,
  backup, audit trail of which order belonged to which record, or staged
  rollout (e.g. a dry-run report first). A wrong merge — see R-skeptic-1 — cannot
  be undone. Keep a mapping (old id → kept id, with the deleted row's data) or
  soft-delete, and add a dry-run task before the first real run.
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
  says whether `Ada@Example.org` and `ada@example.org` "share an e-mail address"
  (the design lower-cases, the spec does not say), which record wins on equal
  `created_at`, or what happens to a guest checkout inserting a customer or an
  order for a record while the job is merging it. Add scenarios for case
  differences, ties and a concurrent checkout.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-4 Domains the plan touches are left unticked without a reason

```yaml
severity: medium
confidence: high
location: "proposal.md#Domains"
evidence: |
  - [ ] operations
  - [ ] migration
recommendation: >-
  The plan adds a scheduled nightly job ("A nightly job merges customers that
  share an e-mail address.") and rewrites existing production data
  (`orders.customer_id`, deletions in `customers`), yet operations and migration
  are unticked with no reason, so neither expert reviews it. Tick them, or give
  the reason on each line.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-5 The Why names the cause, but the plan only cleans up after it

```yaml
severity: low
confidence: high
location: "proposal.md#Why"
evidence: |
  Guest checkouts create a new customer each time; support sees five records for one person.
recommendation: >-
  `app/checkout.py` `guest_checkout` inserts a new customer on every call. The
  plan leaves that untouched, so duplicates keep appearing and support still sees
  them until the next nightly run. Weigh reusing an existing record at checkout
  (with the same identity rule as R-skeptic-1), or state why the nightly job alone
  is preferred.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-6 No task makes the job run nightly

```yaml
severity: low
confidence: high
location: "tasks.md#1. Merge"
evidence: |
  - [ ] 1.1 Nightly merge job, verified by tests "Two records, one e-mail" and "No duplicate"
recommendation: >-
  The tests verify the merge logic, not that the job is scheduled; the Impact
  lists only `jobs/merge_customers.py (new)`. Add a task, with an observable done
  state, that wires the job to the scheduler.
history:
  - status: open
    by: skeptic
```

### privacy

Disabled by configuration.
