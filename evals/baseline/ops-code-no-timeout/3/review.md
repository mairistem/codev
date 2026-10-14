# Review record: charge-orders

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### ops

#### R-ops-1 The provider call has no timeout

```yaml
severity: high
confidence: high
location: "app/payments.py:14"
evidence: |
  response = requests.post(PAYMENT_URL, json={"order": order_id, "amount": amount_cents})
recommendation: >-
  Pass an explicit `timeout=` (connect and read) to `requests.post`. Without
  it, when the payment provider accepts the connection but stops answering,
  `charge()` blocks forever on the checkout path: the request worker hangs,
  nothing is logged and the operator only sees checkouts piling up.
history:
  - status: open
    by: ops
```

#### R-ops-2 Unbounded retry of a non-idempotent charge can duplicate charges

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
  Bound the retries (a few attempts, exponential backoff with jitter), then
  raise an error naming the order. Retry only when the charge is safe to
  repeat: send an idempotency key derived from `order_id` that the provider
  honours, and do not retry on 4xx responses. Today, when the provider
  charges the card but the response is lost (read error, 5xx after
  processing), the loop posts the charge again every second and charges the
  customer twice; when the provider is down or rejects the request (e.g. a
  400 for an invalid amount), the loop never ends and hammers the provider at
  1 request per second per pending order.
history:
  - status: open
    by: ops
```

#### R-ops-3 Failed charge attempts are swallowed without a log

```yaml
severity: medium
confidence: high
location: "app/payments.py:17-18"
evidence: |
  except requests.RequestException:
      time.sleep(1)
recommendation: >-
  Log each failed attempt as a structured event (order id, attempt number,
  HTTP status, error class, no request body or credentials) and emit a
  metric for charge failures. Today, when the provider returns errors, the
  exception is dropped: an operator investigating a stuck checkout has no
  log, no status code and no order id to diagnose from.
history:
  - status: open
    by: ops
```

#### R-ops-4 Provider URL is hard-coded

```yaml
severity: medium
confidence: high
location: "app/payments.py:7"
evidence: |
  PAYMENT_URL = "https://payments.example.com/v2/charges"
recommendation: >-
  Read the provider URL from the environment or configuration (e.g.
  `PAYMENT_URL` env var), with no production default for non-production
  environments. The task is verified against the provider's sandbox, yet the
  code only knows the production endpoint: pointing a test or staging
  environment at the sandbox needs a code edit, and an environment run as is
  sends real charges to production.
history:
  - status: open
    by: ops
```

### newcomer

Disabled by configuration.
