# Review record: add-monthly-report

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### performance

#### R-performance-1 One customer query per invoice in the monthly report

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
  N+1 queries: through `customer_label` (`db.customer(invoice.customer_id)?`,
  src/report.rs:39), `monthly_report` runs `SELECT ... FROM customers WHERE id = $1` once per
  invoice, so a month of ~3 000 invoices (proposal) costs ~3 000 round trips for ~800
  customers, growing linearly with invoice volume on the report finance reads. Collect the
  distinct `customer_id`s, load them in one call with the existing
  `Db::customers(&[i64])` (`WHERE id = ANY($1)`), index them in a `HashMap<i64, Customer>`
  and build each label from the map. Keep the current error behaviour for a missing
  customer (return a `DbError` rather than silently skipping the line).
history:
  - status: open
    by: performance
```

#### R-performance-2 Purchase-order regex compiled once per invoice

```yaml
severity: medium
confidence: high
location: "src/report.rs:45-46"
evidence: |
  let pattern = Regex::new(r"PO-\d{4}-\d{4}").expect("a valid pattern");
  pattern.find(notes).map(|m| m.as_str().to_string())
recommendation: >-
  `purchase_order` is called for every invoice of the month and recompiles the same
  pattern each time; compiling a regex is far more expensive than matching it against a
  short notes string, so the cost grows with the ~3 000 invoices per report. Compile it
  once, e.g. `static PO_PATTERN: std::sync::LazyLock<Regex> = LazyLock::new(|| ...)`
  (or `once_cell`), or compile it in `monthly_report` and pass it in.
history:
  - status: open
    by: performance
```

### newcomer

Disabled by configuration.
