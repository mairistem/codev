# Review record: rename-listen-setting

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### migration

#### R-migration-1 No automated test that an old config file with `listen` still loads

```yaml
severity: low
confidence: high
location: "src/config.rs:10-11"
evidence: |
  #[serde(alias = "listen")]
  pub bind_address: String,
recommendation: >-
  Add a unit test calling `config::load` on a file holding only `listen = "…"`
  (as every file written before this change does) and asserting
  `bind_address` gets its value, next to one with `bind_address`. The alias
  is correct today; the role's criterion asks for the migration path to be
  tested, and task 1.1 checked it only by hand on staging, so removing or
  mistyping the alias later would make old files fail at start-up with a
  missing-field error and no test would catch it.
history:
  - status: open
    by: migration
```

#### R-migration-2 No automated test that the order JSON still says `customer_id`

```yaml
severity: low
confidence: high
location: "src/api.rs:10-11"
evidence: |
  #[serde(rename = "customer_id")]
  pub buyer_id: i64,
recommendation: >-
  Add a unit test serializing an `OrderResponse` and asserting the JSON key is
  `customer_id` (and that no `buyer_id` key appears). The rename keeps
  partners' integrations working today, and docs/api.md promises the field is
  never renamed without two releases of deprecation; task 1.2 checked it only
  by comparing staging responses by hand, so dropping the attribute later
  would silently rename the public field.
history:
  - status: open
    by: migration
```

### newcomer

Disabled by configuration.
