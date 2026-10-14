# Review record: switch-payment-provider

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### skeptic

Disabled by configuration.

### ops

#### R-ops-1 The Paylane switch-over has no way back

```yaml
severity: high
confidence: high
location: "design.md#Decision: All traffic moves at deploy"
evidence: |
  The Stripe client is deleted and replaced by the Paylane client in the same
  release. There is no switch between the two: keeping both would double the
  code to maintain. The Stripe account is closed the day after the release.
recommendation: >-
  Keep a way back for the provider switch-over. If Paylane rejects or fails
  payments after the release, redeploying the previous version cannot help:
  the Stripe client is gone from the new release and, from the day after, the
  Stripe account it would call is closed, so checkout stays down until a fix
  ships (deploys are about weekly). Keep the Stripe client behind a
  configuration switch (provider per environment) for at least one release
  cycle, and close the Stripe account only once Paylane has run in production
  without incident for an agreed period; state the rollback procedure in the
  design.
history:
  - status: open
    by: ops
```

#### R-ops-2 Nobody sees the settlement worker fail

```yaml
severity: high
confidence: high
location: "design.md#Decision: A worker settles payments nightly"
evidence: |
  A new worker, `settlement-worker`, runs at 02:00, fetches the day's Paylane
  payments and matches them to orders. It runs as a Kubernetes CronJob and
  writes its result to the database.
recommendation: >-
  Decide how a failed or missing run is seen. If the 02:00 CronJob crashes,
  times out against the Paylane API or is never scheduled, nothing in the
  design reports it: the only alert (spec "A payment without order") fires
  from a run that succeeded, so a night with no settlement looks the same as a
  night with nothing unmatched, and unreconciled payments pile up unnoticed.
  Add to the design an alert on a failed Job and on the absence of a
  successful run in the last 24 hours (e.g. a last-success timestamp or
  metric), plus a structured start/end log with the counts of payments
  fetched, matched and unmatched.
history:
  - status: open
    by: ops
```
