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
  The proposal promises that the button "erases the member's personal data", but delete_account
  only soft-deletes the row: email, display_name, phone and postal_address (app/schema.sql:3-6)
  stay in the users table indefinitely. A member who clicks "Delete my account" still has their
  e-mail, name, phone number and postal address stored, so the right to erasure (GDPR Art. 17)
  and storage limitation (Art. 5(1)(e)) are not met. Overwrite or null these fields in the same
  UPDATE (email is NOT NULL UNIQUE, so replace it with a unique non-identifying value such as
  "deleted-<id>@invalid"), keeping the row only so orders.user_id still resolves to a now
  pseudonymous record for accounting. Add a test asserting the personal columns are gone after
  delete_account — the current tests only check sign-in and sessions.
history:
  - status: open
    by: privacy
```

#### R-privacy-2 Analytics events keyed by analytics_id are not erased with the account

```yaml
severity: medium
confidence: low
location: "app/account.py:11-13"
evidence: |
  def analytics_id(user_id: int) -> str:
      """The pseudonymous id analytics events carry instead of the user id."""
      return hashlib.sha256(f"{_SALT}:{user_id}".encode()).hexdigest()
recommendation: >-
  analytics_id is a deterministic hash of the user id under a long-lived salt, so every analytics
  event it tags stays linkable to the member (anyone holding ANALYTICS_SALT and the users/orders
  ids can re-identify it) — it is pseudonymous, not anonymous, and remains personal data. The
  change adds it but delete_account does nothing about the events carrying it, so erasure misses
  that copy. State where analytics events are stored and who processes them (a new processor or
  transfer outside the EEA must be identified), and have delete_account erase or detach the
  member's analytics events (or use a per-user random id deleted with the account). Confidence
  low: the analytics store and its callers are not visible in this change.
history:
  - status: open
    by: privacy
```

#### R-privacy-3 No retention rule for deleted accounts and their backups

```yaml
severity: medium
confidence: low
location: "app/account.py:17"
evidence: |
  """Deletes the member's account; their orders are kept for accounting."""
recommendation: >-
  The change defines no retention period: soft-deleted users rows are never purged, orders are
  "kept for accounting" with no stated duration, and nothing says how long erased data survives
  in backups of members.db. Define the accounting retention period for orders, a purge of
  soft-deleted rows once it has elapsed, and the backup retention after which an erasure is
  complete (GDPR Art. 5(1)(e), Art. 17). Confidence low: the backup setup and the accounting
  obligation are not visible in the repository — a question for the team.
history:
  - status: open
    by: privacy
```
