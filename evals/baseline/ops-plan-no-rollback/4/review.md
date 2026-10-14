# Review record: switch-payment-provider

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### skeptic

Disabled by configuration.

### ops

#### R-ops-1 The switch to Paylane has no way back once the Stripe account is closed

```yaml
severity: high
confidence: high
location: "design.md#Decision: All traffic moves at deploy"
evidence: |
  There is no switch between the two: keeping both would double the
  code to maintain. The Stripe account is closed the day after the release.
recommendation: >-
  Keep a way back that a redeploy alone cannot give. Paylane rejects
  authorizations or captures after the release (outage, misconfigured
  merchant account, a card network it declines). Every card payment fails,
  and the operator's only recovery is to redeploy the previous release. But
  from the next day the Stripe account is closed, so that release cannot take
  payments either: checkout stays down until Paylane is fixed. With weekly
  blue/green deploys, even a rollback on day one takes a full release. Keep
  the Stripe client behind a configuration switch (or a percentage rollout)
  for a set period, and close the Stripe account only once Paylane has run
  cleanly for that period. Write in the design the rollback procedure and the
  criterion for closing the account, and add a task for the switch.
history:
  - status: open
    by: ops
```

#### R-ops-2 The nightly settlement-worker can fail or not run without anyone noticing

```yaml
severity: high
confidence: high
location: "design.md#Decision: A worker settles payments nightly"
evidence: |
  It runs as a Kubernetes CronJob and
  writes its result to the database.
recommendation: >-
  Make the new worker visible when it fails. At 02:00 the Paylane API times
  out, the database is unreachable, or the CronJob is never scheduled (bad
  manifest, suspended job): no payment is reconciled, and nothing tells the
  operator. The only alert the plan defines (the finance channel, for an
  unmatched payment) fires only when the worker runs and succeeds, so a
  missing run looks the same as a clean night. Specify in the design a
  structured log at start and end of each run (date settled, payments
  fetched, matched, unmatched), a run-status metric or a last-success
  timestamp, and an alert when a run fails or no successful run happened in
  the last 24 hours. Also decide what a failed night does: re-run it safely
  (the matching should be idempotent) and catch up the missed days. Add a
  scenario to the spec and a task for this.
history:
  - status: open
    by: ops
```
