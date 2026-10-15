# Review record: import-accounts

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### newcomer

#### R-newcomer-1 `import` nests six levels deep and hides its filtering rules

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
  Flatten the function: skip rows with an empty email by an early
  `continue`; extract a helper (e.g. `add_new_seats(db, account_id,
  existing, seats) -> Result<usize, DbError>`) that holds the single seat
  loop now duplicated in the `Some` and `None` arms (lines 19-26 and
  31-36); name the `seat.contains('@')` test (e.g. `is_email_seat`) and
  say in a comment why non-email seats are dropped. Most of all, explain
  the `account.plan != row.plan` guard on line 18: a reader cannot tell
  without the plan that seats of an existing account are only added when
  its plan differs from the row's — it reads like a bug, and the doc
  comment ("returns how many seats were created") says nothing of it.
  Rows skipped for an empty email are likewise silent; document the
  skip in the doc comment.
history:
  - status: open
    by: newcomer
```
