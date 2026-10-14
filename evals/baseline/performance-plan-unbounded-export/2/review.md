# Review record: add-audit-export

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### performance

#### R-performance-1 The export loads the whole audit log and builds the CSV in memory on the event loop

```yaml
severity: high
confidence: high
location: "design.md#Decision: The export loads every event, then writes the CSV"
evidence: |
  The handler runs `SELECT * FROM audit_events WHERE org_id = $1 ORDER BY
  created_at`, loads the rows into an array, builds the whole CSV string with
  `rows.map(toCsvLine).join("\n")`, and sends it.
recommendation: >-
  The result set grows with the organization's history: the design's own
  Context gives about 40 000 events per organization and per month, kept seven
  years, so up to about 3.4 million rows per request. All of them are held in
  one array, then mapped to a second array of lines, then joined into one
  string — memory and CPU grow linearly with the log, and the
  `map`/`join` runs synchronously on the single event loop ("one event loop
  per instance"), stalling every other request on the instance for the
  duration; a few concurrent exports can exhaust the heap. The spec requires
  every event, so keep the full export but stream it: read the rows in
  batches (a server-side cursor or keyset pagination on `created_at, id`),
  write each batch's CSV lines to the response as a stream honouring
  backpressure, and keep the order oldest first. Record the alternative in
  the decision instead of "none".
history:
  - status: open
    by: performance
```

#### R-performance-2 No index is shown for the export's filter and sort

```yaml
severity: medium
confidence: low
location: "design.md#Decision: The export loads every event, then writes the CSV"
evidence: |
  The handler runs `SELECT * FROM audit_events WHERE org_id = $1 ORDER BY
  created_at`
recommendation: >-
  The schema of `audit_events` is not in the repository, so the index could
  not be checked. Confirm with `EXPLAIN` that an index on
  `(org_id, created_at)` serves both the filter and the sort; without it the
  query scans the table and sorts millions of rows per export, a cost growing
  with every organization's data. Add the index to the plan if it is missing.
history:
  - status: open
    by: performance
```

### skeptic

Disabled by configuration.

### security

Disabled by configuration.
