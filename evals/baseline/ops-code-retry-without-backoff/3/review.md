# Review record: add-merchant-webhooks

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### ops

#### R-ops-1 Webhook retries fire back-to-back with no backoff

```yaml
severity: medium
confidence: high
location: "src/webhooks.ts:9-23"
evidence: |
  for (let attempt = 1; attempt <= MAX_ATTEMPTS; attempt++) {
    try {
      const response = await fetch(url, {
recommendation: >-
  Wait between attempts with an exponential backoff and jitter (for example
  1 s, 2 s, 4 s, 8 s, capped), and do not retry responses that will not change
  on a retry (4xx other than 408/429). Scenario: a merchant endpoint returns
  503 during a 10-second restart; each non-ok response loops straight into the
  next attempt, so all five POSTs are spent within the outage and the event is
  dropped, while a merchant under load receives five requests in a burst. The
  retries are bounded and deduplicated by `x-event-id`, but the step's
  criterion requires them to be spaced.
history:
  - status: open
    by: ops
```

#### R-ops-2 A dropped webhook is only logged at debug level, as free text

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
  Log the final drop at error (and each failed attempt at warn) through the
  service's structured logger, with fields such as `eventId`, `paymentId`,
  merchant id, `attempt`, the HTTP status or error, and the outcome, and count
  drops in a metric an alert can watch. Scenario: a merchant's endpoint is down
  for an hour; every settlement notification is permanently lost, but the only
  trace is a `console.debug` line, which production log levels usually discard,
  so neither the operator nor the merchant learns the payment was never
  notified. Non-ok responses (lines 17-19) leave no trace at all: the status
  code a merchant returned is never logged, so even with debug on an operator
  cannot tell a 500 from a 404. `deliver` returns `false` but the caller at
  line 34 ignores it, so the drop is silent end to end.
history:
  - status: open
    by: ops
```

### newcomer

Disabled by configuration.
