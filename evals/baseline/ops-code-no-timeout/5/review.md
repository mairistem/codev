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
recommendation: Pass an explicit `timeout=` (connect and read) to `requests.post`, so a provider that accepts the connection and never answers fails the call instead of holding the checkout worker forever; the timeout must then be handled by the bounded retry of R-ops-2.
history:
  - status: open
    by: ops
```

Scenario: the provider accepts the TCP connection but stalls. `requests` without `timeout=` waits indefinitely, so the checkout request hangs with no error, no log and no exception — the operator sees workers piling up on the checkout path and nothing that names the provider.

#### R-ops-2 Charge is retried forever, without backoff, on a non-idempotent POST

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
recommendation: Bound the retries (a few attempts) with exponential backoff and jitter, retry only errors that are safe to retry (connection errors, 5xx/429 — not 4xx such as a declined card or a bad request, which `raise_for_status` also turns into a `RequestException`), and send an idempotency key derived from `order_id` so a retried POST cannot charge the order twice; after the last attempt, raise an error naming the order and the provider's status.
history:
  - status: open
    by: ops
```

Scenario: the provider charges the card but the response is lost (connection reset, 502 from a proxy). The loop POSTs again with no idempotency key and the customer is charged twice, every second, until a call succeeds. A 4xx such as a declined card or an invalid amount never succeeds, so `charge()` never returns, and during a provider outage every checkout hammers the provider once a second with no limit — a retry storm.

#### R-ops-3 Provider errors are swallowed without a log

```yaml
severity: medium
confidence: high
location: "app/payments.py:17-18"
evidence: |
  except requests.RequestException:
      time.sleep(1)
recommendation: Log each failed attempt as a structured event — fields such as `order_id`, attempt number, HTTP status and exception class, without the request body or any credential — and log or raise the final failure, so an operator can see which orders fail and why.
history:
  - status: open
    by: ops
```

Scenario: the provider returns 401 after a credential rotation. Every attempt raises `HTTPError`, which is caught and dropped; nothing is logged, so the operator has no trace of the failing provider, the status code or the affected orders, only stuck checkouts.

#### R-ops-4 Payment provider URL is hard-coded

```yaml
severity: medium
confidence: high
location: "app/payments.py:7"
evidence: |
  PAYMENT_URL = "https://payments.example.com/v2/charges"
recommendation: Read the provider URL from the environment or configuration (for example `PAYMENT_URL = os.environ["PAYMENT_URL"]`), so the sandbox the task verifies against and production use the same code.
history:
  - status: open
    by: ops
```

Scenario: task 1.1 is verified against the provider's sandbox, but the only endpoint in the code is the production one. Running in staging or a test environment sends real charges to the production provider, and pointing an environment at the sandbox requires a code change and a release.

### newcomer

Disabled by configuration.
