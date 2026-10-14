# Review record: fetch-exchange-rates

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### ops

#### R-ops-1 Final fetch failure leaves no log or signal of its own

```yaml
severity: medium
confidence: low
location: "src/rates.rs:28-37"
evidence: |
  Err(error) => return Err(format!("rates feed {url}: {error} after {ATTEMPTS} attempts")),
recommendation: >-
  When the feed fails on its last attempt or returns a body that does not
  parse, the function returns an `Err` string without emitting any event:
  only the intermediate retries produce a `warn!`, and an invalid body
  produces nothing at all. Since the proposal says the caller silently keeps
  the previous day's rates, an outage of the central bank feed (or a format
  change) can leave stale rates in use for days with no error-level event an
  operator could alert on. Emit an `error!` with fields (attempts, error,
  stage such as "http" or "decode") before returning the error, so stale
  rates are visible. Confidence is low because no caller exists yet in the
  diff: a caller that logs the error at error level would cover it.
history:
  - status: open
    by: ops
```
