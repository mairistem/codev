# Review record: switch-payment-provider

<!-- codev-review: 1 -->

## plan.challenge

### ops

#### R-ops-1 No way to turn Paylane off or fall back to Stripe

```yaml
severity: high
confidence: high
location: "design.md#Decision: All traffic moves at deploy"
evidence: |
  The Stripe client is deleted and replaced by the Paylane client in the same
  release. There is no switch between the two: keeping both would double the
  code to maintain.
recommendation: Keep the Stripe client behind a configuration switch (or a percentage flag) selecting the provider, so an operator can send traffic back to Stripe without a release; remove it in a later change once Paylane is proven.
history:
  - status: open
    by: ops
```

If Paylane fails in production (outage, integration bug, mass declines), every card payment fails and the only remedy is a new release, with weekly deploys.

#### R-ops-2 Closing the Stripe account removes the redeploy rollback too

```yaml
severity: high
confidence: high
location: "design.md#Decision: All traffic moves at deploy"
evidence: |
  The Stripe account is closed the day after the release.
recommendation: Keep the Stripe account open until Paylane has run through a defined observation period (for example several settlement cycles without incident), and state that exit criterion in the design.
history:
  - status: open
    by: ops
```

Once the account is closed, redeploying the previous blue/green version no longer restores payments: the change has no rollback path at all after day one.

#### R-ops-3 Settlement worker has no health, metrics or failure alert

```yaml
severity: high
confidence: high
location: "design.md#Decision: A worker settles payments nightly"
evidence: |
  A new worker, `settlement-worker`, runs at 02:00, fetches the day's Paylane
  payments and matches them to orders. It runs as a Kubernetes CronJob and
  writes its result to the database.
recommendation: Specify how a failed or missed run is detected and alerted (job failure / last-success timestamp metric, counts of payments fetched, matched and unmatched, a start/end log with run id), and add a scenario for "the worker fails".
history:
  - status: open
    by: ops
```

Only the unmatched-payment case alerts; if the CronJob crashes, cannot reach Paylane or never starts, nothing tells anyone, and payments silently go unsettled.

#### R-ops-4 No timeout or retry policy for Paylane calls

```yaml
severity: medium
confidence: high
location: "specs/payments/spec.md#Requirement: Card payments go through Paylane"
evidence: |
  Every card payment SHALL be authorized and captured through Paylane once the change is deployed.
recommendation: State a timeout on every Paylane call (checkout and worker), bounded retries with backoff only on idempotent operations, and an idempotency key on capture so a retry cannot charge twice.
history:
  - status: open
    by: ops
```

A hung Paylane call blocks checkout requests indefinitely, and an unbounded retry of capture can double-charge a customer.

#### R-ops-5 Paylane errors other than a decline are not specified

```yaml
severity: medium
confidence: high
location: "specs/payments/spec.md#Scenario: A declined card"
evidence: |
  - **WHEN** Paylane declines the card
  - **THEN** the customer sees that the payment was declined
recommendation: Add a scenario for Paylane being unavailable or returning an error (timeout, 5xx, auth failure) — what the customer sees and what is logged, as structured fields with the order id and Paylane reference and no card data or API key.
history:
  - status: open
    by: ops
```

When Paylane errors rather than declines, the plan defines neither the customer outcome nor a log an operator can act on, so an outage is hard to diagnose.

#### R-ops-6 Paylane credentials and endpoint configuration are unplanned

```yaml
severity: low
confidence: low
location: "proposal.md#Impact"
evidence: |
  - `services/checkout`, a new `settlement-worker`, Kubernetes manifests.
recommendation: State that the Paylane URL and API key are read from environment / Kubernetes secrets for both checkout and the worker, and add the secret to the manifests task.
history:
  - status: open
    by: ops
```

Neither the design nor the tasks say where the Paylane credentials live; they risk being hard-coded or missing from the worker's manifest.

### skeptic

Disabled by configuration.
