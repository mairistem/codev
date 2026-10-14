# Review record: add-invoices-api

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### security

#### R-security-1 Invoice detail endpoint returns any organization's invoice (IDOR)

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
  call currentUser(req) and query "... WHERE id = $1 AND org_id = $2" with
  [id, user.orgId], answering 404 when no row matches so other organizations'
  ids are not revealed. Add a test where a user of org 7 asks for an invoice of
  org 8 and gets 404. Attack: any authenticated user sends GET /invoices/1,
  /invoices/2, ... and reads every organization's invoices (CWE-639 / CWE-862).
history:
  - status: open
    by: security
```

#### R-security-2 Error handler sends the exception message and stack trace to the client

```yaml
severity: medium
confidence: high
location: "src/server.ts:17"
evidence: |
  res.status(500).json({ error: err.message, stack: err.stack });
recommendation: >-
  Keep logging the error server-side, but answer a generic body such as
  { error: "internal error" } without err.message or err.stack. Scenario: any
  request whose handler throws (a database failure, a malformed query) returns
  file paths, module layout and driver error text — possibly SQL or connection
  details — to the caller (CWE-209).
history:
  - status: open
    by: security
```

### newcomer

Disabled by configuration.
