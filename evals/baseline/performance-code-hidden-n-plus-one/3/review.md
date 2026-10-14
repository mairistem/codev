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
  N+1 queries: called from the invoice loop (src/report.rs:23-26,
  `customer: customer_label(db, invoice)?`), `monthly_report` runs `SELECT ... FROM customers WHERE id = $1`
  once per invoice, so a month of about 3 000 invoices (proposal) costs about
  3 000 sequential round trips for only 800 distinct customers; the cost grows
  linearly with the month's invoices. `Db` already offers
  `customers(&[i64])` (`WHERE id = ANY($1)`, src/db.rs:26): collect the
  distinct `customer_id`s, fetch them in one call, index them in a
  `HashMap<i64, Customer>` and build each label from it, keeping the invoice
  order and returning the same error when a customer is missing.
history:
  - status: open
    by: performance
```

#### R-performance-2 The purchase-order regex is compiled again for every invoice

```yaml
severity: medium
confidence: high
location: "src/report.rs:44-46"
evidence: |
    let pattern = Regex::new(r"PO-\d{4}-\d{4}").expect("a valid pattern");
recommendation: >-
  `purchase_order` is called once per invoice in the report loop
  (src/report.rs:28) and compiles the same pattern on each call, so a month of
  about 3 000 invoices compiles it about 3 000 times; compiling a regex is
  far more expensive than matching one short note. Compile it once, in a
  `static PATTERN: std::sync::LazyLock<Regex>` (or `OnceLock`), and reuse it.
history:
  - status: open
    by: performance
```

### newcomer

Disabled by configuration.
