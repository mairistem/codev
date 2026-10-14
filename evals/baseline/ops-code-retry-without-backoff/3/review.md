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
  Wait between attempts with exponential backoff and jitter (for example 1 s,
  2 s, 4 s, 8 s, capped), or hand failed deliveries to a queue that retries
  later. Scenario: a merchant endpoint that answers 503 for 30 seconds during a
  deploy, or refuses connections, gets its five attempts within a few hundred
  milliseconds, all fail, and the event is dropped — the retry gives no chance
  to recover from a short outage while hammering a struggling server. Retrying
  is safe (merchants deduplicate by `x-event-id`), so only the spacing is
  missing. Also stop retrying on a 4xx other than 408/429: a 400 or 410 will
  not succeed on a second try.
history:
  - status: open
    by: ops
```

#### R-ops-2 A dropped webhook is logged only at debug level, in free text, through console

```yaml
severity: high
confidence: high
location: "src/webhooks.ts:20-25"
evidence: |
  console.debug(`webhook ${event.id} attempt ${attempt} failed`, error);
  }
  }
  console.debug(`webhook ${event.id} dropped after ${MAX_ATTEMPTS} attempts`);
  return false;
recommendation: >-
  Use the project's structured logger (`src/logger.ts`, as `src/events.ts`
  does) and log the final drop at `error` level with fields — eventId, event
  type, merchant id or URL host, last HTTP status or error, attempts — and log
  each failed attempt at `warn` with the same fields. Scenario: a merchant's
  endpoint returns 500 for every attempt; no attempt is logged at all, because
  only thrown errors reach the `catch` and a non-ok response falls through
  silently; the final drop goes to `console.debug`, which production log levels
  usually discard; `deliver` returns `false` and the listener ignores it. The
  merchant never receives the payment notification and the operator has no
  trace, no status and no count to see it. Consider a counter of delivered and
  dropped webhooks so a merchant-wide failure shows on a dashboard.
history:
  - status: open
    by: ops
```

#### R-ops-3 Delivery runs inside the payment event publish, holding it up to 25 seconds

```yaml
severity: medium
confidence: low
location: "src/webhooks.ts:33-35"
evidence: |
  if (rows.length > 0) {
    await deliver(rows[0].webhook_url, event);
  }
recommendation: >-
  `publish` in `src/events.ts` awaits each listener in turn, so a merchant
  endpoint that hangs makes this listener take up to five 5-second timeouts
  (25 s, more once backoff is added) before the next listener runs and
  `publish` returns. Whether that stalls the payment service depends on how its
  caller awaits `publish`, which this change does not show. Prefer enqueueing
  the delivery (or not awaiting it, with its own error handling) so a slow
  merchant cannot delay settlement processing.
history:
  - status: open
    by: ops
```

### newcomer

Disabled by configuration.
