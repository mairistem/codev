# Review record: drop-legacy-email

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### migration

#### R-migration-1 JSON field `user_name` renamed `userName` without a BREAKING mark or transition

```yaml
severity: high
confidence: high
location: "src/api.rs:8-9"
evidence: |
  #[serde(rename = "userName")]
  pub user_name: String,
recommendation: >-
  Every existing consumer of the public HTTP API that reads `user_name` from an
  Account response gets a missing field (undefined / deserialization error) as
  soon as this ships, with no warning: the proposal's release note lists it as
  "Changed: Account JSON uses camelCase." with no **BREAKING** item and no
  instruction for consumers. Mark it **BREAKING** in the release note with what
  consumers must do, and give them a transition: serialize both `user_name` and
  `userName` for a deprecation period (e.g. a second serialized field or a
  custom Serialize), announce the removal date, then drop `user_name` in a later
  release.
history:
  - status: open
    by: migration
```

#### R-migration-2 Irreversible `DROP COLUMN legacy_email` with no down migration or backup

```yaml
severity: high
confidence: high
location: "migrations/0005_drop_legacy_email.sql:2"
evidence: |
  ALTER TABLE users DROP COLUMN legacy_email;
recommendation: >-
  Running migration 0005 permanently deletes every stored `users.legacy_email`
  value; the project pairs migrations with a down file
  (`migrations/0004_accounts.down.sql`), but 0005 has none, and neither the
  proposal nor tasks.md states a backup or recovery plan, so a rollback cannot
  restore the data. Before dropping, copy the column to an archive table (or
  take and document a backup), add `0005_drop_legacy_email.down.sql` that
  re-adds the column (and restores from the archive), or state in the plan why
  the data is disposable and how to recover.
history:
  - status: open
    by: migration
```

### newcomer

Disabled by configuration.
