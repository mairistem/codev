# Review record: import-accounts

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### newcomer

#### R-newcomer-1 `import` nests five levels deep and hides its seat rules
```yaml
severity: medium
confidence: high
location: "src/import.rs:14-40"
evidence: |
      for row in rows {
          if !row.email.is_empty() {
              match db.find_account(&row.email)? {
                  Some(account) => {
                      if account.plan != row.plan {
                          for seat in &row.seats {
                              if !account.seats.contains(seat) {
                                  if seat.contains('@') {
recommendation: >-
  Flatten and name the rules. Skip empty emails with `if row.email.is_empty() { continue; }`;
  extract the seat filter into a named helper (e.g. `fn is_valid_seat(seat: &str) -> bool`,
  with a comment saying why a seat must contain '@') and the duplicated "add each valid seat,
  count it" loop of lines 19-26 and 31-36 into one function such as
  `add_seats(db, account_id, seats, existing) -> Result<usize, DbError>`, so each match arm is
  one call. Say in a comment why an existing account only gets new seats when its plan differs
  from the row (`if account.plan != row.plan`): a reader without the plan stumbles there and
  cannot tell whether that condition is intended or inverted — nor why the account's plan
  itself is never updated. The doc comment should also say that rows with an empty email are
  skipped and that seats without '@' are ignored.
history:
  - status: open
    by: newcomer
```
