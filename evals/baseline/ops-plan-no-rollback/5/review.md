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
recommendation: >
  Keep a way back until Paylane is proven in production: keep the Stripe
  client behind a configuration switch (or a percentage rollout) for at least
  one release cycle, and keep the Stripe account open until the switch has
  been off for that period and the first settlements have run cleanly. State
  in the design the rollback procedure (flip the switch, or redeploy the
  previous version while Stripe is still open) and the criterion for closing
  the Stripe account. Scenario that fails today: Paylane has an outage or the
  new client mis-captures payments on day two; deploys are weekly, the Stripe
  account is already closed, so redeploying the previous version cannot
  restore card payments and checkout stays down.
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
recommendation: >
  Decide how an operator learns that the worker failed or did not run: alert
  on a failed or missed CronJob run (or a "last successful settlement"
  timestamp/metric checked against its schedule), log the start and the
  outcome of each run with counts (payments fetched, matched, unmatched), and
  state whether a failed run can be re-run safely for the same day. Scenario
  that fails today: the Paylane API is unreachable at 02:00 or the job
  crashes; nothing is written, no alert fires (the only alert in the spec is
  for an unmatched payment), and the day's payments stay unreconciled
  without anyone knowing.
history:
  - status: open
    by: ops
```
