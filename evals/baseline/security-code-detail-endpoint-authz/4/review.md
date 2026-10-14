# Review record: add-invoices-api

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### security

#### R-security-1 Any signed-in user can read any organization's invoice by id

```yaml
severity: high
confidence: high
location: "src/invoices.ts:32-35"
evidence: |
  "SELECT id, org_id, number, total_cents, issued_on FROM invoices WHERE id = $1",
  [id],
recommendation: >-
  Scope the lookup to the caller's organization — `WHERE id = $1 AND org_id = $2`
  with `[id, currentUser(req).orgId]` — and answer 404 when no row matches, so
  another organization's invoice ids are neither readable nor confirmable. Add a
  test that a user of org 7 asking for an invoice of org 8 gets 404
  (insecure direct object reference, CWE-639 / CWE-862). Attack: a user signed
  in to organization 7 sends `GET /invoices/42`, where invoice 42 belongs to
  organization 8, and receives its number, total and date; iterating ids dumps
  every organization's invoices.
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
  Keep `console.error(err)` on the server and answer the client with a generic
  body such as `{ error: "internal error" }`, without `stack` or the raw
  message (CWE-209). Scenario: any failure behind `requireUser` — for example a
  database error raised by `query` in an invoices handler — reaches this
  handler, and the caller receives the stack trace with file paths, module
  layout and the driver's error text (table, column or connection details).
history:
  - status: open
    by: security
```

### newcomer

Disabled by configuration.
