# Review record: drop-legacy-email

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### migration

#### R-migration-1 Public API field `user_name` renamed `userName` without a BREAKING mark or compatibility path

```yaml
severity: high
confidence: high
location: "src/api.rs:8-9"
evidence: |
  #[serde(rename = "userName")]
  pub user_name: String,
recommendation: >-
  `Account` is a type of the public HTTP API (src/api.rs:1). Every existing
  consumer that reads `user_name` from the account response gets a missing
  field after deploy, with no warning: the release note only says "Changed:
  Account JSON uses camelCase." Mark it **BREAKING** in the release note with
  what consumers must do, and give consumers time — e.g. emit both
  `user_name` and `userName` for a deprecation period (or version the
  endpoint), then remove `user_name` in a later release.
history:
  - status: open
    by: migration
```

#### R-migration-2 `DROP COLUMN legacy_email` is irreversible, with no down migration and no backup step

```yaml
severity: high
confidence: high
location: "migrations/0005_drop_legacy_email.sql:2"
evidence: |
  ALTER TABLE users DROP COLUMN legacy_email;
recommendation: >-
  The rows imported in 2023 still hold their `legacy_email` values
  (column created in migrations/0004_accounts.sql:4); running 0005 destroys
  them permanently. Unlike 0004, which ships 0004_accounts.down.sql, 0005 has
  no down migration, and neither the plan nor the tasks name a backup or a
  recovery path. Add a backup step (copy the column to an archive table or
  export it) before the drop and a `0005_drop_legacy_email.down.sql` that
  re-adds the column (and restores from the backup), or state in the plan
  why the data can be lost and how to recover.
history:
  - status: open
    by: migration
```

### newcomer

Disabled by configuration.
