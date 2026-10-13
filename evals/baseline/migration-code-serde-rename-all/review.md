# Review record: add-order-currency

<!-- codev-review: 1 -->

## code.review

### migration

#### R-migration-1 `rename_all = "camelCase"` renames every existing field of the public order JSON

```yaml
severity: high
confidence: high
location: "src/api.rs:5-13"
evidence: |
  #[derive(Serialize)]
  #[serde(rename_all = "camelCase")]
  pub struct OrderResponse {
      pub order_id: i64,
      pub customer_id: i64,
      pub total_cents: i64,
recommendation: Remove `#[serde(rename_all = "camelCase")]` so the existing keys stay `order_id`, `customer_id` and `total_cents`; only `currency` is added. If camelCase is wanted, ship it as a separate change marked BREAKING, emitting both spellings for the two releases of deprecation that docs/api.md promises.
history:
  - status: open
    by: migration
```

The attribute turns `order_id`, `customer_id` and `total_cents` into `orderId`, `customerId` and `totalCents` in `GET /orders/{id}`. Partners' integrations that read the current keys break silently on upgrade. `docs/api.md` still lists the snake_case names and says "a field is never renamed or removed without two releases of deprecation". The proposal's release note announces only an addition, and nothing is marked **BREAKING**.

#### R-migration-2 `default_currency` is a required key: existing `config.toml` files no longer load

```yaml
severity: high
confidence: high
location: "src/config.rs:9-10"
evidence: |
  /// The currency of orders that do not name one (ISO 4217).
  pub default_currency: String,
recommendation: Add `#[serde(default = "...")]` returning "EUR" (the currency used until now) so existing deployments keep starting, and add a test that loads a config without the key. Otherwise mark the change BREAKING and tell operators in the release note to add the key before upgrading.
history:
  - status: open
    by: migration
```

`Config` derives `Deserialize` with no `#[serde(default)]`. An existing deployment's `config.toml` has no `default_currency` key, so after the upgrade `toml::from_str` fails with "missing field `default_currency`" and the service does not start. Only `config.example.toml` was updated. The release note says nothing about the new key, and nothing is marked **BREAKING**.

#### R-migration-3 Release note announces neither the breaking changes nor the new configuration key

```yaml
severity: medium
confidence: high
location: "proposal.md#Release note"
evidence: |
  - Added: Orders carry their `currency`.
recommendation: Once R-migration-1 and R-migration-2 are settled, list each remaining breaking change as **BREAKING** with what partners and operators must do. Also add a line for the new `default_currency` configuration key and its default.
history:
  - status: open
    by: migration
```

#### R-migration-4 No test covers the migration path for old configuration files or API consumers

```yaml
severity: medium
confidence: high
location: "tasks.md#1. Implementation"
evidence: |
  - [x] 1.2 `default_currency` in `src/config.rs` and `config.example.toml`, verified by starting the service on staging
recommendation: Add tests that (a) load a config.toml without `default_currency`, and (b) serialize an `OrderResponse` and assert the exact JSON keys (`order_id`, `customer_id`, `total_cents`, `currency`, `status`), so a rename is caught.
history:
  - status: open
    by: migration
```

Task 1.1 is verified only "by the order endpoint's response on staging". Task 1.2 is verified only by starting the service with the new example config, which already has the key. The crate has no test. Neither check would catch R-migration-1 or R-migration-2.

#### R-migration-5 Public `report::SalesRow` and its field `count` renamed with no deprecation

```yaml
severity: low
confidence: low
location: "src/report.rs:3-7"
evidence: |
  pub struct DailySales {
      pub day: String,
      pub order_count: u32,
recommendation: If any crate depends on `orders` as a library, keep `#[deprecated] pub type SalesRow = DailySales;` for one release and note the rename. Otherwise no action is needed.
history:
  - status: open
    by: migration
```

The proposal calls the type "internal". It is still `pub` in `pub mod report` of the `orders` library crate (`src/lib.rs`), and `SalesRow` is removed in the same release that introduces `DailySales`. The reviewer could not see whether any other crate imports it, hence the low confidence. The printed output of `orders report` is unchanged.

### newcomer

Disabled by configuration.
