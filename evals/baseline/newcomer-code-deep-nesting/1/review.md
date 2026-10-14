# Review record: import-accounts

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### newcomer

#### R-newcomer-1 `import` nests six levels deep, duplicates its seat loop and leaves its filters unexplained

```yaml
severity: medium
confidence: high
location: "src/import.rs:14-40"
evidence: |
              match db.find_account(&row.email)? {
                  Some(account) => {
                      if account.plan != row.plan {
                          for seat in &row.seats {
                              if !account.seats.contains(seat) {
                                  if seat.contains('@') {
                                      db.add_seat(account.id, seat)?;
recommendation: >-
  Flatten and name the rules so the function reads without the plan.
  Skip rows with an empty email by an early `continue` (`if row.email.is_empty() { continue; }`)
  and say in a comment why such rows are dropped. Extract the duplicated seat loop
  (lines 19-26 and 31-36) into one helper, e.g. `add_new_seats(db, account_id, existing, seats) -> Result<usize, DbError>`,
  whose filters use `Iterator::filter` and a named predicate such as `is_seat_email(seat)` instead of the bare
  `seat.contains('@')`, so the reader learns that a seat is an email address and that others are skipped.
  Replace the nested `if !account.seats.contains(seat)` / `if seat.contains('@')` pair with one combined condition.
  The condition `if account.plan != row.plan` is the line a newcomer would misread: nothing says why seats are
  only added to an existing account whose plan differs from the CSV row (and the plan itself is never updated);
  the proposal only says "creates missing accounts and their seats". Name that rule with a comment or a
  well-named function stating why, so the reader does not take it for a bug or a typo for `==`.
  This restructuring must keep the current behaviour.
history:
  - status: open
    by: newcomer
```
