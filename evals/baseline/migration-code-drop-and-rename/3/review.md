# Review record: drop-legacy-email

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### migration

#### R-migration-1 Public JSON field `user_name` renamed `userName` without a BREAKING mark or transition

```yaml
severity: high
confidence: high
location: "src/api.rs:8-9"
evidence: |
  #[serde(rename = "userName")]
  pub user_name: String,
recommendation: >-
  Every existing client of the public HTTP API that reads `user_name` from an
  Account response gets a missing field (undefined / deserialization error)
  as soon as this ships, with no warning: the release note only says
  "Changed: Account JSON uses camelCase." Mark it **BREAKING** in the release
  note and say what clients must do (read `userName`). Give consumers time:
  serialize both keys during a deprecation window (e.g. a second
  `#[serde(rename = "user_name")]` field or a custom Serialize emitting both),
  announce the removal date of `user_name`, or version the endpoint; add a
  test that pins the response keys.
history:
  - status: open
    by: migration
```

#### R-migration-2 Irreversible `DROP COLUMN legacy_email` with no down migration, backup step or release note

```yaml
severity: high
confidence: high
location: "migrations/0005_drop_legacy_email.sql:2"
evidence: |
  ALTER TABLE users DROP COLUMN legacy_email;
recommendation: >-
  Running migration 0005 permanently deletes every value still stored in
  `users.legacy_email` (rows imported in 2023 keep it, per
  `migrations/0004_accounts.sql`), and there is no
  `0005_drop_legacy_email.down.sql` while 0004 has one, so a rollback cannot
  restore the column or its data. Add a down migration
  (`ALTER TABLE users ADD COLUMN legacy_email TEXT;`) and a backup/export step
  of the column before the drop (or a copy to an archive table), or state in
  the plan why the data is disposable and how to recover. Check that no
  deployed instance or external consumer still reads the column before
  dropping it (contract step after the code stops using it), and list the
  removal in the release note, which currently omits it.
history:
  - status: open
    by: migration
```

### newcomer

Disabled by configuration.
