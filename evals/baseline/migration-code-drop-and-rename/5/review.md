# Review record: drop-legacy-email

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### migration

#### R-migration-1 Renamed public JSON field breaks existing API consumers without warning

```yaml
severity: high
confidence: high
location: "src/api.rs:8"
evidence: |
  #[serde(rename = "userName")]
recommendation: >-
  `Account` is a type of the public HTTP API: every existing client that reads
  `user_name` from the account endpoint gets a missing field after deploy
  (null/undefined, or a deserialization error for strict clients). Mark the
  change **BREAKING** in the proposal's release note with what clients must do
  (read `userName`), and give consumers a transition: e.g. emit both keys for
  a deprecation period (keep `user_name` alongside `userName`, or version the
  endpoint) before removing the old one. The release note line
  "Changed: Account JSON uses camelCase." is neither marked BREAKING nor says
  what users must do.
history:
  - status: open
    by: migration
```

#### R-migration-2 DROP COLUMN is irreversible, with no down migration and no backup step

```yaml
severity: high
confidence: high
location: "migrations/0005_drop_legacy_email.sql:2"
evidence: |
  ALTER TABLE users DROP COLUMN legacy_email;
recommendation: >-
  Running migration 0005 deletes every value still stored in
  `users.legacy_email` (the 2023 import data); the column is unwritten, not
  empty. Unlike `0004_accounts.down.sql`, there is no
  `0005_drop_legacy_email.down.sql`, and neither the proposal nor tasks.md
  plans a backup or explains how to recover. Add a down migration that
  re-creates the column (`ALTER TABLE users ADD COLUMN legacy_email TEXT;`),
  and a backup step (copy the column to an archive table or dump it) before
  the drop — or state in the plan why the data can be lost and how to
  recover it.
history:
  - status: open
    by: migration
```

#### R-migration-3 Column dropped in the same release as code, while old instances may still read it

```yaml
severity: medium
confidence: low
location: "migrations/0005_drop_legacy_email.sql:2"
evidence: |
  ALTER TABLE users DROP COLUMN legacy_email;
recommendation: >-
  No reader of `legacy_email` is visible in this repository, but the plan does
  not say that no running service, report or `SELECT *` consumer still reads
  it. If one does, it fails as soon as the migration runs during a rolling
  deploy. Confirm no consumer reads the column (or stop reading it in a
  release before the one that drops it — expand/contract), and note the
  rollout order in the plan.
history:
  - status: open
    by: migration
```

### newcomer

Disabled by configuration.
