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
  organization 7 who sends `GET /invoices/42` receives invoice 42 even when it
  belongs to another organization: the query selects by `id` alone and the
  handler never compares `org_id` with `currentUser(req).orgId`, unlike the
  list route. Scope the lookup to the caller's organization
  (`WHERE id = $1 AND org_id = $2`, `[id, currentUser(req).orgId]`) and answer
  404 when nothing matches, so the existence of other organizations' invoices
  is not revealed; add a test asserting the org filter on the detail route,
  as the list route's test does.
history:
  - status: open
    by: security
```

#### R-security-2 Error handler sends error messages and stack traces to clients

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
  Information exposure through error messages (CWE-209). A signed-in user who
  sends `GET /invoices/99999999999` passes the `Number.isInteger` check, the
  database rejects the value as out of range for an integer column, and the
  500 response carries the PostgreSQL error message and the server's stack
  trace (file paths, module layout, driver internals); any other failure —
  connection errors naming the host, for instance — leaks the same way. Log
  the error server-side only and answer a generic `{ error: "internal error" }`;
  also bound the id at the boundary (positive, within the column's range) so
  such input gets a 400.
history:
  - status: open
    by: security
```
