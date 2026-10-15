# Review record: charge-orders

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### newcomer

Disabled by configuration.

### ops

#### R-ops-1 Unbounded retry of a non-idempotent charge can bill an order several times

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
  Bound the retries (a few attempts with exponential backoff and jitter) and
  retry only errors that are safe to repeat: connection errors before the
  request was sent, 429 and 5xx — never 4xx such as a declined card or a bad
  amount, which today loop forever. Make the POST idempotent by sending an
  idempotency key derived from the order (e.g. an `Idempotency-Key: <order_id>`
  header, if the provider supports it) so a retry after a read timeout or a 5xx
  that was in fact processed does not create a second charge. After the last
  attempt, raise an error naming the order and the provider status.
history:
  - status: open
    by: ops
```

Scenario: the provider processes the charge but the response is lost (connection reset, 502 from a proxy). `requests.RequestException` is caught, the loop sleeps one second and POSTs again with no idempotency key: the customer is charged twice, and keeps being charged until a call succeeds. During a provider outage or for a permanent 4xx (declined card, invalid amount), the call never returns and the checkout request hangs forever, with retries every second hammering the provider.

#### R-ops-2 The payment call has no timeout and can hang the checkout path

```yaml
severity: high
confidence: high
location: "app/payments.py:14"
evidence: |
            response = requests.post(PAYMENT_URL, json={"order": order_id, "amount": amount_cents})
recommendation: >-
  Pass an explicit `timeout=` (connect and read, e.g. `timeout=(3, 10)`) to
  `requests.post`, and treat a read timeout as an outcome of unknown state —
  retried only with the idempotency key of R-ops-1.
history:
  - status: open
    by: ops
```

Scenario: the provider accepts the TCP connection but never answers (overload, half-open connection). `requests` without `timeout=` waits indefinitely, so the worker serving the checkout blocks forever; under load all workers are exhausted and checkout goes down, with no error raised anywhere for the operator to see.

#### R-ops-3 Payment failures are swallowed without any log

```yaml
severity: high
confidence: high
location: "app/payments.py:17-18"
evidence: |
        except requests.RequestException:
            time.sleep(1)
recommendation: >-
  Log each failed attempt with structured fields — `order_id`, attempt number,
  HTTP status or exception class, provider error code — at warning level, and
  at error level when giving up; never log the request body beyond the order
  and amount. Consider a counter metric of charge attempts and failures by
  outcome so a provider outage is visible on a dashboard.
history:
  - status: open
    by: ops
```

Scenario: the provider starts returning 401 after a credential rotation, or 5xx during an outage. Every exception is caught and dropped; nothing is logged and nothing is raised, so the operator sees only stuck checkouts with no message, order id or status code to diagnose from.

#### R-ops-4 The provider URL is hard-coded instead of read from configuration

```yaml
severity: medium
confidence: high
location: "app/payments.py:7"
evidence: |
  PAYMENT_URL = "https://payments.example.com/v2/charges"
recommendation: >-
  Read the provider URL from the environment or the application's
  configuration (e.g. `PAYMENT_URL = os.environ["PAYMENT_URL"]`), so staging
  and tests target the sandbox and production the live endpoint.
history:
  - status: open
    by: ops
```

Scenario: task 1.1 is verified against the provider's sandbox, yet the code only knows one URL. Either the sandbox verification used a locally edited constant, or staging and tests hit the live endpoint and create real charges; switching environments or provider endpoints requires a code change and a release.
