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
  The design's own volumes (about 40 000 events per organization and per month,
  kept seven years) mean up to about 3.4 million rows per request: the query has
  no bound, every row is held in an array, then a second full copy is made as
  CSV lines and a third as the joined string, all synchronously on the
  instance's single event loop. Cost grows linearly with the organization's
  history, per request: memory spikes can kill the instance, and the
  synchronous map/join blocks every other request on that instance for the
  duration. Stream instead: read with a server-side cursor (or keyset batches on
  created_at, id), format each batch, and write it to the response stream with
  backpressure (`Content-Type: text/csv`, chunked), so memory stays bounded by
  the batch size and the event loop yields between batches. This keeps the
  spec's "every audit event … oldest first" requirement and its ordering
  unchanged. Optionally offer a date-range filter, but do not drop events from
  the default export.
history:
  - status: open
    by: performance
```

#### R-performance-2 No index is known to serve the export's filter and sort

```yaml
severity: medium
confidence: low
location: "design.md#Decision: The export loads every event, then writes the CSV"
evidence: |
  The handler runs `SELECT * FROM audit_events WHERE org_id = $1 ORDER BY
  created_at`
recommendation: >-
  The audit_events schema is not in the repository, so the index could not be
  checked. Verify with EXPLAIN that an index on (org_id, created_at) exists;
  without it the query scans and sorts the whole table, across all
  organizations, on each export. Add the index in this change if it is missing.
history:
  - status: open
    by: performance
```

### skeptic

Disabled by configuration.

### security

Disabled by configuration.
