# Review record: rename-listen-setting

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### migration

#### R-migration-1 The compatibility paths for `listen` and `customer_id` have no automated test

```yaml
severity: medium
confidence: low
location: "src/config.rs:10-11"
evidence: |
  #[serde(alias = "listen")]
  pub bind_address: String,
recommendation: Add a unit test that loads a configuration file holding only `listen = "..."` through `config::load` and checks `bind_address`, and one that serializes an `OrderResponse` with `serde_json` and checks the key is `customer_id` (src/api.rs:10-11, `#[serde(rename = "customer_id")]`). Tasks 1.1 and 1.2 were verified only by a one-off manual check on staging, and the repository has no test, so removing the alias or the rename later would break existing `config.toml` files (start-up error on the missing `bind_address`) and partners' integrations (`customer_id` gone from the JSON) with nothing to catch it, against the criterion that the migration path is tested.
history:
  - status: open
    by: migration
```

### newcomer

Disabled by configuration.
