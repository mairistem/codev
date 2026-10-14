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
  Wait between attempts with exponential backoff and jitter (for example 1s,
  2s, 4s, 8s), and retry only on network errors, timeouts, 429 and 5xx — not
  on other 4xx responses, which will not succeed on a retry. The loop is
  bounded and the POST is safe to repeat (merchants deduplicate by
  `x-event-id`), but the attempts are not spaced: when a merchant's server
  restarts or returns 503 for a few seconds, the five attempts are spent
  within that window (instantly on fast 5xx responses) and the event is
  dropped, while an overloaded merchant gets hit five times in a row. As
  `publish` in `src/events.ts` awaits each listener in turn, the retries
  also hold the payment service's call for up to 25 s per event; spacing
  them out makes that worse, so move delivery off the publish path (a
  queue or a background job) when adding the backoff.
history:
  - status: open
    by: ops
```

#### R-ops-2 Failed and dropped deliveries are invisible to operators

```yaml
severity: high
confidence: high
location: "src/webhooks.ts:17-25"
evidence: |2
        if (response.ok) {
          return true;
        }
      } catch (error) {
        console.debug(`webhook ${event.id} attempt ${attempt} failed`, error);
      }
    }
    console.debug(`webhook ${event.id} dropped after ${MAX_ATTEMPTS} attempts`);
    return false;
recommendation: >-
  Log through the project's structured `logger` (`src/logger.ts`, as
  `src/events.ts` does) rather than `console.debug`: a `warn` per failed
  attempt with `eventId`, `attempt`, the HTTP `status` or the error, and an
  `error` when the event is dropped, with `eventId`, the merchant id and the
  payment id; add a counter of dropped deliveries an operator can alert on.
  Today a merchant whose endpoint returns 500 gets no attempt logged at all
  (a non-ok response falls through without any log), the final drop is a
  free-text debug line that production log levels filter out, and the
  `false` returned by `deliver` is ignored by the listener at line 34 — so a
  payment webhook is lost permanently with nothing for the operator or the
  merchant support team to see.
history:
  - status: open
    by: ops
```
