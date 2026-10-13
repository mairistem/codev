# Review record: add-monthly-report

<!-- codev-review: 1 -->

## code.review

### performance

#### R-performance-1 One customer query per invoice (N+1)

```yaml
severity: high
confidence: high
location: "src/report.rs:23-26"
evidence: |
      for invoice in &invoices {
          lines.push(ReportLine {
              number: invoice.number.clone(),
              customer: customer_label(db, invoice)?,
recommendation: >-
  `customer_label` runs `db.customer(invoice.customer_id)?` (src/report.rs:39),
  one `SELECT … FROM customers WHERE id = $1` per invoice.
  Collect the distinct `customer_id`s of the month's invoices, load them in one
  query with the existing `Db::customers(&[i64])` (`WHERE id = ANY($1)`), index
  them in a `HashMap<i64, Customer>`, and build each label from that map. The
  report then runs 3 queries instead of N + 2 (about 3 000 customer queries a
  month for 800 distinct customers, per the proposal), with the same lines and
  the same error when a customer is missing.
history:
  - status: open
    by: performance
```

#### R-performance-2 The purchase-order regex is compiled once per invoice

```yaml
severity: medium
confidence: high
location: "src/report.rs:44-45"
evidence: |
  fn purchase_order(notes: &str) -> Option<String> {
      let pattern = Regex::new(r"PO-\d{4}-\d{4}").expect("a valid pattern");
recommendation: >-
  Compile the pattern once — a `static PO: LazyLock<Regex>` (or
  `OnceLock`), or build it in `monthly_report` before the loop and pass it in.
  Regex compilation costs far more than the match, and here it runs once per
  invoice of the month (about 3 000 times per report).
history:
  - status: open
    by: performance
```

#### R-performance-3 The regex crate is added for one fixed-shape pattern

```yaml
severity: low
confidence: high
location: "Cargo.toml:8"
evidence: |
  regex = "1"
recommendation: >-
  Weigh the dependency against its use: `PO-` followed by `\d{4}-\d{4}` can be
  matched by a short scan over `notes` (find `PO-`, then check 4 ASCII digits,
  `-`, 4 ASCII digits), avoiding `regex` and its transitive crates
  (`regex-automata`, `regex-syntax`, `aho-corasick`, `memchr`) in build time and
  binary size. Keep `regex` only if more patterns are expected.
history:
  - status: open
    by: performance
```

#### R-performance-4 Invoice numbers are cloned while the invoices are dropped

```yaml
severity: low
confidence: high
location: "src/report.rs:23-25"
evidence: |
      for invoice in &invoices {
          lines.push(ReportLine {
              number: invoice.number.clone(),
recommendation: >-
  `invoices` is owned and not used after the loop: iterate it by value
  (`for invoice in invoices`) and move `invoice.number` into the line, saving
  one `String` allocation and copy per invoice.
history:
  - status: open
    by: performance
```

### newcomer

Disabled by configuration.
