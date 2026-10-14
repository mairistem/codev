# Review record: add-monthly-report

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### performance

#### R-performance-1 One customer query per invoice in the monthly report

```yaml
severity: high
confidence: high
location: "src/report.rs:38-39"
evidence: |
  fn customer_label(db: &dyn Db, invoice: &Invoice) -> Result<String, DbError> {
      let customer = db.customer(invoice.customer_id)?;
recommendation: >-
  `customer_label` is called for every invoice in the loop of
  `monthly_report` (`src/report.rs:23-26`), so it runs `SELECT ... FROM customers WHERE id = $1` once per invoice:
  with the proposal's ~3 000 invoices a month from 800 customers, one report
  issues ~3 000 customer queries (cost grows linearly with the month's
  invoices, and repeats each customer ~4 times). `Db::customers(&[i64])`
  (`src/db.rs:26`, `WHERE id = ANY($1)`) already serves the whole set:
  collect the distinct `customer_id`s, fetch them in one call, index them in
  a `HashMap<i64, Customer>`, and build each line from the map. Keep the
  invoice-number order and return a `DbError` when a customer is missing,
  as today.
history:
  - status: open
    by: performance
```

#### R-performance-2 The purchase-order regex is compiled once per invoice

```yaml
severity: medium
confidence: high
location: "src/report.rs:44-46"
evidence: |
  fn purchase_order(notes: &str) -> Option<String> {
      let pattern = Regex::new(r"PO-\d{4}-\d{4}").expect("a valid pattern");
recommendation: >-
  `purchase_order` is called for every invoice of the month (`src/report.rs:28`),
  so `Regex::new` compiles the same pattern ~3 000 times per report; compiling
  a pattern is expensive compared with matching one short note. Compile it
  once — a `static PATTERN: LazyLock<Regex>` (or `once_cell::sync::Lazy`), or
  build it in `monthly_report` and pass it in.
history:
  - status: open
    by: performance
```

### newcomer

Disabled by configuration.
