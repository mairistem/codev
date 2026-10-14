# Review record: charge-orders

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### ops

#### R-ops-1 Unbounded retry of a non-idempotent charge can bill an order several times

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
  Bound the retries (a few attempts with exponential backoff), retry only
  errors where the charge certainly did not happen (connection refused, 429,
  503) and send an idempotency key derived from the order id so the provider
  deduplicates a repeated POST; after the last attempt, raise an error naming
  the order. Scenario: the provider accepts the charge but the response is
  lost (read error) or a 5xx comes back after processing — the loop POSTs
  again every second and the customer is charged twice or more; when the
  provider is down or rejects the request permanently (400, 401, 402),
  checkout spins forever at one request per second against the provider.
history:
  - status: open
    by: ops
```

#### R-ops-2 Charge call has no timeout and can hang checkout forever

```yaml
severity: high
confidence: high
location: "app/payments.py:14"
evidence: |
  response = requests.post(PAYMENT_URL, json={"order": order_id, "amount": amount_cents})
recommendation: >-
  Pass `timeout=(connect, read)` to `requests.post`, e.g. `timeout=(3, 10)`.
  Scenario: the provider accepts the TCP connection but never answers —
  `requests` waits indefinitely, the checkout worker is stuck with no error
  and no log, and an operator sees only hung requests. Note that a read
  timeout on a POST is ambiguous (the charge may have happened), so it must
  only be retried with the idempotency key of R-ops-1.
history:
  - status: open
    by: ops
```

#### R-ops-3 Charge failures are swallowed with no log

```yaml
severity: medium
confidence: high
location: "app/payments.py:17-18"
evidence: |
  except requests.RequestException:
      time.sleep(1)
recommendation: >-
  Log each failed attempt with structured fields — order_id, attempt, HTTP
  status, provider error code, exception type — never the request body or
  credentials, and log the final failure at error level. Scenario: the
  provider returns 402 or 500 for an order — nothing is logged, so an
  operator at 3 a.m. cannot tell which order is failing, why, or that the
  provider is failing at all.
history:
  - status: open
    by: ops
```

#### R-ops-4 Provider URL is hard-coded instead of read from configuration

```yaml
severity: medium
confidence: high
location: "app/payments.py:7"
evidence: |
  PAYMENT_URL = "https://payments.example.com/v2/charges"
recommendation: >-
  Read the provider URL (and its credentials, when added) from the
  environment or the application's configuration, failing at start-up when it
  is missing. Scenario: tasks.md verifies the charge against the provider's
  sandbox, but the code always targets the production endpoint — a test or
  staging environment either charges real orders, or needs a code edit to
  point at the sandbox.
history:
  - status: open
    by: ops
```

### newcomer

Disabled by configuration.
