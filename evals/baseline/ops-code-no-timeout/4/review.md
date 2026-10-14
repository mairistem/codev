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
recommendation: Pass an explicit `timeout=` (connect and read) to `requests.post`. Without it, a provider that accepts the connection but never answers blocks the checkout worker indefinitely; no exception is raised, so the retry loop and any logging never run and the operator sees only stuck requests with no error.
history:
  - status: open
    by: ops
```

#### R-ops-2 Unbounded retry of a non-idempotent charge can duplicate charges and never ends

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
recommendation: Bound the retries (a small maximum attempt count) with exponential backoff and jitter, retry only transient errors (connection errors, timeouts, 5xx/429) and never 4xx such as a declined card, and send an idempotency key derived from `order_id` so a retry after a lost response cannot charge the customer twice. Today, a provider 5xx or a dropped response after the charge succeeded makes the loop POST the charge again every second, and a permanent 4xx (invalid amount, declined) loops forever, hammering the provider at a fixed 1 s rate during an outage and never returning to the caller. Once the attempts are exhausted, raise an error naming the order.
history:
  - status: open
    by: ops
```

#### R-ops-3 Charge failures are swallowed with no log, so an operator cannot see them

```yaml
severity: high
confidence: high
location: "app/payments.py:17-18"
evidence: |
  except requests.RequestException:
      time.sleep(1)
recommendation: Log each failed attempt with structured fields (`order_id`, attempt number, HTTP status or exception type, provider URL host; never the request credentials) and raise an actionable error naming the order when retries are exhausted. As written, when the provider rejects or is unreachable, nothing is logged and nothing is raised; the checkout just spins, and the operator has no error, no order id and no status code to diagnose it in the middle of the night.
history:
  - status: open
    by: ops
```

#### R-ops-4 Production provider URL is hard-coded instead of read from configuration

```yaml
severity: medium
confidence: high
location: "app/payments.py:7"
evidence: |
  PAYMENT_URL = "https://payments.example.com/v2/charges"
recommendation: Read the provider endpoint from the environment or the project configuration (with no default to the production host). With the URL baked in, every environment — local, CI, staging — sends real charges to the production provider, and pointing a deployment at the provider's sandbox or a new endpoint requires a code release.
history:
  - status: open
    by: ops
```

### newcomer

Disabled by configuration.
