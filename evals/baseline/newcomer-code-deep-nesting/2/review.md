# Review record: import-accounts

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### newcomer

#### R-newcomer-1 `import` nests five levels deep, duplicates its seat loop and leaves its filters unexplained

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
recommendation: >-
  The innermost `db.add_seat` sits under a `for`, an `if`, a `match`, an `if`, a `for`
  and two more `if`s, so a reader has to hold seven conditions to know when a seat is
  added. Three things cost the reader most. (1) `if account.plan != row.plan` (line 18):
  a newcomer reads it as "only touch accounts whose plan changed" and cannot tell, without
  the plan, whether skipping new seats on an account whose plan is unchanged is intended —
  it is the line most likely to be misread; name the rule or explain it in a comment.
  (2) `seat.contains('@')` (lines 21 and 32) is an unexplained filter repeated twice;
  give it a name such as `fn is_valid_seat(seat: &str) -> bool` saying what a valid seat is.
  (3) The seat-adding loop is duplicated between the `Some` and `None` arms (lines 19-26
  and 31-36). Flatten with `continue` for an empty email (line 15), and extract a helper
  such as `fn add_new_seats(db, account_id, existing: &[String], seats: &[String]) -> Result<usize, DbError>`
  that filters with `is_valid_seat` and skips existing seats, called from both arms.
  Behavior stays the same.
history:
  - status: open
    by: newcomer
```
