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
  belongs to another organization: the query filters on `id` only, and ids are
  sequential numbers easy to enumerate. Scope the query to the caller's
  organization, as the list endpoint does — `WHERE id = $1 AND org_id = $2`
  with `[id, currentUser(req).orgId]` — and keep answering 404 when no row
  matches, so the existence of other organizations' invoices is not revealed.
  Add a test where the invoice belongs to another organization and the answer
  is 404.
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
  Information exposure through an error message (CWE-209). An unauthenticated
  client that sends a malformed JSON body (`Content-Type: application/json`,
  body `{`) to any route makes `express.json()` raise a parse error, which
  reaches this handler and is answered with its full stack trace — server file
  paths, dependency versions — and, for errors raised further in, database or
  driver messages. Log the error server-side only and answer a generic body
  such as `{ "error": "internal error" }`; respect `err.status` for client
  errors such as the 400 of a bad JSON body instead of turning them into 500.
history:
  - status: open
    by: security
```

#### R-security-3 Unbounded numeric id reaches the database and its rejection escapes Express

```yaml
severity: medium
confidence: low
location: "src/invoices.ts:27-31"
evidence: |
  const id = Number(req.params.id);
  if (!Number.isInteger(id)) {
recommendation: >-
  The id is checked to be an integer but not bounded: `GET /invoices/1e19`
  passes `Number.isInteger` and is sent to PostgreSQL, which rejects it as out
  of range for an integer or bigint column. The handler is `async` and Express
  4.19.2 does not forward a rejected promise to the error handler, so the
  rejection is unhandled — with Node's default `--unhandled-rejections=throw`
  any signed-in user can crash the process with one request. Validate the id
  against an allow-list at the boundary (`/^\d{1,9}$/` or a positive safe
  integer within the column's range), and wrap the async handlers so a
  rejection reaches `next(err)`. Confidence is low: the column type and the
  Node runtime and its process supervision were not visible.
history:
  - status: open
    by: security
```

### newcomer

Disabled by configuration.
