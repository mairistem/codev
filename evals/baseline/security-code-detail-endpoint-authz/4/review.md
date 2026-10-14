# Review record: add-invoices-api

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### security

#### R-security-1 GET /invoices/:id returns any organization's invoice

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
  Scope the lookup to the caller's organization, as the list endpoint does:
  `WHERE id = $1 AND org_id = $2` with `[id, currentUser(req).orgId]`, and keep
  answering 404 when no row matches so other organizations' ids are not
  revealed. Add a test that a user of org 7 asking for an invoice of org 8 gets
  404 (CWE-639 / CWE-862, insecure direct object reference).
history:
  - status: open
    by: security
```

Any signed-in user of one organization sends `GET /invoices/42` (or iterates
ids 1, 2, 3…) and receives the number, total and date of invoices belonging to
other organizations: the query filters on `id` only, never on the caller's
`orgId`, although the proposal promises each organization's invoices.

#### R-security-2 The error handler sends the error message and stack trace to the client

```yaml
severity: medium
confidence: high
location: "src/server.ts:15-18"
evidence: |
  res.status(500).json({ error: err.message, stack: err.stack });
recommendation: >-
  Log the error server-side only and answer a generic body such as
  `{ error: "internal error" }` (with the status of `err.status` when it is a
  client error, e.g. 400 for a malformed body). Never put `err.stack` or a raw
  database message in the response (CWE-209).
history:
  - status: open
    by: security
```

The handler is mounted on the whole app, so it also catches errors raised
before `requireUser`: an unauthenticated client posting a malformed JSON body
makes `express.json()` fail, and gets back the stack trace with server file
paths and module versions; a database error reaching it would likewise expose
its message (table, column or connection details).

### newcomer

Disabled by configuration.
