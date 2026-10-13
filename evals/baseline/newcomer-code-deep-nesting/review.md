# Review record: import-accounts

<!-- codev-review: 1 -->

## code.review

### newcomer

#### R-newcomer-1 `import` nests six levels deep and hides its seat rules

```yaml
severity: medium
confidence: high
location: "src/import.rs:12-42"
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
  Flatten and name the rules so the function reads without the plan.
  Skip rows with an empty email with an early `continue` (and say why
  they are skipped). Extract the duplicated seat loop — it appears once
  under `Some(account)` and once under `None` — into a helper such as
  `add_new_seats(db, account_id, seats, existing) -> Result<usize, DbError>`,
  and replace the bare `seat.contains('@')` by a named predicate
  (`is_valid_seat_email`) so the reader knows seats that are not emails are
  dropped on purpose. The condition `account.plan != row.plan` is where a
  newcomer stumbles: read at this depth, it says that seats are added to an
  existing account only when its plan differs from the CSV row, and that
  the plan itself is never updated — nothing tells whether that is the
  intended rule or an inverted test; give it a name or a comment stating
  the why (or, if unintended, raise it to the developer). Then the body is
  one match of two arms, each a call, at most two levels deep.
history:
  - status: open
    by: newcomer
```

Where a newcomer stumbles: at `if account.plan != row.plan` (line 18),
five levels in, with no way to tell without the plan why matching plans
receive no seats; and at `seat.contains('@')` (lines 21 and 32), a filter
with no explanation that silently drops seats.
