# Review record: add-monthly-report

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### performance

#### R-performance-1 One customer query per invoice in the monthly report

```yaml
severity: high
confidence: high
location: "src/report.rs:38-39"
evidence: |
  fn customer_label(db: &dyn Db, invoice: &Invoice) -> Result<String, DbError> {
      let customer = db.customer(invoice.customer_id)?;
recommendation: >-
  N+1 queries: `monthly_report` runs `SELECT ... FROM customers WHERE id = $1`
  once per invoice, so a month of about 3 000 invoices (proposal) costs
  about 3 000 round trips for only 800 distinct customers, and the cost
  grows linearly with the month's invoices on a report finance waits for.
  `Db` already offers `customers(&[i64])` (`WHERE id = ANY($1)`): collect
  the distinct `customer_id`s, load them in one call, index them in a
  `HashMap<i64, Customer>`, and build each label from the map — keeping
  the invoice-number order and returning an error when a customer is
  missing, as `db.customer` does today.
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
  let pattern = Regex::new(r"PO-\d{4}-\d{4}").expect("a valid pattern");
recommendation: >-
  `purchase_order` is called for each invoice of the month (line 28), and
  each call compiles the pattern again: about 3 000 regex compilations per
  report, a cost that grows with the invoices and dwarfs the match itself.
  Compile it once, in a `static` `std::sync::LazyLock<Regex>` (or
  `OnceLock`), and reuse it in `purchase_order`; the results stay the same.
history:
  - status: open
    by: performance
```

### newcomer

Disabled by configuration.
