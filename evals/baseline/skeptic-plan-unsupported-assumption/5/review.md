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
  Drop or qualify the premise. The design's Context states that "Business
  customers often share a purchasing mailbox (`orders@acme.example`) between
  several buyers, each with their own customer record and billing address":
  with the planned rule, the job fuses those buyers into the oldest record,
  reassigns every buyer's orders to it and deletes the other buyers' records
  and billing addresses. Choose a key that tells distinct people apart (e-mail
  plus name and/or billing address, or exclude shared mailboxes), state it in
  the spec's requirement, and add a scenario where two different buyers share
  one e-mail and stay separate.
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
  The merge is a one-way door: once the duplicate rows are deleted and their
  orders moved, nothing in the plan records which order belonged to which
  record, nor keeps the deleted name and billing address. Add a way back —
  keep a merge log (merged id → surviving id, the deleted row's data) or
  soft-delete, and a dry-run or staged first run — plus a task that checks it.
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
  The two scenarios ("Two records, one e-mail", "No duplicate") leave
  unspecified what the design already decides or the data allows: e-mails that
  differ only by case (the design lower-cases, the requirement does not say
  so), duplicates whose name or billing address differ (which values the
  surviving record keeps), and a guest checkout that inserts a customer or an
  order for a record while the job is merging it. Add a scenario per case and
  extend task 1.1 to test them.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-4 The operations domain is unticked for a new scheduled job

```yaml
severity: medium
confidence: high
location: "proposal.md#Domains"
evidence: |
  - [ ] operations
recommendation: >-
  The change adds "A nightly job" (`jobs/merge_customers.py`) that must be
  scheduled, run and monitored, and that deletes rows in production; tick
  `operations` so its expert reviews how the job is deployed, scheduled and
  watched, or give the reason on the line.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-5 The cause of the duplicates is left in place

```yaml
severity: low
confidence: low
location: "proposal.md#Why"
evidence: |
  Guest checkouts create a new customer each time; support sees five records for one person.
recommendation: >-
  The Why names the cause — `guest_checkout` in `app/checkout.py` inserts a
  new customer on every order — yet the plan only cleans up after it nightly,
  so duplicates keep appearing during the day, and the design's "Alternatives
  considered" weighs only fuzzy matching. Say why reusing an existing customer
  at checkout was not chosen, or plan it alongside the job.
history:
  - status: open
    by: skeptic
```

### privacy

Disabled by configuration.
