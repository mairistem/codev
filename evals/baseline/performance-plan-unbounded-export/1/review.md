# Review record: add-audit-export

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### performance

#### R-performance-1 The export buffers the whole audit log and builds the CSV synchronously on the event loop

```yaml
severity: high
confidence: high
location: "design.md#Decision: The export loads every event, then writes the CSV"
evidence: |
  The handler runs `SELECT * FROM audit_events WHERE org_id = $1 ORDER BY
  created_at`, loads the rows into an array, builds the whole CSV string with
  `rows.map(toCsvLine).join("\n")`, and sends it.
recommendation: >-
  Cost grows linearly with the organization's history, on a user-facing request:
  the design's own Context gives about 40 000 events per organization and per
  month kept seven years, so one export reaches about 3.4 million rows. With
  `query()` in src/db.ts returning `result.rows` as one array, every row is held
  in memory, then a second copy as one CSV string, then `map`/`join` run
  synchronously on the single event loop ("one event loop per instance"),
  stalling every other request of the instance for the whole build, and a few
  concurrent exports can exhaust the process memory (a single JS string is also
  capped near 512 MB). The requirement "every audit event ... oldest first" can
  be kept while bounding the work: read the rows with a server-side cursor
  (e.g. `pg-cursor`/`pg-query-stream`, or keyset batches on
  `(created_at, id)` with a LIMIT), write each batch's CSV lines to the response
  as a stream honouring backpressure, and select only the exported columns
  instead of `SELECT *`. Record this choice in the design (replacing
  "Alternatives considered: none") and add a task that exercises the export on a
  realistic volume, measuring memory and event-loop delay.
history:
  - status: open
    by: performance
```

#### R-performance-2 No index is planned for the export's filter and sort

```yaml
severity: medium
confidence: low
location: "design.md#Decision: The export loads every event, then writes the CSV"
evidence: |
  `SELECT * FROM audit_events WHERE org_id = $1 ORDER BY
  created_at`
recommendation: >-
  The schema of `audit_events` is not in the repository, so whether an index
  covers `(org_id, created_at)` could not be checked. Without one, each export
  scans and sorts a table holding every organization's seven years of events,
  a cost that grows with the whole table rather than with the organization.
  Check the existing indexes (and `EXPLAIN` the query); if missing, plan an
  index on `(org_id, created_at)` — which keyset batching from R-performance-1
  also needs — and say so in the design and tasks.
history:
  - status: open
    by: performance
```

### skeptic

Disabled by configuration.

### security

Disabled by configuration.
