# Review record: switch-payment-provider

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### ops

#### R-ops-1 Switch to Paylane has no way back once the Stripe account is closed

```yaml
severity: high
confidence: high
location: "design.md#Decision: All traffic moves at deploy"
evidence: |
  The Stripe client is deleted and replaced by the Paylane client in the same
  release. There is no switch between the two: keeping both would double the
  code to maintain. The Stripe account is closed the day after the release.
recommendation: >-
  Keep a way back for the provider switch-over: keep the Stripe client behind
  a configuration switch (or a percentage rollout) selecting the provider, and
  keep the Stripe account open until Paylane has run in production for at least
  one full deploy cycle (about a week) with settlement confirmed. Failure
  scenario: Paylane rejects or times out on authorizations, or a defect in the
  new client shows up, two days after the release — deploying the previous
  version calls Stripe, whose account is already closed, so card payments stay
  down until a fix is written and shipped in a new release. Also state in the
  design how operators detect a Paylane failure rate rising after the switch.
history:
  - status: open
    by: ops
```

#### R-ops-2 settlement-worker has no failure signal, so a missed nightly run goes unnoticed

```yaml
severity: high
confidence: high
location: "design.md#Decision: A worker settles payments nightly"
evidence: |
  A new worker, `settlement-worker`, runs at 02:00, fetches the day's Paylane
  payments and matches them to orders. It runs as a Kubernetes CronJob and
  writes its result to the database.
recommendation: >-
  Decide in the design how the worker is observed: a structured log at start
  and end of each run (run date, payments fetched, matched, unmatched), a
  metric or last-success timestamp, and an alert when the 02:00 run fails or
  does not complete (CronJob failed job, missed schedule, or Paylane API
  unreachable). Failure scenario: the Paylane API is unreachable at 02:00 or
  the job crashes — the CronJob fails, nothing is written, and since the only
  alert in the spec fires for a payment without order, finance sees no alert
  and the day's payments stay unsettled silently. Say also whether a failed
  night is re-run (catching up a missed day) and that a re-run is safe to repeat.
history:
  - status: open
    by: ops
```

### skeptic

Disabled by configuration.
