# Review record: drop-legacy-email

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### migration

#### R-migration-1 Public JSON field renamed without a BREAKING mark or transition

```yaml
severity: high
confidence: high
location: "src/api.rs:8-9"
evidence: |
    #[serde(rename = "userName")]
    pub user_name: String,
recommendation: >-
  The public account JSON (src/api.rs is "The JSON types of the public HTTP API")
  now emits `userName` instead of `user_name`; every existing client that reads
  `user_name` gets a missing field after deploy, with no warning. The release note
  lists it as "Changed: Account JSON uses camelCase." with no **BREAKING** marker
  and no instruction for consumers. Mark it **BREAKING** in the release note with
  what clients must do, and give consumers a transition: emit both names for a
  deprecation period (or version the endpoint), then drop `user_name` in a later
  release; add a test asserting the serialized field names.
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
  Running migration 0005 destroys every value of `users.legacy_email` left from
  the 2023 import; there is no `0005_drop_legacy_email.down.sql` (0004 has one)
  and the plan names no backup or recovery, so a rollback cannot restore the data.
  Add a backup step (copy the column to an archive table or export it) before the
  drop, add a down migration that re-adds the column and restores from the
  backup, or state in the plan why the data can be lost and how to recover. Also
  ship the drop only once no running instance still selects the column
  (expand/contract order).
history:
  - status: open
    by: migration
```

### newcomer

Disabled by configuration.
