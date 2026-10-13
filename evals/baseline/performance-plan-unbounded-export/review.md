# Review record: add-audit-export

<!-- codev-review: 1 -->

## plan.challenge

### skeptic

Disabled by configuration.

### security

Disabled by configuration.

### performance

#### R-performance-1 The export loads the organization's whole audit log into memory

```yaml
severity: high
confidence: high
location: "design.md#Decision: The export loads every event, then writes the CSV"
evidence: |
  The handler runs `SELECT * FROM audit_events WHERE org_id = $1 ORDER BY
  created_at`, loads the rows into an array, builds the whole CSV string with
  `rows.map(toCsvLine).join("\n")`, and sends it.
recommendation: >-
  Stream the rows instead of buffering them: read with a server-side cursor
  (for example `pg-cursor` or `pg-query-stream`) or keyset pages on
  `(created_at, id)` of a few thousand rows, and write each CSV line to the
  response as it is read, honouring back-pressure. Memory then stays
  constant per request instead of growing with the log.
history:
  - status: open
    by: performance
```

The design's own context gives the size: about 40 000 events per
organization per month, kept seven years, so up to about 3.4 million rows
held in one array, then again as one CSV string, per request. Cost grows
linearly with the retained history on a user-facing endpoint; a few
concurrent exports can exhaust the instance's heap.

#### R-performance-2 Building the CSV with map/join blocks the single event loop

```yaml
severity: high
confidence: high
location: "design.md#Decision: The export loads every event, then writes the CSV"
evidence: |
  builds the whole CSV string with
  `rows.map(toCsvLine).join("\n")`, and sends it.
recommendation: >-
  Produce the CSV incrementally from the row stream (a Transform stream
  piped to `res`), so serialization work is spread across ticks and other
  requests keep being served during an export.
history:
  - status: open
    by: performance
```

The design states "The API is a Node.js service with one event loop per
instance." Mapping and joining millions of rows is synchronous CPU work and
string allocation proportional to the log size; while it runs, every other
request on the instance, `/health` included, waits.

#### R-performance-3 The requirement mandates an unbounded result with no range

```yaml
severity: medium
confidence: high
location: "specs/audit-export/spec.md#Requirement: Administrators export the audit log"
evidence: |
  `GET /admin/audit-events.csv` SHALL return every audit event of the organization as CSV, oldest first.
recommendation: >-
  Either state that the response is streamed (so "every event" is
  delivered without buffering) or add an optional date range (`from`,
  `to`) and a scenario covering a large log; leave the choice to the
  author, since a range changes observable behaviour.
history:
  - status: open
    by: performance
```

"Every audit event" over a seven-year retention is a result set that grows
without bound, and no scenario covers its size, so the tests in task 1.1
("An export", "Not an administrator") would pass with the buffering design.

#### R-performance-4 No index is named for the export's filter and sort

```yaml
severity: medium
confidence: low
location: "design.md#Decision: The export loads every event, then writes the CSV"
evidence: |
  The handler runs `SELECT * FROM audit_events WHERE org_id = $1 ORDER BY
  created_at`
recommendation: >-
  Check with `EXPLAIN` that an index on `audit_events (org_id, created_at)`
  exists, and add it to the plan (a migration task) if it does not; without
  it the query scans and sorts the whole table, all organizations included.
history:
  - status: open
    by: performance
```

The repository holds no schema or migration for `audit_events`, so whether
the index exists could not be verified; the design does not mention it.

#### R-performance-5 The approach is chosen without weighing alternatives or measuring

```yaml
severity: low
confidence: high
location: "design.md#Decision: The export loads every event, then writes the CSV"
evidence: |
  **Alternatives considered**: none; the code is the simplest.
recommendation: >-
  Record the streaming alternative and the reason for the choice, and back
  it with a measurement of memory and latency on a realistic volume (for
  example one organization with seven years of events).
history:
  - status: open
    by: performance
```
