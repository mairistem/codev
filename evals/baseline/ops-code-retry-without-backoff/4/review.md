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
  Space the attempts with an exponential backoff and jitter (for example
  1 s, 2 s, 4 s, 8 s, capped), so a merchant endpoint that is restarting or
  briefly overloaded gets time to recover. The operation is safe to repeat
  (merchants deduplicate on `x-event-id`) and the count is bounded, but the
  spacing is missing. Consider also not retrying 4xx responses other than
  408/429, which will not succeed on a retry.
history:
  - status: open
    by: ops
```

Scenario: a merchant's server returns 503 for a few seconds during a deploy.
The loop sends its five POSTs as fast as each response comes back — within
milliseconds when the server answers 503 immediately — so all five attempts
are spent inside the outage, and the event is dropped even though the
endpoint is back a few seconds later. When many events hit the same
struggling merchant, each one multiplies its load by five at once.

#### R-ops-2 A dropped webhook is logged only at debug level, in free text, and failed HTTP statuses are never logged

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
  Use the project's structured `logger` from `src/logger.ts`: log each
  failed attempt with `logger.warn` and fields (`eventId`, `attempt`,
  `status` or `error`, the merchant id or URL host), and the final drop with
  `logger.error` and the same fields, so an operator can find and replay
  lost events. `deliver` returns `false`, but the listener at line 34
  ignores it, so this log is the only trace of the loss.
history:
  - status: open
    by: ops
```

Scenario: a merchant's endpoint answers 500 to every attempt. The non-ok
branch logs nothing (only thrown errors reach the `catch`), and the final
"dropped" message goes to `console.debug`, outside the JSON log stream that
`src/logger.ts` ships to the log platform and below the levels usually kept
in production. The merchant never receives the payment event and no
operator sees that it was lost, nor for which merchant or with what status.

#### R-ops-3 Webhook delivery runs inline in the payment event chain, for up to about 25 s per event

```yaml
severity: medium
confidence: low
location: "src/webhooks.ts:33-35"
evidence: |
    if (rows.length > 0) {
      await deliver(rows[0].webhook_url, event);
    }
recommendation: >-
  Hand the delivery off instead of awaiting it in the listener — a queue or
  outbox table read by a worker, or at least a non-awaited task with its own
  error handling — so a slow merchant cannot delay the other listeners or
  the payment service that calls `publish`.
history:
  - status: open
    by: ops
```

`publish` in `src/events.ts` awaits each listener in turn, and this listener
awaits up to five 5-second timeouts. Whether the payment service awaits
`publish` on its request path could not be verified from the repository, so
the impact on payment latency is unconfirmed.
