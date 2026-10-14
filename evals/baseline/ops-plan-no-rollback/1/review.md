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
  Keep a way back until Paylane is proven in production: a configuration
  switch selecting Stripe or Paylane at runtime (or a gradual rollout behind
  it), with the Stripe client kept for that period, and the Stripe account
  kept open until a stated exit criterion is met (e.g. a full settlement
  cycle with no unmatched payments). Today, if Paylane fails or misbehaves
  after the release, the operator has no switch, and redeploying the previous
  version no longer works once the Stripe account is closed the next day:
  card payments stay down until a new release against a reopened account.
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
  Decide in the design how an operator sees the worker fail or not run:
  an alert on a failed or missed CronJob run (e.g. on the time of the last
  successful run), metrics for payments fetched, matched and unmatched, and
  a log at start and end of each run. Today, if the Paylane API is down at
  02:00 or the job crashes or is never scheduled, no settlement happens and
  nothing reports it — the finance alert of the spec fires only for unmatched
  payments of a run that succeeded — so payments go unreconciled for days
  unnoticed.
history:
  - status: open
    by: ops
```

### skeptic

Disabled by configuration.
