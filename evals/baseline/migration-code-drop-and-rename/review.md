# Review record: drop-legacy-email

<!-- codev-review: 1 -->

## code.review

### migration

#### R-migration-1 Irreversible DROP COLUMN with no down migration or backup

```yaml
severity: high
confidence: high
location: "migrations/0005_drop_legacy_email.sql:1-2"
evidence: |
  -- Drop the column left from the 2023 import.
  ALTER TABLE users DROP COLUMN legacy_email;
recommendation: Add `migrations/0005_drop_legacy_email.down.sql` (re-adding `legacy_email TEXT`), as 0004 has a down file, and add a backup step (e.g. copy `id, legacy_email` to an archive table or a dump) before the drop; or state in the plan why the data can be lost and how to recover it.
history:
  - status: open
    by: migration
```

Migration 0004 ships `0004_accounts.down.sql`; 0005 has no down counterpart, and nothing in the change saves the existing `legacy_email` values before they are destroyed. "Not written since 2023" does not mean the old values are unread or worthless.

#### R-migration-2 Account JSON field rename is a breaking API change not marked BREAKING

```yaml
severity: high
confidence: high
location: "src/api.rs:8-9"
evidence: |
      #[serde(rename = "userName")]
      pub user_name: String,
recommendation: Mark the rename **BREAKING** in the release note with what consumers must do (read `userName` instead of `user_name`), and treat it as a major version bump of the public API.
history:
  - status: open
    by: migration
```

`src/api.rs` documents these as "The JSON types of the public HTTP API". Every consumer reading `user_name` from the account response silently gets a missing field. The proposal's release note only says "Changed: Account JSON uses camelCase." — no **BREAKING** marker and no consumer action.

#### R-migration-3 Field removed in the same release as its replacement, no transition period

```yaml
severity: medium
confidence: high
location: "src/api.rs:8"
evidence: |
      #[serde(rename = "userName")]
recommendation: Expand then contract — emit both `user_name` (deprecated) and `userName` for at least one release (e.g. a second serialized field or a custom `Serialize`), announce the removal date, then drop `user_name` in a later change.
history:
  - status: open
    by: migration
```

The old key disappears in the very release that introduces the new one, so consumers get no time to migrate, and during a rolling deploy old and new instances return different field names for the same endpoint.

### newcomer

Disabled by configuration.
