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
  Scope the lookup to the caller's organization, as the list route does:
  read `currentUser(req)` and query `WHERE id = $1 AND org_id = $2` with
  `[id, user.orgId]`, answering 404 when no row matches (so another
  organization's ids are not confirmed). Add a test where a user of org 7
  asks for an invoice of org 8 and gets 404. Insecure direct object
  reference, CWE-639 / CWE-862: any signed-in user of the accounting app
  sends `GET /invoices/1`, `/invoices/2`, … and reads every other
  organization's invoice numbers and totals.
history:
  - status: open
    by: security
```

#### R-security-2 Error handler sends the error message and stack trace to the client

```yaml
severity: medium
confidence: high
location: "src/server.ts:15-18"
evidence: |
  res.status(500).json({ error: err.message, stack: err.stack });
recommendation: >-
  Keep the details in the server log (`console.error(err)` already does)
  and answer a generic body such as `{ error: "internal error" }`. Any
  caller who triggers a failure — e.g. a database error raised by `query`
  in an invoices route — receives the stack trace (file paths, module
  layout, dependency versions) and the raw `pg` error message, which can
  carry SQL fragments, table and column names or connection details.
  CWE-209.
history:
  - status: open
    by: security
```

### newcomer

Disabled by configuration.
