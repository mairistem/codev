# Review record: add-monthly-report

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### performance

#### R-performance-1 One customer query per invoice (N+1)

```yaml
severity: high
confidence: high
location: "src/report.rs:39"
evidence: |
  let customer = db.customer(invoice.customer_id)?;
recommendation: >-
  The loop runs `SELECT ... FROM customers WHERE id = $1` once per invoice: with the
  proposal's ~3 000 invoices a month that is ~3 000 sequential round trips (for only
  ~800 distinct customers), growing linearly with invoice volume on the report finance
  reads. Collect the distinct `customer_id`s, load them in one call with the existing
  `db.customers(&ids)` (`WHERE id = ANY($1)`), index them in a `HashMap<i64, Customer>`,
  and build each label from the map. Keep the current error behavior for an invoice
  whose customer is missing (return a `DbError` rather than skipping the line).
history:
  - status: open
    by: performance
```

#### R-performance-2 The purchase-order regex is compiled once per invoice

```yaml
severity: medium
confidence: high
location: "src/report.rs:45"
evidence: |
  let pattern = Regex::new(r"PO-\d{4}-\d{4}").expect("a valid pattern");
recommendation: >-
  `purchase_order` is called for every invoice and calls `Regex::new` each time, so the
  pattern is parsed and compiled ~3 000 times per report, a cost far larger than the
  match itself and growing with invoice volume. Compile it once — a
  `static PATTERN: std::sync::LazyLock<Regex>` (or `once_cell::sync::Lazy`), or build
  it in `monthly_report` before the loop and pass `&Regex` in.
history:
  - status: open
    by: performance
```

### newcomer

Disabled by configuration.
