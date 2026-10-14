# Review record: import-accounts

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### newcomer

#### R-newcomer-1 `import` nests six levels deep and hides its seat rules in unexplained filters

```yaml
severity: medium
confidence: high
location: "src/import.rs:14-40"
evidence: |
                  Some(account) => {
                      if account.plan != row.plan {
                          for seat in &row.seats {
                              if !account.seats.contains(seat) {
                                  if seat.contains('@') {
                                      db.add_seat(account.id, seat)?;
                                      created += 1;
                                  }
recommendation: >
  Flatten the function and name its rules so a reader does not need the plan.
  (1) Skip empty emails with an early `continue` (`if row.email.is_empty() { continue; }`)
  instead of wrapping the whole loop body in `if !row.email.is_empty()`.
  (2) Extract the duplicated seat loop — present in both the `Some` and `None`
  branches with the same `seat.contains('@')` test — into one helper, e.g.
  `add_new_seats(db, account_id, seats, existing: &[String]) -> Result<usize, DbError>`,
  filtering with a named predicate such as `is_seat_email(seat)` and a comment
  saying why a seat without '@' is silently dropped.
  (3) The condition `if account.plan != row.plan` is the line a newcomer
  misreads: nothing says why seats of an existing account are added only when
  its plan differs from the CSV row (and why the plan itself is then not
  updated). Name it (`let plan_changed = ...`) and add a comment giving the
  reason, or a doc line on `import` stating the rule. Also extend the doc
  comment: "returns how many seats were created" does not say that accounts
  are created too, nor that rows with an empty email are skipped.
history:
  - status: open
    by: newcomer
```
