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
  "UPDATE users SET deleted_at = datetime('now'), password_hash = NULL WHERE id = ?",
recommendation: >-
  Erase the personal columns of the row on deletion (GDPR Art. 17, Art. 5(1)(c)
  and (e)): overwrite or null `email`, `display_name`, `phone` and
  `postal_address` (app/schema.sql:3-6) — `email` is NOT NULL UNIQUE, so replace
  it with an opaque placeholder such as `deleted-<id>@invalid` — keeping only the
  id that `orders.user_id` references, so orders stay for accounting without
  personal data as the proposal promises. Add a test asserting these columns no
  longer hold the member's values after `delete_account`; the current tests only
  check sign-in and sessions. State how backups are purged or expire.
history:
  - status: open
    by: privacy
```

Scenario: a member with `email = 'ada@example.org'`, a phone and a postal
address clicks "Delete my account"; afterwards the `users` row still holds
all four values, only `deleted_at` is set and `password_hash` cleared — the
proposal's "erases the member's personal data" is not met and the e-mail
stays reserved by the UNIQUE constraint.

#### R-privacy-2 Pseudonymous analytics id survives erasure and has no use in the change

```yaml
severity: medium
confidence: low
location: "app/account.py:8-13"
evidence: |
  _SALT = os.environ["ANALYTICS_SALT"]
recommendation: >-
  `analytics_id` is added with no caller in the change; a salted hash with one
  global, permanent salt is pseudonymous personal data (GDPR Recital 26), not
  anonymous, so events carrying it stay linkable to the member after deletion.
  Either drop it from this change, or state where those analytics events go
  (processor, retention) and how `delete_account` erases them or breaks the
  link (e.g. a per-user salt deleted with the account).
history:
  - status: open
    by: privacy
```

### newcomer

Disabled by configuration.
