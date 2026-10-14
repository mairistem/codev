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
recommendation: Pass an explicit `timeout=` (connect and read) to `requests.post`. Without it, when the provider accepts the connection but stops answering, the call blocks indefinitely and the checkout request that called `charge` never returns; the operator sees stuck workers with no error and no log.
history:
  - status: open
    by: ops
```

#### R-ops-2 Unbounded retry of a non-idempotent charge can duplicate charges

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
recommendation: Bound the retries (a small maximum, with exponential backoff and jitter instead of a fixed 1 s sleep) and raise once they are exhausted. Retry only transient failures (connection errors, timeouts, 5xx/429) — `raise_for_status` turns a 4xx such as a declined card into an `HTTPError`, which this loop retries forever. Send an idempotency key derived from `order_id` so a retry after a read timeout or a lost response, where the provider already charged, does not charge the customer twice. Today, a provider outage turns every checkout into an infinite loop hammering the provider once a second, and a lost response can charge the same order repeatedly.
history:
  - status: open
    by: ops
```

#### R-ops-3 Failed charge attempts are swallowed without any log

```yaml
severity: high
confidence: high
location: "app/payments.py:17-18"
evidence: |
  except requests.RequestException:
      time.sleep(1)
recommendation: >-
  Log each failed attempt with structured fields (order_id, attempt number, HTTP status or exception type, provider URL — never the request body or credentials), and log/raise an actionable error when retries are exhausted. As written, when the provider rejects or fails every request, nothing is logged and no error surfaces: the operator cannot tell that charges are failing, nor for which orders or why.
history:
  - status: open
    by: ops
```

#### R-ops-4 Payment provider URL hard-coded instead of read from configuration

```yaml
severity: medium
confidence: high
location: "app/payments.py:7"
evidence: |
  PAYMENT_URL = "https://payments.example.com/v2/charges"
recommendation: Read the provider URL from the environment or the application's configuration (with the production value as deployment configuration, not a code default). The provider endpoint differs between environments; hard-coded, a staging or local run charges the production provider, and switching to a sandbox or a new endpoint requires a new release.
history:
  - status: open
    by: ops
```

### newcomer

Disabled by configuration.
