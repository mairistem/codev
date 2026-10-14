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
  The proposal promises that the button "erases the member's personal data" and that
  orders are kept "without personal data", but `delete_account` only soft-deletes:
  `email`, `display_name`, `phone` and `postal_address` (app/schema.sql:3-6) stay in the
  `users` row, which `orders.user_id` still references, so the orders remain linked to an
  identified person. Scenario: member 1 (ada@example.org, Ada) clicks "Delete my account";
  afterwards `SELECT email, display_name, phone, postal_address FROM users WHERE id = 1`
  still returns all four — the right to erasure (GDPR Art. 17) and storage limitation
  (Art. 5(1)(e)) are not met. Because `email` is UNIQUE, the kept address also blocks the
  person from signing up again. Overwrite or NULL these columns in the same UPDATE (e.g.
  email replaced by a non-identifying placeholder such as `deleted-<id>@invalid`,
  display_name by a neutral value, phone and postal_address NULL), keeping only the id the
  orders need, and add a test asserting the row no longer holds any of them.
history:
  - status: open
    by: privacy
```

#### R-privacy-2 Erasure does not reach analytics events keyed by `analytics_id`

```yaml
severity: medium
confidence: low
location: "app/account.py:11-13"
evidence: |
  def analytics_id(user_id: int) -> str:
      """The pseudonymous id analytics events carry instead of the user id."""
      return hashlib.sha256(f"{_SALT}:{user_id}".encode()).hexdigest()
recommendation: >-
  The salted hash is pseudonymous, not anonymous: anyone holding `ANALYTICS_SALT` can
  recompute it from the kept user id, so analytics events carrying it stay personal data
  after deletion. `delete_account` neither purges nor detaches those events, and the change
  does not say where they are stored or for how long. Confirm with the team where
  analytics events live and either delete the member's events on account deletion or
  state why the pseudonymous id is acceptable; the analytics store was not visible in
  this repository.
history:
  - status: open
    by: privacy
```

#### R-privacy-3 Retention and erasure in backups are not defined

```yaml
severity: medium
confidence: low
location: "proposal.md#What Changes"
evidence: |
  - A "Delete my account" button erases the member's personal data and signs them out everywhere; their orders are kept for accounting, without personal data.
recommendation: >-
  Neither the plan nor the code says how long backups of `members.db` keep a deleted
  member's data or how a restore avoids bringing it back. Ask the team to state the backup
  retention period and the rule for re-applying deletions after a restore; nothing about
  backups was visible in the repository.
history:
  - status: open
    by: privacy
```

**To decide**

- Domain `performance` is suggested and unticked — first match `app/account.py:19` (SQL `UPDATE users ... SET`). Choices: tick it in the proposal's `## Domains` so its expert reviews this code, or keep it unticked with a reason on its line.
- Domain `operations` is suggested and unticked — first match `app/account.py:8` (`os.environ["ANALYTICS_SALT"]`). Choices: tick it so its expert reviews this code, or keep it unticked with a reason on its line.
