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
  Insecure direct object reference (CWE-639 / CWE-862). Any signed-in user of
  organization 7 who sends `GET /invoices/42` gets invoice 42 even when it
  belongs to another organization: the handler never calls `currentUser(req)`
  and filters on `id` alone, while the list endpoint scopes by `org_id`. Scope
  the query to the caller — `WHERE id = $1 AND org_id = $2` with
  `[id, currentUser(req).orgId]` — so another organization's invoice answers
  404, and add a test asserting the `org_id` filter as the list test does.
history:
  - status: open
    by: security
```

#### R-security-2 The error handler sends the error message and stack trace to the client

```yaml
severity: medium
confidence: high
location: "src/server.ts:15-18"
evidence: |
  app.use((err: Error, _req: Request, res: Response, _next: NextFunction) => {
    console.error(err);
    res.status(500).json({ error: err.message, stack: err.stack });
  });
recommendation: >-
  Information exposure through an error message (CWE-209). An unauthenticated
  client that POSTs a malformed JSON body to any route makes `express.json()`
  (mounted before `requireUser`) fail, and this handler answers with the
  parser's message and full stack trace — server file paths and module
  layout; any other error reaching it (a database error, for instance) would
  likewise disclose its message. Log the error server-side only and answer a
  generic body such as `{ error: "internal error" }`, keeping the error's own
  status when it carries one (e.g. 400 for a body-parser error).
history:
  - status: open
    by: security
```

### newcomer

Disabled by configuration.
