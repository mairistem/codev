# Review record: add-monthly-report

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### performance

#### R-performance-1 Index on invoices.month not verifiable

```yaml
severity: low
confidence: low
location: "src/report.rs:25"
evidence: |
  let invoices = db.invoices_of_month(month)?;
recommendation: The report runs two queries filtering `invoices` on `month` (`invoices_of_month` and `month_total`, see `src/db.rs:21` and `src/db.rs:27`); the schema is not in the repository, so check with `EXPLAIN` that `invoices` has an index on `month` (ideally `(month, number)` to serve the `ORDER BY number`). Without it, each report scans the whole invoices table, a cost that grows with every month of history rather than with the month's ~3 000 invoices.
history:
  - status: open
    by: performance
```

### newcomer

Disabled by configuration.
