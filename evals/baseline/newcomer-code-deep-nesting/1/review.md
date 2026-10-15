# Review record: import-accounts

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### newcomer

#### R-newcomer-1 `import` nests six levels deep and hides its rules in unexplained conditions

```yaml
severity: medium
confidence: high
location: "src/import.rs:12-42"
evidence: |
                    if account.plan != row.plan {
                        for seat in &row.seats {
                            if !account.seats.contains(seat) {
                                if seat.contains('@') {
                                    db.add_seat(account.id, seat)?;
                                    created += 1;
recommendation: >-
  The seat-adding branch sits six levels deep (for row / if email / match /
  if plan / for seat / if not present / if '@'), so a newcomer has to hold
  every enclosing condition in mind to know when a seat is added. Three
  business rules are expressed with no word of why: rows with an empty email
  are silently skipped (line 15), seats are only added to an existing account
  when its plan differs from the row's (line 18) — a reader cannot tell
  whether this is intended or an inverted test without the plan — and seats
  without an '@' are dropped (lines 21 and 32). The seat loop with the '@'
  filter is also duplicated between the `Some` and `None` arms. Flatten
  without changing behavior: `continue` early on an empty email, extract a
  helper such as `add_new_seats(db, account_id, seats, existing) ->
  Result<usize, DbError>` with a named predicate (e.g. `is_seat_email`) used
  by both arms, and add a one-line comment giving the reason for the
  plan-change condition and the skipped rows.
history:
  - status: open
    by: newcomer
```
