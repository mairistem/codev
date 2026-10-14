# Review record: import-accounts

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### newcomer

#### R-newcomer-1 `import` nests five levels deep and hides its rules in unexplained conditions

```yaml
severity: medium
confidence: high
location: "src/import.rs:12-42"
evidence: |
            match db.find_account(&row.email)? {
                Some(account) => {
                    if account.plan != row.plan {
                        for seat in &row.seats {
                            if !account.seats.contains(seat) {
                                if seat.contains('@') {
                                    db.add_seat(account.id, seat)?;
                                    created += 1;
recommendation: >-
  Flatten the function: skip rows with an empty email with an early
  `continue` (and say why such rows are dropped); resolve the account id
  first (existing or newly created) and then run a single seat loop,
  extracted into a helper such as `add_missing_seats(db, account_id,
  existing_seats, &row.seats)`, instead of the two copies of the
  `seat.contains('@')` / `add_seat` / `created += 1` block (lines 21-24 and
  32-35). Name the unexplained filters: replace `seat.contains('@')` by a
  named predicate (e.g. `is_seat_email`) with a comment on why seats
  without `@` are silently ignored, and comment why seats are only added
  to an existing account when its plan differs from the row's
  (`account.plan != row.plan`) — a newcomer reading line 18 cannot tell
  whether that condition is a business rule or an inverted test, and
  would likely misread that an existing account on the same plan never
  gets its new seats. The doc comment should also say that existing
  accounts' plans are not updated and which rows are skipped.
history:
  - status: open
    by: newcomer
```
