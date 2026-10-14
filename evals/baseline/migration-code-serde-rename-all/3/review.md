# Review record: add-order-currency

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### migration

#### R-migration-1 camelCase renaming silently renames every existing order field

```yaml
severity: high
confidence: high
location: "src/api.rs:6"
evidence: |
  #[serde(rename_all = "camelCase")]
recommendation: Remove `#[serde(rename_all = "camelCase")]` so the existing fields keep serializing as `order_id`, `customer_id` and `total_cents`; the new `currency` field is the same in both casings and needs no attribute. If camelCase is ever wanted, follow docs/api.md's rule — emit both names for two releases of deprecation, mark the change **BREAKING** in the release note, and tell partners what to change.
history:
  - status: open
    by: migration
```

The attribute was added to the existing `OrderResponse`, so `GET /orders/{id}` now returns `orderId`, `customerId` and `totalCents` in place of `order_id`, `customer_id` and `total_cents`. Partners' integrations read those keys, so on the first call after deployment they find the fields missing: deserialization fails, or the integrations read nulls. Nothing marks this as breaking. The release note only says "Added: Orders carry their `currency`." The proposal lists only a new field. docs/api.md still lists the snake_case names and promises "a field is never renamed or removed without two releases of deprecation".

#### R-migration-2 New required `default_currency` key makes existing config files fail to load

```yaml
severity: high
confidence: high
location: "src/config.rs:9-10"
evidence: |
  /// The currency of orders that do not name one (ISO 4217).
  pub default_currency: String,
recommendation: Give the field a default, with `#[serde(default = "...")]` returning "EUR" (the shop's currency before this change), so that existing `config.toml` files keep loading. Add a test that loads a config without the key. Otherwise, mark the change **BREAKING** and state in the release note that operators must add `default_currency` before upgrading.
history:
  - status: open
    by: migration
```

`Config` derives `Deserialize` and has no `#[serde(default)]`, and `load` calls `toml::from_str(text)`. Only `config.example.toml` was updated. A deployed `config.toml` with just `database_url` and `listen` now fails with a `missing field `default_currency`` error, so the service does not start after the upgrade. Neither the release note nor the plan gives operators a migration step.

### newcomer

Disabled by configuration.
