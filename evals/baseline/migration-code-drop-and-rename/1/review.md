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
  Every existing `users` row loses its `legacy_email` value when 0005 runs, and
  nothing can bring it back: unlike `0004_accounts.sql`, which ships
  `0004_accounts.down.sql`, there is no `0005_drop_legacy_email.down.sql`, and
  the plan has no backup step. "Not written since 2023" does not mean the
  values are no longer needed. Copy the column out first (for example
  `CREATE TABLE users_legacy_email AS SELECT id, legacy_email FROM users WHERE
  legacy_email IS NOT NULL;`), add a down migration that re-adds the column
  and restores it from that copy, or say in the plan why the data can be lost
  and how to recover it.
history:
  - status: open
    by: migration
```

#### R-migration-2 Public JSON field renamed user_name to userName with no BREAKING mark or compatibility

```yaml
severity: high
confidence: high
location: "src/api.rs:8-9"
evidence: |
      #[serde(rename = "userName")]
      pub user_name: String,
recommendation: >-
  `Account` is a type of the public HTTP API ("The JSON types of the public
  HTTP API"). Before this change, the account endpoint returned `user_name`.
  After it, the endpoint returns only `userName`, so every existing client
  that reads `user_name` gets a missing field or a deserialization error
  without warning. The proposal's release note lists it only as "Changed:
  Account JSON uses camelCase.": it has no **BREAKING** item and does not say
  what consumers must do. Also, no deprecation period is planned. Mark it
  **BREAKING** in the release note and give the migration step. Better, keep
  old consumers working during a deprecation window: serialize both names
  (for example a second `#[serde(rename = "user_name")]` field, or a
  versioned endpoint), then remove `user_name` in a later release.
history:
  - status: open
    by: migration
```

### newcomer

Disabled by configuration.
