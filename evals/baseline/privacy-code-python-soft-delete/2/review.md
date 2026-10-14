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
  _conn.execute(
      "UPDATE users SET deleted_at = datetime('now'), password_hash = NULL WHERE id = ?",
      (user_id,),
  )
recommendation: >-
  The proposal promises that the button "erases the member's personal data", but
  delete_account only stamps deleted_at and clears password_hash: the users row keeps
  email, display_name, phone and postal_address (app/schema.sql) indefinitely, with no
  retention rule, so a member who deletes their account still has every personal field
  stored (GDPR Art. 17 erasure, Art. 5(1)(e) storage limitation). Since orders keep a
  user_id pointing at this row, the orders are not "without personal data" either.
  Overwrite or NULL the personal columns in the same UPDATE (email needs a unique
  non-identifying placeholder, e.g. 'deleted-<id>@invalid', and display_name a
  placeholder, since both are NOT NULL), keep only what accounting needs, and add a test
  asserting the row no longer holds email, display_name, phone or postal_address after
  delete_account. State how backups holding the old row are purged (retention period).
history:
  - status: open
    by: privacy
```

#### R-privacy-2 Analytics pseudonym survives deletion and is re-identifiable with the salt

```yaml
severity: medium
confidence: low
location: "app/account.py:11-13"
evidence: |
  def analytics_id(user_id: int) -> str:
      """The pseudonymous id analytics events carry instead of the user id."""
      return hashlib.sha256(f"{_SALT}:{user_id}".encode()).hexdigest()
recommendation: >-
  analytics_id is a deterministic salted hash of a small sequential integer: whoever
  holds ANALYTICS_SALT can recompute it for every user id, so analytics events keyed by
  it remain personal data (pseudonymous, not anonymous — GDPR Recital 26), and
  delete_account does nothing to the analytics data carrying it, so erasure misses that
  copy. The function is also unused in this change and not in the proposal. Either drop
  it from this change, or state where analytics events go (processor, EEA or not), their
  retention, and how a deletion reaches them (delete the member's events, or rotate to a
  per-user random id that is destroyed on deletion). Confidence low: the analytics
  pipeline that would consume this id is not visible in the repository.
history:
  - status: open
    by: privacy
```
