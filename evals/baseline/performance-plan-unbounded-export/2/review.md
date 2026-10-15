# Review record: add-audit-export

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### performance

#### R-performance-1 The export loads the whole audit log into memory and builds one CSV string

```yaml
severity: high
confidence: high
location: "design.md#Decision: The export loads every event, then writes the CSV"
evidence: |
  The handler runs `SELECT * FROM audit_events WHERE org_id = $1 ORDER BY
  created_at`, loads the rows into an array, builds the whole CSV string with
  `rows.map(toCsvLine).join("\n")`, and sends it.
recommendation: >-
  Stream the export: read the rows with a server-side cursor or keyset
  batches on (org_id, created_at, id), turn each batch into CSV lines and
  write them to the response stream with backpressure, so memory stays
  bounded by the batch size whatever the log's size. This keeps the spec's
  "every event, oldest first" unchanged. The same streaming also fixes the
  event-loop stall: with the rows and the CSV built in one synchronous
  `map`/`join`, the instance's single event loop (design.md#Context) serves
  no other request until the whole string is built. Select only the columns
  the CSV needs instead of `SELECT *`. Record the streaming choice and its
  rejected alternative under "Alternatives considered".
history:
  - status: open
    by: performance
```

Cost grows per row of the organization's audit log, on a user-facing request:
with about 40 000 events per organization per month kept seven years
(design.md#Context), one export reaches some 3.4 million rows held at once as
row objects, then as an array of lines, then as one string — several copies of
the whole log in the process's memory, built synchronously on the instance's
only event loop. A large organization's export can exhaust the heap or exceed
the engine's maximum string length, and every other request on that instance
waits while it runs.

#### R-performance-2 No index is shown for the export's filter and sort

```yaml
severity: medium
confidence: low
location: "design.md#Decision: The export loads every event, then writes the CSV"
evidence: |
  The handler runs `SELECT * FROM audit_events WHERE org_id = $1 ORDER BY
  created_at`
recommendation: >-
  Check that `audit_events` has an index on (org_id, created_at) — the
  schema is not in the repository — and run EXPLAIN on the export query
  against a production-sized table; add the index in this change if it is
  missing, so the ordered read neither scans the whole table nor sorts
  millions of rows.
history:
  - status: open
    by: performance
```

### skeptic

Disabled by configuration.

### security

Disabled by configuration.
