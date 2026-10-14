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
  If Paylane fails in production after the release (authorizations
  rejected, outage, captures not settling), redeploying the previous
  version does not restore payments: the Stripe account is already closed
  the next day, so checkout stays broken with no rollback. Keep the Stripe
  client behind a configuration switch (provider selected per environment)
  and keep the Stripe account open until Paylane has run in production for
  an agreed period; decide the rollback path in the design and add a task
  for it.
history:
  - status: open
    by: ops
```

#### R-ops-2 Nobody would see the settlement worker fail

```yaml
severity: high
confidence: high
location: "design.md#Decision: A worker settles payments nightly"
evidence: |
  A new worker, `settlement-worker`, runs at 02:00, fetches the day's Paylane
  payments and matches them to orders. It runs as a Kubernetes CronJob and
  writes its result to the database.
recommendation: >-
  If the CronJob fails, is not scheduled, or cannot reach Paylane at 02:00,
  the only output is a database write that never happens: the spec alerts
  finance only for an unmatched payment, so a night with no settlement at
  all goes unnoticed until someone checks the books. Decide in the design
  how the worker reports a run — a log at start and end with the run's
  counts, a metric or last-success timestamp, and an alert when a night's
  run fails or does not complete — and add it to task 1.2.
history:
  - status: open
    by: ops
```

### skeptic

Disabled by configuration.
