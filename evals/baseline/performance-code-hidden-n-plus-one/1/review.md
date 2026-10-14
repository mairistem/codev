# Review record: add-monthly-report

<!-- codev-review: 1 -->

## code.review

Mode: inline.

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
  `customer_label` runs `db.customer(invoice.customer_id)` (src/report.rs:39),
  i.e. `SELECT ... FROM customers WHERE id = $1`, once per invoice: about
  3 000 queries for a month of 3 000 invoices from 800 customers, growing
  linearly with the month's invoices. Collect the distinct `customer_id`s,
  load them in one call with the existing `Db::customers(&[i64])`
  (`WHERE id = ANY($1)`, src/db.rs:26), index them in a `HashMap<i64, Customer>`
  and build each label from it. Keep the current error behavior when a
  customer is missing (return a `DbError`) and the invoice-number order.
history:
  - status: open
    by: performance
```

#### R-performance-2 Purchase-order regex compiled once per invoice

```yaml
severity: medium
confidence: high
location: "src/report.rs:45"
evidence: |
  let pattern = Regex::new(r"PO-\d{4}-\d{4}").expect("a valid pattern");
recommendation: >-
  `purchase_order` is called for every invoice of the month
  (src/report.rs:28), so the pattern is compiled about 3 000 times per
  report, cost growing with the number of invoices, while it never changes.
  Compile it once — a `static` `std::sync::OnceLock<Regex>` (or
  `LazyLock`), or build it in `monthly_report` before the loop and pass it
  to `purchase_order`.
history:
  - status: open
    by: performance
```

### newcomer

Disabled by configuration.
