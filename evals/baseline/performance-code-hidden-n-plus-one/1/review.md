# Review record: add-monthly-report

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### performance

#### R-performance-1 One customer query per invoice (N+1)

```yaml
severity: medium
confidence: high
location: "src/report.rs:39"
evidence: |
  let customer = db.customer(invoice.customer_id)?;
recommendation: >-
  The loop of `monthly_report` (`src/report.rs:23-30`) calls, through
  `customer_label` (`src/report.rs:26`), `db.customer` (`SELECT ... FROM customers WHERE id = $1`) once
  per invoice, so a report runs 1 + N + 1 queries: about 3 000 round trips for
  a month of 3 000 invoices from only 800 customers, and the count grows with
  the month's invoices. `Db::customers(&[i64])` (`WHERE id = ANY($1)`) already
  serves the whole set in one query: collect the distinct `customer_id`s,
  fetch them once, index them in a `HashMap<i64, Customer>` and build each
  label from it. Keep the current behavior when a customer is missing (an
  error), and keep the invoice-number order.
history:
  - status: open
    by: performance
```

#### R-performance-2 The purchase-order regex is compiled for every invoice

```yaml
severity: low
confidence: high
location: "src/report.rs:44-46"
evidence: |
  fn purchase_order(notes: &str) -> Option<String> {
      let pattern = Regex::new(r"PO-\d{4}-\d{4}").expect("a valid pattern");
recommendation: >-
  `purchase_order` is called once per invoice (`src/report.rs:28`) and compiles
  the same pattern each time — about 3 000 compilations per report, a cost far
  larger than the match itself. Compile it once: a `static` with
  `std::sync::OnceLock<Regex>` (or `LazyLock`), or build it in
  `monthly_report` and pass it in.
history:
  - status: open
    by: performance
```

### newcomer

Disabled by configuration.
