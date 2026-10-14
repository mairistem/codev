# Review record: add-monthly-report

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### performance

#### R-performance-1 One customer query per invoice (N+1) in the monthly report

```yaml
severity: medium
confidence: high
location: "src/report.rs:39"
evidence: |
  let customer = db.customer(invoice.customer_id)?;
recommendation: >-
  Collect the distinct `customer_id`s of the month's invoices, load them in one
  query with the existing `Db::customers(&ids)` (`WHERE id = ANY($1)`), index
  them in a `HashMap<i64, Customer>` and build each label from the map. The
  report then runs three queries whatever the month's volume, instead of
  2 + one per invoice; keep the invoice-number order and the error behavior
  (a missing customer still fails the report).
history:
  - status: open
    by: performance
```

Cost grows with the number of invoices: `monthly_report` runs one
`SELECT … FROM customers WHERE id = $1` per invoice. With the proposal's
volume (about 3 000 invoices a month from 800 customers), one report issues
about 3 000 round-trips, most of them re-reading a customer already loaded,
although `Db::customers` serves the whole set in one call.

#### R-performance-2 The purchase-order regex is compiled once per invoice

```yaml
severity: low
confidence: high
location: "src/report.rs:44-46"
evidence: |
  let pattern = Regex::new(r"PO-\d{4}-\d{4}").expect("a valid pattern");
recommendation: >-
  Compile the pattern once — a `static PATTERN: LazyLock<Regex>` (or
  `once_cell::sync::Lazy`) — or compile it in `monthly_report` before the
  loop and pass it to `purchase_order`. Results are unchanged.
history:
  - status: open
    by: performance
```

`purchase_order` is called for every invoice of the loop (`src/report.rs:28`),
so a report over about 3 000 invoices compiles the same regex about 3 000
times; compiling a pattern is expensive compared with matching it on one
short notes field.

### newcomer

Disabled by configuration.
