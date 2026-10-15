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
  design.md#Context states: "Business customers often share a purchasing mailbox
  (`orders@acme.example`) between several buyers, each with their own customer record and billing
  address." The load-bearing assumption "an e-mail address identifies exactly one person" is refuted by the
  design's own Context: several buyers sharing `orders@acme.example` each have their own record and
  billing address. As written, the nightly job would collapse those distinct buyers into the oldest
  one, reassign all their orders to it and delete their records with their billing addresses — it
  builds the wrong thing for business customers. Choose a key that separates them (e.g. e-mail plus
  name and billing address, or restrict the merge to guest-created records with matching identity
  fields), or exclude shared mailboxes explicitly, and state that rule in the design and the spec.
  Add a spec scenario "Two buyers share a purchasing mailbox → both records remain" and test it in
  task 1.1.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-2 Deleting merged customers is a one-way door with no way back

```yaml
severity: high
confidence: high
location: "design.md#Decision: Customers are merged by e-mail"
evidence: |
  moves the others' orders to it and deletes the others.
recommendation: >-
  The job deletes customer records (with their name and billing address) every night, and the
  plan gives no backup, no record of which orders moved from which customer, no dry run and no
  staged rollout. A wrong merge — such as the shared-mailbox case of R-skeptic-1 — cannot be
  undone. Plan a way back: keep a merge log (old id → kept id, original row) or soft-delete the
  merged records, and start with a dry-run report before enabling deletion; add a task for it.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-3 The spec covers only the happy path and does not pin down what "share an e-mail address" means

```yaml
severity: medium
confidence: high
location: "specs/customer-merge/spec.md#Requirement: Duplicate customers are merged"
evidence: |
  A nightly job SHALL merge customer records that share an e-mail address into the oldest one, moving their orders to it.
recommendation: >-
  The design groups by "lower-cased e-mail", but the requirement says only "share an e-mail
  address", and its two scenarios (`ada@example.org` twice, all distinct) cannot tell whether
  `Ada@Example.org` and `ada@example.org` must merge. There is no scenario for case or surrounding
  whitespace differences, for three or more records (which one is kept, ties on `created_at`), nor
  for a record that is not a duplicate of the same person. State the normalization in the
  requirement and add those scenarios, then name them in task 1.1.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-4 A new scheduled job is introduced with the operations domain unticked and no reason

```yaml
severity: medium
confidence: high
location: "proposal.md#Domains"
evidence: |
  - [ ] operations
recommendation: >-
  The proposal adds "A nightly job merges customers that share an e-mail address." — something
  that must be scheduled, monitored and run against production data — yet `operations` is unticked
  without a reason, so no expert reviews how it is run, alerted on or stopped. Tick it, or keep it
  unticked with the reason on its line.
history:
  - status: open
    by: skeptic
```

### privacy

Disabled by configuration.
