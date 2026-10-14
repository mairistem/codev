# Review record: charge-orders

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### ops

#### R-ops-1 Provider call has no timeout and can hang checkout forever

```yaml
severity: high
confidence: high
location: "app/payments.py:14"
evidence: |
  response = requests.post(PAYMENT_URL, json={"order": order_id, "amount": amount_cents})
recommendation: Pass an explicit `timeout=(connect, read)` to `requests.post` so a provider that accepts the connection but never answers raises `requests.Timeout` instead of blocking the checkout worker indefinitely; without it, a stalled provider freezes every checkout with nothing in the logs.
history:
  - status: open
    by: ops
```

#### R-ops-2 Charge is retried forever, at a fixed 1 s pace, even on permanent errors

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
recommendation: Bound the retries (a small maximum number of attempts) with exponential backoff and jitter, retry only transient failures (connection errors, timeouts, 429/5xx), and raise on 4xx. Today `raise_for_status()` turns a 400/402 (declined card, invalid amount) into an `HTTPError`, a `RequestException`, so `charge()` never returns and hammers the provider once a second; during a provider outage every pending checkout does the same, a retry storm the operator cannot stop short of killing the process.
history:
  - status: open
    by: ops
```

#### R-ops-3 Non-idempotent charge is retried without an idempotency key, risking duplicate charges

```yaml
severity: high
confidence: high
location: "app/payments.py:14"
evidence: |
  response = requests.post(PAYMENT_URL, json={"order": order_id, "amount": amount_cents})
recommendation: >-
  Send an idempotency key derived from the order (for example an `Idempotency-Key: <order_id>` header, or whatever the provider supports) on every attempt, so a retry after a read timeout, a dropped connection or a 5xx returned after the provider already captured the payment does not charge the customer a second time. Until the provider deduplicates, retry only when the request is known not to have reached it (connect errors).
history:
  - status: open
    by: ops
```

#### R-ops-4 Payment failures are swallowed with no log

```yaml
severity: medium
confidence: high
location: "app/payments.py:17-18"
evidence: |
  except requests.RequestException:
      time.sleep(1)
recommendation: >-
  Log each failed attempt with structured fields (`order_id`, attempt number, HTTP status, provider error code, exception type — never the payload's secrets), and log/raise a final error naming the order once retries are exhausted. As written, when the provider rejects or is down, the operator sees no error, no metric and no order id: only a checkout that never completes.
history:
  - status: open
    by: ops
```

#### R-ops-5 Provider URL is hard-coded instead of read from configuration

```yaml
severity: medium
confidence: high
location: "app/payments.py:7"
evidence: |
  PAYMENT_URL = "https://payments.example.com/v2/charges"
recommendation: Read the provider URL from the environment or the project's configuration (e.g. `PAYMENT_URL = os.environ["PAYMENT_URL"]`). The task was verified against the provider's sandbox, yet the code points at a single fixed host, so test or staging environments either hit production and charge real money, or need a code change to switch hosts.
history:
  - status: open
    by: ops
```

### newcomer

Disabled by configuration.
