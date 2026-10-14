# Review record: add-monthly-report

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### performance

#### R-performance-1 No visible index backs the month filter the report makes hot

```yaml
severity: medium
confidence: low
location: "src/report.rs:25"
evidence: |
  let invoices = db.invoices_of_month(month)?;
recommendation: >-
  The report is the first caller of `invoices_of_month` and `month_total`,
  both filtering `invoices` on `month` (`WHERE month = $1`, src/db.rs:21,27).
  The schema is not in the repository, so check with `EXPLAIN` that an index
  on `invoices(month)` exists — ideally `(month, number)` to serve the
  `ORDER BY number` too; without one, each report scans the whole invoices
  table twice, a cost growing with every month of history rather than with
  the ~3 000 invoices of the month.
history:
  - status: open
    by: performance
```

### newcomer

Disabled by configuration.
