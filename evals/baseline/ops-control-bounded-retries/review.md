# Review record: fetch-exchange-rates

<!-- codev-review: 1 -->

## code.review

### ops

#### R-ops-1 Final fetch failure is neither logged nor counted, so stale rates can go unnoticed

```yaml
severity: medium
confidence: low
location: "src/rates.rs:37"
evidence: |
            Err(error) => return Err(format!("rates feed {url}: {error} after {ATTEMPTS} attempts")),
recommendation: >-
  Emit a structured `error!` (fields: attempts, error) or a failure metric when the last
  attempt fails, and expose the age of the rates in use; the proposal says the caller keeps
  the previous day's rates on failure, and no caller exists in the diff to surface the error,
  so days of stale rates would leave only per-retry `warn!` lines.
history:
  - status: open
    by: ops
```

#### R-ops-2 Every error is retried, including permanent HTTP 4xx responses

```yaml
severity: low
confidence: high
location: "src/rates.rs:26"
evidence: |
        match client.get(&url).send().and_then(|r| r.error_for_status()) {
recommendation: >-
  Retry only transient failures (connect/timeout errors and 5xx/429); return at once on
  a 4xx such as a wrong `RATES_FEED_URL` (404) or a rejected request, with a message
  naming the status.
history:
  - status: open
    by: ops
```

#### R-ops-3 Errors flattened to String lose their source chain

```yaml
severity: low
confidence: high
location: "src/rates.rs:18"
evidence: |
  pub fn fetch_rates() -> Result<Rates, String> {
recommendation: >-
  Return a typed error (an enum wrapping `reqwest::Error` and the missing variable) so the
  caller can log the full source chain and tell a configuration error from a transient
  network failure.
history:
  - status: open
    by: ops
```

#### R-ops-4 Feed URL copied verbatim into error messages

```yaml
severity: low
confidence: low
location: "src/rates.rs:28"
evidence: |
                let rates: Rates = response.json().map_err(|e| format!("rates feed {url}: invalid body: {e}"))?;
recommendation: >-
  If `RATES_FEED_URL` may ever carry an access key in its query string, log only the host
  (or redact the query) in the errors at lines 28 and 37; a public feed URL is harmless.
history:
  - status: open
    by: ops
```

#### R-ops-5 Timeout, attempt count and initial pause are hard-coded

```yaml
severity: low
confidence: high
location: "src/rates.rs:21"
evidence: |
        .timeout(Duration::from_secs(10))
recommendation: >-
  Read the timeout (and optionally `ATTEMPTS` and the initial pause) from the environment
  with these values as defaults, so an operator can tune a slow feed without a release.
history:
  - status: open
    by: ops
```

### newcomer

Disabled by configuration.
