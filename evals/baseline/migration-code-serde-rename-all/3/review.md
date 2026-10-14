# Review record: add-order-currency

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### newcomer

Disabled by configuration.

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
  emitting `order_id`, `customer_id` and `total_cents`; the new field
  `currency` is a single word and needs no renaming. Today a partner's
  integration reading `order_id`, `customer_id` or `total_cents` gets a
  missing field (deserialization error or null) on the first response after
  deploy, with no warning: the change ships no BREAKING item, the release
  note only says "Added: Orders carry their `currency`", and `docs/api.md`
  still lists the snake_case names and promises "a field is never renamed or
  removed without two releases of deprecation". If camelCase is wanted, it
  must go through that deprecation (emit both names for two releases), be
  marked BREAKING in the release note, and be reflected in `docs/api.md`.
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
  returning `"EUR"`, the currency the shop sold in before this change) so a
  deployed `config.toml` holding only `database_url` and `listen` keeps
  loading. As written, `toml::from_str` returns `missing field
  `default_currency`` for every existing configuration file and the service
  refuses to start after the upgrade; only `config.example.toml` was
  updated, and the release note announces no manual step. If the key must
  stay required, mark it BREAKING in the release note with the line to add,
  and add a test loading a config without the key.
history:
  - status: open
    by: migration
```
