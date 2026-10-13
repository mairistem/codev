# Review record: add-merchant-webhooks

<!-- codev-review: 1 -->

## code.review

### ops

#### R-ops-1 Retries fire back to back, with no backoff

```yaml
severity: medium
confidence: high
location: "src/webhooks.ts:9-23"
evidence: |
  for (let attempt = 1; attempt <= MAX_ATTEMPTS; attempt++) {
    try {
      const response = await fetch(url, {
recommendation: Wait between attempts with exponential backoff and jitter (e.g. 1s, 2s, 4s…), or better, hand failed deliveries to a queue retried over minutes; a merchant endpoint that is down for a few seconds otherwise exhausts all five attempts at once and the event is lost.
history:
  - status: open
    by: ops
```

#### R-ops-2 Every non-2xx response is retried and never logged

```yaml
severity: medium
confidence: high
location: "src/webhooks.ts:17-19"
evidence: |
  if (response.ok) {
    return true;
  }
recommendation: Log the response status (and event id, attempt) when it is not ok, and stop retrying on non-retryable statuses (4xx other than 408/429); today a 404 or 401 from the merchant is retried five times and leaves no trace of why the delivery failed.
history:
  - status: open
    by: ops
```

#### R-ops-3 Dropped deliveries are logged only at debug level, through console

```yaml
severity: high
confidence: high
location: "src/webhooks.ts:24"
evidence: |
  console.debug(`webhook ${event.id} dropped after ${MAX_ATTEMPTS} attempts`);
recommendation: The per-attempt failure at line 21 is logged the same way. Use the project's structured logger (`src/logger.ts`, as `src/events.ts` does) with fields (eventId, merchant, attempt, status, error), log the final drop at error level, and emit a delivery-failure metric; as written, a merchant that never receives a settled-payment notification goes unnoticed in production, where debug output is normally filtered.
history:
  - status: open
    by: ops
```

#### R-ops-4 Delivery runs inline in the payment event publish, blocking it for up to ~25 s

```yaml
severity: medium
confidence: high
location: "src/webhooks.ts:33-35"
evidence: |
  if (rows.length > 0) {
    await deliver(rows[0].webhook_url, event);
  }
recommendation: Do not await delivery inside the listener — enqueue it (outbox table or job queue) and deliver asynchronously; `publish` in `src/events.ts` awaits each listener in turn, so a slow or down merchant endpoint (5 attempts × 5 s timeout) stalls the payment service's publish path and every listener after this one.
history:
  - status: open
    by: ops
```

#### R-ops-5 No switch to turn webhooks off without a redeploy

```yaml
severity: medium
confidence: high
location: "src/webhooks.ts:28"
evidence: |
  onPaymentEvent(async (event) => {
recommendation: Register the listener behind a configuration flag read from the environment (e.g. `WEBHOOKS_ENABLED`), so operators can stop outgoing calls to merchants — for instance during a retry storm — without rolling back the release.
history:
  - status: open
    by: ops
```

#### R-ops-6 Timeout and attempt limit are hard-coded

```yaml
severity: low
confidence: high
location: "src/webhooks.ts:4"
evidence: |
  const MAX_ATTEMPTS = 5;
recommendation: Read the attempt limit and the request timeout (`AbortSignal.timeout(5000)` at line 15) from configuration, with these values as defaults, so they can be tuned without a release.
history:
  - status: open
    by: ops
```

### newcomer

Disabled by configuration.
