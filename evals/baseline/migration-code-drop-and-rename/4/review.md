# Review record: drop-legacy-email

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### migration

#### R-migration-1 Account JSON field renamed without a BREAKING mark

```yaml
severity: high
confidence: high
location: "src/api.rs:8-9"
evidence: |
  #[serde(rename = "userName")]
  pub user_name: String,
recommendation: >-
  The public Account JSON field `user_name` becomes `userName`: every existing
  API consumer reading `user_name` gets a missing field (null/undefined or a
  deserialization error) as soon as this version is deployed, with no warning.
  The proposal's release note lists it only as "Changed: Account JSON uses
  camelCase." Mark it **BREAKING** in the release note with what consumers must
  do, and give them a transition: serialize both `user_name` and `userName`
  for a deprecation period (e.g. a second getter field or a custom Serialize),
  then remove `user_name` in a later release — so old and new consumers work
  side by side during rollout.
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
  Running migration 0005 deletes every value still stored in
  `users.legacy_email` (rows imported in 2023 keep their data even if the
  column is no longer written), and unlike 0004 (`0004_accounts.down.sql`)
  there is no `0005_drop_legacy_email.down.sql`, no backup step and no word in
  the plan on how to recover. Add a backup step (e.g. copy `id, legacy_email`
  into an archive table or export before dropping) and a down migration that
  re-adds the column (restoring from the backup), or state in the plan why the
  data is disposable and how to recover; also list the column removal in the
  release note.
history:
  - status: open
    by: migration
```

### newcomer

Disabled by configuration.
