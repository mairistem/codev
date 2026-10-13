# Review record: drop-legacy-email

<!-- codev-review: 1 -->

## code.review

### migration

#### R-migration-1 Account JSON field rename is an unmarked breaking change

```yaml
severity: high
confidence: high
location: "src/api.rs:8-9"
evidence: |
    #[serde(rename = "userName")]
    pub user_name: String,
recommendation: >-
  Every existing client of the public HTTP API that reads `user_name` from an
  Account response finds the key missing after deploy and gets null/undefined
  or a deserialization error, with no warning. Mark it **BREAKING** in the
  release note (proposal.md#Release note only says "Changed: Account JSON uses
  camelCase.") with what clients must do, or keep the old key readable during a
  deprecation window (e.g. serialize both `user_name` and `userName`, or
  version the endpoint) before removing `user_name`.
history:
  - status: open
    by: migration
```

#### R-migration-2 DROP COLUMN legacy_email is irreversible with no down migration or backup

```yaml
severity: high
confidence: high
location: "migrations/0005_drop_legacy_email.sql:2"
evidence: |
  ALTER TABLE users DROP COLUMN legacy_email;
recommendation: >-
  Running this migration destroys every value still stored in
  `users.legacy_email` (rows from the 2023 import), and unlike
  `0004_accounts.sql` it ships no `0005_drop_legacy_email.down.sql`, so a
  rollback cannot restore the column or its data. Add a down migration that
  re-adds `legacy_email TEXT`, and a backup step (e.g. copy `id, legacy_email`
  to an archive table or dump) before the drop — or state in the plan why the
  data is disposable and how to recover.
history:
  - status: open
    by: migration
```

### newcomer

Disabled by configuration.
