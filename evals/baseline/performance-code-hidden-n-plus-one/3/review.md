# Review record: add-monthly-report

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### newcomer

Disabled by configuration.

### performance

#### R-performance-1 One customer query per invoice (N+1)

```yaml
severity: medium
confidence: high
location: "src/report.rs:23-26"
evidence: |
  for invoice in &invoices {
      lines.push(ReportLine {
          number: invoice.number.clone(),
          customer: customer_label(db, invoice)?,
recommendation: >-
  customer_label calls db.customer(invoice.customer_id) (src/report.rs:39), one
  `SELECT ... FROM customers WHERE id = $1` per invoice: with the proposal's
  ~3 000 invoices a month, one report runs ~3 000 customer queries for ~800
  distinct customers, and the count grows linearly with invoices. Collect the
  distinct customer_ids, fetch them once with the existing db.customers(&ids)
  (`WHERE id = ANY($1)`), index them in a HashMap<i64, Customer>, and build each
  label from the map. Keep the current error behavior for a missing customer
  (return a DbError) rather than silently skipping the line.
history:
  - status: open
    by: performance
```

#### R-performance-2 Regex compiled once per invoice

```yaml
severity: medium
confidence: high
location: "src/report.rs:44-46"
evidence: |
  fn purchase_order(notes: &str) -> Option<String> {
      let pattern = Regex::new(r"PO-\d{4}-\d{4}").expect("a valid pattern");
recommendation: >-
  purchase_order is called for every invoice (src/report.rs:28) and compiles
  the same pattern each time: ~3 000 regex compilations per monthly report,
  growing linearly with invoices, where the per-item work is a short search.
  Compile it once, e.g. a `static PO_PATTERN: std::sync::LazyLock<Regex>` (or
  OnceLock), and use it in purchase_order. Results are unchanged.
history:
  - status: open
    by: performance
```
