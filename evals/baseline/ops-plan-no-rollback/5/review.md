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
  Stripe client behind a configuration switch (provider selected per
  environment), or at least keep the Stripe account open and the previous
  release deployable for a stated period (for example several weekly
  deploy cycles), and only close the account and delete the client in a
  later change. Failure scenario: Paylane rejects or times out on
  authorizations after the release; redeploying the previous version
  (blue/green) does not help once the Stripe account is closed the next
  day, so card payments stay down until a new release is built. State the
  rollback procedure and the condition for closing the Stripe account in
  the design, and add a task for the switch.
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
  Decide how operators see the worker's health: a log at start and end of
  each run with structured fields (run date, payments fetched, matched,
  unmatched), a metric or a last-successful-run timestamp, and an alert
  when a run fails or does not complete by a deadline. Failure scenario:
  the Paylane API is unreachable at 02:00 or the CronJob is never
  scheduled; the job fails, nothing is written, and since the only alert
  in the plan fires on unmatched payments, the finance channel stays
  silent and the night's settlement is missing unnoticed — the
  requirement "Payments are settled nightly" is broken without a signal.
  Also say whether a failed run may be re-run safely for the same day
  (idempotent matching) and add the monitoring to task 1.2.
history:
  - status: open
    by: ops
```
