# Review record: charge-orders

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### ops

#### R-ops-1 Payment call has no timeout and can hang the checkout forever

```yaml
severity: high
confidence: high
location: "app/payments.py:14"
evidence: |
  response = requests.post(PAYMENT_URL, json={"order": order_id, "amount": amount_cents})
recommendation: >-
  Pass an explicit `timeout=` (connect and read) to `requests.post`. Without it, when the
  provider accepts the connection but stops answering, `charge()` blocks indefinitely: the
  checkout request and its worker hang with no error and no log, and the operator only sees
  stuck workers. Treat a read timeout as an ambiguous outcome (the charge may have gone
  through), see R-ops-3.
history:
  - status: open
    by: ops
```

#### R-ops-2 Unbounded retry loop with fixed one-second delay, including on permanent errors

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
  Bound the retries (a small maximum number of attempts) with exponential backoff and
  jitter, then raise. As written, `raise_for_status()` turns a 4xx (declined card, invalid
  amount, bad credentials) into `requests.HTTPError`, a `RequestException`, so a request
  that can never succeed is retried every second forever: `charge()` never returns, and
  during a provider outage every checkout hammers the provider once a second (retry storm).
  Retry only transient failures (connection errors, 5xx, 429) and surface the others to the
  caller.
history:
  - status: open
    by: ops
```

#### R-ops-3 Retrying a non-idempotent charge can charge the customer twice

```yaml
severity: high
confidence: high
location: "app/payments.py:14-18"
evidence: |
  response = requests.post(PAYMENT_URL, json={"order": order_id, "amount": amount_cents})
recommendation: >-
  Make the retried request idempotent: send an idempotency key derived from `order_id`
  (e.g. an `Idempotency-Key` header, if the provider supports it) so the provider
  deduplicates repeats, or do not retry an ambiguous outcome. Scenario: the provider
  processes the charge but the response is lost (connection reset, 502 from a proxy, or a
  timeout once R-ops-1 is fixed); the `except` branch sleeps and POSTs the same charge
  again, creating a second charge for the same order.
history:
  - status: open
    by: ops
```

#### R-ops-4 Payment failures are swallowed without any log

```yaml
severity: medium
confidence: high
location: "app/payments.py:17-18"
evidence: |
  except requests.RequestException:
      time.sleep(1)
recommendation: >-
  Log each failed attempt with structured fields (order_id, attempt number, HTTP status,
  exception type; never the request body or credentials) and raise an actionable error
  naming the order and the provider status when retries are exhausted. Today, when the
  provider rejects or fails a charge, the exception is dropped: no log, no metric, no error
  reaches the caller, so an operator investigating a stuck or unpaid order has nothing to
  diagnose it with.
history:
  - status: open
    by: ops
```

#### R-ops-5 Provider URL hard-coded instead of read from configuration

```yaml
severity: medium
confidence: high
location: "app/payments.py:7"
evidence: |
  PAYMENT_URL = "https://payments.example.com/v2/charges"
recommendation: >-
  Read the provider URL from the environment or the project's configuration. The endpoint
  differs between environments (provider sandbox vs production); with it baked in, a
  development or staging deployment that calls `charge()` hits the production endpoint,
  and pointing an environment elsewhere requires a code change and a release.
history:
  - status: open
    by: ops
```
