# Review record: rename-listen-setting

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### migration

#### R-migration-1 The `listen` alias has no automated test

```yaml
severity: low
confidence: high
location: "src/config.rs:10-11"
evidence: |
  #[serde(alias = "listen")]
  pub bind_address: String,
recommendation: Add a unit test that calls `config::load` on a file holding `listen = "0.0.0.0:8080"` and asserts `bind_address`, and one on a file holding `bind_address`, so the compatibility path for existing configuration files stays covered after the manual staging check of task 1.1 (the role asks for a tested migration path). A test that `OrderResponse` still serializes `customer_id` would cover the API side the same way.
history:
  - status: open
    by: migration
```

#### R-migration-2 Public Rust fields renamed without deprecation

```yaml
severity: low
confidence: low
location: "src/config.rs:11"
evidence: |
  pub bind_address: String,
recommendation: >-
  `Config::listen` (src/config.rs) and `OrderResponse::customer_id` (src/api.rs:11, now `pub buyer_id: i64,`) are `pub` fields of the `orders` library crate (`src/lib.rs` exports `pub mod api; pub mod config;`); any crate depending on `orders` that reads or builds these structs stops compiling. If such a consumer exists, mark the rename **BREAKING** in the release note with the new field names; if the crate has no outside consumer, nothing to do. Could not verify whether other crates depend on it.
history:
  - status: open
    by: migration
```

### newcomer

Disabled by configuration.
