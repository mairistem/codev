# Review record: add-audit-export

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### performance

#### R-performance-1 The export loads the whole audit log into memory and builds the CSV on the event loop

```yaml
severity: high
confidence: high
location: "design.md#Decision: The export loads every event, then writes the CSV"
evidence: |
  The handler runs `SELECT * FROM audit_events WHERE org_id = $1 ORDER BY
  created_at`, loads the rows into an array, builds the whole CSV string with
  `rows.map(toCsvLine).join("\n")`, and sends it.
recommendation: >-
  Cost grows linearly with the organization's history, per request. The design's own
  context gives about 40 000 events per organization per month, kept seven years:
  up to about 3.4 million rows for one export. `query()` in `src/db.ts` returns
  `result.rows` as one array, so the handler holds every row, then an array of CSV
  lines, then the joined string, all at once — hundreds of megabytes per request,
  enough to exhaust the heap or hit V8's maximum string length. `rows.map(...).join`
  is synchronous: on a service with "one event loop per instance" it blocks every
  other request of the instance for the whole build. Stream instead: read the rows
  through a server-side cursor (`pg-cursor` or `pg-query-stream`) in batches, write
  each batch's CSV lines to the response as they come, and respect backpressure.
  Replace "Alternatives considered: none" with this alternative and why it is or is
  not chosen. Ordering (oldest first) and the set of rows stay unchanged.
history:
  - status: open
    by: performance
```

#### R-performance-2 The requirement exports every event with no bound on the result set

```yaml
severity: medium
confidence: high
location: "specs/audit-export/spec.md#Requirement: Administrators export the audit log"
evidence: |
  `GET /admin/audit-events.csv` SHALL return every audit event of the organization as CSV, oldest first.
recommendation: >-
  The result set grows without limit — up to seven years of events per request — and
  the requirement allows no way to ask for less: an auditor wanting one quarter still
  pulls the whole history, holding a database connection and a full scan of the
  organization's rows for the length of the transfer, and any administrator can
  repeat it at will. Bound it: add `from` / `to` date parameters (with a default or
  maximum range) and a scenario for them, or state explicitly that the unbounded
  export is intended and rely on streaming (R-performance-1) plus a rate limit. This
  is a separate fix from streaming: streaming bounds memory, not the work per request.
history:
  - status: open
    by: performance
```

#### R-performance-3 No index is stated for the export's filter and sort

```yaml
severity: medium
confidence: low
location: "design.md#Decision: The export loads every event, then writes the CSV"
evidence: |
  The handler runs `SELECT * FROM audit_events WHERE org_id = $1 ORDER BY
  created_at`
recommendation: >-
  The schema of `audit_events` is not in the repository, so the index cannot be
  checked. Without an index on `(org_id, created_at)` the query scans the whole table
  (every organization's events) and sorts the result, a cost that grows with the
  total table size, not just the organization's. Check the existing indexes with
  `EXPLAIN` on a production-sized table, and add a task to create the composite
  index if it is missing.
history:
  - status: open
    by: performance
```
