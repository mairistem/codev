# Review record: add-api-tokens

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### security

#### R-security-1 Bearer tokens are logged at info level with every request

```yaml
severity: high
confidence: high
location: "design.md#Decision: Every request is logged with its headers"
evidence: |
  To diagnose integration problems, the middleware logs each request's
  method, path and headers at `info` level, including `Authorization`, so
  support can see which token an integration sent.
recommendation: >-
  Never log credentials (CWE-532). Anyone with read access to the logs
  (support, a log aggregator, a leaked log file) copies a valid bearer
  token from any request and calls the API as that user; since revocation
  is a Non-Goal, the stolen token stays valid indefinitely. The same
  decision also logs the `Cookie` header, exposing the session cookies
  `authenticate` reads in `src/http/middleware.rs`. Redact `Authorization`
  and `Cookie` (and any other credential header) before logging; to let
  support identify the token, log a non-secret token id or a short prefix
  instead. Update task 1.3 so `logs_requests` asserts the redaction.
history:
  - status: open
    by: security
```

#### R-security-2 API tokens are stored in clear in the database

```yaml
severity: high
confidence: high
location: "design.md#Decision: Tokens are stored as given"
evidence: |
  The `api_tokens` table holds `user_id`, `token` and `created_at`; the
  middleware looks a request's token up with `WHERE token = $1`. Storing the
  token as given keeps the lookup a simple indexed equality.
recommendation: >-
  Store a hash of the token, not the token (CWE-312 / CWE-256). Anyone who
  reads the `api_tokens` table — a database backup, a read-only replica, an
  SQL injection elsewhere — gets every user's working credential and can
  call the API as them, with no revocation to stop it. Since the tokens are
  32 random bytes, a fast SHA-256 of the token is enough: store
  `token_hash` with a unique index and look up `WHERE token_hash = $1` on
  the hash of the presented token, which keeps the lookup a simple indexed
  equality. This also matches the spec's "shown once": the plaintext then
  exists only in the creation response. Update the migration of task 1.1
  accordingly.
history:
  - status: open
    by: security
```

### skeptic

Disabled by configuration.
