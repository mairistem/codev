# Review record: opposed-findings

<!-- codev-review: 1 -->

## plan.challenge

Mode: isolated.

### skeptic

#### R-skeptic-1 One old human entry settles every conflict later linked to it

```yaml
severity: medium
confidence: high
location: "design.md#Decision: \"Settled\" means a human entry in the history of one of the conflict's findings"
evidence: |
  A conflict is settled once one of its findings' histories holds an entry
  whose `by` is `human` or `human:<name>`.
recommendation: >-
  Links may cross steps and a conflict was a whole connected component, so
  a human entry made for an earlier conflict settled any conflict linked
  to it later. At plan.challenge the human accepts B's risk, so A is
  chosen and fixed. At code.review a new high finding C opposes A's fix:
  {A, B, C} is one component that already holds a human entry, so C
  awaits only the author, who may dispute it by citing A, and the gate
  does not block. Settle per link, not per component, and add a
  cross-step scenario with its unit test.
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    ref: "specs/review-record/spec.md (settlement per link, scenario 'An earlier choice does not settle a later link'), design.md, tasks.md 1.2-1.3"
  - status: fixed
    by: skeptic
```

#### R-skeptic-2 Planned exit fails when the human picks the fix already made

```yaml
severity: medium
confidence: high
location: "design.md#Decision: The archive blocks an unsettled conflict only through a `high` finding"
evidence: |
  `fixed` finding can be reopened by its role (`fixed` → `open`, with a note
  naming the conflict), and then takes the human's answer.
recommendation: >-
  When the chosen finding is fixed and verified and the other is rejected,
  "keep the applied fix" has no legal record: no entry may follow
  `rejected`, and reopening a holding fix requires a note saying why it
  does not hold. Either spell out and test that route, or let a human's
  choice be recorded on either side whatever the statuses.
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    ref: "specs/review-record/spec.md (`conflict_decision`, scenarios 'A human's choice on a finding that takes no more entries', 'A conflict decision lets the change go'), specs/skills/spec.md, design.md, tasks.md 1.1, 1.4, 3.2"
  - status: fixed
    by: skeptic
```

#### R-skeptic-3 A conflict of only low-confidence findings is stuck

```yaml
severity: medium
confidence: high
location: "specs/skills/spec.md#Requirement: The `propose` and `apply` skills keep the review record and run the answer loop"
evidence: |
  one to answer the other: the conflict goes to the human as it is.
recommendation: >-
  The skill rule forbids the author to answer any linked finding, while
  the record makes a low-only conflict await the author and the human
  never sees it. Make the restraint apply only while a link waits for the
  human, and say that a low-only link is answered as usual; add it to the
  rendered-text assertion of task 3.2.
history:
  - status: open
    by: skeptic
  - status: fixed
    by: qa
    ref: "specs/skills/spec.md (restraint only while a link waits for the human), design.md low-confidence decision, tasks.md 3.2"
  - status: fixed
    by: skeptic
```

### migration

No finding.
