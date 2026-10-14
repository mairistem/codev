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
  moves the others' orders to it and deletes the others.
recommendation: >-
  The load-bearing assumption is refuted three lines above it, in Context:
  "Business customers often share a purchasing mailbox
  (`orders@acme.example`) between several buyers, each with their own
  customer record and billing address." Scenario: two buyers at Acme check
  out with `orders@acme.example`; the job keeps the older record, moves the
  second buyer's orders onto it and deletes the second buyer's record, so
  their name and billing address are lost and their orders are now
  attributed to another person. Pick a merge key that does not collapse
  distinct people (e.g. e-mail plus name and billing address, or e-mail
  only for guest records with matching identity fields), or exclude shared
  mailboxes explicitly; then add a spec scenario "two customers share an
  e-mail but have different names / billing addresses → both records
  remain", covered by a test in tasks.md. Reconsider the rejected
  alternative ("matching on name and address") with this case in mind —
  the design gives no reason for rejecting it.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-2 Losing records are deleted with no way back

```yaml
severity: high
confidence: high
location: "design.md#Decision: Customers are merged by e-mail"
evidence: |
  moves the others' orders to it and deletes the others.
recommendation: >-
  The merge is a one-way door: once a record is deleted, its name, billing
  address and created_at are gone, and the orders' original customer_id is
  overwritten. Any wrong merge (see R-skeptic-1) cannot be undone. The
  design has no Risks / Trade-offs or Migration Plan section and states no
  rollback. Keep the merged-away records recoverable (soft-delete or a
  merge log recording the old customer_id of each moved order and the
  deleted row), and/or run a dry-run report before the first destructive
  run; add a task that verifies a merge can be reverted.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-3 The spec covers only the happy path and leaves "share an e-mail address" undefined

```yaml
severity: medium
confidence: high
location: "specs/customer-merge/spec.md#Requirement: Duplicate customers are merged"
evidence: |
  A nightly job SHALL merge customer records that share an e-mail address into the oldest one, moving their orders to it.
recommendation: >-
  The design groups by "lower-cased e-mail", but the requirement does not
  say whether `Ada@Example.org` and `ada@example.org` "share an e-mail
  address", so the case-folding behavior cannot be checked against the
  spec. The only scenarios are "Two records, one e-mail" and "No
  duplicate": add edge cases for differing case / surrounding whitespace,
  a group of three or more records (which one survives, all orders moved),
  and records with the same created_at (tie-break for "the oldest one").
history:
  - status: open
    by: skeptic
```

#### R-skeptic-4 A new nightly job is deployed with operations unticked and no reason

```yaml
severity: medium
confidence: high
location: "proposal.md#Domains"
evidence: |
  - [ ] operations
recommendation: >-
  What Changes adds "A nightly job merges customers that share an e-mail
  address." — a scheduled component someone must run, observe and be able
  to stop, and which rewrites existing rows. The operations box is
  unticked with no reason, so the ops expert never reviews scheduling,
  failure reporting or how to switch the job off. Tick operations (and
  consider migration, since the job rewrites and deletes existing data),
  or state on the line why it does not apply.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-5 The privacy role is disabled for a change that merges and deletes personal data

```yaml
severity: medium
confidence: high
location: "_codev/changes/merge-duplicate-customers/change.yaml:3-4"
evidence: |
  roles:
    privacy: never
recommendation: >-
  The proposal ticks "personal-data — the job merges and deletes customer
  records", yet the change disables the privacy role, so the one domain the
  plan itself flags is reviewed by nobody (no review of erasure, retention,
  or attributing one person's orders and data to another). Remove the
  `privacy: never` override, or record who decided it and why.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-6 A nightly clean-up leaves the cause of the duplicates in place

```yaml
severity: low
confidence: low
location: "proposal.md#Why"
evidence: |
  Guest checkouts create a new customer each time; support sees five records for one person.
recommendation: >-
  The Why names the cause — `guest_checkout` in app/checkout.py inserts a
  new customer on every order — but the plan only cleans up afterwards, so
  support still sees duplicates for orders placed since the last run. The
  design does not weigh reusing a matching customer at checkout. Say why a
  nightly merge is preferred, or note it as a follow-up.
history:
  - status: open
    by: skeptic
```

### privacy

Disabled by configuration.
