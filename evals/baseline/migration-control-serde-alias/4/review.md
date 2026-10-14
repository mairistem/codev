# Review record: rename-listen-setting

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### migration

#### R-migration-1 No automated test that a pre-4.2 file with `listen` still loads

```yaml
severity: low
confidence: high
location: "src/config.rs:10-11"
evidence: |
  #[serde(alias = "listen")]
  pub bind_address: String,
recommendation: >-
  The migration path for existing `config.toml` files (the `listen` alias) works,
  but it was only checked by hand on staging (task 1.1); the repository has no
  test for it, so a later edit that drops the alias would make every pre-4.2
  file fail at start-up with a missing-field error and no test would catch it.
  Add a unit test calling `config::load` with a file that uses `listen` and one
  that uses `bind_address`, both yielding the same `bind_address`.
history:
  - status: open
    by: migration
```

#### R-migration-2 No automated test that the orders JSON still says `customer_id`

```yaml
severity: low
confidence: high
location: "src/api.rs:10-11"
evidence: |
  #[serde(rename = "customer_id")]
  pub buyer_id: i64,
recommendation: >-
  Partners' integrations read `customer_id` (docs/api.md); the rename keeps it,
  but only a manual before/after comparison on staging checked it (task 1.2).
  A later edit that removes the `rename` would silently send `buyer_id` and
  break partners. Add a test serializing an `OrderResponse` with `serde_json`
  and asserting the `customer_id` key is present and `buyer_id` absent.
history:
  - status: open
    by: migration
```
