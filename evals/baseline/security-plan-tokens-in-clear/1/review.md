# Review record: add-api-tokens

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### skeptic

Disabled by configuration.

### security

#### R-security-1 API tokens are stored in clear in the database

```yaml
severity: high
confidence: high
location: "design.md#Decision: Tokens are stored as given"
evidence: |
  The `api_tokens` table holds `user_id`, `token` and `created_at`; the
  middleware looks a request's token up with `WHERE token = $1`. Storing the
  token as given keeps the lookup a simple indexed equality.
recommendation: >-
  Store only a hash of the token (SHA-256 is enough for a random 32-byte
  value; no bcrypt needed) in a `token_hash` column with a unique index, and
  look it up with `WHERE token_hash = $1` on the hash of the presented token —
  still an indexed equality, so the stated reason for storing it in clear does
  not hold. Anyone who reads the table today (a database backup, a replica, a
  read-only analyst account, an SQL injection elsewhere in the service) gets
  every user's live bearer credentials and can call the API as them, the same
  exposure the proposal's Why sets out to remove (CWE-312, CWE-256). Keep a
  short non-secret prefix if support needs to identify a token. Update task
  1.1 and the spec ("shown once") accordingly.
history:
  - status: open
    by: security
```

#### R-security-2 The Authorization header, with the bearer token, is logged at info level

```yaml
severity: high
confidence: high
location: "design.md#Decision: Every request is logged with its headers"
evidence: |
  To diagnose integration problems, the middleware logs each request's
  method, path and headers at `info` level, including `Authorization`, so
  support can see which token an integration sent.
recommendation: >-
  Redact `Authorization` (and `Cookie`, `Set-Cookie`, `Proxy-Authorization`)
  before logging; to let support see which token was sent, log the token's
  non-secret prefix or its id after lookup. As designed, anyone with access to
  the logs — support staff, the log aggregation service, a leaked log archive —
  copies a live token from any request line and replays it in
  `Authorization: Bearer` to act as that user (CWE-532). Task 1.3's test
  `logs_requests` should assert the header is redacted.
history:
  - status: open
    by: security
```

#### R-security-3 A leaked token cannot be revoked and never expires

```yaml
severity: medium
confidence: high
location: "design.md#Goals / Non-Goals"
evidence: |
  **Non-Goals:** OAuth, scopes, revocation (a later change).
recommendation: >-
  Ship at least a way to delete one's own token (or an expiry) with this
  change, or record the gap under Risks / Trade-offs with who decides it. The
  proposal's Why is that "a leaked integration exposes the account"; with this
  plan, once a token leaks (for instance through R-security-2's logs) the
  holder keeps full account access indefinitely and, unlike a password, the
  user has no way to change or cut it off. The Risks section only covers a
  lost token, not a leaked one.
history:
  - status: open
    by: security
```

#### R-security-4 Authorization of token creation is not specified

```yaml
severity: medium
confidence: low
location: "specs/api-tokens/spec.md#Requirement: A user creates an API token"
evidence: |
  The service SHALL let a signed-in user create an API token, shown once, and SHALL accept it in the `Authorization: Bearer` header of later requests.
recommendation: >-
  State whether "signed-in" includes a request authenticated by an API token.
  If it does, a stolen token can mint further tokens that survive any later
  revocation of the first one; require a password session (or a
  re-authentication) for the creation endpoint and add a scenario for it.
  Recorded with low confidence: the plan does not show the creation endpoint
  nor how the middleware marks a session as signed in.
history:
  - status: open
    by: security
```
