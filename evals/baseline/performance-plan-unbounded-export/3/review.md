# Review record: add-audit-export

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### skeptic

Disabled by configuration.

### security

Disabled by configuration.

### performance

#### R-performance-1 The export loads the whole audit log into memory and builds one CSV string on the event loop

```yaml
severity: high
confidence: high
location: "design.md#Decision: The export loads every event, then writes the CSV"
evidence: |
  The handler runs `SELECT * FROM audit_events WHERE org_id = $1 ORDER BY
  created_at`, loads the rows into an array, builds the whole CSV string with
  `rows.map(toCsvLine).join("\n")`, and sends it.
recommendation: >-
  Stream the export: read the rows with a database cursor or keyset batches
  (`WHERE org_id = $1 AND (created_at, id) > ($2, $3) ORDER BY created_at, id
  LIMIT n`), convert each batch to CSV lines and write them to the response
  stream, honouring backpressure. Memory then stays bounded by the batch size
  and the event loop is yielded between batches. Keep the spec's ordering
  (oldest first) and its "every event" result; only the way rows are read and
  written changes. Cost today grows linearly with the organization's history:
  per design.md#Context, about 40 000 rows a month kept seven years, that is
  up to ~3.4 million rows per organization held at once as row objects, then
  as an array of lines, then as one string — enough to exhaust the Node.js
  heap or exceed V8's maximum string length, and the synchronous map/join
  blocks the single event loop for every other request of the instance while
  it runs. Add a task that exercises the export on a large organization.
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
  created_at`
recommendation: >-
  The schema of `audit_events` is not in the repository, so this could not be
  verified. Check that an index on `(org_id, created_at)` exists (or add one
  to the plan), and run `EXPLAIN` on the query against a production-sized
  table: without it the database sorts every row of the organization, over a
  table holding seven years of events for all organizations, on each export.
history:
  - status: open
    by: performance
```
