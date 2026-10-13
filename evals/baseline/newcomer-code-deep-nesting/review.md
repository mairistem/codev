# Review record: import-accounts

<!-- codev-review: 1 -->

## code.review

### newcomer

#### R-newcomer-1 `import` nests six levels deep

```yaml
severity: low
confidence: high
location: "src/import.rs:14-41"
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
                                    created += 1;
recommendation: >-
  Flatten without changing behavior: `continue` early on an empty email and
  on an unchanged plan, merge the two seat conditions with `&&` (or a
  `filter`), and extract the per-row work into a helper (e.g.
  `import_row(db, row) -> Result<usize, DbError>`) so the loop reads as a
  sum over rows.
history:
  - status: open
    by: newcomer
```

#### R-newcomer-2 Seat filtering on `'@'` is duplicated and unexplained

```yaml
severity: low
confidence: high
location: "src/import.rs:21-32"
evidence: |
  if seat.contains('@') {
recommendation: >-
  The same unexplained check appears at line 21 (existing account) and line
  32 (new account). Name the rule once, e.g. `fn is_valid_seat(seat: &str) -> bool`, with a
  comment saying why seats without '@' are silently skipped (they are
  expected to be e-mail addresses?), and use it in both branches.
history:
  - status: open
    by: newcomer
```

#### R-newcomer-3 Seats are added to an existing account only when its plan differs, with no explanation

```yaml
severity: medium
confidence: low
location: "src/import.rs:17-18"
evidence: |
                Some(account) => {
                    if account.plan != row.plan {
recommendation: >-
  A reader cannot tell whether this is intended: an existing account whose
  plan is unchanged never receives the new seats of the CSV, and when the
  plan differs the plan itself is not updated. Either document the business
  rule in a comment or confirm with the author whether the condition is a
  defect (the proposal only says "creates missing accounts and their
  seats").
history:
  - status: open
    by: newcomer
```

#### R-newcomer-4 Rows with an empty email are skipped silently

```yaml
severity: low
confidence: high
location: "src/import.rs:15"
evidence: |
        if !row.email.is_empty() {
recommendation: >-
  Turn it into an early `continue` with a one-line comment saying why such
  rows are ignored (and that they are not counted or reported).
history:
  - status: open
    by: newcomer
```

#### R-newcomer-5 No test describes the import behavior

```yaml
severity: medium
confidence: high
location: "src/import.rs:11-12"
evidence: |
  /// Imports the rows and returns how many seats were created.
  pub fn import(db: &Db, rows: &[Row]) -> Result<usize, DbError> {
recommendation: >-
  The task is "verified by `cargo test import`", yet the change adds no test
  (no `#[cfg(test)] mod tests` anywhere in `src/`). Add tests named after the
  behavior: creates a missing account with its seats, skips rows without an
  email, skips seats without '@', does not duplicate an existing seat, and
  the plan-unchanged case of R-newcomer-3 once decided.
history:
  - status: open
    by: newcomer
```
