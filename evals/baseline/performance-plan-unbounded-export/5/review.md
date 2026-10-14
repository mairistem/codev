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
  Cost grows linearly with the organization's history, per request: the design's
  own context gives about 40 000 events per organization per month kept seven
  years, so an export reaches about 3.4 million rows. The query has no LIMIT, every
  row is held in one array, then copied once more into an array of lines and once
  more into a single string, and the map/join runs synchronously on the single
  event loop of the instance ("one event loop per instance"), stalling every other
  request it serves until it ends, with a memory peak that can exhaust the heap.
  Keep the requirement (every event, oldest first) but stream it: read the rows
  through a server-side cursor or keyset batches (`WHERE org_id = $1 AND
  (created_at, id) > $last ORDER BY created_at, id LIMIT n`), write each batch's
  CSV lines to the response stream with backpressure, and never hold the whole
  set or string. Record the decision's alternatives (streaming, an asynchronous
  export job) instead of "none", and back the chosen approach with a measurement
  on an organization with seven years of events.
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
  `SELECT * FROM audit_events WHERE org_id = $1 ORDER BY
  created_at`
recommendation: >-
  The schema of `audit_events` is not in the repository, so the existing indexes
  could not be checked. Without an index on `(org_id, created_at)`, each export
  scans and sorts a table that grows with every organization's seven years of
  events. Check the query plan (`EXPLAIN`) and, if missing, add the index in this
  change; keyset batches (R-performance-1) need it too.
history:
  - status: open
    by: performance
```

### skeptic

Disabled by configuration.

### security

Disabled by configuration.
