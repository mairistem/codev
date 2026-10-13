# Review record: fetch-exchange-rates

<!-- codev-review: 1 -->

## code.review

### ops

#### R-ops-1 Final fetch failure is returned but never logged or counted

```yaml
severity: medium
confidence: low
location: "src/rates.rs:37"
evidence: |
  Err(error) => return Err(format!("rates feed {url}: {error} after {ATTEMPTS} attempts")),
recommendation: Emit a `warn!`/`error!` with fields (attempts, error) on the final failure, or confirm the caller logs it, so that stale rates kept by the caller do not go unnoticed for days.
history:
  - status: open
    by: ops
```

The proposal says the caller keeps the previous day's rates on failure; no caller is in the diff, so whether the exhausted retries ever reach a log or alert could not be verified. Only the intermediate attempts are logged (`warn!` at line 33).

#### R-ops-2 Errors flattened to String, losing their source chain

```yaml
severity: low
confidence: high
location: "src/rates.rs:18-37"
evidence: |
  pub fn fetch_rates() -> Result<Rates, String> {
recommendation: Return a typed error (e.g. an enum wrapping `reqwest::Error`, or `anyhow::Error` with context) so the caller can distinguish a missing variable, a transport failure and an invalid body, and keep the source chain.
history:
  - status: open
    by: ops
```

Lines 19, 23, 28 and 37 all convert errors with `format!`, which the role's Rust guidance flags as losing the source chain.

#### R-ops-3 Permanent HTTP errors (4xx) are retried like transient ones

```yaml
severity: low
confidence: high
location: "src/rates.rs:26"
evidence: |
  match client.get(&url).send().and_then(|r| r.error_for_status()) {
recommendation: Retry only on transport errors, timeouts, 429 and 5xx; fail at once on other 4xx (e.g. a wrong `RATES_FEED_URL` returning 404) so the misconfiguration surfaces without the backoff delay.
history:
  - status: open
    by: ops
```

#### R-ops-4 Feed URL embedded in error messages

```yaml
severity: low
confidence: low
location: "src/rates.rs:28"
evidence: |
  let rates: Rates = response.json().map_err(|e| format!("rates feed {url}: invalid body: {e}"))?;
recommendation: Keep the URL out of error strings (or log only its host), in case `RATES_FEED_URL` ever carries an API key in its query string; line 37 does the same.
history:
  - status: open
    by: ops
```

The feed is described as public, so no secret is known to be in the URL today.

### newcomer

Disabled by configuration.
