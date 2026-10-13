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
recommendation: Pass an explicit `timeout=` (connect and read, read from configuration) to `requests.post`, so a provider that stops answering fails the call instead of hanging the checkout worker forever.
history:
  - status: open
    by: ops
```

#### R-ops-2 Unbounded retry of a non-idempotent charge, without backoff

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
recommendation: Bound the retries (a small maximum, exponential backoff with jitter), retry only transient failures (connection errors, timeouts, 5xx/429, never a 4xx such as a declined card), and send an idempotency key derived from the order id so a retry after a lost response cannot charge the customer twice; raise once the attempts are exhausted.
history:
  - status: open
    by: ops
```

#### R-ops-3 Charge failures are swallowed with no log or metric

```yaml
severity: high
confidence: high
location: "app/payments.py:17-18"
evidence: |
        except requests.RequestException:
            time.sleep(1)
recommendation: Log each failed attempt as structured fields (order_id, attempt, HTTP status, error type — never the payload's secrets), count attempts and failures in a metric, and surface a final actionable error naming the order when retries are exhausted, so an operator can see a stuck or failing charge.
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
recommendation: Read the provider URL from the environment or configuration, so sandbox and production can be switched, and the endpoint changed, without a new release.
history:
  - status: open
    by: ops
```

#### R-ops-5 No switch to turn automatic charging off

```yaml
severity: medium
confidence: low
location: "app/payments.py:10-11"
evidence: |
  def charge(order_id: str, amount_cents: int) -> dict:
      """Charges the order and returns the provider's receipt."""
recommendation: Guard automatic charging behind a configuration flag so operators can fall back to manual charging in the provider's dashboard without redeploying; if a redeploy of the previous version is the accepted rollback, record it in the plan.
history:
  - status: open
    by: ops
```

### newcomer

Disabled by configuration.
