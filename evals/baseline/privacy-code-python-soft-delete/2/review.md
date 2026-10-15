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
  The proposal promises that the button "erases the member's personal data" and that orders are kept
  "without personal data", but delete_account only soft-deletes: it sets deleted_at and clears
  password_hash, leaving email, display_name, phone and postal_address (app/schema.sql:3-6) in the
  users row. After a member presses "Delete my account", their row still holds e.g.
  'ada@example.org' / 'Ada' / phone / address, and every kept order still points to it through
  orders.user_id, so the orders stay linked to identifying data — the right to erasure (GDPR Art. 17)
  and storage limitation (Art. 5(1)(e)) are not met. Overwrite or NULL every personal column in the
  same UPDATE (e.g. email replaced by a unique non-identifying tombstone such as 'deleted-<id>' to
  satisfy NOT NULL UNIQUE, display_name by a placeholder, phone and postal_address by NULL), and add
  a test asserting that no personal column of the deleted row still holds the original values.
history:
  - status: open
    by: privacy
```

#### R-privacy-2 Erasure does not reach analytics events keyed by analytics_id

```yaml
severity: medium
confidence: low
location: "app/account.py:11-13"
evidence: |
  def analytics_id(user_id: int) -> str:
      """The pseudonymous id analytics events carry instead of the user id."""
      return hashlib.sha256(f"{_SALT}:{user_id}".encode()).hexdigest()
recommendation: >-
  analytics_id is a deterministic, salted hash of the user id: it is pseudonymous, not anonymous, so
  events carrying it remain personal data as long as the salt and the users table exist
  (GDPR Recital 26). delete_account neither deletes those events nor breaks the link (for example
  with a per-user salt destroyed on deletion), so the analytics copies survive the erasure. The
  analytics store and its callers are not visible in this repository; confirm where these events go
  (and whether that processor is outside the EEA) and make account deletion reach them.
history:
  - status: open
    by: privacy
```

#### R-privacy-3 No retention or purge for the soft-deleted row and its backups

```yaml
severity: medium
confidence: low
location: "app/account.py:16-23"
evidence: |
  def delete_account(user_id: int) -> None:
      """Deletes the member's account; their orders are kept for accounting."""
recommendation: >-
  The change keeps a tombstone users row (deleted_at) forever and states no retention period for it
  nor for the copies of the deleted member's data in database backups of members.db. Define how long
  the tombstone and backups keep the data and how they are purged (GDPR Art. 5(1)(e)); the backup
  policy is not visible in the repository, so this is a question for the team.
history:
  - status: open
    by: privacy
```

### newcomer

Disabled by configuration.
