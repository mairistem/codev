# Review record: import-accounts

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### newcomer

#### R-newcomer-1 `import` nests six levels deep and hides its seat rules in unexplained conditions

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
  Flatten the function and name its rules. A newcomer reading line 18 cannot
  tell, without the plan, why seats of an existing account are only added
  when the row's plan differs from the stored one (and that the stored plan
  is never updated), nor why a seat without '@' (lines 21 and 32) or a row
  with an empty email (line 15) is silently skipped. The seat loop is also
  duplicated between the `Some` and `None` arms (lines 19-26 and 31-36).
  Suggested shape: `if row.email.is_empty() { continue; }` as an early
  return; extract `fn is_valid_seat(seat: &str) -> bool` (or a named
  constant/comment explaining the '@' check); extract one helper such as
  `fn add_new_seats(db, account_id, existing: &[String], seats: &[String])
  -> Result<usize, DbError>` used by both arms; and a comment stating why
  the plan comparison gates seat creation. Behavior stays the same.
history:
  - status: open
    by: newcomer
```
