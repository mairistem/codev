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
  Keep a way back until Paylane has proven itself in production: keep the
  Stripe client behind a configuration switch (or a percentage rollout) for
  at least one weekly release cycle, and do not close the Stripe account
  until that window has passed without incident. Failure scenario: Paylane
  starts declining or timing out on authorizations two days after the
  release; the previous version can be redeployed, but it calls a Stripe
  account that was closed the day after the release, so no card payment can
  be taken by either provider and the only remedy is a new release against
  a provider that is failing. State the rollback path and the account
  closure criterion in the design.
history:
  - status: open
    by: ops
```

#### R-ops-2 The nightly settlement worker can fail without anyone seeing it

```yaml
severity: high
confidence: high
location: "design.md#Decision: A worker settles payments nightly"
evidence: |
  A new worker, `settlement-worker`, runs at 02:00, fetches the day's Paylane
  payments and matches them to orders. It runs as a Kubernetes CronJob and
  writes its result to the database.
recommendation: >-
  Decide in the design how an operator learns that a run failed or did not
  run: a log line at start and end of each run with the counts fetched,
  matched and unmatched, a metric or heartbeat for the last successful run,
  and an alert when the CronJob fails or no successful run is recorded by a
  given hour. Failure scenario: the Paylane API is unreachable at 02:00 (or
  the CronJob is never scheduled after a manifest error); the job exits or
  never starts, nothing is written to the database and nothing alerts, so
  the day's payments stay unreconciled and the spec's alert to the finance
  channel for unmatched payments never fires either. Add a task for this
  monitoring to tasks.md.
history:
  - status: open
    by: ops
```

### skeptic

Disabled by configuration.
