# Review record: apply-after-settled-plan-review

<!-- codev-review: 1 -->

## plan.challenge

Mode: isolated.

### skeptic

#### R-skeptic-1 The status JSON read by `/codev-update` changes too, so `plan.update` falls under decision 0008 and goes unmeasured

```yaml
severity: medium
confidence: high
location: "design.md#Decision: Exposed as `planReview` in `codev status --json`, present only with a record"
evidence: |
  Present only with a record also keeps the document byte-identical for
  every `codev status` the other skills run before the review exists:
  `/codev-propose` at `plan.write`, `/codev-update`, and every existing
  implementation case, whose fixtures hold no `review.md`.
recommendation: Correct the design's claim and its rejected alternative; `/codev-update` runs `codev status --json` on reviewed changes, and two `plan.update` cases (update-tasks-add, update-specs-ripple-tasks) hold a `review.md`. Add a task that runs `--step plan.update` and compares it with the baseline, or record why the step is exempt.
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    ref: "design.md#Decision: Exposed as `planReview` in `codev status --json`, present only with a record; design.md#Decision: Measured on `code.implement` with a second refusal case; tasks.md 5.2"
  - status: fixed
    by: skeptic
```

#### R-skeptic-2 The plan gate counts any `accepted-risk` or `deferred` as settled, even one that is not by a human or has no justification

```yaml
severity: medium
confidence: high
location: "specs/review-record/spec.md#Requirement: `codev status` says whether the plan review lets the implementation start"
evidence: |
  plan's `high` findings are all `fixed`, `rejected`, `accepted-risk` or
  `deferred`, and whatever the findings of the `code.review` section are.
recommendation: Make the plan gate treat a `high` `plan.challenge` finding whose deciding `accepted-risk` or `deferred` entry breaks the record's rules (not by a human, no note, no `to`) as a blocker, `review_decision_incomplete`; add a spec scenario and a test.
history:
  - status: open
    by: skeptic
  - status: fixed
    by: qa
    ref: "specs/review-record/spec.md (new blocker and scenario: A risk accepted by an agent does not settle the plan); design.md#Decision: The archive gate's judgement, restricted to `plan.challenge`; tasks.md 1.1"
  - status: fixed
    by: skeptic
```

### migration

#### R-migration-1 The new refusal on changes already in flight is not marked BREAKING

```yaml
severity: medium
confidence: high
location: "proposal.md#Release note"
evidence: |
  - Changed: `/codev-apply` refuses to start while the plan review holds a `high` finding `open` or `disputed`; fix the plan, or have a human accept the risk or defer the finding in `review.md`, then run it again.
recommendation: Mark the release-note item and the matching What Changes bullet **BREAKING**, as the archive gate's refusal was in CHANGELOG.md; keep the "what to do" text.
history:
  - status: open
    by: migration
  - status: fixed
    by: product
    ref: "proposal.md#Release note; proposal.md#What Changes"
  - status: fixed
    by: migration
```
