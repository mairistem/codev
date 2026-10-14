# Review record: rename-listen-setting

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### migration

#### R-migration-1 The `listen` alias and the `customer_id` JSON key have no automated test

```yaml
severity: low
confidence: high
location: "src/config.rs:10-11"
evidence: |
  #[serde(alias = "listen")]
  pub bind_address: String,
recommendation: >-
  The migration paths were verified once by hand on staging (tasks 1.1 and 1.2).
  Add unit tests that load a TOML file using `listen` and one using `bind_address`
  through `config::load`, and that serialize an `OrderResponse` and assert the JSON
  key is `customer_id` (src/api.rs:10-11), so that dropping the alias or the rename
  in a later change fails the build instead of breaking existing config files or
  partners' integrations.
history:
  - status: open
    by: migration
```

#### R-migration-2 Rolling back to 4.1 fails on a config file already moved to `bind_address`

```yaml
severity: medium
confidence: low
location: "config.example.toml:2"
evidence: |
  bind_address = "0.0.0.0:8080"
recommendation: >-
  The release note invites operators to rename `listen` to `bind_address`, but the
  previous release's `Config` requires `listen`, so a 4.1 instance started on a
  migrated file (rollback, or old and new instances sharing one file during a rolling
  deploy) fails at start-up with a missing-field error. Whether that happens depends
  on deployment practice the reviewer could not see. Add to the release note that
  files should keep `listen` until rollback to 4.1 is no longer needed, or that a
  rollback requires restoring `listen`.
history:
  - status: open
    by: migration
```

### newcomer

Disabled by configuration.
