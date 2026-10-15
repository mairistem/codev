# Review record: charge-orders

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### ops

#### R-ops-1 Payment call has no timeout and can hang checkout forever

```yaml
severity: high
confidence: high
location: "app/payments.py:14"
evidence: |
  response = requests.post(PAYMENT_URL, json={"order": order_id, "amount": amount_cents})
recommendation: >-
  Pass an explicit `timeout=` (connect and read, e.g. `timeout=(3, 10)`) to
  `requests.post`. Without it, when the payment provider accepts the
  connection but stops answering, `charge()` blocks indefinitely on the
  checkout path: the worker or request thread hangs, nothing is logged, and
  the operator only sees checkouts piling up with no error.
history:
  - status: open
    by: ops
```

#### R-ops-2 Unbounded retry of a non-idempotent charge can duplicate charges and loop forever

```yaml
severity: high
confidence: high
location: "app/payments.py:12-18"
evidence: |
      while True:
          try:
              response = requests.post(PAYMENT_URL, json={"order": order_id, "amount": amount_cents})
              response.raise_for_status()
              return response.json()
          except requests.RequestException:
              time.sleep(1)
recommendation: >-
  Bound the retries (a small maximum number of attempts) with exponential
  backoff, and retry only when the charge is safe to repeat: send an
  idempotency key derived from `order_id` that the provider deduplicates on,
  and retry only on connection errors, timeouts and 5xx — never on 4xx.
  Failure scenarios: (1) the provider charges the card but the response is
  lost (read error, or a 5xx after commit) — the loop POSTs again and the
  customer is charged twice, every second, until a call succeeds;
  (2) the provider rejects the charge with a 4xx (card declined, invalid
  amount) — `raise_for_status()` raises `HTTPError`, a `RequestException`,
  so the loop retries a request that can never succeed, forever, and
  `charge()` never returns; (3) during a provider outage every caller retries
  once a second with no limit, a retry storm against a recovering provider.
  After the last attempt, raise an error naming the order so the caller can
  fail the checkout.
history:
  - status: open
    by: ops
```

#### R-ops-3 Charge failures are swallowed without any log or actionable error

```yaml
severity: medium
confidence: high
location: "app/payments.py:17-18"
evidence: |
          except requests.RequestException:
              time.sleep(1)
recommendation: >-
  Log each failed attempt with structured fields — `order_id`, attempt
  number, HTTP status or exception type, provider error code — without the
  request body or any credential, and when attempts are exhausted raise an
  error that names the order and the provider's response. Today, when the
  provider returns errors or is unreachable, the exception is discarded and
  nothing is emitted: an operator sees no log, no error and no metric, and
  cannot tell which orders are stuck or why.
history:
  - status: open
    by: ops
```

#### R-ops-4 Payment provider URL is hard-coded to production

```yaml
severity: medium
confidence: high
location: "app/payments.py:7"
evidence: |
  PAYMENT_URL = "https://payments.example.com/v2/charges"
recommendation: >-
  Read the provider URL from the environment or configuration (e.g. a
  `PAYMENT_URL` environment variable, failing at start if absent). The URL
  differs between environments — task 1.1 is verified against the
  provider's sandbox — so with the production host baked in, a test or
  staging deployment sends real charges to the live provider, and pointing
  an environment at the sandbox requires a code change and a release.
history:
  - status: open
    by: ops
```

### newcomer

Disabled by configuration.
