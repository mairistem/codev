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
recommendation: >-
  The proposal promises that the button "erases the member's personal data" and
  keeps orders "without personal data", but delete_account only soft-deletes:
  it sets deleted_at and clears password_hash, leaving email, display_name,
  phone and postal_address (app/schema.sql:3-6) in the users row. After a member
  clicks "Delete my account", every one of those fields is still stored and
  still joined to their orders through orders.user_id — the right to erasure
  (GDPR Art. 17) and storage limitation (Art. 5(1)(e)) are not met. Overwrite or
  null those columns in the same UPDATE (e.g. email set to a unique opaque
  placeholder such as 'deleted-<id>' since the column is NOT NULL UNIQUE, which
  also frees the address for a new sign-up; display_name to a placeholder;
  phone and postal_address to NULL), so the kept orders point at a row with no
  personal data. Add a test asserting these columns no longer hold the member's
  values, and state how backups are purged or expire, since the plan says
  nothing about them.
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
  analytics_id is added with no caller in the change, and is a deterministic
  salted hash of the user id: whoever holds ANALYTICS_SALT can re-link events to
  the member, so they are pseudonymous, still personal data (GDPR recital 26).
  delete_account does nothing about events already carrying this id. The
  analytics store was not visible to the reviewer; the team should state whether
  those events are erased, aggregated or expire on deletion, or drop the
  function from this change if it is unused.
history:
  - status: open
    by: privacy
```

### newcomer

Disabled by configuration.
