# Review record: add-merchant-webhooks

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### ops

#### R-ops-1 Failed deliveries are retried immediately, with no backoff

```yaml
severity: medium
confidence: high
location: "src/webhooks.ts:9-23"
evidence: |
  for (let attempt = 1; attempt <= MAX_ATTEMPTS; attempt++) {
recommendation: >-
  Wait between attempts with an exponential backoff and jitter (for example
  1 s, 2 s, 4 s, 8 s, capped), or better, hand failed deliveries to a
  scheduled retry queue so the spacing can span minutes. The retries are
  bounded and the operation is idempotent (merchants deduplicate on
  `x-event-id`); only the spacing is missing.
history:
  - status: open
    by: ops
```

The loop goes straight to the next attempt after a failure. When a merchant's
endpoint returns a 503 or refuses connections during a deploy or a brief
outage, the five attempts fire back to back within milliseconds, all fail
inside the same outage, and the event is dropped — the retry gives no real
second chance. Against a struggling merchant server, every payment event
also becomes a burst of five immediate POSTs, adding load at the worst time.

#### R-ops-2 A dropped webhook is logged only with console.debug, in free text

```yaml
severity: high
confidence: high
location: "src/webhooks.ts:24"
evidence: |
  console.debug(`webhook ${event.id} dropped after ${MAX_ATTEMPTS} attempts`);
recommendation: >-
  Log the final drop with the project's structured logger at error level —
  `logger.error("webhook delivery dropped", { eventId, paymentId, merchant
  identifier, attempts, lastStatus or lastError })` — and count drops in a
  metric an alert can watch, so an operator sees lost notifications and can
  replay them.
history:
  - status: open
    by: ops
```

When a merchant's endpoint is down for the duration of the five attempts, the
payment notification is lost for good, and the only trace is a debug-level,
free-text line that bypasses `src/logger.ts` (one JSON object per line,
shipped to the log platform). It cannot be queried by event, payment or
merchant, carries no last status or error, and is typically filtered out at
debug level: the merchant never learns the payment settled and nobody on the
operator side notices.

#### R-ops-3 Non-2xx responses are retried without any log, and attempt failures are free text

```yaml
severity: medium
confidence: high
location: "src/webhooks.ts:17-22"
evidence: |
  if (response.ok) {
    return true;
  }
  } catch (error) {
    console.debug(`webhook ${event.id} attempt ${attempt} failed`, error);
recommendation: >-
  Log every failed attempt through `logger.warn` with fields — eventId,
  attempt, the HTTP status for a non-2xx response or `String(error)` for a
  network error or timeout — so an operator can tell a merchant returning
  401/404/500 from a timeout.
history:
  - status: open
    by: ops
```

When a merchant's endpoint answers 401, 404 or 500, the response is not
logged at all: the loop silently moves on and only the final drop line
appears. When an operator investigates a merchant's complaint about missing
webhooks, they cannot see which status the merchant returned or whether the
calls timed out; network errors are logged as free text at debug level
outside the structured logger.
