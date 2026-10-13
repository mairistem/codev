# Review record: add-monthly-report

<!-- codev-review: 1 -->

## code.review

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
  `customer_label` calls `db.customer(invoice.customer_id)` (src/report.rs:39,
  `SELECT ... FROM customers WHERE id = $1`) once per invoice: one query for
  the list plus one per invoice, so about 3 000 queries for a month of
  3 000 invoices from only 800 customers, growing linearly with invoice
  volume. Collect the distinct `customer_id`s, fetch them once with the
  existing batch method `db.customers(&ids)` (`WHERE id = ANY($1)`), index
  them in a `HashMap<i64, Customer>` and build each label from the map.
  Keep the invoice-number order and return an error when an id is missing,
  as `db.customer` does today.
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
  `purchase_order` is called for every invoice of the month
  (src/report.rs:28) and compiles the same pattern on each call: about
  3 000 regex compilations per report, a cost far above the match itself
  and growing with invoice volume. Compile it once — a
  `static PO: std::sync::OnceLock<Regex>` (or `LazyLock`) — or build it in
  `monthly_report` before the loop and pass it in.
history:
  - status: open
    by: performance
```

### newcomer

Disabled by configuration.
