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
    "SELECT id, org_id, number, total_cents, issued_on FROM invoices WHERE id = $1",
    [id],
recommendation: >-
  Scope the lookup to the caller's organization — `WHERE id = $1 AND org_id = $2`
  with `[id, currentUser(req).orgId]` — and answer 404 when no row matches, so the
  endpoint does not reveal which ids exist in other organizations. Add a test that
  asserts the detail query is bound to the caller's `orgId`, like the list test does.
  Insecure direct object reference / missing authorization (CWE-639, CWE-862): any
  signed-in user of organization A sends `GET /invoices/42` (or iterates ids) and
  receives organization B's invoice number, total and date; the handler never calls
  `currentUser`, while the proposal states the API serves "the caller's
  organization's invoices".
history:
  - status: open
    by: security
```

#### R-security-2 The JSON error handler sends the exception message and stack trace to the client

```yaml
severity: medium
confidence: high
location: "src/server.ts:15-18"
evidence: |
  res.status(500).json({ error: err.message, stack: err.stack });
recommendation: >-
  Log the error server-side and answer a generic body such as
  `{ error: "internal error" }`; keep `err.message` only for errors explicitly
  marked as safe to expose (and honour `err.status`/`err.expose` for body-parser
  errors). Information exposure through an error message (CWE-209): anyone, even
  unauthenticated since `express.json()` runs before `requireUser`, sends a
  malformed JSON body with `Content-Type: application/json`; the parser's
  SyntaxError reaches this handler and the response carries the stack trace with
  server file paths and dependency versions — and a database failure would leak
  the `pg` error text (table, column, constraint names).
history:
  - status: open
    by: security
```

### newcomer

Disabled by configuration.
