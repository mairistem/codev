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
  The proposal promises that the button "erases the member's personal data"
  and keeps orders "without personal data", but `delete_account` only
  soft-deletes: it sets `deleted_at` and clears `password_hash`, while
  `users.email`, `users.display_name`, `users.phone` and
  `users.postal_address` (app/schema.sql) stay in the table, still joined to
  the kept `orders` rows through `orders.user_id`. Scenario: a member clicks
  "Delete my account"; their row still holds their e-mail, name, phone and
  address, indefinitely, linked to their order history — the erasure right
  (GDPR Art. 17) and storage limitation (Art. 5(1)(e)) are not met, and the
  `UNIQUE` e-mail also blocks them from signing up again. Overwrite or null
  those columns in the same UPDATE (e.g. `email` set to a non-identifying
  placeholder such as `'deleted-' || id`, `display_name` to a fixed label,
  `phone` and `postal_address` to NULL), so the row that `orders` references
  carries no personal data, and add a test asserting the columns are erased.
history:
  - status: open
    by: privacy
```

#### R-privacy-2 Erasure does not reach analytics data keyed by `analytics_id`

```yaml
severity: medium
confidence: low
location: "app/account.py:13"
evidence: |
  return hashlib.sha256(f"{_SALT}:{user_id}".encode()).hexdigest()
recommendation: >-
  The change adds a pseudonymous `analytics_id` that analytics events carry,
  derived deterministically from the user id and a fixed salt. Since the
  `users` row and its `id` are kept after deletion, anyone holding the salt
  can recompute the id and re-link the member's analytics events to them, and
  `delete_account` does nothing about those events. Nothing in the change
  calls `analytics_id`, and the analytics store is not visible here, so this
  depends on how it is used: either remove the function until a use needs it,
  or make deletion reach the analytics data (delete the member's events, or
  break the link, e.g. a random per-user analytics id stored in `users` and
  cleared on deletion). The analytics recipient should also be identified as
  a processor if it is a third party.
history:
  - status: open
    by: privacy
```

#### R-privacy-3 No retention or purge defined for deleted accounts, backups included

```yaml
severity: medium
confidence: low
location: "app/account.py:16-17"
evidence: |
  def delete_account(user_id: int) -> None:
      """Deletes the member's account; their orders are kept for accounting."""
recommendation: >-
  Deleted accounts stay in `users` with `deleted_at` set and no rule says
  when the row (or what is left of it) is purged, nor how deleted members'
  data is removed from database backups of `members.db`. State the
  retention period for kept orders (the accounting obligation) and for the
  deleted `users` row, and how backups age out the erased data; this is a
  question for the team, as only the controller can set the periods.
history:
  - status: open
    by: privacy
```

### newcomer

Disabled by configuration.
