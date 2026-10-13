# Review record: switch-payment-provider

<!-- codev-review: 1 -->

## plan.challenge

### skeptic

Disabled by configuration.

### ops

#### R-ops-1 No way back from the switch to Paylane

```yaml
severity: high
confidence: high
location: "design.md#Decision: All traffic moves at deploy"
evidence: |
  The Stripe client is deleted and replaced by the Paylane client in the same
  release. There is no switch between the two: keeping both would double the
  code to maintain. The Stripe account is closed the day after the release.
recommendation: >-
  Keep the Stripe client behind a configuration switch (or feature flag) that
  selects the provider, so card payments can go back to Stripe without a
  release; keep the Stripe account open until Paylane has run in production
  for an agreed period, and write the rollback procedure in the design.
history:
  - status: open
    by: ops
```

#### R-ops-2 A failed or missed nightly settlement goes unnoticed

```yaml
severity: high
confidence: high
location: "design.md#Decision: A worker settles payments nightly"
evidence: |
  A new worker, `settlement-worker`, runs at 02:00, fetches the day's Paylane
  payments and matches them to orders. It runs as a Kubernetes CronJob and
  writes its result to the database.
recommendation: >-
  Give the worker structured start/end logs, metrics (payments fetched,
  matched, unmatched, duration, last success time) and an alert when the job
  fails or has not succeeded in 24 hours; state how a missed night is re-run
  without settling a payment twice.
history:
  - status: open
    by: ops
```

#### R-ops-3 No timeout, retry or idempotency policy for Paylane calls

```yaml
severity: medium
confidence: low
location: "specs/payments/spec.md#Requirement: Card payments go through Paylane"
evidence: |
  Every card payment SHALL be authorized and captured through Paylane once the change is deployed.
recommendation: >-
  Specify in the design a timeout on every Paylane call from
  `services/checkout` and from the worker, bounded retries with backoff, and
  an idempotency key on authorize/capture so a retry cannot charge a card
  twice; define what the customer sees when Paylane does not answer.
history:
  - status: open
    by: ops
```

#### R-ops-4 Unmatched payments are written to a table nobody watches

```yaml
severity: medium
confidence: high
location: "specs/payments/spec.md#Scenario: A payment without order"
evidence: |
  - **THEN** it is written to the `unmatched_payments` table
recommendation: >-
  Emit a metric and an alert (or a report to the team that handles payments)
  when `unmatched_payments` receives rows, with the payment identifier and
  amount, so captured money without an order is acted on.
history:
  - status: open
    by: ops
```

#### R-ops-5 Paylane configuration and credentials are not specified

```yaml
severity: low
confidence: low
location: "proposal.md#Impact"
evidence: |
  - `services/checkout`, a new `settlement-worker`, Kubernetes manifests.
recommendation: >-
  State that the Paylane endpoint, API key and the worker schedule are read
  from the environment (Kubernetes secrets/config), never hard-coded or
  logged.
history:
  - status: open
    by: ops
```
