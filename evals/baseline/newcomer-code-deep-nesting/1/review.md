# Review record: import-accounts

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### newcomer

#### R-newcomer-1 `import` nests six levels deep and hides its rules in unexplained conditions

```yaml
severity: medium
confidence: high
location: "src/import.rs:14-40"
evidence: |
  if account.plan != row.plan {
      for seat in &row.seats {
          if !account.seats.contains(seat) {
              if seat.contains('@') {
recommendation: >-
  Flatten `import` and name its rules. The seat-adding loop sits six levels
  deep (for / if / match / if / for / if / if), and a reader stumbles on three
  questions the code does not answer without the plan: why seats are added to
  an existing account only when its plan differs from the row's
  (`account.plan != row.plan` — it reads like a bug), why a seat without '@'
  is silently skipped (`seat.contains('@')`), and why rows with an empty email
  are skipped. Use `continue` for the empty-email row
  (`if row.email.is_empty() { continue; }`), merge the two nested `if`s into
  one condition, and extract the seat loop duplicated between the `Some` and
  `None` arms (lines 19-26 and 31-36) into a helper such as
  `add_valid_seats(db, account_id, seats, existing) -> Result<usize, DbError>`
  with a named predicate like `is_valid_seat_email`. Add a short comment
  giving the why of the plan-mismatch condition and of the '@' filter.
  No behavior change.
history:
  - status: open
    by: newcomer
```
