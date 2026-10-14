# Review record: import-accounts

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### newcomer

#### R-newcomer-1 `import` nests six levels deep and hides its two unexplained filters

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
recommendation: >-
  Flatten `import` and name its rules. Skip an empty email with an early
  `continue` instead of wrapping the whole body in `if !row.email.is_empty()`.
  Extract the per-seat loop, which is written twice (lines 19-26 and 31-36),
  into one helper such as `add_new_seats(db, account_id, existing, &row.seats)`
  that filters `seat.contains('@')` once, behind a named predicate such as
  `is_email_seat`, with a comment saying why a seat without `@` is dropped
  silently. Turn `if account.plan != row.plan` into a guard with a comment or
  a named condition saying why seats are added to an existing account only
  when its plan changed: a newcomer reading line 18 cannot tell, without the
  plan, whether it is a rule or a mistake, and would likely read the import as
  "add the missing seats to every existing account", which it does not do.
  The doc comment on `import` should also say that rows with an empty email
  and seats without `@` are skipped.
history:
  - status: open
    by: newcomer
```
