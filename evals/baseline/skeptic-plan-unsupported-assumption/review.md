# Review record: merge-duplicate-customers

<!-- codev-review: 1 -->

## plan.challenge

### skeptic

#### R-skeptic-1 The merge key rests on an assumption the design itself contradicts

```yaml
severity: high
confidence: high
location: "design.md#Decision: Customers are merged by e-mail"
evidence: |
  Since an e-mail address identifies exactly one person, the job groups
  customers by lower-cased e-mail, keeps the oldest record of each group,
recommendation: Drop the assumption that one e-mail means one person. The design's own Context says business buyers share a purchasing mailbox (`orders@acme.example`), each with their own record and billing address, so a pure e-mail merge would fold several real customers into one. Narrow the merge key, for example to guest records only, or e-mail plus name or billing address, or exclude shared mailboxes, and add a spec scenario for the shared-mailbox case.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-2 Losing records are deleted outright, with no way back

```yaml
severity: high
confidence: high
location: "design.md#Decision: Customers are merged by e-mail"
evidence: |
  moves the others' orders to it and deletes the others.
recommendation: Make the merge reversible. Soft-delete the merged records or keep a mapping of merged id to surviving id with the original data, and add a dry-run or report mode before the first destructive run. Say what happens to the deleted records' own data (billing addresses, names) and to anything else that references their ids besides orders.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-3 The specs only cover the happy path

```yaml
severity: medium
confidence: high
location: "specs/customer-merge/spec.md#Requirement: Duplicate customers are merged"
evidence: |
  A nightly job SHALL merge customer records that share an e-mail address into the oldest one, moving their orders to it.
recommendation: |
  Add scenarios for: e-mails that differ only in case (the design lower-cases them, the requirement does not say so); a missing or empty e-mail; a shared business mailbox (see R-skeptic-1); conflicting data between records (which billing address and name survive); and a job that fails half-way or overlaps a checkout in progress.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-4 No domain is ticked although the job deletes customers' personal data

```yaml
severity: medium
confidence: high
location: "proposal.md#Domains"
evidence: |
  - [ ] personal-data
  - [ ] user-interface
  - [ ] performance
  - [ ] operations
  - [ ] migration
recommendation: Tick personal-data (the job merges and deletes customer records holding e-mails, names and billing addresses) and migration (it rewrites existing orders and removes existing rows), and consider operations for a nightly job (logging, monitoring, rollback). Otherwise give the reason on each unticked line. As written, no expert reviews the change.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-5 The plan assumes customer and order tables the code does not show

```yaml
severity: medium
confidence: low
location: "proposal.md#Impact"
evidence: |
  - `jobs/merge_customers.py` (new).
recommendation: Check the data model the job will run against. The only database code in the repository (`app/db.py`) is for a "members service" with a `users` table (`INSERT INTO users (email, display_name)`). It has no customer, order or billing-address table and no job runner. Name the real tables, their foreign keys and the nightly scheduler in the design, or plan the work to create them.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-6 The task is verified only by the two happy-path tests

```yaml
severity: low
confidence: high
location: "tasks.md#1. Merge"
evidence: |
  - [ ] 1.1 Nightly merge job, verified by tests "Two records, one e-mail" and "No duplicate"
recommendation: |
  Split it into observable tasks: the merge logic, the nightly scheduling, and a way to undo or audit a run. Tie each one to the scenarios R-skeptic-3 asks for.
history:
  - status: open
    by: skeptic
```
