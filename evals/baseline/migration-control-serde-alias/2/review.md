# Review record: rename-listen-setting

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### migration

#### R-migration-1 Public Rust fields renamed with no deprecation or BREAKING note

```yaml
severity: medium
confidence: low
location: "src/config.rs:10-11"
evidence: |
  #[serde(alias = "listen")]
  pub bind_address: String,
recommendation: >-
  The crate `orders` is a library (`src/lib.rs` exports `pub mod api; pub mod config;`),
  and the change renames two public struct fields at the Rust level —
  `Config::listen` → `Config::bind_address` and `OrderResponse::customer_id` →
  `OrderResponse::buyer_id` (src/api.rs:11-12). The serde attributes keep the TOML
  and JSON compatible, but any Rust code that reads `config.listen` or builds
  `OrderResponse { customer_id, .. }` stops compiling on upgrade to 4.2, with no
  deprecation period and no **BREAKING** item in the proposal's release note. If the
  crate has Rust consumers outside this repository (not verifiable here), add a
  **BREAKING** release-note item naming both fields and the replacement, or keep a
  deprecated accessor (e.g. `#[deprecated] pub fn listen(&self) -> &str`) for one
  release; if it has none, say so in the proposal's Impact.
history:
  - status: open
    by: migration
```

#### R-migration-2 Compatibility paths verified only by a manual staging check, no test

```yaml
severity: low
confidence: high
location: "tasks.md#1. Implementation"
evidence: |
  - [x] 1.1 `bind_address` with the `listen` alias in `src/config.rs` and `config.example.toml`, verified by starting the service on staging with an old and a new file
  - [x] 1.2 `buyer_id` in `src/api.rs`, verified by comparing the order endpoint's response on staging before and after
recommendation: >-
  The role asks that the migration path for existing configuration and API consumers
  be tested. Both compatibility guarantees — an existing `config.toml` with
  `listen = "…"` still loading via `#[serde(alias = "listen")]`, and the JSON of
  `OrderResponse` still emitting `customer_id` (docs/api.md promises partners no
  rename without two releases of deprecation) — rest on a one-off staging check.
  Nothing in the repository fails if a later edit drops the `alias` or the `rename`.
  Add two unit tests: `config::load("database_url = \"x\"\nlisten = \"0.0.0.0:8080\"")`
  yields `bind_address == "0.0.0.0:8080"`, and `serde_json::to_value(OrderResponse { .. })`
  has a `customer_id` key and no `buyer_id` key.
history:
  - status: open
    by: migration
```

### newcomer

Disabled by configuration.
