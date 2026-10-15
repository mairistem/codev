# Review record: fetch-exchange-rates

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### ops

#### R-ops-1 Final failure of the rates fetch emits no log or signal of its own

```yaml
severity: medium
confidence: low
location: "src/rates.rs:28-37"
evidence: |
  Err(error) => return Err(format!("rates feed {url}: {error} after {ATTEMPTS} attempts")),
recommendation: >-
  The last failed attempt and an invalid body only return a `String`; only the
  intermediate attempts are logged (`warn!` at line 33). Since the proposal has
  the caller silently keep the previous day's rates, a feed that stays down or
  changes its format leaves prices on stale rates with nothing an operator can
  see, unless the caller logs the error. No caller is in the diff, hence the low
  confidence. Emit an `error!` with fields (`attempts`, the error) on both final
  failure paths, or make sure the caller does, and consider logging or exposing
  the date of the last successful fetch so staleness is visible.
history:
  - status: open
    by: ops
```

### newcomer

Disabled by configuration.
