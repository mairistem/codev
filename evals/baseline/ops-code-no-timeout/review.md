# Review record: charge-orders

<!-- codev-review: 1 -->

## code.review

### ops

#### R-ops-1 Payment call has no timeout and can hang the checkout forever

```yaml
severity: high
confidence: high
location: "app/payments.py:14"
evidence: |
            response = requests.post(PAYMENT_URL, json={"order": order_id, "amount": amount_cents})
recommendation: Pass an explicit `timeout=` (connect and read) to `requests.post`, read from configuration, so a stalled provider fails fast instead of blocking the checkout worker indefinitely.
history:
  - status: open
    by: ops
```

#### R-ops-2 Unbounded retry of a non-idempotent charge can duplicate payments

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
recommendation: Bound the retries (a small maximum count) with exponential backoff and jitter, retry only transient errors (timeouts, connection errors, 5xx/429), and send an idempotency key derived from `order_id` so a retry after a lost response cannot charge the order twice; raise once retries are exhausted.
history:
  - status: open
    by: ops
```

#### R-ops-3 Failures are swallowed silently and never surface to the caller or the logs

```yaml
severity: high
confidence: high
location: "app/payments.py:17-18"
evidence: |
        except requests.RequestException:
            time.sleep(1)
recommendation: "Log each failed attempt as a structured event (order_id, attempt, HTTP status, provider error code — no card or credential data), and raise an actionable exception after the last attempt. Permanent errors such as a 4xx decline must not be retried at all: today a declined card loops forever with no trace."
history:
  - status: open
    by: ops
```

#### R-ops-4 Provider URL is hard-coded

```yaml
severity: low
confidence: high
location: "app/payments.py:7"
evidence: |
  PAYMENT_URL = "https://payments.example.com/v2/charges"
recommendation: Read the endpoint from the environment or configuration so it can be pointed at a sandbox or switched without a release.
history:
  - status: open
    by: ops
```

### newcomer

Disabled by configuration.
