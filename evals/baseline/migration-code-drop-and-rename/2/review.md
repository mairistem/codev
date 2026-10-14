# Review record: drop-legacy-email

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### migration

#### R-migration-1 Irreversible column drop with no down migration or backup step

```yaml
severity: high
confidence: high
location: "migrations/0005_drop_legacy_email.sql:1-2"
evidence: |
  -- Drop the column left from the 2023 import.
  ALTER TABLE users DROP COLUMN legacy_email;
recommendation: >-
  Running migration 0005 on production permanently deletes every value still
  stored in `users.legacy_email` (rows from the 2023 import); unlike 0004, which
  ships `0004_accounts.down.sql`, 0005 has no `0005_drop_legacy_email.down.sql`,
  and neither the proposal nor tasks.md plans a backup or says why the data is
  not needed, so a rollback cannot restore it. Add a backup step before the drop
  (e.g. copy `id, legacy_email` to an archive table or export), add a down
  migration that re-adds the nullable column (and restores from the backup if
  kept), or state in the plan why the data is disposable and how to recover.
history:
  - status: open
    by: migration
```

#### R-migration-2 Public JSON field renamed user_name → userName with no BREAKING mark or transition

```yaml
severity: high
confidence: high
location: "src/api.rs:8-9"
evidence: |
      #[serde(rename = "userName")]
      pub user_name: String,
recommendation: >-
  `Account` is a type of the public HTTP API; every existing client that reads
  `user_name` from the account endpoint gets a missing field (null/undefined or
  a deserialization error) as soon as this ships. The proposal's release note
  lists it only as "Changed: Account JSON uses camelCase." — not marked
  **BREAKING**, with no instruction for consumers and no deprecation period.
  Mark it **BREAKING** in the release note with what clients must do, and give
  consumers time: serialize both keys for a release (e.g. a deprecated
  `user_name` alongside `userName`) or version the endpoint, then remove the
  old key later.
history:
  - status: open
    by: migration
```

### newcomer

Disabled by configuration.
