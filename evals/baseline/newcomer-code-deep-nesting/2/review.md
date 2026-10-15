# Review record: import-accounts

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### newcomer

#### R-newcomer-1 `import` nests six levels deep and hides its filters
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
  Flatten the loop body: skip empty emails with an early `continue`, and
  extract the duplicated seat loop (lines 19-26 and 31-36) into one helper
  such as `add_missing_seats(db, account_id, existing_seats, &row.seats)`
  that both branches call. Give the two unexplained filters a name or a
  comment that says why: `seat.contains('@')` (a seat is identified by an
  email address, so an entry without `@` is skipped as invalid?) and
  `account.plan != row.plan` (why an existing account on the same plan gets
  none of its new seats — a reader at line 18 will take it for a bug or
  will not notice it at all, and cannot tell which without the plan). The
  doc comment "returns how many seats were created" should also say that
  rows with an empty email and seats without `@` are silently skipped.
history:
  - status: open
    by: newcomer
```
