# Review record: drop-legacy-email

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### migration

#### R-migration-1 Account JSON field renamed with no BREAKING mark and no path for consumers

```yaml
severity: high
confidence: high
location: "src/api.rs:8-9"
evidence: |
  #[serde(rename = "userName")]
  pub user_name: String,
recommendation: >-
  The public HTTP API's `Account` JSON now emits `userName` instead of
  `user_name` (src/api.rs is "The JSON types of the public HTTP API").
  Every existing client reading `user_name` gets a missing field (undefined,
  or a deserialization error in strict clients) on the first response after
  deploy, with no warning: the proposal's release note lists it as
  "Changed: Account JSON uses camelCase." with no **BREAKING** item and no
  action for users. Mark it **BREAKING** in the release note with what
  clients must do, and give consumers a transition: emit both `user_name`
  and `userName` for a deprecation period (or version the endpoint), then
  remove `user_name` in a later release.
history:
  - status: open
    by: migration
```

#### R-migration-2 Irreversible DROP COLUMN with no down migration and no backup step

```yaml
severity: high
confidence: high
location: "migrations/0005_drop_legacy_email.sql:2"
evidence: |
  ALTER TABLE users DROP COLUMN legacy_email;
recommendation: >-
  Running migration 0005 deletes every value of `users.legacy_email` (the
  data from the 2023 import, per the migration's comment) with no way back:
  there is no `0005_drop_legacy_email.down.sql`, while 0004 has one, and
  neither the proposal nor tasks.md mention a backup or a recovery plan.
  A rollback after deploy cannot restore the column or its data. Before
  dropping, export the column (e.g. copy `id, legacy_email` to an archive
  table or dump), add a down migration that re-adds `legacy_email TEXT`
  (and restores from the archive if kept), or state in the plan why the
  data can be lost and how to recover.
history:
  - status: open
    by: migration
```

### newcomer

Disabled by configuration.
