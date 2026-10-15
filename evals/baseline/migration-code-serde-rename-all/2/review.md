# Review record: add-order-currency

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### migration

#### R-migration-1 camelCase rename silently breaks every existing order JSON field

```yaml
severity: high
confidence: high
location: "src/api.rs:6-9"
evidence: |
  #[serde(rename_all = "camelCase")]
  pub struct OrderResponse {
      pub order_id: i64,
      pub customer_id: i64,
      pub total_cents: i64,
recommendation: >-
  Remove `#[serde(rename_all = "camelCase")]` so `GET /orders/{id}` keeps
  emitting `order_id`, `customer_id` and `total_cents` (the new `currency`
  field is a single word and needs no rename). Today partners' integrations
  read `order_id`, `customer_id` and `total_cents`; after this change the
  endpoint returns `orderId`, `customerId` and `totalCents`, so every partner
  reading those keys gets a missing field on the first deploy, with no
  **BREAKING** item in the release note (which only says "Added: Orders carry
  their `currency`"), while `docs/api.md` still lists the snake_case names and
  promises "a field is never renamed or removed without two releases of
  deprecation". If camelCase is really wanted, it must go through that
  deprecation (emit both names for two releases), be marked BREAKING in the
  release note and the docs, and follow semantic versioning for the 4.1.0 crate.
history:
  - status: open
    by: migration
```

#### R-migration-2 Existing config.toml files fail to load without `default_currency`

```yaml
severity: high
confidence: high
location: "src/config.rs:9-10"
evidence: |
      /// The currency of orders that do not name one (ISO 4217).
      pub default_currency: String,
recommendation: >-
  Give the key a default so existing configuration keeps working, e.g.
  `#[serde(default = "default_currency")]` returning `"EUR"` (the currency the
  shop sold in before this change), and add a test that `config::load` accepts
  a file holding only `database_url` and `listen`. Today every deployed
  `config.toml` has only `database_url` and `listen` (as `config.example.toml`
  did on `main`); `Config` derives `Deserialize` with `default_currency` as a
  required `String`, so `toml::from_str` returns `missing field
  default_currency` and the service refuses to start on upgrade. Nothing in
  the release note tells operators to add the key; if it stays required, say
  so as a BREAKING item with the step to take.
history:
  - status: open
    by: migration
```

### newcomer

Disabled by configuration.
