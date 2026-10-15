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
recommendation: >-
  Wait between attempts with exponential backoff and jitter (for example 1s, 2s, 4s, 8s),
  or hand failed deliveries to a delayed queue. When a merchant's endpoint answers 503
  or restarts for a few seconds, the five attempts currently run one right after the
  other, so they hit the struggling server in a burst and all fail during the same short
  outage. The event is then dropped although waiting a few seconds would have delivered it.
  Also stop retrying on non-retryable responses (4xx other than 408/429), which can never
  succeed.
history:
  - status: open
    by: ops
```

#### R-ops-2 Failed and dropped deliveries are logged only at debug level, as free text, without the HTTP status

```yaml
severity: medium
confidence: high
location: "src/webhooks.ts:17-24"
evidence: |
  if (response.ok) {
    return true;
  }
  } catch (error) {
    console.debug(`webhook ${event.id} attempt ${attempt} failed`, error);
  }
  }
  console.debug(`webhook ${event.id} dropped after ${MAX_ATTEMPTS} attempts`);
recommendation: >-
  Use the project's structured `logger` (src/logger.ts, already used by src/events.ts)
  and log fields: eventId, merchant or URL host, attempt, HTTP status or error. Log a
  dropped webhook at error level, and add a counter for delivered and dropped webhooks.
  Today, when a merchant's endpoint answers 500, nothing is logged for that attempt
  because only thrown errors reach the catch block. The final "dropped" line is a free-text
  `console.debug` with no status and no merchant, so an operator cannot query it, may not
  see it at production log levels, and cannot tell a merchant why their webhooks stopped.
history:
  - status: open
    by: ops
```

#### R-ops-3 Delivery is awaited inside the payment event listener, so a slow merchant can delay the publisher for up to 25 s

```yaml
severity: medium
confidence: low
location: "src/webhooks.ts:33-35"
evidence: |
  if (rows.length > 0) {
    await deliver(rows[0].webhook_url, event);
  }
recommendation: >-
  Do not await delivery inline: enqueue it, or start it without awaiting and report its
  failure through the logger. `publish` in src/events.ts awaits each listener in turn, so a
  merchant that times out makes the publisher wait 5 attempts × 5 s, and any backoff added
  for R-ops-1 makes that delay longer. Whether this delays settlement depends on how the
  payment service calls `publish`, which this review could not see.
history:
  - status: open
    by: ops
```

### newcomer

Disabled by configuration.
