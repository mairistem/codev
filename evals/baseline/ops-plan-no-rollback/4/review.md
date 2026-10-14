# Review record: switch-payment-provider

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### skeptic

Disabled by configuration.

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
  Stripe client behind a configuration switch (provider selected from the
  environment) for at least one release cycle, and do not close the Stripe
  account until that period ends and a rollback criterion is met. State in
  the design how an operator switches back and who decides. Add the
  corresponding task to tasks.md.
history:
  - status: open
    by: ops
```

Failure scenario: after the release, Paylane fails authorizations or
captures (outage, misconfigured account, a card network it rejects). Since
there is no switch, the only remedy is redeploying the previous version —
and once the Stripe account is closed the day after, that previous version
cannot take payments either. With weekly deploys, checkout is down with no
rollback.

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
  Decide in the design how a failed or missed run is seen: an alert when
  the CronJob fails or has not completed successfully by a deadline (e.g.
  last-success timestamp metric or a heartbeat), a structured log at start
  and end with the run date and counts (fetched, matched, unmatched), and
  a way to re-run a given day safely. Add a task for it in tasks.md.
history:
  - status: open
    by: ops
```

Failure scenario: at 02:00 the Paylane API is unreachable, the job's
credentials expired, or the CronJob is never scheduled. The design only
alerts on a payment without an order (spec "A payment without order"); a
run that crashes or never starts produces no alert, so the day's payments
stay unreconciled and nobody notices until finance does.
