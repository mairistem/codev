# Review record: drop-legacy-email

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### migration

#### R-migration-1 Account field renamed in the public API without a BREAKING mark or a transition

```yaml
severity: high
confidence: high
location: "src/api.rs:8-9"
evidence: |
  #[serde(rename = "userName")]
  pub user_name: String,
recommendation: >-
  `Account` is a type of the public HTTP API (src/api.rs:1); every existing client
  that reads `user_name` from the account endpoint gets a missing field after deploy,
  with no warning. Mark it **BREAKING** in the proposal's release note (today it reads
  "Changed: Account JSON uses camelCase.") with what clients must do, and give consumers
  time: serialize both `user_name` (deprecated) and `userName` for a release, or version
  the endpoint, before removing the snake_case field.
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
  Running 0005 on production deletes every stored `legacy_email` value for good: there
  is no `0005_drop_legacy_email.down.sql` (0004 has one), and neither the plan nor the
  tasks back up the column. Add a backup step (copy `id, legacy_email` to an archive
  table or dump) and a down migration that re-adds the column and restores it, or state
  in the plan why the data can be lost and how to recover.
history:
  - status: open
    by: migration
```

### newcomer

Disabled by configuration.
