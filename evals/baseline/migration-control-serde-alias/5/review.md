# Review record: rename-listen-setting

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### migration

#### R-migration-1 No automated test pins the `listen` alias and the `customer_id` JSON name

```yaml
severity: low
confidence: high
location: "src/config.rs:10-11"
evidence: |
  #[serde(alias = "listen")]
  pub bind_address: String,
recommendation: Add unit tests that load a TOML file using `listen` and one using `bind_address` through `config::load`, and that serialize an `OrderResponse` and assert the JSON key is `customer_id` (src/api.rs:10-11). Both migration paths are today only verified by a manual staging run (tasks 1.1 and 1.2), so a later edit dropping the `alias` or the `rename` would silently break existing config files or partners' integrations with nothing failing.
history:
  - status: open
    by: migration
```

#### R-migration-2 Public Rust fields renamed without deprecation for crate consumers

```yaml
severity: medium
confidence: low
location: "src/config.rs:11"
evidence: |
  pub bind_address: String,
recommendation: "`orders` is a library crate (src/lib.rs exposes `pub mod api; pub mod config;`) and the public fields `Config::listen` and `OrderResponse::customer_id` (src/api.rs:11, now `buyer_id`) are renamed in one step, so any Rust code reading `config.listen` or building `OrderResponse { customer_id, .. }` stops compiling, with no BREAKING item in the release note. No such consumer is visible in this repository; if one exists, mark the rename BREAKING in the release note or keep a deprecated accessor for a release."
history:
  - status: open
    by: migration
```

### newcomer

Disabled by configuration.
