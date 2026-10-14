# Review record: add-audit-export

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### performance

#### R-performance-1 Export loads the whole audit log into memory and builds the CSV on the event loop

```yaml
severity: high
confidence: high
location: "design.md#Decision: The export loads every event, then writes the CSV"
evidence: |
  The handler runs `SELECT * FROM audit_events WHERE org_id = $1 ORDER BY
  created_at`, loads the rows into an array, builds the whole CSV string with
  `rows.map(toCsvLine).join("\n")`, and sends it.
recommendation: >-
  Cost grows linearly with the organization's history and is paid in one
  request: at the design's own volumes (about 40 000 events per month, kept
  seven years) one export reads up to ~3.4 million rows with no LIMIT,
  materializes them all as objects through `query()` in src/db.ts (which
  returns `result.rows` whole), then holds a second full copy as the
  `rows.map(...)` array and a third as the joined string — hundreds of MB
  per request, enough to exhaust the heap of an instance or let two
  concurrent exports kill it. The `map`/`join` over millions of rows is also
  synchronous CPU work on the single event loop the design mentions, so every
  other request on that instance stalls for the duration. Stream instead:
  read with a server-side cursor (pg-cursor / pg-query-stream, or keyset
  pagination on (created_at, id) in batches of a few thousand), convert each
  batch to CSV lines and write it to the response honoring backpressure, so
  memory stays bounded by the batch size and the event loop yields between
  batches. Record the rejected "load everything" alternative under
  "Alternatives considered", and keep the ordering (oldest first) and the
  "every event" result the spec requires.
history:
  - status: open
    by: performance
```

#### R-performance-2 No index stated for the org_id filter and created_at sort

```yaml
severity: medium
confidence: low
location: "design.md#Decision: The export loads every event, then writes the CSV"
evidence: |
  The handler runs `SELECT * FROM audit_events WHERE org_id = $1 ORDER BY
  created_at`, loads the rows into an array, builds the whole CSV string with
recommendation: >-
  The schema of `audit_events` is not in the repository, so whether an index
  serves this query cannot be verified. Without an index on
  (org_id, created_at), each export scans and sorts the whole table — all
  organizations' events over seven years — and the cost grows with total
  data, not with the requesting organization's. Check with EXPLAIN on a
  production-sized table and state in the design the index the query relies
  on (adding it if missing); a cursor or keyset stream (R-performance-1)
  also needs this index to read in order without a sort.
history:
  - status: open
    by: performance
```
