# Review record: charge-orders

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### ops

#### R-ops-1 Charge call to the payment provider has no timeout

```yaml
severity: high
confidence: high
location: "app/payments.py:14"
evidence: |
  response = requests.post(PAYMENT_URL, json={"order": order_id, "amount": amount_cents})
recommendation: >-
  Pass an explicit `timeout=` (connect and read) to `requests.post`. Without it,
  when the provider accepts the connection but never answers, `requests` waits
  indefinitely: the checkout request that calls `charge()` hangs, no exception
  is raised, nothing is logged, and the operator sees stuck workers with no
  error to diagnose.
history:
  - status: open
    by: ops
```

#### R-ops-2 Unbounded retry of a non-idempotent charge, on every error

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
  Bound the retries (a few attempts, with exponential backoff and jitter) and
  retry only transient failures (connection errors, 5xx, 429), never 4xx such
  as a declined card or invalid amount, which today loops forever every second.
  Make the POST safe to repeat by sending an idempotency key derived from
  `order_id` (if the provider supports one); otherwise a read timeout or 5xx
  returned after the provider already charged makes the retry charge the
  customer twice. During a provider outage, every pending checkout retries
  every second with no limit, a retry storm that also blocks callers forever.
  After the last attempt, raise an error carrying the order id and the
  provider's status.
history:
  - status: open
    by: ops
```

#### R-ops-3 Charge failures are swallowed without any log

```yaml
severity: high
confidence: high
location: "app/payments.py:17-18"
evidence: |
  except requests.RequestException:
      time.sleep(1)
recommendation: >-
  Log each failed attempt as a structured event (order id, attempt number,
  HTTP status or exception type, never the request body or credentials) and
  expose a counter of charge failures. Today, when the provider is down or
  rejects a charge, the exception is dropped: the operator has no log line, no
  metric and no error, and cannot tell a slow checkout from a provider outage
  or a declined order.
history:
  - status: open
    by: ops
```

#### R-ops-4 Provider URL hard-coded instead of read from configuration

```yaml
severity: medium
confidence: high
location: "app/payments.py:7"
evidence: |
  PAYMENT_URL = "https://payments.example.com/v2/charges"
recommendation: >-
  Read the provider URL from the environment or the application
  configuration. The task was verified against the provider's sandbox, while
  production must call the live endpoint: with the URL baked in, switching
  between sandbox and production, or pointing a test environment away from
  real charges, needs a code change and a release.
history:
  - status: open
    by: ops
```

### newcomer

Disabled by configuration.
