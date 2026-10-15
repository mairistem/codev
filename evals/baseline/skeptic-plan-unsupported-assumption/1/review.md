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
  The design's Context states the opposite: "Business customers often share a
  purchasing mailbox (`orders@acme.example`) between several buyers, each with
  their own customer record and billing address." Under the decision as
  written, the nightly job collapses every buyer behind `orders@acme.example`
  into the oldest record: their orders are attached to another buyer, and
  their own name and billing address are deleted (`app/schema.sql` keeps
  `name` and `billing_address` per customer row). Choose a key that
  distinguishes the guest duplicates of the Why from distinct people sharing a
  mailbox (e.g. e-mail plus name and/or billing address, or restricting the
  merge to guest-created records), state the fact that justifies it, and add
  a spec scenario "Two buyers share a mailbox → both records remain".
history:
  - status: open
    by: skeptic
```

#### R-skeptic-2 Merged records are deleted with no way back

```yaml
severity: high
confidence: high
location: "design.md#Decision: Customers are merged by e-mail"
evidence: |
  moves the others' orders to it and deletes the others.
recommendation: >-
  Deletion is irreversible and the plan has no rollback, audit trail or staged
  rollout: a wrong merge (see R-skeptic-1) cannot be undone, since the original
  `customer_id` of each moved order and the deleted rows' name and billing
  address are lost. Record each merge (survivor id, merged id, moved order
  ids, the deleted row) or soft-delete the merged records, provide a dry-run
  mode for the first runs, and add a task that verifies a merge can be
  reverted.
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
  The requirement does not say how e-mails are compared, while the design
  groups "by lower-cased e-mail": `Ada@example.org` and `ada@example.org`
  are duplicates per the design but not per the spec, so the requirement
  cannot be checked against the implementation. Add scenarios for case (and
  surrounding whitespace) variants of one address, for three or more records
  of one address (all orders end on the oldest), for ties on `created_at`
  (which record is "the oldest"), and for the shared-mailbox case of
  R-skeptic-1; extend task 1.1 to cite those scenarios' tests.
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
  The change adds a new scheduled job ("A nightly job merges customers that
  share an e-mail address.") and rewrites production data in place (moves
  orders, deletes customers), yet `operations` and `migration` are unticked
  with no reason on their line, so no expert reviews the scheduling,
  monitoring and failure handling of the job, nor the irreversible data
  rewrite. Tick them, or keep them unticked with the reason on the line.
history:
  - status: open
    by: skeptic
```

### privacy

Disabled by configuration.
