# Review record: import-accounts

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### newcomer

#### R-newcomer-1 `import` nests five levels deep, duplicates the seat loop and hides its filters

```yaml
severity: medium
confidence: high
location: "src/import.rs:14-40"
evidence: |
                    if account.plan != row.plan {
                        for seat in &row.seats {
                            if !account.seats.contains(seat) {
                                if seat.contains('@') {
                                    db.add_seat(account.id, seat)?;
                                    created += 1;
recommendation: >-
  Flatten and name the rules so the function reads without the plan.
  (1) Replace `if !row.email.is_empty() { ... }` with an early
  `if row.email.is_empty() { continue; }`. (2) Extract the seat loop,
  duplicated at lines 19-26 and 31-36, into one helper such as
  `add_new_seats(db, account_id, seats, existing) -> Result<usize, DbError>`
  so both branches call it. (3) Name the unexplained filter
  `seat.contains('@')` (e.g. `fn is_seat_email(seat: &str) -> bool`) or
  comment why a seat without `@` is skipped silently. (4) Say why seats are
  only added to an existing account when `account.plan != row.plan`: a
  reader of line 18 cannot tell whether that condition is the intended rule
  or a mistake, and would likely assume an existing account on the same plan
  still gets its new seats. A comment giving the reason, or a well-named
  guard (`if account.plan == row.plan { continue; }` with its why), answers
  it. The doc comment on line 11 should also say which seats count
  (new, email-shaped) so the returned number is not misread.
history:
  - status: open
    by: newcomer
```
