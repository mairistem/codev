# Review record: charge-orders

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### ops

#### R-ops-1 Charge is retried forever, without backoff, on a non-idempotent call

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
  Bound the retries (a few attempts) with exponential backoff and jitter, retry only
  transient errors (connection errors, timeouts, 5xx/429) and never 4xx such as a
  declined card, and send an idempotency key derived from order_id so the provider
  deduplicates a retried charge. After the last attempt, raise an error naming the
  order and the provider status. Scenario: the provider charges the card but the
  response is lost or returns 502, so the loop posts the same charge again and the
  customer is charged twice; a 400/402 is retried every second forever, pinning the
  checkout worker and hammering the provider during an outage.
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
  Pass an explicit timeout, e.g. timeout=(3, 10) for connect and read. Scenario: the
  provider accepts the connection but never answers; requests waits indefinitely,
  the checkout request hangs with no error and no log, and the worker is lost until
  restarted. Combined with R-ops-1, a read timeout must not lead to a blind re-charge:
  rely on the idempotency key.
history:
  - status: open
    by: ops
```

#### R-ops-3 Payment failures are swallowed without any log

```yaml
severity: medium
confidence: high
location: "app/payments.py:17-18"
evidence: |
        except requests.RequestException:
            time.sleep(1)
recommendation: >-
  Log every failed attempt with structured fields (order_id, attempt, HTTP status,
  exception class, provider error code — never card data or credentials) and the
  final failure at error level. Scenario: the provider is down or rejects the API
  call; the operator sees only stuck checkouts, with nothing in the logs to say that
  the payment provider is failing or for which orders.
history:
  - status: open
    by: ops
```

#### R-ops-4 Provider URL hard-coded instead of read from configuration

```yaml
severity: medium
confidence: low
location: "app/payments.py:7"
evidence: |
  PAYMENT_URL = "https://payments.example.com/v2/charges"
recommendation: >-
  Read the provider URL (and any credentials added later) from the environment or the
  application configuration, so staging and tests can point at the provider's sandbox.
  Scenario: if the provider has a sandbox endpoint, every environment, staging
  included, charges against the production endpoint. Confidence low: whether the
  provider uses distinct endpoints per environment could not be verified.
history:
  - status: open
    by: ops
```

### newcomer

Disabled by configuration.
