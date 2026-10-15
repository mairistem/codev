# Review record: add-audit-export

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### performance

#### R-performance-1 The export loads the whole audit log of an organization into memory

```yaml
severity: high
confidence: high
location: "design.md#Decision: The export loads every event, then writes the CSV"
evidence: |
  The handler runs `SELECT * FROM audit_events WHERE org_id = $1 ORDER BY
  created_at`, loads the rows into an array, builds the whole CSV string with
recommendation: >-
  The query has no bound and `src/db.ts` `query()` returns `result.rows`, so the
  whole result is buffered. With the design's own figures (about 40 000 events
  per organization per month, kept seven years) one request materializes up to
  ~3.4 million row objects plus a CSV string of the same size in one Node.js
  process: memory grows linearly with the organization's history, and a few
  concurrent exports can exhaust the heap and crash the instance for every user.
  The spec requires every event, so do not add a LIMIT: read the rows in bounded
  batches (a server-side cursor such as `pg-cursor`/`pg-query-stream`, or keyset
  pagination on `(created_at, id)`) and stream each batch to the response,
  respecting backpressure. Select only the exported columns instead of `*`.
  Record the decision with its alternatives, since "none" were considered.
history:
  - status: open
    by: performance
```

#### R-performance-2 Building the whole CSV with map/join blocks the event loop

```yaml
severity: high
confidence: high
location: "design.md#Decision: The export loads every event, then writes the CSV"
evidence: |
  `rows.map(toCsvLine).join("\n")`, and sends it.
recommendation: >-
  The design states the API runs "one event loop per instance". Formatting and
  joining millions of rows is synchronous CPU work proportional to the size of
  the log: while one administrator exports, every other request on that
  instance (including `/health`) waits, and the time grows with each month of
  retained events. Write the CSV incrementally — format each batch from the
  cursor and `res.write` it, awaiting `drain` — so the loop yields between
  batches; set `Content-Type: text/csv` with chunked transfer.
history:
  - status: open
    by: performance
```

#### R-performance-3 No index is stated for the org_id filter sorted by created_at

```yaml
severity: medium
confidence: low
location: "design.md#Decision: The export loads every event, then writes the CSV"
evidence: |
  The handler runs `SELECT * FROM audit_events WHERE org_id = $1 ORDER BY
  created_at`
recommendation: >-
  The schema of `audit_events` is not in the repository, so the index could not
  be checked. Verify with `EXPLAIN` that an index on `(org_id, created_at)`
  exists; otherwise the query scans and sorts a table that holds every
  organization's seven years of events. Keyset pagination (R-performance-1)
  also depends on that index.
history:
  - status: open
    by: performance
```

#### R-performance-4 The choice of the simplest approach is not backed by a measurement

```yaml
severity: low
confidence: low
location: "design.md#Decision: The export loads every event, then writes the CSV"
evidence: |
  **Alternatives considered**: none; the code is the simplest.
recommendation: >-
  Measure the export on an organization with the full retention (memory peak,
  duration, event-loop delay) before choosing, and add a task that verifies the
  export of a large log stays within a stated memory and latency budget.
history:
  - status: open
    by: performance
```

### skeptic

Disabled by configuration.

### security

Disabled by configuration.
