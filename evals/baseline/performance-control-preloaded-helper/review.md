# Review record: add-monthly-report

<!-- codev-review: 1 -->

## code.review

### performance

#### R-performance-1 The month total costs a second scan of the month's invoices

```yaml
severity: low
confidence: high
location: "src/report.rs:43-46"
evidence: |
  Ok(Report {
      lines,
      total_cents: db.month_total(month)?,
  })
recommendation: Sum `total_cents` over the invoices already loaded by `invoices_of_month` instead of running `month_total`, which re-reads every invoice of the month (one extra aggregate query per report, growing with the month's invoices). This also makes the total consistent with the listed lines; confirm finance expects the total of the listed lines before changing it.
history:
  - status: open
    by: performance
```

#### R-performance-2 The `regex` crate is added for one fixed-shape pattern

```yaml
severity: low
confidence: high
location: "Cargo.toml:8"
evidence: |
  regex = "1"
recommendation: Weigh the dependency's cost (compile time, binary size, transitive `regex-syntax`/`aho-corasick`/`memchr`) against matching `PO-\d{4}-\d{4}` by hand — scan for `PO-` and check the 4 + `-` + 4 ASCII digits that follow — or record in the proposal why the crate is worth it.
history:
  - status: open
    by: performance
```

#### R-performance-3 The customer label is formatted once per invoice, not once per customer

```yaml
severity: low
confidence: high
location: "src/report.rs:51-55"
evidence: |
  fn customer_label(customers: &HashMap<i64, Customer>, invoice: &Invoice) -> String {
      match customers.get(&invoice.customer_id) {
          Some(c) => format!("{} ({})", c.name, c.country),
recommendation: Build the "Name (country)" label once per customer when filling the map (about 800 per month) and clone it per line, instead of a `format!` per invoice (about 3 000 per month); or keep as is given the small volumes.
history:
  - status: open
    by: performance
```

#### R-performance-4 Invoice numbers are cloned while the invoices are dropped right after

```yaml
severity: low
confidence: high
location: "src/report.rs:35-37"
evidence: |
  for invoice in &invoices {
      lines.push(ReportLine {
          number: invoice.number.clone(),
recommendation: Iterate `invoices` by value (`into_iter()`) and move `number` into the line, avoiding one `String` copy per invoice.
history:
  - status: open
    by: performance
```

#### R-performance-5 The month's invoices are loaded without a bound, on a filter whose index is not visible

```yaml
severity: low
confidence: low
location: "src/report.rs:25"
evidence: |
  let invoices = db.invoices_of_month(month)?;
recommendation: The report reads every invoice of the month (about 3 000 today, growing with sales) in one result, with no `LIMIT` or streaming, through `SELECT … FROM invoices WHERE month = $1 ORDER BY number` (src/db.rs:21), which filters and sorts on `invoices.month`/`number`; the schema is not in the repository, so check with `EXPLAIN` that an index on `(month, number)` serves both, and decide whether a bound or streaming is needed as volumes grow.
history:
  - status: open
    by: performance
```

### newcomer

Disabled by configuration.
