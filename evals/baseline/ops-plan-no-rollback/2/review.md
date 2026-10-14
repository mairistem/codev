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
  Keep a way back until Paylane has proven itself in production: either keep
  the Stripe client behind a configuration switch (provider selected per
  environment, flippable without a release), or at least keep the Stripe
  account open for a stated rollback window (several weekly release cycles)
  so that deploying the previous version still processes payments. Close the
  account in a later change, once the window has passed. Say in the design how
  a rollback is done and who decides it. Scenario that fails: on day 2 Paylane
  rejects or times out on a share of authorizations; the previous release
  still calls Stripe, but the Stripe account is closed, so a redeploy cannot
  restore card payments, and the next fix waits for the weekly release.
history:
  - status: open
    by: ops
```

#### R-ops-2 Nobody sees the settlement worker fail or not run

```yaml
severity: high
confidence: high
location: "design.md#Decision: A worker settles payments nightly"
evidence: |
  A new worker, `settlement-worker`, runs at 02:00, fetches the day's Paylane
  payments and matches them to orders. It runs as a Kubernetes CronJob and
  writes its result to the database.
recommendation: >-
  Decide in the design how an operator learns that a run failed or never
  happened: alert on a failed CronJob (or on the absence of a successful run
  by a set hour), log a structured start and end of each run with the counts
  fetched, matched and unmatched, and expose the last successful run time.
  Add a requirement and a scenario for a failed run (Paylane unreachable,
  database error) next to "A payment without order". Scenario that fails:
  the Paylane API is down at 02:00 or the job crashes; the CronJob fails, no
  row is written and no alert is sent — the finance alert only fires for a
  payment without order in a successful run — so unsettled payments go
  unnoticed for days.
history:
  - status: open
    by: ops
```

### skeptic

Disabled by configuration.
