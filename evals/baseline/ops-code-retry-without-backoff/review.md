# Review record: add-merchant-webhooks

<!-- codev-review: 1 -->

## code.review

### ops

#### R-ops-1 Webhook retries fire back-to-back with no backoff

```yaml
severity: medium
confidence: high
location: "src/webhooks.ts:9-11"
evidence: |
  for (let attempt = 1; attempt <= MAX_ATTEMPTS; attempt++) {
    try {
      const response = await fetch(url, {
recommendation: >-
  Space the attempts with exponential backoff and jitter (e.g. 1s, 4s, 16s…),
  ideally from a persisted retry queue rather than an in-process loop. Today a
  merchant endpoint that answers 503 during a few-second restart receives five
  requests within milliseconds, all fail, and the event is dropped: the
  "retried, up to five attempts" promise of the proposal buys no resilience,
  and a struggling merchant is hammered instead of given time to recover.
  While there, do not retry non-retryable answers (4xx other than 408/429):
  they fail identically five times.
history:
  - status: open
    by: ops
```

#### R-ops-2 A dropped webhook is invisible to operators

```yaml
severity: high
confidence: high
location: "src/webhooks.ts:24"
evidence: |
  console.debug(`webhook ${event.id} dropped after ${MAX_ATTEMPTS} attempts`);
  return false;
recommendation: >-
  When a merchant endpoint keeps failing, the payment notification is lost and
  the only trace is a free-text `console.debug` line, which production log
  levels usually discard; a non-2xx response is not logged at all, so its
  status code never appears, and the `false` returned by `deliver` is ignored
  by the listener. Use the project's structured `logger` (src/logger.ts):
  log each failed attempt at warn with fields (eventId, merchant/payment id,
  attempt, HTTP status or error), log the final drop at error, and emit a
  metric/counter for delivered vs dropped webhooks so an alert can fire. Never
  log the full webhook URL if it may embed a merchant secret.
history:
  - status: open
    by: ops
```

#### R-ops-3 Webhook delivery blocks the payment event publisher for up to 25 s

```yaml
severity: medium
confidence: low
location: "src/webhooks.ts:34"
evidence: |
  await deliver(rows[0].webhook_url, event);
recommendation: >-
  `publish` in src/events.ts awaits each listener in turn, so a merchant
  endpoint that times out holds the payment service's `publish` call for
  5 attempts × 5 s = 25 s, and delays every later listener. Whether the
  payment service awaits `publish` on its request path was not verified.
  Hand delivery off to a background queue/worker (which also gives R-ops-1 its
  backoff) instead of awaiting it inside the listener.
history:
  - status: open
    by: ops
```

### newcomer

Disabled by configuration.
