# Review record: add-audit-export

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### performance

#### R-performance-1 The export buffers every event of the organization in memory and builds the CSV in one synchronous pass

```yaml
severity: high
confidence: high
location: "design.md#Decision: The export loads every event, then writes the CSV"
evidence: |
  The handler runs `SELECT * FROM audit_events WHERE org_id = $1 ORDER BY
  created_at`, loads the rows into an array, builds the whole CSV string with
  `rows.map(toCsvLine).join("\n")`, and sends it.
recommendation: >-
  Cost grows with the organization's history: at "about 40 000 per organization
  and per month, kept seven years" (design.md#Context), one export reads up to
  ~3.4 million rows with no LIMIT, holds them all as objects, then as one string
  of the same size again. Scenario: an administrator of a seven-year-old
  organization requests the export; the instance allocates hundreds of MB to
  GB (risking an out-of-memory crash), and `rows.map(...).join(...)` runs
  synchronously over millions of rows on the instance's single event loop
  ("one event loop per instance"), stalling every other request it serves.
  Stream instead: read with a server-side cursor (or keyset pages on
  `(created_at, id)`), convert each batch to CSV lines and write them to the
  response through a stream that respects backpressure, so memory stays
  bounded per batch and the event loop yields between batches. Keep the
  ordering "oldest first" required by the spec. Also record why no
  alternative was considered: "none; the code is the simplest" does not
  weigh the data volume stated in the design's own context.
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
  The schema of `audit_events` is not in the repository, so the index could not
  be checked. Verify with `EXPLAIN` that an index on `(org_id, created_at)`
  exists; without it, each export scans and sorts the whole table, whose cost
  grows with every organization's seven years of events. Say in the design
  which index serves the query, or add it.
history:
  - status: open
    by: performance
```

### skeptic

Disabled by configuration.

### security

Disabled by configuration.
