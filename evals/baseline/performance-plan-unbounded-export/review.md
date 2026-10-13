# Review record: add-audit-export

<!-- codev-review: 1 -->

## plan.challenge

### skeptic

Disabled by configuration.

### security

Disabled by configuration.

### performance

#### R-performance-1 Export loads the whole audit log into memory and builds the CSV on the event loop

```yaml
severity: high
confidence: high
location: "design.md#Decision: The export loads every event, then writes the CSV"
evidence: |
  The handler runs `SELECT * FROM audit_events WHERE org_id = $1 ORDER BY
  created_at`, loads the rows into an array, builds the whole CSV string with
  `rows.map(toCsvLine).join("\n")`, and sends it.
recommendation: >-
  Stream the export: read the rows with a server-side cursor (for example
  `pg-cursor` or `pg-query-stream` on the existing `pg` pool) in fixed-size
  batches, and write each batch's CSV lines to the response as they arrive,
  honouring back-pressure, so memory stays constant and the event loop yields
  between batches. Keep the `ORDER BY created_at` so the spec's "oldest
  first" result is unchanged, and state the expected cost in the design.
history:
  - status: open
    by: performance
```

The cost grows with the organization's whole history, per request. The
design itself states "about 40 000 per organization and per month, kept
seven years", that is up to about 3.4 million rows for one export. All of
them are held in an array, then copied into one CSV string by a synchronous
`map(...).join(...)` on the instance's single event loop ("one event loop per
instance"). One administrator's export of a mature organization therefore
blocks every other request on that instance for the duration of the join and
can exhaust the process heap (V8 also caps a single string at about 512 MB),
failing the export the spec requires to hold "every event".

#### R-performance-2 No index is shown for the export's filter and sort

```yaml
severity: medium
confidence: low
location: "design.md#Decision: The export loads every event, then writes the CSV"
evidence: |
  The handler runs `SELECT * FROM audit_events WHERE org_id = $1 ORDER BY
  created_at`, loads the rows into an array, builds the whole CSV string with
recommendation: >-
  Check with `EXPLAIN` that `audit_events` has an index on
  `(org_id, created_at)`; if not, add one in this change, so the query reads
  one organization's rows already in order instead of scanning and sorting
  the table shared by every organization.
history:
  - status: open
    by: performance
```

The schema of `audit_events` is not in the repository, so the index cannot
be verified. Without an index on `(org_id, created_at)`, each export scans the
table of all organizations and sorts the matching rows, a cost that grows
with every organization's seven-year history.
