# Review record: add-order-currency

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### migration

#### R-migration-1 camelCase renames every existing order JSON field

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
  Remove `#[serde(rename_all = "camelCase")]` so the existing fields keep their
  snake_case names (`order_id`, `customer_id`, `total_cents`) and only
  `currency` is added. Today `GET /orders/{id}` returns `orderId`,
  `customerId` and `totalCents` instead: every partner integration that reads
  `order_id`, `customer_id` or `total_cents` gets a missing field on the first
  response after deploy, with no **BREAKING** item in the release note (which
  only says "Added: Orders carry their `currency`") and in breach of
  docs/api.md's promise of two releases of deprecation before any rename. Add
  a test that serializes an `OrderResponse` and asserts the exact keys.
history:
  - status: open
    by: migration
```

#### R-migration-2 Existing config.toml files fail to load without default_currency

```yaml
severity: high
confidence: high
location: "src/config.rs:9-10"
evidence: |
      /// The currency of orders that do not name one (ISO 4217).
      pub default_currency: String,
recommendation: >-
  Give the new key a default, e.g. `#[serde(default = "default_currency")]`
  returning `"EUR"` (the currency the shop sold in before this change), and
  test that a config holding only `database_url` and `listen` still loads.
  As written the field is required: every deployed `config.toml`, which holds
  only `database_url` and `listen` like the previous config.example.toml, makes
  `load` return a `missing field `default_currency`` error and the service no
  longer starts after the upgrade. Nothing in the release note tells operators
  to add the key; if a default is not wanted, mark it **BREAKING** with the
  manual step.
history:
  - status: open
    by: migration
```
