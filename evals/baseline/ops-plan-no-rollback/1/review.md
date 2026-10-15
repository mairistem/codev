# Review record: switch-payment-provider

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### ops

#### R-ops-1 The switch to Paylane has no way back

```yaml
severity: high
confidence: high
location: "design.md#Decision: All traffic moves at deploy"
evidence: |
  The Stripe client is deleted and replaced by the Paylane client in the same
  release. There is no switch between the two: keeping both would double the
  code to maintain. The Stripe account is closed the day after the release.
recommendation: >-
  Keep a way back until Paylane is proven in production: keep the Stripe
  client behind a configuration switch (or a gradual rollout) for a defined
  period, and keep the Stripe account open until that period ends. Failure
  scenario: Paylane has an outage, rejects valid cards or a defect appears in
  the new client a few days after the release; redeploying the previous
  version brings back the Stripe client, but the Stripe account is already
  closed, so card payments stop entirely with no rollback, and the next
  release is up to a week away. State the rollback procedure and the date
  the Stripe account may be closed in the design.
history:
  - status: open
    by: ops
```

#### R-ops-2 The settlement worker can fail without anyone seeing it

```yaml
severity: high
confidence: high
location: "design.md#Decision: A worker settles payments nightly"
evidence: |
  A new worker, `settlement-worker`, runs at 02:00, fetches the day's Paylane
  payments and matches them to orders. It runs as a Kubernetes CronJob and
  writes its result to the database.
recommendation: >-
  Decide how an operator learns that a run failed or did not run: a log at
  start and end with the run's counts, a metric or last-success timestamp,
  and an alert when the CronJob fails or no successful run is recorded for
  the day. Failure scenario: the Paylane API is unreachable at 02:00 or the
  job crashes; the CronJob fails, nothing is written and, since the only
  alert planned (spec "A payment without order") fires from a successful
  run, finance is never told that the day's payments were not settled.
  Add a scenario for a failed run to the spec and a task for it.
history:
  - status: open
    by: ops
```

### skeptic

Disabled by configuration.
