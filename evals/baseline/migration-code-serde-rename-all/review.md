# Review record: add-order-currency

<!-- codev-review: 1 -->

## code.review

### migration

#### R-migration-1 `rename_all = "camelCase"` renames every existing order JSON field

```yaml
severity: high
confidence: high
location: "src/api.rs:5-12"
evidence: |
  #[derive(Serialize)]
  #[serde(rename_all = "camelCase")]
  pub struct OrderResponse {
      pub order_id: i64,
      pub customer_id: i64,
      pub total_cents: i64,
recommendation: >-
  Remove `#[serde(rename_all = "camelCase")]` so `GET /orders/{id}` keeps
  emitting `order_id`, `customer_id` and `total_cents` (the new `currency`
  field is unaffected either way). Today a partner integration reading
  `order_id` gets no such field after the deploy and breaks without warning;
  docs/api.md promises a field is never renamed without two releases of
  deprecation, and the proposal's release note announces only an addition,
  with no BREAKING item. If camelCase is really wanted, it needs its own
  change: emit both names during the deprecation window, mark it BREAKING
  and announce it.
history:
  - status: open
    by: migration
```

#### R-migration-2 Existing `config.toml` files without `default_currency` no longer load

```yaml
severity: medium
confidence: high
location: "src/config.rs:9-10"
evidence: |
  /// The currency of orders that do not name one (ISO 4217).
  pub default_currency: String,
recommendation: >-
  Add `#[serde(default = ...)]` returning "EUR" (the currency the shop sold
  in before this change) so a deployed `config.toml` holding only
  `database_url` and `listen` keeps loading; today `load` returns a
  `missing field default_currency` error and the service fails at start-up
  after the upgrade. Otherwise document the required new key as a manual
  upgrade step in the release note. Add a test loading the old two-key
  config.
history:
  - status: open
    by: migration
```
