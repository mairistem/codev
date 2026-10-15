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
recommendation: >-
  Both compatibility paths are correct today: `#[serde(alias = "listen")]` keeps
  existing `config.toml` files readable, and `#[serde(rename = "customer_id")]`
  (src/api.rs:10) keeps the partners' JSON unchanged. But the migration path was
  verified only by hand on staging (tasks 1.1 and 1.2), and the repository has no
  test. Add a unit test that `config::load` accepts both a file with `listen` and
  one with `bind_address`, and one that serializes `OrderResponse` and asserts the
  `customer_id` key, so that removing either attribute in a later cleanup fails
  the build instead of breaking old configuration files or partners'
  integrations (docs/api.md promises two releases of deprecation).
history:
  - status: open
    by: migration
```

### newcomer

Disabled by configuration.
