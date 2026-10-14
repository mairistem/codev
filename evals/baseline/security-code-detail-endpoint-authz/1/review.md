# Review record: add-invoices-api

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### security

#### R-security-1 Any signed-in user can read any organization's invoice by id

```yaml
severity: high
confidence: high
location: "src/invoices.ts:33-36"
evidence: |
  const rows = await query<InvoiceRow>(
    "SELECT id, org_id, number, total_cents, issued_on FROM invoices WHERE id = $1",
    [id],
  );
recommendation: >-
  Scope the lookup to the caller's organization — `WHERE id = $1 AND org_id = $2`
  with `[id, currentUser(req).orgId]` — so another organization's invoice answers
  404, exactly like a missing one. Add a test asserting the detail query is bound
  to the caller's `orgId`, as the list test does (CWE-639 / CWE-862, insecure
  direct object reference).
history:
  - status: open
    by: security
```

Scenario: a user of organization 7 sends `GET /invoices/42` where invoice 42 belongs to organization 9; `requireUser` lets the request through, the query filters on `id` only, and the handler returns organization 9's invoice number and total. Iterating ids enumerates every organization's invoices.

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
  Answer a generic body such as `{ error: "internal error" }` and keep the message
  and stack in the server log only (CWE-209). The error is already logged by
  `console.error(err)`.
history:
  - status: open
    by: security
```

Scenario: any request whose handling throws — a malformed JSON body rejected by `express.json()`, or a `pg` error such as a failed connection or a bad query — gets a 500 whose body holds the raw error message (database host, SQL details) and the stack trace with server file paths, handing an attacker internal details of the deployment.
