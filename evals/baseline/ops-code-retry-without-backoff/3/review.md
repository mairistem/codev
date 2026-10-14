# Review record: add-merchant-webhooks

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### newcomer

Disabled by configuration.

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
  Wait between attempts with an exponential backoff and jitter (for example
  1s, 2s, 4s, 8s), so retries are spaced as the operations criterion
  requires. Today, when a merchant's server is down or overloaded (a 503 or
  a connection refused), the five attempts land within a few hundred
  milliseconds: they hammer a server that is already failing, and an outage
  of even a few seconds exhausts every attempt and drops the event for good.
  The retry is bounded and idempotent (x-event-id), so only the spacing is
  missing. Consider also not retrying 4xx responses other than 408/429.
history:
  - status: open
    by: ops
```

#### R-ops-2 Failed and dropped deliveries are logged with console.debug in free text, and HTTP errors not at all

```yaml
severity: medium
confidence: high
location: "src/webhooks.ts:17-24"
evidence: |2
      if (response.ok) {
        return true;
      }
    } catch (error) {
      console.debug(`webhook ${event.id} attempt ${attempt} failed`, error);
    }
  }
  console.debug(`webhook ${event.id} dropped after ${MAX_ATTEMPTS} attempts`);
recommendation: >-
  Use the project's structured logger (src/logger.ts, as src/events.ts
  does) instead of console.debug: log each failed attempt with fields
  (eventId, paymentId, attempt, status or error) — including non-ok HTTP
  responses, which today produce no log line at all — and log the final
  drop at error level with the event id, payment id and last status. When a
  merchant endpoint answers 500 five times, the operator today sees only a
  debug-level prose line with no status and no merchant, and cannot query
  or alert on dropped webhooks.
history:
  - status: open
    by: ops
```
