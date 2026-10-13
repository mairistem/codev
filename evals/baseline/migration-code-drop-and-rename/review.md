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
recommendation: >-
  Add `migrations/0005_drop_legacy_email.down.sql` (re-adding `legacy_email TEXT`,
  as 0004 has its own `.down.sql`) and, since the down cannot restore the values,
  copy the column's data to a backup table or export before dropping it — or state
  in the plan why the data is disposable and how to recover. "Not written since
  2023" does not mean empty: existing rows still hold the imported emails.
history:
  - status: open
    by: migration
```

#### R-migration-2 Public JSON field renamed with no BREAKING mark

```yaml
severity: high
confidence: high
location: "src/api.rs:8-9"
evidence: |
      #[serde(rename = "userName")]
      pub user_name: String,
recommendation: >-
  `Account` is a public HTTP API type: its `user_name` field becomes `userName`,
  so every API consumer reading `user_name` breaks silently. Mark the item
  **BREAKING** in the proposal's release note, with what consumers must do
  (read `userName` instead), and treat it as a major version change.
history:
  - status: open
    by: migration
```

#### R-migration-3 Release note announces the rename as a plain change

```yaml
severity: medium
confidence: high
location: "proposal.md#Release note"
evidence: |
  - Changed: Account JSON uses camelCase.
recommendation: >-
  Write it as a **BREAKING** item naming the field (`user_name` → `userName`) and
  the consumer action; also add an item for the dropped `users.legacy_email`
  column, which the release note does not mention.
history:
  - status: open
    by: migration
```

#### R-migration-4 Rename shipped without a deprecation or side-by-side period

```yaml
severity: medium
confidence: high
location: "src/api.rs:8"
evidence: |
      #[serde(rename = "userName")]
recommendation: >-
  Consumers get no time to move: the old key disappears in the same release that
  introduces the new one. Expand then contract — serialize both `user_name` and
  `userName` for a deprecation period (e.g. a second field or a custom
  `Serialize`), announce the removal of `user_name`, then drop it in a later
  release. Add a test asserting the serialized keys.
history:
  - status: open
    by: migration
```

#### R-migration-5 No test covers the migration path

```yaml
severity: low
confidence: low
location: "tasks.md#1. Implementation"
evidence: |
  - [x] 1.1 Migration 0005, verified by `sqlx migrate run` on a copy of production
  - [x] 1.2 camelCase account field, verified by `cargo test api`
recommendation: >-
  The diff adds no test: nothing checks that 0005 can be rolled back, nor the
  serialized key names of `Account`. Add a serialization test for `Account` and
  exercise the down migration once it exists.
history:
  - status: open
    by: migration
```

### newcomer

Disabled by configuration.
