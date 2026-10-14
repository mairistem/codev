# Review record: add-merchant-webhooks

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### ops

#### R-ops-1 Retries fire back-to-back with no backoff

```yaml
severity: medium
confidence: high
location: "src/webhooks.ts:9-23"
evidence: |
  for (let attempt = 1; attempt <= MAX_ATTEMPTS; attempt++) {
recommendation: >-
  Wait between attempts with exponential backoff and jitter (for example 1s, 2s, 4s, 8s,
  capped), or better, hand failed deliveries to a delayed retry queue. As written, when a
  merchant's server returns 5xx or is briefly unreachable, the five POSTs are sent
  immediately one after another: a restart of a few seconds exhausts every attempt and the
  event is lost, while a merchant under load receives a burst of retries for every payment
  event (a retry storm across all its events). Also stop retrying on non-retryable 4xx
  responses (other than 408/429), which the loop currently retries identically.
history:
  - status: open
    by: ops
```

#### R-ops-2 A dropped webhook is only logged at debug level, in free text

```yaml
severity: high
confidence: high
location: "src/webhooks.ts:21-24"
evidence: |2
      console.debug(`webhook ${event.id} attempt ${attempt} failed`, error);
    }
  }
  console.debug(`webhook ${event.id} dropped after ${MAX_ATTEMPTS} attempts`);
recommendation: >-
  Use the project's structured `logger` (src/logger.ts, as src/events.ts does) instead of
  `console.debug`: log each failed attempt with fields (eventId, merchant/payment id, URL host,
  attempt, HTTP status or error) and log the final drop at error level, plus a counter metric
  for delivered/failed/dropped webhooks. Today, when a merchant endpoint is down, debug output
  is typically filtered in production, so the event is permanently lost and nobody — operator
  or merchant — knows. A non-ok HTTP response (`response.ok` false) is not logged at all, so
  the status code that would explain the failure is never visible.
history:
  - status: open
    by: ops
```

#### R-ops-3 Delivery retries block the payment event dispatch loop

```yaml
severity: medium
confidence: low
location: "src/webhooks.ts:33-35"
evidence: |
  if (rows.length > 0) {
    await deliver(rows[0].webhook_url, event);
  }
recommendation: >-
  `publish` in src/events.ts awaits each listener in turn, so a merchant endpoint that times
  out holds the dispatch of that event — and every listener after this one — for up to
  5 × 5 s = 25 s (more once backoff is added). Whether this stalls the payment service depends
  on whether its caller awaits `publish`, which the change does not show. Consider dispatching
  delivery asynchronously (a queue or a fire-and-forget task with its own error handling).
history:
  - status: open
    by: ops
```

### newcomer

Disabled by configuration.
