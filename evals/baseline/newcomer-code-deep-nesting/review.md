# Review record: import-accounts

<!-- codev-review: 1 -->

## code.review

### newcomer

#### R-newcomer-1 `import` nests six levels deep

```yaml
severity: low
confidence: high
location: "src/import.rs:14-24"
evidence: |
    for row in rows {
        if !row.email.is_empty() {
            match db.find_account(&row.email)? {
                Some(account) => {
                    if account.plan != row.plan {
                        for seat in &row.seats {
                            if !account.seats.contains(seat) {
                                if seat.contains('@') {
                                    db.add_seat(account.id, seat)?;
recommendation: >-
  Flatten without changing behavior: `continue` early on an empty email and
  on an unchanged plan, merge the two seat conditions into one filter, and
  move the seat loop into a helper so `import` reads as "find or create the
  account, then add its new seats".
history:
  - status: open
    by: newcomer
```

#### R-newcomer-2 Seat-adding loop duplicated in both match arms

```yaml
severity: low
confidence: high
location: "src/import.rs:31-37"
evidence: |
                    for seat in &row.seats {
                        if seat.contains('@') {
                            db.add_seat(id, seat)?;
                            created += 1;
                        }
                    }
recommendation: >-
  Extract one helper, e.g. `add_new_seats(db, account_id, existing, seats) ->
  Result<usize, DbError>`, used by both the `Some` and `None` arms, so the
  seat filter lives in a single place.
history:
  - status: open
    by: newcomer
```

#### R-newcomer-3 Seats are only added to an existing account when its plan differs, without saying why

```yaml
severity: medium
confidence: low
location: "src/import.rs:18"
evidence: |
                    if account.plan != row.plan {
recommendation: >-
  A reader without the plan cannot tell whether skipping new seats on an
  account whose plan is unchanged is intended or a bug; the proposal only
  says "creates missing accounts and their seats". State the rule in a
  comment (or confirm it is a defect), and note that the account's plan
  itself is never updated here either.
history:
  - status: open
    by: newcomer
```

#### R-newcomer-4 Unexplained filters: `contains('@')` and empty email skip silently

```yaml
severity: low
confidence: high
location: "src/import.rs:15-21"
evidence: |
        if !row.email.is_empty() {
            match db.find_account(&row.email)? {
                Some(account) => {
                    if account.plan != row.plan {
                        for seat in &row.seats {
                            if !account.seats.contains(seat) {
                                if seat.contains('@') {
recommendation: >-
  Name the checks (e.g. `fn looks_like_email(seat: &str) -> bool`) and say in
  a comment why rows without an email and seats without '@' are dropped
  silently rather than reported; `seats: Vec<String>` could also document
  that a seat is an email address.
history:
  - status: open
    by: newcomer
```

#### R-newcomer-5 No test describes the import behavior

```yaml
severity: low
confidence: high
location: "tasks.md#1. Implementation"
evidence: |
  - [x] 1.1 `import`, verified by importing last month's CSV export on staging
recommendation: >-
  Add tests named after the behavior (creates a missing account with its
  seats, adds only new seats to an existing account, skips rows without an
  email, ignores seats without '@') so a newcomer can learn the rules from
  them; today the only verification is a manual staging run.
history:
  - status: open
    by: newcomer
```
