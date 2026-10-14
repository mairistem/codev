# Review record: drop-legacy-email

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### migration

#### R-migration-1 Irreversible column drop with no down migration or backup

```yaml
severity: high
confidence: high
location: "migrations/0005_drop_legacy_email.sql:2"
evidence: |
  ALTER TABLE users DROP COLUMN legacy_email;
recommendation: >-
  Running migration 0005 on production permanently deletes every value in
  users.legacy_email. There is no 0005_drop_legacy_email.down.sql (unlike
  0004, which has 0004_accounts.down.sql), and neither the proposal nor
  tasks.md has a backup or recovery step; task 1.1 only checks that
  `sqlx migrate run` succeeds. Add a down migration that re-adds the column
  (`ALTER TABLE users ADD COLUMN legacy_email TEXT;`), and add a step that
  exports `id, legacy_email` (or snapshots the table) before the drop.
  Otherwise, say in the plan that the data is deliberately abandoned and
  how to recover it from backups. Also confirm no running instance still
  reads the column before the drop ships (contract only after the code no
  longer uses it).
history:
  - status: open
    by: migration
```

#### R-migration-2 Account JSON field renamed to userName without a BREAKING mark or compatibility path

```yaml
severity: high
confidence: high
location: "src/api.rs:8-9"
evidence: |
  #[serde(rename = "userName")]
  pub user_name: String,
recommendation: >-
  `Account` is a type of the public HTTP API (src/api.rs: "The JSON types of
  the public HTTP API."). Every existing client that reads `user_name` from
  the account endpoint gets a missing field after deploy: it reads
  undefined/null, or fails to deserialize. The proposal's release note only
  says "Changed: Account JSON uses camelCase." with no **BREAKING** mark and
  nothing on what clients must do, and no deprecation period. Mark the item
  **BREAKING** in the release note with the old and new name and the
  required client change. Give consumers time: emit both `user_name` and
  `userName` for one release (e.g. a second serialized field or a custom
  Serialize), deprecate `user_name`, then remove it, or version the
  endpoint.
history:
  - status: open
    by: migration
```

### newcomer

Disabled by configuration.
