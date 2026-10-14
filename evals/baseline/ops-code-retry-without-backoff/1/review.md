# Review record: add-merchant-webhooks

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### ops

#### R-ops-1 Webhook delivery retries immediately, with no backoff

```yaml
severity: medium
confidence: high
location: "src/webhooks.ts:9-23"
evidence: |
  for (let attempt = 1; attempt <= MAX_ATTEMPTS; attempt++) {
recommendation: >-
  Wait between attempts with an exponential backoff and jitter (for example
  1s, 2s, 4s, 8s), or hand failed deliveries to a queue that retries later.
  As written, when a merchant's server returns 503 or is briefly down, the
  five attempts fire back to back within milliseconds (or every 5s on
  timeouts), hammering an endpoint that is already failing and exhausting
  every attempt before a short outage ends, so the event is dropped.
  Retrying is safe here since merchants deduplicate on `x-event-id`; only
  the spacing is missing.
history:
  - status: open
    by: ops
```

#### R-ops-2 Failed and dropped deliveries are logged as free text at debug level, HTTP failures not at all

```yaml
severity: medium
confidence: high
location: "src/webhooks.ts:17-24"
evidence: |
  console.debug(`webhook ${event.id} dropped after ${MAX_ATTEMPTS} attempts`);
recommendation: >-
  Use the project's structured `logger` (as `src/events.ts` does) and log the
  drop at `error` (attempt failures at `warn`) with fields: eventId,
  merchant or paymentId, attempt, HTTP status or error. Today a merchant
  answering 500 or 404 produces no log at all (only thrown errors reach the
  `catch`), and a webhook given up after five attempts is a `console.debug`
  prose line that debug-filtered production logs hide and that cannot be
  queried or alerted on: a merchant who never received a payment
  notification leaves the operator nothing to find. Consider a metric or
  counter of dropped deliveries as well.
history:
  - status: open
    by: ops
```
