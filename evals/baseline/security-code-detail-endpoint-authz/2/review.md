# Review record: add-invoices-api

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### security

#### R-security-1 GET /invoices/:id returns any organization's invoice

```yaml
severity: high
confidence: high
location: "src/invoices.ts:32-40"
evidence: |
  const rows = await query<InvoiceRow>(
    "SELECT id, org_id, number, total_cents, issued_on FROM invoices WHERE id = $1",
    [id],
  );
recommendation: >-
  Insecure direct object reference (CWE-639 / CWE-862). Any authenticated user of
  organization 7 who sends `GET /invoices/42` receives invoice 42 even when it belongs
  to another organization, and can enumerate every invoice by iterating ids. Scope the
  query to the caller, as the list route does — `WHERE id = $1 AND org_id = $2` with
  `[id, currentUser(req).orgId]` — and answer 404 when no row matches, so existence in
  another organization is not revealed. Add a test that a caller of another
  organization gets 404.
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
  res.status(500).json({ error: err.message, stack: err.stack });
recommendation: >-
  Information exposure through an error message (CWE-209). Any request whose handler
  throws — a database failure in `query`, a driver error quoting the SQL or the
  connection details — returns the raw message and the full stack trace (file paths,
  module layout, library versions) to the caller. Keep `console.error(err)` on the
  server and answer a generic body such as `{ error: "internal error" }`.
history:
  - status: open
    by: security
```

#### R-security-3 An empty or zero id passes the id validation

```yaml
severity: low
confidence: high
location: "src/invoices.ts:27-31"
evidence: |
  const id = Number(req.params.id);
  if (!Number.isInteger(id)) {
recommendation: >-
  `Number(" ")` is 0 and `Number("1e3")` is 1000, so whitespace or exponent forms pass
  the integer check. No data is exposed by it, but an allow-list at the boundary is
  stricter: accept only `/^[1-9]\d*$/` and answer 400 otherwise.
history:
  - status: open
    by: security
```

### newcomer

Disabled by configuration.
