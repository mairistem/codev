# Review record: add-order-currency

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### migration

#### R-migration-1 rename_all = "camelCase" renames every existing field of the public order JSON

```yaml
severity: high
confidence: high
location: "src/api.rs:5-6"
evidence: |
  #[derive(Serialize)]
  #[serde(rename_all = "camelCase")]
recommendation: >-
  Remove `#[serde(rename_all = "camelCase")]`: it turns `order_id`, `customer_id` and
  `total_cents` into `orderId`, `customerId` and `totalCents`, so every partner
  integration reading `GET /orders/{id}` today gets missing fields on the first
  response after the upgrade. docs/api.md still lists the snake_case names and
  promises "a field is never renamed or removed without two releases of
  deprecation"; the proposal's release note only says "Added", with no BREAKING
  item. Keep the snake_case names (the new field is then `currency`), and add a
  serialization test pinning the JSON keys. If camelCase is ever wanted, follow
  the documented two-release deprecation and mark it BREAKING in the release note.
history:
  - status: open
    by: migration
```

#### R-migration-2 Existing config.toml files without default_currency stop loading

```yaml
severity: high
confidence: high
location: "src/config.rs:9-10"
evidence: |
  /// The currency of orders that do not name one (ISO 4217).
  pub default_currency: String,
recommendation: >-
  `default_currency` is required with no `#[serde(default)]`, so every deployed
  `config.toml` written before this change (holding only `database_url` and
  `listen`) fails in `toml::from_str` with a "missing field `default_currency`"
  error and the service does not start after the upgrade. Nothing in the release
  note tells operators to add the key. Give the field a default (e.g.
  `#[serde(default = "default_currency")]` returning "EUR", the currency the shop
  sold in before), add a test loading the old two-key config, or else mark the
  new required key BREAKING in the release note with the manual step.
history:
  - status: open
    by: migration
```
