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
  The proposal promises that the button "erases the member's personal data", but
  delete_account only soft-deletes: it sets deleted_at and clears password_hash,
  while the users row keeps email, display_name, phone and postal_address
  (app/schema.sql:3-6). Scenario: a member clicks "Delete my account"; their
  e-mail, name, phone and postal address stay in the users table indefinitely,
  still joined to their orders through orders.user_id — the right to erasure
  (GDPR Art. 17) and storage limitation (Art. 5(1)(e)) are not met. Overwrite or
  NULL every personal column (e.g. set email to a unique non-identifying
  placeholder such as 'deleted-<id>' since it is NOT NULL UNIQUE, display_name to
  a fixed value, phone and postal_address to NULL) so orders keep only the opaque
  user id, add a test asserting those columns no longer hold the member's data,
  and state how long backups keep the erased data before they expire.
history:
  - status: open
    by: privacy
```

#### R-privacy-2 Analytics events keyed by analytics_id are not reached by the erasure

```yaml
severity: medium
confidence: low
location: "app/account.py:11-13"
evidence: |
  def analytics_id(user_id: int) -> str:
      """The pseudonymous id analytics events carry instead of the user id."""
      return hashlib.sha256(f"{_SALT}:{user_id}".encode()).hexdigest()
recommendation: >-
  The change adds a pseudonymous analytics identifier derived deterministically
  from the user id and a fixed salt, but delete_account does nothing about the
  analytics events it keys: after deletion, anyone holding the salt can still
  link those events back to the kept users row (pseudonymised data is still
  personal data). The analytics store and its callers are not visible in this
  repository, so this depends on what it holds. Either erase or detach the
  member's analytics events on deletion, or document that they are anonymised
  (e.g. salt rotation) and their retention period; also state its purpose and
  lawful basis, since nothing in the change uses it yet.
history:
  - status: open
    by: privacy
```

### newcomer

Disabled by configuration.
