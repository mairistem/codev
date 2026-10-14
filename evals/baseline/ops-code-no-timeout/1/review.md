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
recommendation: Pass an explicit `timeout=` (connect and read) to `requests.post`, so that a provider that accepts the connection but never answers raises `requests.Timeout` instead of blocking the checkout worker indefinitely; handle that timeout as an outcome of unknown state (see R-ops-3).
history:
  - status: open
    by: ops
```

When the payment provider stalls (accepts the TCP connection, never responds), `requests` waits with no limit: the checkout request and its worker hang, and since nothing is logged the operator sees only workers piling up with no error.

#### R-ops-2 Unbounded retry loop with a fixed 1 s sleep, also on non-retryable errors

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
recommendation: Bound the retries (a small maximum attempt count or total deadline), space them with exponential backoff and jitter, retry only transient failures (connection errors, timeouts, 429/5xx) and raise immediately on 4xx such as a declined card or a bad request, then raise an actionable error once retries are exhausted.
history:
  - status: open
    by: ops
```

`raise_for_status()` raises `HTTPError` (a `RequestException`) for a 400/402 response, e.g. a declined card or an invalid amount: the loop retries it every second forever, so `charge()` never returns. During a provider outage every in-flight checkout loops at a fixed 1 s cadence with no backoff, hammering the provider as it recovers (retry storm).

#### R-ops-3 Retrying a non-idempotent charge POST without an idempotency key can charge an order twice

```yaml
severity: high
confidence: high
location: "app/payments.py:14"
evidence: |
  response = requests.post(PAYMENT_URL, json={"order": order_id, "amount": amount_cents})
recommendation: Send an idempotency key derived from the order (e.g. an `Idempotency-Key` header built from `order_id`, if the provider supports it) on every attempt so the provider deduplicates retries; without one, do not retry after a request may have reached the provider (read timeout, 5xx, connection reset after send) — surface it as an unknown-state failure to reconcile instead.
history:
  - status: open
    by: ops
```

If the provider records the charge but the response is lost (connection reset, 502 from a proxy, a read timeout once R-ops-1 adds one), the `except` branch posts the same charge again: the customer is billed twice and no record shows it.

#### R-ops-4 Payment failures are swallowed without any log

```yaml
severity: high
confidence: high
location: "app/payments.py:17-18"
evidence: |
  except requests.RequestException:
      time.sleep(1)
recommendation: Log each failed attempt with structured fields (order_id, attempt, HTTP status or exception class, provider request id when present — never card data or credentials), and raise an error naming the order and the last cause when giving up, so an operator can see and act on payment failures.
history:
  - status: open
    by: ops
```

When the provider is down, rejects the API call, or returns 4xx, the exception is discarded: no log line, no metric, no error reaches the caller. At 3 a.m. the operator sees checkouts stuck and has nothing that says the payment provider is the cause or which orders are affected.

#### R-ops-5 Production payment URL hard-coded instead of read from configuration

```yaml
severity: medium
confidence: low
location: "app/payments.py:7"
evidence: |
  PAYMENT_URL = "https://payments.example.com/v2/charges"
recommendation: Read the provider endpoint from the environment or the project configuration (e.g. `PAYMENT_URL` env var), so staging and tests can target the provider's sandbox and an operator can repoint it without a release.
history:
  - status: open
    by: ops
```

Every environment, staging and local runs included, would post charges to the same endpoint; if it is the live provider, a test checkout charges a real order. Confidence is low: whether the provider exposes a separate sandbox endpoint could not be verified from the repository.

### newcomer

Disabled by configuration.
