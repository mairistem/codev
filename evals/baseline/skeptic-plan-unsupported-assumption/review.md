# Review record: merge-duplicate-customers

<!-- codev-review: 1 -->

## plan.challenge

### skeptic

#### R-skeptic-1 The merge rests on "one e-mail = one person", which the design itself refutes

```yaml
severity: high
confidence: high
location: "design.md#Decision: Customers are merged by e-mail"
evidence: |
  Since an e-mail address identifies exactly one person, the job groups
  customers by lower-cased e-mail, keeps the oldest record of each group,
recommendation: Drop or qualify the assumption. The Context of the same design states that several business buyers share `orders@acme.example`, each with their own customer record and billing address; grouping on e-mail alone would fold those distinct buyers into one record and attach their orders to the wrong person. Choose a key that separates them (e.g. e-mail plus name/billing address, or guest-only records), or restrict the job to guest records, and add a spec scenario for the shared mailbox.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-2 Records are deleted with no rollback, dry run or audit trail

```yaml
severity: high
confidence: high
location: "design.md#Decision: Customers are merged by e-mail"
evidence: |
  moves the others' orders to it and deletes the others.
recommendation: The nightly job is a one-way door on customer data. Plan a way back before it runs unattended — a dry-run mode or report, a log of every merge (kept id, removed ids, moved order ids, removed field values), soft deletion or a backup — and a staged rollout, with a task and test for each.
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
recommendation: Add scenarios for the cases that decide correctness — a shared business mailbox with different names and billing addresses, e-mails differing only by case or surrounding whitespace (the design lower-cases, the spec says nothing), records with equal `created_at`, and a checkout inserting a customer while the job runs — and state the expected outcome of each.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-4 What happens to the merged records' name and billing address is unspecified

```yaml
severity: medium
confidence: high
location: "design.md#Decision: Customers are merged by e-mail"
evidence: |
  customers by lower-cased e-mail, keeps the oldest record of each group,
  moves the others' orders to it and deletes the others.
recommendation: Only orders are moved; each `customers` row also holds `name` and `billing_address` (app/schema.sql:4-5), so keeping the oldest record discards the most recent billing details, and every order is then attached to an old address. Say which values survive (oldest, newest, kept as history) and how this personal data is handled; with the personal-data domain ticked and the privacy role disabled by `change.yaml`, no other reviewer checks this.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-5 The cause of the duplicates stays in place, and the simpler fix is not weighed

```yaml
severity: low
confidence: high
location: "design.md#Decision: Customers are merged by e-mail"
evidence: |
  **Alternatives considered**: matching on name and address (fuzzy).
recommendation: "guest_checkout inserts a new customer on every order (app/checkout.py:7-11), so duplicates keep reappearing between runs. Consider, or explicitly reject with a reason, reusing an existing guest record at checkout; the only alternative listed also gives no reason for its rejection."
history:
  - status: open
    by: skeptic
```

#### R-skeptic-6 A single task with only the two happy-path tests

```yaml
severity: low
confidence: high
location: "tasks.md#1. Merge"
evidence: |
  - [ ] 1.1 Nightly merge job, verified by tests "Two records, one e-mail" and "No duplicate"
recommendation: Split the job (grouping, order move, deletion, scheduling) into tasks that each fit one session, and verify each edge case and the rollback path with its own test once the spec gains those scenarios.
history:
  - status: open
    by: skeptic
```

### privacy

Disabled by configuration.
