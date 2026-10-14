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
  Space the attempts with an exponential backoff and jitter (for example 1 s,
  2 s, 4 s, 8 s), or hand failed deliveries to a delayed retry queue so the
  payment event listener is not held while waiting. The retries are bounded
  and the operation is deduplicated by `x-event-id`, so only the spacing is
  missing.
history:
  - status: open
    by: ops
```

When a merchant's endpoint has a short outage or answers 503 while it
restarts, the five attempts are sent immediately one after the other (a
non-ok response goes straight to the next iteration, an exception only
waits for the log call). All five fail within the same outage window and the
event is dropped, so the retry gives no real resilience; and during a wider
incident every payment event hammers the already struggling merchant servers
with five requests in a burst.

#### R-ops-2 A dropped webhook is only logged at debug level, in free text, without the HTTP status

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
  Use the project's structured `logger` (as `src/events.ts` does) instead of
  `console.debug`: log each failed attempt with fields (`eventId`,
  `attempt`, `status` of a non-ok response or the error), and log the final
  drop at `error` level with `eventId`, the merchant or payment id, the last
  status and the error, so a dropped delivery is visible and alertable. The
  URL may carry a merchant token: do not log it whole.
history:
  - status: open
    by: ops
```

When a merchant's endpoint answers 500 (or 401, 404) on every attempt, the
non-ok branch logs nothing — the status is never recorded — and the final
"dropped" line is emitted with `console.debug`, which is usually filtered out
in production. The merchant never receives the payment notification and the
operator has no error log, no queryable field and no status code to notice
or diagnose it; `deliver` returns `false` and the listener ignores that
return value.
