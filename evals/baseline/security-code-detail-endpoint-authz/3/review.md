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
  organization A sends `GET /invoices/42` (or iterates ids 1, 2, 3…) and gets
  the number, total and date of organization B's invoices: the query selects
  by `id` alone, with no check against the caller's organization, while
  `GET /invoices` scopes by `org_id`. Scope the query to the caller —
  `WHERE id = $1 AND org_id = $2` with `[id, currentUser(req).orgId]` — and
  answer 404 (not 403) when no row matches, so ids of other organizations
  cannot be probed. Add a test asserting the query carries the caller's
  `orgId`, alongside the existing list test.
history:
  - status: open
    by: security
```

#### R-security-2 Error handler sends error messages and stack traces to the client

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
  Information exposure through an error message (CWE-209). Any client, even
  without a session — `express.json()` runs before `requireUser` — sends a
  request with `Content-Type: application/json` and a malformed body; the
  body parser's error reaches this handler and the response carries its
  message and full stack trace, revealing the server's file paths, module
  layout and dependency versions. Database errors forwarded here would
  likewise expose `pg` messages (table, column, constraint names). Answer a
  generic body (`{ error: "internal error" }`, with the client-error status
  for parser errors such as `err.status` 400) and keep the details in the
  server log only.
history:
  - status: open
    by: security
```

### newcomer

Disabled by configuration.
