# Review record: drop-legacy-email

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### migration

#### R-migration-1 Irreversible DROP COLUMN with no down migration or backup

```yaml
severity: high
confidence: high
location: "migrations/0005_drop_legacy_email.sql:2"
evidence: |
  ALTER TABLE users DROP COLUMN legacy_email;
recommendation: >-
  Every legacy_email value still stored in production rows (the column was
  filled by the 2023 import, only no longer written) is destroyed when the
  migration runs, and nothing can bring it back: unlike
  migrations/0004_accounts.down.sql, there is no 0005 down migration, and
  neither the proposal nor tasks.md has a backup step. Add
  migrations/0005_drop_legacy_email.down.sql
  (`ALTER TABLE users ADD COLUMN legacy_email TEXT;`), and before the drop
  copy the data out (e.g. `CREATE TABLE users_legacy_email_backup AS SELECT
  id, legacy_email FROM users WHERE legacy_email IS NOT NULL;`) or state in
  the plan why the data may be lost and how to recover it.
history:
  - status: open
    by: migration
```

#### R-migration-2 JSON field user_name renamed userName with no BREAKING mark or transition

```yaml
severity: high
confidence: high
location: "src/api.rs:8"
evidence: |
  #[serde(rename = "userName")]
recommendation: >-
  Every existing consumer of the public HTTP API that reads
  `account.user_name` gets a missing field after deployment, with no
  warning: the proposal's release note lists it only as "Changed: Account
  JSON uses camelCase." with no **BREAKING** item and no instruction for
  consumers, and the change ships rename and removal in one step, so old
  clients cannot keep working side by side. Mark it **BREAKING** in the
  release note with what consumers must do, and either keep emitting the
  old field for a deprecation window (serialize both `user_name` and
  `userName`, e.g. a second deprecated field or a custom Serialize) or
  version the endpoint; `#[serde(alias)]` does not help here since it only
  affects deserialization.
history:
  - status: open
    by: migration
```

### newcomer

Disabled by configuration.
