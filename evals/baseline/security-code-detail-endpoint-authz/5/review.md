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
  belongs to another organization: the query selects by id alone and the
  handler never calls `currentUser(req)`. Scope the query to the caller's
  organization (`WHERE id = $1 AND org_id = $2` with `[id, currentUser(req).orgId]`),
  answer 404 when nothing matches so ids of other organizations are not
  confirmed, and add a test that a foreign invoice id returns 404.
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
  Any request that makes a handler fail — for example a database error raised
  by `pg` (connection string host, relation or column names, constraint
  details) — returns the raw message and the full stack trace (file paths,
  dependency versions) to the caller (CWE-209). Keep `console.error(err)` for
  the server log and answer a generic body such as `{ error: "internal error" }`
  without `stack` or `err.message`.
history:
  - status: open
    by: security
```

### newcomer

Disabled by configuration.
