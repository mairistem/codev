# Review record: drop-legacy-email

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### migration

#### R-migration-1 Account JSON field renamed with no BREAKING mark or transition

```yaml
severity: high
confidence: high
location: "src/api.rs:8-9"
evidence: |
  #[serde(rename = "userName")]
  pub user_name: String,
recommendation: >-
  Every existing consumer of the public HTTP API that reads `user_name` from an
  Account response gets a missing field after deploy (deserialization error or
  silent empty value) with no warning. Mark the rename **BREAKING** in the
  proposal's release note, with what consumers must do (read `userName`), and
  give them time: serve both names for one release (e.g. a deprecated
  `user_name` alongside `userName`) or version the endpoint, then remove the old
  name in a later change.
history:
  - status: open
    by: migration
```

#### R-migration-2 Irreversible DROP COLUMN with no down migration or backup step

```yaml
severity: high
confidence: high
location: "migrations/0005_drop_legacy_email.sql:2"
evidence: |
  ALTER TABLE users DROP COLUMN legacy_email;
recommendation: >-
  Running 0005 on production deletes every value still held in
  `users.legacy_email` (rows imported in 2023), and there is no
  `0005_drop_legacy_email.down.sql` although 0004 has one, so a rollback cannot
  restore the column or its data. Add a down migration that re-adds the nullable
  column, and add a backup step before the drop (copy the column to an archive
  table or take a dump), or state in the plan why the data can be lost and how
  to recover.
history:
  - status: open
    by: migration
```

### newcomer

Disabled by configuration.
