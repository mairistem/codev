# Review record: add-account-deletion

<!-- codev-review: 1 -->

## code.review

### privacy

#### R-privacy-1 Account deletion keeps the member's email, name, phone and postal address

```yaml
severity: high
confidence: high
location: "app/account.py:18-21"
evidence: |
  "UPDATE users SET deleted_at = datetime('now'), password_hash = NULL WHERE id = ?",
recommendation: >-
  The proposal promises that the button "erases the member's personal data",
  yet delete_account only sets deleted_at and clears password_hash: after a
  member deletes their account, the users row still holds email,
  display_name, phone and postal_address (app/schema.sql:3-6) indefinitely,
  with no purge. A member exercising their right to erasure (GDPR Art. 17)
  keeps all their contact data in the users table, and the orders kept for
  accounting still point at it through orders.user_id, so they are not
  "without personal data" either. Overwrite or null the personal columns in
  the same UPDATE (e.g. email replaced by a non-identifying tombstone such as
  'deleted-<id>' to satisfy NOT NULL UNIQUE, display_name by a placeholder,
  phone and postal_address set to NULL), and add a test asserting those
  columns no longer hold the member's data. State how long backups keep the
  erased data before it expires.
history:
  - status: open
    by: privacy
```

#### R-privacy-2 New analytics identifier added with no use and outside the erasure

```yaml
severity: low
confidence: low
location: "app/account.py:11-13"
evidence: |
  def analytics_id(user_id: int) -> str:
      """The pseudonymous id analytics events carry instead of the user id."""
      return hashlib.sha256(f"{_SALT}:{user_id}".encode()).hexdigest()
recommendation: >-
  The change adds a per-member analytics identifier that nothing in the
  change calls and that the proposal does not mention, and delete_account
  does nothing about analytics events already keyed by it: with a fixed salt
  the id stays linkable to the member after deletion. Either drop it from
  this change, or state its purpose and processor and make account deletion
  reach the analytics data. Confidence is low because the callers and the
  analytics pipeline are not visible in this diff.
history:
  - status: open
    by: privacy
```
