# Review record: add-order-currency

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### migration

#### R-migration-1 rename_all camelCase silently renames the existing public JSON fields

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
  Remove `#[serde(rename_all = "camelCase")]` so `GET /orders/{id}` keeps emitting
  `order_id`, `customer_id` and `total_cents` (the new `currency` field is a single
  word and needs no rename). Today partners' integrations read `order_id`; after
  this change the JSON carries `orderId`, `customerId` and `totalCents`, so every
  partner reading the documented names gets a missing field on the first response
  after deploy. This is an unmarked breaking change: the release note only says
  "Added", and docs/api.md still lists the snake_case names and promises "a field is
  never renamed or removed without two releases of deprecation". If camelCase is
  really wanted, it needs a deprecation period (emit both names) and a **BREAKING**
  release note item.
history:
  - status: open
    by: migration
```

#### R-migration-2 Existing config.toml files without default_currency no longer load

```yaml
severity: medium
confidence: high
location: "src/config.rs:9-10"
evidence: |
  /// The currency of orders that do not name one (ISO 4217).
  pub default_currency: String,
recommendation: >-
  Give the field a default (`#[serde(default = "...")]` returning "EUR", the
  currency orders had before this change) so every deployed `config.toml` written
  for the previous version keeps loading. As written, `config::load` on an existing
  file with only `database_url` and `listen` fails with a missing-field error and
  the service does not start after the upgrade. Only config.example.toml was
  updated, and the release note does not tell operators to add the key. Add a test
  that loads the old two-key configuration.
history:
  - status: open
    by: migration
```
