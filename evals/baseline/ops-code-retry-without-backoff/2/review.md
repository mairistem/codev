# Review record: add-merchant-webhooks

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### ops

#### R-ops-1 A dropped webhook is only logged at debug level, and a non-2xx response is not logged at all

```yaml
severity: high
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
  When a merchant's endpoint answers 500 five times, the loop falls through
  without any log for those attempts (only thrown errors are logged), and the
  final give-up is a `console.debug` line, filtered out in production: the
  payment event is permanently lost and no operator can see it or tell the
  merchant. Use the project's structured `logger` (as `src/events.ts` does):
  log each failed attempt with fields (`eventId`, `attempt`, `status` or
  `error`, merchant id or host, never the full URL if it can carry a secret),
  and log the drop with `logger.error` so it can be alerted on; consider a
  counter of dropped deliveries.
history:
  - status: open
    by: ops
```

#### R-ops-2 Failed deliveries are retried immediately, with no backoff

```yaml
severity: medium
confidence: high
location: "src/webhooks.ts:9-16"
evidence: |
  for (let attempt = 1; attempt <= MAX_ATTEMPTS; attempt++) {
    try {
      const response = await fetch(url, {
recommendation: >-
  The retries are bounded and idempotent (`x-event-id`), but not spaced: when
  a merchant's server returns 503 or refuses connections during a restart, the
  five attempts fire back to back within milliseconds, hammering an endpoint
  that is already struggling and exhausting every attempt inside a few-second
  outage, so the event is dropped (see R-ops-1). Wait between attempts with
  exponential backoff and jitter (e.g. 1s, 2s, 4s, 8s), ideally from a queue
  or scheduled job rather than an in-process sleep (see R-ops-3).
history:
  - status: open
    by: ops
```

#### R-ops-3 Webhook delivery runs inline in the payment event listener and blocks `publish`

```yaml
severity: medium
confidence: high
location: "src/webhooks.ts:33-35"
evidence: |
  if (rows.length > 0) {
    await deliver(rows[0].webhook_url, event);
  }
recommendation: >-
  `publish` in `src/events.ts`, called by the payment service once a payment
  is settled, awaits each listener in turn. When a merchant's endpoint hangs,
  `deliver` holds that `publish` call for up to five 5-second timeouts (25s,
  more once backoff is added), delaying every listener registered after it and
  the payment service's own flow; a slow merchant becomes an incident on the
  payment path. Enqueue the delivery (outbox table or job queue) and return
  from the listener immediately, or at least do not await `deliver` there,
  catching and logging its outcome.
history:
  - status: open
    by: ops
```

### newcomer

Disabled by configuration.
