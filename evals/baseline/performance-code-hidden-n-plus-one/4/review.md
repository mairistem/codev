# Review record: add-monthly-report

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### performance

#### R-performance-1 One customer query per invoice in the monthly report

```yaml
severity: medium
confidence: high
location: "src/report.rs:39"
evidence: |
  let customer = db.customer(invoice.customer_id)?;
recommendation: >-
  `monthly_report` calls `customer_label` once per invoice, and each call runs
  `SELECT ... FROM customers WHERE id = $1`: a month of about 3 000 invoices
  (proposal) runs about 3 000 customer queries for 800 distinct customers, and
  the count grows linearly with the month's invoices. Collect the distinct
  `customer_id`s of `invoices`, load them in one call with the existing
  `Db::customers(&ids)` (`WHERE id = ANY($1)`), index them in a `HashMap` by
  id, and build each label from the map. Keep the current error when a
  customer is missing and the invoice-number order of the lines.
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
  `purchase_order` is called for every invoice of the month and compiles the
  same pattern each time: about 3 000 regex compilations per report, a cost
  that grows with the month's invoices and dwarfs the match itself. Compile
  it once, in a `static` `std::sync::LazyLock<Regex>` (or `once_cell`), and
  reuse it in `purchase_order`; the results stay the same.
history:
  - status: open
    by: performance
```

### newcomer

Disabled by configuration.
