# Review record: add-audit-export

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### performance

#### R-performance-1 Export loads the whole audit log into memory and builds the CSV synchronously

```yaml
severity: high
confidence: high
location: "design.md#Decision: The export loads every event, then writes the CSV"
evidence: |
  The handler runs `SELECT * FROM audit_events WHERE org_id = $1 ORDER BY
  created_at`, loads the rows into an array, builds the whole CSV string with
  `rows.map(toCsvLine).join("\n")`, and sends it.
recommendation: >-
  Stream the export instead of materializing it: read `audit_events` through a
  server-side cursor (or keyset pages on `created_at`, id) in bounded batches,
  convert each batch to CSV lines and write them to the response stream with
  backpressure, so memory stays constant and the event loop is yielded between
  batches. Keep the order (oldest first) and the "every event" result the spec
  requires. Select only the exported columns rather than `SELECT *`. Record the
  rejected "load everything" alternative in the design, and back the choice with
  a measurement on a seven-year organization (memory peak, event-loop lag).
  Also check that an index on `audit_events (org_id, created_at)` exists — the
  schema is not in the repository — and confirm with `EXPLAIN`.
history:
  - status: open
    by: performance
```

Cost grows linearly with the organization's history, per request, on a
user-facing path. With the design's own figures — about 40 000 events per
organization per month, kept seven years — one export reads up to about
3.4 million rows. The handler holds every row object in an array, then a second
array of CSV lines, then one string of the whole file: several copies of the
log in the heap of a single Node.js process, close to or beyond V8's string
and heap limits. `rows.map(...).join("\n")` runs synchronously on the one event
loop of the instance (`design.md#Context`: "one event loop per instance"), so
every other request on that instance stalls for the whole build; two
administrators exporting at once can exhaust memory and crash the instance.

### skeptic

Disabled by configuration.

### security

Disabled by configuration.
