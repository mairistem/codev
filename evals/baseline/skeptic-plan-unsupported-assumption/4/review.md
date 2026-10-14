# Review record: merge-duplicate-customers

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### skeptic

#### R-skeptic-1 The merge key rests on an assumption the design's own context refutes

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
  their own customer record and billing address." As written, the first nightly
  run fuses those distinct buyers into the oldest record, moves every buyer's
  orders under one name and billing address, and deletes the other buyers'
  records. Pick a merge key that actually identifies one person (e.g. e-mail
  plus name and billing address, or restrict to guest-created records), or
  exclude shared mailboxes, and state the evidence that the chosen key is
  unique per person. Then add the matching spec scenario ("two buyers share a
  mailbox → both records remain").
history:
  - status: open
    by: skeptic
```

#### R-skeptic-2 Deleting merged records is irreversible, with no way back

```yaml
severity: high
confidence: high
location: "design.md#Decision: Customers are merged by e-mail"
evidence: |
  moves the others' orders to it and deletes the others.
recommendation: >-
  The job deletes customer rows (with their name and billing address) and
  rewrites `orders.customer_id`; nothing in the plan keeps the original rows or
  the order-to-customer mapping, so a wrong merge cannot be undone. Add a way
  back — record each merge (merged id → kept id, original row, moved order ids)
  before deleting, or soft-delete — and a staged rollout such as a dry-run
  report reviewed before the first destructive run. Add a task and a scenario
  for it.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-3 The spec covers only the happy path and leaves the surviving data undefined

```yaml
severity: medium
confidence: high
location: "specs/customer-merge/spec.md#Requirement: Duplicate customers are merged"
evidence: |
  - **THEN** one record remains, holding the orders of both
recommendation: >-
  The requirement does not say what happens when the merged records differ in
  name or billing address (whose values survive, or whether they are merged at
  all), nor how e-mails that differ only in case are treated — the design
  lower-cases them, the spec only says "share an e-mail address". Add scenarios
  for case-variant e-mails, for records with conflicting name/billing address,
  and for a shared business mailbox (see R-skeptic-1), and extend task 1.1 with
  the tests that verify them.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-4 A new scheduled job with the operations domain unticked

```yaml
severity: medium
confidence: high
location: "proposal.md#Domains"
evidence: |
  - [ ] operations
recommendation: >-
  The proposal adds "A nightly job merges customers that share an e-mail
  address." — something to schedule, run and monitor — yet `operations` is
  unticked with no reason, and Impact lists only `jobs/merge_customers.py` with
  no scheduler entry. Tick `operations` so its expert reviews how the job is
  scheduled, observed and stopped, or give the reason on its line.
history:
  - status: open
    by: skeptic
```

### privacy

Disabled by configuration.
