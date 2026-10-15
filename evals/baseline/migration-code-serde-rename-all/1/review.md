# Review record: add-order-currency

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### migration

#### R-migration-1 `rename_all = "camelCase"` renames every existing field of the public order JSON

```yaml
severity: high
confidence: high
location: "src/api.rs:6-10"
evidence: |
  #[serde(rename_all = "camelCase")]
  pub struct OrderResponse {
      pub order_id: i64,
      pub customer_id: i64,
      pub total_cents: i64,
recommendation: >-
  Remove `#[serde(rename_all = "camelCase")]` so `GET /orders/{id}` keeps
  emitting `order_id`, `customer_id` and `total_cents`, and only gains
  `currency`. Today every partner integration that reads `order_id`,
  `customer_id` or `total_cents` (as `docs/api.md` still documents them)
  finds them missing after the deploy, with no BREAKING item in the release
  note and against the documented rule of two releases of deprecation
  before a field is renamed. If camelCase is really wanted, it needs its
  own change: emit both names during the deprecation window, mark it
  BREAKING, and announce it.
history:
  - status: open
    by: migration
```

#### R-migration-2 New required `default_currency` key makes existing `config.toml` files fail to load

```yaml
severity: high
confidence: high
location: "src/config.rs:9-10"
evidence: |
  /// The currency of orders that do not name one (ISO 4217).
  pub default_currency: String,
recommendation: >-
  Give the field a default (`#[serde(default = "default_currency")]`
  returning `"EUR"`, the currency every existing order was in), and test
  that a config without the key still loads. Today `config::load` returns
  `missing field `default_currency`` for every deployed `config.toml`
  written before this change, so the service refuses to start on upgrade;
  the release note says nothing about a manual step. If the key must stay
  required, mark it BREAKING in the release note with the line operators
  must add.
history:
  - status: open
    by: migration
```

### newcomer

Disabled by configuration.
