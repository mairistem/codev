# Review record: merge-duplicate-customers

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### skeptic

#### R-skeptic-1 The merge rests on "one e-mail = one person", which the design's own context contradicts

```yaml
severity: high
confidence: high
location: "design.md#Decision: Customers are merged by e-mail"
evidence: |
  Since an e-mail address identifies exactly one person, the job groups
  customers by lower-cased e-mail, keeps the oldest record of each group,
recommendation: >-
  The decision's premise is refuted three lines above it, in Context: "Business customers often
  share a purchasing mailbox (`orders@acme.example`) between several buyers, each with their own
  customer record and billing address." Running the job as planned would merge distinct buyers
  into the oldest record, moving their orders to the wrong person and dropping their own billing
  addresses. The Why ("Guest checkouts create a new customer each time") also only concerns guest
  records, while the requirement merges every customer sharing an e-mail. Restrict the merge to
  what the Why needs (for example, guest records folded into the matching record) or add a
  matching key that tells buyers of a shared mailbox apart; then add a spec scenario
  "shared business mailbox: records are not merged" and a test for it in task 1.1.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-2 Deleting the merged records is irreversible and the plan has no way back

```yaml
severity: high
confidence: high
location: "design.md#Decision: Customers are merged by e-mail"
evidence: |
  moves the others' orders to it and deletes the others.
recommendation: >-
  The nightly job hard-deletes customer records with no rollback, dry-run, audit of what was
  merged, or staged rollout; a wrong merge (see R-skeptic-1) cannot be undone, and the design has
  no Risks / Trade-offs or Migration Plan section. Keep a reversible trace (soft-delete or a merge
  log mapping each removed record and its orders to the survivor), run a dry-run report before
  the first real run, and state how to unmerge; cover it with a task and a test.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-3 The requirement leaves e-mail matching edge cases unspecified

```yaml
severity: medium
confidence: high
location: "specs/customer-merge/spec.md#Requirement: Duplicate customers are merged"
evidence: |
  A nightly job SHALL merge customer records that share an e-mail address into the oldest one, moving their orders to it.
recommendation: >-
  The scenarios cover only an exact match and "every e-mail is distinct". Whether
  `Ada@Example.org` and `ada@example.org` "share an e-mail address" is decided only in the design
  ("lower-cased e-mail"), not in the spec, so the requirement cannot be checked on that case;
  nothing says what happens to customers with no or empty e-mail (grouped together and merged, or
  skipped), nor what happens to data held only on the deleted records (billing address). Add
  scenarios for case-insensitive matching, missing e-mail, and three or more duplicates, and cover
  each in task 1.1's verification.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-4 A new nightly job is deployed but the operations domain is unticked with no reason

```yaml
severity: medium
confidence: high
location: "proposal.md#Domains"
evidence: |
  - [ ] operations
recommendation: >-
  What Changes adds "A nightly job merges customers that share an e-mail address." — a new
  scheduled component that must be scheduled, observed, and switched off or rolled back. Tick
  `operations` so its expert reviews the job, or state on the line why it does not apply.
history:
  - status: open
    by: skeptic
```

### privacy

Disabled by configuration.
