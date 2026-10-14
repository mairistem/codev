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
  N+1 queries: `monthly_report` runs `invoices_of_month` once, then
  `customer_label` (`src/report.rs:39`, `db.customer(invoice.customer_id)?`) runs `SELECT id, name, country FROM customers WHERE id = $1`
  once per invoice — about 3 000 round trips for a month of finance's data
  (proposal: 3 000 invoices from 800 customers), growing linearly with the
  month's invoices, and fetching the same customer many times. The `Db` trait
  already offers the batch `customers(&self, ids: &[i64])`
  (`WHERE id = ANY($1)`): collect the distinct `customer_id`s, call
  `db.customers(&ids)` once, build a `HashMap<i64, Customer>`, and look each
  invoice's customer up in it. Keep the invoice-number order and return the
  same error as today when a customer is missing.
history:
  - status: open
    by: performance
```

#### R-performance-2 Regex compiled again for every invoice

```yaml
severity: medium
confidence: high
location: "src/report.rs:44-46"
evidence: |
  fn purchase_order(notes: &str) -> Option<String> {
      let pattern = Regex::new(r"PO-\d{4}-\d{4}").expect("a valid pattern");
recommendation: >-
  `purchase_order` is called once per invoice from the loop at
  `src/report.rs:28`, and each call compiles `PO-\d{4}-\d{4}` anew — a pattern
  compilation per item (about 3 000 per monthly report), far costlier than the
  match itself. Compile it once, in a `static PATTERN: std::sync::LazyLock<Regex>`
  (or `OnceLock`), or compile it in `monthly_report` before the loop and pass
  it in.
history:
  - status: open
    by: performance
```

### newcomer

Disabled by configuration.
