# Review record: add-invoices-api

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### security

#### R-security-1 Any signed-in user reads any organization's invoice by id

```yaml
severity: high
confidence: high
location: "src/invoices.ts:32-35"
evidence: |
  const rows = await query<InvoiceRow>(
    "SELECT id, org_id, number, total_cents, issued_on FROM invoices WHERE id = $1",
    [id],
  );
recommendation: >-
  Insecure direct object reference (CWE-639 / CWE-862). A user of organization 7
  sends `GET /invoices/42` and gets invoice 42 of another organization: the handler
  never calls `currentUser(req)` and the query filters on `id` only, so ids can be
  enumerated to read every customer's invoices (number, total, date). Scope the
  query to the caller's organization — `WHERE id = $1 AND org_id = $2` with
  `[id, currentUser(req).orgId]` — so a foreign id answers 404, and add a test
  asserting the org filter, like the list endpoint's test.
history:
  - status: open
    by: security
```

#### R-security-2 Error handler returns the error message and stack trace to the client

```yaml
severity: medium
confidence: high
location: "src/server.ts:15-18"
evidence: |
  res.status(500).json({ error: err.message, stack: err.stack });
recommendation: >-
  Information exposure through an error message (CWE-209). Any request that makes a
  handler throw — e.g. a database failure while serving `GET /invoices` — answers
  the client with the raw error message (pg errors can carry SQL fragments, table
  and constraint names, connection details) and the server stack trace with file
  paths and dependency versions. Log the error server-side as is done, and answer
  only a generic body such as `{ error: "internal error" }`.
history:
  - status: open
    by: security
```

### newcomer

Disabled by configuration.
