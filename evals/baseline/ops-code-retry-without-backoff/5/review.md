# Review record: add-merchant-webhooks

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### ops

#### R-ops-1 Failed deliveries are retried back-to-back, with no backoff

```yaml
severity: medium
confidence: high
location: "src/webhooks.ts:9-23"
evidence: |
  for (let attempt = 1; attempt <= MAX_ATTEMPTS; attempt++) {
recommendation: >-
  When a merchant's endpoint returns a 5xx or refuses connections, the loop
  fires the next attempt immediately: the five POSTs land back-to-back, so a
  merchant that is briefly down or overloaded is hammered, a few seconds of
  outage exhausts every attempt and the event is dropped, and a burst of
  payments turns into a retry storm against the merchant's server. Wait
  between attempts with exponential backoff and jitter (for example 1s, 2s,
  4s, 8s), or hand failed deliveries to a delayed retry queue so the wait does
  not hold the payment listener. Being bounded and idempotent (x-event-id) is
  not enough: the attempts must also be spaced.
history:
  - status: open
    by: ops
```

#### R-ops-2 Dropped webhooks are logged only at debug level, in free text

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
  When a merchant's endpoint stays down, the event is given up after five
  attempts and the only trace is a `console.debug` line that bypasses the
  project's structured logger (src/logger.ts, shipped to the log platform) and
  is usually filtered out in production; the listener also ignores `deliver`
  returning false. Lost payment notifications go unnoticed and a merchant's
  report of a missing webhook cannot be diagnosed. Use `logger.warn` per failed
  attempt and `logger.error` when the event is dropped, with fields such as
  `eventId`, `paymentId`, `attempt`, `status` and `error` (not the full
  webhook URL if it may carry a merchant secret), and act on the false return
  in the listener.
history:
  - status: open
    by: ops
```

#### R-ops-3 Non-2xx responses are retried without recording their status

```yaml
severity: medium
confidence: high
location: "src/webhooks.ts:17-19"
evidence: |
  if (response.ok) {
    return true;
  }
recommendation: >-
  When a merchant's endpoint answers 404, 401 or 500, nothing is logged for
  that attempt (the catch only runs on network errors and timeouts) and the
  final "dropped" line carries no status: an operator cannot tell a
  misconfigured URL from a merchant outage, and permanent client errors burn
  all five attempts. On a non-ok response, log the attempt with
  `response.status`, and retry only statuses worth retrying (408, 429, 5xx),
  giving up at once on other 4xx.
history:
  - status: open
    by: ops
```

### newcomer

Disabled by configuration.
