# Review record: add-invoices-api

<!-- codev-review: 1 -->

## code.review

### security

#### R-security-1 GET /invoices/:id returns any organization's invoice

```yaml
severity: high
confidence: high
location: "src/invoices.ts:32-35"
evidence: |
    "SELECT id, org_id, number, total_cents, issued_on FROM invoices WHERE id = $1",
    [id],
recommendation: Scope the lookup to the caller's organization (`WHERE id = $1 AND org_id = $2` with `currentUser(req).orgId`), answer 404 when it does not match, and add a test that a caller from another organization gets 404 (CWE-639 / CWE-862).
history:
  - status: open
    by: security
```

Any signed-in user can enumerate integer ids (`GET /invoices/1`, `/2`, …) and read every other organization's invoices: the detail route never calls `currentUser` and filters by `id` only, unlike the list route which filters by `org_id`. The proposal promises each organization's invoices; the only test covers the list route.

#### R-security-2 Error handler sends the exception message and stack trace to the client

```yaml
severity: medium
confidence: high
location: "src/server.ts:15-18"
evidence: |
    res.status(500).json({ error: err.message, stack: err.stack });
recommendation: |
  Log the error server-side only and answer a generic body such as `{ error: "internal error" }`; never serialize `err.stack` or raw `err.message` into the response (CWE-209).
history:
  - status: open
    by: security
```

Any caller who triggers a failure (for example a database error from `query`) receives file paths, dependency versions and possibly SQL or connection details from the driver's message, which helps an attacker map the backend.

### newcomer

Disabled by configuration.
