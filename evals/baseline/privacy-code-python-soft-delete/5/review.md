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
  "UPDATE users SET deleted_at = datetime('now'), password_hash = NULL WHERE id = ?",
recommendation: >-
  The proposal promises that "Delete my account" erases the member's personal data (GDPR Art. 17),
  but delete_account only soft-deletes: it sets deleted_at and clears password_hash, while email,
  display_name, phone and postal_address (app/schema.sql:3-6) stay in the users table forever.
  Scenario: Ada deletes her account; SELECT email, phone, postal_address FROM users WHERE id = 1
  still returns her e-mail, phone and address. Because orders.user_id references that row, her
  kept orders also stay linked to her identity, contradicting "kept for accounting, without
  personal data". Overwrite or NULL every personal column on deletion (e.g. email to a unique
  non-identifying placeholder such as 'deleted-<id>', display_name to a placeholder, phone and
  postal_address to NULL) so the row only anchors the orders, and add a test asserting those
  columns no longer hold the member's data after delete_account. Also state how backups holding
  the old values are purged (retention of backups is not defined by the change).
history:
  - status: open
    by: privacy
```

#### R-privacy-2 Analytics pseudonym survives erasure and is not reached by deletion

```yaml
severity: medium
confidence: low
location: "app/account.py:11-13"
evidence: |
  def analytics_id(user_id: int) -> str:
      """The pseudonymous id analytics events carry instead of the user id."""
      return hashlib.sha256(f"{_SALT}:{user_id}".encode()).hexdigest()
recommendation: >-
  The change adds a deterministic pseudonymous id for analytics, derived from a global salt and
  the user id; it is pseudonymous, not anonymous, data (still personal data under GDPR), yet
  delete_account does nothing about analytics events carrying it, and anyone holding the salt can
  recompute the link to a user id still present in users and orders. The analytics pipeline was
  not visible in the change, hence low confidence. Ask the team whether analytics events keyed by
  analytics_id must be erased or unlinked on account deletion (e.g. a per-user salt deleted with
  the account), and identify the analytics processor receiving these ids; the function is also
  unused within this change, so consider dropping it from this change if it has no purpose here.
history:
  - status: open
    by: privacy
```

### newcomer

Disabled by configuration.
