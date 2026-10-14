# Review record: import-accounts

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### newcomer

#### R-newcomer-1 `import` hides its rules in five levels of nesting and a duplicated seat loop

```yaml
severity: medium
confidence: high
location: "src/import.rs:14-40"
evidence: |
  if !row.email.is_empty() {
      match db.find_account(&row.email)? {
          Some(account) => {
              if account.plan != row.plan {
                  for seat in &row.seats {
                      if !account.seats.contains(seat) {
                          if seat.contains('@') {
recommendation: >-
  Flatten and name the rules so the function reads without the plan. A
  reader of the existing-account branch stumbles at
  `if account.plan != row.plan`, five levels deep: it reads as "update the
  plan", yet the plan is never written — it only gates whether new seats
  are added, so an existing account whose plan is unchanged silently gets
  none of the CSV's new seats; nothing says whether that is intended.
  The other filters are unexplained too: rows with an empty email are
  skipped without a word (`if !row.email.is_empty()`), and
  `seat.contains('@')` is an undocumented "a seat is an email address"
  check, duplicated in both branches together with the
  `db.add_seat` / `created += 1` loop. Simpler version: `continue` early on
  an empty email; resolve the account into `(id, existing_seats)` (creating
  it when absent); extract one helper such as
  `add_missing_seats(db, account_id, existing, &row.seats) -> Result<usize, DbError>`
  that filters with a named predicate (`is_seat_email`) and returns the
  count; and give the plan-difference condition a name or a comment that
  states why seats are only added when the plan differs.
history:
  - status: open
    by: newcomer
```
