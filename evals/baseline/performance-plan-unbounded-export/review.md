# Review record: add-audit-export

<!-- codev-review: 1 -->

## plan.challenge

### performance

#### R-performance-1 The export loads the whole audit log of an organization into memory

```yaml
severity: high
confidence: high
location: "design.md#Decision: The export loads every event, then writes the CSV"
evidence: |
  The handler runs `SELECT * FROM audit_events WHERE org_id = $1 ORDER BY
  created_at`, loads the rows into an array, builds the whole CSV string with
recommendation: >-
  Do not materialize the result: read the rows through a database cursor (or
  keyset pages on created_at, id) and stream each CSV line to the response,
  honouring backpressure. Cost is then constant in memory instead of linear
  in the organization's history. If a bound is preferred (e.g. a date range
  per request), that changes the spec and must be decided there.
history:
  - status: open
    by: performance
```

The query has no `LIMIT` and no range, and the spec requires every event.
With the design's own figures (about 40 000 events per organization per month,
kept seven years) one request reads up to about 3.4 million rows, all held in
one array per request, on a user-facing endpoint: memory grows per row and per
concurrent export, and can exhaust the instance.

#### R-performance-2 Building the whole CSV string blocks the event loop

```yaml
severity: high
confidence: high
location: "design.md#Decision: The export loads every event, then writes the CSV"
evidence: |
  `rows.map(toCsvLine).join("\n")`, and sends it.
recommendation: >-
  Write the CSV line by line to a stream (e.g. a Transform from the cursor to
  the response) so the work is split across event-loop turns, and no second
  copy of the data (lines array plus joined string) is allocated.
history:
  - status: open
    by: performance
```

`map` then `join` over millions of rows is synchronous CPU work and allocates
an intermediate array of lines plus the final string, on top of the rows array.
The design states "one event loop per instance": while one export is built,
every other request served by that instance waits. Cost grows per row of the
organization's history.

#### R-performance-3 No index is planned for the filter and sort of the export query

```yaml
severity: medium
confidence: low
location: "design.md#Decision: The export loads every event, then writes the CSV"
evidence: |
  The handler runs `SELECT * FROM audit_events WHERE org_id = $1 ORDER BY
recommendation: >-
  Check with EXPLAIN that an index on (org_id, created_at) exists, or add it
  to the plan; otherwise the database sorts the organization's rows on every
  export.
history:
  - status: open
    by: performance
```

The query filters on `org_id` and sorts on `created_at` of a table that grows
with every organization's activity. Neither the design nor the tasks mention an
index or a query plan; the schema of `audit_events` was not visible to the
reviewer, hence the low confidence.

#### R-performance-4 The spec requires an unbounded export with no performance criterion

```yaml
severity: medium
confidence: high
location: "specs/audit-export/spec.md#Requirement: Administrators export the audit log"
evidence: |
  `GET /admin/audit-events.csv` SHALL return every audit event of the organization as CSV, oldest first.
recommendation: >-
  State how the export stays bounded in resources: either a streamed response
  (memory independent of the number of events) with a scenario covering a
  large organization, or an explicit range or page parameter. Add a task
  that measures the export on a realistic volume (millions of rows).
history:
  - status: open
    by: performance
```

The only scenarios are "An export" and "Not an administrator"; nothing checks
behaviour at the volumes the design announces, and the task list has no
measurement, so the unbounded cost would pass the tests on a small fixture.

#### R-performance-5 `SELECT *` reads every column for a fixed CSV

```yaml
severity: low
confidence: low
location: "design.md#Decision: The export loads every event, then writes the CSV"
evidence: |
  The handler runs `SELECT * FROM audit_events WHERE org_id = $1 ORDER BY
recommendation: >-
  Select only the columns the CSV writes, so wide or future columns (payloads,
  JSON blobs) are not read and transferred for millions of rows.
history:
  - status: open
    by: performance
```

### security

Disabled by configuration.

### skeptic

Disabled by configuration.
