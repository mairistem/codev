# Review record: import-accounts

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### newcomer

#### R-newcomer-1 `import` nests five levels deep and hides its rules in unexplained filters

```yaml
severity: medium
confidence: high
location: "src/import.rs:14-40"
evidence: |
      for row in rows {
          if !row.email.is_empty() {
              match db.find_account(&row.email)? {
                  Some(account) => {
                      if account.plan != row.plan {
                          for seat in &row.seats {
                              if !account.seats.contains(seat) {
                                  if seat.contains('@') {
recommendation: >-
  Flatten and name the rules. Skip empty emails with an early
  `continue` (`if row.email.is_empty() { continue; }`). Extract the seat
  loop, which is duplicated in the `Some` and `None` arms (lines 19-26
  and 31-36), into one helper such as
  `add_new_seats(db, account_id, &row.seats, &existing_seats) -> Result<usize, DbError>`
  that filters with `.filter(|s| !existing.contains(s)).filter(|s| is_seat_email(s))`.
  Give the two bare filters a name or a comment saying why:
  `seat.contains('@')` (line 21, 32) — a reader cannot tell whether it
  validates an email or drops some other kind of seat — and
  `account.plan != row.plan` (line 18), which means an existing account
  gets its new seats only when the CSV's plan differs from the stored one
  and is otherwise silently skipped; a newcomer reads it as inverted and
  cannot answer why without the plan, which does not say either. Note
  also that the plan itself is never updated in that branch, so the
  condition reads like a missing step. The doc comment "returns how many
  seats were created" should say that seats of unchanged-plan accounts
  and seats without `@` are skipped.
history:
  - status: open
    by: newcomer
```
