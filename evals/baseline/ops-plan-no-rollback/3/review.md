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
recommendation: >
  Keep a way back for the provider switch-over. If Paylane fails in production
  (outage, authorizations rejected, a bad integration found after the release),
  the only recourse is redeploying the previous version — and once the Stripe
  account is closed the day after, that version can no longer take a single
  card payment: checkout is down with no rollback, and a weekly release cadence
  makes a forward fix slow. Keep the Stripe client behind a configuration switch
  (or a feature flag) for at least one release cycle, keep the Stripe account
  open until Paylane has run in production without incident for an agreed
  period, and add the switch and the account closure as explicit tasks with a
  rollback procedure.
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
recommendation: >
  Make the new CronJob observable. The design's only signal is the finance alert
  for an unmatched payment (spec "A payment without order"); if the job fails
  (Paylane API down, database error, crash) or never starts (CronJob suspended,
  schedule missed), no row is written and no alert fires, so the day's payments
  go unreconciled and an operator sees nothing. Decide in the design how a
  failed or missed run is detected — a structured log at start and end with the
  counts processed, a success metric or last-success timestamp with an alert
  when it is older than a day, and an alert on job failure — and add it to
  task 1.2.
history:
  - status: open
    by: ops
```
