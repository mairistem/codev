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
  delete_account only soft-deletes the row: email, display_name, phone and
  postal_address (app/schema.sql) stay in `users` forever. Scenario: member 1
  (ada@example.org, 'Ada') deletes their account; the row still holds
  'ada@example.org' and 'Ada', and orders still reference it through
  orders.user_id, so the orders kept "for accounting, without personal data"
  remain linked to the member's identity. This breaks the right to erasure
  (GDPR Art. 17) and storage limitation (Art. 5(1)(e)). Overwrite or null every
  personal column in the same UPDATE (e.g. email = 'deleted-<id>@invalid' to keep
  the UNIQUE NOT NULL constraint, display_name = '', phone = NULL,
  postal_address = NULL), and add a test asserting that no personal column of the
  deleted row still holds the original values.
history:
  - status: open
    by: privacy
```

#### R-privacy-2 Erasure does not reach analytics events keyed by the stable analytics_id

```yaml
severity: medium
confidence: low
location: "app/account.py:11-13"
evidence: |
  def analytics_id(user_id: int) -> str:
      """The pseudonymous id analytics events carry instead of the user id."""
      return hashlib.sha256(f"{_SALT}:{user_id}".encode()).hexdigest()
recommendation: >-
  The change adds a pseudonymous id for analytics events, derived from the user
  id with one global salt. It is personal data (pseudonymised data stays
  personal under GDPR Recital 26): anyone with the salt can recompute it for any
  user id and re-link the events. delete_account does nothing about these
  events, so a deleted member's analytics history stays linked to them. The
  analytics store and its processor are not visible in the diff: state the
  purpose and retention of these events, identify the analytics processor
  (Art. 28, transfers outside the EEA), and make deletion either purge the
  member's events or break the link (e.g. a per-user random id deleted with the
  account instead of a salted hash of the user id).
history:
  - status: open
    by: privacy
```

#### R-privacy-3 No retention or purge defined for deleted accounts, backups included

```yaml
severity: medium
confidence: low
location: "app/account.py:17"
evidence: |
  """Deletes the member's account; their orders are kept for accounting."""
recommendation: >-
  Neither the code nor the proposal says how long the deleted `users` row and the
  orders kept for accounting are retained, nor how deletion reaches backups of
  members.db. Ask the team to state the accounting retention period for orders,
  a purge of the soft-deleted rows after it, and how restored backups re-apply
  past deletions.
history:
  - status: open
    by: privacy
```

### newcomer

Disabled by configuration.
