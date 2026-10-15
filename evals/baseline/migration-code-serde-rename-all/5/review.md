# Review record: add-order-currency

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### newcomer

Disabled by configuration.

### migration

#### R-migration-1 `rename_all = "camelCase"` renames every existing order JSON field

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
  Remove `#[serde(rename_all = "camelCase")]` so `order_id`, `customer_id`
  and `total_cents` keep their snake_case names, and add `currency` as a
  purely additive field. If a camelCase API is wanted, it is a separate,
  **BREAKING** change with two releases of deprecation (per docs/api.md),
  serving both names in the meantime and announced in the release note.
  Add a test that serializes an `OrderResponse` and asserts the existing
  field names are unchanged.
history:
  - status: open
    by: migration
```

Scenario: a partner integration that today reads `order_id`,
`customer_id` and `total_cents` from `GET /orders/{id}` receives
`orderId`, `customerId` and `totalCents` after the deploy; its lookups
return nothing and it breaks without warning. `docs/api.md` states "a
field is never renamed or removed without two releases of deprecation",
still documents the snake_case names, and the proposal's release note
lists only "Added" — no **BREAKING** item.

#### R-migration-2 Required `default_currency` makes existing `config.toml` files fail to load

```yaml
severity: high
confidence: high
location: "src/config.rs:9-10"
evidence: |
  /// The currency of orders that do not name one (ISO 4217).
  pub default_currency: String,
recommendation: >-
  Give the field a default so existing files keep loading — e.g.
  `#[serde(default = "default_currency")]` returning `"EUR"`, the currency
  the shop sold in before this change — and add a test that `load` accepts
  the previous `config.example.toml` (without `default_currency`). Otherwise
  mark the new key **BREAKING** in the release note with the line operators
  must add.
history:
  - status: open
    by: migration
```

Scenario: an existing deployment's `config.toml` holds only
`database_url` and `listen` (as the previous `config.example.toml` did).
After upgrading, `toml::from_str` returns a "missing field
`default_currency`" error and the service no longer starts. Only
`config.example.toml` was updated; the release note says nothing about the
new key.
