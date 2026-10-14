# Review record: add-order-currency

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### migration

#### R-migration-1 rename_all = "camelCase" renames every existing field of the public order JSON

```yaml
severity: high
confidence: high
location: "src/api.rs:6-12"
evidence: |
  #[serde(rename_all = "camelCase")]
  pub struct OrderResponse {
      pub order_id: i64,
      pub customer_id: i64,
      pub total_cents: i64,
recommendation: >-
  Remove `#[serde(rename_all = "camelCase")]` so `order_id`, `customer_id` and
  `total_cents` keep their snake_case names and only `currency` is added. Today a
  partner integration reading `order_id` from `GET /orders/{id}` gets nothing
  after this change: the JSON now holds `orderId`, `customerId`, `totalCents`.
  This breaks the contract in docs/api.md ("a field is never renamed or removed
  without two releases of deprecation"), is marked nowhere as BREAKING, is absent
  from the release note ("Added: Orders carry their `currency`."), and
  contradicts docs/api.md itself, which still lists the snake_case names. If
  camelCase is wanted, emit both names for two releases and announce it as
  BREAKING; add a test pinning the serialized field names.
history:
  - status: open
    by: migration
```

#### R-migration-2 Existing config.toml files without default_currency no longer load

```yaml
severity: high
confidence: high
location: "src/config.rs:9-10"
evidence: |
      /// The currency of orders that do not name one (ISO 4217).
      pub default_currency: String,
recommendation: >-
  Give the field a default (`#[serde(default = "default_currency")]` returning
  "EUR", the currency sold before this change) so existing files keep loading.
  As written, every deployed `config.toml` — which today holds only
  `database_url` and `listen`, as config.example.toml did on main — makes
  `toml::from_str` fail with a missing-field error at start-up after the upgrade,
  and the service does not start; nothing in the proposal or release note tells
  operators to add the key. Add a test loading the old two-key file. If the key
  must stay required, mark it BREAKING with the step to take in the release note.
history:
  - status: open
    by: migration
```

### newcomer

Disabled by configuration.
