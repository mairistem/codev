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
    "SELECT id, org_id, number, total_cents, issued_on FROM invoices WHERE id = $1",
    [id],
recommendation: >-
  Scope the lookup to the caller's organization: call `currentUser(req)` and
  query `WHERE id = $1 AND org_id = $2` with `[id, user.orgId]`, answering 404
  when no row matches so other organizations' ids are not confirmed. Add a test
  asserting the detail query is bound to the caller's `orgId`, as the list test
  does (CWE-639 / CWE-862). Attack: any signed-in user of organization A sends
  `GET /invoices/42`, `GET /invoices/43`, ... and receives the number, total and
  date of every other organization's invoices, since ids are sequential integers.
history:
  - status: open
    by: security
```

#### R-security-2 Error handler sends the stack trace to the client

```yaml
severity: medium
confidence: high
location: "src/server.ts:15-18"
evidence: |
    res.status(500).json({ error: err.message, stack: err.stack });
recommendation: >-
  Keep `console.error(err)` for the server log, but answer only a generic body
  such as `{ error: "internal error" }` (CWE-209). The handler is registered
  globally, so it also handles errors raised by `express.json()` before
  `requireUser`: an unauthenticated client posting a malformed JSON body to any
  route receives the error message and the stack with the server's file paths
  and module layout; any error forwarded from `query` would likewise reveal
  database messages (table, column, constraint names).
history:
  - status: open
    by: security
```

### newcomer

Disabled by configuration.
