# Review record: add-monthly-report

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### performance

#### R-performance-1 One customer query per invoice in the monthly report

```yaml
severity: high
confidence: high
location: "src/report.rs:38-40"
evidence: |
  let customer = db.customer(invoice.customer_id)?;
recommendation: >-
  N+1 queries: `customer_label` is called once per invoice inside the loop of
  `monthly_report` (src/report.rs:23-26), so a month of about 3 000 invoices
  runs about 3 000 `SELECT ... FROM customers WHERE id = $1` queries, growing
  linearly with the month's invoices, while only about 800 distinct customers
  exist. `Db::customers(&[i64])` (src/db.rs:26, `WHERE id = ANY($1)`) already
  serves the whole set: collect the distinct `customer_id`s, load them in one
  call, index them in a `HashMap<i64, Customer>` and build each label from it,
  keeping the invoice-number order and returning the same error when a
  customer is missing.
history:
  - status: open
    by: performance
```

#### R-performance-2 The purchase-order pattern is compiled once per invoice

```yaml
severity: medium
confidence: high
location: "src/report.rs:45"
evidence: |
  let pattern = Regex::new(r"PO-\d{4}-\d{4}").expect("a valid pattern");
recommendation: >-
  `purchase_order` is called for every invoice (src/report.rs:28) and compiles
  the regex on each call, so a 3 000-invoice month compiles the same pattern
  3 000 times — compiling a pattern is expensive compared with matching one
  short note. Compile it once, in a `static` `std::sync::LazyLock<Regex>` (or
  `once_cell`), and reuse it; the matches stay the same.
history:
  - status: open
    by: performance
```

### newcomer

Disabled by configuration.
