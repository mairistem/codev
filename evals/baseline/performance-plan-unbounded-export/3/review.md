# Review record: add-audit-export

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### skeptic

Disabled by configuration.

### security

Disabled by configuration.

### performance

#### R-performance-1 The export loads the whole audit log and the whole CSV into memory on the event loop

```yaml
severity: high
confidence: high
location: "design.md#Decision: The export loads every event, then writes the CSV"
evidence: |
  The handler runs `SELECT * FROM audit_events WHERE org_id = $1 ORDER BY
  created_at`, loads the rows into an array, builds the whole CSV string with
  `rows.map(toCsvLine).join("\n")`, and sends it.
recommendation: >-
  With about 40 000 events per organization and per month kept seven years
  (design.md#Context), one export reaches about 3.4 million rows. The query
  has no bound, `src/db.ts` `query()` buffers every row in `result.rows`, and
  the handler then holds a second copy as one CSV string: memory per request
  grows linearly with the organization's history, two concurrent exports can
  exhaust the heap, and the synchronous `map`/`join` over millions of rows
  blocks the single event loop of the instance (design.md#Context), stalling
  every other request it serves. Stream the export instead: read the rows
  with a server-side cursor (e.g. `pg-cursor` or `pg-query-stream`) in
  batches, write each CSV line to the response as it comes, and respect
  backpressure; this keeps the requirement "every event, oldest first" while
  bounding memory per request. Alternatively, bound the export by a date
  range parameter. Record the chosen approach as a decision and replace
  "Alternatives considered: none".
history:
  - status: open
    by: performance
```

#### R-performance-2 No index is stated for the export's filter and sort

```yaml
severity: medium
confidence: low
location: "design.md#Decision: The export loads every event, then writes the CSV"
evidence: |
  The handler runs `SELECT * FROM audit_events WHERE org_id = $1 ORDER BY
recommendation: >-
  The query filters on `org_id` and sorts on `created_at` over a table that
  grows by about 40 000 rows per organization and per month. The schema of
  `audit_events` is not in the repository, so the index could not be
  checked: confirm an index on `(org_id, created_at)` exists (check with
  `EXPLAIN` that the plan avoids a sort over the organization's rows), or
  add one, and state it in the design.
history:
  - status: open
    by: performance
```
