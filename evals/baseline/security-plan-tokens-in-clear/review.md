# Review record: add-api-tokens

<!-- codev-review: 1 -->

## plan.challenge

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
  Store only a hash of the token (SHA-256 of the 32 random bytes is enough
  for a high-entropy secret; no bcrypt needed) in a `token_hash` column, and
  have the middleware hash the presented bearer value and look it up with
  `WHERE token_hash = $1` — still an indexed equality, so the performance
  argument holds. Anyone who reads the `api_tokens` table (a database
  backup, a read replica, a support SQL console, an SQL injection elsewhere)
  otherwise obtains every user's working bearer credential and can call the
  API as them; since revocation is a Non-Goal, a leaked token cannot even be
  invalidated. Update the decision, the migration task 1.1 and add a
  scenario stating the token is not stored in recoverable form (CWE-312,
  CWE-256).
history:
  - status: open
    by: security
```

#### R-security-2 The Authorization header, carrying the bearer token, is logged at info level

```yaml
severity: high
confidence: high
location: "design.md#Decision: Every request is logged with its headers"
evidence: |
  To diagnose integration problems, the middleware logs each request's
  method, path and headers at `info` level, including `Authorization`, so
  support can see which token an integration sent.
recommendation: >-
  Redact `Authorization` (and `Cookie`) before logging: log the scheme and,
  at most, a non-secret identifier such as the token's id or the first few
  characters of its hash. As designed, every authenticated request writes a
  live bearer token to the logs, so anyone with access to the logs or the
  log pipeline (support, a log aggregation vendor, a leaked log archive)
  can replay it and act as the user — with no revocation to stop it. The
  same line would also log Basic-auth passwords that integrations send
  today. Change task 1.3 so `logs_requests` asserts the header value is
  absent from the log output (CWE-532).
history:
  - status: open
    by: security
```

### skeptic

Disabled by configuration.
