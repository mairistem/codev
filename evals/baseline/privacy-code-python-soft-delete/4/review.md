# Review record: add-account-deletion

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### privacy

#### R-privacy-1 Account deletion keeps the member's e-mail, name, phone and postal address

```yaml
severity: high
confidence: high
location: "app/account.py:18-21"
evidence: |
  "UPDATE users SET deleted_at = datetime('now'), password_hash = NULL WHERE id = ?",
recommendation: >-
  The proposal promises that the button "erases the member's personal data" (GDPR Art. 17, Art. 5(1)(e)),
  but delete_account only soft-deletes: it sets deleted_at and clears password_hash, leaving
  users.email, display_name, phone and postal_address (app/schema.sql:3-6) in the table forever.
  Scenario: a member clicks "Delete my account"; their row still holds 'ada@example.org', 'Ada',
  their phone and address, readable by anyone with database access and in every later export.
  Erase or overwrite those columns in the same UPDATE (email needs a unique placeholder such as
  'deleted-<id>@invalid' because of its NOT NULL UNIQUE constraint, display_name a non-null
  placeholder, phone and postal_address NULL), so the orders kept for accounting point at a row
  with no personal data. Add a test asserting the columns are gone after delete_account: the
  current tests only check sign-in and sessions, which a soft delete passes.
history:
  - status: open
    by: privacy
```

#### R-privacy-2 Erasure of backups and analytics copies is not defined

```yaml
severity: medium
confidence: low
location: "app/account.py:16-23"
evidence: |
  def delete_account(user_id: int) -> None:
      """Deletes the member's account; their orders are kept for accounting."""
recommendation: >-
  Nothing in the change or its proposal says how long database backups keep the deleted
  member's data, nor whether the analytics events keyed by analytics_id (app/account.py:11-13)
  are purged or detached when the account is deleted. The backup policy and the analytics store
  are not visible in the repository, so this is a question for the team (GDPR Art. 17, Art. 5(1)(e)):
  state the backup retention period after which the erased data disappears, and either purge the
  member's analytics events on deletion or confirm they hold nothing that re-identifies the member.
history:
  - status: open
    by: privacy
```

#### R-privacy-3 analytics_id is added with no use in the change and is only pseudonymous

```yaml
severity: low
confidence: low
location: "app/account.py:8-13"
evidence: |
  _SALT = os.environ["ANALYTICS_SALT"]
recommendation: >-
  analytics_id is not called by delete_account nor by anything else in the change, and the
  proposal does not mention analytics, so its purpose is not stated (GDPR Art. 5(1)(b)-(c)).
  Being a deterministic salted hash of the user id, it stays linkable to the member as long as
  the salt exists: it is pseudonymous data, not anonymous, and remains personal data after the
  account is deleted. Either drop it from this change or state its purpose in the proposal and
  cover it in the erasure (see R-privacy-2).
history:
  - status: open
    by: privacy
```

### newcomer

Disabled by configuration.
