# Review record: add-order-currency

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### migration

#### R-migration-1 rename_all = "camelCase" renames every existing order JSON field

```yaml
severity: high
confidence: high
location: "src/api.rs:6"
evidence: |
  #[serde(rename_all = "camelCase")]
recommendation: Remove `#[serde(rename_all = "camelCase")]` so `order_id`, `customer_id`, `total_cents` and `status` keep their names and only `currency` is added. Today `GET /orders/{id}` returns `orderId`, `customerId`, `totalCents` instead of the snake_case fields that docs/api.md still lists and partners' integrations read; they break silently on the release, with no **BREAKING** item in the release note and against the documented rule of two releases of deprecation before a rename. If a camelCase API is wanted, it is a separate change that serves both names during the deprecation window.
history:
  - status: open
    by: migration
```

#### R-migration-2 Existing config.toml files without default_currency stop the service from starting

```yaml
severity: high
confidence: high
location: "src/config.rs:9-10"
evidence: |
  /// The currency of orders that do not name one (ISO 4217).
  pub default_currency: String,
recommendation: Give the field a default (`#[serde(default = "...")]` returning "EUR", the currency orders had before this change) so an existing `config.toml` holding only `database_url` and `listen` still loads; today `toml::from_str` fails with "missing field `default_currency`" and the service refuses to start after the upgrade. Add a test loading the old two-key config. If the key is meant to be required, mark it **BREAKING** in the release note with the line operators must add.
history:
  - status: open
    by: migration
```

### newcomer

Disabled by configuration.
