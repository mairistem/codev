# Review record: add-merchant-webhooks

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### ops

#### R-ops-1 A dropped webhook is logged only at debug level, in free text

```yaml
severity: high
confidence: high
location: "src/webhooks.ts:24-25"
evidence: |
  console.debug(`webhook ${event.id} dropped after ${MAX_ATTEMPTS} attempts`);
  return false;
recommendation: >-
  When a merchant's endpoint is down or answers 5xx for all five attempts, the
  event is dropped and the only trace is a `console.debug` prose line, which
  production log levels typically discard and which the log platform cannot
  query; the listener also ignores the `false` returned by `deliver`, so the
  merchant silently never learns the payment settled and no operator is
  alerted. A non-ok response (`response.ok` false) is not logged at all, so its
  status code is lost. Use the project's structured `logger` from
  `src/logger.ts`: `logger.warn` per failed attempt with fields `eventId`,
  `attempt`, `status` (or `error: String(error)`), and `logger.error` when the
  event is dropped, with `eventId`, `paymentId` and `attempts`. Log the
  merchant host rather than the full URL, in case it carries a token.
history:
  - status: open
    by: ops
```

#### R-ops-2 Failed deliveries are retried immediately, with no backoff

```yaml
severity: medium
confidence: high
location: "src/webhooks.ts:9-23"
evidence: |
  for (let attempt = 1; attempt <= MAX_ATTEMPTS; attempt++) {
    try {
      const response = await fetch(url, {
recommendation: >-
  The loop starts the next attempt as soon as the previous one fails: when a
  merchant's server is overloaded or restarting and answers 503, the five POSTs
  hit it back to back within milliseconds, add load at the worst moment, and
  all fail inside the same outage window, so the event is dropped even though a
  retry a few seconds later would have succeeded. Space the attempts with
  exponential backoff and jitter (for example 1 s, 2 s, 4 s, 8 s), and retry
  only on network errors, 429 and 5xx, not on other 4xx. Because `publish` in
  `src/events.ts` awaits each listener in turn, sleeping inside this loop would
  hold the payment service's `publish` call; run the retries off that path (a
  queue or a scheduled retry) rather than awaiting them in the listener.
history:
  - status: open
    by: ops
```

### newcomer

Disabled by configuration.
