# Review record: charge-orders

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### newcomer

Disabled by configuration.

### ops

#### R-ops-1 Unbounded retry of a non-idempotent charge

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
  Bound the retries (a few attempts) with exponential backoff, and retry only
  failures that are safe to repeat: send an idempotency key derived from
  order_id so the provider deduplicates a repeated charge, and do not retry
  4xx responses. When the provider accepts a charge but the response is lost
  (connection reset, read error, 5xx after processing), the loop POSTs again
  every second and charges the customer twice; a definitive 4xx (card
  declined, invalid amount) is retried forever at a fixed 1 s, holding the
  checkout request and hammering the provider. After the last attempt, raise
  an error naming the order_id and the provider status.
history:
  - status: open
    by: ops
```

#### R-ops-2 Payment call has no timeout

```yaml
severity: high
confidence: high
location: "app/payments.py:14"
evidence: |
            response = requests.post(PAYMENT_URL, json={"order": order_id, "amount": amount_cents})
recommendation: >-
  Pass timeout= (connect and read) to requests.post. Without it, a provider
  that accepts the connection but never answers blocks the checkout call
  forever; no RequestException is raised, so even the retry loop never runs,
  and the operator sees hung workers with nothing in the logs.
history:
  - status: open
    by: ops
```

#### R-ops-3 Charge failures are swallowed without a log

```yaml
severity: high
confidence: high
location: "app/payments.py:17-18"
evidence: |
        except requests.RequestException:
            time.sleep(1)
recommendation: >-
  Log each failed attempt as structured fields (order_id, attempt, HTTP status
  or exception type, never the card or credentials) and raise an actionable
  error when retries are exhausted. Today, when the provider is down or
  rejects the charge, the exception is dropped and the loop spins silently:
  an operator at 3 a.m. sees stalled checkouts and no error, metric or log
  line pointing at the payment provider.
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
  Read the provider URL from the environment or configuration. The endpoint
  differs between environments (sandbox vs. production): as written, every
  environment — tests and staging included — charges against the production
  endpoint, and switching it requires a new release.
history:
  - status: open
    by: ops
```
