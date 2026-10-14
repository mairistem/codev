# Review record: add-order-currency

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### migration

#### R-migration-1 `rename_all = "camelCase"` silently renames every existing field of the public order JSON

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
  Remove `#[serde(rename_all = "camelCase")]` so that `GET /orders/{id}` keeps
  emitting `order_id`, `customer_id` and `total_cents`, and only adds
  `currency` (single-word, unaffected). Today a partner integration that reads
  `order["order_id"]` gets nothing after the release: the response now carries
  `orderId`, `customerId` and `totalCents`, with no **BREAKING** item in the
  proposal's release note and against docs/api.md, which still lists the
  snake_case names and promises "a field is never renamed or removed without
  two releases of deprecation". If camelCase is wanted, it is a separate
  change that emits both names for two releases, is marked BREAKING and
  announced. Add a test that serializes an `OrderResponse` and asserts the
  documented field names.
history:
  - status: open
    by: migration
```

#### R-migration-2 New required `default_currency` key makes existing `config.toml` files fail to load

```yaml
severity: high
confidence: high
location: "src/config.rs:5-10"
evidence: |
  #[derive(Deserialize)]
  pub struct Config {
      pub database_url: String,
      pub listen: String,
      /// The currency of orders that do not name one (ISO 4217).
      pub default_currency: String,
recommendation: >-
  Give the field a default (`#[serde(default = "default_currency")]` returning
  `"EUR"`, the currency every existing order was in) so that a deployed
  `config.toml` holding only `database_url` and `listen` keeps loading. As it
  stands, `toml::from_str` fails with "missing field `default_currency`" and
  the service refuses to start after the upgrade; only config.example.toml was
  updated, and the release note does not tell operators to add the key. If a
  required key is intended, mark it **BREAKING** in the release note with the
  line to add. Add a test that loads the pre-change configuration.
history:
  - status: open
    by: migration
```

### newcomer

Disabled by configuration.
