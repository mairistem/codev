# Review record: add-account-deletion

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### privacy

#### R-privacy-1 Account deletion keeps the member's personal data in `users`

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
  The proposal promises that "Delete my account" erases the member's personal
  data (GDPR Art. 17), but `delete_account` only stamps `deleted_at` and clears
  `password_hash`: `email`, `display_name`, `phone` and `postal_address`
  (app/schema.sql:3-6) stay in the `users` row indefinitely. Scenario: a member
  clicks "Delete my account"; their e-mail, name, phone and postal address are
  still readable in `members.db` afterwards, and the `orders` rows still point
  through `user_id` to that identified row, so the orders are not kept
  "without personal data" either. Overwrite or null those columns (e.g. set
  `email` to a unique non-identifying placeholder such as `deleted-<id>`,
  `display_name` to a placeholder, `phone` and `postal_address` to NULL) in the
  same UPDATE, and add a test asserting that no personal column of the row
  still holds the member's data after deletion.
history:
  - status: open
    by: privacy
```

#### R-privacy-2 No retention or purge rule for deleted accounts, backups included

```yaml
severity: medium
confidence: low
location: "app/account.py:16-23"
evidence: |
  def delete_account(user_id: int) -> None:
      """Deletes the member's account; their orders are kept for accounting."""
recommendation: >-
  The change defines neither how long the soft-deleted `users` row is kept nor
  when it is purged, nor how erasure reaches backups of `members.db`. State the
  retention period of the residual row and of the accounting orders, and how
  erased data expires from backups (or is re-erased on restore). Depends on
  backup and accounting policies not visible in the repository.
history:
  - status: open
    by: privacy
```

#### R-privacy-3 Analytics pseudonym stays linkable after erasure

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
  The change adds a pseudonymous analytics identifier derived deterministically
  from the user id with a single global salt. It is pseudonymous, not anonymous
  (GDPR recital 26): anyone holding the salt and the kept `users`/`orders` ids
  can recompute it, and `delete_account` erases nothing on the analytics side,
  so the deleted member's events remain linkable to them. It is also unused by
  the change and not in its plan. Either remove it from this change, or make
  erasure reach the analytics data (delete the member's events, or use a
  per-user random pseudonym stored in `users` and destroyed on deletion), and
  identify the analytics processor that receives it. Depends on the analytics
  pipeline, which is not visible.
history:
  - status: open
    by: privacy
```

### newcomer

Disabled by configuration.
