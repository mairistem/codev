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
recommendation: >
  Keep a way back until Paylane is proven in production: keep the Stripe
  client behind a configuration switch (provider selected per environment,
  flipped without a release), and keep the Stripe account open until a
  stated exit criterion is met (for example N days of Paylane payments with
  an error rate under a threshold). Write the rollback procedure in the
  design. Failure scenario: Paylane starts failing authorizations or
  captures after the release (outage, integration bug, account suspended);
  the next deploy is about a week away, and redeploying the previous blue
  version sends payments to a Stripe account already closed — so no card
  payment can be taken at all, and the operator has no switch to flip.
history:
  - status: open
    by: ops
```

#### R-ops-2 The nightly settlement worker fails unseen

```yaml
severity: high
confidence: high
location: "design.md#Decision: A worker settles payments nightly"
evidence: |
  A new worker, `settlement-worker`, runs at 02:00, fetches the day's Paylane
  payments and matches them to orders. It runs as a Kubernetes CronJob and
  writes its result to the database.
recommendation: >
  Decide in the design how an operator learns that a run failed or did not
  run: a structured log at start and end of each run (run date, payments
  fetched, matched, unmatched), a metric or heartbeat for the last
  successful run, and an alert when a run fails or no successful run is
  recorded by a deadline. Add the matching task to tasks.md. Failure
  scenario: the Paylane API is unreachable at 02:00, or the CronJob is
  never scheduled (bad manifest, suspended job); the worker exits or never
  starts, nothing is written, and the only alert the spec defines — the
  finance-channel alert for an unmatched payment — never fires, so a
  missed settlement looks exactly like a night where everything matched.
history:
  - status: open
    by: ops
```
