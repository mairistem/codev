# Review record: switch-payment-provider

<!-- codev-review: 1 -->

## plan.challenge

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
  Keep a way back for the switch-over: keep the Stripe client behind a
  configuration switch (or keep the previous release deployable against a
  live Stripe account), and close the Stripe account only once Paylane has
  run in production for an agreed period with its settlement checked. Scenario
  that fails: Paylane rejects or times out on authorizations two days after
  the release; redeploying the previous version calls a closed Stripe account,
  so every card payment fails and no configuration change restores payments.
history:
  - status: open
    by: ops
```

#### R-ops-2 The nightly settlement worker can fail unseen

```yaml
severity: high
confidence: high
location: "design.md#Decision: A worker settles payments nightly"
evidence: |
  A new worker, `settlement-worker`, runs at 02:00, fetches the day's Paylane
  payments and matches them to orders. It runs as a Kubernetes CronJob and
  writes its result to the database.
recommendation: >-
  Decide in the design how an operator sees the worker fail or not run: a
  log at start and end with run id and counts, a metric of the last
  successful run and of payments settled/unmatched, and an alert when a run
  fails or no successful run happened in the last 24 hours. Scenario that
  fails: the Paylane API is unreachable at 02:00 or the CronJob is never
  scheduled; the job exits with an error or does not start, nothing alerts,
  and the day's payments stay unreconciled until finance notices days later.
history:
  - status: open
    by: ops
```

### skeptic

Disabled by configuration.
