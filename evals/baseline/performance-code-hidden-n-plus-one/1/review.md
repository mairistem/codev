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
  N+1 queries: `customer_label` calls `db.customer(invoice.customer_id)` (src/report.rs:39,
  `SELECT ... FROM customers WHERE id = $1`) once per invoice, so a month of about 3 000
  invoices from 800 customers runs about 3 000 customer queries plus the list query, and the
  count grows linearly with the month's invoices. Collect the distinct `customer_id`s, load
  them with the existing batch method `db.customers(&ids)` (`WHERE id = ANY($1)`), index them
  in a `HashMap<i64, Customer>` and build each label from the map — two queries per report
  whatever the volume. Keep the invoice-number order and return the same error when a
  customer is missing.
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
  `purchase_order` is called for every invoice of the loop (src/report.rs:28) and calls
  `Regex::new` each time, so a report of about 3 000 invoices compiles the same pattern
  about 3 000 times — compile cost that grows with the month's invoices and dwarfs the
  per-note match. Compile it once: a `static PO_PATTERN: std::sync::LazyLock<Regex>` (or
  `once_cell::sync::Lazy`), or build it in `monthly_report` before the loop and pass it in.
  The matching behavior stays the same.
history:
  - status: open
    by: performance
```

### newcomer

Disabled by configuration.
