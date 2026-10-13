# Review record: charge-orders

<!-- codev-review: 1 -->

## code.review

### ops

#### R-ops-1 Payment call has no timeout

```yaml
severity: high
confidence: high
location: "app/payments.py:14"
evidence: |
  response = requests.post(PAYMENT_URL, json={"order": order_id, "amount": amount_cents})
recommendation: >-
  Pass an explicit timeout to requests.post (e.g. timeout=(3.05, 10)). Without
  one, requests waits forever: when the provider accepts the connection but
  stops answering, the checkout request hangs indefinitely, holding its worker,
  and no error or log line ever tells the operator why checkouts stall.
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
recommendation: >-
  Bound the retries (a few attempts with exponential backoff and jitter), retry
  only transient errors (connection errors, timeouts, 5xx/429) and never 4xx
  such as a declined card, and send an idempotency key derived from order_id so
  a retried POST cannot charge twice; raise once retries are exhausted.
  Today a response lost after the provider charged (read error, 5xx from a
  proxy) is retried and charges the customer again; a permanent error (400,
  401, 402) loops forever at one request per second per order, hammering the
  provider and never returning to the caller.
history:
  - status: open
    by: ops
```

#### R-ops-3 Payment failures are swallowed without a log

```yaml
severity: medium
confidence: high
location: "app/payments.py:17-18"
evidence: |
  except requests.RequestException:
      time.sleep(1)
recommendation: >-
  Log each failed attempt with structured fields (order_id, attempt, HTTP
  status, provider error code — never the payload's card data or credentials)
  and raise an actionable error naming the order once retries are exhausted.
  Today, when the provider is down or rejects the charge, the operator sees no
  log, no error and no metric: only stuck checkouts.
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
recommendation: >-
  Read the provider URL from the environment or configuration (e.g.
  PAYMENT_URL env var). The task was verified against the provider's sandbox,
  so the URL differs between environments: as written, every environment —
  dev, staging, tests — sends real charges to the production endpoint, or
  production must be edited and redeployed to switch provider host.
history:
  - status: open
    by: ops
```

### newcomer

Disabled by configuration.
