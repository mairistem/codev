# Review record: add-account-deletion

<!-- codev-review: 1 -->

## code.review

### privacy

#### R-privacy-1 Account deletion keeps every personal field of the member

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
  Erase the personal data on deletion, as the proposal promises: delete the users row,
  or overwrite email, display_name, phone and postal_address (email needs a unique
  non-identifying placeholder because of its UNIQUE NOT NULL constraint) so that the
  orders kept for accounting point to a row with no personal data. GDPR Art. 17 (erasure)
  and Art. 5(1)(e) (storage limitation).
history:
  - status: open
    by: privacy
```

The change only soft-deletes: `email`, `display_name`, `phone` and `postal_address`
(`app/schema.sql:3-6`) stay in `users` forever, and the orders "kept for accounting,
without personal data" still join to them through `orders.user_id`. The proposal says the
button "erases the member's personal data"; the code does not.

#### R-privacy-2 Tests do not check that personal data is erased

```yaml
severity: medium
confidence: high
location: "tests/test_account.py:10-11"
evidence: |
  delete_account(1)
  assert not can_sign_in("ada@example.org")
recommendation: >-
  Add a test asserting that after delete_account no row of users still holds the member's
  email, display name, phone or postal address, while their orders remain.
history:
  - status: open
    by: privacy
```

The tests cover sign-in and sessions only, so the erasure requirement of the proposal is
unverified — which is how R-privacy-1 passes the suite.

#### R-privacy-3 Erasure does not reach analytics data linked by analytics_id

```yaml
severity: medium
confidence: low
location: "app/account.py:11-13"
evidence: |
  def analytics_id(user_id: int) -> str:
      """The pseudonymous id analytics events carry instead of the user id."""
      return hashlib.sha256(f"{_SALT}:{user_id}".encode()).hexdigest()
recommendation: >-
  State what happens to analytics events on deletion: delete them at the analytics
  processor by analytics_id, or make the id unlinkable (per-user random id deleted with the
  account). Identify the analytics processor (GDPR Art. 28) if it is a third party.
history:
  - status: open
    by: privacy
```

The id is deterministic from a fixed salt and the user id, so the member's events stay
pseudonymous personal data (re-linkable while the salt exists) after the account is deleted;
`delete_account` does nothing about them. Where those events go could not be seen in the
repository.

#### R-privacy-4 Retention of deleted accounts and backups is not defined

```yaml
severity: medium
confidence: low
location: "proposal.md#What Changes"
evidence: |
  - A "Delete my account" button erases the member's personal data and signs them out everywhere; their orders are kept for accounting, without personal data.
recommendation: >-
  State the retention of the soft-deleted row (if any is kept) and of the orders, and how
  erasure reaches backups of members.db (expiry window or re-applying deletions on restore).
  GDPR Art. 5(1)(e).
history:
  - status: open
    by: privacy
```

### newcomer

Disabled by configuration.
