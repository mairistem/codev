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
  The proposal promises that the button "erases the member's personal data", but delete_account only
  sets deleted_at and clears password_hash: email, display_name, phone and postal_address (app/schema.sql)
  stay in the users row indefinitely. Scenario: a member clicks "Delete my account"; afterwards
  `SELECT email, display_name, phone, postal_address FROM users WHERE id = ?` still returns all of them,
  and the orders kept for accounting stay linked to them through orders.user_id — so the orders are not
  kept "without personal data" either. This breaks the right to erasure (GDPR Art. 17) and storage
  limitation (Art. 5(1)(e)). Overwrite or NULL the personal columns (email needs a unique placeholder such
  as `deleted-<id>@invalid`, display_name a non-null placeholder) in the same UPDATE, keep only the id the
  orders reference, and add a test asserting those columns are gone after deletion.
history:
  - status: open
    by: privacy
```

#### R-privacy-2 New analytics pseudonym is never cut off when the account is deleted

```yaml
severity: medium
confidence: low
location: "app/account.py:8-13"
evidence: |
  _SALT = os.environ["ANALYTICS_SALT"]


  def analytics_id(user_id: int) -> str:
      """The pseudonymous id analytics events carry instead of the user id."""
      return hashlib.sha256(f"{_SALT}:{user_id}".encode()).hexdigest()
recommendation: >-
  The change adds a pseudonymous analytics identifier that the plan does not mention and that nothing in
  the change uses. A salted hash of the user id with one global salt is pseudonymised, still personal data
  (GDPR recital 26): whoever holds the salt and the kept users/orders ids can re-link every analytics event
  to the member, and delete_account neither deletes nor unlinks the events carrying it, so erasure does not
  reach them (Art. 17). It also leaves the analytics processor unidentified. Either remove analytics_id
  from this change, or state its purpose and processor in the proposal and make deletion reach it (purge
  the member's events, or use a per-user random id stored on the row and destroyed on deletion). Confidence
  low: the analytics pipeline that would consume it is not visible in the repository.
history:
  - status: open
    by: privacy
```

#### R-privacy-3 No retention or backup rule for deleted accounts

```yaml
severity: medium
confidence: low
location: "app/account.py:17"
evidence: |
  """Deletes the member's account; their orders are kept for accounting."""
recommendation: >-
  Neither the plan nor the code says how long the soft-deleted users row and the orders kept "for
  accounting" are retained, nor when copies in backups of members.db are purged. State the accounting
  retention period (and its legal basis) for orders, a purge of the remaining row once it lapses, and how
  deletion propagates to or expires from backups (GDPR Art. 5(1)(e), Art. 17). Confidence low: backup and
  retention policies may exist outside this repository.
history:
  - status: open
    by: privacy
```

### newcomer

Disabled by configuration.
