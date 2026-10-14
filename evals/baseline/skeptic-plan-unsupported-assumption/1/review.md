# Review record: merge-duplicate-customers

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### skeptic

#### R-skeptic-1 The design's own context contradicts the claim that one e-mail address means one person

```yaml
severity: high
confidence: high
location: "design.md#Decision: Customers are merged by e-mail"
evidence: |
  Since an e-mail address identifies exactly one person, the job groups
  customers by lower-cased e-mail, keeps the oldest record of each group,
recommendation: >-
  The merge key rests on an unverified assumption, and the design's own
  Context disproves it: "Business customers often share a purchasing mailbox
  (`orders@acme.example`) between several buyers, each with their own
  customer record and billing address." Failing scenario: two buyers at
  Acme check out with `orders@acme.example`. The job merges them into the
  oldest record and deletes the other, so that buyer's orders show the first
  buyer's name and billing address. Pick a key that tells shared mailboxes
  apart, for example e-mail plus name or billing address, or limit the merge
  to guest records with matching identity fields. Add a spec scenario where
  two different people share one e-mail and stay separate. The spec's
  Purpose ("customer records that belong to the same person") and its
  requirement ("share an e-mail address") currently encode the same
  assumption, so update both.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-2 The job deletes customer records and gives no way back

```yaml
severity: high
confidence: high
location: "design.md#Decision: Customers are merged by e-mail"
evidence: |
  moves the others' orders to it and deletes the others.
recommendation: >-
  Deleting the merged records cannot be undone. Once their orders move, the
  deleted rows' name and billing address (`app/schema.sql` customers columns)
  exist nowhere else, and a wrong merge (see R-skeptic-1) cannot be split
  again. Add a way back before anything is deleted: soft-delete or a
  `merged_into` link, a merge log that records the moved order ids and the
  deleted rows, or a dry-run first release that only reports the groups it
  would merge. Add a task and a scenario that check the reversal.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-3 The spec covers only the happy path: no rule for e-mail case or for conflicting fields

```yaml
severity: medium
confidence: high
location: "specs/customer-merge/spec.md#Requirement: Duplicate customers are merged"
evidence: |
  A nightly job SHALL merge customer records that share an e-mail address into the oldest one, moving their orders to it.
recommendation: >-
  The design groups by "lower-cased e-mail", but the requirement only says
  "share an e-mail address" and its single duplicate scenario uses identical
  strings. Failing scenario: with `Ada@Example.org` and `ada@example.org`,
  the spec does not say whether they merge, so a test cannot decide whether
  an implementation is right. The spec also does not say which name and
  billing address survive when the records differ, or what happens with
  three or more records or with equal `created_at` values (both inserted by
  `datetime('now')` in `app/checkout.py`, which has one-second precision).
  Add scenarios for case differences, differing fields, more than two
  records and a tie on the oldest record.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-4 The plan deletes personal data, but its expert is disabled for this change

```yaml
severity: medium
confidence: high
location: "proposal.md#Domains"
evidence: |
  - [x] personal-data — the job merges and deletes customer records
recommendation: >-
  The proposal correctly ticks personal-data, but `change.yaml` sets
  `privacy: never`, so no expert will review how this plan merges and deletes
  personal records (names, billing addresses). A person should either
  re-enable the privacy role for this change or record why the review is not
  needed.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-5 The plan cleans up duplicates nightly but leaves the checkout that creates them unchanged

```yaml
severity: low
confidence: high
location: "proposal.md#Why"
evidence: |
  Guest checkouts create a new customer each time; support sees five records for one person.
recommendation: >-
  The Why names the cause: `app/checkout.py` `guest_checkout` always inserts
  a new customer. If nothing changes there, duplicates come back every day
  until the next nightly run, and support still sees them between runs.
  Consider reusing a matching customer at checkout, keyed the way
  R-skeptic-1 settles, so the job only has to clear the existing backlog. Or
  state why the plan keeps creating records at checkout.
history:
  - status: open
    by: skeptic
```

### privacy

Disabled by configuration.
