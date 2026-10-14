# Review record: add-merchant-webhooks

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### ops

#### R-ops-1 Failed deliveries are retried immediately, with no backoff

```yaml
severity: medium
confidence: high
location: "src/webhooks.ts:9-23"
evidence: |
  for (let attempt = 1; attempt <= MAX_ATTEMPTS; attempt++) {
recommendation: |
  Space the attempts with an exponential backoff and jitter (for example 1 s, 2 s, 4 s, 8 s) before each retry, and do not retry on 4xx responses other than 408/429. The retries are bounded and the POST is deduplicated by `x-event-id`, but with no delay between them, a merchant endpoint that is restarting or returning 503 for a few seconds receives five back-to-back POSTs and the event is given up within that same window, so a short outage on the merchant side loses the webhook. Spacing the retries in the payment listener lengthens its run (see R-ops-3): moving delivery off the publish path fixes both.
history:
  - status: open
    by: ops
```

#### R-ops-2 Failed and dropped webhooks are logged at debug level, outside the structured logger

```yaml
severity: high
confidence: high
location: "src/webhooks.ts:24"
evidence: |
  console.debug(`webhook ${event.id} dropped after ${MAX_ATTEMPTS} attempts`);
recommendation: |
  Log through `logger` from `src/logger.ts` with fields — `logger.warn` per failed attempt and `logger.error` when the event is dropped, with `eventId`, `paymentId`, `merchantId` or host, `attempt` and the HTTP `status` (a non-2xx response is currently not logged at all, only thrown errors are) — and emit a delivered/dropped counter. When a merchant's endpoint returns 500 five times, `deliver` returns `false`, the listener ignores that result (`await deliver(rows[0].webhook_url, event);`), and the only trace is a free-text `console.debug` line that bypasses the project's structured logger shipped to the log platform: the merchant never gets the payment event and no operator can see or query that it was dropped.
history:
  - status: open
    by: ops
```

#### R-ops-3 Webhook delivery runs inline in the payment event publish path

```yaml
severity: medium
confidence: low
location: "src/webhooks.ts:28-36"
evidence: |
  onPaymentEvent(async (event) => {
recommendation: "`publish` in `src/events.ts` awaits each listener in turn, so a slow or unreachable merchant holds it for up to five 5-second timeouts (about 25 s, more once backoff is added) before the next listener runs and `publish` resolves. Whether this delays settlement depends on how the payment service awaits `publish`, which the change does not show. Enqueue the delivery (a job or outbox table) and return from the listener at once."
history:
  - status: open
    by: ops
```

### newcomer

Disabled by configuration.
