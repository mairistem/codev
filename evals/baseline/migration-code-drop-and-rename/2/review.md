# Review record: drop-legacy-email

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### migration

#### R-migration-1 Public JSON field renamed with no BREAKING mark nor compatibility path

```yaml
severity: high
confidence: high
location: "src/api.rs:8-9"
evidence: |
    #[serde(rename = "userName")]
    pub user_name: String,
recommendation: >-
  `Account` is a type of the public HTTP API (src/api.rs:1), so every existing
  client that reads `user_name` from the account response gets a missing field
  after deploy, with no warning. Mark the rename **BREAKING** in the release
  note (it is only listed as "Changed: Account JSON uses camelCase.") with what
  clients must do, and give consumers a transition: emit both `user_name` and
  `userName` for a deprecation period (e.g. a second serialized field or a
  versioned endpoint), then remove `user_name` in a later release.
history:
  - status: open
    by: migration
```

#### R-migration-2 Irreversible DROP COLUMN with no down migration nor backup

```yaml
severity: high
confidence: high
location: "migrations/0005_drop_legacy_email.sql:2"
evidence: |
  ALTER TABLE users DROP COLUMN legacy_email;
recommendation: >-
  Running 0005 on production permanently deletes every `users.legacy_email`
  value imported in 2023 ("not written since" does not mean empty), and there
  is no `0005_drop_legacy_email.down.sql`, while 0004 ships one
  (`migrations/0004_accounts.down.sql`). Add a down migration that re-adds the
  nullable column, and a backup step before the drop (e.g. copy
  `id, legacy_email` into an archive table or export it), or state in the plan
  why the data can be lost and how to recover it.
history:
  - status: open
    by: migration
```

### newcomer

Disabled by configuration.
