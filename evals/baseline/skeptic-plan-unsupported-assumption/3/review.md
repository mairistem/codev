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
recommendation: >-
  The decision only holds if one e-mail means one person, and the same
  design's Context says the opposite: "Business customers often share a
  purchasing mailbox (`orders@acme.example`) between several buyers, each
  with their own customer record and billing address." As written, the
  nightly job would fold every buyer of `orders@acme.example` into the
  oldest one, attribute all their orders to that person and delete the
  other buyers' records, names and billing addresses (`orders` has no
  billing address of its own, see app/schema.sql). Choose a merge key
  that is actually one-person (e.g. e-mail plus name and billing address,
  or an explicit exclusion of shared/business mailboxes, or manual
  confirmation), state the assumption it relies on and the evidence for
  it, and add a spec scenario where two different people share one
  e-mail and are NOT merged.
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
  The merge is a one-way door: once orders are re-pointed and the source
  rows deleted, a wrong merge (see R-skeptic-1, or any bug in the job)
  cannot be undone, and it runs unattended every night. Add a way back
  before any deletion: soft-delete or a merge log recording each removed
  record and the orders moved from it, an unmerge procedure, and a staged
  rollout (dry-run report first, then enabling deletion). Add a task for it.
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
  Only "two records, one e-mail" and "no duplicate" are specified. Missing
  cases the design itself raises or implies: two different people sharing
  a mailbox (must not be merged, cf. R-skeptic-1); e-mails differing only
  by case (the design lower-cases, the requirement says "share an e-mail
  address" and does not say whether `Ada@Example.org` matches
  `ada@example.org`); three or more records in a group; which name and
  billing address the surviving record keeps when they differ; and a job
  failing half-way (orders moved but records not deleted, or the
  reverse). Add a scenario for each, and extend task 1.1's tests to match.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-4 Operations is unticked although the plan adds a nightly job

```yaml
severity: medium
confidence: high
location: "proposal.md#Domains"
evidence: |
  - [ ] operations
recommendation: >-
  The plan introduces a new scheduled job ("A nightly job merges customers
  that share an e-mail address.") in a new `jobs/` location the repository
  does not have yet, but leaves operations unticked with no reason, so no
  expert reviews how it is scheduled, monitored and recovered on failure.
  Tick operations, or give the reason on its line. Consider the same for
  migration: the job rewrites `orders.customer_id` and deletes existing
  `customers` rows in production data.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-5 The plan leaves the source of duplicates in place

```yaml
severity: low
confidence: low
location: "proposal.md#Why"
evidence: |
  Guest checkouts create a new customer each time; support sees five records for one person.
recommendation: >-
  The Why names the cause — `guest_checkout` in app/checkout.py inserts a
  new customer on every call — yet the plan only cleans up afterwards,
  nightly, so support still sees duplicates for up to a day and the job
  must run forever. State why the checkout itself is not changed (or why
  that is out of scope), so the reader can tell the cleanup is the
  intended answer rather than a workaround.
history:
  - status: open
    by: skeptic
```

### privacy

Disabled by configuration.
