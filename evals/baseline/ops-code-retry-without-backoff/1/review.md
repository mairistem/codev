# Review record: add-merchant-webhooks

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### ops

#### R-ops-1 Webhook retries fire back to back, with no backoff

```yaml
severity: medium
confidence: high
location: "src/webhooks.ts:9-23"
evidence: |
  for (let attempt = 1; attempt <= MAX_ATTEMPTS; attempt++) {
    try {
      const response = await fetch(url, {
recommendation: Wait between attempts with an exponential backoff and jitter (for example 1 s, 2 s, 4 s, 8 s), ideally from a durable queue rather than an in-process loop, so a merchant endpoint that is briefly down or rate-limiting gets time to recover. Today, when the merchant returns a 503 or resets connections for a few seconds, the five attempts run immediately one after another, all fail inside that window, and the event is dropped; meanwhile every payment event hits the struggling server five times in a burst. Also stop retrying on non-retryable answers (4xx other than 408/429), which the loop currently retries just the same.
history:
  - status: open
    by: ops
```

#### R-ops-2 A dropped webhook is invisible to operators

```yaml
severity: high
confidence: high
location: "src/webhooks.ts:24-25"
evidence: |
  console.debug(`webhook ${event.id} dropped after ${MAX_ATTEMPTS} attempts`);
  return false;
recommendation: When a merchant endpoint fails five times, the payment notification is permanently lost and the only trace is a `console.debug` free-text line, which production log levels usually suppress; a non-2xx response is not logged at all (only thrown errors reach the catch), and the caller at line 34 ignores the `false` return. Log the final drop at error level through a structured logger with fields (event id, merchant id, URL host, attempt, HTTP status or error), emit a metric for delivered/failed/dropped deliveries so an alert can fire, log the status of non-ok responses, and persist dropped events so they can be replayed.
history:
  - status: open
    by: ops
```

#### R-ops-3 Errors in the payment listener are not handled

```yaml
severity: medium
confidence: low
location: "src/webhooks.ts:28-36"
evidence: |
  onPaymentEvent(async (event) => {
    const rows = await query<{ webhook_url: string }>(
recommendation: If `query` rejects (database unavailable), the async handler's promise rejects; depending on how `onPaymentEvent` (not in this diff) treats a returned promise, this becomes an unhandled rejection that can crash the process or be silently lost with no event id in the log. Wrap the handler body in a try/catch that logs the event id and error through the structured logger.
history:
  - status: open
    by: ops
```

### newcomer

Disabled by configuration.
