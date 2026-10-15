# Review record: add-monthly-report

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### performance

#### R-performance-1 One customer query per invoice in the monthly report

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
  `customer_label` calls `db.customer(invoice.customer_id)` (src/report.rs:39), which runs
  `SELECT ... FROM customers WHERE id = $1` once per invoice: with the proposal's ~3 000
  invoices a month from 800 customers, one report costs ~3 001 round trips instead of 2,
  and grows linearly with the month's invoices. Collect the distinct `customer_id`s, load
  them in one call with the existing `db.customers(&ids)` (`WHERE id = ANY($1)`), index them
  in a `HashMap<i64, Customer>` and build each label from the map; keep invoice-number order
  and report a missing customer as an error, as today.
history:
  - status: open
    by: performance
```

#### R-performance-2 The purchase-order regex is compiled again for every invoice

```yaml
severity: medium
confidence: high
location: "src/report.rs:45"
evidence: |
  let pattern = Regex::new(r"PO-\d{4}-\d{4}").expect("a valid pattern");
recommendation: >-
  `purchase_order` is called once per invoice (src/report.rs:28) and compiles the pattern
  on each call: ~3 000 regex compilations per report, a cost per item far above the
  `find` itself, growing with the month's invoices. Compile it once — a
  `static PO: std::sync::LazyLock<Regex>` (or `once_cell::sync::Lazy`), or build it once in
  `monthly_report` and pass it in — and keep the same pattern and match semantics.
history:
  - status: open
    by: performance
```

### newcomer

Disabled by configuration.
