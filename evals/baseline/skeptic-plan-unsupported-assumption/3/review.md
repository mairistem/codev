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
recommendation: >-
  The whole plan depends on "an e-mail address identifies exactly one person", which
  the Context section contradicts: "Business customers often share a purchasing mailbox
  (`orders@acme.example`) between several buyers, each with their own customer record
  and billing address." Failing scenario: three buyers at Acme check out with
  orders@acme.example; the nightly job keeps the oldest record, moves the other two
  buyers' orders to it and deletes their records, so their orders now carry the wrong
  buyer's name and billing address (app/schema.sql keeps name and billing_address on
  customers, not on orders). Restrict the merge key so shared mailboxes are not
  collapsed (e.g. e-mail plus name/billing address, or guest-only records, or a
  review queue for groups with differing identities), state the assumption the new key
  relies on, and add a spec scenario "Shared purchasing mailbox → records stay
  separate" with a matching test in tasks.md.
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
  The job hard-deletes the merged records nightly, and neither the design nor tasks.md
  gives a rollback, a record of which orders moved from which customer, or a staged
  rollout (dry run first). A wrong merge cannot be undone: the deleted record's name
  and billing address are gone and the orders' original customer_id is lost. Add a
  merge log (old id → kept id, moved order ids, the deleted row's data) or a soft
  delete, a dry-run mode reviewed before the first real run, and a task that verifies
  a merge can be reversed.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-3 The spec covers only the happy path and leaves the matching rule unstated

```yaml
severity: medium
confidence: high
location: "specs/customer-merge/spec.md#Requirement: Duplicate customers are merged"
evidence: |
  A nightly job SHALL merge customer records that share an e-mail address into the oldest one, moving their orders to it.
recommendation: >-
  The requirement says "share an e-mail address" while the design groups by
  "lower-cased e-mail"; the spec does not say whether `Ada@Example.org` and
  `ada@example.org` match, nor what happens with surrounding whitespace, with two
  records sharing the same created_at, or with records whose names or billing addresses
  differ. Only "Two records, one e-mail" and "No duplicate" are specified, so the
  matching rule cannot be verified. State the normalization in the requirement and add
  scenarios for case variants, ties on the oldest record and differing identities
  (see R-skeptic-1), each with a test named in task 1.1.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-4 A new scheduled job with the operations domain unticked and no reason

```yaml
severity: medium
confidence: high
location: "proposal.md#Domains"
evidence: |
  - [ ] operations
recommendation: >-
  What Changes adds "A nightly job merges customers that share an e-mail address.",
  something deployed and scheduled that must be monitored and can fail half-way, yet
  `operations` is unticked with no reason on its line, so no operations expert reviews
  the scheduling, failure handling or alerting. Tick it, or keep it unticked with the
  reason on its line.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-5 The ticked personal-data domain has no active expert

```yaml
severity: medium
confidence: low
location: "proposal.md#Domains"
evidence: |
  - [x] personal-data — the job merges and deletes customer records
recommendation: >-
  The proposal ticks personal-data because the job merges and deletes customer
  records, but change.yaml sets `privacy: never`, so the privacy role is disabled for
  this change and nobody reviews the deletion of personal data. The reason for the
  override is not visible in the plan; either re-enable the privacy role for this
  change or record why it is not needed.
history:
  - status: open
    by: skeptic
```

### privacy

Disabled by configuration.
