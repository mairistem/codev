# Review record: import-accounts

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### newcomer

#### R-newcomer-1 `import` nests five levels deep and hides its rules in unexplained conditions

```yaml
severity: medium
confidence: high
location: "src/import.rs:14-39"
evidence: |
                Some(account) => {
                    if account.plan != row.plan {
                        for seat in &row.seats {
                            if !account.seats.contains(seat) {
                                if seat.contains('@') {
                                    db.add_seat(account.id, seat)?;
                                    created += 1;
recommendation: >-
  The seat insertion sits five levels deep (for / if / match / if / for / if / if),
  so a reader joining the project must hold every enclosing condition in mind to
  know when a seat is added, and cannot answer without the plan: why seats are
  only added to an existing account when its plan differs from the row's
  (`account.plan != row.plan` reads like a filter for plan changes, while the
  doc comment says the function returns the seats created), why a seat without
  `'@'` is silently dropped, and why a row with an empty email is silently
  skipped. The seat loop is also duplicated between the `Some` and `None` arms.
  Flatten it: `continue` early on an empty email, extract one helper such as
  `add_new_seats(db, account_id, existing_seats, &row.seats) -> Result<usize, DbError>`
  used by both arms, name the `'@'` test (e.g. `is_email_seat(seat)`), and give
  the plan-comparison and skip rules a one-line comment stating why — or, if the
  plan comparison is not intended, raise it with the developer, as it decides
  which seats get created. Behavior must not change in the refactor.
history:
  - status: open
    by: newcomer
```
