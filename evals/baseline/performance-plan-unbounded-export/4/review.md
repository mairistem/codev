# Review record: add-audit-export

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### performance

#### R-performance-1 The export loads an unbounded result set into memory

```yaml
severity: high
confidence: high
location: "design.md#Decision: The export loads every event, then writes the CSV"
evidence: |
  The handler runs `SELECT * FROM audit_events WHERE org_id = $1 ORDER BY
  created_at`, loads the rows into an array, builds the whole CSV string with
recommendation: >-
  Cost grows linearly with the organization's history on a request path: at
  about 40 000 events per organization and per month kept seven years
  (design.md#Context), one export holds up to ~3.4 million rows in an array,
  then a second full copy as one CSV string, per concurrent request. Stream
  instead: read the rows with a server-side cursor or keyset batches
  (`WHERE org_id = $1 AND (created_at, id) > ($2, $3) ORDER BY created_at, id
  LIMIT n`), select only the exported columns rather than `*`, and write each
  batch to the response stream as it arrives. Keep the spec's result intact
  (every event, oldest first). Add a task for it, and a test with a large
  fixture checking memory stays bounded.
history:
  - status: open
    by: performance
```

#### R-performance-2 Building the whole CSV synchronously blocks the event loop

```yaml
severity: high
confidence: high
location: "design.md#Decision: The export loads every event, then writes the CSV"
evidence: |
  `rows.map(toCsvLine).join("\n")`, and sends it.
recommendation: >-
  The service has "one event loop per instance" (design.md#Context): mapping
  and joining millions of rows in one synchronous call stalls every other
  request on that instance for the whole build, and its cost grows with the
  audit log. Format and write the CSV per batch (a Transform stream, or
  `res.write` per batch honouring backpressure via `drain`) so the loop
  yields between batches. Same streaming rework as R-performance-1, but a
  distinct defect: a cursor alone would still block if the string is built
  in one pass.
history:
  - status: open
    by: performance
```

#### R-performance-3 No index stated for the export query's filter and sort

```yaml
severity: medium
confidence: low
location: "design.md#Decision: The export loads every event, then writes the CSV"
evidence: |
  The handler runs `SELECT * FROM audit_events WHERE org_id = $1 ORDER BY
  created_at`, loads the rows into an array, builds the whole CSV string with
recommendation: >-
  The schema of `audit_events` is not in the repository, so the index could
  not be checked. Confirm an index on `(org_id, created_at)` exists (or add
  it in this change) and check the plan with `EXPLAIN`; without it each
  export scans and sorts the whole table across all organizations.
history:
  - status: open
    by: performance
```

#### R-performance-4 "Simplest" is the only rationale, with no measurement

```yaml
severity: low
confidence: high
location: "design.md#Decision: The export loads every event, then writes the CSV"
evidence: |
  **Alternatives considered**: none; the code is the simplest.
recommendation: >-
  Record the streaming alternative and the expected volume (rows and bytes
  for a seven-year organization) in the decision, and back the choice with a
  measurement of memory and event-loop delay on a realistic export.
history:
  - status: open
    by: performance
```

### skeptic

Disabled by configuration.

### security

Disabled by configuration.
