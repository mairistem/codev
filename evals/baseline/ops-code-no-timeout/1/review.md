# Review record: charge-orders

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### ops

#### R-ops-1 Payment call has no timeout
```yaml
severity: high
confidence: high
location: "app/payments.py:14"
evidence: |
  response = requests.post(PAYMENT_URL, json={"order": order_id, "amount": amount_cents})
recommendation: Pass an explicit `timeout=` (connect and read) to `requests.post`. Without it, when the provider accepts the connection but stops answering, `charge()` blocks forever on the checkout path; the operator sees stuck checkouts and worker threads, with no error and no log to point at the provider.
history:
  - status: open
    by: ops
```

#### R-ops-2 Unbounded retry of a non-idempotent charge
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
recommendation: Bound the retries (a small maximum, with exponential backoff) and make the POST idempotent before retrying it — send an idempotency key derived from `order_id` if the provider supports one, otherwise do not retry once the request may have reached the provider. As written, a 4xx (declined card, bad amount) is retried every second forever, hammering the provider; and a 5xx or a dropped connection after the provider recorded the charge triggers another POST that charges the customer twice. Re-raise once the retries are exhausted.
history:
  - status: open
    by: ops
```

#### R-ops-3 Payment errors are swallowed without a log
```yaml
severity: medium
confidence: high
location: "app/payments.py:17-18"
evidence: |
  except requests.RequestException:
              time.sleep(1)
recommendation: Log each failed attempt with structured fields — `order_id`, attempt number, HTTP status, exception type — and never the request body beyond those fields. Today, when the provider fails or rejects a charge, nothing is logged and no error surfaces, so the operator cannot tell which orders are failing nor why.
history:
  - status: open
    by: ops
```

#### R-ops-4 Provider URL hard-coded
```yaml
severity: medium
confidence: high
location: "app/payments.py:7"
evidence: |
  PAYMENT_URL = "https://payments.example.com/v2/charges"
recommendation: Read the provider URL from the environment or configuration. The task was verified against the provider's sandbox, which differs from production; with the URL baked in, pointing an environment at the sandbox or at production requires a code change, and a test or staging deploy can send real charges.
history:
  - status: open
    by: ops
```

### newcomer

Disabled by configuration.
