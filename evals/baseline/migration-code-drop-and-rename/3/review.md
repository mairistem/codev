# Review record: drop-legacy-email

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### migration

#### R-migration-1 Account field renamed in the public API without a BREAKING mark or transition

```yaml
severity: high
confidence: high
location: "src/api.rs:8-9"
evidence: |
  #[serde(rename = "userName")]
  pub user_name: String,
recommendation: >-
  Every existing consumer of the public HTTP API that reads `user_name` from the
  Account JSON gets a missing field (null/undefined or a deserialization error)
  the moment this ships, with no warning: the release note only says
  "Changed: Account JSON uses camelCase." Mark the item **BREAKING** in the
  release note and say what consumers must do (read `userName`); and, since
  consumers need time, give them a transition — serialize both `user_name`
  (deprecated) and `userName` for a release, or version the endpoint — then
  remove the old name in a later change. Add a test of the serialized Account
  JSON covering the field names.
history:
  - status: open
    by: migration
```

#### R-migration-2 Irreversible DROP COLUMN with no down migration or backup

```yaml
severity: high
confidence: high
location: "migrations/0005_drop_legacy_email.sql:2"
evidence: |
  ALTER TABLE users DROP COLUMN legacy_email;
recommendation: >-
  Running migration 0005 deletes every value stored in `users.legacy_email`
  (the column still holds the data from the 2023 import, even if it is no
  longer written) and there is no way back: unlike 0004, which ships
  `0004_accounts.down.sql`, there is no `0005_drop_legacy_email.down.sql`,
  and the plan names no backup step. Add a down migration that re-adds the
  column (`ALTER TABLE users ADD COLUMN legacy_email TEXT;`), and either copy
  the data out first (e.g. into an archive table, or a documented
  `pg_dump` of the column) before dropping it, or state in the plan why the
  data can be lost and how to recover it.
history:
  - status: open
    by: migration
```

### newcomer

Disabled by configuration.
