# Review record: switch-payment-provider

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### ops

#### R-ops-1 The switch to Paylane has no way back once the Stripe account is closed

```yaml
severity: high
confidence: high
location: "design.md#Decision: All traffic moves at deploy"
evidence: |
  The Stripe client is deleted and replaced by the Paylane client in the same
  release. There is no switch between the two: keeping both would double the
  code to maintain. The Stripe account is closed the day after the release.
recommendation: >-
  Keep a way back for the provider switch-over. When Paylane fails in
  production (outage, authorizations declined, capture errors) after the
  Stripe account is closed, redeploying the previous version brings back a
  Stripe client with no account behind it, and the next blue/green release
  is about a week away: card payments stay down. Keep the Stripe account
  open until Paylane has run in production for an agreed period, and either
  keep the Stripe client behind a configuration switch (the provider chosen
  from the environment) or state explicitly that the rollback is a redeploy
  of the previous version, valid as long as the Stripe account stays open.
  Add a task for the rollback path and for closing the Stripe account only
  after that period.
history:
  - status: open
    by: ops
```

#### R-ops-2 The nightly settlement worker can fail or not run without anyone seeing it

```yaml
severity: high
confidence: high
location: "design.md#Decision: A worker settles payments nightly"
evidence: |
  A new worker, `settlement-worker`, runs at 02:00, fetches the day's Paylane
  payments and matches them to orders. It runs as a Kubernetes CronJob and
  writes its result to the database.
recommendation: >-
  Decide how an operator sees the worker fail. If the Paylane API is down at
  02:00, the CronJob crashes or is never scheduled, nothing is reconciled and
  nothing alerts: the only alert the spec defines is for an unmatched payment,
  which a run that did not happen never raises. Have the worker log its start
  and end with structured fields (date settled, payments fetched, matched,
  unmatched), emit a metric or a last-success timestamp, and alert when a run
  fails or no successful run is recorded for the day; add this to the spec
  ("Payments are settled nightly") and to task 1.2.
history:
  - status: open
    by: ops
```

### skeptic

Disabled by configuration.
