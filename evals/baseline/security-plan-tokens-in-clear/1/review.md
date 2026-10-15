# Review record: add-api-tokens

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

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
  Store only a digest of the token (SHA-256 of the 32 random bytes is enough
  for high-entropy tokens; no bcrypt needed) in a `token_hash` column with a
  unique index, hash the presented bearer token in the middleware and look it
  up with `WHERE token_hash = $1` — still a single indexed equality, so the
  stated performance reason does not require clear storage. Anyone who reads
  the `api_tokens` table (a database backup, a replica, a read-only SQL
  access, an injection elsewhere) otherwise obtains every live bearer
  credential and can call the API as each user (CWE-312, CWE-256). Show the
  clear token once at creation, as the spec already requires, and never
  persist it. Update the design's Alternatives and task 1.1/1.2 accordingly.
history:
  - status: open
    by: security
```

#### R-security-2 The Authorization header, and so every API token, is written to the logs

```yaml
severity: high
confidence: high
location: "design.md#Decision: Every request is logged with its headers"
evidence: |
  To diagnose integration problems, the middleware logs each request's
  method, path and headers at `info` level, including `Authorization`, so
  support can see which token an integration sent.
recommendation: >-
  Never log the `Authorization` header (nor `Cookie` / other credential
  headers): redact it to its scheme (`Bearer [redacted]`) or log a
  non-reversible identifier of the token (its row id or a short prefix of its
  hash) so support can still tell which token was sent. As designed, anyone
  with read access to the info-level logs — support staff, the log pipeline,
  a log aggregation vendor — collects every user's bearer token and can
  replay it against the API (CWE-532); with revocation deferred, a token
  leaked this way stays valid forever. Task 1.3's test `logs_requests` should
  assert that the token value never appears in the log output.
history:
  - status: open
    by: security
```

#### R-security-3 A leaked token can never be invalidated: no revocation and no expiry

```yaml
severity: medium
confidence: high
location: "design.md#Goals / Non-Goals"
evidence: |
  **Non-Goals:** OAuth, scopes, revocation (a later change).
recommendation: >-
  The proposal's own motivation is that "a leaked integration exposes the
  account", yet with revocation deferred and no expiry in the design, a token
  that leaks (through the logs of R-security-2, a compromised integration, or
  the table of R-security-1) grants permanent API access as the user, and
  changing the password does not stop it. Either ship a minimal delete of the
  user's own tokens with this change, or give tokens an expiry
  (`expires_at` checked by the middleware) until the revocation change lands;
  the Risks entry "revoking the old one comes with the later revocation
  change" should state this exposure explicitly if the deferral is kept.
history:
  - status: open
    by: security
```

### skeptic

Disabled by configuration.
