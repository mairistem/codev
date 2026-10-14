# Review record: add-account-deletion

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### privacy

#### R-privacy-1 Account deletion keeps the member's personal data in the users row

```yaml
severity: high
confidence: high
location: "app/account.py:18-21"
evidence: |
  _conn.execute(
      "UPDATE users SET deleted_at = datetime('now'), password_hash = NULL WHERE id = ?",
      (user_id,),
  )
recommendation: >
  The proposal promises that the button "erases the member's personal data", and the
  member exercises the right to erasure (GDPR Art. 17), but delete_account only sets
  deleted_at and clears password_hash: email, display_name, phone and postal_address
  (app/schema.sql) stay in the users row indefinitely. Scenario: a member clicks
  "Delete my account"; afterwards SELECT email, display_name, phone, postal_address
  FROM users WHERE id = ? still returns all four values. The orders kept for
  accounting reference users.id, so they stay linked to the member's identity, contrary
  to "kept for accounting, without personal data". Overwrite the personal columns in the
  same UPDATE — display_name, phone and postal_address to NULL or a placeholder, email
  to a unique non-personal value such as 'deleted-<id>' (the column is NOT NULL UNIQUE,
  which also frees the address for a later sign-up) — and add a test asserting those
  columns no longer hold the member's data after delete_account.
history:
  - status: open
    by: privacy
```

#### R-privacy-2 The new analytics id stays linkable after the account is deleted

```yaml
severity: medium
confidence: low
location: "app/account.py:8-13"
evidence: |
  _SALT = os.environ["ANALYTICS_SALT"]


  def analytics_id(user_id: int) -> str:
      """The pseudonymous id analytics events carry instead of the user id."""
      return hashlib.sha256(f"{_SALT}:{user_id}".encode()).hexdigest()
recommendation: >
  analytics_id is added by this change but nothing in the change uses it, and the plan
  states no purpose for it (GDPR Art. 5(1)(b)-(c)). It is a pseudonym, not an anonymous
  value: with one global salt it is a deterministic function of users.id, which the
  change keeps (the users row and the orders that reference it), so analytics events
  carrying it stay re-identifiable after the member deletes the account, and
  delete_account does nothing to the analytics data. The analytics pipeline is not
  visible here, hence confidence low. Either drop analytics_id from this change, or
  state its purpose and make erasure reach it — delete or detach the member's events,
  or derive the id from a per-user random value that delete_account destroys.
history:
  - status: open
    by: privacy
```

#### R-privacy-3 Erasure from backups and the retention of deleted rows are not defined

```yaml
severity: medium
confidence: low
location: "app/account.py:16-23"
evidence: |
  def delete_account(user_id: int) -> None:
      """Deletes the member's account; their orders are kept for accounting."""
recommendation: >
  Neither the plan nor the code says how long a deleted users row is kept, nor how the
  erasure reaches backups of members.db (GDPR Art. 5(1)(e), Art. 17). The backup setup
  is not visible here, hence confidence low. State the retention of soft-deleted rows
  (or purge them after a set delay) and how backups expire or are scrubbed, so that an
  erased member's data does not survive a restore.
history:
  - status: open
    by: privacy
```
