# Review record: add-audit-export

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### performance

#### R-performance-1 The export loads the whole audit log into memory and builds it on the event loop

```yaml
severity: high
confidence: high
location: "design.md#Decision: The export loads every event, then writes the CSV"
evidence: |
  created_at`, loads the rows into an array, builds the whole CSV string with
  `rows.map(toCsvLine).join("\n")`, and sends it.
recommendation: >-
  The result is unbounded and grows with the organization's history: the
  design's own figures (about 40 000 events per organization per month, kept
  seven years) give up to about 3.4 million rows for one request. Per export,
  memory holds every row object, the array of CSV lines and the joined string
  at once, and the synchronous map/join runs on the instance's single event
  loop, stalling every other request it serves until it finishes; two
  concurrent exports can exhaust the heap. Stream instead: read with a
  server-side cursor or keyset batches on (created_at, id), format each batch
  to CSV and write it to the response as a stream honouring backpressure
  (e.g. pipeline from a query stream through a CSV transform). This keeps the
  spec's "every audit event, oldest first" and its ordering unchanged. Record
  the choice and its alternatives in the design (the current "Alternatives
  considered: none" gives no reason), and update task 1.1 accordingly.
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
recommendation: >-
  The query filters on org_id and sorts on created_at over a table holding
  seven years of every organization's events. The schema of audit_events is
  not in the repository, so whether an index on (org_id, created_at) exists
  could not be checked. Check it with EXPLAIN on a production-sized table; if
  missing, add it (or state in the design that it exists), otherwise each
  export does a full scan and sort of the table.
history:
  - status: open
    by: performance
```
