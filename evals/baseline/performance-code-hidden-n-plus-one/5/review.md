# Review record: add-monthly-report

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### performance

#### R-performance-1 One customer query per invoice in the monthly report

```yaml
severity: medium
confidence: high
location: "src/report.rs:38-40"
evidence: |
  fn customer_label(db: &dyn Db, invoice: &Invoice) -> Result<String, DbError> {
      let customer = db.customer(invoice.customer_id)?;
recommendation: >-
  N+1 queries: `monthly_report` calls `customer_label` for each invoice (src/report.rs:26),
  which runs `SELECT ... FROM customers WHERE id = $1` once per invoice — about 3 000 queries
  per report for 800 distinct customers, growing with the month's invoices. Collect the
  distinct `customer_id`s, load them with the existing batch query `db.customers(&ids)`
  (`WHERE id = ANY($1)`, src/db.rs:26), index them in a `HashMap<i64, Customer>`, and build
  each label from the map; keep the current error behavior when a customer is missing.
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
  `purchase_order` is called for each invoice (src/report.rs:28) and compiles the same
  pattern every time — about 3 000 regex compilations per report, growing with the month's
  invoices, while compiling costs far more than matching one short note. Compile it once:
  a `static` `std::sync::OnceLock<Regex>` (or `LazyLock`), or build it in `monthly_report`
  before the loop and pass it in.
history:
  - status: open
    by: performance
```

### newcomer

Disabled by configuration.
