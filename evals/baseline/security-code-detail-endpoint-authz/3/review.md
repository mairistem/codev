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
  belongs to another organization, and can enumerate every customer's invoices
  by walking the ids. Scope the lookup to the caller like the list does —
  `WHERE id = $1 AND org_id = $2` with `[id, currentUser(req).orgId]` — and keep
  answering 404 when no row matches, so foreign ids are indistinguishable from
  missing ones. Add a test asserting the query is scoped by `org_id`.
history:
  - status: open
    by: security
```

#### R-security-2 Error handler returns the error message and stack trace to any client

```yaml
severity: medium
confidence: high
location: "src/server.ts:15-18"
evidence: |
  app.use((err: Error, _req: Request, res: Response, _next: NextFunction) => {
    console.error(err);
    res.status(500).json({ error: err.message, stack: err.stack });
recommendation: >-
  Information exposure through an error message (CWE-209). `express.json()`
  runs before `requireUser`, so an unauthenticated client that POSTs a
  malformed JSON body to any path gets the body-parser error forwarded here
  and receives the full stack trace (server file paths, node_modules layout and
  versions); any other error reaching the handler (database errors included)
  leaks its message the same way. Keep logging the error server-side, but answer
  a generic body such as `{ "error": "internal error" }` (and the error's own
  status, e.g. 400 for `err.status`/`err.expose` errors, instead of a blanket 500).
history:
  - status: open
    by: security
```

#### R-security-3 Invoice id is not range-checked and a database error escapes the async handler

```yaml
severity: medium
confidence: high
location: "src/invoices.ts:27-31"
evidence: |
  const id = Number(req.params.id);
  if (!Number.isInteger(id)) {
    res.status(400).json({ error: "id must be a number" });
    return;
  }
recommendation: >-
  The boundary check accepts any integer-valued number, so a signed-in user who
  sends `GET /invoices/100000000000000000000` (or `/invoices/1e20`) passes
  validation and the query fails in PostgreSQL with "out of range for type
  integer". With express 4.19.2 (package.json) a rejected promise in an async
  handler is not forwarded to the error handler: the request hangs and the
  rejection is unhandled, which terminates the process under Node's default
  `--unhandled-rejections=throw` — one request takes the API down for everyone.
  Validate with an allow-list (`/^[1-9][0-9]{0,9}$/` and an upper bound matching
  the column type) and wrap the async handlers so rejections reach `next(err)`.
history:
  - status: open
    by: security
```

### newcomer

Disabled by configuration.
