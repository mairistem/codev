# Review record: switch-payment-provider

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### skeptic

Disabled by configuration.

### ops

#### R-ops-1 The switch to Paylane has no way back once Stripe is closed

```yaml
severity: high
confidence: high
location: "design.md#Decision: All traffic moves at deploy"
evidence: |
  The Stripe client is deleted and replaced by the Paylane client in the same
  release. There is no switch between the two: keeping both would double the
  code to maintain. The Stripe account is closed the day after the release.
recommendation: >-
  Keep a way back that does not depend on a redeploy alone: keep the Stripe
  client behind a configuration switch (provider selected per environment)
  for at least one release cycle, and keep the Stripe account open until
  Paylane has run in production through several nightly settlements without
  incident. State in the design the rollback procedure and the condition that
  allows closing the Stripe account. Scenario that fails: Paylane rejects or
  times out on authorizations two days after the release; deploys are weekly
  and the previous release calls a closed Stripe account, so every card
  payment fails until a new release ships.
history:
  - status: open
    by: ops
```

#### R-ops-2 Nobody would see the settlement worker fail or not run

```yaml
severity: high
confidence: high
location: "design.md#Decision: A worker settles payments nightly"
evidence: |
  A new worker, `settlement-worker`, runs at 02:00, fetches the day's Paylane
  payments and matches them to orders. It runs as a Kubernetes CronJob and
  writes its result to the database.
recommendation: >-
  Decide in the design how a failed or missed run is detected: log a
  structured start and end line with the run date and the counts fetched,
  matched and unmatched; expose a last-successful-run timestamp (metric or
  database row) and alert when it is older than a day; alert on a CronJob
  failure or a Paylane fetch error with the run date. Add a task for it.
  Scenario that fails: the Paylane API is down at 02:00 or the pod is
  evicted; the CronJob fails, no payment is written to `unmatched_payments`,
  so the only alert the spec defines never fires and the day stays
  unreconciled with no one aware.
history:
  - status: open
    by: ops
```
