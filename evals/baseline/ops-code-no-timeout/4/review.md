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
recommendation: Pass an explicit `timeout=` (connect and read) to `requests.post`, so a provider that accepts the connection but never answers raises `requests.Timeout` instead of blocking the checkout worker forever; handle that timeout as an outcome of unknown status (see R-ops-2), not as a plain failure.
history:
  - status: open
    by: ops
```

Scenario: the provider accepts the TCP connection and stalls (overload, network partition). `requests` waits indefinitely, the checkout request hangs with no log and no error, and the worker is lost until restart — the operator sees stuck checkouts and nothing that names the provider.

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
recommendation: Bound the retries (a few attempts with exponential backoff and jitter) and make the charge idempotent before retrying it — send an idempotency key derived from `order_id` (if the provider supports one) or check the charge status before re-posting; retry only errors where the charge is known not to have happened (connection refused, 429/503 before processing), never a 4xx such as a declined card; after the last attempt, raise an error naming the order.
history:
  - status: open
    by: ops
```

Scenario 1: the provider processes the charge but the connection drops or a 502 comes back from a proxy before the response — `RequestException` is caught and the same POST is sent again a second later, charging the customer twice (and again each second while the failure lasts). Scenario 2: the provider rejects the request permanently (400 invalid amount, 402 card declined) — `raise_for_status()` raises `HTTPError`, a `RequestException`, so the loop retries forever at a fixed 1-second pace, hammering the provider and never returning to the caller.

#### R-ops-3 Charge failures are swallowed without any log

```yaml
severity: high
confidence: high
location: "app/payments.py:17-18"
evidence: |
  except requests.RequestException:
      time.sleep(1)
recommendation: Log each failed attempt with structured fields — `order_id`, attempt number, HTTP status or exception type, provider error code — without the request body beyond what is safe, and surface the final failure to the caller as an actionable error naming the order and the provider status, instead of discarding the exception.
history:
  - status: open
    by: ops
```

Scenario: the provider is down or rejects every charge. The exception is dropped and nothing is logged, so the operator on call sees no error, no failing order id and no provider status — only checkouts that never complete — and cannot tell a provider outage from a bug.

#### R-ops-4 Payment provider URL is hard-coded

```yaml
severity: medium
confidence: high
location: "app/payments.py:7"
evidence: |
  PAYMENT_URL = "https://payments.example.com/v2/charges"
recommendation: Read the provider URL (and any credentials added later) from the environment or the application configuration, so staging and tests can point at the provider's sandbox and production at the live endpoint without a code change.
history:
  - status: open
    by: ops
```

Scenario: the code is deployed to staging or run in tests — every charge goes to the single hard-coded production endpoint, so a test checkout either charges real money or fails, and switching to the sandbox or a new provider host requires a new release.

### newcomer

Disabled by configuration.
