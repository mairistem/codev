# Review record: add-audit-export

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### performance

#### R-performance-1 The export buffers the whole audit log in memory and builds the CSV in one synchronous call

```yaml
severity: high
confidence: high
location: "design.md#Decision: The export loads every event, then writes the CSV"
evidence: |
  The handler runs `SELECT * FROM audit_events WHERE org_id = $1 ORDER BY
  created_at`, loads the rows into an array, builds the whole CSV string with
  `rows.map(toCsvLine).join("\n")`, and sends it.
recommendation: >-
  Cost grows linearly with the organization's history, on a user-facing
  request: at the design's own figures (about 40 000 events per month, kept
  seven years) one export reaches about 3.4 million rows. `src/db.ts` `query`
  returns `result.rows` as one array, so every row is held in memory, then a
  second copy is held as the per-line strings and a third as the joined CSV
  string; `rows.map(toCsvLine).join("\n")` runs synchronously on the
  instance's single event loop, stalling every other request on that
  instance for the duration, and a few concurrent exports can exhaust the
  heap. Stream the export instead: read in keyset-paginated batches with the
  existing `query` helper (`WHERE org_id = $1 AND (created_at, id) > ($2, $3)
  ORDER BY created_at, id LIMIT 5000`), write each batch's CSV lines to the
  response and wait for `drain` when `res.write` returns false, so memory
  stays bounded by one batch and the event loop is yielded between batches
  (a cursor via `pg-query-stream` is an alternative, at the cost of a new
  dependency to weigh). Keep the order oldest first, as the requirement
  states. Replace "Alternatives considered: none" with this comparison, and
  add a task that checks memory and event-loop latency on a large export.
history:
  - status: open
    by: performance
```

#### R-performance-2 The export's filter and sort may lack a supporting index

```yaml
severity: medium
confidence: low
location: "design.md#Decision: The export loads every event, then writes the CSV"
evidence: |
  The handler runs `SELECT * FROM audit_events WHERE org_id = $1 ORDER BY
  created_at`
recommendation: >-
  The `audit_events` schema is not in the repository, so the index could not
  be checked. Without an index on `(org_id, created_at)` (plus `id` for the
  keyset of R-performance-1), each export scans the whole table, which holds
  every organization's seven years of events, and sorts the result: cost
  grows with the total table size, not with the organization's share. Check
  the existing indexes with `EXPLAIN`, and if one is missing, add it to the
  design and to the tasks.
history:
  - status: open
    by: performance
```

### skeptic

Disabled by configuration.

### security

Disabled by configuration.
