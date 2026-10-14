# Review record: merge-duplicate-customers

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### skeptic

#### R-skeptic-1 The plan assumes one e-mail is one person, which its own context contradicts

```yaml
severity: high
confidence: high
location: "design.md#Decision: Customers are merged by e-mail"
evidence: |
  Since an e-mail address identifies exactly one person, the job groups
  customers by lower-cased e-mail, keeps the oldest record of each group,
  moves the others' orders to it and deletes the others.
recommendation: >-
  The design's own Context says the opposite: "Business customers often
  share a purchasing mailbox (`orders@acme.example`) between several buyers,
  each with their own customer record and billing address." Under the
  decision as written, the nightly job merges those distinct buyers into the
  oldest one, moves every buyer's orders to that one person and deletes the
  others' records along with their names and billing addresses — the wrong
  thing built, and data lost. Drop the assumption or ground it: choose a
  matching key that separates shared mailboxes (e.g. e-mail plus name and
  billing address, or only records created by guest checkout with identical
  details), or exclude known shared mailboxes. Then add a spec scenario
  "Shared mailbox, different buyers" whose THEN is that both records remain,
  and cover it in task 1.1.
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
  The deletion is a one-way door: the deleted records' `name` and
  `billing_address` (app/schema.sql, NOT NULL columns) are gone, and a wrong
  merge cannot be undone because nothing records which orders belonged to
  which customer. Add a way back before deleting — a merge log of the moved
  order ids and the removed rows, or a soft delete / `merged_into` column —
  and a staged rollout (a dry-run mode reporting the groups it would merge
  before the first real run). Add the corresponding requirement and task.
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
  The only scenarios are "Two records, one e-mail" and "No duplicate". The
  design matches on "lower-cased e-mail", but no scenario says that
  `Ada@Example.org` and `ada@example.org` are merged, so the case rule is
  not verifiable. Missing too: a group of three or more records, two records
  with the same `created_at` (which one is "the oldest"), and a guest
  checkout that inserts a customer or an order while the job runs. Add a
  scenario for each case the job must handle, and list them in task 1.1.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-4 Operations is unticked although the plan adds a scheduled job

```yaml
severity: medium
confidence: high
location: "proposal.md#Domains"
evidence: |
  - [ ] operations
recommendation: >-
  The What Changes adds "A nightly job merges customers that share an
  e-mail address." — something that must be scheduled, monitored and
  stopped if it misbehaves — yet the operations box is unticked with no
  reason, so no expert reviews how it runs. Tick it, or keep it unticked
  with the reason on its line.
history:
  - status: open
    by: skeptic
```

### privacy

Disabled by configuration.
