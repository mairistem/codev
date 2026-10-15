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
recommendation: >-
  The design's Context states that "Business customers often share a purchasing
  mailbox (`orders@acme.example`) between several buyers, each with their own
  customer record and billing address". Scenario that fails: three buyers of Acme
  each have a record with `orders@acme.example`; the nightly job keeps the oldest,
  moves the other two buyers' orders to it and deletes their records, so their
  billing addresses are lost and their orders are attributed to the wrong person.
  The Why only concerns guest checkouts ("Guest checkouts create a new customer
  each time"), yet the decision merges every customer sharing an e-mail. Restrict
  the merge to what the Why needs (e.g. guest records only, or e-mail plus another
  matching attribute), state the rule in the spec, and add a scenario
  "two distinct customers share a mailbox → both records remain".
history:
  - status: open
    by: skeptic
```

#### R-skeptic-2 Customer records are deleted with no way back

```yaml
severity: high
confidence: high
location: "design.md#Decision: Customers are merged by e-mail"
evidence: |
  moves the others' orders to it and deletes the others.
recommendation: >-
  The merge is a one-way door: once the duplicates are deleted and their orders
  moved, a wrong merge (see R-skeptic-1) cannot be undone, and the plan has no
  Migration Plan, rollback, dry run or staged rollout. Keep a trace of each merge
  (merged-into link or soft delete, original order ownership) so a merge can be
  reverted, consider a dry-run report before the first real run, and add a task
  that verifies the revert.
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
  The scenarios are only "Two records, one e-mail" and "No duplicate". Missing
  cases: e-mails differing only by case (the design groups by "lower-cased
  e-mail", the spec does not say so, so `Ada@example.org` and `ada@example.org`
  are untested), customers with no or empty e-mail (grouping them would merge
  unrelated guests into one record), a group of more than two records, and a
  checkout creating a record while the job runs. Add a scenario per case and
  extend task 1.1 to test them.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-4 Operations and migration are unticked with no reason

```yaml
severity: medium
confidence: high
location: "proposal.md#Domains"
evidence: |
  - [ ] operations
  - [ ] migration
recommendation: >-
  The plan adds a scheduled job ("A nightly job merges customers that share an
  e-mail address") and rewrites existing production data (moves orders, deletes
  customer records), yet operations and migration are unticked with no reason, so
  no expert reviews scheduling, monitoring and failure handling of the job, nor
  the bulk data change on existing records. Tick both, or give the reason on each
  line.
history:
  - status: open
    by: skeptic
```

### privacy

Disabled by configuration.
